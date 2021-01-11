use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for OwnModule {
    type ArchetypeForm = Archetype;
    type Form = OwnModule;

    const TYPE_ID: usize = YIN_MAX_ID + 34;
    const TYPE_NAME: &'static str = "own-module";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for OwnModule {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for OwnModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for OwnModule {}

impl From<OwnModule> for Tao {
    fn from(this: OwnModule) -> Tao {
        Tao::from(this.base)
    }
}

impl From<OwnModule> for Relation {
    fn from(this: OwnModule) -> Relation {
        Relation::from(this.base)
    }
}

impl From<OwnModule> for Flag {
    fn from(this: OwnModule) -> Flag {
        Flag::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(OwnModule::archetype().id(), OwnModule::TYPE_ID);
        assert_eq!(
            OwnModule::archetype().internal_name(),
            Some(Rc::from(OwnModule::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = OwnModule::new();
        concept.set_internal_name("A");
        assert_eq!(OwnModule::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(OwnModule::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(OwnModule::archetype().added_attributes(), vec![]);
        assert_eq!(
            OwnModule::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = OwnModule::new();
        let concept_copy = OwnModule::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = OwnModule::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
