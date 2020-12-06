use std::convert::{From, TryFrom};
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
pub struct AttributeAnalogue {
    base: FinalNode,
}

impl Debug for AttributeAnalogue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("AttributeAnalogue", self, f)
    }
}

impl From<usize> for AttributeAnalogue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for AttributeAnalogue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for AttributeAnalogue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for AttributeAnalogue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for AttributeAnalogue {
    type ArchetypeForm = Archetype;
    type Form = AttributeAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 13;
    const TYPE_NAME: &'static str = "attribute-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for AttributeAnalogue {}

impl From<AttributeAnalogue> for Flag {
    fn from(this: AttributeAnalogue) -> Flag {
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
        #[rustfmt::skip]
        assert_eq!(AttributeAnalogue::archetype().id(), AttributeAnalogue::TYPE_ID);
        assert_eq!(
            AttributeAnalogue::archetype().internal_name_str(),
            Some(Rc::from(AttributeAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(AttributeAnalogue::archetype().added_attributes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(AttributeAnalogue::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = AttributeAnalogue::new();
        let concept_copy = AttributeAnalogue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = AttributeAnalogue::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(AttributeAnalogue::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(AttributeAnalogue::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = AttributeAnalogue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
