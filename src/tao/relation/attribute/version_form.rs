use crate::tao::form::rust_item::data::StrConcept;
use crate::tao::form::rust_item::Crate;
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

/// Version number for a versioned object.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    base: FinalNode,
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Version", self, f)
    }
}

impl From<usize> for Version {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Version {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Version {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Version {
    type ArchetypeForm = AttributeArchetype;
    type Form = Version;

    const TYPE_ID: usize = YIN_MAX_ID + 37;
    const TYPE_NAME: &'static str = "version";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for Version {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Version {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Version {}

impl From<Version> for Tao {
    fn from(this: Version) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Version> for Relation {
    fn from(this: Version) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Version> for Attribute {
    fn from(this: Version) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for Version {
    type OwnerForm = Crate;
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
        assert_eq!(Version::archetype().id(), Version::TYPE_ID);
        assert_eq!(
            Version::archetype().internal_name(),
            Some(Rc::from(Version::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Version::new();
        concept.set_internal_name("A");
        assert_eq!(Version::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Version::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Version::archetype().added_attributes(), vec![]);
        assert_eq!(
            Version::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Version::new();
        let concept_copy = Version::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Version::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Version::archetype().owner_archetype(),
            Crate::archetype().into()
        );
        assert_eq!(
            Version::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Version::new();
        let owner_of_instance = Crate::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Version::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
