use crate::tao::action::Implement;
use crate::tao::form::rust_item::{Concept, Module, Trait};
use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::{Target, ImplementsTrait};
use heck::SnakeCase;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use std::ops::Deref;

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
        implementation.set_target(&self.as_form());
        implementation.set_embodiment(&new_module.into());
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

    /// Get the implementation for this node as a Concept node.
    fn concept_implementation(&self) -> Option<Implement> {
        self.implementations().into_iter().find(|i| {
            i.embodiment()
                .unwrap()
                .has_ancestor(Concept::archetype().into())
        })
    }

    /// Get the implementation for the accessors of this current node.
    fn accessor_implementation(&self) -> Option<Implement> {
        self.implementations().into_iter().find(|i| {
            i.embodiment()
                .unwrap()
                .has_ancestor(Concept::archetype().into())
        })
    }

    /// Add a trait to be implemented by this concept and all its descendants.
    fn add_trait_implementation(&mut self, new_trait: &Trait) {
        self.deref_mut().add_outgoing(ImplementsTrait::TYPE_ID, new_trait.deref());
    }

    /// Retrieve all traits implemented by this concept. Some may have been introduced by an 
    /// ancestor.
    fn trait_implementations(&self) -> Vec<Trait> {
        self.deref()
            .outgoing_nodes(ImplementsTrait::TYPE_ID)
            .into_iter()
            .map(|f| Trait::from(f.id()))
            .collect()
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
        form_subtype.impl_mod("foosbar");
        let implement_concept = form_subtype.implement();
        assert_eq!(implement_concept.target(), Some(form_subtype.as_form()));
        assert_eq!(form_subtype.implementations().len(), 2);
        assert_eq!(
            form_subtype.concept_implementation(),
            Some(implement_concept)
        );
        assert_eq!(
            form_subtype.accessor_implementation(),
            Some(implement_concept)
        );
    }

    #[test]
    fn test_trait_implementation_inheritance() {
        initialize_kb();
        let mut new_type = Form::archetype().individuate_as_archetype();
        let mut new_subtype = new_type.individuate_as_archetype();
        assert_eq!(new_subtype.trait_implementations(), vec![]);

        let trait1 = Trait::new();
        let trait2 = Trait::new();
        new_type.add_trait_implementation(&trait1);
        new_subtype.add_trait_implementation(&trait2);
        assert_eq!(new_subtype.trait_implementations(), vec![trait1, trait2]);
    }
}
