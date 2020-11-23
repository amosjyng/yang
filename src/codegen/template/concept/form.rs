use super::tao::tao_file_fragment;
use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment};
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get both the Tao and Form fragments.
pub fn form_fragment(cfg: &TaoConfig) -> FileFragment {
    let form_specific_fragment = AtomicFragment {
        imports: vec![format!("{}::tao::form::FormTrait", cfg.yin_crate)],
        atom: formatdoc! {r#"
            impl FormTrait for {name} {{}}
            "#, name = cfg.this.name,
        },
    };
    let mut file = tao_file_fragment(cfg);
    file.append(Rc::new(RefCell::new(form_specific_fragment)));
    file
}

/// Generate code for a form concept.
pub fn code_form(cfg: &TaoConfig) -> String {
    form_fragment(cfg).generate_code()
}
