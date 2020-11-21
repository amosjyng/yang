use crate::tao::relation::attribute::HasMember;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Trait to add and retrieve members for anything capable of being the owner of a HasMember
/// attribute.
pub trait SupportsMembership: FormTrait + CommonNodeTrait {
    /// Add a member to this collection.
    fn add_member(&mut self, member: &Form) {
        self.essence_mut()
            .add_outgoing(HasMember::TYPE_ID, member.essence());
    }

    /// Get the members of this collection.
    fn members(&self) -> Vec<Form> {
        // no need to worry about inheritance because HasMember is not Inherits
        // todo: only worry about inheritance for archetypal lens
        self.essence()
            .outgoing_nodes(HasMember::TYPE_ID)
            .iter()
            .map(|f| Form::from(*f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

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
