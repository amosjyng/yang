/// Generate code for attribute concepts.
pub mod attribute;
/// Config values at the time of attribute string generation.
mod attribute_format_config;
mod data_concept;
mod data_format_config;
mod form;
/// Config values at the time of string generation.
pub mod format_config;
/// Intermediate code fragments produced during code generation.
pub mod fragments;
/// Import-related code generation.
pub mod imports;
mod main;
/// Generate code for generic Tao concepts.
pub mod tao;

pub use attribute_format_config::AttributeFormatConfig;
pub use data_concept::code_data_concept;
pub use data_format_config::DataFormatConfig;
pub use form::{code_form, form_fragment};
pub use format_config::FormatConfig;
pub use main::{code_main, MainConfig};
