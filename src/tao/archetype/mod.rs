mod attribute_activation;

use crate::tao::{Implement, ImplementConfig};
pub use attribute_activation::CodegenFlags;
use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::FormTrait;
use crate::tao::BuildInfo;

/// Convenience trait for creating a new implementation of a concept.
pub trait CreateImplementation: FormTrait {
    /// Create a new implementation for a concept.
    fn implement(&self) -> Implement {
        let mut implementation = Implement::individuate();
        implementation.set_target(Archetype::from(self.id()));
        implementation
    }

    /// Create a new implementation with the specified ID and documentation string.
    fn implement_with(&self, id: usize, doc: &str) -> Implement {
        let mut implementation = self.implement();
        implementation.set_config(ImplementConfig {
            id,
            doc: Some(doc.to_owned()),
        });
        implementation
    }

    /// Look at this concept through the BuildInfo lens.
    fn build_info(&self) -> BuildInfo {
        BuildInfo::from(self.id())
    }
}

impl CreateImplementation for Archetype {}
impl CreateImplementation for AttributeArchetype {}
