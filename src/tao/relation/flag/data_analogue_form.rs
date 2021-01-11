use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::relation::Relation;
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Marks an archetype and all its descendants as requiring data-specific logic
/// during generation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataAnalogue {
    base: FinalNode,
}

impl Debug for DataAnalogue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DataAnalogue", self, f)
    }
}

impl From<usize> for DataAnalogue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DataAnalogue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DataAnalogue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for DataAnalogue {
    type ArchetypeForm = Archetype;
    type Form = DataAnalogue;

    const TYPE_ID: usize = YIN_MAX_ID + 27;
    const TYPE_NAME: &'static str = "data-analogue";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for DataAnalogue {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DataAnalogue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for DataAnalogue {}

impl From<DataAnalogue> for Tao {
    fn from(this: DataAnalogue) -> Tao {
        Tao::from(this.base)
    }
}

impl From<DataAnalogue> for Relation {
    fn from(this: DataAnalogue) -> Relation {
        Relation::from(this.base)
    }
}

impl From<DataAnalogue> for Flag {
    fn from(this: DataAnalogue) -> Flag {
        Flag::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::relation::attribute::Owner;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(DataAnalogue::archetype().id(), DataAnalogue::TYPE_ID);
        assert_eq!(
            DataAnalogue::archetype().internal_name(),
            Some(Rc::from(DataAnalogue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DataAnalogue::new();
        concept.set_internal_name("A");
        assert_eq!(
            DataAnalogue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(DataAnalogue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(DataAnalogue::archetype().added_attributes(), vec![]);
        assert_eq!(
            DataAnalogue::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DataAnalogue::new();
        let concept_copy = DataAnalogue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DataAnalogue::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
