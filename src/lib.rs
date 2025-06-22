//! D&D 5e types and common mechanics.
//!
//! This crate provides types and common mechanics used in Dungeons & Dragons 5th Edition (D&D 5e).
//!
//! ## Getting Started
//!
//! To use this crate, add it to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies.dnd]
//! version = "0.1"
//! ```
//!
//! You can then import the necessary modules in your Rust code:
//!
//! ```rust
//! use dnd::core::{AbilityScore, AbilityModifier, Level, ProficiencyBonus};
//!
//! let strength = AbilityScore::new(16);
//! let modifier = AbilityModifier::from(strength);
//! let level = Level::new(5);
//! let proficiency_bonus = ProficiencyBonus::from(level);
//! assert_eq!(modifier.value(), 3);
//! assert_eq!(proficiency_bonus.value(), 3);
//! ```
//!
//! ## Features
//!
//! It is compatible with `no_std` environments, making it suitable for embedded systems:
//!
//! ```toml
//! [dependencies.dnd]
//! version = "0.1"
//! default-features = false
//! ```

#![no_std]

/// A dependency-free[^1] foundation of `dnd`.
///
/// [^1]: Optionally includes `serde` for serialization and deserialization features.
pub mod core;
