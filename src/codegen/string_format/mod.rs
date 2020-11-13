/// Generate code for attribute concepts.
pub mod attribute;
/// Config values at the time of attribute string generation.
mod attribute_format_config;
/// Config values at the time of string generation.
pub mod format_config;
/// Intermediate code fragments produced during code generation.
pub mod fragments;
/// Import-related code generation.
pub mod imports;
mod main;
mod string_concept;
/// Generate code for generic Tao concepts.
pub mod tao;

pub use attribute_format_config::{AttributeFormatConfig, OWNER_FORM_KEY, VALUE_FORM_KEY};
pub use format_config::FormatConfig;
pub use main::{code_main, MainConfig};
pub use string_concept::code_string_concept;
