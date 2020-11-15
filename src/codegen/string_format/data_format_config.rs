use super::FormatConfig;
use crate::codegen::CodeConfig;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct DataFormatConfig {
    /// Regular concept config.
    pub tao_cfg: FormatConfig,
    /// Rust primitive that this concept represents.
    pub rust_primitive_name: String,
    /// Rust code representation of the default value of this concept.
    pub default_value: String,
}

impl<'a> From<&'a CodeConfig<'a>> for DataFormatConfig {
    /// Extract format values from code config.
    fn from(cfg: &CodeConfig) -> Self {
        Self {
            tao_cfg: FormatConfig::from(cfg),
            rust_primitive_name: cfg.rust_primitive_name.clone(),
            default_value: cfg.default_value.clone(),
        }
    }
}
