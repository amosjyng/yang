use crate::tao::form::{BuildInfo, BuildInfoExtension, Crate};
use crate::tao::relation::attribute::SupportsMembership;
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

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
}

impl SupportsMembership for Crate {}
impl CrateExtension for Crate {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

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
}
