use super::BuildInfo;
use crate::tao::form::{Crate, CrateExtension, Module};
use crate::tao::relation::attribute::{
    Member, ImplementationName, MostProminentMember, SupportsMembership,
};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::StrongValue;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Trait to extend BuildInfo functionality that has not been auto-generated.
pub trait BuildInfoExtension: FormTrait + CommonNodeTrait {
    /// Set crate which the object was built as a part of.
    fn set_crate_name(&mut self, name: &str) {
        let mut c = match Crate::lookup(name) {
            Some(found_crate) => found_crate,
            None => {
                let mut new_crate = Crate::new();
                new_crate.set_implementation_name(name);
                new_crate
            }
        };
        c.add_member(&Form::from(self.id()));
    }

    /// Retrieve crate which the object was built as a part of. This is called `crate_name` instead
    /// of just `crate` because `crate` is a reserved keyword in Rust.
    fn crate_name(&self) -> Option<Rc<str>> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .incoming_nodes(Member::TYPE_ID)
            .iter()
            .map(|n| Form::from(n.id()))
            .find(|f| f.has_ancestor(Crate::archetype()))
            .map(|c| Crate::from(c.id()).implementation_name())
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
    fn implementation_name(&self) -> Option<Rc<str>> {
        // todo: retrieve using StringConcept API once that is correctly generated once more
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ImplementationName::TYPE_ID)
            .first()
            .map(|s| {
                StringConcept::from(s.id())
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }

    /// Get the module that represents this archetype as its primary concept.
    fn representative_module(&self) -> Option<Module> {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .incoming_nodes(MostProminentMember::TYPE_ID)
            .first()
            .map(|b| Module::from(b.id()))
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
    fn set_and_retrieve_implementation_name() {
        initialize_kb();
        let mut info = BuildInfo::new();
        info.set_implementation_name("Yolo");
        assert_eq!(info.implementation_name(), Some(Rc::from("Yolo")));
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
            Some(Rc::from("zamm_yang::import::path"))
        );
        assert_eq!(info.implementation_name(), Some(Rc::from("Yolo")));
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

    #[test]
    fn test_reversed_module_membership() {
        initialize_kb();
        let my_type = Tao::archetype().individuate_as_archetype();
        let mut module = Module::new();
        module.set_most_prominent_member(&my_type.as_form());
        assert_eq!(
            BuildInfo::from(my_type.id()).representative_module(),
            Some(module)
        );
    }

    #[test]
    fn test_reversed_module_membership_not_inherited() {
        initialize_kb();
        let my_type = Tao::archetype().individuate_as_archetype();
        let my_subtype = my_type.individuate_as_archetype();
        let mut parent_module = Module::new();
        parent_module.set_most_prominent_member(&my_type.as_form());
        let mut child_module = Module::new();
        child_module.set_most_prominent_member(&my_subtype.as_form());
        assert_eq!(
            BuildInfo::from(my_type.id()).representative_module(),
            Some(parent_module)
        );
        assert_eq!(
            BuildInfo::from(my_subtype.id()).representative_module(),
            Some(child_module)
        );
    }
}
