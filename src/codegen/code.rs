use super::CodegenConfig;
pub use crate::codegen::name_transform::NameTransform;
use crate::codegen::postprocessing::post_process_generation;
use crate::codegen::string_format::attribute::code_attribute;
use crate::codegen::string_format::code_string_concept;
use crate::codegen::string_format::tao::code_tao;
use crate::codegen::string_format::FormatConfig;
use crate::concepts::ImplementConfig;

/// Configuration settings for generating a single concept's contents.
pub struct CodeConfig<'a> {
    /// Name of the concept to generate.
    pub name: &'a str,
    /// Name of the concept's parent.
    pub parent_name: &'a str,
    /// Concept-specific implementation settings.
    pub impl_cfg: ImplementConfig,
    /// Code generation settings for all concepts.
    pub codegen_cfg: CodegenConfig,
}

impl<'a> Default for CodeConfig<'a> {
    fn default() -> Self {
        Self {
            name: "dummy",
            parent_name: "Tao",
            impl_cfg: ImplementConfig::default(),
            codegen_cfg: CodegenConfig::default(),
        }
    }
}

/// Generate the final version of code, to be output to a file as-is.
pub fn code(cfg: &CodeConfig) -> String {
    let format_cfg = FormatConfig::from(cfg);
    let initial_code = if cfg.parent_name == "Attribute" {
        code_attribute(&format_cfg)
    } else if cfg.parent_name == "Data" {
        code_string_concept(&format_cfg)
    } else {
        code_tao(&format_cfg)
    };
    post_process_generation(&initial_code, &cfg.codegen_cfg)
}
