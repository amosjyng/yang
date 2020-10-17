/// Generate code for attribute concepts.
pub mod attribute;
/// Config values at the time of string generation.
pub mod format_config;
/// Intermediate code fragments produced during code generation.
pub mod fragments;
/// Import-related code generation.
pub mod imports;
mod string_concept;
/// Generate code for generic Tao concepts.
pub mod tao;

pub use format_config::FormatConfig;
pub use imports::sort_imports;
pub use string_concept::code_string_concept;
