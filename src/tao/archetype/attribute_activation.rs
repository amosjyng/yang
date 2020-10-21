use crate::tao::{OwnModule, UsesAttributeLogic};
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::FormTrait;

/// Archetype code generation flags defined when reading from a Yin.md
pub trait CodegenFlags: FormTrait {
    /// Activate attribute-specific logic for this concept during code generation.
    fn activate_attribute_logic(&mut self) {
        self.essence_mut().add_flag(UsesAttributeLogic::TYPE_ID);
    }

    /// Whether this concept should have attribute-specific logic activated during code generation.
    fn attribute_logic_activated(&self) -> bool {
        self.essence().has_flag(UsesAttributeLogic::TYPE_ID)
    }

    /// Mark concept to be generated inside its own module.
    fn mark_own_module(&mut self) {
        self.essence_mut().add_flag(OwnModule::TYPE_ID);
    }

    /// Whether or not concept should be generated inside its own module.
    fn own_module(&self) -> bool {
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
        assert!(!new_attr.own_module());

        new_attr.mark_own_module();
        assert!(new_attr.own_module());
    }
}
