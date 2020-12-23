use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment, ImplementationFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Form code generation.
#[derive(Default)]
pub struct FormFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// Meta archetype specifically for this type, if one exists.
    pub meta_archetype: Option<StructConfig>,
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

/// Get the form extension fragment, if there is a meta object to be had.
fn form_extension_fragment(cfg: &FormFormatConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("zamm_yin::tao::form::FormExtension".to_owned()),
        cfg.tao_cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    let meta = cfg.meta_archetype.as_ref().unwrap();
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![meta.import.clone()],
        atom: format!("type MetaType = {};", meta.name),
    })));
    implementation
}

/// Add the form fragment to a file.
pub fn add_form_fragment(cfg: &FormFormatConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(form_impl_fragment(&cfg.tao_cfg))));
    if cfg.meta_archetype.is_some() {
        file.append(Rc::new(RefCell::new(form_extension_fragment(cfg))));
    }
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

    #[test]
    fn test_archetype_form_trait_fragment() {
        assert_eq!(
            form_extension_fragment(&FormFormatConfig {
                tao_cfg: TaoConfig {
                    this: StructConfig::new("crate::MyAttribute".to_owned()),
                    ..TaoConfig::default()
                },
                meta_archetype: Some(StructConfig::new("crate::MyAttributeArchetype".to_owned())),
                ..FormFormatConfig::default()
            })
            .body(80),
            indoc! {"
                impl FormExtension for MyAttribute {
                    type MetaType = MyAttributeArchetype;
                }"}
        );
    }
}
