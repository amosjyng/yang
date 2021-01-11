use crate::tao::archetype::rust_item_archetype::RustItemArchetype;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// An item recognized by the Rust programming language.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustItem {
    base: FinalNode,
}

impl Debug for RustItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("RustItem", self, f)
    }
}

impl From<usize> for RustItem {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for RustItem {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for RustItem {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for RustItem {
    type ArchetypeForm = RustItemArchetype;
    type Form = RustItem;

    const TYPE_ID: usize = YIN_MAX_ID + 1;
    const TYPE_NAME: &'static str = "rust-item";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl Deref for RustItem {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for RustItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for RustItem {}

impl From<RustItem> for Tao {
    fn from(this: RustItem) -> Tao {
        Tao::from(this.base)
    }
}

impl From<RustItem> for Form {
    fn from(this: RustItem) -> Form {
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
        assert_eq!(RustItem::archetype().id(), RustItem::TYPE_ID);
        assert_eq!(
            RustItem::archetype().internal_name(),
            Some(Rc::from(RustItem::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RustItem::new();
        concept.set_internal_name("A");
        assert_eq!(RustItem::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(RustItem::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(RustItem::archetype().added_attributes(), vec![]);
        assert_eq!(RustItem::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = RustItem::new();
        let concept_copy = RustItem::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = RustItem::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
