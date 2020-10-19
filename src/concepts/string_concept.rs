use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};

/// The concept of a string of characters.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringConcept {
    base: Tao,
}

impl Debug for StringConcept {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("StringConcept", self, f)
    }
}

impl From<usize> for StringConcept {
    fn from(id: usize) -> Self {
        Self {
            base: Tao::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for StringConcept {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Tao::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for StringConcept {
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

impl<'a> ArchetypeTrait<'a, StringConcept> for StringConcept {
    const TYPE_ID: usize = YIN_MAX_ID + 4;
    const TYPE_NAME: &'static str = "string-concept";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Tao::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for StringConcept {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.base.essence_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(StringConcept::archetype().id(), StringConcept::TYPE_ID);
        assert_eq!(
            StringConcept::archetype().internal_name(),
            Some(Rc::new(StringConcept::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = StringConcept::individuate();
        let concept_copy = StringConcept::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = StringConcept::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(StringConcept::try_from("A"), Ok(concept));
        assert!(StringConcept::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = StringConcept::individuate();
        let concept2 = StringConcept::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = StringConcept::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
