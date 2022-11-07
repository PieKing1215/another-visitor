use another_visitor::{Visitable, Visitor, VisitorHelper};

#[derive(Visitable)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable)]
struct B {
    #[visit(skip)]
    msg: String,
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
    println!("{}", vis.visit(&dat));
}
