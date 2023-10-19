%% IntoString
%%
%% // A trait definition establishes the unit of shared behavior.
%% trait ToString {
%%   fn to_string(&self) -> String;
%% }
%%
%% // A trait implementation associates a type with a trait. Logically, this is the fact:
%% //   (i32, i32): ToString.
%% impl ToString for (i32, i32) {
%%   fn to_string(&self) -> String {
%%     format!("({}, {})", self.0, self.1)
%%   }
%% }
%%
%% // An implementation can be parameteric. Logically, this is the rule:
%% //   Vec<T>: ToString :- T: ToString.
%% impl<T: ToString> ToString for Vec<T> {
%%   fn to_string(&self) -> String {
%%     let s = self.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
%%     format!("[{s}]")
%%   }
%% }
%%
%% // A trait method is normally invoked with the dot operator. Logically, this is the query:
%% //   ?- Vec<(i32, i32)>: ToString
%% fn main() {
%%   let v = vec![(0, 1), (2, 3)];
%%   println!("{}", v.to_string());
%% }

% to_string/1
to_string(tuple(i32, i32)).
to_string(vec(T)) :- to_string(T).

main_typeck :- to_string(vec(tuple(i32, i32))).
