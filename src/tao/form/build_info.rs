use crate::tao::attribute::{Crate, ImplementationName, ImportPath};
use crate::tao::StringConcept;
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

impl BuildInfo {
    /// Set crate which the object was built as a part of.
    pub fn set_crate_name(&mut self, name: &str) {
        let mut s = StringConcept::individuate();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(name.to_owned())));
        self.base.add_outgoing(Crate::TYPE_ID, s.essence());
    }

    /// Retrieve crate which the object was built as a part of. This is called `crate_name` instead
    /// of just `crate` because `crate` is a reserved keyword in Rust.
    pub fn crate_name(&self) -> Option<String> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.base
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(Crate::TYPE_ID)
            .first()
            .map(|s| unwrap_strong::<String>(&s.value()).cloned())
            .flatten()
    }

    /// Set import path the concept ended up at, relative to the crate.
    pub fn set_import_path(&mut self, path: &str) {
        let mut s = StringConcept::individuate();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(path.to_owned())));
        self.base.add_outgoing(ImportPath::TYPE_ID, s.essence());
    }

    /// Retrieve import path the concept ended up at, relative to the crate.
    pub fn import_path(&self) -> Option<String> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.base
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ImportPath::TYPE_ID)
            .first()
            .map(|s| unwrap_strong::<String>(&s.value()).cloned())
            .flatten()
    }

    /// Set name the concept took on for its actual implementation.
    pub fn set_implementation_name(&mut self, name: &str) {
        let mut s = StringConcept::individuate();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(name.to_owned())));
        self.base
            .add_outgoing(ImplementationName::TYPE_ID, s.essence());
    }

    /// Retrieve name the concept took on for its actual implementation.
    pub fn implementation_name(&self) -> Option<String> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.base
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ImplementationName::TYPE_ID)
            .first()
            .map(|s| unwrap_strong::<String>(&s.value()).cloned())
            .flatten()
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
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

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
    fn set_and_retrieve_crate() {
        initialize_kb();
        let mut info = BuildInfo::individuate();
        info.set_crate_name("zamm_yang");
        assert_eq!(info.crate_name(), Some("zamm_yang".to_owned()));
    }

    #[test]
    fn set_and_retrieve_import_path() {
        initialize_kb();
        let mut info = BuildInfo::individuate();
        info.set_import_path("zamm_yang::import::path");
        assert_eq!(
            info.import_path(),
            Some("zamm_yang::import::path".to_owned())
        );
    }

    #[test]
    fn set_and_retrieve_implementation_name() {
        initialize_kb();
        let mut info = BuildInfo::individuate();
        info.set_implementation_name("Yolo");
        assert_eq!(info.implementation_name(), Some("Yolo".to_owned()));
    }

    /// Test that the attributes don't get mixed up.
    #[test]
    fn set_and_retrieve_all() {
        initialize_kb();
        let mut info = BuildInfo::individuate();
        info.set_crate_name("zamm_yang");
        info.set_import_path("zamm_yang::import::path");
        info.set_implementation_name("Yolo");

        assert_eq!(info.crate_name(), Some("zamm_yang".to_owned()));
        assert_eq!(
            info.import_path(),
            Some("zamm_yang::import::path".to_owned())
        );
        assert_eq!(info.implementation_name(), Some("Yolo".to_owned()));
    }

    /// Build info should never be inherited
    #[test]
    fn test_no_build_info_inherited() {
        initialize_kb();
        let type1 = Tao::archetype().individuate_as_archetype();
        let mut info = BuildInfo::from(type1.id());
        info.set_crate_name("zamm_yang");
        info.set_import_path("zamm_yang::import::path");
        info.set_implementation_name("Yolo");

        let type2 = type1.individuate_as_archetype();
        let info2 = BuildInfo::from(type2.id());
        assert_eq!(info2.crate_name(), None);
        assert_eq!(info2.import_path(), None);
        assert_eq!(info2.implementation_name(), None);
    }
}
