//! Command to implement something.

use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use zamm_yin::concepts::{Archetype, ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::wrappers::{debug_wrapper, CommonNodeTrait, FinalWrapper};

/// Represents a command to implement something.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Implement {
    base: Tao,
}

impl Debug for Implement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Implement", Box::new(self), f)
    }
}

impl From<usize> for Implement {
    fn from(id: usize) -> Self {
        Implement {
            base: Tao::from(id),
        }
    }
}

impl From<Tao> for Implement {
    fn from(t: Tao) -> Self {
        Implement { base: t }
    }
}

impl CommonNodeTrait for Implement {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl ArchetypeTrait<Implement> for Implement {
    const TYPE_ID: usize = YIN_MAX_ID + 1;
    const TYPE_NAME: &'static str = "Implement";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Self::individuate_with_parent(Self::TYPE_ID)
    }

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Tao::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Implement {
    fn essence(&self) -> &FinalWrapper {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalWrapper {
        self.base.essence_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

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
        let concept = Implement::individuate();
        let concept_copy = Implement::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Implement::individuate();
        let concept2 = Implement::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Implement::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
