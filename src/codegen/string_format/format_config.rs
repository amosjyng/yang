use crate::codegen::docstring::into_docstring;
use crate::codegen::{CodegenConfig, ImplementConfig, NameTransform};

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
    /// Rustdoc for the class.
    pub doc: String,
    /// ID of the concept.
    pub id: String,
}

impl FormatConfig {
    /// Extract format values from input configs.
    pub fn from_cfgs(implement: &ImplementConfig, options: &CodegenConfig) -> Self {
        let yin_crate = if options.yin { "crate" } else { "zamm_yin" };
        let imports = if options.yin {
            None
        } else {
            Some("zamm_yin::concepts::YIN_MAX_ID".to_owned())
        };
        let name_transform = NameTransform::from_camel_case(&implement.name);
        let parent_name = implement.parent_name().to_string();
        let doc = match &implement.doc {
            Some(d) => format!("\n{}", into_docstring(d.as_str(), 0)),
            None => String::new(),
        };
        let id = if options.yin {
            format!("{}", implement.id)
        } else {
            format!("YIN_MAX_ID + {}", implement.id)
        };

        Self {
            yin_crate: yin_crate.to_owned(),
            imports,
            name: name_transform.to_camel_case(),
            parent_name,
            internal_name: name_transform.to_kebab_case(),
            doc,
            id,
        }
    }
}
