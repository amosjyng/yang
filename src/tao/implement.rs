use crate::tao::Target;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::graph::value_wrappers::{unwrap_value, StrongValue};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, FinalNode};
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::{Tao, YIN_MAX_ID};
use zamm_yin::Wrapper;

/// Represents a command to implement something.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Implement {
    base: FinalNode,
}

/// Contains all information needed to generate a concept. Because it's too difficult to store
/// things in the KB right now, we'll use a custom struct for now.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplementConfig {
    /// ID of the concept being implemented.
    pub id: usize,
    /// Documentation, if any, for the concept being implemented.
    pub doc: Option<String>,
}

impl Default for ImplementConfig {
    fn default() -> Self {
        Self { id: 1, doc: None }
    }
}

impl Implement {
    /// Set another concept as an implementation target.
    pub fn set_target(&mut self, target: Archetype) {
        self.essence_mut()
            .add_outgoing(Target::TYPE_ID, target.essence());
    }

    /// Retrieve implementation target.
    pub fn target(&self) -> Option<Archetype> {
        self.essence()
            .outgoing_nodes(Target::TYPE_ID)
            .into_iter()
            .next()
            .map(Archetype::from)
    }

    /// Set the config for this Implement.
    pub fn set_config(&mut self, config: ImplementConfig) {
        self.essence_mut()
            .set_value(Rc::new(StrongValue::new(config)));
    }

    /// Retrieve the config stored for this Implement.
    pub fn config(&self) -> Option<Rc<ImplementConfig>> {
        unwrap_value::<ImplementConfig>(self.essence().value())
    }
}

impl Debug for Implement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Implement", self, f)
    }
}

impl From<usize> for Implement {
    fn from(id: usize) -> Self {
        Implement {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Implement {
    fn from(f: FinalNode) -> Self {
        Implement { base: f }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Implement::archetype().id(), Implement::TYPE_ID);
        assert_eq!(
            Implement::archetype().internal_name(),
            Some(Rc::new(Implement::TYPE_NAME.to_string()))
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
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Implement::new();
        let concept2 = Implement::new();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Implement::new();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn set_and_retrieve_target() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_target(Owner::archetype().as_archetype());
        assert_eq!(implement.target(), Some(Owner::archetype().as_archetype()));
    }

    #[test]
    fn set_and_retrieve_config() {
        initialize_kb();
        let mut implement = Implement::new();
        implement.set_config(ImplementConfig {
            id: 2,
            doc: Some("Hi".to_owned()),
        });
        assert_eq!(
            implement.config(),
            Some(Rc::new(ImplementConfig {
                id: 2,
                doc: Some("Hi".to_owned()),
            }))
        );
    }
}
