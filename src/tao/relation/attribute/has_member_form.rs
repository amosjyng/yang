use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks the value as being part of the owner. The owner should presumably be a
/// collection of some sort.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasMember {
    base: FinalNode,
}

impl Debug for HasMember {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasMember", self, f)
    }
}

impl From<usize> for HasMember {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for HasMember {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for HasMember {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for HasMember {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for HasMember {
    type ArchetypeForm = AttributeArchetype;
    type Form = HasMember;

    const TYPE_ID: usize = YIN_MAX_ID + 8;
    const TYPE_NAME: &'static str = "has-member";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for HasMember {}

impl AttributeTrait for HasMember {
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
        assert_eq!(HasMember::archetype().id(), HasMember::TYPE_ID);
        assert_eq!(
            HasMember::archetype().internal_name_str(),
            Some(Rc::from(HasMember::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(HasMember::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(HasMember::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasMember::new();
        let concept_copy = HasMember::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasMember::new();
        concept.set_internal_name_str("A");
        assert_eq!(HasMember::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(HasMember::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = HasMember::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            HasMember::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            HasMember::archetype().value_archetype(),
            Tao::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasMember::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasMember::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
