use super::fragments::{AtomicFragment, FileFragment};
use super::tao::{tao_fragment, tao_test_fragment};
use super::FormatConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get the attribute body fragment.
pub fn attribute_fragment(cfg: &FormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            format!("{}::tao::Form", cfg.yin_crate),
            format!("{}::tao::attribute::AttributeTrait", cfg.yin_crate),
        ],
        atom: formatdoc! {r#"
            impl AttributeTrait for {name} {{
                type OwnerForm = Form;
                type ValueForm = Form;
            }}"#, name = cfg.name},
    }
}

/// Get the attribute test fragment.
pub fn attribute_test_fragment(cfg: &FormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: Vec::new(),
        atom: formatdoc! {r#"
            #[test]
            fn get_owner() {{
                initialize_kb();
                let mut instance = {name}::individuate();
                let owner_of_owner = {name}::individuate();
                instance.set_owner(&owner_of_owner.as_form());
                assert_eq!(instance.owner(), Some(owner_of_owner.as_form()));
                assert_eq!(instance.value(), None);
            }}
        
            #[test]
            fn get_value() {{
                initialize_kb();
                let mut instance = {name}::individuate();
                let value_of_owner = {name}::individuate();
                instance.set_value(&value_of_owner.as_form());
                assert_eq!(instance.owner(), None);
                assert_eq!(instance.value(), Some(value_of_owner.as_form()));
            }}"#, name = cfg.name},
    }
}

/// Generate code for an Attribute config.
pub fn code_attribute(cfg: &FormatConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.append(Rc::new(RefCell::new(attribute_fragment(cfg))));
    let mut test_frag = tao_test_fragment(cfg);
    test_frag.append(Rc::new(RefCell::new(attribute_test_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(test_frag)));
    file.generate_code()
}
