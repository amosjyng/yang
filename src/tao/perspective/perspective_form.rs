use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for Perspective {
    type ArchetypeForm = Archetype;
    type Form = Perspective;

    const TYPE_ID: usize = YIN_MAX_ID + 19;
    const TYPE_NAME: &'static str = "perspective";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl Deref for Perspective {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Perspective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Perspective {}

impl From<Perspective> for Tao {
    fn from(this: Perspective) -> Tao {
        Tao::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Perspective::archetype().id(), Perspective::TYPE_ID);
        assert_eq!(
            Perspective::archetype().internal_name(),
            Some(Rc::from(Perspective::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Perspective::new();
        concept.set_internal_name("A");
        assert_eq!(Perspective::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Perspective::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Perspective::archetype().added_attributes(), vec![]);
        assert_eq!(Perspective::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Perspective::new();
        let concept_copy = Perspective::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Perspective::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
