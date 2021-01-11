use crate::tao::form::rust_item::data::StrConcept;
use crate::tao::form::rust_item::Module;
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

/// Marks the owner module as re-exporting the value symbol.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReExport {
    base: FinalNode,
}

impl Debug for ReExport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ReExport", self, f)
    }
}

impl From<usize> for ReExport {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for ReExport {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for ReExport {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for ReExport {
    type ArchetypeForm = AttributeArchetype;
    type Form = ReExport;

    const TYPE_ID: usize = YIN_MAX_ID + 32;
    const TYPE_NAME: &'static str = "re-export";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for ReExport {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for ReExport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for ReExport {}

impl From<ReExport> for Tao {
    fn from(this: ReExport) -> Tao {
        Tao::from(this.base)
    }
}

impl From<ReExport> for Relation {
    fn from(this: ReExport) -> Relation {
        Relation::from(this.base)
    }
}

impl From<ReExport> for Attribute {
    fn from(this: ReExport) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for ReExport {
    type OwnerForm = Module;
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
        assert_eq!(ReExport::archetype().id(), ReExport::TYPE_ID);
        assert_eq!(
            ReExport::archetype().internal_name(),
            Some(Rc::from(ReExport::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ReExport::new();
        concept.set_internal_name("A");
        assert_eq!(ReExport::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(ReExport::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(ReExport::archetype().added_attributes(), vec![]);
        assert_eq!(
            ReExport::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ReExport::new();
        let concept_copy = ReExport::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = ReExport::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            ReExport::archetype().owner_archetype(),
            Module::archetype().into()
        );
        assert_eq!(
            ReExport::archetype().value_archetype(),
            StrConcept::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = ReExport::new();
        let owner_of_instance = Module::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = ReExport::new();
        let value_of_instance = StrConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
