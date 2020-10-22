use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{KBValue, StrongValue};
#[rustfmt::skip]
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::data::Data;
use zamm_yin::tao::{FormTrait, YIN_MAX_ID};

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

impl<'a> ArchetypeTrait<'a> for StringConcept {
    type ArchetypeForm = Archetype;
    type Form = StringConcept;

    const TYPE_ID: usize = YIN_MAX_ID + 8;
    const TYPE_NAME: &'static str = "string-concept";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl FormTrait for StringConcept {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl StringConcept {
    /// Set String value for this concept.
    pub fn set_value(&mut self, value: String) {
        self.essence_mut()
            .set_value(Rc::new(StrongValue::new(value)));
    }

    /// Retrieve String-valued StrongValue.
    pub fn value(&self) -> Option<Rc<dyn KBValue>> {
        self.essence().value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::graph::value_wrappers::unwrap_strong;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

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

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = StringConcept::individuate();
        assert_eq!(unwrap_strong::<String>(&concept.value()), None);
    }

    #[test]
    fn get_value_string() {
        initialize_kb();
        let mut concept = StringConcept::individuate();
        concept.set_value("value".to_owned());
        assert_eq!(
            unwrap_strong::<String>(&concept.value()),
            Some(&"value".to_owned())
        );
    }
}
