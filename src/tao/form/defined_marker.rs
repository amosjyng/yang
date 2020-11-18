use crate::tao::NewlyDefined;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait,
};
use zamm_yin::tao::form::{Form, FormTrait};

/// Define new concept when reading from a Yin.md
pub trait DefinedMarker: FormTrait + CommonNodeTrait {
    /// Mark a concept as having been newly defined as part of the current build.
    fn mark_newly_defined(&mut self) {
        self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
    }

    /// Whether or not a concept has been newly defined as part of the current build.
    fn is_newly_defined(&self) -> bool {
        self.essence().has_flag(NewlyDefined::TYPE_ID)
    }

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

impl DefinedMarker for Form {}
impl DefinedMarker for Archetype {}
impl DefinedMarker for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::Tao;

    #[test]
    fn test_newly_defined() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_form();
        assert!(!new_attr.is_newly_defined());

        new_attr.mark_newly_defined();
        assert!(new_attr.is_newly_defined());
    }

    #[test]
    fn test_activation_inherited() {
        initialize_kb();
        let mut new_attr = Tao::archetype().individuate_as_archetype();
        let sub_attr = new_attr.individuate_as_form();
        assert!(!sub_attr.is_newly_defined());

        new_attr.mark_newly_defined();
        assert!(sub_attr.is_newly_defined());
    }
}
