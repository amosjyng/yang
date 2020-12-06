use crate::tao::perspective::Perspective;
use crate::tao::relation::flag::OwnModule;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Represents build information about a generated concept.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuildInfo {
    base: FinalNode,
}

impl Debug for BuildInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("BuildInfo", self, f)
    }
}

impl From<usize> for BuildInfo {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for BuildInfo {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for BuildInfo {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for BuildInfo {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for BuildInfo {
    type ArchetypeForm = Archetype;
    type Form = BuildInfo;

    const TYPE_ID: usize = YIN_MAX_ID + 17;
    const TYPE_NAME: &'static str = "build-info";
    const PARENT_TYPE_ID: usize = Perspective::TYPE_ID;
}

impl FormTrait for BuildInfo {}

impl From<BuildInfo> for Perspective {
    fn from(this: BuildInfo) -> Perspective {
        Perspective::from(this.base)
    }
}

impl BuildInfo {
    /// Whether this is marked as residing in its own Rust module.
    pub fn is_own_module(&self) -> bool {
        self.essence().has_flag(OwnModule::TYPE_ID)
    }

    /// Mark this as residing in its own Rust module.
    pub fn mark_own_module(&mut self) {
        self.essence_mut().add_flag(OwnModule::TYPE_ID);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::perspective::BuildInfo;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(BuildInfo::archetype().id(), BuildInfo::TYPE_ID);
        assert_eq!(
            BuildInfo::archetype().internal_name_str(),
            Some(Rc::from(BuildInfo::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(BuildInfo::archetype().added_attributes(), vec![]);
        assert_eq!(BuildInfo::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = BuildInfo::new();
        let concept_copy = BuildInfo::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = BuildInfo::new();
        concept.set_internal_name_str("A");
        assert_eq!(BuildInfo::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(BuildInfo::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = BuildInfo::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn test_mark_and_check_own_module() {
        initialize_kb();
        let mut new_instance = BuildInfo::new();
        assert!(!new_instance.is_own_module());

        new_instance.mark_own_module();
        assert!(new_instance.is_own_module());
    }

    #[test]
    fn test_own_module_inheritance() {
        initialize_kb();
        let new_type = BuildInfo::archetype().individuate_as_archetype();
        let new_instance = BuildInfo::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_own_module());

        BuildInfo::from(new_type.id()).mark_own_module();
        assert!(new_instance.is_own_module());
    }
}
