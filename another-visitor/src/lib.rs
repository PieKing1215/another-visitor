use downcast_rs::{impl_downcast, Downcast};

pub trait Visitable: Downcast {
    fn children(&self) -> Vec<&dyn Visitable>;
}
impl_downcast!(Visitable);

pub trait VisitableMut: Downcast {
    fn children_mut(&mut self) -> Vec<&mut dyn VisitableMut>;
}
impl_downcast!(VisitableMut);

pub trait VisitorHelper {
    type Output: Default;

    #[allow(unused_variables)]
    fn aggregate(&mut self, a: Self::Output, b: Self::Output) -> Self::Output {
        b
    }
}

pub trait Visitor: VisitorHelper {
    fn visit(&mut self, v: &dyn Visitable) -> Self::Output;

    fn visit_children(&mut self, v: &dyn Visitable) -> Self::Output {
        let mut res = Self::Output::default();
        for ele in v.children() {
            let v = self.visit(ele);
            res = self.aggregate(res, v);
        }
        res
    }
}

pub trait VisitorMut: VisitorHelper {
    fn visit(&mut self, v: &mut dyn VisitableMut) -> Self::Output;

    fn visit_children(&mut self, v: &mut dyn VisitableMut) -> Self::Output {
        let mut res = Self::Output::default();
        for ele in v.children_mut() {
            let v = self.visit(ele);
            res = self.aggregate(res, v);
        }
        res
    }
}
