use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// A dummy value for a type of data. This helps with testing.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DummyValue {
    base: FinalNode,
}

impl Debug for DummyValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DummyValue", self, f)
    }
}

impl From<usize> for DummyValue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DummyValue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DummyValue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for DummyValue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for DummyValue {
    type ArchetypeForm = AttributeArchetype;
    type Form = DummyValue;

    const TYPE_ID: usize = YIN_MAX_ID + 24;
    const TYPE_NAME: &'static str = "dummy-value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for DummyValue {}

impl From<DummyValue> for Attribute {
    fn from(this: DummyValue) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DummyValue {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::relation::attribute::{Owner, Value};
    use zamm_yin::tao::Tao;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(DummyValue::archetype().id(), DummyValue::TYPE_ID);
        assert_eq!(
            DummyValue::archetype().internal_name_str(),
            Some(Rc::from(DummyValue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DummyValue::new();
        concept.set_internal_name_str("A");
        assert_eq!(DummyValue::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(DummyValue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(DummyValue::archetype().added_attributes(), vec![]);
        assert_eq!(
            DummyValue::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DummyValue::new();
        let concept_copy = DummyValue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DummyValue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(DummyValue::archetype().owner_archetype(), Tao::archetype());
        assert_eq!(DummyValue::archetype().value_archetype(), Tao::archetype());
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DummyValue::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DummyValue::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
