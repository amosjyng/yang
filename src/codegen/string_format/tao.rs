use super::fragments::{AtomicFragment, FileFragment, ModuleFragment};
use super::FormatConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get the Tao body fragment.
pub fn tao_fragment(cfg: &FormatConfig) -> AtomicFragment {
    let mut imports = vec![
        "std::convert::TryFrom".to_owned(),
        "std::fmt".to_owned(),
        "std::fmt::Debug".to_owned(),
        "std::fmt::Formatter".to_owned(),
        "std::rc::Rc".to_owned(),
        format!("{}::{}", cfg.yin_crate, cfg.parent_import),
        format!("{}::tao::archetype::ArchetypeTrait", cfg.yin_crate),
        format!("{}::tao::archetype::{}", cfg.yin_crate, cfg.archetype_name),
        format!("{}::tao::FormTrait", cfg.yin_crate),
        format!("{}::node_wrappers::debug_wrapper", cfg.yin_crate),
        format!("{}::node_wrappers::CommonNodeTrait", cfg.yin_crate),
        format!("{}::node_wrappers::FinalNode", cfg.yin_crate),
    ];
    if let Some(import) = &cfg.imports {
        imports.push(import.clone());
    }
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
            
            impl CommonNodeTrait for {name} {{
                fn id(&self) -> usize {{
                    self.base.id()
                }}
            
                fn set_internal_name(&mut self, name: String) {{
                    self.base.set_internal_name(name);
                }}
            
                fn internal_name(&self) -> Option<Rc<String>> {{
                    self.base.internal_name()
                }}
            }}
            
            impl<'a> ArchetypeTrait<'a> for {name} {{
                type ArchetypeForm = {archetype};
                type Form = {name};

                const TYPE_ID: usize = {id};
                const TYPE_NAME: &'static str = "{internal_name}";
                const PARENT_TYPE_ID: usize = {parent}::TYPE_ID;
            }}
            
            impl FormTrait for {name} {{
                fn essence(&self) -> &FinalNode {{
                    &self.base
                }}
            
                fn essence_mut(&mut self) -> &mut FinalNode {{
                    &mut self.base
                }}
            }}"#,
            doc = cfg.doc,
            name = cfg.name,
            internal_name = cfg.internal_name,
            parent = cfg.parent_name,
            archetype = cfg.archetype_name,
            id = cfg.id,
        },
    }
}

/// Get the Tao test fragment
pub fn tao_test_fragment(cfg: &FormatConfig) -> ModuleFragment {
    let mut test_mod = ModuleFragment::new_test_module();
    let test_body = AtomicFragment {
        imports: vec!["crate::tao::initialize_kb".to_owned()],
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
                assert_eq!({name}::try_from("A"), Ok(concept));
                assert!({name}::try_from("B").is_err());
            }}

            #[test]
            fn create_and_retrieve_node_id() {{
                initialize_kb();
                let concept1 = {name}::individuate();
                let concept2 = {name}::individuate();
                assert_eq!(concept1.id() + 1, concept2.id());
            }}

            #[test]
            fn create_and_retrieve_node_name() {{
                initialize_kb();
                let mut concept = {name}::individuate();
                concept.set_internal_name("A".to_string());
                assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
            }}"#,
            name = cfg.name,
        },
    };
    test_mod.append(Rc::new(RefCell::new(test_body)));
    test_mod
}

/// Generate code for a Tao concept config.
pub fn code_tao(cfg: &FormatConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(tao_test_fragment(cfg))));
    file.generate_code()
}
