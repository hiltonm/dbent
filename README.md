
# dbent - database entity types

Provides types for defining simple database entities and relationships
in structs for when you don't want to use an ORM but still want to organize
your code in an object-oriented manner.

## Usage

This crate is [on crates.io](https://crates.io/crates/dbent) and can be
used by adding `dbent` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
dbent = "0.1.0"
```

## Features

The following features are supported:

- `default`: enables `serde` and `derive` features by default
- `serde`: for `serde` serialization
- `rusqlite`: for `rusqlite` ToSql and FromSql implementations for the Key type
- `derive`: for the derive macros `Entity` and `Label`

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in dbent by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

