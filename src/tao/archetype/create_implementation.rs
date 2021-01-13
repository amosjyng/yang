use crate::tao::action::Implement;
use crate::tao::form::rust_item::{Module, Concept};
use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::Target;
use heck::SnakeCase;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;

/// Convenience trait for creating a new implementation of a concept.
pub trait CreateImplementation: FormTrait + CommonNodeTrait {
    /// Create a new implementation for a concept.
    fn implement(&self) -> Implement {
        let mut implementation = Implement::new();
        implementation.set_target(&self.as_form());
        implementation.set_embodiment(&Concept::new().into());
        implementation
    }

    /// Implement this concept with the given documentation string.
    fn implement_with_doc(&self, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.set_documentation(doc);
        implementation
    }

    /// Implement the module for this concept.
    fn impl_mod(&self, doc: &str) -> Module {
        // todo: implementation info should be built as part of Yin, so that we know here what to
        // use for the intermediate modules
        let mut implementation = Implement::new();
        let mut new_module = Module::new();
        new_module.set_most_prominent_member(&self.as_form());
        if let Some(name) = self.internal_name() {
            BuildInfo::from(new_module.id()).set_implementation_name(&name.to_snake_case());
        }
        implementation.set_target(&new_module.as_form());
        implementation.set_documentation(doc);
        new_module
    }

    /// Look at this concept through the BuildInfo lens.
    fn build_info(&self) -> BuildInfo {
        BuildInfo::from(self.id())
    }

    /// Grab all implementations for this current node.
    fn implementations(&self) -> Vec<Implement> {
        self.base_wrapper()
            .incoming_nodes(Target::TYPE_ID)
            .into_iter()
            .map(|f| Implement::from(f.id()))
            .collect()
    }

    /// Get the implementation for the accessors of this current node.
    fn accessor_implementation(&self) -> Option<Implement> {
        self.implementations()
            .into_iter()
            .find(|i| !i.target().unwrap().has_ancestor(Module::archetype().into()))
    }
}

impl CreateImplementation for Archetype {}
impl CreateImplementation for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn retrieve_implementations() {
        initialize_kb();
        let form_subtype = Form::archetype().individuate_as_archetype();
        let implement = form_subtype.implement();
        assert_eq!(implement.target(), Some(form_subtype.as_form()));
        assert!(implement.embodiment().unwrap().has_ancestor(Concept::archetype().into()));
        assert_eq!(form_subtype.implementations(), vec![implement]);
    }

    #[test]
    fn retrieve_accessor_implementations() {
        initialize_kb();
        let form_subtype = Form::archetype().individuate_as_archetype();
        form_subtype.impl_mod("foosbar");
        let implement = form_subtype.implement();
        assert_eq!(implement.target(), Some(form_subtype.as_form()));
        assert_eq!(form_subtype.implementations(), vec![implement]);
    }
}
