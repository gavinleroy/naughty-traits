// trait A {}

// impl A for i32 {}
// impl<'a> A for String {}

// fn consume<T: A>(a: T) {}

// fn foo(s: &str) {
//     consume(s)
// }

// ---

#[derive(Clone)]
struct Foo;
struct Bar;

fn require_clone<T: Clone>(v: T) {}

fn test(v1: Vec<Foo>, v2: Vec<Bar>) {
    require_clone(v1);
    require_clone(&v2);
}

// ---
// This didn't actully do anything ...

// fn foo<A, B>(a: A, vec_b: Option<B>)
// where
//     A: Borrow<B>,
// {
// }

// fn test() {
//     // What we saw before:
//     let mut t: Vec<_> = vec![]; // Type: Vec<?T>
//     let mut u: Option<_> = None; // Type: Option<?U>
//     foo(t, u); // `Vec<?T>: Borrow<?U>` => ambiguous

//     // New stuff:
//     // u = Some(vec![]); // ?U = Vec<?V>
// }

fn main() {}
