use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Crate that a concept was built as a part of.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Crate {
    base: FinalNode,
}

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Crate", self, f)
    }
}

impl From<usize> for Crate {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Crate {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Crate {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Crate {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Crate {
    type ArchetypeForm = AttributeArchetype;
    type Form = Crate;

    const TYPE_ID: usize = YIN_MAX_ID + 12;
    const TYPE_NAME: &'static str = "crate";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for Crate {}

impl AttributeTrait for Crate {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::FormTrait;
    use zamm_yin::tao::relation::attribute::{Owner, Value};
    use zamm_yin::tao::Tao;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Crate::archetype().id(), Crate::TYPE_ID);
        assert_eq!(
            Crate::archetype().internal_name(),
            Some(Rc::new(Crate::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Crate::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(Crate::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Crate::individuate();
        let concept_copy = Crate::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Crate::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Crate::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Crate::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Crate::individuate();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Crate::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            Crate::archetype().value_archetype(),
            Tao::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Crate::individuate();
        let owner_of_instance = Tao::individuate();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Crate::individuate();
        let value_of_instance = Tao::individuate();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
