#![recursion_limit = "256"]

pub trait Visit<T: ?Sized> {
    fn visit(&mut self, elem: &T);
}

pub enum Ast {
    Null,
    App(Box<Ast>, Box<Ast>),
}

impl<T> Visit<Ast> for T
where
    T: Visit<Box<Ast>>,
{
    fn visit(&mut self, elem: &Ast) {}
}

impl<T, S: ?Sized> Visit<Box<S>> for T
where
    T: Visit<S>,
{
    fn visit(&mut self, elem: &Box<S>) {}
}

pub struct MyVisitor {}

fn test() {
    fn require_bound<T: Visit<Ast>>() {}
    require_bound::<MyVisitor>();
    // MyVisitor {}.visit(&Ast::Null)
}

fn main() {}
