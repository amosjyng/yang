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

impl ArchetypeTrait for NewlyDefined {
    type ArchetypeForm = Archetype;
    type Form = NewlyDefined;

    const TYPE_ID: usize = YIN_MAX_ID + 21;
    const TYPE_NAME: &'static str = "newly-defined";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for NewlyDefined {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for NewlyDefined {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for NewlyDefined {}

impl From<NewlyDefined> for Tao {
    fn from(this: NewlyDefined) -> Tao {
        Tao::from(this.base)
    }
}

impl From<NewlyDefined> for Relation {
    fn from(this: NewlyDefined) -> Relation {
        Relation::from(this.base)
    }
}

impl From<NewlyDefined> for Flag {
    fn from(this: NewlyDefined) -> Flag {
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
        assert_eq!(NewlyDefined::archetype().id(), NewlyDefined::TYPE_ID);
        assert_eq!(
            NewlyDefined::archetype().internal_name(),
            Some(Rc::from(NewlyDefined::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = NewlyDefined::new();
        concept.set_internal_name("A");
        assert_eq!(
            NewlyDefined::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(NewlyDefined::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(NewlyDefined::archetype().added_attributes(), vec![]);
        assert_eq!(
            NewlyDefined::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = NewlyDefined::new();
        let concept_copy = NewlyDefined::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = NewlyDefined::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
