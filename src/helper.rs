use crate::tao::perspective::KnowledgeGraphNode;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::Attribute;

/// Defines a new concept with the given name.
#[macro_export]
macro_rules! define {
    ($name:ident) => {
        let mut $name = Tao::archetype().individuate_as_archetype();
        $name.set_internal_name_str(stringify!($name));
        zamm_yang::tao::perspective::KnowledgeGraphNode::from($name.id()).mark_newly_defined();
    };
}

/// Defines a new concept as a child of the given parent type, defined within the current context.
#[macro_export]
macro_rules! define_child {
    ($name:ident, $parent:ident) => {
        define!($name);
        $name.add_parent($parent);
    };
    ($name:ident, $parent:ident, $doc:expr) => {
        define!($name);
        $name.add_parent($parent);
        $name.implement_with_doc($doc);
    };
}

/// Defines a new concept as a child of the given imported parent type.
#[macro_export]
macro_rules! define_child_imported {
    ($name:ident, $parent:ty) => {
        define!($name);
        $name.add_parent(<$parent>::archetype().into());
    };
    ($name:ident, $parent:ty, $doc:expr) => {
        define!($name);
        $name.add_parent(<$parent>::archetype().into());
        $name.implement_with_doc($doc);
    };
}

/// Defines a new flag, and add it as a property of the owner.
#[macro_export]
macro_rules! add_flag {
    ($name:ident, $owner:ident) => {
        define_child_imported!($name, Flag);
        zamm_yang::tao::archetype::AttributeArchetype::from($name.id()).set_owner_archetype($owner);
        $owner.add_flag($name);
    };
}

/// Convenience function to convert an `Archetype` to an `AttributeArchetype`.
pub fn aa(archetype: Archetype) -> AttributeArchetype {
    if !(KnowledgeGraphNode::from(archetype.id()).is_attribute_analogue()
        || archetype.has_parent(Attribute::archetype().into())
        || archetype
            .parents()
            .iter()
            .any(|a| KnowledgeGraphNode::from(a.id()).is_attribute_analogue()))
    {
        // currently catches Relation, which is not an attribute but still deserves to have its
        // owner archetype set
        println!("Warning: {:?} is not known to be an attribute.", archetype);
    }
    AttributeArchetype::from(archetype.id())
}

/// Backwards compatibility trait to handle API changes for this yang-0.x.* branch.
pub trait BackwardsCompatibility {}
