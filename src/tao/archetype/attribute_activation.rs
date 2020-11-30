use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode};
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;

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
    #[deprecated(
        since = "0.1.8",
        note = "Please use KnowledgeGraphNode::mark_root_analogue"
    )]
    fn activate_root_node_logic(&mut self) {
        KnowledgeGraphNode::from(self.id()).mark_root_analogue();
    }

    /// Whether this concept should have root-node-specific logic activated during code generation.
    #[deprecated(
        since = "0.1.8",
        note = "Please use KnowledgeGraphNode::is_root_analogue"
    )]
    fn root_node_logic_activated(&self) -> bool {
        KnowledgeGraphNode::from(self.id()).is_root_analogue()
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
