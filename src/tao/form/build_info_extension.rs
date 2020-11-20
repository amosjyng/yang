use crate::tao::form::BuildInfo;
use crate::tao::relation::attribute::{Crate, ImplementationName, ImportPath};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{unwrap_value, StrongValue};
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::Wrapper;

/// Trait to extend BuildInfo functionality that has not been auto-generated.
pub trait BuildInfoExtension: FormTrait {
    /// Set crate which the object was built as a part of.
    fn set_crate_name(&mut self, name: &str) {
        let mut s = StringConcept::new();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(name.to_owned())));
        self.essence_mut().add_outgoing(Crate::TYPE_ID, s.essence());
    }

    /// Retrieve crate which the object was built as a part of. This is called `crate_name` instead
    /// of just `crate` because `crate` is a reserved keyword in Rust.
    fn crate_name(&self) -> Option<Rc<str>> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(Crate::TYPE_ID)
            .first()
            .map(|s| unwrap_value::<String>(s.value()).map(|rc| Rc::from(rc.as_str())))
            .flatten()
    }

    /// Set import path the concept ended up at, relative to the crate.
    fn set_import_path(&mut self, path: &str) {
        let mut s = StringConcept::new();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(path.to_owned())));
        self.essence_mut()
            .add_outgoing(ImportPath::TYPE_ID, s.essence());
    }

    /// Retrieve import path the concept ended up at, relative to the crate.
    fn import_path(&self) -> Option<Rc<String>> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ImportPath::TYPE_ID)
            .first()
            .map(|s| unwrap_value::<String>(s.value()))
            .flatten()
    }

    /// Set name the concept took on for its actual implementation.
    fn set_implementation_name(&mut self, name: &str) {
        let mut s = StringConcept::new();
        // todo: set using StringConcept API once that is correctly generated once more
        s.essence_mut()
            .set_value(Rc::new(StrongValue::new(name.to_owned())));
        self.essence_mut()
            .add_outgoing(ImplementationName::TYPE_ID, s.essence());
    }

    /// Retrieve name the concept took on for its actual implementation.
    fn implementation_name(&self) -> Option<Rc<String>> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ImplementationName::TYPE_ID)
            .first()
            .map(|s| unwrap_value::<String>(s.value()))
            .flatten()
    }
}

impl BuildInfoExtension for BuildInfo {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::Tao;

    #[test]
    fn set_and_retrieve_crate() {
        initialize_kb();
        let mut info = BuildInfo::new();
        info.set_crate_name("zamm_yang");
        assert_eq!(info.crate_name(), Some(Rc::from("zamm_yang")));
    }

    #[test]
    fn set_and_retrieve_import_path() {
        initialize_kb();
        let mut info = BuildInfo::new();
        info.set_import_path("zamm_yang::import::path");
        assert_eq!(
            info.import_path(),
            Some(Rc::new("zamm_yang::import::path".to_owned()))
        );
    }

    #[test]
    fn set_and_retrieve_implementation_name() {
        initialize_kb();
        let mut info = BuildInfo::new();
        info.set_implementation_name("Yolo");
        assert_eq!(info.implementation_name(), Some(Rc::new("Yolo".to_owned())));
    }

    /// Test that the attributes don't get mixed up.
    #[test]
    fn set_and_retrieve_all() {
        initialize_kb();
        let mut info = BuildInfo::new();
        info.set_crate_name("zamm_yang");
        info.set_import_path("zamm_yang::import::path");
        info.set_implementation_name("Yolo");

        assert_eq!(info.crate_name(), Some(Rc::from("zamm_yang")));
        assert_eq!(
            info.import_path(),
            Some(Rc::new("zamm_yang::import::path".to_owned()))
        );
        assert_eq!(info.implementation_name(), Some(Rc::new("Yolo".to_owned())));
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
