use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks an archetype and all its descendants as having been newly defined as
/// part of this particular build.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewlyDefined {
    base: FinalNode,
}

impl Debug for NewlyDefined {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("NewlyDefined", self, f)
    }
}

impl From<usize> for NewlyDefined {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for NewlyDefined {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for NewlyDefined {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for NewlyDefined {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for NewlyDefined {
    type ArchetypeForm = Archetype;
    type Form = NewlyDefined;

    const TYPE_ID: usize = YIN_MAX_ID + 5;
    const TYPE_NAME: &'static str = "newly-defined";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for NewlyDefined {}

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
        assert_eq!(NewlyDefined::archetype().id(), NewlyDefined::TYPE_ID);
        assert_eq!(
            NewlyDefined::archetype().internal_name(),
            Some(Rc::new(NewlyDefined::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(NewlyDefined::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(NewlyDefined::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = NewlyDefined::new();
        let concept_copy = NewlyDefined::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = NewlyDefined::new();
        concept.set_internal_name("A".to_owned());
        #[rustfmt::skip]
        assert_eq!(NewlyDefined::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(NewlyDefined::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = NewlyDefined::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
