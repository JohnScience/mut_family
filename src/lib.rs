#![doc = include_str!("../README.md")]
#![no_std]

/// The module that contains the items for writing the code that is generic
/// over [interior/exterior mutability] wrappers.
///
/// [interior/exterior mutability]: https://doc.rust-lang.org/reference/interior-mutability.html
pub mod in_ex_mut;
/// The module that contains the items for working with references.
/// This is needed to permit abstractions over references of arbitrary mutability.
pub mod refs;
