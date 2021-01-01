use crate::tao::callbacks::implements;
use crate::tao::perspective::KnowledgeGraphNode;
use std::cell::Cell;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::Attribute;

thread_local! {
    static IMPORT_IN_PROGRESS: Cell<bool> = Cell::new(false);
}

/// Returns whether or not we're currently in the process of importing another file.
pub fn import_in_progress() -> bool {
    IMPORT_IN_PROGRESS.with(|i| i.get())
}

fn mark_import(status: bool) {
    IMPORT_IN_PROGRESS.with(|i| i.set(status));
}

/// Defines a new concept with the given name.
#[macro_export]
macro_rules! define {
    ($name:ident) => {
        let mut $name = Tao::archetype().individuate_as_archetype();
        $name.set_internal_name(stringify!($name));
        {
            let mut kgn = zamm_yang::tao::perspective::KnowledgeGraphNode::from($name.id());
            if zamm_yang::helper::import_in_progress() {
                kgn.mark_imported();
            } else {
                kgn.mark_newly_defined();
            }
        }
    };
    ($name:ident, $doc:expr) => {
        define!($name);
        $name.implement_with_doc($doc);
    };
}

/// Defines a module for the given concept.
#[macro_export]
macro_rules! module {
    ($name:expr, $doc:expr) => {
        $name.impl_mod($doc)
    };
    ($name:expr, $doc:expr, [$($extension:expr),*]) => {
        {
            let mut new_mod = $name.impl_mod($doc);
            $(
                new_mod.has_extension($extension);
            )*
        }
    };
}

/// Defines a new concept as a child of the given parent type, defined within the current context.
#[macro_export]
macro_rules! define_child {
    ($name:ident, $parent:expr) => {
        define!($name);
        $name.add_parent($parent.into());
    };
    ($name:ident, $parent:expr, $doc:expr) => {
        define!($name, $doc);
        $name.add_parent($parent.into());
    };
}

/// Defines a new flag, and add it as a property of the owner.
#[macro_export]
macro_rules! add_flag {
    ($name:ident, $owner:ident, $doc:expr, $dual_doc:expr) => {
        add_flag!(
            $name <= zamm_yang::tao::relation::flag::Flag::archetype(),
            $owner,
            $doc,
            $dual_doc
        );
    };
    ($name:ident <= $parent:expr, $owner:ident, $doc:expr, $dual_doc:expr) => {
        define_child!($name, $parent);
        zamm_yang::tao::archetype::AttributeArchetype::from($name.id()).set_owner_archetype($owner);
        $owner.add_flag($name);
        {
            let mut new_impl = $name.implement_with_doc($doc);
            new_impl.dual_document($dual_doc);
        }
    };
}

/// Defines a new attribute, and add it as a property of the owner.
#[macro_export]
macro_rules! add_attr {
    ($name:ident, $owner:expr, $value:expr, $doc:expr, $dual_doc:expr) => {
        add_attr!(
            $name <= zamm_yang::tao::relation::attribute::Attribute::archetype(),
            $owner,
            $value,
            $doc,
            $dual_doc
        );
    };
    ($name:ident <= $parent:expr, $owner:expr, $value:expr, $doc:expr, $dual_doc:expr) => {
        define_child!($name, $parent);
        {
            let mut new_aa = zamm_yang::tao::archetype::AttributeArchetype::from($name.id());
            if $owner != zamm_yang::tao::Tao::archetype() {
                new_aa.set_owner_archetype($owner);
            }
            if $value != zamm_yang::tao::Tao::archetype() {
                new_aa.set_value_archetype($value);
            }
            $owner.add_attribute(new_aa);
            let mut new_impl = $name.implement_with_doc($doc);
            new_impl.dual_document($dual_doc);
        }
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

/// Start interpreting new information as imported. New concepts will not be marked as new, but
/// instead as imported.
pub fn start_imports() {
    mark_import(true);
}

/// Mark all concepts introduced thus far as imported ones. Imports are informational-only and will
/// not be acted on.
pub fn end_imports() {
    mark_import(false);
    for i in implements() {
        KnowledgeGraphNode::from(i.id()).mark_imported();
    }
}

/// Backwards compatibility trait to handle API changes for this yang-0.x.* branch.
pub trait BackwardsCompatibility {}
