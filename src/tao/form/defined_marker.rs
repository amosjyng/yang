use crate::tao::NewlyDefined;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::{Form, FormTrait};

/// Define new concept when reading from a Yin.md
pub trait DefinedMarker: FormTrait {
    /// Mark a concept as having been newly defined as part of the current build.
    fn mark_newly_defined(&mut self) {
        self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
    }

    /// Whether or not a concept has been newly defined as part of the current build.
    fn is_newly_defined(&self) -> bool {
        self.essence().has_flag(NewlyDefined::TYPE_ID)
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
