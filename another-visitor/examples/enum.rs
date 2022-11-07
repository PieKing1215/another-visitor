use another_visitor::{Visitable, Visitor};

#[derive(Visitable)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable)]
enum B {
    Var1(C),
    Var2(D),
}

#[derive(Visitable)]
struct C {
    #[visit(skip)]
    i: i32,
}

#[derive(Visitable)]
struct D {
    #[visit(skip)]
    msg: String,
}

#[derive(Visitor)]
#[visit(A, B, C, D)]
struct AVisitor {}

impl another_visitor::VisitorHelper for AVisitor {
    type Output = String;

    #[allow(unused_variables)]
    fn aggregate(&mut self, a: Self::Output, b: Self::Output) -> Self::Output {
        format!("{a}{b}")
    }
}

impl AVisitor {
    fn visit_a(&mut self, a: &A) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(A {} {})", self.visit(&a.b1), self.visit(&a.b2))
    }

    fn visit_b(&mut self, b: &B) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(B {})", self.visit_children(b))
    }

    fn visit_c(&mut self, c: &C) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(C {})", c.i)
    }

    fn visit_d(&mut self, d: &D) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(D {})", d.msg)
    }
}

fn main() {
    let dat = A {
        b1: B::Var1(C { i: 1 }),
        b2: B::Var2(D { msg: "a".into() }),
    };

    let mut vis = AVisitor {};
    println!("{}", vis.visit(&dat));
}
