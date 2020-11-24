use super::tao::{tao_file_fragment, TaoConfig};
use crate::codegen::template::basic::{FileFragment, ImplementationFragment};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

fn form_fragment(cfg: &TaoConfig) -> ImplementationFragment {
    let mut implementation = ImplementationFragment::new(
        StructConfig::new(format!("{}::tao::form::FormTrait", cfg.yin_crate)),
        cfg.this.clone(),
    );
    implementation.mark_same_file_as_struct();
    implementation
}

/// Get both the Tao and Form fragments.
pub fn form_file_fragment(cfg: &TaoConfig) -> FileFragment {
    let mut file = tao_file_fragment(cfg);
    file.append(Rc::new(RefCell::new(form_fragment(cfg))));
    file
}

/// Generate code for a form concept.
pub fn code_form(cfg: &TaoConfig) -> String {
    form_file_fragment(cfg).generate_code()
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
            .body(),
            "impl FormTrait for MyConcept {}"
        );
    }
}
