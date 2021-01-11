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

/// The concept of a string of characters.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringConcept {
    base: FinalNode,
}

impl Debug for StringConcept {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("StringConcept", self, f)
    }
}

impl From<usize> for StringConcept {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for StringConcept {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for StringConcept {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for StringConcept {
    type ArchetypeForm = DataArchetype;
    type Form = StringConcept;

    const TYPE_ID: usize = YIN_MAX_ID + 5;
    const TYPE_NAME: &'static str = "string-concept";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl Deref for StringConcept {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for StringConcept {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for StringConcept {}

impl From<StringConcept> for Tao {
    fn from(this: StringConcept) -> Tao {
        Tao::from(this.base)
    }
}

impl From<StringConcept> for Form {
    fn from(this: StringConcept) -> Form {
        Form::from(this.base)
    }
}

impl From<StringConcept> for RustItem {
    fn from(this: StringConcept) -> RustItem {
        RustItem::from(this.base)
    }
}

impl From<StringConcept> for Data {
    fn from(this: StringConcept) -> Data {
        Data::from(this.base)
    }
}

impl StringConcept {
    /// Set String value for this concept.
    pub fn set_value(&mut self, value: String) {
        self.deref_mut()
            .set_value(Rc::new(StrongValue::new_rc(Rc::<String>::from(value))));
    }

    /// Retrieve String-valued StrongValue.
    #[allow(clippy::rc_buffer)]
    pub fn value(&self) -> Option<Rc<String>> {
        unwrap_value::<String>(self.deref().value())
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
        assert_eq!(StringConcept::archetype().id(), StringConcept::TYPE_ID);
        assert_eq!(
            StringConcept::archetype().internal_name(),
            Some(Rc::from(StringConcept::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = StringConcept::new();
        concept.set_internal_name("A");
        assert_eq!(
            StringConcept::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(StringConcept::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(StringConcept::archetype().added_attributes(), vec![]);
        assert_eq!(StringConcept::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = StringConcept::new();
        let concept_copy = StringConcept::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = StringConcept::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = StringConcept::new();
        assert_eq!(concept.value(), None);
    }

    #[test]
    fn get_value_some() {
        initialize_kb();
        let mut concept = StringConcept::new();
        concept.set_value(String::new());
        assert_eq!(concept.value(), Some(Rc::from(String::new())));
    }
}
