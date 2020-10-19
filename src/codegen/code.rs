use super::CodegenConfig;
pub use crate::codegen::name_transform::NameTransform;
use crate::codegen::postprocessing::post_process_generation;
use crate::codegen::string_format::attribute::code_attribute;
use crate::codegen::string_format::code_string_concept;
use crate::codegen::string_format::tao::code_tao;
use crate::codegen::string_format::{AttributeFormatConfig, FormatConfig};
use crate::tao::ImplementConfig;
use std::collections::HashMap;

/// Config representing an imported struct.
#[derive(Clone)]
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
pub struct CodeConfig<'a> {
    /// Name of the concept to generate.
    pub name: &'a str,
    /// The concept's parent.
    pub parent: StructConfig,
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

impl<'a> Default for CodeConfig<'a> {
    fn default() -> Self {
        Self {
            name: "dummy",
            parent: StructConfig::default(),
            all_attributes: Vec::default(),
            introduced_attributes: Vec::default(),
            attribute_structs: HashMap::default(),
            impl_cfg: ImplementConfig::default(),
            codegen_cfg: CodegenConfig::default(),
        }
    }
}

/// Generate the final version of code, to be output to a file as-is.
pub fn code(cfg: &CodeConfig) -> String {
    let initial_code = if cfg.parent.name.to_lowercase() == "attribute" {
        code_attribute(&AttributeFormatConfig::from(cfg))
    } else if cfg.parent.name.to_lowercase() == "data" {
        code_string_concept(&FormatConfig::from(cfg))
    } else {
        code_tao(&FormatConfig::from(cfg))
    };
    post_process_generation(&initial_code, &cfg.codegen_cfg)
}
