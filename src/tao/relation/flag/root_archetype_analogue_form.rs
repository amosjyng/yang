use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for RootArchetypeAnalogue {
    type ArchetypeForm = Archetype;
    type Form = RootArchetypeAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 25;
    const TYPE_NAME: &'static str = "root-archetype-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for RootArchetypeAnalogue {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for RootArchetypeAnalogue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for RootArchetypeAnalogue {}

impl From<RootArchetypeAnalogue> for Tao {
    fn from(this: RootArchetypeAnalogue) -> Tao {
        Tao::from(this.base)
    }
}

impl From<RootArchetypeAnalogue> for Relation {
    fn from(this: RootArchetypeAnalogue) -> Relation {
        Relation::from(this.base)
    }
}

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
            RootArchetypeAnalogue::archetype().internal_name(),
            Some(Rc::from(RootArchetypeAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RootArchetypeAnalogue::new();
        concept.set_internal_name("A");
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
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
