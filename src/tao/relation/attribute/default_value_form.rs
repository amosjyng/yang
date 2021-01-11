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

/// The default value of a data structure.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultValue {
    base: FinalNode,
}

impl Debug for DefaultValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DefaultValue", self, f)
    }
}

impl From<usize> for DefaultValue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DefaultValue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DefaultValue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for DefaultValue {
    type ArchetypeForm = AttributeArchetype;
    type Form = DefaultValue;

    const TYPE_ID: usize = YIN_MAX_ID + 8;
    const TYPE_NAME: &'static str = "default-value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for DefaultValue {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DefaultValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for DefaultValue {}

impl From<DefaultValue> for Tao {
    fn from(this: DefaultValue) -> Tao {
        Tao::from(this.base)
    }
}

impl From<DefaultValue> for Relation {
    fn from(this: DefaultValue) -> Relation {
        Relation::from(this.base)
    }
}

impl From<DefaultValue> for Attribute {
    fn from(this: DefaultValue) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DefaultValue {
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
        assert_eq!(DefaultValue::archetype().id(), DefaultValue::TYPE_ID);
        assert_eq!(
            DefaultValue::archetype().internal_name(),
            Some(Rc::from(DefaultValue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DefaultValue::new();
        concept.set_internal_name("A");
        assert_eq!(
            DefaultValue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(DefaultValue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(DefaultValue::archetype().added_attributes(), vec![]);
        assert_eq!(
            DefaultValue::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DefaultValue::new();
        let concept_copy = DefaultValue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DefaultValue::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            DefaultValue::archetype().owner_archetype(),
            DataArchetype::archetype().into()
        );
        assert_eq!(
            DefaultValue::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DefaultValue::new();
        let owner_of_instance = DataArchetype::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DefaultValue::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
