//! Yang is a code generator for [Yin](https://crates.io/crates/zamm_yin).

#![warn(missing_docs)]

/// Code generation utilities.
///
/// This is roughly analogous to the MVC framework: Yin concepts are the model, templates are the
/// view, and the planning modules are the controllers. This is perhaps not too surprising, because
/// HTML is code too, webservers are code generators for the HTML language. Right now, we're merely
/// rendering higher-level concepts in the form of Rust code instead of HTML code, and we're using
/// the Rust compiler instead of the browser to render the generated code in a format that the end
/// user finds useful.
pub mod codegen;
/// Helper functions to make codegen specification smoother.
pub mod helper;
/// Yang-specific concepts.
pub mod tao;
/// Re-export of Yin wrappers.
pub mod node_wrappers {
    pub use zamm_yin::node_wrappers::*;
}
