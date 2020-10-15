use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID};
use zamm_yin::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};

/// Structures that can ultimately be compiled down to bits.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Data {
    base: Tao,
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Data", self, f)
    }
}

impl From<usize> for Data {
    fn from(id: usize) -> Self {
        Self {
            base: Tao::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for Data {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Tao::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for Data {
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

impl<'a> ArchetypeTrait<'a, Data> for Data {
    const TYPE_ID: usize = YIN_MAX_ID + 4;
    const TYPE_NAME: &'static str = "data";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Tao::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Data {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
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
        assert_eq!(Data::archetype().id(), Data::TYPE_ID);
        assert_eq!(
            Data::archetype().internal_name(),
            Some(Rc::new(Data::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Data::individuate();
        let concept_copy = Data::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Data::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Data::try_from("A"), Ok(concept));
        assert!(Data::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Data::individuate();
        let concept2 = Data::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Data::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
