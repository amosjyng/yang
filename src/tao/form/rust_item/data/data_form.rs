use crate::tao::archetype::rust_item_archetype::DataArchetype;
use crate::tao::form::rust_item::RustItem;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Data that actually exist concretely as bits on the machine, as opposed to
/// only existing as a hypothetical, as an idea.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Data {
    base: FinalNode,
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Data", self, f)
    }
}

impl From<usize> for Data {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Data {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Data {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Data {
    type ArchetypeForm = DataArchetype;
    type Form = Data;

    const TYPE_ID: usize = YIN_MAX_ID + 3;
    const TYPE_NAME: &'static str = "data";
    const PARENT_TYPE_ID: usize = RustItem::TYPE_ID;
}

impl Deref for Data {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Data {}

impl From<Data> for Tao {
    fn from(this: Data) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Data> for Form {
    fn from(this: Data) -> Form {
        Form::from(this.base)
    }
}

impl From<Data> for RustItem {
    fn from(this: Data) -> RustItem {
        RustItem::from(this.base)
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
        assert_eq!(Data::archetype().id(), Data::TYPE_ID);
        assert_eq!(
            Data::archetype().internal_name(),
            Some(Rc::from(Data::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Data::new();
        concept.set_internal_name("A");
        assert_eq!(Data::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Data::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Data::archetype().added_attributes(), vec![]);
        assert_eq!(Data::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Data::new();
        let concept_copy = Data::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Data::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
