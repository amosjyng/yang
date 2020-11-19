use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, AttributeArchetype, AttributeArchetypeFormTrait};
use zamm_yin::tao::form::{Form, FormTrait};

/// Define new concept when reading from a Yin.md
#[deprecated(
    since = "0.1.2",
    note = "Please use AttributeArchetypeFormTrait instead."
)]
pub trait DefinedMarker: FormTrait + CommonNodeTrait {
    /// Dummy function to handle API change in the move to AttributeArchetypeFormTrait. This is
    /// here because BackwardsCompatibility was not implemented at the time.
    #[deprecated(
        since = "0.1.1",
        note = "Please import AttributeArchetypeFormTrait instead."
    )]
    fn set_owner_archetype(&mut self, owner_archetype: Archetype) {
        AttributeArchetypeFormTrait::set_owner_archetype(
            &mut AttributeArchetype::from(self.id()),
            owner_archetype,
        )
    }

    /// Dummy function to handle API change in the move to AttributeArchetypeFormTrait. This is
    /// here because BackwardsCompatibility was not implemented at the time.
    #[deprecated(
        since = "0.1.1",
        note = "Please import AttributeArchetypeFormTrait instead."
    )]
    fn owner_archetype(&mut self) -> Archetype {
        AttributeArchetypeFormTrait::owner_archetype(&AttributeArchetype::from(self.id()))
    }

    /// Dummy function to handle API change in the move to AttributeArchetypeFormTrait. This is
    /// here because BackwardsCompatibility was not implemented at the time.
    #[deprecated(
        since = "0.1.1",
        note = "Please import AttributeArchetypeFormTrait instead."
    )]
    fn set_value_archetype(&mut self, value_archetype: Archetype) {
        AttributeArchetypeFormTrait::set_value_archetype(
            &mut AttributeArchetype::from(self.id()),
            value_archetype,
        )
    }

    /// Dummy function to handle API change in the move to AttributeArchetypeFormTrait. This is
    /// here because BackwardsCompatibility was not implemented at the time.
    #[deprecated(
        since = "0.1.1",
        note = "Please import AttributeArchetypeFormTrait instead."
    )]
    fn value_archetype(&mut self) -> Archetype {
        AttributeArchetypeFormTrait::value_archetype(&AttributeArchetype::from(self.id()))
    }
}

#[allow(deprecated)]
impl DefinedMarker for Form {}
#[allow(deprecated)]
impl DefinedMarker for Archetype {}
#[allow(deprecated)]
impl DefinedMarker for AttributeArchetype {}
