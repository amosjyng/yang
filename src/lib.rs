//! Yang is a code generator for [Yin](https://crates.io/crates/zamm_yin).

#![warn(missing_docs)]

/// Code generation utilities.
pub mod codegen;
/// Interact with the terminal environment.
pub mod commands;
/// Input file parsing.
pub mod parse;
/// Yang-specific concepts.
pub mod tao;
