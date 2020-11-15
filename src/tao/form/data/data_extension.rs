use crate::tao::form::data::StringConcept;
use crate::tao::relation::attribute::RustPrimitive;
use zamm_yin::graph::value_wrappers::unwrap_strong;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Trait to extend Data functionality that has not been auto-generated.
pub trait DataExtension: FormTrait {
    /// Set the name of the Rust primitive that this concept represents.
    fn set_rust_primitive(&mut self, primitive_name: &str) {
        let mut name_str = StringConcept::individuate();
        name_str.set_value(primitive_name.to_owned());
        self.essence_mut()
            .add_outgoing(RustPrimitive::TYPE_ID, name_str.essence());
    }

    /// Get the name of the Rust primitive that this concept represents.
    fn rust_primitive(&self) -> Option<String> {
        // todo: change once StringConcept retrieves the value directly
        self.essence()
            .outgoing_nodes(RustPrimitive::TYPE_ID)
            .first()
            .map(|p| {
                let kb_value = StringConcept::from(*p).value();
                unwrap_strong::<String>(&kb_value).cloned()
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
    use crate::tao::form::data::Data;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn set_and_get_rust_primitive() {
        initialize_kb();

        let mut new_data = Data::archetype().individuate_as_archetype();
        new_data.set_rust_primitive("u64");
        assert_eq!(new_data.rust_primitive(), Some("u64".to_owned()));
    }
}
