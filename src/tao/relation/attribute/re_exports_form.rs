use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Marks the owner module as re-exporting the value symbol.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReExports {
    base: FinalNode,
}

impl Debug for ReExports {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ReExports", self, f)
    }
}

impl From<usize> for ReExports {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for ReExports {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for ReExports {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for ReExports {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for ReExports {
    type ArchetypeForm = AttributeArchetype;
    type Form = ReExports;

    const TYPE_ID: usize = YIN_MAX_ID + 9;
    const TYPE_NAME: &'static str = "re-exports";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for ReExports {}

impl From<ReExports> for Attribute {
    fn from(this: ReExports) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for ReExports {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    #[rustfmt::skip]
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::relation::attribute::{Owner, Value};
    use zamm_yin::tao::Tao;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(ReExports::archetype().id(), ReExports::TYPE_ID);
        assert_eq!(
            ReExports::archetype().internal_name_str(),
            Some(Rc::from(ReExports::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(ReExports::archetype().added_attributes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(ReExports::archetype().attributes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ReExports::new();
        let concept_copy = ReExports::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ReExports::new();
        concept.set_internal_name_str("A");
        assert_eq!(ReExports::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(ReExports::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = ReExports::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(ReExports::archetype().owner_archetype(), Tao::archetype());
        assert_eq!(ReExports::archetype().value_archetype(), Tao::archetype());
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = ReExports::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = ReExports::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
