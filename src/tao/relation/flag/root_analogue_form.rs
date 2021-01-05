use std::convert::{From, TryFrom};
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
pub struct RootAnalogue {
    base: FinalNode,
}

impl Debug for RootAnalogue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("RootAnalogue", self, f)
    }
}

impl From<usize> for RootAnalogue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for RootAnalogue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for RootAnalogue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for RootAnalogue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for RootAnalogue {
    type ArchetypeForm = Archetype;
    type Form = RootAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 12;
    const TYPE_NAME: &'static str = "root-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for RootAnalogue {}

impl From<RootAnalogue> for Flag {
    fn from(this: RootAnalogue) -> Flag {
        Flag::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(RootAnalogue::archetype().id(), RootAnalogue::TYPE_ID);
        assert_eq!(
            RootAnalogue::archetype().internal_name_str(),
            Some(Rc::from(RootAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RootAnalogue::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            RootAnalogue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(RootAnalogue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(RootAnalogue::archetype().added_attributes(), vec![]);
        assert_eq!(
            RootAnalogue::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = RootAnalogue::new();
        let concept_copy = RootAnalogue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = RootAnalogue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
