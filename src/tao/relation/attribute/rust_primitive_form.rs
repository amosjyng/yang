use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// The Rust primitive that a Yin data concept is implemented by.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustPrimitive {
    base: FinalNode,
}

impl Debug for RustPrimitive {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("RustPrimitive", self, f)
    }
}

impl From<usize> for RustPrimitive {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for RustPrimitive {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for RustPrimitive {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for RustPrimitive {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for RustPrimitive {
    type ArchetypeForm = AttributeArchetype;
    type Form = RustPrimitive;

    const TYPE_ID: usize = YIN_MAX_ID + 12;
    const TYPE_NAME: &'static str = "rust-primitive";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for RustPrimitive {}

impl AttributeTrait for RustPrimitive {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    #[rustfmt::skip]
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::form::FormTrait;
    use zamm_yin::tao::relation::attribute::{Owner, Value};
    use zamm_yin::tao::Tao;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(RustPrimitive::archetype().id(), RustPrimitive::TYPE_ID);
        assert_eq!(
            RustPrimitive::archetype().internal_name_str(),
            Some(Rc::from(RustPrimitive::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(RustPrimitive::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(RustPrimitive::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = RustPrimitive::new();
        let concept_copy = RustPrimitive::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RustPrimitive::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(RustPrimitive::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(RustPrimitive::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = RustPrimitive::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            RustPrimitive::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            RustPrimitive::archetype().value_archetype(),
            Tao::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = RustPrimitive::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = RustPrimitive::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
