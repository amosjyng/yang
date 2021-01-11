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

/// Whether or not this data structure has a known size at compile-time.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unsized {
    base: FinalNode,
}

impl Debug for Unsized {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Unsized", self, f)
    }
}

impl From<usize> for Unsized {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Unsized {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Unsized {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Unsized {
    type ArchetypeForm = Archetype;
    type Form = Unsized;

    const TYPE_ID: usize = YIN_MAX_ID + 11;
    const TYPE_NAME: &'static str = "unsized";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for Unsized {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Unsized {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Unsized {}

impl From<Unsized> for Tao {
    fn from(this: Unsized) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Unsized> for Relation {
    fn from(this: Unsized) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Unsized> for Flag {
    fn from(this: Unsized) -> Flag {
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
        assert_eq!(Unsized::archetype().id(), Unsized::TYPE_ID);
        assert_eq!(
            Unsized::archetype().internal_name(),
            Some(Rc::from(Unsized::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Unsized::new();
        concept.set_internal_name("A");
        assert_eq!(Unsized::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Unsized::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Unsized::archetype().added_attributes(), vec![]);
        assert_eq!(Unsized::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Unsized::new();
        let concept_copy = Unsized::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Unsized::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
