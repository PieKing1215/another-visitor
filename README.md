# another-visitor

[![crates.io](https://img.shields.io/crates/v/another-visitor.svg)](https://crates.io/crates/another-visitor)

A crate that lets you derive visitor pattern implementations for your structs.<br>
Made because I couldn't find an existing crate supporting the exact pattern I wanted.

The general flow is inspired by how it works in ANTLR4:
- Visitor has a return type that all visit_* fns return
- Allows only implementing visit fns for some types in the tree (defaults to visiting all children)
- Allows manually visiting children if you do implement a visit fn for a type
- Allows mutation (using VisitableMut and VisitorMut)

```rust
#[derive(Visitable)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable)]
struct B {
    #[visit(skip)]
    msg: String
}

#[derive(Visitor)]
#[visit(A, B)]
struct AVisitor {}

impl VisitorHelper for AVisitor {
    type Output = String;
}

impl AVisitor {
    fn visit_a(&mut self, a: &A) -> <Self as VisitorHelper>::Output {
        format!("(A {} {})", self.visit(&a.b1), self.visit(&a.b2))
    }

    fn visit_b(&mut self, b: &B) -> <Self as VisitorHelper>::Output {
        format!("(B {})", b.msg)
    }
}

fn main() {
    let dat = A {
        b1: B { msg: "Hello".into() },
        b2: B { msg: "World!".into() },
    };

    let mut vis = AVisitor {};
    println!("{}", vis.visit(&dat)); // => "(A (B Hello) (B World!))"
}
```
See [another-visitor/examples](another-visitor/examples) for more examples.

## TODO
- Derive Visitable(Mut) for more types (only basic structs and enums are supported)
- Visitable(Mut) impls for more std containers
- Good error messages in proc macros
- Documentation
- Tests

This project is a WIP, if you have suggestions for changes or new features, please open an issue!

## Licenses

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
