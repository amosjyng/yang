use crate::tao::action::Implement;
use crate::tao::form::rust_item::{Concept, Module, Trait};
use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode};
use crate::tao::relation::attribute::{ImplementsTrait, Target};
use crate::codegen::planning::imports::import_path;
use itertools::Itertools;
use heck::{CamelCase, SnakeCase};
use std::ops::Deref;
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
        implementation.set_target(&self.as_form());
        implementation.set_embodiment(&new_module.into());
        implementation.set_documentation(doc);
        new_module
    }

    /// Implement the trait for this concept.
    fn impl_trait(&mut self) -> Trait {
        // don't implement as a struct, but instead separately as a trait that can be quickly
        // implemented by descendant concepts
        let self_trait = Trait::new();
        let mut self_trait_build = BuildInfo::from(self_trait.id());
        // todo: use implementation name. To do that, implementation name needs to default to
        // camel-cased internal name.
        let struct_name = self.internal_name().unwrap().to_camel_case();
        let trait_name = format!("{}Trait", struct_name);
        self_trait_build.set_implementation_name(&trait_name);
        let self_import_path = import_path(&KnowledgeGraphNode::from(self.id()), false);
        let self_import_modules = self_import_path.split("::").collect::<Vec<&str>>();
        let self_module_path = self_import_modules.iter().take(self_import_modules.len() - 1).format("::").to_string();
        self_trait_build.set_import_path(&format!("{}::{}", self_module_path, trait_name));

        // implement this concept's introduced properties as a trait
        let mut trait_impl = Implement::new();
        trait_impl.set_target(&self.as_form());
        trait_impl.set_embodiment(&self_trait.into());
        trait_impl.set_documentation(&format!(
            "This trait implements the set of properties introduced by `{}` that all its conceptual descendants should take on.",
            struct_name
        ));

        // then make sure that trait gets implemented by the structs for this concept and all
        // its descendants
        self.add_trait_implementation(&self_trait);

        self_trait
    }

    /// Look at this concept through the BuildInfo lens.
    fn build_info(&self) -> BuildInfo {
        BuildInfo::from(self.id())
    }

    /// Grab all implementations for this current node. This includes imported ones.
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

    /// Get the implementation for the accessors of this current node as a trait.
    ///
    /// This is as opposed to `trait_implementations`, which refers to all of the traits that the
    /// concept struct will implement, when this concept gets implemented as a concept struct.
    fn as_trait_implementation(&self) -> Option<Implement> {
        self.implementations().into_iter().find(|i| {
            i.embodiment()
                .unwrap()
                .has_ancestor(Trait::archetype().into())
        })
    }

    /// Add a trait to be implemented by this concept and all its descendants.
    fn add_trait_implementation(&mut self, new_trait: &Trait) {
        self.deref_mut()
            .add_outgoing(ImplementsTrait::TYPE_ID, new_trait.deref());
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
    use std::rc::Rc;

    #[test]
    fn retrieve_implementations() {
        initialize_kb();
        let mut form_subtype = Form::archetype().individuate_as_archetype();
        form_subtype.set_internal_name("foosbar");
        form_subtype.impl_mod("foosbar");
        let implement_concept = form_subtype.implement();
        assert_eq!(implement_concept.target(), Some(form_subtype.as_form()));
        let concept_trait = form_subtype.impl_trait();
        assert_eq!(form_subtype.implementations().len(), 3);
        assert_eq!(
            form_subtype.concept_implementation(),
            Some(implement_concept)
        );
        assert_eq!(
            form_subtype.accessor_implementation(),
            Some(implement_concept)
        );
        assert_eq!(
            form_subtype.as_trait_implementation().unwrap().embodiment(),
            Some(concept_trait.into())
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

    #[test]
    fn test_impl_trait() {
        initialize_kb();
        let mut new_type = Form::archetype().individuate_as_archetype();
        new_type.set_internal_name("my-type");
        let mut kgn = KnowledgeGraphNode::from(new_type.id());
        kgn.mark_newly_defined();
        let new_trait_build = BuildInfo::from(new_type.impl_trait().id());
        assert_eq!(new_trait_build.implementation_name(), Some(Rc::from("MyTypeTrait")));
        // sanity check that we're getting the expected module path right at least
        assert_eq!(import_path(&kgn, false), "crate::tao::form::MyType");
        assert_eq!(new_trait_build.import_path(), Some(Rc::from("crate::tao::form::MyTypeTrait")));
    }
}
