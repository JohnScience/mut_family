# mut_family

[![Latest Version](https://img.shields.io/crates/v/mut_family.svg)][`mut_family`]
[![Downloads](https://img.shields.io/crates/d/mut_family.svg)][`mut_family`]
[![Documentation](https://docs.rs/mut_family/badge.svg)][`mut_family`/docs]
[![License](https://img.shields.io/crates/l/mut_family.svg)][`mut_family`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/mut_family/status.svg)][`mut_family`/dep_status]

A [GAT]-based library for writing code that is [generic] over
[interior/exterior mutability].

## SemVer Policy

At the moment, there's no any semver guarantees. The crate may undergo breaking changes.
However, you stil can use it in your project if you select a specific version,
your crate is an application, and your upstream crates do not use [`mut_family`].

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
