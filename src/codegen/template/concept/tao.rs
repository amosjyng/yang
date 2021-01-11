use super::util::{add_assert, add_assert_frags, new_kb_test};
use crate::codegen::template::basic::{
    AppendedFragment, AtomicFragment, FileFragment, ImplementationFragment, VecFragment,
};
use crate::codegen::StructConfig;
use indoc::{formatdoc, indoc};
use std::cell::RefCell;
use std::rc::Rc;

/// Templating config values for all concepts.
#[derive(Clone)]
pub struct TaoConfig {
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
    /// The archetype used to represent the meta-object for this struct.
    pub archetype: StructConfig,
    /// List of attributes this class has.
    pub all_attributes: Vec<String>,
    /// Imports for above list of introduced attributes.
    pub all_attribute_imports: Vec<String>,
    /// List of attributes this class introduced.
    pub introduced_attributes: Vec<String>,
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
            form: StructConfig::default(),
            parent_name: "Tao".to_owned(),
            parent_import: "tao::Tao".to_owned(),
            archetype: StructConfig::new("crate::tao::archetype::Archetype".to_owned()),
            all_attributes: vec![],
            all_attribute_imports: vec![],
            introduced_attributes: vec![],
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
        cfg.archetype.import.clone(),
        "zamm_yin::node_wrappers::debug_wrapper".to_owned(),
        "zamm_yin::node_wrappers::FinalNode".to_owned(),
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
            
            impl ArchetypeTrait for {name} {{
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
            archetype = cfg.archetype.name,
            id = cfg.id
        },
    }
}

/// Get the Tao test fragment
fn tao_test_fragment(cfg: &TaoConfig) -> AppendedFragment {
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

    let check_type_created = new_kb_test(&mut test_frag, "check_type_created");
    add_assert(
        &check_type_created,
        format!("{}::archetype().id()", name),
        format!("{}::TYPE_ID", name),
    );
    add_assert(
        &check_type_created,
        format!("{name}::archetype().internal_name()", name = name),
        format!("Some(Rc::from({name}::TYPE_NAME))", name = name),
    );

    let from_name = new_kb_test(&mut test_frag, "from_name");
    from_name
        .borrow_mut()
        .append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc!(
            r#"
            let mut concept = {name}::new();
            concept.set_internal_name("A");"#,
            name = name
        )))));
    add_assert(
        &from_name,
        format!(r#"{}::try_from("A").map(|c| c.id())"#, name),
        "Ok(concept.id())".to_owned(),
    );
    from_name
        .borrow_mut()
        .append(Rc::new(RefCell::new(AtomicFragment::new(format!(
            r#"assert!({}::try_from("B").is_err());"#,
            name
        )))));

    let check_type_attributes = new_kb_test(&mut test_frag, "check_type_attributes");
    let mut introduced_attrs = VecFragment::new();
    for attr in &cfg.introduced_attributes {
        introduced_attrs.add_element_str(&attr);
    }
    add_assert_frags(
        &check_type_attributes,
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "{name}::archetype().added_attributes()",
            name = name
        )))),
        Rc::new(RefCell::new(introduced_attrs)),
    );
    let mut all_attrs = VecFragment::new();
    for attr in &cfg.all_attributes {
        all_attrs.add_element_str(&attr);
    }
    add_assert_frags(
        &check_type_attributes,
        Rc::new(RefCell::new(AtomicFragment::new(format!(
            "{name}::archetype().attributes()",
            name = name
        )))),
        Rc::new(RefCell::new(all_attrs)),
    );

    test_frag.append(Rc::new(RefCell::new(AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            #[test]
            fn from_node_id() {{
                initialize_kb();
                let concept = {name}::new();
                let concept_copy = {name}::from(concept.id());
                assert_eq!(concept.id(), concept_copy.id());
            }}

            #[test]
            fn test_wrapper_implemented() {{
                initialize_kb();
                let concept = {name}::new();
                assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
            }}"#,
            name = cfg.this.name,
        },
    })));
    test_frag
}

fn deref_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("std::ops::Deref".to_owned()),
        cfg.this.clone(),
    );
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec!["zamm_yin::node_wrappers::FinalNode".to_owned()],
        atom: "type Target = FinalNode;".to_owned(),
    })));
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![],
        atom: indoc! {"
            fn deref(&self) -> &Self::Target {
                &self.base
            }"}
        .to_owned(),
    })));
    implementation.mark_same_file_as_struct();
    implementation
}

fn deref_mut_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("std::ops::DerefMut".to_owned()),
        cfg.this.clone(),
    );
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![],
        atom: indoc! {"
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }"}
        .to_owned(),
    })));
    implementation.mark_same_file_as_struct();
    implementation
}

/// Returns a file fragment, which may be appended to further.
pub fn tao_file_fragment(cfg: &TaoConfig) -> FileFragment {
    let mut file = FileFragment::default();
    file.set_self_import(cfg.this.import.clone());
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.append(Rc::new(RefCell::new(deref_fragment(cfg))));
    file.append(Rc::new(RefCell::new(deref_mut_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(tao_test_fragment(cfg))));
    file
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;

    fn test_cfg() -> TaoConfig {
        TaoConfig {
            this: StructConfig {
                name: String::from("MyConcept"),
                ..StructConfig::default()
            },
            ..TaoConfig::default()
        }
    }

    #[test]
    fn test_default_internal_name_used() {
        let code = tao_file_fragment(&TaoConfig::default()).generate_code();
        assert!(code.contains(".set_internal_name("));
        assert!(!code.contains(".set_internal_name_str("));
        assert!(code.contains(".internal_name("));
        assert!(!code.contains(".internal_name_str("));
        assert!(!code.contains(".to_owned()"));
    }

    #[test]
    fn test_deref_fragment() {
        assert_eq!(
            deref_fragment(&test_cfg()).body(80),
            indoc! {"
                impl Deref for MyConcept {
                    type Target = FinalNode;
                
                    fn deref(&self) -> &Self::Target {
                        &self.base
                    }
                }"}
        );
    }

    #[test]
    fn test_deref_mut_fragment() {
        assert_eq!(
            deref_mut_fragment(&test_cfg()).body(80),
            indoc! {"
                impl DerefMut for MyConcept {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.base
                    }
                }"}
        );
    }
}
