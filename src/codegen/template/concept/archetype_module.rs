use crate::codegen::template::basic::ModuleFragment;

/// Config values at the time of Archetype module code generation.
#[derive(Default)]
pub struct ArchetypeModuleConfig {
    /// Submodules that are not to be accessible outside of this module.
    pub private_submodules: Vec<String>,
    /// Submodules that are to be accessible outside of this module. Usually user-defined ones.
    pub public_submodules: Vec<String>,
    /// Re-exports from this module.
    pub re_exports: Vec<String>,
}

/// Returns a module that represents an archetype and its descendants.
pub fn archetype_module_fragment(cfg: &ArchetypeModuleConfig) -> ModuleFragment {
    let mut module = ModuleFragment::new_file_module();

    for private_module in &cfg.private_submodules {
        module.add_submodule(private_module.clone());
    }
    for public_module in &cfg.public_submodules {
        let public_mod = module.add_submodule(public_module.clone());
        public_mod.borrow_mut().mark_as_public();
    }
    for re_export in &cfg.re_exports {
        module.re_export(re_export.clone());
    }

    module
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    #[test]
    fn test_archetype_module() {
        let frag = archetype_module_fragment(&ArchetypeModuleConfig {
            private_submodules: vec![
                "primary_form".to_owned(),
                "concept1_form".to_owned(),
                "concept2_form".to_owned(),
            ],
            public_submodules: vec!["subtype".to_owned(), "primary_extension".to_owned()],
            re_exports: vec![
                "concept1_form::Concept1".to_owned(),
                "concept2_form::Concept2".to_owned(),
                "primary_form::Primary".to_owned(),
                "zamm_yin::path::to::primary::*".to_owned(),
            ],
        });

        assert_eq!(
            frag.body(),
            indoc! {"
                pub mod primary_extension;
                pub mod subtype;

                mod concept1_form;
                mod concept2_form;
                mod primary_form;

                pub use concept1_form::Concept1;
                pub use concept2_form::Concept2;
                pub use primary_form::Primary;
                pub use zamm_yin::path::to::primary::*;
            "}
        );
    }
}
