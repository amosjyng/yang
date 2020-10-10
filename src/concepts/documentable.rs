use std::rc::Rc;
use zamm_yin::concepts::{Archetype, FormTrait};
use zamm_yin::graph::value_wrappers::KBValue;
use zamm_yin::graph::value_wrappers::StrongValue;
use zamm_yin::node_wrappers::BaseNodeTrait;

/// This trait allows documentation to be set and retrieved for a concept. This does not appear to
/// be usable from external crates.
pub trait Documentable {
    /// Set documentation for this concept.
    fn set_documentation(&mut self, doc: &str);

    /// Retrieve documentation for this concept, if it exists.
    fn documentation(&self) -> Option<Rc<Box<dyn KBValue>>>;
}

impl Documentable for Archetype {
    fn set_documentation(&mut self, doc: &str) {
        set_documentation(Box::new(self), doc)
    }

    fn documentation(&self) -> Option<Rc<Box<dyn KBValue>>> {
        self.essence().value()
    }
}

/// Set documentation for this concept.
pub fn set_documentation(form: Box<&mut dyn FormTrait>, doc: &str) {
    form.essence_mut()
        .set_value(Box::new(StrongValue::new(doc.to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{initialize_kb, Implement};
    use zamm_yin::concepts::ArchetypeTrait;
    use zamm_yin::graph::value_wrappers::unwrap_strong;

    #[test]
    fn test_archetype_documentation() {
        initialize_kb();
        Implement::archetype().set_documentation("This implements stuff.");
        assert_eq!(
            unwrap_strong(&Implement::archetype().documentation()),
            Some(&"This implements stuff.".to_string())
        );
    }
}
