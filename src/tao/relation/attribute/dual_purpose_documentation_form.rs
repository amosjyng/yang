use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Dual-purpose documentation that can be used in more than one situation.
///
/// For example, the same substring might be usable for both the getter and
/// setter of a string.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DualPurposeDocumentation {
    base: FinalNode,
}

impl Debug for DualPurposeDocumentation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DualPurposeDocumentation", self, f)
    }
}

impl From<usize> for DualPurposeDocumentation {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DualPurposeDocumentation {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DualPurposeDocumentation {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for DualPurposeDocumentation {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for DualPurposeDocumentation {
    type ArchetypeForm = AttributeArchetype;
    type Form = DualPurposeDocumentation;

    const TYPE_ID: usize = YIN_MAX_ID + 5;
    const TYPE_NAME: &'static str = "dual-purpose-documentation";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for DualPurposeDocumentation {}

impl From<DualPurposeDocumentation> for Attribute {
    fn from(this: DualPurposeDocumentation) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DualPurposeDocumentation {
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
        #[rustfmt::skip]
        assert_eq!(DualPurposeDocumentation::archetype().id(), DualPurposeDocumentation::TYPE_ID);
        assert_eq!(
            DualPurposeDocumentation::archetype().internal_name_str(),
            Some(Rc::from(DualPurposeDocumentation::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(DualPurposeDocumentation::archetype().added_attributes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(DualPurposeDocumentation::archetype().attributes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DualPurposeDocumentation::new();
        let concept_copy = DualPurposeDocumentation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DualPurposeDocumentation::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(DualPurposeDocumentation::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(DualPurposeDocumentation::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DualPurposeDocumentation::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            DualPurposeDocumentation::archetype().owner_archetype(),
            Tao::archetype()
        );
        assert_eq!(
            DualPurposeDocumentation::archetype().value_archetype(),
            Tao::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DualPurposeDocumentation::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DualPurposeDocumentation::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
