use crate::tao::form::{BuildInfo, BuildInfoExtension, Crate};
use crate::tao::relation::attribute::{SupportsMembership, Version};
use std::convert::TryFrom;
use std::rc::Rc;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::Wrapper;

/// Trait to extend Crate functionality that has not been auto-generated.
pub trait CrateExtension: FormTrait + CommonNodeTrait + SupportsMembership {
    /// Set the public name for the crate.
    fn set_implementation_name(&mut self, name: &str) {
        BuildInfo::from(self.id()).set_implementation_name(name);
    }

    /// Public name for the crate.
    fn implementation_name(&self) -> Option<Rc<str>> {
        BuildInfo::from(self.id()).implementation_name()
    }

    /// Lookup a crate by name.
    fn lookup(name: &str) -> Option<Crate> {
        Crate::archetype()
            .individuals()
            .iter()
            .map(|i| Crate::from(i.id()))
            .find(|c| {
                let crate_name = c.implementation_name();
                crate_name.is_some() && &*crate_name.unwrap() == name
            })
    }

    /// Set the crate version.
    fn set_version(&mut self, version: &str) {
        let mut version_string = StringConcept::new();
        version_string.set_value(version.to_owned());
        self.essence_mut()
            .add_outgoing(Version::TYPE_ID, version_string.essence());
    }

    /// Get the crate version.
    fn version(&self) -> Option<Rc<str>> {
        // no need to worry about inheritance because crates don't inherit from each other.
        self.essence()
            .outgoing_nodes(Version::TYPE_ID)
            .first()
            .map(|f| Rc::from(StringConcept::from(*f).value().unwrap().as_str()))
    }

    /// Name for the Yin crate.
    const YIN_CRATE_NAME: &'static str = "zamm_yin";
    /// Name for the Yang crate.
    const YANG_CRATE_NAME: &'static str = "zamm_yang";
    /// Internal name for the current crate.
    const CURRENT_CRATE_INTERNAL_NAME: &'static str = "DUMMY-crate";

    /// Get the Yin crate as a concept.
    fn yin() -> Crate {
        Crate::lookup(Self::YIN_CRATE_NAME).unwrap()
    }

    /// Get the Yang crate as a concept.
    fn yang() -> Crate {
        Crate::lookup(Self::YANG_CRATE_NAME).unwrap()
    }

    /// Get the current crate as a concept.
    fn current() -> Crate {
        Crate::try_from(Self::CURRENT_CRATE_INTERNAL_NAME).unwrap()
    }
}

impl SupportsMembership for Crate {}
impl CrateExtension for Crate {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

    #[test]
    fn test_const_crate_init() {
        initialize_kb();
        // this not only tests that they are different crates, but also implicitly tests that
        // they've been successfully retrieved, and therefore successfully initialized
        assert_ne!(Crate::yin(), Crate::yang());
        // check current crate exists and has no name
        assert_eq!(Crate::current().implementation_name(), None);
    }

    #[test]
    fn set_and_retrieve_implementation_name() {
        initialize_kb();
        let mut c = Crate::new();
        c.set_implementation_name("Yolo");
        assert_eq!(c.implementation_name(), Some(Rc::from("Yolo")));
    }

    #[test]
    fn test_lookup_crate() {
        initialize_kb();
        let mut c1 = Crate::new();
        c1.set_implementation_name("one");
        let mut c2 = Crate::new();
        c2.set_implementation_name("two");

        assert_eq!(Crate::lookup("one"), Some(c1));
        assert_eq!(Crate::lookup("two"), Some(c2));
        assert_eq!(Crate::lookup("three"), None);
    }

    #[test]
    fn add_and_retrieve_version() {
        initialize_kb();
        let mut c = Crate::new();
        c.set_version("0.1.0");
        assert_eq!(c.version(), Some(Rc::from("0.1.0")));
    }
}
