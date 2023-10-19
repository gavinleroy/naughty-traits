use traits::IntoString;

fn invoke_trait_bound<T: IntoString>(v: T) {}

fn test(v: Vec<(i32, i32)>) {
    invoke_trait_bound(v);
}

fn main() {}

mod traits {

    // A trait definition establishes the unit of shared behavior.
    pub trait IntoString {
        fn to_string(&self) -> String;
    }

    // A trait implementation associates a type with a trait. Logically, this is the fact:
    //   (i32, i32): IntoString.
    impl IntoString for (i32, i32) {
        fn to_string(&self) -> String {
            /*  */
        }
    }

    // An implementation can be parameteric. Logically, this is the rule:
    //   Vec<T>: IntoString :- T: IntoString.
    impl<T: IntoString> IntoString for Vec<T> {
        fn to_string(&self) -> String {
            /* */
        }
    }
}
