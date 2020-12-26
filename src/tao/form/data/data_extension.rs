use std::rc::Rc;
use zamm_yin::tao::archetype::{Archetype, DataArchetype};
use zamm_yin::tao::form::FormTrait;

/// Trait to extend Data functionality that has not been auto-generated.
pub trait DataExtension: FormTrait {
    /// Set the name of the Rust primitive that this concept represents.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::set_rust_primitive")]
    fn set_rust_primitive(&mut self, primitive_name: &str) {
        DataArchetype::from(*self.essence()).set_rust_primitive(primitive_name)
    }

    /// Get the name of the Rust primitive that this concept represents.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::rust_primitive")]
    fn rust_primitive(&self) -> Option<Rc<str>> {
        DataArchetype::from(*self.essence()).rust_primitive()
    }

    /// Set the unboxed version of this primitive.
    #[deprecated(
        since = "0.1.9",
        note = "Please use DataArchetype::set_unboxed_representation"
    )]
    fn set_unboxed_representation(&mut self, primitive_name: &str) {
        DataArchetype::from(*self.essence()).set_unboxed_representation(primitive_name)
    }

    /// Get the unboxed version of this primitive.
    #[deprecated(
        since = "0.1.9",
        note = "Please use DataArchetype::unboxed_representation"
    )]
    fn unboxed_representation(&self) -> Option<Rc<str>> {
        DataArchetype::from(*self.essence()).unboxed_representation()
    }

    /// Set the Rust code representation for the default value of this concept.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::set_default_value")]
    fn set_default_value(&mut self, default_value_as_code: &str) {
        DataArchetype::from(*self.essence()).set_default_value(default_value_as_code)
    }

    /// Get the Rust code representation for the default value of this concept.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::default_value")]
    fn default_value(&self) -> Option<Rc<str>> {
        DataArchetype::from(*self.essence()).default_value()
    }

    /// Set the Rust code representation for the dummy test value of this concept.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::set_dummy_value")]
    fn set_dummy_value(&mut self, dummy_value_as_code: &str) {
        DataArchetype::from(*self.essence()).set_dummy_value(dummy_value_as_code)
    }

    /// Get the Rust code representation for the dummy test value of this concept.
    #[deprecated(since = "0.1.9", note = "Please use DataArchetype::dummy_value")]
    fn dummy_value(&self) -> Option<Rc<str>> {
        DataArchetype::from(*self.essence()).dummy_value()
    }
}

// technically, this should be limited to Data-specific archetypes, but there's currently no way to
// do that.
impl DataExtension for Archetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
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
