use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use crate::tao::form::Collection;

/// Trait to add and retrieve members for anything capable of being the owner of a HasMember
/// attribute.
pub trait SupportsMembership: FormTrait + CommonNodeTrait {
    /// Add a member to this collection.
    fn add_member(&mut self, member: &Form) {
        Collection::from(self.id()).add_member(member)
    }

    /// Get the members of this collection.
    fn members(&self) -> Vec<Form> {
        Collection::from(self.id()).members()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeTrait;

    // only for test purposes
    impl SupportsMembership for Form {}

    #[test]
    fn add_and_retrieve_members() {
        initialize_kb();
        let member = Form::new();
        let mut collection = Form::new();
        SupportsMembership::add_member(&mut collection, &member);
        assert_eq!(SupportsMembership::members(&collection), vec![member]);
    }
}
