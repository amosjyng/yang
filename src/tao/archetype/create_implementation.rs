use crate::tao::form::Module;
use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::Target;
use crate::tao::Implement;
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
        implementation
    }

    /// Implement this concept with the given documentation string.
    fn implement_with_doc(&self, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.set_documentation(doc.to_owned());
        implementation
    }

    /// Implement the module for this concept.
    fn impl_mod(&self, doc: &str) -> Module {
        // todo: implementation info should be built as part of Yin, so that we know here what to
        // use for the intermediate modules
        let mut implementation = Implement::new();
        let mut new_module = Module::new();
        new_module.set_most_prominent_member(&self.as_form());
        if let Some(name) = self.internal_name_str() {
            BuildInfo::from(new_module.id()).set_implementation_name(&name.to_snake_case());
        }
        implementation.set_target(&new_module.as_form());
        implementation.set_documentation(doc.to_owned());
        new_module
    }

    /// Create a new implementation with the specified ID and documentation string.
    #[deprecated(
        since = "0.1.1",
        note = "Please use implement_with_doc instead, and leave the ID up to the program."
    )]
    fn implement_with(&self, id: usize, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.set_concept_id(id);
        implementation.set_documentation(doc.to_owned());
        implementation
    }

    /// Look at this concept through the BuildInfo lens.
    fn build_info(&self) -> BuildInfo {
        BuildInfo::from(self.id())
    }

    /// Grab all implementations for this current node.
    fn implementations(&self) -> Vec<Implement> {
        self.essence()
            .base_wrapper()
            .incoming_nodes(Target::TYPE_ID)
            .into_iter()
            .map(|f| Implement::from(f.id()))
            .collect()
    }
}

impl CreateImplementation for Archetype {}
impl CreateImplementation for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;
    use zamm_yin::tao::initialize_kb;

    #[test]
    fn retrieve_implementations() {
        initialize_kb();
        let form_subtype = Form::archetype().individuate_as_archetype();
        let implement = form_subtype.implement();
        assert_eq!(implement.target(), Some(form_subtype.as_form()));
        assert_eq!(form_subtype.implementations(), vec![implement]);
    }
}
