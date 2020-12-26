use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks an archetype as requiring root-archetype-specific logic during
/// generation. None of its descendants will inherit this.
///
/// The root archetype node is different from the root node. All nodes descend
/// from the root node, including the root archetype node; all archetypes
/// descend from the root archetype node.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RootArchetypeAnalogue {
    base: FinalNode,
}

impl Debug for RootArchetypeAnalogue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("RootArchetypeAnalogue", self, f)
    }
}

impl From<usize> for RootArchetypeAnalogue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for RootArchetypeAnalogue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for RootArchetypeAnalogue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for RootArchetypeAnalogue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for RootArchetypeAnalogue {
    type ArchetypeForm = Archetype;
    type Form = RootArchetypeAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 13;
    const TYPE_NAME: &'static str = "root-archetype-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for RootArchetypeAnalogue {}

impl From<RootArchetypeAnalogue> for Flag {
    fn from(this: RootArchetypeAnalogue) -> Flag {
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
        assert_eq!(
            RootArchetypeAnalogue::archetype().id(),
            RootArchetypeAnalogue::TYPE_ID
        );
        assert_eq!(
            RootArchetypeAnalogue::archetype().internal_name_str(),
            Some(Rc::from(RootArchetypeAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RootArchetypeAnalogue::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            RootArchetypeAnalogue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(RootArchetypeAnalogue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            RootArchetypeAnalogue::archetype().added_attributes(),
            vec![]
        );
        assert_eq!(
            RootArchetypeAnalogue::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = RootArchetypeAnalogue::new();
        let concept_copy = RootArchetypeAnalogue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = RootArchetypeAnalogue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
