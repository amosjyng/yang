use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
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

    const TYPE_ID: usize = YIN_MAX_ID + 7;
    const TYPE_NAME: &'static str = "module";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl FormTrait for Module {}

impl From<Module> for Form {
    fn from(this: Module) -> Form {
        Form::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

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
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Module::archetype().added_attributes(), vec![]);
        assert_eq!(Module::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Module::new();
        let concept_copy = Module::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
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
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Module::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
