use crate::codegen::StructConfig;

/// Config values at the time of string generation.
pub struct FormatConfig {
    /// Name to use for the yin crate.
    pub yin_crate: String,
    /// Main file imports.
    pub imports: Option<String>,
    /// Class representing the concept itself.
    pub this: StructConfig,
    /// Name of the concept.
    pub internal_name: String,
    /// The form representing the concept.
    pub form: StructConfig,
    /// Name of the parent class.
    pub parent_name: String,
    /// Import path for the parent class.
    pub parent_import: String,
    /// Name of the archetype used to represent this.
    pub archetype_name: String,
    /// List of attributes this class has.
    pub all_attributes: String,
    /// Imports for above list of introduced attributes.
    pub all_attribute_imports: Vec<String>,
    /// List of attributes this class introduced.
    pub introduced_attributes: String,
    /// Imports for above list of introduced attributes.
    pub introduced_attribute_imports: Vec<String>,
    /// Rustdoc for the class.
    pub doc: String,
    /// ID of the concept.
    pub id: String,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            yin_crate: "zamm_yin".to_owned(),
            imports: Some("zamm_yin::tao::YIN_MAX_ID".to_owned()),
            this: StructConfig::default(),
            internal_name: "dummy".to_owned(),
            form: StructConfig::default(),
            parent_name: "Tao".to_owned(),
            parent_import: "tao::Tao".to_owned(),
            archetype_name: "Archetype".to_owned(),
            all_attributes: "vec![]".to_owned(),
            all_attribute_imports: vec![],
            introduced_attributes: "vec![]".to_owned(),
            introduced_attribute_imports: vec![],
            doc: "".to_owned(),
            id: "1".to_owned(),
        }
    }
}
