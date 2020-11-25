use crate::tao::perspective::Perspective;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Look at all information as knowledge graph entities.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KnowledgeGraph {
    base: FinalNode,
}

impl Debug for KnowledgeGraph {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("KnowledgeGraph", self, f)
    }
}

impl From<usize> for KnowledgeGraph {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for KnowledgeGraph {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for KnowledgeGraph {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for KnowledgeGraph {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for KnowledgeGraph {
    type ArchetypeForm = Archetype;
    type Form = KnowledgeGraph;

    const TYPE_ID: usize = YIN_MAX_ID + 19;
    const TYPE_NAME: &'static str = "knowledge-graph";
    const PARENT_TYPE_ID: usize = Perspective::TYPE_ID;
}

impl FormTrait for KnowledgeGraph {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::FormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(KnowledgeGraph::archetype().id(), KnowledgeGraph::TYPE_ID);
        assert_eq!(
            KnowledgeGraph::archetype().internal_name_str(),
            Some(Rc::from(KnowledgeGraph::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(KnowledgeGraph::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(KnowledgeGraph::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = KnowledgeGraph::new();
        let concept_copy = KnowledgeGraph::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = KnowledgeGraph::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(KnowledgeGraph::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(KnowledgeGraph::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = KnowledgeGraph::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
