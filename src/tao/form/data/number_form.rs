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

/// The concept of numbers.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    base: FinalNode,
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Number", self, f)
    }
}

impl From<usize> for Number {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Number {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Number {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Number {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Number {
    type ArchetypeForm = Archetype;
    type Form = Number;

    const TYPE_ID: usize = YIN_MAX_ID + 17;
    const TYPE_NAME: &'static str = "number";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl FormTrait for Number {}

impl Number {
    /// Set usize value for this concept.
    pub fn set_value(&mut self, value: usize) {
        self.essence_mut()
            .set_value(Rc::new(StrongValue::new(value)));
    }

    /// Retrieve usize-valued StrongValue.
    pub fn value(&self) -> Option<Rc<usize>> {
        unwrap_value::<usize>(self.essence().value())
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
        assert_eq!(Number::archetype().id(), Number::TYPE_ID);
        assert_eq!(
            Number::archetype().internal_name(),
            Some(Rc::new(Number::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Number::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Number::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Number::individuate();
        let concept_copy = Number::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Number::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Number::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Number::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Number::individuate();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = Number::individuate();
        assert_eq!(concept.value(), None);
    }

    #[test]
    fn get_value_some() {
        initialize_kb();
        let mut concept = Number::individuate();
        concept.set_value(0);
        assert_eq!(concept.value(), Some(Rc::new(0)));
    }
}
