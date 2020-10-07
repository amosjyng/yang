//! The target of an implement command.

use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use zamm_yin::concepts::attributes::{Attribute, AttributeTrait};
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::wrappers::{debug_wrapper, CommonNodeTrait, FinalWrapper};

/// The target of an implementation.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Target {
    attr: Attribute,
}

impl Debug for Target {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Target", Box::new(self), f)
    }
}

impl From<usize> for Target {
    fn from(id: usize) -> Self {
        Self {
            attr: Attribute::from(id),
        }
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

impl ArchetypeTrait<Target> for Target {
    const TYPE_ID: usize = YIN_MAX_ID + 2;
    const TYPE_NAME: &'static str = "Target";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            attr: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Target {
    fn essence(&self) -> &FinalWrapper {
        self.attr.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalWrapper {
        self.attr.essence_mut()
    }
}

impl AttributeTrait<Target> for Target {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.attr.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.attr.owner()
    }

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
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
        instance.set_owner(Box::new(&owner_of_owner));
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Target::individuate();
        let value_of_owner = Target::individuate();
        instance.set_value(Box::new(&value_of_owner));
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
