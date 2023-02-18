# dbent-derive - procedural macros for dbent

This crate defines two procedural macros that generate implementations for the
`Keyed` and `Label` traits.

The `Entity` macro generates an implementation of the `Keyed` trait, which requires
a type to have a `dbent::Key<T>` as its first field.

The `Label` macro generates an implementation of the `Label` trait, which requires
a type to mark a `Display`-friendly field as `#[label]`.
