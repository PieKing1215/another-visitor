use core::panic;

use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Ident, Meta, NestedMeta};

#[proc_macro_derive(Visitable, attributes(visit))]
pub fn derive_visitable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fields: proc_macro2::TokenStream = if let Data::Struct(struct_) = data {
        struct_
            .fields
            .into_iter()
            .filter_map(|f| {
                let skip = should_skip(&f);

                if !skip {
                    let id = f.ident;
                    Some(quote! { &self.#id, })
                } else {
                    None
                }
            })
            .collect()
    } else {
        panic!("Only structs can have Visitable derived");
    };
    let output = quote! {
        impl another_visitor::Visitable for #ident {
            fn children(&self) -> Vec<&dyn another_visitor::Visitable> {
                vec![#fields]
            }
        }
    };
    output.into()
}

#[proc_macro_derive(VisitableMut, attributes(visit))]
pub fn derive_visitable_mut(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fields: proc_macro2::TokenStream = if let Data::Struct(struct_) = data {
        struct_
            .fields
            .into_iter()
            .filter_map(|f| {
                let skip = should_skip(&f);

                if !skip {
                    let id = f.ident;
                    Some(quote! { &mut self.#id, })
                } else {
                    None
                }
            })
            .collect()
    } else {
        panic!("Only structs can have Visitable derived");
    };
    let output = quote! {
        impl another_visitor::VisitableMut for #ident {
            fn children_mut(&mut self) -> Vec<&mut dyn another_visitor::VisitableMut> {
                vec![#fields]
            }
        }
    };
    output.into()
}

fn should_skip(field: &Field) -> bool {
    field.attrs.iter().any(|a| {
        if a.path.segments.len() == 1 && a.path.segments[0].ident == "visit" {
            if let Ok(Meta::List(mut meta)) = a.parse_meta() {
                let val = meta.nested.pop().unwrap().into_value();
                if let NestedMeta::Meta(meta) = val {
                    if let Meta::Path(p) = meta {
                        return p.segments[0].ident == "skip";
                    }
                }
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

fn visit_types_from_attrs(attrs: &Vec<Attribute>) -> Vec<(Ident, proc_macro2::TokenStream)> {
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
