use super::tao::TaoConfig;
use crate::codegen::template::basic::{
    Appendable, AtomicFragment, FileFragment, ImplementationFragment,
};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Archetype code generation.
#[derive(Default)]
pub struct ArchetypeFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// The infra archetype of this archetype.
    pub infra_archetype: StructConfig,
}

fn archetype_impl_fragment(cfg: &ArchetypeFormatConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("zamm_yin::tao::archetype::ArchetypeFormTrait".to_owned()),
        cfg.tao_cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    implementation.append(Rc::new(RefCell::new(AtomicFragment {
        imports: vec![cfg.infra_archetype.import.clone()],
        atom: format!("type SubjectForm = {};", cfg.infra_archetype.name),
    })));
    implementation
}

/// Add the archetype fragment to a file.
pub fn add_archetype_fragment(cfg: &ArchetypeFormatConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(archetype_impl_fragment(&cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    #[test]
    fn test_archetype_form_trait_fragment() {
        assert_eq!(
            archetype_impl_fragment(&ArchetypeFormatConfig {
                tao_cfg: TaoConfig {
                    this: StructConfig::new("crate::MyArchetype".to_owned()),
                    ..TaoConfig::default()
                },
                infra_archetype: StructConfig::new("crate::MyAttribute".to_owned()),
            })
            .body(80),
            indoc! {"
                impl ArchetypeFormTrait for MyArchetype {
                    type SubjectForm = MyAttribute;
                }"}
        );
    }
}
