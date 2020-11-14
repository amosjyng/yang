use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::{FormTrait, Tao, YIN_MAX_ID};

/// Marks an archetype and all its descendants as requiring data-specific logic
/// during generation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsesDataLogic {
    base: FinalNode,
}

impl Debug for UsesDataLogic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("UsesDataLogic", self, f)
    }
}

impl From<usize> for UsesDataLogic {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for UsesDataLogic {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for UsesDataLogic {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for UsesDataLogic {
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

impl<'a> ArchetypeTrait<'a> for UsesDataLogic {
    type ArchetypeForm = Archetype;
    type Form = UsesDataLogic;

    const TYPE_ID: usize = YIN_MAX_ID + 9;
    const TYPE_NAME: &'static str = "uses-data-logic";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for UsesDataLogic {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(UsesDataLogic::archetype().id(), UsesDataLogic::TYPE_ID);
        assert_eq!(
            UsesDataLogic::archetype().internal_name(),
            Some(Rc::new(UsesDataLogic::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(UsesDataLogic::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(UsesDataLogic::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = UsesDataLogic::individuate();
        let concept_copy = UsesDataLogic::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = UsesDataLogic::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(UsesDataLogic::try_from("A"), Ok(concept));
        assert!(UsesDataLogic::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = UsesDataLogic::individuate();
        let concept2 = UsesDataLogic::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = UsesDataLogic::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
