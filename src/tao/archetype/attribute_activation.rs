use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode};
use crate::tao::relation::flag::UsesRootNodeLogic;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::Tao;

/// Archetype code generation flags defined when reading from a Yin.md
pub trait CodegenFlags: FormTrait + CommonNodeTrait {
    /// Mark a concept as having been newly defined as part of the current build.
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::mark_newly_defined"
    )]
    fn mark_newly_defined(&mut self) {
        KnowledgeGraphNode::from(self.id()).mark_newly_defined()
    }

    /// Whether or not a concept has been newly defined as part of the current build.
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::is_newly_defined"
    )]
    fn is_newly_defined(&self) -> bool {
        KnowledgeGraphNode::from(self.id()).is_newly_defined()
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
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::mark_attribute_analogue"
    )]
    fn activate_attribute_logic(&mut self) {
        KnowledgeGraphNode::from(self.id()).mark_attribute_analogue()
    }

    /// Whether this concept should have attribute-specific logic activated during code generation.
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::is_attribute_analogue"
    )]
    fn attribute_logic_activated(&self) -> bool {
        KnowledgeGraphNode::from(self.id()).is_attribute_analogue()
    }

    /// Activate data-specific logic for this concept during code generation.
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::mark_data_analogue"
    )]
    fn activate_data_logic(&mut self) {
        KnowledgeGraphNode::from(self.id()).mark_data_analogue();
    }

    /// Whether this concept should have data-specific logic activated during code generation.
    #[deprecated(
        since = "0.1.7",
        note = "Please use KnowledgeGraphNode::is_data_analogue"
    )]
    fn data_logic_activated(&self) -> bool {
        KnowledgeGraphNode::from(self.id()).is_data_analogue()
    }

    /// Mark concept to be generated inside its own module.
    #[deprecated(since = "0.1.7", note = "Please use BuildInfo::mark_own_module")]
    fn mark_own_module(&mut self) {
        BuildInfo::from(self.id()).mark_own_module();
    }

    /// Whether or not concept should be generated inside its own module.
    #[deprecated(since = "0.1.7", note = "Please use BuildInfo::is_own_module")]
    fn force_own_module(&self) -> bool {
        BuildInfo::from(self.id()).is_own_module()
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
}
