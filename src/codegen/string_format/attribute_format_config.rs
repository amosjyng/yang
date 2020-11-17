use super::FormatConfig;
use crate::codegen::StructConfig;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct AttributeFormatConfig {
    /// Regular concept config.
    pub tao_cfg: FormatConfig,
    /// Attribute's owner archetype.
    pub owner_type: StructConfig,
    /// Attribute's owner form.
    pub owner_form: StructConfig,
    /// Attribute's value archetype.
    pub value_type: StructConfig,
    /// Attribute's value form.
    pub value_form: StructConfig,
}
