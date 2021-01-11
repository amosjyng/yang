use crate::tao::action::Implement;
use crate::tao::form::rust_item::data::Number;
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

/// The integer ID associated with a concept.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConceptId {
    base: FinalNode,
}

impl Debug for ConceptId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ConceptId", self, f)
    }
}

impl From<usize> for ConceptId {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for ConceptId {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for ConceptId {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for ConceptId {
    type ArchetypeForm = AttributeArchetype;
    type Form = ConceptId;

    const TYPE_ID: usize = YIN_MAX_ID + 16;
    const TYPE_NAME: &'static str = "concept-id";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for ConceptId {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for ConceptId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for ConceptId {}

impl From<ConceptId> for Tao {
    fn from(this: ConceptId) -> Tao {
        Tao::from(this.base)
    }
}

impl From<ConceptId> for Relation {
    fn from(this: ConceptId) -> Relation {
        Relation::from(this.base)
    }
}

impl From<ConceptId> for Attribute {
    fn from(this: ConceptId) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for ConceptId {
    type OwnerForm = Implement;
    type ValueForm = Number;
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
        assert_eq!(ConceptId::archetype().id(), ConceptId::TYPE_ID);
        assert_eq!(
            ConceptId::archetype().internal_name(),
            Some(Rc::from(ConceptId::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ConceptId::new();
        concept.set_internal_name("A");
        assert_eq!(ConceptId::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(ConceptId::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(ConceptId::archetype().added_attributes(), vec![]);
        assert_eq!(
            ConceptId::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ConceptId::new();
        let concept_copy = ConceptId::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = ConceptId::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            ConceptId::archetype().owner_archetype(),
            Implement::archetype().into()
        );
        assert_eq!(
            ConceptId::archetype().value_archetype(),
            Number::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = ConceptId::new();
        let owner_of_instance = Implement::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = ConceptId::new();
        let value_of_instance = Number::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
