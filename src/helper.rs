use crate::tao::archetype::CodegenFlags;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::Attribute;

/// Defines a new concept with the given name.
#[macro_export]
macro_rules! define {
    ($name:ident) => {
        let mut $name = Tao::archetype().individuate_as_archetype();
        $name.set_internal_name(stringify!($name).to_owned());
    };
}

/// Convenience function to convert an `Archetype` to an `AttributeArchetype`.
pub fn aa(archetype: Archetype) -> AttributeArchetype {
    if !(archetype.attribute_logic_activated()
        || archetype.has_parent(Attribute::archetype().as_archetype())
        || archetype
            .parents()
            .iter()
            .any(|a| a.attribute_logic_activated()))
    {
        // currently catches Relation, which is not an attribute but still deserves to have its
        // owner archetype set
        println!("Warning: {:?} is not known to be an attribute.", archetype);
    }
    AttributeArchetype::from(archetype.id())
}
