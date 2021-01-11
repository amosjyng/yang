use crate::tao::relation::attribute::Member;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use zamm_yin::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::{Tao, YIN_MAX_ID};

/// Anything that has members/sub-components.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Collection {
    base: FinalNode,
}

impl Debug for Collection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Collection", self, f)
    }
}

impl From<usize> for Collection {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Collection {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Collection {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Collection {
    type ArchetypeForm = Archetype;
    type Form = Collection;

    const TYPE_ID: usize = YIN_MAX_ID + 30;
    const TYPE_NAME: &'static str = "collection";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl Deref for Collection {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Collection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Collection {}

impl From<Collection> for Tao {
    fn from(this: Collection) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Collection> for Form {
    fn from(this: Collection) -> Form {
        Form::from(this.base)
    }
}

impl Collection {
    /// Get the members of this collection.
    pub fn members(&self) -> Vec<Form> {
        self.deref()
            .base_wrapper()
            .outgoing_nodes(Member::TYPE_ID)
            .into_iter()
            .map(|f| Form::from(f.id()))
            .collect()
    }

    /// Add one of the members of this collection.
    pub fn add_member(&mut self, member: &Form) {
        self.deref_mut()
            .add_outgoing(Member::TYPE_ID, member.deref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::form::Collection;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Member;
    use std::rc::Rc;
    use zamm_yin::node_wrappers::CommonNodeTrait;
    use zamm_yin::tao::archetype::ArchetypeFormTrait;
    use zamm_yin::tao::form::Form;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Collection::archetype().id(), Collection::TYPE_ID);
        assert_eq!(
            Collection::archetype().internal_name(),
            Some(Rc::from(Collection::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Collection::new();
        concept.set_internal_name("A");
        assert_eq!(Collection::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Collection::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Collection::archetype().added_attributes(),
            vec![Member::archetype()]
        );
        assert_eq!(
            Collection::archetype().attributes(),
            vec![Member::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Collection::new();
        let concept_copy = Collection::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Collection::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_and_get_member() {
        initialize_kb();
        let mut new_instance = Collection::new();
        assert_eq!(new_instance.members(), vec![]);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.add_member(&value);
        assert_eq!(new_instance.members(), vec![value]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_member_non_inheritance() {
        initialize_kb();
        let new_type = Collection::archetype().individuate_as_archetype();
        let new_instance = Collection::from(new_type.individuate_as_form().id());
        assert_eq!(new_instance.members(), vec![]);

        let value = Form::new();
        #[allow(clippy::clone_on_copy)]
        Collection::from(new_type.id()).add_member(&value);
        assert_eq!(new_instance.members(), vec![]);
    }

    #[test]
    #[allow(clippy::clone_double_ref)]
    fn test_set_member_multiple_times() {
        initialize_kb();
        let mut new_instance = Collection::new();
        let default = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.add_member(&default);
        #[allow(clippy::clone_on_copy)]
        assert_eq!(new_instance.members(), vec![default]);

        let new_value = Form::new();
        #[allow(clippy::clone_on_copy)]
        new_instance.add_member(&new_value);
        assert_eq!(new_instance.members(), vec![default, new_value]);
    }
}
