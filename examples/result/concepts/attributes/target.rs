// AUTOGENERATED CODE -- DO NOT EDIT
use std::fmt::{Debug, Formatter, Result};
// AUTOGENERATED CODE -- DO NOT EDIT
use std::rc::Rc;
// AUTOGENERATED CODE -- DO NOT EDIT
use zamm_yin::concepts::attributes::{Attribute, AttributeTrait};
// AUTOGENERATED CODE -- DO NOT EDIT
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
// AUTOGENERATED CODE -- DO NOT EDIT
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};

/// The target of an implement command.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
// AUTOGENERATED CODE -- DO NOT EDIT
pub struct Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    attr: Attribute,
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl Debug for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn fmt(&self, f: &mut Formatter) -> Result {
        // AUTOGENERATED CODE -- DO NOT EDIT
        debug_wrapper("Target", Box::new(self), f)
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl From<usize> for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn from(id: usize) -> Self {
        // AUTOGENERATED CODE -- DO NOT EDIT
        Self {
            // AUTOGENERATED CODE -- DO NOT EDIT
            attr: Attribute::from(id),
        } // AUTOGENERATED CODE -- DO NOT EDIT
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl CommonNodeTrait for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn id(&self) -> usize {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.id()
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn set_internal_name(&mut self, name: String) {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.set_internal_name(name);
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn internal_name(&self) -> Option<Rc<String>> {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.internal_name()
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl ArchetypeTrait<Target> for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    const TYPE_ID: usize = YIN_MAX_ID + 1;
    // AUTOGENERATED CODE -- DO NOT EDIT
    const TYPE_NAME: &'static str = "Target";
    // AUTOGENERATED CODE -- DO NOT EDIT
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn individuate_with_parent(parent_id: usize) -> Self {
        // AUTOGENERATED CODE -- DO NOT EDIT
        Self {
            // AUTOGENERATED CODE -- DO NOT EDIT
            attr: Attribute::individuate_with_parent(parent_id),
        } // AUTOGENERATED CODE -- DO NOT EDIT
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl FormTrait for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn essence(&self) -> &FinalNode {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.essence()
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn essence_mut(&mut self) -> &mut FinalNode {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.essence_mut()
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
impl AttributeTrait<Target> for Target {
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.set_owner(owner);
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn owner(&self) -> Option<Tao> {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.owner()
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.set_value(value);
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    fn value(&self) -> Option<Tao> {
        // AUTOGENERATED CODE -- DO NOT EDIT
        self.attr.value()
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT

// AUTOGENERATED CODE -- DO NOT EDIT
#[cfg(test)]
// AUTOGENERATED CODE -- DO NOT EDIT
mod tests {
    // AUTOGENERATED CODE -- DO NOT EDIT
    use super::*;
    // AUTOGENERATED CODE -- DO NOT EDIT
    use crate::concepts::initialize_kb;

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn check_type_created() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(Target::archetype().id(), Target::TYPE_ID);
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(
            // AUTOGENERATED CODE -- DO NOT EDIT
            Target::archetype().internal_name(),
            // AUTOGENERATED CODE -- DO NOT EDIT
            Some(Rc::new(Target::TYPE_NAME.to_string()))
        ); // AUTOGENERATED CODE -- DO NOT EDIT
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn from_node_id() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let concept = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let concept_copy = Target::from(concept.id());
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(concept.id(), concept_copy.id());
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn create_and_retrieve_node_id() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let concept1 = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let concept2 = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(concept1.id() + 1, concept2.id());
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn create_and_retrieve_node_name() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let mut concept = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        concept.set_internal_name("A".to_string());
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn get_owner() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let mut instance = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let owner_of_owner = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        instance.set_owner(Box::new(&owner_of_owner));
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(instance.value(), None);
    } // AUTOGENERATED CODE -- DO NOT EDIT

    // AUTOGENERATED CODE -- DO NOT EDIT
    #[test]
    // AUTOGENERATED CODE -- DO NOT EDIT
    fn get_value() {
        // AUTOGENERATED CODE -- DO NOT EDIT
        initialize_kb();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let mut instance = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        let value_of_owner = Target::individuate();
        // AUTOGENERATED CODE -- DO NOT EDIT
        instance.set_value(Box::new(&value_of_owner));
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(instance.owner(), None);
        // AUTOGENERATED CODE -- DO NOT EDIT
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    } // AUTOGENERATED CODE -- DO NOT EDIT
} // AUTOGENERATED CODE -- DO NOT EDIT
