use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment, ImplementationFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct FormFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// All ancestors of the concept.
    pub ancestors: Vec<StructConfig>,
}

fn form_impl_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("zamm_yin::tao::form::FormTrait".to_owned()),
        cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    implementation
}

fn deref_fragment(this_name: &str, ancestor: &StructConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig {
            name: format!("From<{}>", this_name),
            import: "std::convert::From".to_owned(),
        },
        ancestor.clone(),
    );
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![ancestor.import.clone()],
        atom: formatdoc! {"
            fn from(this: {this}) -> {parent} {{
                {parent}::from(this.base)
            }}",
        this = this_name,
        parent = ancestor.name},
    })));
    implementation.mark_same_file_as_struct();
    implementation
}

/// Add the form fragment to a file.
pub fn add_form_fragment(cfg: &FormFormatConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(form_impl_fragment(&cfg.tao_cfg))));
    for ancestor in &cfg.ancestors {
        file.append(Rc::new(RefCell::new(deref_fragment(
            &cfg.tao_cfg.this.name,
            &ancestor,
        ))));
    }
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
            deref_fragment(
                "MyConcept",
                &StructConfig::new("crate::MyParent".to_owned())
            )
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
