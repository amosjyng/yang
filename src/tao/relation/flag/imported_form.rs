use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks a concept as being defined in an imported file.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Imported {
    base: FinalNode,
}

impl Debug for Imported {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Imported", self, f)
    }
}

impl From<usize> for Imported {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Imported {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Imported {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Imported {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Imported {
    type ArchetypeForm = Archetype;
    type Form = Imported;

    const TYPE_ID: usize = YIN_MAX_ID + 10;
    const TYPE_NAME: &'static str = "imported";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for Imported {}

impl From<Imported> for Flag {
    fn from(this: Imported) -> Flag {
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
        assert_eq!(Imported::archetype().id(), Imported::TYPE_ID);
        assert_eq!(
            Imported::archetype().internal_name_str(),
            Some(Rc::from(Imported::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Imported::new();
        concept.set_internal_name_str("A");
        assert_eq!(Imported::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Imported::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Imported::archetype().added_attributes(), vec![]);
        assert_eq!(Imported::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Imported::new();
        let concept_copy = Imported::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Imported::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
