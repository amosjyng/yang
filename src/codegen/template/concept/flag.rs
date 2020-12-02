use crate::codegen::template::basic::{
    AtomicFragment, FileFragment, FunctionFragment, ImplementationFragment, ItemDeclarationAPI,
    SelfReference,
};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Prefix for flag setter function name.
const SETTER_PREFIX: &str = "mark_";
/// Prefix for flag getter function name.
const GETTER_PREFIX: &str = "is_";

/// Config values at the time of Flag getter/setter code generation.
pub struct FlagConfig {
    /// The public name to serve as a basis for the getter/setter function names.
    pub property_name: Rc<str>,
    /// Whether or not the getters and setters should be marked public. False if this is to be
    /// defined within a trait or an impl for a trait, true if defined within a pure impl.
    pub public: bool,
    /// An explanation of what this property is. Will be prepended by different strings for the
    /// setter and getter documentation.
    pub doc: Rc<str>,
    /// Concept representing the flag.
    pub flag: StructConfig,
    /// Concept representing the owner of the flag.
    pub owner_type: StructConfig,
    /// Whether or not the flag will be passed on to the owner's children via the `Inherits`
    /// attribute.
    pub hereditary: bool,
}

impl Default for FlagConfig {
    fn default() -> Self {
        Self {
            property_name: Rc::from(""),
            public: false,
            doc: Rc::from(""),
            flag: StructConfig::default(),
            owner_type: StructConfig::default(),
            hereditary: true,
        }
    }
}

/// Get the setter fragment for the flag.
fn setter_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("{}{}", SETTER_PREFIX, cfg.property_name));
    if cfg.public {
        f.mark_as_public();
    }
    f.document(format!("Mark this as {}", cfg.doc));
    f.set_self_reference(SelfReference::Mutable);
    f.add_import(cfg.flag.import.clone());
    f.add_import("zamm_yin::tao::archetype::ArchetypeTrait".to_owned());
    f.add_import("zamm_yin::tao::form::FormTrait".to_owned());
    f.add_import("zamm_yin::node_wrappers::BaseNodeTrait".to_owned());
    f.append(Rc::new(RefCell::new(AtomicFragment::new(format!(
        "self.essence_mut().add_flag({}::TYPE_ID);",
        cfg.flag.name
    )))));
    f
}

/// Get the getter fragment for the flag.
fn getter_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("{}{}", GETTER_PREFIX, cfg.property_name));
    if cfg.public {
        f.mark_as_public();
    }
    f.document(format!("Whether this is marked as {}", cfg.doc));
    f.set_self_reference(SelfReference::Immutable);
    f.set_return("bool".to_owned());
    f.add_import(cfg.flag.import.clone());
    f.add_import("zamm_yin::tao::archetype::ArchetypeTrait".to_owned());
    f.add_import("zamm_yin::tao::form::FormTrait".to_owned());
    f.add_import("zamm_yin::node_wrappers::BaseNodeTrait".to_owned());
    if cfg.hereditary {
        f.append(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "self.essence().has_flag({}::TYPE_ID)",
            cfg.flag.name
        )))));
    } else {
        f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc!(
            "
            self.essence()
                .inheritance_wrapper()
                .base_wrapper()
                .has_flag({}::TYPE_ID)",
            cfg.flag.name
        )))));
    }
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

/// Test that the getter and setter work as intended when inherited.
fn test_inheritance_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let inheritance_name = if cfg.hereditary {
        "inheritance"
    } else {
        "non_inheritance"
    };
    let inheritance_check = if cfg.hereditary { "" } else { "!" };
    let mut f = FunctionFragment::new(format!("test_{}_{}", cfg.property_name, inheritance_name));
    f.mark_as_test();
    f.add_import("crate::tao::initialize_kb".to_owned());
    f.add_import(cfg.owner_type.import.clone());
    f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        initialize_kb();
        let new_type = {owner}::archetype().individuate_as_archetype();
        let new_instance = {owner}::from(new_type.individuate_as_form().id());
        assert!(!new_instance.{getter}{property}());

        {owner}::from(new_type.id()).{setter}{property}();
        assert!({inheritance}new_instance.{getter}{property}());
    ", owner = cfg.owner_type.name,
        getter = GETTER_PREFIX,
        setter = SETTER_PREFIX,
        property = cfg.property_name,
        inheritance = inheritance_check,
    }))));
    f
}

/// Add these flags to an implementation and its corresponding test module.
pub fn add_flag_to_impl(
    cfg: &FlagConfig,
    implementation: &mut ImplementationFragment,
    file: &mut FileFragment,
) {
    implementation.append(Rc::new(RefCell::new(getter_fragment(cfg))));
    implementation.append(Rc::new(RefCell::new(setter_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(test_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(test_inheritance_fragment(cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    fn test_config() -> FlagConfig {
        FlagConfig {
            public: false,
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
            hereditary: true,
        }
    }

    #[test]
    fn test_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&test_config()).body(80),
            indoc! {"
                /// Mark this as newly defined as part of the current build.
                fn mark_newly_defined(&mut self) {
                    self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
                }"}
        );
    }

    #[test]
    fn test_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&test_config()).body(80),
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
            test_fragment(&test_config()).body(80),
            indoc! {"
                #[test]
                fn test_mark_and_check_newly_defined() {
                    initialize_kb();
                    let mut new_instance = Tao::new();
                    assert!(!new_instance.is_newly_defined());
            
                    new_instance.mark_newly_defined();
                    assert!(new_instance.is_newly_defined());
                }"}
        );
    }

    #[test]
    fn test_test_inheritance_fragment_body() {
        assert_eq!(
            test_inheritance_fragment(&test_config()).body(80),
            indoc! {"
                #[test]
                fn test_newly_defined_inheritance() {
                    initialize_kb();
                    let new_type = Tao::archetype().individuate_as_archetype();
                    let new_instance = Tao::from(new_type.individuate_as_form().id());
                    assert!(!new_instance.is_newly_defined());

                    Tao::from(new_type.id()).mark_newly_defined();
                    assert!(new_instance.is_newly_defined());
                }"}
        );
    }
}
