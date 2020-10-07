use std::rc::Rc;
use zamm_yin::concepts::{Archetype, FormTrait};
use zamm_yin::graph::{unwrap_strong, StrongWrapper};
use zamm_yin::wrappers::BaseNodeTrait;

/// This trait allows documentation to be set and retrieved for a concept.
pub trait Documentable {
    /// Set documentation for this concept.
    fn set_documentation(&mut self, doc: &str);

    /// Retrieve documentation for this concept, if it exists.
    fn documentation(&self) -> Option<Rc<String>>;
}

impl Documentable for Archetype {
    fn set_documentation(&mut self, doc: &str) {
        self.essence_mut()
            .set_value(Box::new(StrongWrapper::new(doc.to_string())))
    }

    fn documentation(&self) -> Option<Rc<String>> {
        unwrap_strong(self.essence().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{initialize_kb, Implement};
    use zamm_yin::concepts::ArchetypeTrait;

    #[test]
    fn test_archetype_documentation() {
        initialize_kb();
        Implement::archetype().set_documentation("This implements stuff.");
        assert_eq!(
            Implement::archetype().documentation(),
            Some(Rc::new("This implements stuff.".to_string()))
        )
    }
}
