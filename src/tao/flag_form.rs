use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::{FormTrait, Tao, YIN_MAX_ID};

/// Unary relations.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flag {
    base: FinalNode,
}

impl Debug for Flag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Flag", self, f)
    }
}

impl From<usize> for Flag {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Flag {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Flag {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for Flag {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a> for Flag {
    type ArchetypeForm = Archetype;
    type Form = Flag;

    const TYPE_ID: usize = YIN_MAX_ID + 3;
    const TYPE_NAME: &'static str = "flag";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Flag {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Flag::archetype().id(), Flag::TYPE_ID);
        assert_eq!(
            Flag::archetype().internal_name(),
            Some(Rc::new(Flag::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Flag::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Flag::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Flag::individuate();
        let concept_copy = Flag::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Flag::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Flag::try_from("A"), Ok(concept));
        assert!(Flag::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Flag::individuate();
        let concept2 = Flag::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Flag::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
