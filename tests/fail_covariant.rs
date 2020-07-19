mod common;

use common::*;

fn covariant_fail<'a>() {
    let _co: Co<Lifetime<'static>> = Co(
        Covariant::<Lifetime<'a>>::default(),
    );
}

fn main() {}
