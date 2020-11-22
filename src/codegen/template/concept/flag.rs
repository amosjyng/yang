use crate::codegen::template::basic::{
    AtomicFragment, FunctionFragment, ItemDeclarationAPI, SelfReference,
};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Prefix for setter function name.
const SETTER_PREFIX: &str = "mark_as_";
/// Prefix for getter function name.
const GETTER_PREFIX: &str = "is_";

/// Config values at the time of Flag getter/setter code generation.
pub struct FlagConfig {
    /// The public name to serve as a basis for the getter/setter function names.
    pub property_name: Rc<str>,
    /// An explanation of what this property is. Will be prepended by different strings for the
    /// setter and getter documentation.
    pub doc: Rc<str>,
    /// Concept representing the flag.
    pub flag: StructConfig,
    /// Concept representing the owner of the flag.
    pub owner_type: StructConfig,
    /// String to use for the Yin crate.
    pub yin_crate: Rc<str>,
}

impl Default for FlagConfig {
    fn default() -> Self {
        Self {
            property_name: Rc::from(""),
            doc: Rc::from(""),
            flag: StructConfig::default(),
            owner_type: StructConfig::default(),
            yin_crate: Rc::from("zamm_yin"),
        }
    }
}

/// Get the setter fragment for the flag.
fn setter_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("{}{}", SETTER_PREFIX, cfg.property_name));
    f.document(format!("Mark this as {}", cfg.doc));
    f.set_self_reference(SelfReference::Mutable);
    f.add_import(cfg.flag.import.clone());
    f.add_import(format!("{}::tao::archetype::ArchetypeTrait", cfg.yin_crate));
    f.add_import(format!("{}::tao::form::FormTrait", cfg.yin_crate));
    f.append(Rc::new(RefCell::new(AtomicFragment::new(format!(
        "self.essence_mut().add_flag({}::TYPE_ID);",
        cfg.flag.name
    )))));
    f
}

/// Get the getter fragment for the flag.
fn getter_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("{}{}", GETTER_PREFIX, cfg.property_name));
    f.document(format!("Whether this is marked as {}", cfg.doc));
    f.set_self_reference(SelfReference::Immutable);
    f.set_return("bool".to_owned());
    f.add_import(cfg.flag.import.clone());
    f.add_import(format!("{}::tao::archetype::ArchetypeTrait", cfg.yin_crate));
    f.add_import(format!("{}::tao::form::FormTrait", cfg.yin_crate));
    f.append(Rc::new(RefCell::new(AtomicFragment::new(format!(
        "self.essence().has_flag({}::TYPE_ID)",
        cfg.flag.name
    )))));
    f
}

/// Test that the getter and setter work as intended.
fn test_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("test_mark_and_check_{}", cfg.property_name));
    f.mark_as_test();
    f.add_import("crate::tao::initialize_kb".to_owned());
    f.add_import(cfg.owner_type.import.clone());
    f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        initialize_kb();
        let mut new_instance = {owner}::new();
        assert!(!new_instance.{getter}{property}());

        new_instance.{setter}{property}();
        assert!(new_instance.{getter}{property}());
    ", owner = cfg.owner_type.name,
        getter = GETTER_PREFIX,
        setter = SETTER_PREFIX,
        property = cfg.property_name
    }))));
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    fn test_config() -> FlagConfig {
        FlagConfig {
            property_name: Rc::from("newly_defined"),
            doc: Rc::from("newly defined as part of the current build."),
            flag: StructConfig {
                name: "NewlyDefined".to_owned(),
                import: "crate::tao::relation::flag::NewlyDefined".to_owned(),
            },
            owner_type: StructConfig {
                name: "Tao".to_owned(),
                import: "zamm_yin::tao::Tao".to_owned(),
            },
            yin_crate: Rc::from("zamm_yin"),
        }
    }

    #[test]
    fn test_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&test_config()).body(),
            indoc! {"
                /// Mark this as newly defined as part of the current build.
                fn mark_as_newly_defined(&mut self) {
                    self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
                }"}
        );
    }

    #[test]
    fn test_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&test_config()).body(),
            indoc! {"
                /// Whether this is marked as newly defined as part of the current build.
                fn is_newly_defined(&self) -> bool {
                    self.essence().has_flag(NewlyDefined::TYPE_ID)
                }"}
        );
    }

    #[test]
    fn test_test_fragment_body() {
        assert_eq!(
            test_fragment(&test_config()).body(),
            indoc! {"
                #[test]
                fn test_mark_and_check_newly_defined() {
                    initialize_kb();
                    let mut new_instance = Tao::new();
                    assert!(!new_instance.is_newly_defined());
            
                    new_instance.mark_as_newly_defined();
                    assert!(new_instance.is_newly_defined());
                }"}
        );
    }
}
