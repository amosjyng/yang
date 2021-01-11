use crate::tao::form::rust_item::Module;
use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::{SupportsMembership};
use std::rc::Rc;
use zamm_yin::node_wrappers::{CommonNodeTrait};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::FormTrait;

/// Trait to extend Module functionality that has not been auto-generated.
pub trait ModuleExtension: FormTrait + CommonNodeTrait + SupportsMembership {
    /// Retrieve the public name that this module is actually implemented with.
    fn implementation_name(&self) -> Option<Rc<str>> {
        BuildInfo::from(self.id()).implementation_name()
    }

    /// Add a new submodule as a member of the current one.
    fn add_submodule(&mut self, name: &str) -> Module {
        let new_submodule = Module::new();
        BuildInfo::from(new_submodule.id()).set_implementation_name(name);
        self.add_member(&new_submodule.as_form());
        new_submodule
    }

    /// Get the submodules of this module.
    fn submodules(&self) -> Vec<Module> {
        self.members()
            .iter()
            .filter(|f| f.has_ancestor(Module::archetype().into()))
            .map(|f| Module::from(f.id()))
            .collect()
    }
}

impl Module {
    /// If the submodule is locally defined, then defines it. Marks the trait for re-export.
    pub fn has_extension(&mut self, extension: &str) {
        let import_path = extension.split("::").collect::<Vec<&str>>();
        if import_path.len() == 2 {
            self.add_submodule(import_path.first().unwrap());
        }
        self.add_re_export(extension);
    }
}

impl SupportsMembership for Module {}
impl ModuleExtension for Module {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

    #[test]
    fn add_and_retrieve_submodule() {
        initialize_kb();
        let mut module = Module::new();
        module.add_submodule("submod");
        assert_eq!(
            module
                .submodules()
                .iter()
                .map(|s| s.implementation_name())
                .collect::<Vec<Option<Rc<str>>>>(),
            vec![Some(Rc::from("submod"))]
        );
    }

    #[test]
    fn add_and_retrieve_extension() {
        initialize_kb();
        let mut module = Module::new();
        module.has_extension("submod::X");
        assert_eq!(
            module
                .submodules()
                .iter()
                .map(|s| s.implementation_name())
                .collect::<Vec<Option<Rc<str>>>>(),
            vec![Some(Rc::from("submod"))]
        );
        assert_eq!(module.re_exports(), vec![Rc::from("submod::X")]);
    }

    #[test]
    fn add_and_retrieve_re_exported_extension() {
        initialize_kb();
        let mut module = Module::new();
        module.has_extension("other_crate::submod::X");
        assert_eq!(
            module
                .submodules()
                .iter()
                .map(|s| s.implementation_name())
                .collect::<Vec<Option<Rc<str>>>>(),
            vec![]
        );
        assert_eq!(module.re_exports(), vec![Rc::from("other_crate::submod::X")]);
    }
}
