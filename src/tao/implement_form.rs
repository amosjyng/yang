use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};
use zamm_yin::Wrapper;

/// The act of implementing something. When created, this effectively serves as
/// a call to action for Yang.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Implement {
    base: FinalNode,
}

impl Debug for Implement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Implement", self, f)
    }
}

impl From<usize> for Implement {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Implement {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
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
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::FormTrait;

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
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Implement::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Implement::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Implement::new();
        let concept_copy = Implement::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Implement::new();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Implement::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Implement::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Implement::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
