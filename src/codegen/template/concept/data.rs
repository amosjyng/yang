use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment};
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
pub struct DataFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// Rust primitive that this concept represents, when referenced in an unboxed way.
    pub rust_primitive_unboxed_name: Rc<str>,
    /// Rust primitive that this concept represents, when referenced in a boxed way (e.g. inside an
    /// `Rc` or `Box`).
    pub rust_primitive_boxed_name: Rc<str>,
    /// Rust code representation of the default value of this concept.
    pub default_value: Rc<str>,
    /// Whether we should initialize the Rc explicitly. For backwards compatibility purposes.
    pub explicit_rc: bool,
}

impl Default for DataFormatConfig {
    fn default() -> Self {
        Self {
            tao_cfg: TaoConfig::default(),
            rust_primitive_unboxed_name: Rc::from(""),
            rust_primitive_boxed_name: Rc::from(""),
            default_value: Rc::from(""),
            explicit_rc: true,
        }
    }
}

/// Get the body fragment for a data concept.
fn data_concept_fragment(cfg: &DataFormatConfig) -> AtomicFragment {
    let strongvalue_init = if cfg.explicit_rc {
        format!(
            "StrongValue::new_rc(Rc::<{}>::from(value))",
            cfg.rust_primitive_boxed_name
        )
    } else {
        "StrongValue::new(value)".to_owned()
    };
    AtomicFragment {
        imports: vec![
            "zamm_yin::node_wrappers::BaseNodeTrait".to_owned(),
            "zamm_yin::graph::value_wrappers::StrongValue".to_owned(),
            "zamm_yin::graph::value_wrappers::unwrap_value".to_owned(),
            "std::rc::Rc".to_owned(),
        ],
        // we allow for the potential use of Rc<String> here right now because String is in fact a
        // proper Rust type just like any other, and it is too much trouble to craft a bespoke
        // implementation for it right now when we'll do a more proper job of allowing editing in
        // the future
        atom: formatdoc! {r#"
            impl {name} {{
                /// Set {boxed_primitive} value for this concept.
                pub fn set_value(&mut self, value: {unboxed_primitive}) {{
                    self.deref_mut().set_value(Rc::new({strongvalue_init}));
                }}

                /// Retrieve {boxed_primitive}-valued StrongValue.
                #[allow(clippy::rc_buffer)]
                pub fn value(&self) -> Option<Rc<{boxed_primitive}>> {{
                    unwrap_value::<{boxed_primitive}>(self.deref().value())
                }}
            }}"#, name = cfg.tao_cfg.this.name,
            unboxed_primitive = cfg.rust_primitive_unboxed_name,
            boxed_primitive = cfg.rust_primitive_boxed_name,
            strongvalue_init = strongvalue_init
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
                assert_eq!(concept.value(), Some(Rc::from({sample_value})));
            }}"#, name = cfg.tao_cfg.this.name,
                // todo: create a better sample value than the default. This will require an
                // understanding of what the types actually are and how to construct them.
                sample_value = cfg.default_value,
        },
    }
}

/// Generate code for a Data concept.
pub fn add_data_fragments(cfg: &DataFormatConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(data_concept_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(data_concept_test_fragment(cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_string_output() {
        let mut f = FileFragment::new();
        add_data_fragments(
            &DataFormatConfig {
                rust_primitive_unboxed_name: Rc::from("String"),
                rust_primitive_boxed_name: Rc::from("String"),
                ..DataFormatConfig::default()
            },
            &mut f,
        );
        let code = f.generate_code();
        assert!(code.contains("String"));
        assert!(!code.contains("i64"));
        assert!(code.contains("set_value"));
    }

    #[test]
    fn test_int_output() {
        let mut f = FileFragment::new();
        add_data_fragments(
            &DataFormatConfig {
                rust_primitive_unboxed_name: Rc::from("i64"),
                rust_primitive_boxed_name: Rc::from("i64"),
                ..DataFormatConfig::default()
            },
            &mut f,
        );
        let code = f.generate_code();
        // todo: assert no "String" in code after CommonNodeTrait gets automatically implemented
        assert!(code.contains("i64"));
        assert!(code.contains("set_value"));
    }
}
