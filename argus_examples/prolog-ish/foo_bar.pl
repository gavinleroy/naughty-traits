%% Foo/Bar
%% pub trait Foo {}
%% pub trait Bar {}
%% impl<T: Foo> Bar for T {}
%% impl Foo for i32 {}
%% fn bar<T: Bar>(t: T) {}
%% fn test(i: i32) {
%%     bar(i)
%% }
%% fn main() {}

foo(i32).
foo(u32).
foo(f32).

bar(X) :- foo(X).

typeck_test :- bar(string).
