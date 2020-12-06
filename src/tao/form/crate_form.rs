use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Crate that a concept was built as a part of.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Crate {
    base: FinalNode,
}

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Crate", self, f)
    }
}

impl From<usize> for Crate {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Crate {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Crate {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Crate {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Crate {
    type ArchetypeForm = Archetype;
    type Form = Crate;

    const TYPE_ID: usize = YIN_MAX_ID + 20;
    const TYPE_NAME: &'static str = "crate";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl FormTrait for Crate {}

impl From<Crate> for Form {
    fn from(this: Crate) -> Form {
        Form::from(this.base)
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
        assert_eq!(Crate::archetype().id(), Crate::TYPE_ID);
        assert_eq!(
            Crate::archetype().internal_name_str(),
            Some(Rc::from(Crate::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Crate::archetype().added_attributes(), vec![]);
        assert_eq!(Crate::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Crate::new();
        let concept_copy = Crate::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Crate::new();
        concept.set_internal_name_str("A");
        assert_eq!(Crate::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Crate::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Crate::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
