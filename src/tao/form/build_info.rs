use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{unwrap_strong, StrongValue};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::{FormTrait, Tao, YIN_MAX_ID};

/// Represents build information about a generated concept.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuildInfo {
    base: FinalNode,
}

/// Contains all information about a generated concept. Because it's too difficult to store things
/// in the KB right now, we'll use a custom struct for now.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildInfoConfig {
    /// Crate this concept was built as a part of.
    pub crate_name: String,
    /// Import path the concept ended up at, relative to the crate.
    pub struct_path: String,
    /// Absolute file path for the generated concept.
    pub file_path: String,
}

impl BuildInfo {
    /// Set the build config.
    pub fn set_config(&mut self, config: BuildInfoConfig) {
        self.essence_mut()
            .set_value(Rc::new(StrongValue::new(config)));
    }

    /// Retrieve the config stored for this BuildInfo.
    pub fn config(&self) -> Option<BuildInfoConfig> {
        unwrap_strong::<BuildInfoConfig>(&self.essence().value()).cloned()
    }
}

impl Debug for BuildInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("BuildInfo", self, f)
    }
}

impl From<usize> for BuildInfo {
    fn from(id: usize) -> Self {
        BuildInfo {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for BuildInfo {
    fn from(f: FinalNode) -> Self {
        BuildInfo { base: f }
    }
}

impl<'a> TryFrom<&'a str> for BuildInfo {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for BuildInfo {
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

impl<'a> ArchetypeTrait<'a> for BuildInfo {
    type ArchetypeForm = Archetype;
    type Form = BuildInfo;

    const TYPE_ID: usize = YIN_MAX_ID + 11;
    const TYPE_NAME: &'static str = "build-info";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for BuildInfo {
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

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(BuildInfo::archetype().id(), BuildInfo::TYPE_ID);
        assert_eq!(
            BuildInfo::archetype().internal_name(),
            Some(Rc::new(BuildInfo::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = BuildInfo::individuate();
        let concept_copy = BuildInfo::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = BuildInfo::individuate();
        let concept2 = BuildInfo::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = BuildInfo::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn set_and_retrieve_config() {
        initialize_kb();
        let mut info = BuildInfo::individuate();
        info.set_config(BuildInfoConfig {
            crate_name: "zamm_yang".to_owned(),
            struct_path: "here::Yo".to_owned(),
            file_path: "/home/zamm/Documents/zamm/yang/src/here/yo.rs".to_owned(),
        });
        assert_eq!(
            info.config(),
            Some(BuildInfoConfig {
                crate_name: "zamm_yang".to_owned(),
                struct_path: "here::Yo".to_owned(),
                file_path: "/home/zamm/Documents/zamm/yang/src/here/yo.rs".to_owned(),
            })
        );
    }
}
