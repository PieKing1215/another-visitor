use another_visitor::{Visitable, Visitor};

#[derive(Visitable)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable)]
struct B {
    c1: C,
    c2: C,
}

#[derive(Visitable)]
struct C {
    #[visit(skip)]
    i: i32,
}

#[derive(Visitor)]
#[visit(A, C)]
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

    fn visit_c(&mut self, c: &C) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(C {})", c.i)
    }
}

fn main() {
    let dat = A {
        b1: B { c1: C { i: 0 }, c2: C { i: 1 } },
        b2: B { c1: C { i: 2 }, c2: C { i: 3 } },
    };

    let mut vis = AVisitor {};
    println!("{}", vis.visit(&dat));
}
