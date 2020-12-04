use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::Implement;
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::form::FormTrait;

/// Extension functions for Implement concept.
pub trait ImplementExtension: FormTrait + CommonNodeTrait {
    /// Set the dual-purpose documentation string for this implementation.
    fn dual_document(&mut self, document: &str) {
        // already defined in Implement, it's just that Implement isn't a trait so we can't rely on
        // it here
        let implement = Implement::from(self.id());
        BuildInfo::from(implement.target().unwrap().id()).dual_document(document);
    }

    /// Get the dual-purpose documentation string for this implementation.
    fn dual_documentation(&self) -> Option<Rc<str>> {
        // already defined in Implement, it's just that Implement isn't a trait so we can't rely on
        // it here
        let implement = Implement::from(self.id());
        BuildInfo::from(implement.target().unwrap().id()).dual_documentation()
    }
}

impl ImplementExtension for Implement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn set_and_retrieve_target() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(&Owner::archetype().as_form());
        assert_eq!(implement.target(), Some(Owner::archetype().as_form()));
    }

    #[test]
    fn set_and_retrieve_dual_documentation() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(&implement.as_form());
        implement.dual_document("duels as being outdated.");
        assert_eq!(
            implement.dual_documentation(),
            Some(Rc::from("duels as being outdated."))
        );
    }

    #[test]
    fn test_documentation_and_dual_documentation_independent() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(&implement.as_form());
        implement.set_documentation("Some new thing.".to_owned());
        implement.dual_document("duels as being outdated.");
        assert_eq!(
            implement.documentation(),
            Some(Rc::from("Some new thing.".to_owned()))
        );
        assert_eq!(
            implement.dual_documentation(),
            Some(Rc::from("duels as being outdated."))
        );
    }
}
