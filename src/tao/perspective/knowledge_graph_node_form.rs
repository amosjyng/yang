use crate::tao::perspective::Perspective;
use crate::tao::relation::flag::{
    ArchetypeAnalogue, AttributeAnalogue, DataAnalogue, Imported, NewlyDefined, RootAnalogue,
    RootArchetypeAnalogue,
};
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Look at all information as knowledge graph entities.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KnowledgeGraphNode {
    base: FinalNode,
}

impl Debug for KnowledgeGraphNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("KnowledgeGraphNode", self, f)
    }
}

impl From<usize> for KnowledgeGraphNode {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for KnowledgeGraphNode {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for KnowledgeGraphNode {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for KnowledgeGraphNode {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for KnowledgeGraphNode {
    type ArchetypeForm = Archetype;
    type Form = KnowledgeGraphNode;

    const TYPE_ID: usize = YIN_MAX_ID + 8;
    const TYPE_NAME: &'static str = "knowledge-graph-node";
    const PARENT_TYPE_ID: usize = Perspective::TYPE_ID;
}

impl FormTrait for KnowledgeGraphNode {}

impl From<KnowledgeGraphNode> for Perspective {
    fn from(this: KnowledgeGraphNode) -> Perspective {
        Perspective::from(this.base)
    }
}

impl KnowledgeGraphNode {
    /// Whether this is marked as having been newly defined as part of the
    /// current build.
    pub fn is_newly_defined(&self) -> bool {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .has_flag(NewlyDefined::TYPE_ID)
    }

    /// Mark this as having been newly defined as part of the current build.
    pub fn mark_newly_defined(&mut self) {
        self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
    }

    /// Whether this is marked as imported from another build.
    pub fn is_imported(&self) -> bool {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .has_flag(Imported::TYPE_ID)
    }

    /// Mark this as imported from another build.
    pub fn mark_imported(&mut self) {
        self.essence_mut().add_flag(Imported::TYPE_ID);
    }

    /// Whether this is marked as logically analogous to an attribute node.
    pub fn is_attribute_analogue(&self) -> bool {
        self.essence().has_flag(AttributeAnalogue::TYPE_ID)
    }

    /// Mark this as logically analogous to an attribute node.
    pub fn mark_attribute_analogue(&mut self) {
        self.essence_mut().add_flag(AttributeAnalogue::TYPE_ID);
    }

    /// Whether this is marked as logically analogous to the root node.
    pub fn is_root_analogue(&self) -> bool {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .has_flag(RootAnalogue::TYPE_ID)
    }

    /// Mark this as logically analogous to the root node.
    pub fn mark_root_analogue(&mut self) {
        self.essence_mut().add_flag(RootAnalogue::TYPE_ID);
    }

    /// Whether this is marked as logically analogous to the root archetype
    /// node.
    pub fn is_root_archetype_analogue(&self) -> bool {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .has_flag(RootArchetypeAnalogue::TYPE_ID)
    }

    /// Mark this as logically analogous to the root archetype node.
    pub fn mark_root_archetype_analogue(&mut self) {
        self.essence_mut().add_flag(RootArchetypeAnalogue::TYPE_ID);
    }

    /// Whether this is marked as logically analogous to an archetype node.
    pub fn is_archetype_analogue(&self) -> bool {
        self.essence().has_flag(ArchetypeAnalogue::TYPE_ID)
    }

    /// Mark this as logically analogous to an archetype node.
    pub fn mark_archetype_analogue(&mut self) {
        self.essence_mut().add_flag(ArchetypeAnalogue::TYPE_ID);
    }

    /// Whether this is marked as logically analogous to a data node.
    pub fn is_data_analogue(&self) -> bool {
        self.essence().has_flag(DataAnalogue::TYPE_ID)
    }

    /// Mark this as logically analogous to a data node.
    pub fn mark_data_analogue(&mut self) {
        self.essence_mut().add_flag(DataAnalogue::TYPE_ID);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::perspective::KnowledgeGraphNode;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(
            KnowledgeGraphNode::archetype().id(),
            KnowledgeGraphNode::TYPE_ID
        );
        assert_eq!(
            KnowledgeGraphNode::archetype().internal_name_str(),
            Some(Rc::from(KnowledgeGraphNode::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = KnowledgeGraphNode::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            KnowledgeGraphNode::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(KnowledgeGraphNode::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(KnowledgeGraphNode::archetype().added_attributes(), vec![]);
        assert_eq!(KnowledgeGraphNode::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = KnowledgeGraphNode::new();
        let concept_copy = KnowledgeGraphNode::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = KnowledgeGraphNode::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn test_mark_and_check_newly_defined() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_newly_defined());

        new_instance.mark_newly_defined();
        assert!(new_instance.is_newly_defined());
    }

    #[test]
    fn test_newly_defined_non_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_newly_defined());

        KnowledgeGraphNode::from(new_type.id()).mark_newly_defined();
        assert!(!new_instance.is_newly_defined());
    }

    #[test]
    fn test_mark_and_check_imported() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_imported());

        new_instance.mark_imported();
        assert!(new_instance.is_imported());
    }

    #[test]
    fn test_imported_non_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_imported());

        KnowledgeGraphNode::from(new_type.id()).mark_imported();
        assert!(!new_instance.is_imported());
    }

    #[test]
    fn test_mark_and_check_attribute_analogue() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_attribute_analogue());

        new_instance.mark_attribute_analogue();
        assert!(new_instance.is_attribute_analogue());
    }

    #[test]
    fn test_attribute_analogue_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_attribute_analogue());

        KnowledgeGraphNode::from(new_type.id()).mark_attribute_analogue();
        assert!(new_instance.is_attribute_analogue());
    }

    #[test]
    fn test_mark_and_check_root_analogue() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_root_analogue());

        new_instance.mark_root_analogue();
        assert!(new_instance.is_root_analogue());
    }

    #[test]
    fn test_root_analogue_non_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_root_analogue());

        KnowledgeGraphNode::from(new_type.id()).mark_root_analogue();
        assert!(!new_instance.is_root_analogue());
    }

    #[test]
    fn test_mark_and_check_root_archetype_analogue() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_root_archetype_analogue());

        new_instance.mark_root_archetype_analogue();
        assert!(new_instance.is_root_archetype_analogue());
    }

    #[test]
    fn test_root_archetype_analogue_non_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_root_archetype_analogue());

        KnowledgeGraphNode::from(new_type.id()).mark_root_archetype_analogue();
        assert!(!new_instance.is_root_archetype_analogue());
    }

    #[test]
    fn test_mark_and_check_archetype_analogue() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_archetype_analogue());

        new_instance.mark_archetype_analogue();
        assert!(new_instance.is_archetype_analogue());
    }

    #[test]
    fn test_archetype_analogue_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_archetype_analogue());

        KnowledgeGraphNode::from(new_type.id()).mark_archetype_analogue();
        assert!(new_instance.is_archetype_analogue());
    }

    #[test]
    fn test_mark_and_check_data_analogue() {
        initialize_kb();
        let mut new_instance = KnowledgeGraphNode::new();
        assert!(!new_instance.is_data_analogue());

        new_instance.mark_data_analogue();
        assert!(new_instance.is_data_analogue());
    }

    #[test]
    fn test_data_analogue_inheritance() {
        initialize_kb();
        let new_type = KnowledgeGraphNode::archetype().individuate_as_archetype();
        let new_instance = KnowledgeGraphNode::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_data_analogue());

        KnowledgeGraphNode::from(new_type.id()).mark_data_analogue();
        assert!(new_instance.is_data_analogue());
    }
}
