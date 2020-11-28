use super::tao::TaoConfig;
use crate::codegen::template::basic::{FileFragment, ImplementationFragment};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

fn form_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new_trait_impl(
        StructConfig::new("zamm_yin::tao::form::FormTrait".to_owned()),
        cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    implementation
}

/// Add the form fragment to a file.
pub fn add_form_fragment(cfg: &TaoConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(form_fragment(cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;

    #[test]
    fn test_form_fragment() {
        assert_eq!(
            form_fragment(&TaoConfig {
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
}
