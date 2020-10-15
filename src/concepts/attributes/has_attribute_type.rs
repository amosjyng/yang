use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::concepts::attributes::{Attribute, AttributeTrait};
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};

/// Describes instances of an archetype as having certain types of attributes.
///
/// For example, a string may have a length of 5. But on a more meta level, that
/// means that the string has a length property or length "attribute". That's
/// where this attribute comes in.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasAttributeType {
    base: Attribute,
}

impl Debug for HasAttributeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasAttributeType", self, f)
    }
}

impl From<usize> for HasAttributeType {
    fn from(id: usize) -> Self {
        Self {
            base: Attribute::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for HasAttributeType {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Attribute::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for HasAttributeType {
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

impl<'a> ArchetypeTrait<'a, HasAttributeType> for HasAttributeType {
    const TYPE_ID: usize = YIN_MAX_ID + 3;
    const TYPE_NAME: &'static str = "has-attribute-type";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for HasAttributeType {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.base.essence_mut()
    }
}

impl<'a> AttributeTrait<'a, HasAttributeType> for HasAttributeType {
    fn set_owner(&mut self, owner: &dyn FormTrait) {
        self.base.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.base.owner()
    }

    fn set_value(&mut self, value: &dyn FormTrait) {
        self.base.set_value(value);
    }

    fn value(&self) -> Option<Tao> {
        self.base.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(HasAttributeType::archetype().id(), HasAttributeType::TYPE_ID);
        assert_eq!(
            HasAttributeType::archetype().internal_name(),
            Some(Rc::new(HasAttributeType::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasAttributeType::individuate();
        let concept_copy = HasAttributeType::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasAttributeType::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(HasAttributeType::try_from("A"), Ok(concept));
        assert!(HasAttributeType::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = HasAttributeType::individuate();
        let concept2 = HasAttributeType::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = HasAttributeType::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasAttributeType::individuate();
        let owner_of_owner = HasAttributeType::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasAttributeType::individuate();
        let value_of_owner = HasAttributeType::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
