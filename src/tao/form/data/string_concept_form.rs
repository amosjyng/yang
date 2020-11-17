use crate::tao::form::data::Data;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{unwrap_value, StrongValue};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// The concept of a string of characters.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringConcept {
    base: FinalNode,
}

impl Debug for StringConcept {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("StringConcept", self, f)
    }
}

impl From<usize> for StringConcept {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for StringConcept {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for StringConcept {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for StringConcept {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for StringConcept {
    type ArchetypeForm = Archetype;
    type Form = StringConcept;

    const TYPE_ID: usize = YIN_MAX_ID + 7;
    const TYPE_NAME: &'static str = "string-concept";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl FormTrait for StringConcept {}

impl StringConcept {
    /// Set String value for this concept.
    pub fn set_value(&mut self, value: String) {
        self.essence_mut()
            .set_value(Rc::new(StrongValue::new(value)));
    }

    /// Retrieve String-valued StrongValue.
    pub fn value(&self) -> Option<Rc<String>> {
        unwrap_value::<String>(self.essence().value())
    }
}

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
        assert_eq!(StringConcept::archetype().id(), StringConcept::TYPE_ID);
        assert_eq!(
            StringConcept::archetype().internal_name(),
            Some(Rc::new(StringConcept::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(StringConcept::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(StringConcept::archetype().attribute_archetypes(), vec![]);
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
        #[rustfmt::skip]
        assert_eq!(StringConcept::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(StringConcept::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = StringConcept::individuate();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = StringConcept::individuate();
        assert_eq!(concept.value(), None);
    }

    #[test]
    fn get_value_some() {
        initialize_kb();
        let mut concept = StringConcept::individuate();
        concept.set_value(String::new());
        assert_eq!(concept.value(), Some(Rc::new(String::new())));
    }
}
