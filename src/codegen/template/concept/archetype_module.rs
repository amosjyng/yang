use crate::codegen::template::basic::{CodeFragment, ItemDeclarationAPI, ModuleFragment};
use heck::{CamelCase, SnakeCase};
use std::rc::Rc;

/// Config values at the time of Archetype module code generation.
#[derive(Default)]
pub struct ArchetypeModuleConfig {
    /// Documentation for the archetype.
    pub doc: Option<Rc<str>>,
    /// Names of Archetypes to be included directly in this module.
    pub archetype_names: Vec<Rc<str>>,
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

    if let Some(doc) = &cfg.doc {
        module.document(doc.to_string());
    }

    for archetype_name in &cfg.archetype_names {
        let snakey_name = archetype_name.to_snake_case().to_ascii_lowercase();
        let form_module_name = format!("{}_form", snakey_name);
        // archetype forms are private...
        module.add_submodule(form_module_name.clone());
        // ...so that their re-exports could be public
        module.re_export(format!(
            "{}::{}",
            form_module_name,
            archetype_name.to_camel_case()
        ));
    }

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

/// Actually generate the code for the module.
pub fn code_archetype_module(cfg: &ArchetypeModuleConfig) -> String {
    archetype_module_fragment(cfg).body()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_archetype_module() {
        let frag = archetype_module_fragment(&ArchetypeModuleConfig {
            doc: Some(Rc::from(
                "Primary is the ancestor of all other forms in this module.",
            )),
            archetype_names: vec![
                Rc::from("primary"),
                Rc::from("concept-one"),
                Rc::from("concept-two"),
            ],
            private_submodules: vec![],
            public_submodules: vec!["subtype".to_owned(), "primary_extension".to_owned()],
            re_exports: vec!["zamm_yin::path::to::primary::*".to_owned()],
        });

        assert_eq!(
            frag.body(),
            indoc! {"
                //! Primary is the ancestor of all other forms in this module.
                
                pub mod primary_extension;
                pub mod subtype;

                mod concept_one_form;
                mod concept_two_form;
                mod primary_form;

                pub use concept_one_form::ConceptOne;
                pub use concept_two_form::ConceptTwo;
                pub use primary_form::Primary;
                pub use zamm_yin::path::to::primary::*;
            "}
        );
    }
}
