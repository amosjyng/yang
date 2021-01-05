use crate::tao::perspective::BuildInfo;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::{Attribute, AttributeTrait};
use zamm_yin::tao::YIN_MAX_ID;
use zamm_yin::Wrapper;

/// Describes the import path of a defined struct.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImportPath {
    base: FinalNode,
}

impl Debug for ImportPath {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ImportPath", self, f)
    }
}

impl From<usize> for ImportPath {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for ImportPath {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for ImportPath {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for ImportPath {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for ImportPath {
    type ArchetypeForm = AttributeArchetype;
    type Form = ImportPath;

    const TYPE_ID: usize = YIN_MAX_ID + 23;
    const TYPE_NAME: &'static str = "import-path";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for ImportPath {}

impl From<ImportPath> for Attribute {
    fn from(this: ImportPath) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for ImportPath {
    type OwnerForm = BuildInfo;
    type ValueForm = StringConcept;
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
        assert_eq!(ImportPath::archetype().id(), ImportPath::TYPE_ID);
        assert_eq!(
            ImportPath::archetype().internal_name_str(),
            Some(Rc::from(ImportPath::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ImportPath::new();
        concept.set_internal_name_str("A");
        assert_eq!(ImportPath::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(ImportPath::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(ImportPath::archetype().added_attributes(), vec![]);
        assert_eq!(
            ImportPath::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ImportPath::new();
        let concept_copy = ImportPath::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = ImportPath::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            ImportPath::archetype().owner_archetype(),
            BuildInfo::archetype()
        );
        assert_eq!(
            ImportPath::archetype().value_archetype(),
            StringConcept::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = ImportPath::new();
        let owner_of_instance = BuildInfo::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = ImportPath::new();
        let value_of_instance = StringConcept::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
