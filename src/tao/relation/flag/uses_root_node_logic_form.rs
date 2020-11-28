use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks an archetype as requiring root-node-specific logic during generation.
/// None of its descendants will inherit this.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsesRootNodeLogic {
    base: FinalNode,
}

impl Debug for UsesRootNodeLogic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("UsesRootNodeLogic", self, f)
    }
}

impl From<usize> for UsesRootNodeLogic {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for UsesRootNodeLogic {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for UsesRootNodeLogic {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for UsesRootNodeLogic {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for UsesRootNodeLogic {
    type ArchetypeForm = Archetype;
    type Form = UsesRootNodeLogic;

    const TYPE_ID: usize = YIN_MAX_ID + 15;
    const TYPE_NAME: &'static str = "uses-root-node-logic";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for UsesRootNodeLogic {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::FormTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesRootNodeLogic::archetype().id(), UsesRootNodeLogic::TYPE_ID);
        assert_eq!(
            UsesRootNodeLogic::archetype().internal_name_str(),
            Some(Rc::from(UsesRootNodeLogic::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesRootNodeLogic::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(UsesRootNodeLogic::archetype().attribute_archetypes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = UsesRootNodeLogic::new();
        let concept_copy = UsesRootNodeLogic::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UsesRootNodeLogic::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(UsesRootNodeLogic::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(UsesRootNodeLogic::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = UsesRootNodeLogic::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
