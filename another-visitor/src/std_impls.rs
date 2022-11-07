use crate::{Visitable, VisitableMut};

trait AsVisitable<'a> {
    fn to_visitable(self) -> &'a dyn Visitable;
}

impl<'a, T: Visitable> AsVisitable<'a> for &'a T {
    fn to_visitable(self) -> &'a dyn Visitable {
        self
    }
}

trait AsVisitableMut<'a> {
    #[allow(clippy::wrong_self_convention)]
    fn to_visitable_mut(self) -> &'a mut dyn VisitableMut;
}

impl<'a, T: VisitableMut> AsVisitableMut<'a> for &'a mut T {
    fn to_visitable_mut(self) -> &'a mut dyn VisitableMut {
        self
    }
}

// blanket impl that covers a bunch of std containers (Vec, Option)
impl<C: 'static> Visitable for C
where
    for<'a> &'a C: IntoIterator,
    for<'a> <&'a C as IntoIterator>::Item: AsVisitable<'a>,
{
    fn children(&self) -> Vec<&dyn Visitable> {
        self.into_iter()
            .map(AsVisitable::to_visitable)
            .collect::<Vec<_>>()
    }
}

impl<C: 'static> VisitableMut for C
where
    for<'a> &'a mut C: IntoIterator,
    for<'a> <&'a mut C as IntoIterator>::Item: AsVisitableMut<'a>,
{
    fn children_mut(&mut self) -> Vec<&mut dyn VisitableMut> {
        self.into_iter()
            .map(AsVisitableMut::to_visitable_mut)
            .collect::<Vec<_>>()
    }
}

impl<T: Visitable> Visitable for Box<T> {
    fn children(&self) -> Vec<&dyn Visitable> {
        vec![self.as_ref()]
    }
}

impl<T: VisitableMut> VisitableMut for Box<T> {
    fn children_mut(&mut self) -> Vec<&mut dyn VisitableMut> {
        vec![self.as_mut()]
    }
}
