use another_visitor::VisitorMut;
use another_visitor_macros::{Visitable, VisitableMut, VisitorMut};

#[derive(Visitable, VisitableMut)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable, VisitableMut)]
struct B {
    c1: C,
    c2: C,
}

#[derive(Visitable, VisitableMut)]
struct C {
    #[visit(skip)]
    i: i32,
}

#[derive(VisitorMut)]
#[visit(A, B, C)]
struct AVisitor {}

impl another_visitor::VisitorHelper for AVisitor {
    type Output = String;

    #[allow(unused_variables)]
    fn aggregate(&mut self, a: Self::Output, b: Self::Output) -> Self::Output {
        format!("{a}{b}")
    }
}

impl AVisitor {
    fn visit_a(&mut self, a: &mut A) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(A {} {})", self.visit(&mut a.b1), self.visit(&mut a.b2))
    }

    fn visit_b(&mut self, b: &mut B) -> <Self as another_visitor::VisitorHelper>::Output {
        if b.c1.i == 2 {
            b.c2.i = 10;
        }
        self.visit_children(b)
    }

    fn visit_c(&mut self, c: &mut C) -> <Self as another_visitor::VisitorHelper>::Output {
        format!("(C {})", c.i)
    }
}

fn main() {
    let mut dat = A {
        b1: B { c1: C { i: 0 }, c2: C { i: 1 } },
        b2: B { c1: C { i: 2 }, c2: C { i: 3 } },
    };

    let mut vis = AVisitor {};
    println!("{}", vis.visit(&mut dat));
}
