use crate::tao::form::rust_item::RustItem;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Meta-object for RustItem meta-attributes.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustItemArchetype {
    base: FinalNode,
}

impl Debug for RustItemArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("RustItemArchetype", self, f)
    }
}

impl From<usize> for RustItemArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for RustItemArchetype {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for RustItemArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for RustItemArchetype {
    type ArchetypeForm = Archetype;
    type Form = RustItemArchetype;

    const TYPE_ID: usize = YIN_MAX_ID + 2;
    const TYPE_NAME: &'static str = "rust-item-archetype";
    const PARENT_TYPE_ID: usize = Archetype::TYPE_ID;
}

impl Deref for RustItemArchetype {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for RustItemArchetype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for RustItemArchetype {}

impl From<RustItemArchetype> for Tao {
    fn from(this: RustItemArchetype) -> Tao {
        Tao::from(this.base)
    }
}

impl From<RustItemArchetype> for Archetype {
    fn from(this: RustItemArchetype) -> Archetype {
        Archetype::from(this.base)
    }
}

impl ArchetypeFormTrait for RustItemArchetype {
    type SubjectForm = RustItem;
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
        assert_eq!(
            RustItemArchetype::archetype().id(),
            RustItemArchetype::TYPE_ID
        );
        assert_eq!(
            RustItemArchetype::archetype().internal_name(),
            Some(Rc::from(RustItemArchetype::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = RustItemArchetype::new();
        concept.set_internal_name("A");
        assert_eq!(
            RustItemArchetype::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(RustItemArchetype::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(RustItemArchetype::archetype().added_attributes(), vec![]);
        assert_eq!(RustItemArchetype::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = RustItemArchetype::new();
        let concept_copy = RustItemArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = RustItemArchetype::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
