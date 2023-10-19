#![feature(negative_impls)]

use rayon::prelude::*;
use std::sync::MutexGuard;

#[derive(Debug)]
struct NotSend;

impl !Send for NotSend {}
impl !Sync for NotSend {}

fn parallel_do(vs: Vec<NotSend>) {
    vs.into_par_iter();
}

fn main() {}
