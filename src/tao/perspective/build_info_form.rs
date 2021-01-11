use crate::tao::form::rust_item::data::StrConcept;
use crate::tao::perspective::Perspective;
use crate::tao::relation::attribute::{Alias, ImportPath};
use crate::tao::relation::flag::OwnModule;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for BuildInfo {
    type ArchetypeForm = Archetype;
    type Form = BuildInfo;

    const TYPE_ID: usize = YIN_MAX_ID + 28;
    const TYPE_NAME: &'static str = "build-info";
    const PARENT_TYPE_ID: usize = Perspective::TYPE_ID;
}

impl Deref for BuildInfo {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BuildInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for BuildInfo {}

impl From<BuildInfo> for Tao {
    fn from(this: BuildInfo) -> Tao {
        Tao::from(this.base)
    }
}

impl From<BuildInfo> for Perspective {
    fn from(this: BuildInfo) -> Perspective {
        Perspective::from(this.base)
    }
}

impl BuildInfo {
    /// Whether this is marked as residing in its own Rust module.
    pub fn is_own_module(&self) -> bool {
        self.deref().base_wrapper().has_flag(OwnModule::TYPE_ID)
    }

    /// Mark this as residing in its own Rust module.
    pub fn mark_own_module(&mut self) {
        self.deref_mut().add_flag(OwnModule::TYPE_ID);
    }

    /// Get the import path the Rust implementation ended up at.
    #[allow(clippy::rc_buffer)]
    pub fn import_path(&self) -> Option<Rc<str>> {
        self.deref()
            .base_wrapper()
            .outgoing_nodes(ImportPath::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the import path the Rust implementation ended up at.
    pub fn set_import_path(&mut self, import_path: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(import_path);
        self.deref_mut()
            .add_outgoing(ImportPath::TYPE_ID, value_concept.deref());
    }

    /// Get the alternative import paths for the concept.
    #[allow(clippy::rc_buffer)]
    pub fn aliases(&self) -> Vec<Rc<str>> {
        self.deref()
            .base_wrapper()
            .outgoing_nodes(Alias::TYPE_ID)
            .into_iter()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
            .collect()
    }

    /// Add one of the alternative import paths for the concept.
    pub fn add_alias(&mut self, alias: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(alias);
        self.deref_mut()
            .add_outgoing(Alias::TYPE_ID, value_concept.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::perspective::BuildInfo;
    use crate::tao::relation::attribute::{Alias, ImportPath};
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(BuildInfo::archetype().id(), BuildInfo::TYPE_ID);
        assert_eq!(
            BuildInfo::archetype().internal_name(),
            Some(Rc::from(BuildInfo::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = BuildInfo::new();
        concept.set_internal_name("A");
        assert_eq!(BuildInfo::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(BuildInfo::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            BuildInfo::archetype().added_attributes(),
            vec![ImportPath::archetype(), Alias::archetype()]
        );
        assert_eq!(
            BuildInfo::archetype().attributes(),
            vec![ImportPath::archetype(), Alias::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = BuildInfo::new();
        let concept_copy = BuildInfo::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = BuildInfo::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
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
    fn test_own_module_non_inheritance() {
        initialize_kb();
        let new_type = BuildInfo::archetype().individuate_as_archetype();
        let new_instance = BuildInfo::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_own_module());

        BuildInfo::from(new_type.id()).mark_own_module();
        assert!(!new_instance.is_own_module());
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_import_path() {
        initialize_kb();
        let mut new_instance = BuildInfo::new();
        assert_eq!(new_instance.import_path(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_import_path(value.clone());
        assert_eq!(new_instance.import_path(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_import_path_non_inheritance() {
        initialize_kb();
        let new_type = BuildInfo::archetype().individuate_as_archetype();
        let new_instance = BuildInfo::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.import_path(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        BuildInfo::from(new_type.id()).set_import_path(value);
        assert_eq!(new_instance.import_path(), None);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_import_path_multiple_times() {
        initialize_kb();
        let mut new_instance = BuildInfo::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_import_path(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.import_path(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_import_path(new_value.clone());
        assert_eq!(new_instance.import_path(), Some(Rc::from(new_value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_alias() {
        initialize_kb();
        let mut new_instance = BuildInfo::new();
        assert_eq!(new_instance.aliases(), vec![]);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_alias(value.clone());
        assert_eq!(new_instance.aliases(), vec![Rc::from(value)]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_alias_non_inheritance() {
        initialize_kb();
        let new_type = BuildInfo::archetype().individuate_as_archetype();
        let new_instance = BuildInfo::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.aliases(), vec![]);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        BuildInfo::from(new_type.id()).add_alias(value);
        assert_eq!(new_instance.aliases(), vec![]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_alias_multiple_times() {
        initialize_kb();
        let mut new_instance = BuildInfo::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_alias(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.aliases(), vec![Rc::from(default.clone())]);

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_alias(new_value.clone());
        assert_eq!(
            new_instance.aliases(),
            vec![Rc::from(default), Rc::from(new_value)]
        );
    }
}
