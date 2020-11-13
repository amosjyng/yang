//! Yang is a code generator for [Yin](https://crates.io/crates/zamm_yin).

#![warn(missing_docs)]

/// Code generation utilities.
pub mod codegen;
/// Interact with the terminal environment.
pub mod commands;
/// Helper functions to make codegen specification smoother.
mod helper;
/// Generate code files using Rust code that effectively serves as a `build.rs`.
pub mod main_build;
/// Input file parsing.
pub mod parse;
/// Yang-specific concepts.
pub mod tao;
