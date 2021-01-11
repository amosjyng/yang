use crate::tao::form::Collection;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Marks the value as being part of the owner. The owner should presumably be a
/// collection of some sort.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Member {
    base: FinalNode,
}

impl Debug for Member {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Member", self, f)
    }
}

impl From<usize> for Member {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Member {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Member {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Member {
    type ArchetypeForm = AttributeArchetype;
    type Form = Member;

    const TYPE_ID: usize = YIN_MAX_ID + 31;
    const TYPE_NAME: &'static str = "member";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for Member {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Member {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Member {}

impl From<Member> for Tao {
    fn from(this: Member) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Member> for Relation {
    fn from(this: Member) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Member> for Attribute {
    fn from(this: Member) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for Member {
    type OwnerForm = Collection;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use zamm_yin::tao::relation::attribute::{Owner, Value};
    use zamm_yin::tao::Tao;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Member::archetype().id(), Member::TYPE_ID);
        assert_eq!(
            Member::archetype().internal_name(),
            Some(Rc::from(Member::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Member::new();
        concept.set_internal_name("A");
        assert_eq!(Member::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Member::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Member::archetype().added_attributes(), vec![]);
        assert_eq!(
            Member::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Member::new();
        let concept_copy = Member::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Member::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Member::archetype().owner_archetype(),
            Collection::archetype().into()
        );
        assert_eq!(
            Member::archetype().value_archetype(),
            Tao::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Member::new();
        let owner_of_instance = Collection::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Member::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
