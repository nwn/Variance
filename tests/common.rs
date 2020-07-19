pub use type_variance::{Covariant, Contravariant, Invariant};

pub struct Lifetime<'a> {
    _variance: Covariant<&'a ()>,
}

pub struct Co<X>(pub Covariant<X>);
pub struct Contra<X>(pub Contravariant<X>);
pub struct In<X>(pub Invariant<X>);
