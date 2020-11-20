use crate::tao::form::Module;
use crate::tao::relation::attribute::MostProminentMember;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Trait to extend Module functionality that has not been auto-generated.
pub trait ModuleExtension: FormTrait {
    /// Set most prominent member of the module.
    fn set_most_prominent_member(&mut self, member: &Form) {
        self.essence_mut()
            .add_outgoing(MostProminentMember::TYPE_ID, member.essence());
    }

    /// Retrieve most prominent member of the module.
    fn most_prominent_member(&self) -> Option<Form> {
        self.essence()
            .outgoing_nodes(MostProminentMember::TYPE_ID)
            .first()
            .map(|f| Form::from(*f))
    }
}

impl ModuleExtension for Module {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::Tao;

    #[test]
    fn set_and_retrieve_most_prominent_member() {
        initialize_kb();
        let my_type = Tao::archetype().individuate_as_archetype().as_form();
        let mut module = Module::new();
        module.set_most_prominent_member(&my_type);
        assert_eq!(module.most_prominent_member(), Some(my_type));
    }
}
