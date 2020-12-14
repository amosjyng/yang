use super::util::kb_test_function;
use crate::codegen::template::basic::{
    AtomicFragment, FileFragment, FunctionCallFragment, FunctionFragment, ImplementationFragment,
    ItemDeclarationAPI, SelfReference,
};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Prefix for attribute setter function name.
const SETTER_PREFIX: &str = "set_";

struct PrimitiveValueConfig<'a> {
    pub value: String,
    pub value_get: &'a str,
    pub value_set: &'a str,
}

fn primitive_config(attr_cfg: &AttributePropertyConfig) -> PrimitiveValueConfig {
    match &attr_cfg.primitive_test_value {
        Some(primitive_value) => PrimitiveValueConfig {
            value: primitive_value.to_string(),
            value_set: "value.clone()",
            value_get: "Rc::new(value)",
        },
        None => PrimitiveValueConfig {
            value: format!("{}::new()", attr_cfg.value_type.name),
            value_set: "&value",
            value_get: "value",
        },
    }
}

/// Config values at the time of Attribute getter/setter code generation.
pub struct AttributePropertyConfig {
    /// The public name to serve as a basis for the getter/setter function names.
    pub property_name: Rc<str>,
    /// Whether or not the getters and setters should be marked public. False if this is to be
    /// defined within a trait or an impl for a trait, true if defined within a pure impl.
    pub public: bool,
    /// An explanation of what this property is. Will be prepended by different strings for the
    /// setter and getter documentation.
    pub doc: Rc<str>,
    /// Concept representing the attribute.
    pub attr: StructConfig,
    /// Concept representing the owner of the attribute.
    pub owner_type: StructConfig,
    /// Concept representing the value of the attribute.
    pub value_type: StructConfig,
    /// The Rust primitive that this represents.
    pub rust_primitive: Option<Rc<str>>,
    /// Dummy test value to set the primitive to.
    pub primitive_test_value: Option<Rc<str>>,
    /// Whether or not the flag will be passed on to the owner's children via the `Inherits`
    /// attribute.
    pub hereditary: bool,
    /// Whether or not this attribute can contain multiple values.
    pub multi_valued: bool,
}

impl Default for AttributePropertyConfig {
    fn default() -> Self {
        Self {
            property_name: Rc::from(""),
            public: false,
            doc: Rc::from(""),
            attr: StructConfig::default(),
            owner_type: StructConfig::default(),
            value_type: StructConfig::default(),
            rust_primitive: None,
            primitive_test_value: None,
            hereditary: true,
            multi_valued: false,
        }
    }
}

fn getter_name(cfg: &AttributePropertyConfig) -> Rc<str> {
    if cfg.multi_valued {
        // todo: generate plurals more properly, and add manual override option
        let plural = if cfg.property_name.ends_with('s') {
            format!("{}es", cfg.property_name)
        } else {
            format!("{}s", cfg.property_name)
        };
        Rc::from(plural.as_str())
    } else {
        cfg.property_name.clone()
    }
}

fn setter_name(cfg: &AttributePropertyConfig) -> String {
    if cfg.multi_valued {
        format!("add_{}", cfg.property_name)
    } else {
        format!("{}{}", SETTER_PREFIX, cfg.property_name)
    }
}

/// Get the setter fragment for the attribute property.
fn setter_fragment(cfg: &AttributePropertyConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(setter_name(cfg));

    f.add_import(cfg.attr.import.clone());
    f.add_import(cfg.value_type.import.clone());
    f.add_import("zamm_yin::tao::archetype::ArchetypeTrait".to_owned());
    f.add_import("zamm_yin::tao::form::FormTrait".to_owned());
    f.add_import("zamm_yin::node_wrappers::BaseNodeTrait".to_owned());
    f.add_import("zamm_yin::node_wrappers::CommonNodeTrait".to_owned());

    if cfg.public {
        f.mark_as_public();
    }
    if cfg.multi_valued {
        f.document(format!("Add one of {}", cfg.doc));
    } else {
        f.document(format!("Set {}", cfg.doc));
    }
    f.set_self_reference(SelfReference::Mutable);

    let arg_name = cfg.property_name.to_string();
    match &cfg.rust_primitive {
        Some(primitive) => {
            f.add_arg(arg_name, primitive.to_string());
            f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
                let mut value_concept = {value_concept}::new();
                value_concept.set_value({value});",
                value_concept = cfg.value_type.name,
                value = cfg.property_name
            }))));
        }
        None => f.add_arg(arg_name, format!("&{}", cfg.value_type.name)),
    };

    let final_value = if cfg.rust_primitive.is_some() {
        "value_concept.essence()".to_owned()
    } else {
        format!("{}.essence()", cfg.property_name)
    };
    let mut add_outgoing = FunctionCallFragment::new(AtomicFragment::new(
        "self.essence_mut().add_outgoing".to_owned(),
    ));
    add_outgoing.add_argument(Rc::new(RefCell::new(AtomicFragment::new(format!(
        "{}::TYPE_ID",
        cfg.attr.name
    )))));
    add_outgoing.add_argument(Rc::new(RefCell::new(AtomicFragment::new(final_value))));
    f.append(Rc::new(RefCell::new(add_outgoing)));
    f
}

/// Get the getter fragment for the attribute property.
fn getter_fragment(cfg: &AttributePropertyConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(getter_name(cfg).to_string());

    f.add_import(cfg.attr.import.clone());
    f.add_import("zamm_yin::tao::archetype::ArchetypeTrait".to_owned());
    f.add_import("zamm_yin::tao::form::FormTrait".to_owned());
    f.add_import("zamm_yin::node_wrappers::BaseNodeTrait".to_owned());

    if cfg.public {
        f.mark_as_public();
    }
    f.document(format!("Get {}", cfg.doc));
    if cfg.rust_primitive.is_some() {
        f.add_attribute("allow(clippy::rc_buffer)".to_owned());
    }
    f.set_self_reference(SelfReference::Immutable);
    let base_return_type = match &cfg.rust_primitive {
        Some(primitive) => {
            f.add_import("std::rc::Rc".to_owned());
            format!("Rc<{}>", primitive)
        }
        None => cfg.value_type.name.clone(),
    };
    if cfg.multi_valued {
        f.set_return(format!("Vec<{}>", base_return_type));
    } else {
        f.set_return(format!("Option<{}>", base_return_type));
    }

    let nonhereditary_access = if cfg.hereditary {
        ""
    } else {
        "\n    .base_wrapper()"
    };
    let primitive_map = if cfg.rust_primitive.is_some() {
        ".value().unwrap()"
    } else {
        ""
    };
    let collection = if cfg.multi_valued {
        ".into_iter()"
    } else {
        ".first()"
    };
    let post_collection = if cfg.multi_valued {
        "\n    .collect()"
    } else {
        ""
    };

    f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        self.essence(){inheritance}
            .outgoing_nodes({attr}::TYPE_ID)
            {collection}
            .map(|f| {value_type}::from(f.id()){primitive_map}){post_collection}",
        inheritance = nonhereditary_access,
        attr = cfg.attr.name,
        value_type = cfg.value_type.name,
        primitive_map = primitive_map,
        collection = collection,
        post_collection = post_collection
    }))));

    f
}

/// Test that the getter and setter work as intended.
fn test_fragment(cfg: &AttributePropertyConfig) -> FunctionFragment {
    let setter = setter_name(cfg);
    let getter = getter_name(cfg);

    let mut f = kb_test_function(&format!("test_set_and_get_{}", cfg.property_name));
    f.add_import(cfg.owner_type.import.clone());
    if cfg.rust_primitive.is_none() {
        // if some, will use that directly instead of the concept
        f.add_import(cfg.value_type.import.clone());
    }
    let value_cfg = primitive_config(cfg);
    let empty_value = if cfg.multi_valued {
        "vec![]".to_owned()
    } else {
        "None".to_owned()
    };
    let final_value_get = if cfg.multi_valued {
        format!("vec![{}]", value_cfg.value_get)
    } else {
        format!("Some({})", value_cfg.value_get)
    };
    // todo: only clone when it's not a Copy type, to avoid the clone_on_copy warning. The thing
    // is, this is specific to Rust, so it should be Yang-only knowledge.
    f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        let mut new_instance = {owner}::new();
        assert_eq!(new_instance.{getter}(), {empty});

        let value = {value};
        #[allow(clippy::clone_on_copy)]
        new_instance.{setter}({value_set});
        assert_eq!(new_instance.{getter}(), {value_get});",
        owner = cfg.owner_type.name,
        getter = getter,
        setter = setter,
        empty = empty_value,
        value = value_cfg.value,
        value_set = value_cfg.value_set,
        value_get = final_value_get,
    }))));
    f
}

/// Test that the getter and setter work as intended when inherited.
fn test_inheritance_fragment(cfg: &AttributePropertyConfig) -> FunctionFragment {
    let setter = setter_name(cfg);
    let getter = getter_name(cfg);

    let empty_value = if cfg.multi_valued {
        "vec![]".to_owned()
    } else {
        "None".to_owned()
    };
    let inheritance_name = if cfg.hereditary {
        "inheritance"
    } else {
        "non_inheritance"
    };
    let value_cfg = primitive_config(cfg);
    let value_set = if cfg.primitive_test_value.is_some() && !cfg.hereditary {
        "value" // value won't get used again, so just use it directly
    } else {
        value_cfg.value_set
    };
    let inheritance_check = if cfg.hereditary {
        if cfg.multi_valued {
            format!("vec![{}]", value_cfg.value_get)
        } else {
            format!("Some({})", value_cfg.value_get)
        }
    } else {
        empty_value.clone()
    };
    let mut f = FunctionFragment::new(format!("test_{}_{}", cfg.property_name, inheritance_name));
    f.mark_as_test();
    f.add_import("crate::tao::initialize_kb".to_owned());
    f.add_import(cfg.owner_type.import.clone());
    // todo: only clone when it's not a Copy type, to avoid the clone_on_copy warning. The thing
    // is, this is specific to Rust, so it should be Yang-only knowledge.
    f.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        initialize_kb();
        let new_type = {owner}::archetype().individuate_as_archetype();
        let new_instance = {owner}::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.{getter}(), {empty});

        let value = {value};
        #[allow(clippy::clone_on_copy)]
        {owner}::from(new_type.id()).{setter}({value_set});
        assert_eq!(new_instance.{getter}(), {inheritance});
    ", owner = cfg.owner_type.name,
        getter = getter,
        setter = setter,
        empty = empty_value,
        inheritance = inheritance_check,
        value = value_cfg.value,
        value_set = value_set,
    }))));
    f
}

/// Add these flags to an implementation and its corresponding test module.
pub fn add_attr_to_impl(
    cfg: &AttributePropertyConfig,
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

    fn concept_attr_config() -> AttributePropertyConfig {
        AttributePropertyConfig {
            public: false,
            property_name: Rc::from("associated_crate"),
            doc: Rc::from("the crate associated with the struct."),
            attr: StructConfig {
                name: "AssociatedCrate".to_owned(),
                import: "crate::tao::relations::attribute::AssociatedCrate".to_owned(),
            },
            owner_type: StructConfig {
                name: "Form".to_owned(),
                import: "zamm_yin::tao::form::Form".to_owned(),
            },
            value_type: StructConfig {
                name: "Crate".to_owned(),
                import: "zamm_yin::tao::form::Crate".to_owned(),
            },
            rust_primitive: None,
            primitive_test_value: None,
            hereditary: true,
            multi_valued: false,
        }
    }

    fn primitive_attr_config() -> AttributePropertyConfig {
        AttributePropertyConfig {
            rust_primitive: Some(Rc::from("String")),
            primitive_test_value: Some(Rc::from("String::new()")),
            ..concept_attr_config()
        }
    }

    fn multi_valued_config() -> AttributePropertyConfig {
        AttributePropertyConfig {
            doc: Rc::from("the crates associated with the struct."),
            multi_valued: true,
            ..concept_attr_config()
        }
    }

    #[test]
    fn test_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&concept_attr_config()).body(80),
            indoc! {"
                /// Set the crate associated with the struct.
                fn set_associated_crate(&mut self, associated_crate: &Crate) {
                    self.essence_mut().add_outgoing(
                        AssociatedCrate::TYPE_ID,
                        associated_crate.essence(),
                    );
                }"}
        );
    }

    #[test]
    fn test_primitive_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&primitive_attr_config()).body(80),
            indoc! {"
                /// Set the crate associated with the struct.
                fn set_associated_crate(&mut self, associated_crate: String) {
                    let mut value_concept = Crate::new();
                    value_concept.set_value(associated_crate);
                    self.essence_mut().add_outgoing(
                        AssociatedCrate::TYPE_ID,
                        value_concept.essence(),
                    );
                }"}
        );
    }

    #[test]
    fn test_multi_valued_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&multi_valued_config()).body(80),
            indoc! {"
                /// Add one of the crates associated with the struct.
                fn add_associated_crate(&mut self, associated_crate: &Crate) {
                    self.essence_mut().add_outgoing(
                        AssociatedCrate::TYPE_ID,
                        associated_crate.essence(),
                    );
                }"}
        );
    }

    #[test]
    fn test_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&concept_attr_config()).body(80),
            indoc! {"
                /// Get the crate associated with the struct.
                fn associated_crate(&self) -> Option<Crate> {
                    self.essence()
                        .outgoing_nodes(AssociatedCrate::TYPE_ID)
                        .first()
                        .map(|f| Crate::from(f.id()))
                }"}
        );
    }

    #[test]
    fn test_primitive_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&primitive_attr_config()).body(80),
            indoc! {"
                /// Get the crate associated with the struct.
                #[allow(clippy::rc_buffer)]
                fn associated_crate(&self) -> Option<Rc<String>> {
                    self.essence()
                        .outgoing_nodes(AssociatedCrate::TYPE_ID)
                        .first()
                        .map(|f| Crate::from(f.id()).value().unwrap())
                }"}
        );
    }

    #[test]
    fn test_primitive_nonhereditary_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&AttributePropertyConfig {
                hereditary: false,
                ..primitive_attr_config()
            })
            .body(80),
            indoc! {"
                /// Get the crate associated with the struct.
                #[allow(clippy::rc_buffer)]
                fn associated_crate(&self) -> Option<Rc<String>> {
                    self.essence()
                        .base_wrapper()
                        .outgoing_nodes(AssociatedCrate::TYPE_ID)
                        .first()
                        .map(|f| Crate::from(f.id()).value().unwrap())
                }"}
        );
    }

    #[test]
    fn test_multi_valued_getter_fragment_body() {
        assert_eq!(
            getter_fragment(&multi_valued_config()).body(80),
            indoc! {"
                /// Get the crates associated with the struct.
                fn associated_crates(&self) -> Vec<Crate> {
                    self.essence()
                        .outgoing_nodes(AssociatedCrate::TYPE_ID)
                        .into_iter()
                        .map(|f| Crate::from(f.id()))
                        .collect()
                }"}
        );
    }

    #[test]
    fn test_test_fragment_body() {
        assert_eq!(
            test_fragment(&concept_attr_config()).body(80),
            indoc! {"
                #[test]
                fn test_set_and_get_associated_crate() {
                    initialize_kb();
                    let mut new_instance = Form::new();
                    assert_eq!(new_instance.associated_crate(), None);

                    let value = Crate::new();
                    #[allow(clippy::clone_on_copy)]
                    new_instance.set_associated_crate(&value);
                    assert_eq!(new_instance.associated_crate(), Some(value));
                }"}
        );
    }

    #[test]
    fn test_primitive_test_fragment_body() {
        assert_eq!(
            test_fragment(&primitive_attr_config()).body(80),
            indoc! {"
                #[test]
                fn test_set_and_get_associated_crate() {
                    initialize_kb();
                    let mut new_instance = Form::new();
                    assert_eq!(new_instance.associated_crate(), None);

                    let value = String::new();
                    #[allow(clippy::clone_on_copy)]
                    new_instance.set_associated_crate(value.clone());
                    assert_eq!(new_instance.associated_crate(), Some(Rc::new(value)));
                }"}
        );
    }

    #[test]
    fn test_test_inheritance_fragment_body() {
        assert_eq!(
            test_inheritance_fragment(&concept_attr_config()).body(80),
            indoc! {"
                #[test]
                fn test_associated_crate_inheritance() {
                    initialize_kb();
                    let new_type = Form::archetype().individuate_as_archetype();
                    let new_instance = Form::from(new_type.individuate_as_form().id());
                    assert_eq!(new_instance.associated_crate(), None);

                    let value = Crate::new();
                    #[allow(clippy::clone_on_copy)]
                    Form::from(new_type.id()).set_associated_crate(&value);
                    assert_eq!(new_instance.associated_crate(), Some(value));
                }"}
        );
    }

    #[test]
    fn test_primitive_test_inheritance_fragment_body() {
        assert_eq!(
            test_inheritance_fragment(&primitive_attr_config()).body(80),
            indoc! {"
                #[test]
                fn test_associated_crate_inheritance() {
                    initialize_kb();
                    let new_type = Form::archetype().individuate_as_archetype();
                    let new_instance = Form::from(new_type.individuate_as_form().id());
                    assert_eq!(new_instance.associated_crate(), None);

                    let value = String::new();
                    #[allow(clippy::clone_on_copy)]
                    Form::from(new_type.id()).set_associated_crate(value.clone());
                    assert_eq!(new_instance.associated_crate(), Some(Rc::new(value)));
                }"}
        );
    }
}
