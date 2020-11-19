mod attribute_activation;

use crate::tao::BuildInfo;
use crate::tao::{Implement, ImplementExtension};
pub use attribute_activation::CodegenFlags;
use zamm_yin::node_wrappers::CommonNodeTrait;
pub use zamm_yin::tao::archetype::*;
use zamm_yin::tao::form::FormTrait;

/// Convenience trait for creating a new implementation of a concept.
pub trait CreateImplementation: FormTrait + CommonNodeTrait {
    /// Create a new implementation for a concept.
    fn implement(&self) -> Implement {
        let mut implementation = Implement::new();
        implementation.set_target(Archetype::from(self.id()));
        implementation
    }

    /// Implement this concept with the given documentation string.
    fn implement_with_doc(&self, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.document(doc);
        implementation
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
