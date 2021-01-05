use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks an archetype and all its descendants as requiring archetype-specific
/// logic during generation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchetypeAnalogue {
    base: FinalNode,
}

impl Debug for ArchetypeAnalogue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ArchetypeAnalogue", self, f)
    }
}

impl From<usize> for ArchetypeAnalogue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for ArchetypeAnalogue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for ArchetypeAnalogue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for ArchetypeAnalogue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for ArchetypeAnalogue {
    type ArchetypeForm = Archetype;
    type Form = ArchetypeAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 14;
    const TYPE_NAME: &'static str = "archetype-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for ArchetypeAnalogue {}

impl From<ArchetypeAnalogue> for Flag {
    fn from(this: ArchetypeAnalogue) -> Flag {
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
            ArchetypeAnalogue::archetype().id(),
            ArchetypeAnalogue::TYPE_ID
        );
        assert_eq!(
            ArchetypeAnalogue::archetype().internal_name_str(),
            Some(Rc::from(ArchetypeAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ArchetypeAnalogue::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            ArchetypeAnalogue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(ArchetypeAnalogue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(ArchetypeAnalogue::archetype().added_attributes(), vec![]);
        assert_eq!(
            ArchetypeAnalogue::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ArchetypeAnalogue::new();
        let concept_copy = ArchetypeAnalogue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = ArchetypeAnalogue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
