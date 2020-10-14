use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::concepts::attributes::{Attribute, AttributeTrait};
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};

/// The target of an implement command.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Target {
    attr: Attribute,
}

impl Debug for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Target", self, f)
    }
}

impl From<usize> for Target {
    fn from(id: usize) -> Self {
        Self {
            attr: Attribute::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for Target {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Attribute::try_from(name).map(|a| Self { attr: a })
    }
}

impl CommonNodeTrait for Target {
    fn id(&self) -> usize {
        self.attr.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.attr.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.attr.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a, Target> for Target {
    const TYPE_ID: usize = YIN_MAX_ID + 2;
    const TYPE_NAME: &'static str = "target";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            attr: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Target {
    fn essence(&self) -> &FinalNode {
        self.attr.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.attr.essence_mut()
    }
}

impl<'a> AttributeTrait<'a, Target> for Target {
    fn set_owner(&mut self, owner: &dyn FormTrait) {
        self.attr.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.attr.owner()
    }

    fn set_value(&mut self, value: &dyn FormTrait) {
        self.attr.set_value(value);
    }

    fn value(&self) -> Option<Tao> {
        self.attr.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Target::archetype().id(), Target::TYPE_ID);
        assert_eq!(
            Target::archetype().internal_name(),
            Some(Rc::new(Target::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Target::individuate();
        let concept_copy = Target::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Target::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Target::try_from("A"), Ok(concept));
        assert!(Target::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Target::individuate();
        let concept2 = Target::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Target::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Target::individuate();
        let owner_of_owner = Target::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Target::individuate();
        let value_of_owner = Target::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
