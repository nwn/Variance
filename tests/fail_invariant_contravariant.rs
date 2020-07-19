mod common;

use common::*;

fn invariant_fail_contravariant<'a>() {
    let _in: In<Lifetime<'static>> = In(
        Invariant::<Lifetime<'a>>::default(),
    );
}

fn main() {}
