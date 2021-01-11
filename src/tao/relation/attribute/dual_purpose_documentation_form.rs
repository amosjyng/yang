use crate::tao::action::Implement;
use crate::tao::form::rust_item::data::StrConcept;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

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

impl ArchetypeTrait for DualPurposeDocumentation {
    type ArchetypeForm = AttributeArchetype;
    type Form = DualPurposeDocumentation;

    const TYPE_ID: usize = YIN_MAX_ID + 18;
    const TYPE_NAME: &'static str = "dual-purpose-documentation";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for DualPurposeDocumentation {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DualPurposeDocumentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for DualPurposeDocumentation {}

impl From<DualPurposeDocumentation> for Tao {
    fn from(this: DualPurposeDocumentation) -> Tao {
        Tao::from(this.base)
    }
}

impl From<DualPurposeDocumentation> for Relation {
    fn from(this: DualPurposeDocumentation) -> Relation {
        Relation::from(this.base)
    }
}

impl From<DualPurposeDocumentation> for Attribute {
    fn from(this: DualPurposeDocumentation) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DualPurposeDocumentation {
    type OwnerForm = Implement;
    type ValueForm = StrConcept;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::relation::attribute::{Owner, Value};

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(
            DualPurposeDocumentation::archetype().id(),
            DualPurposeDocumentation::TYPE_ID
        );
        assert_eq!(
            DualPurposeDocumentation::archetype().internal_name(),
            Some(Rc::from(DualPurposeDocumentation::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DualPurposeDocumentation::new();
        concept.set_internal_name("A");
        assert_eq!(
            DualPurposeDocumentation::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(DualPurposeDocumentation::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            DualPurposeDocumentation::archetype().added_attributes(),
            vec![]
        );
        assert_eq!(
            DualPurposeDocumentation::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DualPurposeDocumentation::new();
        let concept_copy = DualPurposeDocumentation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DualPurposeDocumentation::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            DualPurposeDocumentation::archetype().owner_archetype(),
            Implement::archetype().into()
        );
        assert_eq!(
            DualPurposeDocumentation::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DualPurposeDocumentation::new();
        let owner_of_instance = Implement::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DualPurposeDocumentation::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
