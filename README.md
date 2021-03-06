# Variance

Variance is a set of `PhantomData`-like marker types that make it easier to
specify the [variance] of your generic types with respect to their parameters.

[variance]: https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)

[![Latest version](https://img.shields.io/crates/v/type-variance.svg)](https://crates.io/crates/type-variance)
[![Documentation](https://docs.rs/type-variance/badge.svg)](https://docs.rs/type-variance)
[![License](https://img.shields.io/crates/l/type-variance.svg)](https://gitlab.com/nwn/variance/-/blob/master/LICENSE)

## Getting Started
The Variance crate is available on [crates.io](https://crates.io/crates/type-variance). Add the following dependency to your Cargo manifest:
``` toml
[dependencies]
type-variance = "0.1.0"
```
See the [docs](https://docs.rs/type-variance) for detailed usage information.

## Example
The crate provides three zero-sized marker types: `Covariant<T>`,
`Contravariant<T>`, and `Invariant<T>`, each marking the given type parameter
as having the respective variance.

For example:
``` rust
use type_variance::{Covariant, Contravariant};

// UnaryFunction is a zero-sized type that is covariant to `Arg` and
// contravariant to `Ret`.
struct UnaryFunction<Arg, Ret> {
    arg: Covariant<Arg>,
    ret: Contravariant<Ret>,
}

fn foo<'sup>() {
    // Here, the type &'static() is a subtype of &'sup().
    // Therefore, `Arg` may be replaced with a subtype and `Ret` may be
    // replaced with a supertype.
    let _func: UnaryFunction<&'sup(), &'static()> = UnaryFunction {
        arg: Covariant::<&'static()>::default(),
        ret: Contravariant::<&'sup()>::default(),
    };
}
```

## License
This crate is [MIT licensed](LICENSE).
