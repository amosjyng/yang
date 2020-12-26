use crate::tao::relation::attribute::{DummyValue, RustPrimitive, UnboxedRepresentation};
use std::rc::Rc;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::data::StringConcept;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::DefaultValue;
use zamm_yin::Wrapper;

/// Trait to extend Data functionality that has not been auto-generated.
pub trait DataExtension: FormTrait {
    /// Set the name of the Rust primitive that this concept represents.
    fn set_rust_primitive(&mut self, primitive_name: &str) {
        let mut name_str = StringConcept::new();
        name_str.set_value(primitive_name.to_owned());
        self.essence_mut()
            .add_outgoing(RustPrimitive::TYPE_ID, name_str.essence());
    }

    /// Get the name of the Rust primitive that this concept represents.
    fn rust_primitive(&self) -> Option<Rc<str>> {
        self.essence()
            .outgoing_nodes(RustPrimitive::TYPE_ID)
            .first()
            .map(|p| {
                StringConcept::from(*p)
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }

    /// Set the unboxed version of this primitive.
    fn set_unboxed_representation(&mut self, primitive_name: &str) {
        let mut name_str = StringConcept::new();
        name_str.set_value(primitive_name.to_owned());
        self.essence_mut()
            .add_outgoing(UnboxedRepresentation::TYPE_ID, name_str.essence());
    }

    /// Get the unboxed version of this primitive.
    fn unboxed_representation(&self) -> Option<Rc<str>> {
        self.essence()
            .outgoing_nodes(UnboxedRepresentation::TYPE_ID)
            .first()
            .map(|p| {
                StringConcept::from(*p)
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }

    /// Set the Rust code representation for the default value of this concept.
    fn set_default_value(&mut self, default_value_as_code: &str) {
        let mut code_str = StringConcept::new();
        code_str.set_value(default_value_as_code.to_owned());
        self.essence_mut()
            .add_outgoing(DefaultValue::TYPE_ID, code_str.essence());
    }

    /// Get the Rust code representation for the default value of this concept.
    fn default_value(&self) -> Option<Rc<str>> {
        self.essence()
            .outgoing_nodes(DefaultValue::TYPE_ID)
            .first()
            .map(|p| {
                StringConcept::from(*p)
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }

    /// Set the Rust code representation for the dummy test value of this concept.
    fn set_dummy_value(&mut self, dummy_value_as_code: &str) {
        let mut code_str = StringConcept::new();
        code_str.set_value(dummy_value_as_code.to_owned());
        self.essence_mut()
            .add_outgoing(DummyValue::TYPE_ID, code_str.essence());
    }

    /// Get the Rust code representation for the dummy test value of this concept.
    fn dummy_value(&self) -> Option<Rc<str>> {
        self.essence()
            .outgoing_nodes(DummyValue::TYPE_ID)
            .first()
            .map(|p| {
                StringConcept::from(*p)
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }
}

// technically, this should be limited to Data-specific archetypes, but there's currently no way to
// do that.
impl DataExtension for Archetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::data::Data;

    #[test]
    fn set_and_get_rust_primitive() {
        initialize_kb();

        let mut new_data = Data::archetype().individuate_as_archetype();
        new_data.set_rust_primitive("u64");
        assert_eq!(new_data.rust_primitive(), Some(Rc::from("u64")));
    }

    #[test]
    fn set_and_get_unboxed_representation() {
        initialize_kb();

        let mut new_data = Data::archetype().individuate_as_archetype();
        new_data.set_unboxed_representation("&str");
        assert_eq!(new_data.unboxed_representation(), Some(Rc::from("&str")));
    }

    #[test]
    fn set_and_get_default_value() {
        initialize_kb();

        let mut new_data = Data::archetype().individuate_as_archetype();
        new_data.set_default_value("0");
        assert_eq!(new_data.default_value(), Some(Rc::from("0")));
    }

    #[test]
    fn set_and_get_dummy_value() {
        initialize_kb();
        let mut new_data = Data::archetype().individuate_as_archetype();
        assert_eq!(new_data.dummy_value(), None);

        new_data.set_dummy_value("0");
        assert_eq!(new_data.dummy_value(), Some(Rc::from("0")));
    }
}
