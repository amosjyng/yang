use crate::codegen::docstring::into_docstring;
use crate::codegen::{CodeConfig, NameTransform};
use itertools::Itertools;

/// Config values at the time of string generation.
pub struct FormatConfig {
    /// Name to use for the yin crate.
    pub yin_crate: String,
    /// Main file imports.
    pub imports: Option<String>,
    /// Name of the class.
    pub name: String,
    /// Name of the concept.
    pub internal_name: String,
    /// Name of the parent class.
    pub parent_name: String,
    /// Import path for the parent class.
    pub parent_import: String,
    /// Name of the archetype used to represent this.
    pub archetype_name: String,
    /// List of attributes this class has.
    pub all_attributes: String,
    /// Imports for above list of attributes.
    pub all_attribute_imports: Vec<String>,
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
            name: "Dummy".to_owned(),
            internal_name: "dummy".to_owned(),
            parent_name: "Tao".to_owned(),
            parent_import: "tao::Tao".to_owned(),
            archetype_name: "Archetype".to_owned(),
            all_attributes: "vec![]".to_owned(),
            all_attribute_imports: vec![],
            doc: "".to_owned(),
            id: "1".to_owned(),
        }
    }
}

impl<'a> From<&'a CodeConfig<'a>> for FormatConfig {
    /// Extract format values from code config.
    fn from(cfg: &CodeConfig) -> Self {
        let yin_crate = if cfg.codegen_cfg.yin {
            "crate"
        } else {
            "zamm_yin"
        };
        let imports = if cfg.codegen_cfg.yin {
            None
        } else {
            Some("zamm_yin::tao::YIN_MAX_ID".to_owned())
        };
        let name_transform = NameTransform::from(cfg.name);
        let all_attributes = format!(
            "vec![{}]",
            cfg.all_attributes
                .iter()
                .map(|s| format!("{}::archetype()", s.name))
                .format(", ")
        );
        let all_attribute_imports = cfg
            .all_attributes
            .iter()
            .map(|s| s.import.clone())
            .collect();
        let archetype_name = if cfg.parent.name == "Attribute" {
            "AttributeArchetype".to_owned()
        } else {
            "Archetype".to_owned()
        };
        let doc = match &cfg.impl_cfg.doc {
            Some(d) => format!("\n{}", into_docstring(d.as_str(), 0)),
            None => String::new(),
        };
        let id = if cfg.codegen_cfg.yin {
            format!("{}", cfg.impl_cfg.id)
        } else {
            format!("YIN_MAX_ID + {}", cfg.impl_cfg.id)
        };

        Self {
            yin_crate: yin_crate.to_owned(),
            imports,
            name: name_transform.to_camel_case(),
            parent_name: cfg.parent.name.clone(),
            parent_import: cfg.parent.import.clone(),
            all_attributes,
            all_attribute_imports,
            archetype_name,
            internal_name: name_transform.to_kebab_case(),
            doc,
            id,
        }
    }
}
