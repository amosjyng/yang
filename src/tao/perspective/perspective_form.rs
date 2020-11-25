use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};
use zamm_yin::Wrapper;

/// Describes a way of looking at things that is only well-defined within a
/// specific context.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Perspective {
    base: FinalNode,
}

impl Debug for Perspective {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Perspective", self, f)
    }
}

impl From<usize> for Perspective {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Perspective {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Perspective {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Perspective {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Perspective {
    type ArchetypeForm = Archetype;
    type Form = Perspective;

    const TYPE_ID: usize = YIN_MAX_ID + 18;
    const TYPE_NAME: &'static str = "perspective";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Perspective {}

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
        assert_eq!(Perspective::archetype().id(), Perspective::TYPE_ID);
        assert_eq!(
            Perspective::archetype().internal_name_str(),
            Some(Rc::from(Perspective::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Perspective::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Perspective::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Perspective::new();
        let concept_copy = Perspective::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Perspective::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(Perspective::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Perspective::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Perspective::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
