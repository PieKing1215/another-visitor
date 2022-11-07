#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)] // TODO

use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Ident, Meta, NestedMeta};

#[proc_macro_derive(Visitable, attributes(visit))]
pub fn derive_visitable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let children: proc_macro2::TokenStream = match data {
        Data::Struct(struct_) => struct_
            .fields
            .into_iter()
            .filter_map(|f| {
                if should_skip(&f.attrs) {
                    None
                } else {
                    let id = f.ident;
                    Some(quote! { &self.#id, })
                }
            })
            .collect(),
        Data::Enum(enum_) => {
            let matches: proc_macro2::TokenStream = enum_
                .variants
                .into_iter()
                .filter_map(|v| {
                    if should_skip(&v.attrs) {
                        None
                    } else {
                        let id = v.ident;
                        Some(quote! {
                            Self::#id(i) => i,
                        })
                    }
                })
                .collect();
            quote! {
                match self {
                    #matches
                }
            }
        },
        Data::Union(_) => panic!("Deriving Visitable for unions is not supported"),
    };
    let output = quote! {
        impl another_visitor::Visitable for #ident {
            fn children(&self) -> Vec<&dyn another_visitor::Visitable> {
                vec![#children]
            }
        }
    };
    output.into()
}

#[proc_macro_derive(VisitableMut, attributes(visit))]
pub fn derive_visitable_mut(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let children: proc_macro2::TokenStream = match data {
        Data::Struct(struct_) => struct_
            .fields
            .into_iter()
            .filter_map(|f| {
                if should_skip(&f.attrs) {
                    None
                } else {
                    let id = f.ident;
                    Some(quote! { &mut self.#id, })
                }
            })
            .collect(),
        Data::Enum(enum_) => {
            let matches: proc_macro2::TokenStream = enum_
                .variants
                .into_iter()
                .filter_map(|v| {
                    if should_skip(&v.attrs) {
                        None
                    } else {
                        let id = v.ident;
                        Some(quote! {
                            Self::#id(i) => i,
                        })
                    }
                })
                .collect();
            quote! {
                match self {
                    #matches
                }
            }
        },
        Data::Union(_) => panic!("Deriving VisitableMut for unions is not supported"),
    };
    let output = quote! {
        impl another_visitor::VisitableMut for #ident {
            fn children_mut(&mut self) -> Vec<&mut dyn another_visitor::VisitableMut> {
                vec![#children]
            }
        }
    };
    output.into()
}

fn should_skip(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        if a.path.segments.len() == 1 && a.path.segments[0].ident == "visit" {
            if let Ok(Meta::List(mut meta)) = a.parse_meta() {
                if let NestedMeta::Meta(Meta::Path(p)) = meta.nested.pop().unwrap().into_value() {
                    return p.segments[0].ident == "skip";
                }
                panic!("Invalid use of `visit` attribute");
            } else {
                panic!("Invalid use of `visit` attribute");
            }
        }

        false
    })
}

#[proc_macro_derive(Visitor, attributes(visit))]
pub fn derive_visitor(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let ts = visit_types_from_attrs(&attrs);

    let visit_impls: Vec<proc_macro2::TokenStream> = ts
        .iter()
        .map(|(id, full_type)| {
            let fname = format_ident!("visit_{}", id.to_string().to_lowercase());
            quote! {
                if let Some(d) = v.downcast_ref::<#full_type>() {
                    return self.#fname(d);
                }
            }
        })
        .collect();

    let output = quote! {
        impl another_visitor::Visitor for #ident {
            fn visit(&mut self, v: &dyn another_visitor::Visitable) -> Self::Output {
                #(#visit_impls)else *
                self.visit_children(v)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(VisitorMut, attributes(visit))]
pub fn derive_visitor_mut(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let ts = visit_types_from_attrs(&attrs);

    let visit_impls: Vec<proc_macro2::TokenStream> = ts
        .iter()
        .map(|(id, full_type)| {
            let fname = format_ident!("visit_{}", id.to_string().to_lowercase());
            quote! {
                if let Some(d) = v.downcast_mut::<#full_type>() {
                    return self.#fname(d);
                }
            }
        })
        .collect();

    let output = quote! {
        impl another_visitor::VisitorMut for #ident {
            fn visit(&mut self, v: &mut dyn another_visitor::VisitableMut) -> Self::Output {
                #(#visit_impls)else *
                self.visit_children(v)
            }
        }
    };
    output.into()
}

fn visit_types_from_attrs(attrs: &[Attribute]) -> Vec<(Ident, proc_macro2::TokenStream)> {
    attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "visit")
        .flat_map(|a| {
            let v: Vec<(Ident, proc_macro2::TokenStream)> =
                if let Ok(Meta::List(list)) = a.parse_meta() {
                    list.nested
                        .iter()
                        .map(|nested| {
                            if let NestedMeta::Meta(Meta::Path(p)) = nested {
                                (
                                    p.segments.last().unwrap().ident.clone(),
                                    p.to_token_stream(),
                                )
                            } else {
                                panic!("Invalid usage of `visit` attribute");
                            }
                        })
                        .collect()
                } else {
                    panic!("Invalid usage of `visit` attribute")
                };
            v
        })
        .collect::<Vec<_>>()
}
