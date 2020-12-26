use crate::tao::relation::attribute::MostProminentMember;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

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

impl Wrapper for Module {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Module {
    type ArchetypeForm = Archetype;
    type Form = Module;

    const TYPE_ID: usize = YIN_MAX_ID + 18;
    const TYPE_NAME: &'static str = "module";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl FormTrait for Module {}

impl From<Module> for Form {
    fn from(this: Module) -> Form {
        Form::from(this.base)
    }
}

impl Module {
    /// Get the most prominent member of the module. By default, the name of the
    /// module will be the same as the name of this member.
    pub fn most_prominent_member(&self) -> Option<Form> {
        self.essence()
            .outgoing_nodes(MostProminentMember::TYPE_ID)
            .first()
            .map(|f| Form::from(f.id()))
    }

    /// Set the most prominent member of the module. By default, the name of the
    /// module will be the same as the name of this member.
    pub fn set_most_prominent_member(&mut self, most_prominent_member: &Form) {
        self.essence_mut().add_outgoing(
            MostProminentMember::TYPE_ID,
            most_prominent_member.essence(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::form::Module;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::MostProminentMember;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Module::archetype().id(), Module::TYPE_ID);
        assert_eq!(
            Module::archetype().internal_name_str(),
            Some(Rc::from(Module::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Module::new();
        concept.set_internal_name_str("A");
        assert_eq!(Module::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Module::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Module::archetype().added_attributes(),
            vec![MostProminentMember::archetype()]
        );
        assert_eq!(
            Module::archetype().attributes(),
            vec![MostProminentMember::archetype()]
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
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
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
}
