# mut_family

[![Latest Version](https://img.shields.io/crates/v/mut_family.svg)][`mut_family`]
[![Downloads](https://img.shields.io/crates/d/mut_family.svg)][`mut_family`]
[![Documentation](https://docs.rs/mut_family/badge.svg)][`mut_family`/docs]
[![License](https://img.shields.io/crates/l/mut_family.svg)][`mut_family`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/mut_family/status.svg)][`mut_family`/dep_status]

A [GAT]-based library for writing code that is [generic] over
[interior/exterior mutability] and reference of arbitrary mutability.

## SemVer Policy

At the moment, there's no any semver guarantees. The crate may undergo breaking changes.
However, you still can use it in your project if you select a specific version,
your crate is an application, and your upstream crates do not use [`mut_family`].

## Warning

The author currently believes that without proper support for [`mut`](https://doc.rust-lang.org/std/keyword.mut.html)-genericity for references as a part of the overarching [keyword generics](https://doc.rust-lang.org/std/keyword.mut.html) initiative, writing the code that is generic over interior/exterior mutability is complicated to the point of unreasonableness. For example, currently it's impossible to avoid code-duplication related to destructuring. Fundamentally, the problem is the absence of [variadic generics](https://github.com/rust-lang/rust/issues/10124).

```rust
struct MyStruct {
    x: usize,
    y: usize,
}

fn main() {
    let a = MyStruct { x: 0, y: 0 };
    let mut b = MyStruct { x: 0, y: 0 };
    
    // There is no trait for destructuring because it would require
    // variadic generics
    let MyStruct { x: x_ref, y: y_ref } = &a;
    let MyStruct { x: x_mut, y: y_mut } = &mut b;
}
```

[*Rust playground*](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3b9a171dd389fe1654a1974accd2495e)

Potentially, it'll be easier to do with something like [either::for_both](https://docs.rs/either/1.8.1/either/macro.for_both.html).

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`mut_family`]: https://crates.io/crates/mut_family
[`mut_family`/docs]: https://docs.rs/mut_family
[`mut_family`/license]: https://github.com/JohnScience/mut_family#license
[`mut_family`/dep_status]: https://deps.rs/repo/github/JohnScience/mut_family
[GAT]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
[interior/exterior mutability]: https://doc.rust-lang.org/reference/interior-mutability.html
[generic]: https://doc.rust-lang.org/book/ch10-01-syntax.html
