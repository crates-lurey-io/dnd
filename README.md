# dnd

D&D 5e types and common mechanics.

[![Test](https://github.com/crates-lurey-io/dnd/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/dnd/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/dnd)](https://crates.io/crates/dnd)
[![codecov](https://codecov.io/gh/crates-lurey-io/dnd/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/dnd)

This crate provides types and common mechanics used in Dungeons & Dragons 5th Edition (D&D 5e).

## Getting Started

To use this crate, add it to your `Cargo.toml`:

```sh
cargo add dnd
```

You can then import the necessary modules in your Rust code:

```rust
use dnd::core::{AbilityScore, AbilityModifier, Level, ProficiencyBonus};

let strength = AbilityScore::new(16);
let modifier = AbilityModifier::from(strength);
let level = Level::new(5);
let proficiency_bonus = ProficiencyBonus::from(level);

assert_eq!(modifier.value(), 3);
assert_eq!(proficiency_bonus.value(), 3);
```

## Features

This crate has optional features and is `no_std`-compatible:

- **`std`**: Enabled by default; it includes the standard library. If you want to use this crate in a `no_std` environment, you can disable this feature:

  ```toml
  dnd = { version = "...", default-features = false }
  ```

- **`serde`**: Enables serialization and deserialization using `serde`. This feature is optional and can be enabled by adding the `serde` feature in your `Cargo.toml`:

  ```toml
  dnd = { version = "...", features = ["serde"] }
  ```


## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).
