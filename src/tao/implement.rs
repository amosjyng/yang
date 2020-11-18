use crate::tao::relation::attribute::{ConceptId, Documentation};
use crate::tao::Target;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::data::{Number, StringConcept};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};
use zamm_yin::Wrapper;

/// Represents a command to implement something.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Implement {
    base: FinalNode,
}

impl Implement {
    /// Set another concept as an implementation target.
    pub fn set_target(&mut self, target: Archetype) {
        self.essence_mut()
            .add_outgoing(Target::TYPE_ID, target.essence());
    }

    /// Retrieve implementation target.
    pub fn target(&self) -> Option<Archetype> {
        self.essence()
            .outgoing_nodes(Target::TYPE_ID)
            .into_iter()
            .next()
            .map(Archetype::from)
    }

    /// Set concept ID during code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    pub fn set_implementation_id(&mut self, id: usize) {
        let mut n = Number::new();
        n.set_value(id);
        self.base.add_outgoing(ConceptId::TYPE_ID, n.essence());
    }

    /// Get concept ID set for code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    pub fn implementation_id(&self) -> Option<Rc<usize>> {
        self.base
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ConceptId::TYPE_ID)
            .first()
            .map(|v| Number::from(v.id()).value())
            .flatten()
    }

    /// Set the documentation string for this implementation.
    pub fn document(&mut self, document: &str) {
        let mut s = StringConcept::new();
        s.set_value(document.to_owned());
        self.base.add_outgoing(Documentation::TYPE_ID, s.essence());
    }

    /// Get the documentation string for this implementation.
    pub fn documentation(&self) -> Option<Rc<String>> {
        self.base
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(Documentation::TYPE_ID)
            .first()
            .map(|v| StringConcept::from(v.id()).value())
            .flatten()
    }
}

impl Debug for Implement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Implement", self, f)
    }
}

impl From<usize> for Implement {
    fn from(id: usize) -> Self {
        Implement {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Implement {
    fn from(f: FinalNode) -> Self {
        Implement { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Implement {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Implement {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Implement {
    type ArchetypeForm = Archetype;
    type Form = Implement;

    const TYPE_ID: usize = YIN_MAX_ID + 1;
    const TYPE_NAME: &'static str = "implement";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Implement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Implement::archetype().id(), Implement::TYPE_ID);
        assert_eq!(
            Implement::archetype().internal_name(),
            Some(Rc::new(Implement::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Implement::new();
        let concept_copy = Implement::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Implement::new();
        let concept2 = Implement::new();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Implement::new();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

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
