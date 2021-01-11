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

/// The documentation associated with an implementation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Documentation {
    base: FinalNode,
}

impl Debug for Documentation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Documentation", self, f)
    }
}

impl From<usize> for Documentation {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Documentation {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Documentation {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Documentation {
    type ArchetypeForm = AttributeArchetype;
    type Form = Documentation;

    const TYPE_ID: usize = YIN_MAX_ID + 17;
    const TYPE_NAME: &'static str = "documentation";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for Documentation {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Documentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Documentation {}

impl From<Documentation> for Tao {
    fn from(this: Documentation) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Documentation> for Relation {
    fn from(this: Documentation) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Documentation> for Attribute {
    fn from(this: Documentation) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for Documentation {
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
        assert_eq!(Documentation::archetype().id(), Documentation::TYPE_ID);
        assert_eq!(
            Documentation::archetype().internal_name(),
            Some(Rc::from(Documentation::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Documentation::new();
        concept.set_internal_name("A");
        assert_eq!(
            Documentation::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(Documentation::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Documentation::archetype().added_attributes(), vec![]);
        assert_eq!(
            Documentation::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Documentation::new();
        let concept_copy = Documentation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Documentation::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Documentation::archetype().owner_archetype(),
            Implement::archetype().into()
        );
        assert_eq!(
            Documentation::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Documentation::new();
        let owner_of_instance = Implement::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Documentation::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
