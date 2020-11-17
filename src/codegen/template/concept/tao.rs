use crate::codegen::template::basic::{AtomicFragment, FileFragment, ModuleFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Templating config values for all concepts.
pub struct TaoConfig {
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

impl Default for TaoConfig {
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

/// Get the Tao body fragment.
pub fn tao_fragment(cfg: &TaoConfig) -> AtomicFragment {
    let mut imports = vec![
        "std::convert::TryFrom".to_owned(),
        "std::fmt".to_owned(),
        "std::fmt::Debug".to_owned(),
        "std::fmt::Formatter".to_owned(),
        cfg.form.import.clone(),
        cfg.parent_import.to_owned(),
        format!("{}::tao::archetype::ArchetypeTrait", cfg.yin_crate),
        format!("{}::tao::archetype::{}", cfg.yin_crate, cfg.archetype_name),
        format!("{}::node_wrappers::debug_wrapper", cfg.yin_crate),
        format!("{}::Wrapper", cfg.yin_crate),
        format!("{}::node_wrappers::FinalNode", cfg.yin_crate),
    ];
    if let Some(import) = &cfg.imports {
        imports.push(import.clone());
    }
    imports.retain(|i| i != &cfg.this.import); // don't import references to self

    AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            {doc}
            #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub struct {name} {{
                base: FinalNode,
            }}
            
            impl Debug for {name} {{
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
                    debug_wrapper("{name}", self, f)
                }}
            }}
            
            impl From<usize> for {name} {{
                fn from(id: usize) -> Self {{
                    Self {{
                        base: FinalNode::from(id),
                    }}
                }}
            }}

            impl From<FinalNode> for {name} {{
                fn from(f: FinalNode) -> Self {{
                    Self {{ base: f }}
                }}
            }}
            
            impl<'a> TryFrom<&'a str> for {name} {{
                type Error = String;
            
                fn try_from(name: &'a str) -> Result<Self, Self::Error> {{
                    FinalNode::try_from(name).map(|f| Self {{ base: f }})
                }}
            }}

            impl Wrapper for {name} {{
                type BaseType = FinalNode;

                fn essence(&self) -> &FinalNode {{
                    &self.base
                }}

                fn essence_mut(&mut self) -> &mut FinalNode {{
                    &mut self.base
                }}
            }}
            
            impl<'a> ArchetypeTrait<'a> for {name} {{
                type ArchetypeForm = {archetype};
                type Form = {form};

                const TYPE_ID: usize = {id};
                const TYPE_NAME: &'static str = "{internal_name}";
                const PARENT_TYPE_ID: usize = {parent}::TYPE_ID;
            }}"#,
            doc = cfg.doc,
            name = cfg.this.name,
            form = cfg.form.name,
            internal_name = cfg.internal_name,
            parent = cfg.parent_name,
            archetype = cfg.archetype_name,
            id = cfg.id,
        },
    }
}

/// Get the Tao test fragment
pub fn tao_test_fragment(cfg: &TaoConfig) -> ModuleFragment {
    let mut test_mod = ModuleFragment::new_test_module();
    let mut imports = vec![
        "crate::tao::initialize_kb".to_owned(),
        "std::rc::Rc".to_owned(),
        format!("{}::node_wrappers::CommonNodeTrait", cfg.yin_crate),
        format!("{}::tao::archetype::ArchetypeFormTrait", cfg.yin_crate),
        format!("{}::tao::form::FormTrait", cfg.yin_crate),
    ];
    for attr_import in &cfg.all_attribute_imports {
        imports.push(attr_import.clone());
    }
    for attr_import in &cfg.introduced_attribute_imports {
        imports.push(attr_import.clone());
    }
    let test_body = AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            #[test]
            fn check_type_created() {{
                initialize_kb();
                assert_eq!({name}::archetype().id(), {name}::TYPE_ID);
                assert_eq!(
                    {name}::archetype().internal_name(),
                    Some(Rc::new({name}::TYPE_NAME.to_string()))
                );
            }}

            #[test]
            fn check_type_attributes() {{
                initialize_kb();
                assert_eq!({name}::archetype().introduced_attribute_archetypes(), {introduced_attributes});
                assert_eq!({name}::archetype().attribute_archetypes(), {all_attributes});
            }}

            #[test]
            fn from_node_id() {{
                initialize_kb();
                let concept = {name}::individuate();
                let concept_copy = {name}::from(concept.id());
                assert_eq!(concept.id(), concept_copy.id());
            }}

            #[test]
            fn from_name() {{
                initialize_kb();
                let mut concept = {name}::individuate();
                concept.set_internal_name("A".to_owned());
                assert_eq!({name}::try_from("A").map(|c| c.id()), Ok(concept.id()));
                assert!({name}::try_from("B").is_err());
            }}

            #[test]
            fn test_wrapper_implemented() {{
                initialize_kb();
                let concept = {name}::individuate();
                assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
            }}"#,
            name = cfg.this.name,
            introduced_attributes = cfg.introduced_attributes,
            all_attributes = cfg.all_attributes,
        },
    };
    test_mod.append(Rc::new(RefCell::new(test_body)));
    test_mod
}

/// Generate code for a Tao concept config.
pub fn code_tao(cfg: &TaoConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(tao_test_fragment(cfg))));
    file.generate_code()
}
