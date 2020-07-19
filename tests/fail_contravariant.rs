mod common;

use common::*;

fn contravariant_fail<'a>() {
    let _contra: Contra<Lifetime<'a>> = Contra(
        Contravariant::<Lifetime<'static>>::default(),
    );
}

fn main() {}
