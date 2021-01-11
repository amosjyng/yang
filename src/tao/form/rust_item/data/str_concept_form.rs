use crate::tao::archetype::rust_item_archetype::DataArchetype;
use crate::tao::form::rust_item::data::Data;
use crate::tao::form::rust_item::RustItem;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{unwrap_value, StrongValue};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// The Rust-specific concept of an immutable string of characters.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrConcept {
    base: FinalNode,
}

impl Debug for StrConcept {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("StrConcept", self, f)
    }
}

impl From<usize> for StrConcept {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for StrConcept {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for StrConcept {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for StrConcept {
    type ArchetypeForm = DataArchetype;
    type Form = StrConcept;

    const TYPE_ID: usize = YIN_MAX_ID + 6;
    const TYPE_NAME: &'static str = "str-concept";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl Deref for StrConcept {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for StrConcept {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for StrConcept {}

impl From<StrConcept> for Tao {
    fn from(this: StrConcept) -> Tao {
        Tao::from(this.base)
    }
}

impl From<StrConcept> for Form {
    fn from(this: StrConcept) -> Form {
        Form::from(this.base)
    }
}

impl From<StrConcept> for RustItem {
    fn from(this: StrConcept) -> RustItem {
        RustItem::from(this.base)
    }
}

impl From<StrConcept> for Data {
    fn from(this: StrConcept) -> Data {
        Data::from(this.base)
    }
}

impl StrConcept {
    /// Set str value for this concept.
    pub fn set_value(&mut self, value: &str) {
        self.deref_mut()
            .set_value(Rc::new(StrongValue::new_rc(Rc::<str>::from(value))));
    }

    /// Retrieve str-valued StrongValue.
    #[allow(clippy::rc_buffer)]
    pub fn value(&self) -> Option<Rc<str>> {
        unwrap_value::<str>(self.deref().value())
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
        assert_eq!(StrConcept::archetype().id(), StrConcept::TYPE_ID);
        assert_eq!(
            StrConcept::archetype().internal_name(),
            Some(Rc::from(StrConcept::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = StrConcept::new();
        concept.set_internal_name("A");
        assert_eq!(StrConcept::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(StrConcept::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(StrConcept::archetype().added_attributes(), vec![]);
        assert_eq!(StrConcept::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = StrConcept::new();
        let concept_copy = StrConcept::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = StrConcept::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = StrConcept::new();
        assert_eq!(concept.value(), None);
    }

    #[test]
    fn get_value_some() {
        initialize_kb();
        let mut concept = StrConcept::new();
        concept.set_value("");
        assert_eq!(concept.value(), Some(Rc::from("")));
    }
}
