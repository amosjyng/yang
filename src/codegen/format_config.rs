use super::docstring::into_docstring;
use super::{CodegenConfig, ImplementConfig, NameTransform};

/// Config values at the time of string generation.
pub struct FormatConfig {
    /// Name to use for the yin crate.
    pub yin_crate: String,
    /// Main file imports.
    pub imports: String,
    /// Test imports.
    pub test_imports: String,
    /// KB initialization function.
    pub init_kb: String,
    /// Name of the class.
    pub name: String,
    /// Name of the concept.
    pub internal_name: String,
    /// Rustdoc for the class.
    pub doc: String,
    /// ID of the concept.
    pub id: String,
}

impl FormatConfig {
    /// Extract format values from input configs.
    pub fn from_cfgs(implement: &ImplementConfig, options: &CodegenConfig) -> Self {
        let yin_crate = if options.yin { "crate" } else { "zamm_yin" };
        let imports = if options.yin { "" } else { ", YIN_MAX_ID" };
        let test_imports = if options.yin {
            "use crate::graph::bind_in_memory_graph;"
        } else {
            "use crate::concepts::initialize_kb;"
        };
        let init_kb = if options.yin {
            "bind_in_memory_graph();"
        } else {
            "initialize_kb();"
        };
        let name_transform = NameTransform::from_camel_case(&implement.name);
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
            imports: imports.to_owned(),
            test_imports: test_imports.to_owned(),
            init_kb: init_kb.to_owned(),
            name: name_transform.to_camel_case(),
            internal_name: name_transform.to_kebab_case(),
            doc: doc,
            id: id,
        }
    }
}
