//! Types of forms, as opposed to the forms themselves.

pub mod rust_item_archetype;

mod create_implementation;

pub use create_implementation::CreateImplementation;
pub use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait,
};
