macro_rules! tests_for {
    ($($i:ident),*) => {$(
        #[test]
        fn $i() {
            let t = trybuild::TestCases::new();
            let path = format!("tests/{}/*.rs", stringify!($i));
            t.compile_fail(path);
        }
    )*};
}

// Add a new name here!
tests_for! {
    diesel,
    chumsky,
    uom,
    axum,
    bevy,
    easy_ml,
    typed_builder,
    entrait
}
