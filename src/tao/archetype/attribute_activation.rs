use crate::tao::UsesDataLogic;
use crate::tao::relation::flag::{OwnModule, UsesAttributeLogic, UsesRootNodeLogic, NewlyDefined};
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::Tao;

/// Archetype code generation flags defined when reading from a Yin.md
pub trait CodegenFlags: FormTrait + CommonNodeTrait {
    /// Mark a concept as having been newly defined as part of the current build.
    fn mark_newly_defined(&mut self) {
        self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
    }

    /// Whether or not a concept has been newly defined as part of the current build.
    fn is_newly_defined(&self) -> bool {
        self.essence().has_flag(NewlyDefined::TYPE_ID)
    }

    /// Activate root-node-specific logic for this concept during code generation.
    fn activate_root_node_logic(&mut self) {
        self.essence_mut().add_flag(UsesRootNodeLogic::TYPE_ID);
    }

    /// Whether this concept should have root-node-specific logic activated during code generation.
    fn root_node_logic_activated(&self) -> bool {
        self.id() == Tao::TYPE_ID
            || self
                .essence()
                .inheritance_wrapper()
                .base_wrapper()
                .has_flag(UsesRootNodeLogic::TYPE_ID)
    }

    /// Activate attribute-specific logic for this concept during code generation.
    fn activate_attribute_logic(&mut self) {
        self.essence_mut().add_flag(UsesAttributeLogic::TYPE_ID);
    }

    /// Whether this concept should have attribute-specific logic activated during code generation.
    fn attribute_logic_activated(&self) -> bool {
        self.essence().has_flag(UsesAttributeLogic::TYPE_ID)
    }

    /// Activate data-specific logic for this concept during code generation.
    fn activate_data_logic(&mut self) {
        self.essence_mut().add_flag(UsesDataLogic::TYPE_ID);
    }

    /// Whether this concept should have data-specific logic activated during code generation.
    fn data_logic_activated(&self) -> bool {
        self.essence().has_flag(UsesDataLogic::TYPE_ID)
    }

    /// Mark concept to be generated inside its own module.
    fn mark_own_module(&mut self) {
        self.essence_mut().add_flag(OwnModule::TYPE_ID);
    }

    /// Whether or not concept should be generated inside its own module.
    fn force_own_module(&self) -> bool {
        self.essence().has_flag(OwnModule::TYPE_ID)
    }
}

impl CodegenFlags for Archetype {}
impl CodegenFlags for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::Tao;

    #[test]
    fn test_newly_defined() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        assert!(!new_attr.is_newly_defined());

        new_attr.mark_newly_defined();
        assert!(new_attr.is_newly_defined());
    }

    #[test]
    fn test_activation_inherited() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        let sub_attr = new_attr.individuate_as_archetype();
        assert!(!sub_attr.is_newly_defined());

        new_attr.mark_newly_defined();
        assert!(sub_attr.is_newly_defined());
    }

    #[test]
    fn test_root_node_logic_activation() {
        initialize_kb();
        let mut new_root = Tao::archetype().individuate_as_archetype();
        assert!(!new_root.root_node_logic_activated());

        new_root.activate_root_node_logic();
        assert!(new_root.root_node_logic_activated());
    }

    #[test]
    fn test_root_node_logic_activation_if_tao() {
        initialize_kb();
        assert!(Tao::archetype().root_node_logic_activated());
    }

    #[test]
    fn test_root_node_logic_activation_not_inherited() {
        initialize_kb();
        let mut new_root = Tao::archetype().individuate_as_archetype();
        let non_root = new_root.individuate_as_archetype();
        assert!(!non_root.root_node_logic_activated());

        new_root.activate_root_node_logic();
        assert!(!non_root.root_node_logic_activated());
    }

    #[test]
    fn test_attribute_logic_activation() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        assert!(!new_attr.attribute_logic_activated());

        new_attr.activate_attribute_logic();
        assert!(new_attr.attribute_logic_activated());
    }

    #[test]
    fn test_attribute_logic_activation_inherited() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        let sub_attr = new_attr.individuate_as_archetype();
        assert!(!sub_attr.attribute_logic_activated());

        new_attr.activate_attribute_logic();
        assert!(sub_attr.attribute_logic_activated());
    }

    #[test]
    fn test_own_module_activation() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        assert!(!new_attr.force_own_module());

        new_attr.mark_own_module();
        assert!(new_attr.force_own_module());
    }

    #[test]
    fn test_data_activation() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        assert!(!new_attr.data_logic_activated());

        new_attr.activate_data_logic();
        assert!(new_attr.data_logic_activated());
    }
}
