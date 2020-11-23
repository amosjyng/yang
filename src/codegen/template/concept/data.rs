use super::form::form_fragment;
use super::tao::TaoConfig;
use crate::codegen::template::basic::AtomicFragment;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
pub struct DataFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// Rust primitive that this concept represents.
    pub rust_primitive_name: Rc<str>,
    /// Rust code representation of the default value of this concept.
    pub default_value: Rc<str>,
}

impl Default for DataFormatConfig {
    fn default() -> Self {
        Self {
            tao_cfg: TaoConfig::default(),
            rust_primitive_name: Rc::from(""),
            default_value: Rc::from(""),
        }
    }
}

/// Get the body fragment for a data concept.
fn data_concept_fragment(cfg: &DataFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            format!("{}::node_wrappers::BaseNodeTrait", cfg.tao_cfg.yin_crate),
            format!(
                "{}::graph::value_wrappers::StrongValue",
                cfg.tao_cfg.yin_crate
            ),
            format!(
                "{}::graph::value_wrappers::unwrap_value",
                cfg.tao_cfg.yin_crate
            ),
            "std::rc::Rc".to_owned(),
        ],
        // we allow for the potential use of Rc<String> here right now because String is in fact a
        // proper Rust type just like any other, and it is too much trouble to craft a bespoke
        // implementation for it right now when we'll do a more proper job of allowing editing in
        // the future
        atom: formatdoc! {r#"
            impl {name} {{
                /// Set {primitive} value for this concept.
                pub fn set_value(&mut self, value: {primitive}) {{
                    self.essence_mut()
                        .set_value(Rc::new(StrongValue::new(value)));
                }}

                /// Retrieve {primitive}-valued StrongValue.
                #[allow(clippy::rc_buffer)]
                pub fn value(&self) -> Option<Rc<{primitive}>> {{
                    unwrap_value::<{primitive}>(self.essence().value())
                }}
            }}"#, name = cfg.tao_cfg.this.name,
            primitive = cfg.rust_primitive_name,
        },
    }
}

/// Get the data concept test fragment.
fn data_concept_test_fragment(cfg: &DataFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![],
        atom: formatdoc! {r#"
            #[test]
            fn get_value_none() {{
                initialize_kb();
                let concept = {name}::new();
                assert_eq!(concept.value(), None);
            }}
        
            #[test]
            fn get_value_some() {{
                initialize_kb();
                let mut concept = {name}::new();
                concept.set_value({sample_value});
                assert_eq!(concept.value(), Some(Rc::new({sample_value})));
            }}"#, name = cfg.tao_cfg.this.name,
                // todo: create a better sample value than the default. This will require an
                // understanding of what the types actually are and how to construct them.
                sample_value = cfg.default_value,
        },
    }
}

/// Generate code for a Data concept.
pub fn code_data_concept(cfg: &DataFormatConfig) -> String {
    let mut file = form_fragment(&cfg.tao_cfg);
    file.append(Rc::new(RefCell::new(data_concept_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(data_concept_test_fragment(cfg))));
    file.generate_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_string_output() {
        let code = code_data_concept(&DataFormatConfig {
            rust_primitive_name: Rc::from("String"),
            ..DataFormatConfig::default()
        });
        assert!(code.contains("String"));
        assert!(!code.contains("i64"));
    }

    #[test]
    fn test_int_output() {
        let code = code_data_concept(&DataFormatConfig {
            rust_primitive_name: Rc::from("i64"),
            ..DataFormatConfig::default()
        });
        // todo: assert no "String" in code after CommonNodeTrait gets automatically implemented
        assert!(code.contains("i64"));
    }
}
