use crate::tao::perspective::{BuildInfo, BuildInfoExtension};
use crate::tao::relation::attribute::{ConceptId, Documentation, Target};
use crate::tao::Implement;
use std::rc::Rc;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::data::{Number, StringConcept};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::Wrapper;

/// Extension functions for Implement concept.
pub trait ImplementExtension: FormTrait + CommonNodeTrait {
    /// Set another concept as an implementation target.
    fn set_target(&mut self, target: Form) {
        self.essence_mut()
            .add_outgoing(Target::TYPE_ID, target.essence());
    }

    /// Retrieve implementation target.
    fn target(&self) -> Option<Form> {
        self.essence()
            .outgoing_nodes(Target::TYPE_ID)
            .into_iter()
            .next()
            .map(Form::from)
    }

    /// Set concept ID during code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    fn set_implementation_id(&mut self, id: usize) {
        let mut n = Number::new();
        n.set_value(id);
        self.essence_mut()
            .add_outgoing(ConceptId::TYPE_ID, n.essence());
    }

    /// Get concept ID set for code generation time, as opposed to the concept's currently assigned
    /// runtime ID.
    fn implementation_id(&self) -> Option<Rc<usize>> {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(ConceptId::TYPE_ID)
            .first()
            .map(|v| Number::from(v.id()).value())
            .flatten()
    }

    /// Set the documentation string for this implementation.
    fn document(&mut self, document: &str) {
        let mut s = StringConcept::new();
        s.set_value(document.to_owned());
        self.essence_mut()
            .add_outgoing(Documentation::TYPE_ID, s.essence());
    }

    /// Get the documentation string for this implementation.
    fn documentation(&self) -> Option<Rc<str>> {
        self.essence()
            .inheritance_wrapper()
            .base_wrapper()
            .outgoing_nodes(Documentation::TYPE_ID)
            .first()
            .map(|v| {
                StringConcept::from(v.id())
                    .value()
                    .map(|rc| Rc::from(rc.as_str()))
            })
            .flatten()
    }

    /// Set the dual-purpose documentation string for this implementation.
    fn dual_document(&mut self, document: &str) {
        BuildInfo::from(self.target().unwrap().id()).dual_document(document);
    }

    /// Get the dual-purpose documentation string for this implementation.
    fn dual_documentation(&self) -> Option<Rc<str>> {
        BuildInfo::from(self.target().unwrap().id()).dual_documentation()
    }
}

impl ImplementExtension for Implement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn set_and_retrieve_target() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(Owner::archetype().as_form());
        assert_eq!(implement.target(), Some(Owner::archetype().as_form()));
    }

    #[test]
    fn set_and_retrieve_implementation_id() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_implementation_id(17);
        assert_eq!(implement.implementation_id(), Some(Rc::new(17)));
    }

    #[test]
    fn set_and_retrieve_documentation() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.document("Some new thing.");
        assert_eq!(implement.documentation(), Some(Rc::from("Some new thing.")));
    }

    #[test]
    fn set_and_retrieve_dual_documentation() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(implement.as_form());
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
        implement.set_target(implement.as_form());
        implement.document("Some new thing.");
        implement.dual_document("duels as being outdated.");
        assert_eq!(implement.documentation(), Some(Rc::from("Some new thing.")));
        assert_eq!(
            implement.dual_documentation(),
            Some(Rc::from("duels as being outdated."))
        );
    }
}
