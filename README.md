# Rust Foundation Project Grant 2022

This repository is used to track information about my 
[Rust Foundation Project Grant](https://foundation.rust-lang.org/news/2022-06-14-community-grants-program-awards-announcement/) 
to improve error messages emitted by rustc for trait heavy crates. 
As part of this work I will focus on the following points:

1. Collect and categorise various examples of non-optimal error messages from the rust ecosystem. 
This includes error messages generated by crates like diesel, axum or nalgebra which relay heavily
on 
2. Experiment with example cases to see which error messages could be improved by 
the usage of `#[rustc_on_unimplemented]`
3. Implement [RFC-2397](https://github.com/rust-lang/rfcs/blob/master/text/2397-do-not-recommend.md)
4. Experiment with example cases to see which error messages could be improved by
the usage of `#[do_not_recommend]`

## Call for participation

Please submit examples of bad error messages in the context of trait heavy crates 
as issue or PR (with minimal example) to this repository.


