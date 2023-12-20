mod traits;

use traits::IntoString;

fn is_into_string<T: IntoString>() {}

fn main() {
    // is_into_string::<Vec<&mut str>>();
    let v = vec![(0, 1.), (2, 3.)];
    v.to_string()
}
