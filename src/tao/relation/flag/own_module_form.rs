use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::YIN_MAX_ID;

/// Marks an archetype as living inside its own module, even if it doesn't have
/// any defined child archetypes yet.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnModule {
    base: FinalNode,
}

impl Debug for OwnModule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("OwnModule", self, f)
    }
}

impl From<usize> for OwnModule {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for OwnModule {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for OwnModule {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for OwnModule {
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

impl<'a> ArchetypeTrait<'a> for OwnModule {
    type ArchetypeForm = Archetype;
    type Form = OwnModule;

    const TYPE_ID: usize = YIN_MAX_ID + 5;
    const TYPE_NAME: &'static str = "own-module";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for OwnModule {
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
        assert_eq!(OwnModule::archetype().id(), OwnModule::TYPE_ID);
        assert_eq!(
            OwnModule::archetype().internal_name(),
            Some(Rc::new(OwnModule::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(OwnModule::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(OwnModule::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = OwnModule::individuate();
        let concept_copy = OwnModule::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = OwnModule::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(OwnModule::try_from("A"), Ok(concept));
        assert!(OwnModule::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = OwnModule::individuate();
        let concept2 = OwnModule::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = OwnModule::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
