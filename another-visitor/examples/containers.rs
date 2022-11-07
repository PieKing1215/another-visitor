use another_visitor::{Visitable, Visitor};

#[derive(Visitable)]
struct A {
    b1: B,
    b2: B,
}

#[derive(Visitable)]
struct B {
    c1: Box<C>,
    c2: Vec<C>,
    c3: [C; 2],
    c4: Option<C>,
    #[allow(clippy::vec_box)]
    c5: Option<Vec<Box<C>>>,
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
        b1: B {
            c1: Box::new(C { i: 1 }),
            c2: vec![C { i: 2 }, C { i: 3 }],
            c3: [C { i: 4 }, C { i: 5 }],
            c4: Some(C { i: 6 }),
            c5: Some(vec![Box::new(C { i: 7 }), Box::new(C { i: 8 })]),
        },
        b2: B {
            c1: Box::new(C { i: 9 }),
            c2: vec![C { i: 10 }],
            c3: [C { i: 11 }, C { i: 12 }],
            c4: None,
            c5: Some(vec![Box::new(C { i: 13 })]),
        },
    };

    let mut vis = AVisitor {};
    println!("{}", vis.visit(&dat));
}
