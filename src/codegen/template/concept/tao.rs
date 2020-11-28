use crate::codegen::template::basic::{
    AppendedFragment, AssertFragment, AtomicFragment, FileFragment, FunctionCallFragment,
    FunctionFragment,
};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Backwards-compatibility logic for the internal API.
#[derive(Clone)]
pub struct InternalNameConfig {
    /// Getter function for the internal name.
    pub getter: &'static str,
    /// Setter function for the internal name.
    pub setter: &'static str,
    /// Suffix to get strings into the right place.
    pub suffix: &'static str,
    /// Getter for added attributes.
    pub added_attributes: &'static str,
    /// Getter for all attributes.
    pub all_attributes: &'static str,
    /// Import for above attribute functions.
    pub attr_import: Option<&'static str>,
}

impl InternalNameConfig {
    /// Default internal API config, compatible with all Yin 0.1.x versions.
    pub const DEFAULT: Self = Self {
        getter: "internal_name",
        setter: "set_internal_name",
        suffix: ".to_owned()",
        added_attributes: "introduced_attribute_archetypes",
        all_attributes: "attribute_archetypes",
        attr_import: Some("zamm_yin::tao::form::FormTrait"),
    };

    /// Internal API config for Yin versions >= 0.1.1.
    pub const YIN_AT_LEAST_0_1_1: Self = Self {
        getter: "internal_name_str",
        setter: "set_internal_name_str",
        suffix: "",
        added_attributes: "introduced_attribute_archetypes",
        all_attributes: "attribute_archetypes",
        attr_import: Some("zamm_yin::tao::form::FormTrait"),
    };

    /// Internal API config for Yin versions >= 0.1.4.
    pub const YIN_AT_LEAST_0_1_4: Self = Self {
        getter: "internal_name_str",
        setter: "set_internal_name_str",
        suffix: "",
        added_attributes: "added_attributes",
        all_attributes: "attributes",
        attr_import: None,
    };
}

impl Default for InternalNameConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Templating config values for all concepts.
#[derive(Clone)]
pub struct TaoConfig {
    /// Main file imports.
    pub imports: Option<String>,
    /// Class representing the concept itself.
    pub this: StructConfig,
    /// Name of the concept.
    pub internal_name: String,
    /// Config for which API to use for internal names. For backwards compatibility.
    pub internal_name_cfg: InternalNameConfig,
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
            imports: Some("zamm_yin::tao::YIN_MAX_ID".to_owned()),
            this: StructConfig::default(),
            internal_name: "dummy".to_owned(),
            internal_name_cfg: InternalNameConfig::DEFAULT,
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
fn tao_fragment(cfg: &TaoConfig) -> AtomicFragment {
    let mut imports = vec![
        "std::convert::TryFrom".to_owned(),
        "std::fmt".to_owned(),
        "std::fmt::Debug".to_owned(),
        "std::fmt::Formatter".to_owned(),
        cfg.form.import.clone(),
        cfg.parent_import.to_owned(),
        "zamm_yin::tao::archetype::ArchetypeTrait".to_owned(),
        format!("zamm_yin::tao::archetype::{}", cfg.archetype_name),
        "zamm_yin::node_wrappers::debug_wrapper".to_owned(),
        "zamm_yin::Wrapper".to_owned(),
        "zamm_yin::node_wrappers::FinalNode".to_owned(),
    ];
    if let Some(import) = &cfg.imports {
        imports.push(import.clone());
    }
    if let Some(import) = &cfg.internal_name_cfg.attr_import {
        imports.push(import.to_string());
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
fn tao_test_fragment(cfg: &TaoConfig) -> AppendedFragment {
    let init_kb = Rc::new(RefCell::new(FunctionCallFragment::new(AtomicFragment {
        imports: vec!["crate::tao::initialize_kb".to_owned()],
        atom: "initialize_kb".to_owned(),
    })));
    let mut imports = vec![
        "std::rc::Rc".to_owned(),
        "zamm_yin::node_wrappers::CommonNodeTrait".to_owned(),
        "zamm_yin::tao::archetype::ArchetypeFormTrait".to_owned(),
    ];
    for attr_import in &cfg.all_attribute_imports {
        imports.push(attr_import.clone());
    }
    for attr_import in &cfg.introduced_attribute_imports {
        imports.push(attr_import.clone());
    }
    let mut test_frag = AppendedFragment::default();

    let name = &cfg.this.name;
    let mut check_type_created = FunctionFragment::new("check_type_created".to_owned());
    check_type_created.mark_as_test();
    check_type_created.append(init_kb.clone());
    check_type_created.append(Rc::new(RefCell::new(AssertFragment::new_eq(
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "{}::archetype().id()",
            name
        )))),
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "{}::TYPE_ID",
            name
        )))),
    ))));
    check_type_created.append(Rc::new(RefCell::new(AssertFragment::new_eq(
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "{name}::archetype().{getter}()",
            name = name,
            getter = cfg.internal_name_cfg.getter
        )))),
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "Some(Rc::from({name}::TYPE_NAME{suffix}))",
            name = name,
            suffix = cfg.internal_name_cfg.suffix
        )))),
    ))));
    test_frag.append(Rc::new(RefCell::new(check_type_created)));

    test_frag.append(Rc::new(RefCell::new(AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            #[test]
            fn check_type_attributes() {{
                initialize_kb();
                assert_eq!({name}::archetype().{added_attributes_f}(), {introduced_attributes});
                assert_eq!({name}::archetype().{all_attributes_f}(), {all_attributes});
            }}

            #[test]
            fn from_node_id() {{
                initialize_kb();
                let concept = {name}::new();
                let concept_copy = {name}::from(concept.id());
                assert_eq!(concept.id(), concept_copy.id());
            }}

            #[test]
            fn from_name() {{
                initialize_kb();
                let mut concept = {name}::new();
                concept.{internal_name_setter}("A"{internal_name_suffix});
                assert_eq!({name}::try_from("A").map(|c| c.id()), Ok(concept.id()));
                assert!({name}::try_from("B").is_err());
            }}

            #[test]
            fn test_wrapper_implemented() {{
                initialize_kb();
                let concept = {name}::new();
                assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
            }}"#,
            name = cfg.this.name,
            internal_name_setter = cfg.internal_name_cfg.setter,
            internal_name_suffix = cfg.internal_name_cfg.suffix,
            added_attributes_f = cfg.internal_name_cfg.added_attributes,
            all_attributes_f = cfg.internal_name_cfg.all_attributes,
            introduced_attributes = cfg.introduced_attributes,
            all_attributes = cfg.all_attributes,
        },
    })));
    test_frag
}

/// Returns a file fragment, which may be appended to further.
pub fn tao_file_fragment(cfg: &TaoConfig) -> FileFragment {
    let mut file = FileFragment::default();
    file.set_self_import(cfg.this.import.clone());
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(tao_test_fragment(cfg))));
    file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_internal_name_used() {
        let code = tao_file_fragment(&TaoConfig {
            internal_name_cfg: InternalNameConfig::DEFAULT,
            ..TaoConfig::default()
        })
        .generate_code();
        assert!(code.contains(".set_internal_name("));
        assert!(!code.contains(".set_internal_name_str("));
        assert!(code.contains(".internal_name("));
        assert!(!code.contains(".internal_name_str("));
        assert!(code.contains(".to_owned()"));
    }

    #[test]
    fn test_new_yin_internal_name_used() {
        let code = tao_file_fragment(&TaoConfig {
            internal_name_cfg: InternalNameConfig::YIN_AT_LEAST_0_1_1,
            ..TaoConfig::default()
        })
        .generate_code();
        assert!(!code.contains(".set_internal_name("));
        assert!(code.contains(".set_internal_name_str("));
        assert!(!code.contains(".internal_name("));
        assert!(code.contains(".internal_name_str("));
        assert!(!code.contains(".to_owned()"));
    }
}
