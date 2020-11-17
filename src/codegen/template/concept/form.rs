use super::tao::TaoConfig;
use super::tao::{tao_fragment, tao_test_fragment};
use crate::codegen::template::basic::{AppendedFragment, AtomicFragment, FileFragment};
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Get both the Tao and Form fragments.
pub fn form_fragment(cfg: &TaoConfig) -> AppendedFragment {
    let form_specific_fragment = AtomicFragment {
        imports: vec![format!("{}::tao::form::FormTrait", cfg.yin_crate)],
        atom: formatdoc! {r#"
            impl FormTrait for {name} {{}}
            "#, name = cfg.this.name,
        },
    };
    let mut combined_fragment = AppendedFragment::default();
    combined_fragment.append(Rc::new(RefCell::new(tao_fragment(cfg))));
    combined_fragment.append(Rc::new(RefCell::new(form_specific_fragment)));
    combined_fragment
}

/// Generate code for a form concept.
pub fn code_form(cfg: &TaoConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(form_fragment(cfg))));
    file.set_tests(Rc::new(RefCell::new(tao_test_fragment(cfg))));
    file.generate_code()
}
