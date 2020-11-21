use crate::tao::form::{BuildInfoExtension, Module, ModuleExtension};
use crate::tao::BuildInfo;
use crate::tao::{Implement, ImplementExtension};
use heck::SnakeCase;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};

/// Convenience trait for creating a new implementation of a concept.
pub trait CreateImplementation: FormTrait + CommonNodeTrait {
    /// Create a new implementation for a concept.
    fn implement(&self) -> Implement {
        let mut implementation = Implement::new();
        implementation.set_target(Form::from(self.id()));
        implementation
    }

    /// Implement this concept with the given documentation string.
    fn implement_with_doc(&self, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.document(doc);
        implementation
    }

    /// Implement the module for this concept.
    fn impl_mod(&self, doc: &str) -> Module {
        // todo: implementation info should be built as part of Yin, so that we know here what to
        // use for the intermediate modules
        let mut implementation = Implement::new();
        let mut new_module = Module::new();
        BuildInfo::from(new_module.id())
            .set_implementation_name(&self.internal_name().unwrap().to_snake_case());
        new_module.set_most_prominent_member(&self.as_form());
        implementation.set_target(new_module.as_form());
        implementation.document(doc);
        new_module
    }

    /// Create a new implementation with the specified ID and documentation string.
    #[deprecated(
        since = "0.1.1",
        note = "Please use implement_with_doc instead, and leave the ID up to the program."
    )]
    fn implement_with(&self, id: usize, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.set_implementation_id(id);
        implementation.document(doc);
        implementation
    }

    /// Look at this concept through the BuildInfo lens.
    fn build_info(&self) -> BuildInfo {
        BuildInfo::from(self.id())
    }
}

impl CreateImplementation for Archetype {}
impl CreateImplementation for AttributeArchetype {}
