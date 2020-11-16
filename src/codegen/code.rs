use super::CodegenConfig;
use crate::codegen::string_format::attribute::code_attribute;
use crate::codegen::string_format::code_data_concept;
use crate::codegen::string_format::tao::code_tao;
use crate::codegen::string_format::{
    code_form, AttributeFormatConfig, DataFormatConfig, FormatConfig,
};
use crate::tao::ImplementConfig;
use std::collections::HashMap;
use std::rc::Rc;

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
    /// The form that represents the target.
    pub form: StructConfig,
    /// The concept's parent.
    pub parent: StructConfig,
    /// Whether or not to use root node generation logic for this one.
    pub activate_root_node: bool,
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
    /// SPECIFIC to Data concepts: name of Rust primitive.
    pub rust_primitive_name: Option<Rc<String>>,
    /// SPECIFIC to Data concepts: code representation of default value.
    pub default_value: Option<Rc<String>>,
    /// Concept-specific implementation settings.
    pub impl_cfg: ImplementConfig,
    /// Code generation settings for all concepts.
    pub codegen_cfg: CodegenConfig,
}

/// Generate code for a given concept. Post-processing still needed.
pub fn code(cfg: &CodeConfig) -> String {
    if cfg.activate_root_node {
        code_tao(&FormatConfig::from(cfg))
    } else if cfg.activate_attribute {
        code_attribute(&AttributeFormatConfig::from(cfg))
    } else if cfg.activate_data {
        code_data_concept(&DataFormatConfig::from(cfg))
    } else {
        code_form(&FormatConfig::from(cfg))
    }
}
