use super::fragments::{AtomicFragment, FileFragment};
use super::tao::{tao_fragment, tao_test_fragment};
use super::FormatConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get the string concept body fragment.
pub fn string_concept_fragment(cfg: &FormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            format!("{}::node_wrappers::BaseNodeTrait", cfg.yin_crate),
            format!("{}::graph::value_wrappers::KBValue", cfg.yin_crate),
            format!("{}::graph::value_wrappers::StrongValue", cfg.yin_crate),
        ],
        atom: formatdoc! {r#"
            impl {name} {{
                /// Set String value for this concept.
                pub fn set_value(&mut self, value: String) {{
                    self.essence_mut()
                        .set_value(Rc::new(StrongValue::new(value)));
                }}

                /// Retrieve String-valued StrongValue.
                pub fn value(&self) -> Option<Rc<dyn KBValue>> {{
                    self.essence().value()
                }}
            }}"#, name = cfg.name},
    }
}

/// Get the string concept test fragment.
pub fn string_concept_test_fragment(cfg: &FormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![format!(
            "{}::graph::value_wrappers::unwrap_strong",
            cfg.yin_crate
        )],
        atom: formatdoc! {r#"
            #[test]
            fn get_value_none() {{
                initialize_kb();
                let concept = {name}::individuate();
                assert_eq!(unwrap_strong::<String>(&concept.value()), None);
            }}
        
            #[test]
            fn get_value_string() {{
                initialize_kb();
                let mut concept = {name}::individuate();
                concept.set_value("value".to_owned());
                assert_eq!(
                    unwrap_strong::<String>(&concept.value()),
                    Some(&"value".to_owned())
                );
            }}"#, name = cfg.name},
    }
}

/// Generate code for a string concept.
pub fn code_string_concept(cfg: &FormatConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    file.append(Rc::new(RefCell::new(string_concept_fragment(cfg))));
    let mut test_frag = tao_test_fragment(cfg);
    test_frag.append(Rc::new(RefCell::new(string_concept_test_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(test_frag)));
    file.generate_code()
}
