use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

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

impl Wrapper for UsesAttributeLogic {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for UsesAttributeLogic {
    type ArchetypeForm = Archetype;
    type Form = UsesAttributeLogic;

    const TYPE_ID: usize = YIN_MAX_ID + 12;
    const TYPE_NAME: &'static str = "uses-attribute-logic";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for UsesAttributeLogic {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::FormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesAttributeLogic::archetype().id(), UsesAttributeLogic::TYPE_ID);
        assert_eq!(
            UsesAttributeLogic::archetype().internal_name_str(),
            Some(Rc::from(UsesAttributeLogic::TYPE_NAME))
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
        let concept = UsesAttributeLogic::new();
        let concept_copy = UsesAttributeLogic::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UsesAttributeLogic::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(UsesAttributeLogic::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(UsesAttributeLogic::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = UsesAttributeLogic::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
