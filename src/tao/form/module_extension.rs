use crate::tao::form::data::StringConcept;
use crate::tao::form::Module;
use crate::tao::form::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::{HasMember, MostProminentMember, ReExports};
use heck::SnakeCase;
use std::rc::Rc;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Trait to extend Module functionality that has not been auto-generated.
pub trait ModuleExtension: FormTrait + CommonNodeTrait {
    /// Retrieve the public name that this module is actually implemented with.
    fn implementation_name(&self) -> Option<Rc<str>> {
        BuildInfo::from(self.id()).implementation_name()
    }

    /// Set most prominent member of the module. By default, this also sets the name of the module
    /// to be a snake-cased version of that member's name.
    fn set_most_prominent_member(&mut self, member: &Form) {
        self.essence_mut()
            .add_outgoing(MostProminentMember::TYPE_ID, member.essence());
        if let Some(name) = member.internal_name() {
            BuildInfo::from(self.id()).set_implementation_name(&name.to_snake_case())
        }
    }

    /// Retrieve most prominent member of the module.
    fn most_prominent_member(&self) -> Option<Form> {
        // no need to worry about inheritance because MostProminentMember is not Inherits
        self.essence()
            .outgoing_nodes(MostProminentMember::TYPE_ID)
            .first()
            .map(|f| Form::from(*f))
    }

    /// Add a member to the module.
    fn add_member(&mut self, member: &Form) {
        self.essence_mut()
            .add_outgoing(HasMember::TYPE_ID, member.essence());
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
        // no need to worry about inheritance because HasMember is not Inherits
        self.essence()
            .outgoing_nodes(HasMember::TYPE_ID)
            .iter()
            .map(|f| Module::from(*f))
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
        // no need to worry about inheritance because ReExports is not Inherits
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

impl ModuleExtension for Module {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::Tao;

    #[test]
    fn set_and_retrieve_most_prominent_member() {
        initialize_kb();
        let mut my_type = Tao::archetype().individuate_as_archetype().as_form();
        my_type.set_internal_name("my-amazing-type".to_owned());
        let mut module = Module::new();
        module.set_most_prominent_member(&my_type);
        assert_eq!(module.most_prominent_member(), Some(my_type));
        assert_eq!(
            module.implementation_name(),
            Some(Rc::from("my_amazing_type"))
        );
    }

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
