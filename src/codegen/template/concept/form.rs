use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment, ImplementationFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

fn form_impl_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("zamm_yin::tao::form::FormTrait".to_owned()),
        cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    implementation
}

fn deref_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig {
            name: format!("From<{}>", cfg.this.name),
            import: "std::convert::From".to_owned(),
        },
        StructConfig {
            name: cfg.parent_name.clone(),
            import: cfg.parent_import.clone(),
        },
    );
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![cfg.parent_import.clone()],
        atom: formatdoc! {"
            fn from(this: {this}) -> {parent} {{
                {parent}::from(this.base)
            }}",
        this = cfg.this.name,
        parent = cfg.parent_name},
    })));
    implementation.mark_same_file_as_struct();
    implementation
}

/// Add the form fragment to a file.
pub fn add_form_fragment(cfg: &TaoConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(form_impl_fragment(cfg))));
    file.append(Rc::new(RefCell::new(deref_fragment(cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    #[test]
    fn test_form_fragment() {
        assert_eq!(
            form_impl_fragment(&TaoConfig {
                this: StructConfig {
                    name: "MyConcept".to_owned(),
                    ..StructConfig::default()
                },
                ..TaoConfig::default()
            })
            .body(80),
            "impl FormTrait for MyConcept {}"
        );
    }

    #[test]
    fn test_deref_fragment() {
        assert_eq!(
            deref_fragment(&TaoConfig {
                this: StructConfig {
                    name: "MyConcept".to_owned(),
                    ..StructConfig::default()
                },
                parent_name: "MyParent".to_owned(),
                ..TaoConfig::default()
            })
            .body(80),
            indoc! {"
                impl From<MyConcept> for MyParent {
                    fn from(this: MyConcept) -> MyParent {
                        MyParent::from(this.base)
                    }
                }"}
        );
    }
}
