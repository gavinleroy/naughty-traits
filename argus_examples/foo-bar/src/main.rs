pub trait Foo {}
pub trait Bar {}

impl<T: Foo> Bar for T {}

impl Foo for i32 {}

fn bar<T: Bar>(t: T) {}

fn test(i: f32) {
    bar(i)
}

fn main() {}
