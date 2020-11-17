use super::form::form_fragment;
use super::tao::tao_test_fragment;
use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct AttributeFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// Attribute's owner archetype.
    pub owner_type: StructConfig,
    /// Attribute's owner form.
    pub owner_form: StructConfig,
    /// Attribute's value archetype.
    pub value_type: StructConfig,
    /// Attribute's value form.
    pub value_form: StructConfig,
}

/// Get the attribute body fragment.
pub fn attribute_fragment(cfg: &AttributeFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            format!(
                "{}::tao::relation::attribute::AttributeTrait",
                cfg.tao_cfg.yin_crate
            ),
            cfg.owner_form.import.clone(),
            cfg.value_form.import.clone(),
        ],
        atom: formatdoc! {r#"
            impl AttributeTrait for {name} {{
                type OwnerForm = {owner_form};
                type ValueForm = {value_form};
            }}"#, name = cfg.tao_cfg.this.name,
            owner_form = cfg.owner_form.name,
            value_form = cfg.value_form.name,
        },
    }
}

/// Get the attribute test fragment.
pub fn attribute_test_fragment(cfg: &AttributeFormatConfig) -> AtomicFragment {
    let mut imports = vec![
        format!(
            "{}::tao::archetype::ArchetypeFormTrait",
            cfg.tao_cfg.yin_crate
        ),
        format!(
            "{}::tao::archetype::AttributeArchetypeFormTrait",
            cfg.tao_cfg.yin_crate
        ),
    ];
    // there's a chance the form is the same as the type, in which case it will have gotten
    // imported above already
    if cfg.owner_type.name != cfg.owner_form.name {
        imports.push(cfg.owner_type.import.clone());
    }
    // there's a chance the form is the same as the type, in which case it will have gotten
    // imported above already
    if cfg.value_type.name != cfg.value_form.name {
        imports.push(cfg.value_type.import.clone());
    }

    AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            #[test]
            fn check_attribute_constraints() {{
                initialize_kb();
                assert_eq!(
                    {name}::archetype().owner_archetype(),
                    {owner_type}::archetype().as_archetype()
                );
                assert_eq!(
                    {name}::archetype().value_archetype(),
                    {value_type}::archetype().as_archetype()
                );
            }}

            #[test]
            fn get_owner() {{
                initialize_kb();
                let mut instance = {name}::individuate();
                let owner_of_instance = {owner_type}::individuate();
                instance.set_owner(&owner_of_instance);
                assert_eq!(instance.owner(), Some(owner_of_instance));
                assert_eq!(instance.value(), None);
            }}
        
            #[test]
            fn get_value() {{
                initialize_kb();
                let mut instance = {name}::individuate();
                let value_of_instance = {value_type}::individuate();
                instance.set_value(&value_of_instance);
                assert_eq!(instance.owner(), None);
                assert_eq!(instance.value(), Some(value_of_instance));
            }}"#, name = cfg.tao_cfg.this.name,
            owner_type = cfg.owner_type.name,
        value_type = cfg.value_type.name},
    }
}

/// Generate code for an Attribute config.
pub fn code_attribute(cfg: &AttributeFormatConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(form_fragment(&cfg.tao_cfg))));
    file.append(Rc::new(RefCell::new(attribute_fragment(&cfg))));
    let mut test_frag = tao_test_fragment(&cfg.tao_cfg);
    test_frag.append(Rc::new(RefCell::new(attribute_test_fragment(&cfg))));
    file.set_tests(Rc::new(RefCell::new(test_frag)));
    file.generate_code()
}
