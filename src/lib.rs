//! Yang is a code generator for [Yin](https://crates.io/crates/zamm_yin).

#![warn(missing_docs)]

/// Code generation utilities.
pub mod codegen;
/// Helper functions to make codegen specification smoother.
pub mod helper;
/// Yang-specific concepts.
pub mod tao;
/// Re-export of Yin wrappers.
pub mod node_wrappers {
    pub use zamm_yin::node_wrappers::*;
}
