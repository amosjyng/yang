use crate::tao::archetype::rust_item_archetype::RustItemArchetype;
use crate::tao::form::rust_item::data::{Data, StrConcept};
use crate::tao::relation::attribute::{
    DefaultValue, DummyValue, RustPrimitive, UnboxedRepresentation,
};
use crate::tao::relation::flag::Unsized;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Meta-object for Data meta-attributes.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataArchetype {
    base: FinalNode,
}

impl Debug for DataArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DataArchetype", self, f)
    }
}

impl From<usize> for DataArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DataArchetype {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DataArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for DataArchetype {
    type ArchetypeForm = Archetype;
    type Form = DataArchetype;

    const TYPE_ID: usize = YIN_MAX_ID + 4;
    const TYPE_NAME: &'static str = "data-archetype";
    const PARENT_TYPE_ID: usize = RustItemArchetype::TYPE_ID;
}

impl Deref for DataArchetype {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DataArchetype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for DataArchetype {}

impl From<DataArchetype> for Tao {
    fn from(this: DataArchetype) -> Tao {
        Tao::from(this.base)
    }
}

impl From<DataArchetype> for Archetype {
    fn from(this: DataArchetype) -> Archetype {
        Archetype::from(this.base)
    }
}

impl From<DataArchetype> for RustItemArchetype {
    fn from(this: DataArchetype) -> RustItemArchetype {
        RustItemArchetype::from(this.base)
    }
}

impl ArchetypeFormTrait for DataArchetype {
    type SubjectForm = Data;
}

impl DataArchetype {
    /// Whether this is marked as having a known size at compile-time.
    pub fn is_unsized(&self) -> bool {
        self.deref().has_flag(Unsized::TYPE_ID)
    }

    /// Mark this as having a known size at compile-time.
    pub fn mark_unsized(&mut self) {
        self.deref_mut().add_flag(Unsized::TYPE_ID);
    }

    /// Get the Rust code representation for the default value of this concept.
    #[allow(clippy::rc_buffer)]
    pub fn default_value(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(DefaultValue::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the Rust code representation for the default value of this concept.
    pub fn set_default_value(&mut self, default_value: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(default_value);
        self.deref_mut()
            .add_outgoing(DefaultValue::TYPE_ID, value_concept.deref());
    }

    /// Get the name of the Rust primitive that this concept represents.
    #[allow(clippy::rc_buffer)]
    pub fn rust_primitive(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(RustPrimitive::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the name of the Rust primitive that this concept represents.
    pub fn set_rust_primitive(&mut self, rust_primitive: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(rust_primitive);
        self.deref_mut()
            .add_outgoing(RustPrimitive::TYPE_ID, value_concept.deref());
    }

    /// Get the unboxed version of this primitive.
    #[allow(clippy::rc_buffer)]
    pub fn unboxed_representation(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(UnboxedRepresentation::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the unboxed version of this primitive.
    pub fn set_unboxed_representation(&mut self, unboxed_representation: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(unboxed_representation);
        self.deref_mut()
            .add_outgoing(UnboxedRepresentation::TYPE_ID, value_concept.deref());
    }

    /// Get the the Rust code representation for the dummy test value of this
    /// concept.
    #[allow(clippy::rc_buffer)]
    pub fn dummy_value(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(DummyValue::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the the Rust code representation for the dummy test value of this
    /// concept.
    pub fn set_dummy_value(&mut self, dummy_value: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(dummy_value);
        self.deref_mut()
            .add_outgoing(DummyValue::TYPE_ID, value_concept.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::rust_item_archetype::DataArchetype;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{
        DefaultValue, DummyValue, RustPrimitive, UnboxedRepresentation,
    };
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(DataArchetype::archetype().id(), DataArchetype::TYPE_ID);
        assert_eq!(
            DataArchetype::archetype().internal_name(),
            Some(Rc::from(DataArchetype::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DataArchetype::new();
        concept.set_internal_name("A");
        assert_eq!(
            DataArchetype::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(DataArchetype::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            DataArchetype::archetype().added_attributes(),
            vec![
                DefaultValue::archetype(),
                RustPrimitive::archetype(),
                UnboxedRepresentation::archetype(),
                DummyValue::archetype()
            ]
        );
        assert_eq!(
            DataArchetype::archetype().attributes(),
            vec![
                DefaultValue::archetype(),
                RustPrimitive::archetype(),
                UnboxedRepresentation::archetype(),
                DummyValue::archetype()
            ]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DataArchetype::new();
        let concept_copy = DataArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DataArchetype::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn test_mark_and_check_unsized() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        assert!(!new_instance.is_unsized());

        new_instance.mark_unsized();
        assert!(new_instance.is_unsized());
    }

    #[test]
    fn test_unsized_inheritance() {
        initialize_kb();
        let new_type = DataArchetype::archetype().individuate_as_archetype();
        let new_instance = DataArchetype::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_unsized());

        DataArchetype::from(new_type.id()).mark_unsized();
        assert!(new_instance.is_unsized());
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_default_value() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        assert_eq!(new_instance.default_value(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_default_value(value.clone());
        assert_eq!(new_instance.default_value(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_default_value_inheritance() {
        initialize_kb();
        let new_type = DataArchetype::archetype().individuate_as_archetype();
        let new_instance = DataArchetype::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.default_value(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        DataArchetype::from(new_type.id()).set_default_value(value.clone());
        assert_eq!(new_instance.default_value(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_default_value_multiple_times() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_default_value(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.default_value(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_default_value(new_value.clone());
        assert_eq!(new_instance.default_value(), Some(Rc::from(new_value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_rust_primitive() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        assert_eq!(new_instance.rust_primitive(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_rust_primitive(value.clone());
        assert_eq!(new_instance.rust_primitive(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_rust_primitive_inheritance() {
        initialize_kb();
        let new_type = DataArchetype::archetype().individuate_as_archetype();
        let new_instance = DataArchetype::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.rust_primitive(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        DataArchetype::from(new_type.id()).set_rust_primitive(value.clone());
        assert_eq!(new_instance.rust_primitive(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_rust_primitive_multiple_times() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_rust_primitive(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.rust_primitive(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_rust_primitive(new_value.clone());
        assert_eq!(new_instance.rust_primitive(), Some(Rc::from(new_value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_unboxed_representation() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        assert_eq!(new_instance.unboxed_representation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_unboxed_representation(value.clone());
        assert_eq!(new_instance.unboxed_representation(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_unboxed_representation_inheritance() {
        initialize_kb();
        let new_type = DataArchetype::archetype().individuate_as_archetype();
        let new_instance = DataArchetype::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.unboxed_representation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        DataArchetype::from(new_type.id()).set_unboxed_representation(value.clone());
        assert_eq!(new_instance.unboxed_representation(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_unboxed_representation_multiple_times() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_unboxed_representation(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(
            new_instance.unboxed_representation(),
            Some(Rc::from(default))
        );

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_unboxed_representation(new_value.clone());
        assert_eq!(
            new_instance.unboxed_representation(),
            Some(Rc::from(new_value))
        );
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_dummy_value() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        assert_eq!(new_instance.dummy_value(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dummy_value(value.clone());
        assert_eq!(new_instance.dummy_value(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_dummy_value_inheritance() {
        initialize_kb();
        let new_type = DataArchetype::archetype().individuate_as_archetype();
        let new_instance = DataArchetype::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.dummy_value(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        DataArchetype::from(new_type.id()).set_dummy_value(value.clone());
        assert_eq!(new_instance.dummy_value(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_dummy_value_multiple_times() {
        initialize_kb();
        let mut new_instance = DataArchetype::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dummy_value(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.dummy_value(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dummy_value(new_value.clone());
        assert_eq!(new_instance.dummy_value(), Some(Rc::from(new_value)));
    }
}
