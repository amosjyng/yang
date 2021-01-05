use crate::tao::relation::attribute::{ConceptId, Documentation, Target};
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::data::{Number, StringConcept};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};
use zamm_yin::Wrapper;

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

impl Wrapper for Implement {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Implement {
    type ArchetypeForm = Archetype;
    type Form = Implement;

    const TYPE_ID: usize = YIN_MAX_ID + 1;
    const TYPE_NAME: &'static str = "implement";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Implement {}

impl From<Implement> for Tao {
    fn from(this: Implement) -> Tao {
        Tao::from(this.base)
    }
}

impl Implement {
    /// Get target concept for this implementation.
    pub fn target(&self) -> Option<Form> {
        self.essence()
            .outgoing_nodes(Target::TYPE_ID)
            .first()
            .map(|f| Form::from(f.id()))
    }

    /// Set target concept for this implementation.
    pub fn set_target(&mut self, target: &Form) {
        self.essence_mut()
            .add_outgoing(Target::TYPE_ID, target.essence());
    }

    /// Get the concept's ID during code generation time, as opposed to the
    /// concept's currently assigned runtime ID.
    #[allow(clippy::rc_buffer)]
    pub fn concept_id(&self) -> Option<Rc<usize>> {
        self.essence()
            .outgoing_nodes(ConceptId::TYPE_ID)
            .first()
            .map(|f| Number::from(f.id()).value().unwrap())
    }

    /// Set the concept's ID during code generation time, as opposed to the
    /// concept's currently assigned runtime ID.
    pub fn set_concept_id(&mut self, concept_id: usize) {
        let mut value_concept = Number::new();
        value_concept.set_value(concept_id);
        self.essence_mut()
            .add_outgoing(ConceptId::TYPE_ID, value_concept.essence());
    }

    /// Get the documentation string associated with this particular Rust
    /// implementation.
    #[allow(clippy::rc_buffer)]
    pub fn documentation(&self) -> Option<Rc<String>> {
        self.essence()
            .outgoing_nodes(Documentation::TYPE_ID)
            .first()
            .map(|f| StringConcept::from(f.id()).value().unwrap())
    }

    /// Set the documentation string associated with this particular Rust
    /// implementation.
    pub fn set_documentation(&mut self, documentation: String) {
        let mut value_concept = StringConcept::new();
        value_concept.set_value(documentation);
        self.essence_mut()
            .add_outgoing(Documentation::TYPE_ID, value_concept.essence());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::relation::attribute::{ConceptId, Documentation, Target};
    use crate::tao::{initialize_kb, Implement};
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Implement::archetype().id(), Implement::TYPE_ID);
        assert_eq!(
            Implement::archetype().internal_name_str(),
            Some(Rc::from(Implement::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Implement::new();
        concept.set_internal_name_str("A");
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
                Documentation::archetype()
            ]
        );
        assert_eq!(
            Implement::archetype().attributes(),
            vec![
                Target::archetype(),
                ConceptId::archetype(),
                Documentation::archetype()
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
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
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
    fn test_set_and_get_concept_id() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.concept_id(), None);

        let value = 0;
        #[allow(clippy::clone_on_copy)]
        new_instance.set_concept_id(value.clone());
        assert_eq!(new_instance.concept_id(), Some(Rc::new(value)));
    }

    #[test]
    fn test_concept_id_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.concept_id(), None);

        let value = 0;
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_concept_id(value.clone());
        assert_eq!(new_instance.concept_id(), Some(Rc::new(value)));
    }

    #[test]
    fn test_set_and_get_documentation() {
        initialize_kb();
        let mut new_instance = Implement::new();
        assert_eq!(new_instance.documentation(), None);

        let value = String::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.set_documentation(value.clone());
        assert_eq!(new_instance.documentation(), Some(Rc::new(value)));
    }

    #[test]
    fn test_documentation_inheritance() {
        initialize_kb();
        let new_type = Implement::archetype().individuate_as_archetype();
        let new_instance = Implement::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.documentation(), None);

        let value = String::new();
        #[allow(clippy::clone_on_copy)]
        Implement::from(new_type.id()).set_documentation(value.clone());
        assert_eq!(new_instance.documentation(), Some(Rc::new(value)));
    }
}
