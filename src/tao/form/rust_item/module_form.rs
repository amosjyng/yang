use crate::tao::archetype::rust_item_archetype::RustItemArchetype;
use crate::tao::form::rust_item::data::StrConcept;
use crate::tao::form::rust_item::RustItem;
use crate::tao::relation::attribute::{MostProminentMember, ReExport};
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Concept representing a Rust module.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Module {
    base: FinalNode,
}

impl Debug for Module {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Module", self, f)
    }
}

impl From<usize> for Module {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Module {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Module {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Module {
    type ArchetypeForm = RustItemArchetype;
    type Form = Module;

    const TYPE_ID: usize = YIN_MAX_ID + 29;
    const TYPE_NAME: &'static str = "module";
    const PARENT_TYPE_ID: usize = RustItem::TYPE_ID;
}

impl Deref for Module {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Module {}

impl From<Module> for Tao {
    fn from(this: Module) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Module> for Form {
    fn from(this: Module) -> Form {
        Form::from(this.base)
    }
}

impl From<Module> for RustItem {
    fn from(this: Module) -> RustItem {
        RustItem::from(this.base)
    }
}

impl Module {
    /// Get a symbol to be re-exported from this module.
    #[allow(clippy::rc_buffer)]
    pub fn re_exports(&self) -> Vec<Rc<str>> {
        self.deref()
            .outgoing_nodes(ReExport::TYPE_ID)
            .into_iter()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
            .collect()
    }

    /// Add one of a symbol to be re-exported from this module.
    pub fn add_re_export(&mut self, re_export: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(re_export);
        self.deref_mut()
            .add_outgoing(ReExport::TYPE_ID, value_concept.deref());
    }

    /// Get the most prominent member of the module. By default, the name of the
    /// module will be the same as the name of this member.
    pub fn most_prominent_member(&self) -> Option<Form> {
        self.deref()
            .outgoing_nodes(MostProminentMember::TYPE_ID)
            .last()
            .map(|f| Form::from(f.id()))
    }

    /// Set the most prominent member of the module. By default, the name of the
    /// module will be the same as the name of this member.
    pub fn set_most_prominent_member(&mut self, most_prominent_member: &Form) {
        self.deref_mut()
            .add_outgoing(MostProminentMember::TYPE_ID, most_prominent_member.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::form::rust_item::Module;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{MostProminentMember, ReExport};
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Module::archetype().id(), Module::TYPE_ID);
        assert_eq!(
            Module::archetype().internal_name(),
            Some(Rc::from(Module::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Module::new();
        concept.set_internal_name("A");
        assert_eq!(Module::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Module::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Module::archetype().added_attributes(),
            vec![ReExport::archetype(), MostProminentMember::archetype()]
        );
        assert_eq!(
            Module::archetype().attributes(),
            vec![ReExport::archetype(), MostProminentMember::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Module::new();
        let concept_copy = Module::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Module::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_re_export() {
        initialize_kb();
        let mut new_instance = Module::new();
        assert_eq!(new_instance.re_exports(), vec![]);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_re_export(value.clone());
        assert_eq!(new_instance.re_exports(), vec![Rc::from(value)]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_re_export_inheritance() {
        initialize_kb();
        let new_type = Module::archetype().individuate_as_archetype();
        let new_instance = Module::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.re_exports(), vec![]);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        Module::from(new_type.id()).add_re_export(value.clone());
        assert_eq!(new_instance.re_exports(), vec![Rc::from(value)]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_re_export_multiple_times() {
        initialize_kb();
        let mut new_instance = Module::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_re_export(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.re_exports(), vec![Rc::from(default.clone())]);

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.add_re_export(new_value.clone());
        assert_eq!(
            new_instance.re_exports(),
            vec![Rc::from(default), Rc::from(new_value)]
        );
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_most_prominent_member() {
        initialize_kb();
        let mut new_instance = Module::new();
        assert_eq!(new_instance.most_prominent_member(), None);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_most_prominent_member(&value);
        assert_eq!(new_instance.most_prominent_member(), Some(value));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_most_prominent_member_inheritance() {
        initialize_kb();
        let new_type = Module::archetype().individuate_as_archetype();
        let new_instance = Module::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.most_prominent_member(), None);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        Module::from(new_type.id()).set_most_prominent_member(&value);
        assert_eq!(new_instance.most_prominent_member(), Some(value));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_most_prominent_member_multiple_times() {
        initialize_kb();
        let mut new_instance = Module::new();
        let default = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_most_prominent_member(&default);
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.most_prominent_member(), Some(default));

        let new_value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_most_prominent_member(&new_value);
        assert_eq!(new_instance.most_prominent_member(), Some(new_value));
    }
}
