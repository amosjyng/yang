use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// A process that mutates the state of the world.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {
    base: FinalNode,
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Action", self, f)
    }
}

impl From<usize> for Action {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Action {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Action {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Action {
    type ArchetypeForm = Archetype;
    type Form = Action;

    const TYPE_ID: usize = YIN_MAX_ID + 13;
    const TYPE_NAME: &'static str = "action";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl Deref for Action {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Action {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Action {}

impl From<Action> for Tao {
    fn from(this: Action) -> Tao {
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
        assert_eq!(Action::archetype().id(), Action::TYPE_ID);
        assert_eq!(
            Action::archetype().internal_name(),
            Some(Rc::from(Action::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Action::new();
        concept.set_internal_name("A");
        assert_eq!(Action::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Action::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Action::archetype().added_attributes(), vec![]);
        assert_eq!(Action::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Action::new();
        let concept_copy = Action::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Action::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
