use crate::tao::archetype::rust_item_archetype::DataArchetype;
use crate::tao::form::rust_item::data::StrConcept;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for DummyValue {
    type ArchetypeForm = AttributeArchetype;
    type Form = DummyValue;

    const TYPE_ID: usize = YIN_MAX_ID + 12;
    const TYPE_NAME: &'static str = "dummy-value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for DummyValue {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DummyValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for DummyValue {}

impl From<DummyValue> for Tao {
    fn from(this: DummyValue) -> Tao {
        Tao::from(this.base)
    }
}

impl From<DummyValue> for Relation {
    fn from(this: DummyValue) -> Relation {
        Relation::from(this.base)
    }
}

impl From<DummyValue> for Attribute {
    fn from(this: DummyValue) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DummyValue {
    type OwnerForm = DataArchetype;
    type ValueForm = StrConcept;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::relation::attribute::{Owner, Value};

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(DummyValue::archetype().id(), DummyValue::TYPE_ID);
        assert_eq!(
            DummyValue::archetype().internal_name(),
            Some(Rc::from(DummyValue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DummyValue::new();
        concept.set_internal_name("A");
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
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            DummyValue::archetype().owner_archetype(),
            DataArchetype::archetype().into()
        );
        assert_eq!(
            DummyValue::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DummyValue::new();
        let owner_of_instance = DataArchetype::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DummyValue::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
