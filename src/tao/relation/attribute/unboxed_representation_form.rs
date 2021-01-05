use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// The syntax used to refer to an unboxed version of this primitive.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnboxedRepresentation {
    base: FinalNode,
}

impl Debug for UnboxedRepresentation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("UnboxedRepresentation", self, f)
    }
}

impl From<usize> for UnboxedRepresentation {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for UnboxedRepresentation {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for UnboxedRepresentation {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for UnboxedRepresentation {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for UnboxedRepresentation {
    type ArchetypeForm = AttributeArchetype;
    type Form = UnboxedRepresentation;

    const TYPE_ID: usize = YIN_MAX_ID + 6;
    const TYPE_NAME: &'static str = "unboxed-representation";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for UnboxedRepresentation {}

impl From<UnboxedRepresentation> for Attribute {
    fn from(this: UnboxedRepresentation) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for UnboxedRepresentation {
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
        assert_eq!(
            UnboxedRepresentation::archetype().id(),
            UnboxedRepresentation::TYPE_ID
        );
        assert_eq!(
            UnboxedRepresentation::archetype().internal_name_str(),
            Some(Rc::from(UnboxedRepresentation::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UnboxedRepresentation::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            UnboxedRepresentation::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(UnboxedRepresentation::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            UnboxedRepresentation::archetype().added_attributes(),
            vec![]
        );
        assert_eq!(
            UnboxedRepresentation::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = UnboxedRepresentation::new();
        let concept_copy = UnboxedRepresentation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = UnboxedRepresentation::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            UnboxedRepresentation::archetype().owner_archetype(),
            Tao::archetype()
        );
        assert_eq!(
            UnboxedRepresentation::archetype().value_archetype(),
            Tao::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = UnboxedRepresentation::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = UnboxedRepresentation::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
