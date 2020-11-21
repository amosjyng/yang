use crate::tao::form::{BuildInfo, BuildInfoExtension, Crate};
use crate::tao::relation::attribute::HasMember;
use std::rc::Rc;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Trait to extend Crate functionality that has not been auto-generated.
pub trait CrateExtension: FormTrait + CommonNodeTrait {
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

    /// Add a member to the crate.
    fn add_member(&mut self, member: &Form) {
        self.essence_mut()
            .add_outgoing(HasMember::TYPE_ID, member.essence());
    }

    /// Get the members of this crate.
    fn members(&self) -> Vec<Form> {
        // no need to worry about inheritance because HasMember is not Inherits
        self.essence()
            .outgoing_nodes(HasMember::TYPE_ID)
            .iter()
            .map(|f| Form::from(*f))
            .collect()
    }
}

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

    #[test]
    fn add_and_retrieve_members() {
        initialize_kb();
        let f = Form::new();
        let mut c = Crate::new();
        c.add_member(&f);
        assert_eq!(c.members(), vec![f]);
    }
}
