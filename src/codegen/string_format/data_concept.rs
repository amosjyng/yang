use super::fragments::{AtomicFragment, FileFragment};
use super::tao::{tao_fragment, tao_test_fragment};
use super::DataFormatConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get the body fragment for a data concept.
pub fn data_concept_fragment(cfg: &DataFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            format!("{}::node_wrappers::BaseNodeTrait", cfg.tao_cfg.yin_crate),
            format!("{}::graph::value_wrappers::KBValue", cfg.tao_cfg.yin_crate),
            format!(
                "{}::graph::value_wrappers::StrongValue",
                cfg.tao_cfg.yin_crate
            ),
        ],
        atom: formatdoc! {r#"
            impl {name} {{
                /// Set {primitive} value for this concept.
                pub fn set_value(&mut self, value: {primitive}) {{
                    self.essence_mut()
                        .set_value(Rc::new(StrongValue::new(value)));
                }}

                /// Retrieve {primitive}-valued StrongValue.
                pub fn value(&self) -> Option<Rc<dyn KBValue>> {{
                    self.essence().value()
                }}
            }}"#, name = cfg.tao_cfg.name,
            primitive = cfg.rust_primitive_name,
        },
    }
}

/// Get the string concept test fragment.
pub fn string_concept_test_fragment(cfg: &DataFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![format!(
            "{}::graph::value_wrappers::unwrap_strong",
            cfg.tao_cfg.yin_crate
        )],
        atom: formatdoc! {r#"
            #[test]
            fn get_value_none() {{
                initialize_kb();
                let concept = {name}::individuate();
                assert_eq!(unwrap_strong::<{primitive}>(&concept.value()), None);
            }}
        
            #[test]
            fn get_value_some() {{
                initialize_kb();
                let mut concept = {name}::individuate();
                concept.set_value({sample_value});
                assert_eq!(
                    unwrap_strong::<{primitive}>(&concept.value()),
                    Some(&{sample_value})
                );
            }}"#, name = cfg.tao_cfg.name,
                primitive = cfg.rust_primitive_name,
                // todo: create a better sample value than the default. This will require an
                // understanding of what the types actually are and how to construct them.
                sample_value = cfg.default_value,
        },
    }
}

/// Generate code for a string concept.
pub fn code_data_concept(cfg: &DataFormatConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(tao_fragment(&cfg.tao_cfg))));
    file.append(Rc::new(RefCell::new(data_concept_fragment(cfg))));
    let mut test_frag = tao_test_fragment(&cfg.tao_cfg);
    test_frag.append(Rc::new(RefCell::new(string_concept_test_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(test_frag)));
    file.generate_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_string_output() {
        let code = code_data_concept(&DataFormatConfig {
            rust_primitive_name: Rc::new("String".to_owned()),
            ..DataFormatConfig::default()
        });
        assert!(code.contains("String"));
        assert!(!code.contains("i64"));
    }

    #[test]
    fn test_int_output() {
        let code = code_data_concept(&DataFormatConfig {
            rust_primitive_name: Rc::new("i64".to_owned()),
            ..DataFormatConfig::default()
        });
        // todo: assert no "String" in code after CommonNodeTrait gets automatically implemented
        assert!(code.contains("i64"));
    }
}
