use super::CodegenConfig;
use crate::codegen::postprocessing::post_process_generation;
use crate::codegen::string_format::attribute::code_attribute;
use crate::codegen::string_format::code_string_concept;
use crate::codegen::string_format::tao::code_tao;
use crate::codegen::string_format::{AttributeFormatConfig, FormatConfig};
use crate::tao::ImplementConfig;
use std::collections::HashMap;

/// Config representing an imported struct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructConfig {
    /// Name of the Struct.
    pub name: String,
    /// Import path for this Struct.
    pub import: String,
}

impl Default for StructConfig {
    fn default() -> Self {
        Self {
            name: "Tao".to_owned(),
            import: "zamm_yin::tao::Tao".to_owned(),
        }
    }
}

/// Configuration settings for generating a single concept's contents.
#[derive(Default)]
pub struct CodeConfig<'a> {
    /// The target to generate.
    pub target: StructConfig,
    /// The concept's parent.
    pub parent: StructConfig,
    /// Whether or not to use attribute generation logic for this one.
    pub activate_attribute: bool,
    /// Whether or not to use data generation logic for this one.
    pub activate_data: bool,
    /// List of all attributes that this concept has.
    pub all_attributes: Vec<StructConfig>,
    /// List of all attributes introduced by this concept.
    pub introduced_attributes: Vec<StructConfig>,
    /// Structs for additional attributes.
    pub attribute_structs: HashMap<&'a str, StructConfig>,
    /// Concept-specific implementation settings.
    pub impl_cfg: ImplementConfig,
    /// Code generation settings for all concepts.
    pub codegen_cfg: CodegenConfig,
}

/// Generate the final version of code, to be output to a file as-is.
pub fn code(cfg: &CodeConfig) -> String {
    let initial_code = if cfg.activate_attribute {
        code_attribute(&AttributeFormatConfig::from(cfg))
    } else if cfg.activate_data {
        code_string_concept(&FormatConfig::from(cfg))
    } else {
        code_tao(&FormatConfig::from(cfg))
    };
    post_process_generation(&initial_code, &cfg.codegen_cfg)
}
