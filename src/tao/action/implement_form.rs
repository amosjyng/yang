use crate::tao::action::Action;
use crate::tao::form::rust_item::data::{Number, StrConcept};
use crate::tao::relation::attribute::{ConceptId, Documentation, DualPurposeDocumentation, Target};
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// The act of implementing something. When created, this effectively serves as
/// a call to action for Yang.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Implement {
    base: FinalNode,
}

impl Debug for Implement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Implement", self, f)
    }
}

impl From<usize> for Implement {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Implement {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Implement {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Implement {
    type ArchetypeForm = Archetype;
    type Form = Implement;

    const TYPE_ID: usize = YIN_MAX_ID + 14;
    const TYPE_NAME: &'static str = "implement";
    const PARENT_TYPE_ID: usize = Action::TYPE_ID;
}

impl Deref for Implement {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Implement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Implement {}

impl From<Implement> for Tao {
    fn from(this: Implement) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Implement> for Action {
    fn from(this: Implement) -> Action {
        Action::from(this.base)
    }
}

impl Implement {
    /// Get target concept for this implementation.
    pub fn target(&self) -> Option<Form> {
        self.deref()
            .outgoing_nodes(Target::TYPE_ID)
            .last()
            .map(|f| Form::from(f.id()))
    }

    /// Set target concept for this implementation.
    pub fn set_target(&mut self, target: &Form) {
        self.deref_mut()
            .add_outgoing(Target::TYPE_ID, target.deref());
    }

    /// Get the concept's ID during code generation time, as opposed to the
    /// concept's currently assigned runtime ID.
    #[allow(clippy::rc_buffer)]
    pub fn concept_id(&self) -> Option<Rc<usize>> {
        self.deref()
            .outgoing_nodes(ConceptId::TYPE_ID)
            .last()
            .map(|f| Number::from(f.id()).value().unwrap())
    }

    /// Set the concept's ID during code generation time, as opposed to the
    /// concept's currently assigned runtime ID.
    pub fn set_concept_id(&mut self, concept_id: usize) {
        let mut value_concept = Number::new();
        value_concept.set_value(concept_id);
        self.deref_mut()
            .add_outgoing(ConceptId::TYPE_ID, value_concept.deref());
    }

    /// Get the documentation string associated with this particular Rust
    /// implementation.
    #[allow(clippy::rc_buffer)]
    pub fn documentation(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(Documentation::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the documentation string associated with this particular Rust
    /// implementation.
    pub fn set_documentation(&mut self, documentation: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(documentation);
        self.deref_mut()
            .add_outgoing(Documentation::TYPE_ID, value_concept.deref());
    }

    /// Get the dual-purpose documentation substring to be used for the
    /// implementation of this property as getters and setters in a different
    /// concept's class.
    #[allow(clippy::rc_buffer)]
    pub fn dual_purpose_documentation(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(DualPurposeDocumentation::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the dual-purpose documentation substring to be used for the
    /// implementation of this property as getters and setters in a different
    /// concept's class.
    pub fn set_dual_purpose_documentation(&mut self, dual_purpose_documentation: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(dual_purpose_documentation);
        self.deref_mut()
            .add_outgoing(DualPurposeDocumentation::TYPE_ID, value_concept.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::action::Implement;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{
        ConceptId, Documentation, DualPurposeDocumentation, Target,
    };
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Implement::archetype().id(), Implement::TYPE_ID);
        assert_eq!(
            Implement::archetype().internal_name(),
            Some(Rc::from(Implement::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Implement::new();
        concept.set_internal_name("A");
        assert_eq!(Implement::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Implement::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Implement::archetype().added_attributes(),
            vec![
                Target::archetype(),
                ConceptId::archetype(),
                Documentation::archetype(),
                DualPurposeDocumentation::archetype()
            ]
        );
        assert_eq!(
            Implement::archetype().attributes(),
            vec![
                Target::archetype(),
                ConceptId::archetype(),
                Documentation::archetype(),
                DualPurposeDocumentation::archetype()
            ]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Implement::new();
        let concept_copy = Implement::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Implement::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_target() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.target(), None);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_target(&value);
        assert_eq!(new_instance.target(), Some(value));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_target_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.target(), None);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_target(&value);
        assert_eq!(new_instance.target(), Some(value));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_target_multiple_times() {
        initialize_kb();
        let mut new_instance = Implement::new();
        let default = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_target(&default);
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.target(), Some(default));

        let new_value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_target(&new_value);
        assert_eq!(new_instance.target(), Some(new_value));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_concept_id() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.concept_id(), None);

        let value = 0;
        #[allow(clippy::clone_on_copy)]
        new_instance.set_concept_id(value.clone());
        assert_eq!(new_instance.concept_id(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_concept_id_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.concept_id(), None);

        let value = 0;
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_concept_id(value.clone());
        assert_eq!(new_instance.concept_id(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_concept_id_multiple_times() {
        initialize_kb();
        let mut new_instance = Implement::new();
        let default = 0;
        #[allow(clippy::clone_on_copy)]
        new_instance.set_concept_id(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.concept_id(), Some(Rc::from(default)));

        let new_value = 17;
        #[allow(clippy::clone_on_copy)]
        new_instance.set_concept_id(new_value.clone());
        assert_eq!(new_instance.concept_id(), Some(Rc::from(new_value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_documentation() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.documentation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_documentation(value.clone());
        assert_eq!(new_instance.documentation(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_documentation_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.documentation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_documentation(value.clone());
        assert_eq!(new_instance.documentation(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_documentation_multiple_times() {
        initialize_kb();
        let mut new_instance = Implement::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_documentation(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.documentation(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_documentation(new_value.clone());
        assert_eq!(new_instance.documentation(), Some(Rc::from(new_value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_dual_purpose_documentation() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.dual_purpose_documentation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dual_purpose_documentation(value.clone());
        assert_eq!(
            new_instance.dual_purpose_documentation(),
            Some(Rc::from(value))
        );
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_dual_purpose_documentation_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.dual_purpose_documentation(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_dual_purpose_documentation(value.clone());
        assert_eq!(
            new_instance.dual_purpose_documentation(),
            Some(Rc::from(value))
        );
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_dual_purpose_documentation_multiple_times() {
        initialize_kb();
        let mut new_instance = Implement::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dual_purpose_documentation(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(
            new_instance.dual_purpose_documentation(),
            Some(Rc::from(default))
        );

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_dual_purpose_documentation(new_value.clone());
        assert_eq!(
            new_instance.dual_purpose_documentation(),
            Some(Rc::from(new_value))
        );
    }
}
