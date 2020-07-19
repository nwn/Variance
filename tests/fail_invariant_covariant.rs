mod common;

use common::*;

fn invariant_fail_covariant<'a>() {
    let _in: In<Lifetime<'a>> = In(
        Invariant::<Lifetime<'static>>::default(),
    );
}

fn main() {}
