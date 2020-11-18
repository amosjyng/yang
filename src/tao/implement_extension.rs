use crate::tao::relation::attribute::{ConceptId, Documentation};
use crate::tao::Implement;
use crate::tao::Target;
use std::rc::Rc;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::data::{Number, StringConcept};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::Wrapper;

/// Extension functions for Implement concept.
pub trait ImplementExtension: FormTrait {
    /// Set another concept as an implementation target.
    fn set_target(&mut self, target: Archetype) {
        self.essence_mut()
            .add_outgoing(Target::TYPE_ID, target.essence());
    }

    /// Retrieve implementation target.
    fn target(&self) -> Option<Archetype> {
        self.essence()
            .outgoing_nodes(Target::TYPE_ID)
            .into_iter()
            .next()
            .map(Archetype::from)
    }

    /// Set concept ID during code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    fn set_implementation_id(&mut self, id: usize) {
        let mut n = Number::new();
        n.set_value(id);
        self.essence_mut()
            .add_outgoing(ConceptId::TYPE_ID, n.essence());
    }

    /// Get concept ID set for code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    fn implementation_id(&self) -> Option<Rc<usize>> {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ConceptId::TYPE_ID)
            .first()
            .map(|v| Number::from(v.id()).value())
            .flatten()
    }

    /// Set the documentation string for this implementation.
    fn document(&mut self, document: &str) {
        let mut s = StringConcept::new();
        s.set_value(document.to_owned());
        self.essence_mut()
            .add_outgoing(Documentation::TYPE_ID, s.essence());
    }

    /// Get the documentation string for this implementation.
    fn documentation(&self) -> Option<Rc<String>> {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(Documentation::TYPE_ID)
            .first()
            .map(|v| StringConcept::from(v.id()).value())
            .flatten()
    }
}

impl ImplementExtension for Implement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn set_and_retrieve_target() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(Owner::archetype().as_archetype());
        assert_eq!(implement.target(), Some(Owner::archetype().as_archetype()));
    }

    #[test]
    fn set_and_retrieve_implementation_id() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_implementation_id(17);
        assert_eq!(implement.implementation_id(), Some(Rc::new(17)));
    }

    #[test]
    fn set_and_retrieve_documentation() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.document("Some new thing.");
        assert_eq!(
            implement.documentation(),
            Some(Rc::new("Some new thing.".to_owned()))
        );
    }
}
