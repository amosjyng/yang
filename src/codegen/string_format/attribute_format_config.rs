use super::FormatConfig;
use crate::codegen::{CodeConfig, StructConfig};
use zamm_yin::tao::attribute::{OwnerArchetype, ValueArchetype};
use zamm_yin::tao::archetype::ArchetypeTrait;

/// Key for retrieving owner form in AttributeTrait impl.
pub const OWNER_FORM_KEY: &str = "_owner_form";
/// Key for retrieving value form in AttributeTrait impl.
pub const VALUE_FORM_KEY: &str = "_value_form";

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

impl<'a> From<&'a CodeConfig<'a>> for AttributeFormatConfig {
    /// Extract format values from code config.
    fn from(cfg: &CodeConfig) -> Self {
        Self {
            tao_cfg: FormatConfig::from(cfg),
            owner_type: cfg.attribute_structs[OwnerArchetype::TYPE_NAME].clone(),
            owner_form: cfg.attribute_structs[OWNER_FORM_KEY].clone(),
            value_type: cfg.attribute_structs[ValueArchetype::TYPE_NAME].clone(),
            value_form: cfg.attribute_structs[VALUE_FORM_KEY].clone(),
        }
    }
}
