use crate::tao::archetype::rust_item_archetype::RustItemArchetype;
use crate::tao::form::rust_item::data::StrConcept;
use crate::tao::form::rust_item::RustItem;
use crate::tao::relation::attribute::Version;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Crate that a concept was built as a part of.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Crate {
    base: FinalNode,
}

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Crate", self, f)
    }
}

impl From<usize> for Crate {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Crate {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Crate {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Crate {
    type ArchetypeForm = RustItemArchetype;
    type Form = Crate;

    const TYPE_ID: usize = YIN_MAX_ID + 36;
    const TYPE_NAME: &'static str = "crate";
    const PARENT_TYPE_ID: usize = RustItem::TYPE_ID;
}

impl Deref for Crate {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Crate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Crate {}

impl From<Crate> for Tao {
    fn from(this: Crate) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Crate> for Form {
    fn from(this: Crate) -> Form {
        Form::from(this.base)
    }
}

impl From<Crate> for RustItem {
    fn from(this: Crate) -> RustItem {
        RustItem::from(this.base)
    }
}

impl Crate {
    /// Get the version number for the crate.
    #[allow(clippy::rc_buffer)]
    pub fn version(&self) -> Option<Rc<str>> {
        self.deref()
            .outgoing_nodes(Version::TYPE_ID)
            .last()
            .map(|f| StrConcept::from(f.id()).value().unwrap())
    }

    /// Set the version number for the crate.
    pub fn set_version(&mut self, version: &str) {
        let mut value_concept = StrConcept::new();
        value_concept.set_value(version);
        self.deref_mut()
            .add_outgoing(Version::TYPE_ID, value_concept.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::form::rust_item::Crate;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Version;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Crate::archetype().id(), Crate::TYPE_ID);
        assert_eq!(
            Crate::archetype().internal_name(),
            Some(Rc::from(Crate::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Crate::new();
        concept.set_internal_name("A");
        assert_eq!(Crate::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Crate::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Crate::archetype().added_attributes(),
            vec![Version::archetype()]
        );
        assert_eq!(Crate::archetype().attributes(), vec![Version::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Crate::new();
        let concept_copy = Crate::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Crate::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_version() {
        initialize_kb();
        let mut new_instance = Crate::new();
        assert_eq!(new_instance.version(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_version(value.clone());
        assert_eq!(new_instance.version(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_version_inheritance() {
        initialize_kb();
        let new_type = Crate::archetype().individuate_as_archetype();
        let new_instance = Crate::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.version(), None);

        let value = "";
        #[allow(clippy::clone_on_copy)]
        Crate::from(new_type.id()).set_version(value.clone());
        assert_eq!(new_instance.version(), Some(Rc::from(value)));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_version_multiple_times() {
        initialize_kb();
        let mut new_instance = Crate::new();
        let default = "";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_version(default.clone());
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.version(), Some(Rc::from(default)));

        let new_value = "test-dummy-str";
        #[allow(clippy::clone_on_copy)]
        new_instance.set_version(new_value.clone());
        assert_eq!(new_instance.version(), Some(Rc::from(new_value)));
    }
}
