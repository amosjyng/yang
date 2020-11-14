use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::{FormTrait, Tao, YIN_MAX_ID};

/// Marks an archetype and all its descendants as requiring attribute-specific
/// logic during generation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsesAttributeLogic {
    base: FinalNode,
}

impl Debug for UsesAttributeLogic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("UsesAttributeLogic", self, f)
    }
}

impl From<usize> for UsesAttributeLogic {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for UsesAttributeLogic {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for UsesAttributeLogic {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for UsesAttributeLogic {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a> for UsesAttributeLogic {
    type ArchetypeForm = Archetype;
    type Form = UsesAttributeLogic;

    const TYPE_ID: usize = YIN_MAX_ID + 4;
    const TYPE_NAME: &'static str = "uses-attribute-logic";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for UsesAttributeLogic {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesAttributeLogic::archetype().id(), UsesAttributeLogic::TYPE_ID);
        assert_eq!(
            UsesAttributeLogic::archetype().internal_name(),
            Some(Rc::new(UsesAttributeLogic::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesAttributeLogic::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(UsesAttributeLogic::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = UsesAttributeLogic::individuate();
        let concept_copy = UsesAttributeLogic::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UsesAttributeLogic::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(UsesAttributeLogic::try_from("A"), Ok(concept));
        assert!(UsesAttributeLogic::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = UsesAttributeLogic::individuate();
        let concept2 = UsesAttributeLogic::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = UsesAttributeLogic::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
