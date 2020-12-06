use crate::tao::form::Module;
use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::{ReExports, SupportsMembership};
use std::rc::Rc;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

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
            .filter(|f| f.has_ancestor(Module::archetype()))
            .map(|f| Module::from(f.id()))
            .collect()
    }

    /// Add a new symbol to re-export from this module.
    fn re_export(&mut self, symbol: &str) {
        let mut re_export_symbol = StringConcept::new();
        re_export_symbol.set_value(symbol.to_owned());
        self.essence_mut()
            .add_outgoing(ReExports::TYPE_ID, re_export_symbol.essence());
    }

    /// Retrieve all symbols re-exported by this module.
    fn re_exports(&self) -> Vec<Rc<str>> {
        // no need to worry about inheritance because modules don't inherit from each other
        self.essence()
            .outgoing_nodes(ReExports::TYPE_ID)
            .iter()
            .map(|f| Rc::from(StringConcept::from(*f).value().unwrap().as_str()))
            .collect()
    }

    /// Define the submodule for the given trait extension, and mark the trait for re-export.
    fn has_extension(&mut self, extension: &str) {
        self.add_submodule(extension.split("::").next().unwrap());
        self.re_export(extension);
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
    fn add_and_retrieve_re_exports() {
        initialize_kb();
        let mut module = Module::new();
        module.re_export("submod::X");
        assert_eq!(module.re_exports(), vec![Rc::from("submod::X")]);
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
}
