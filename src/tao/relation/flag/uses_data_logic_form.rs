use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks an archetype and all its descendants as requiring data-specific logic
/// during generation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsesDataLogic {
    base: FinalNode,
}

impl Debug for UsesDataLogic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("UsesDataLogic", self, f)
    }
}

impl From<usize> for UsesDataLogic {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for UsesDataLogic {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for UsesDataLogic {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for UsesDataLogic {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for UsesDataLogic {
    type ArchetypeForm = Archetype;
    type Form = UsesDataLogic;

    const TYPE_ID: usize = YIN_MAX_ID + 13;
    const TYPE_NAME: &'static str = "uses-data-logic";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for UsesDataLogic {}

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
        assert_eq!(UsesDataLogic::archetype().id(), UsesDataLogic::TYPE_ID);
        assert_eq!(
            UsesDataLogic::archetype().internal_name(),
            Some(Rc::new(UsesDataLogic::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesDataLogic::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(UsesDataLogic::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = UsesDataLogic::new();
        let concept_copy = UsesDataLogic::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UsesDataLogic::new();
        concept.set_internal_name("A".to_owned());
        #[rustfmt::skip]
        assert_eq!(UsesDataLogic::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(UsesDataLogic::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = UsesDataLogic::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
