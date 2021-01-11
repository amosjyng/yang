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

/// The concept of numbers.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    base: FinalNode,
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Number", self, f)
    }
}

impl From<usize> for Number {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Number {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Number {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Number {
    type ArchetypeForm = DataArchetype;
    type Form = Number;

    const TYPE_ID: usize = YIN_MAX_ID + 7;
    const TYPE_NAME: &'static str = "number";
    const PARENT_TYPE_ID: usize = Data::TYPE_ID;
}

impl Deref for Number {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Number {}

impl From<Number> for Tao {
    fn from(this: Number) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Number> for Form {
    fn from(this: Number) -> Form {
        Form::from(this.base)
    }
}

impl From<Number> for RustItem {
    fn from(this: Number) -> RustItem {
        RustItem::from(this.base)
    }
}

impl From<Number> for Data {
    fn from(this: Number) -> Data {
        Data::from(this.base)
    }
}

impl Number {
    /// Set usize value for this concept.
    pub fn set_value(&mut self, value: usize) {
        self.deref_mut()
            .set_value(Rc::new(StrongValue::new_rc(Rc::<usize>::from(value))));
    }

    /// Retrieve usize-valued StrongValue.
    #[allow(clippy::rc_buffer)]
    pub fn value(&self) -> Option<Rc<usize>> {
        unwrap_value::<usize>(self.deref().value())
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
        assert_eq!(Number::archetype().id(), Number::TYPE_ID);
        assert_eq!(
            Number::archetype().internal_name(),
            Some(Rc::from(Number::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Number::new();
        concept.set_internal_name("A");
        assert_eq!(Number::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Number::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Number::archetype().added_attributes(), vec![]);
        assert_eq!(Number::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Number::new();
        let concept_copy = Number::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Number::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn get_value_none() {
        initialize_kb();
        let concept = Number::new();
        assert_eq!(concept.value(), None);
    }

    #[test]
    fn get_value_some() {
        initialize_kb();
        let mut concept = Number::new();
        concept.set_value(0);
        assert_eq!(concept.value(), Some(Rc::from(0)));
    }
}
