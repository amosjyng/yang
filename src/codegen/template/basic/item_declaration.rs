use super::{AtomicFragment, CodeFragment, NestedFragment};
use crate::codegen::docstring::into_docstring;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// API for all items to be modified in the same way.
pub trait ItemDeclarationAPI {
    /// Mark this as a public declaration.
    fn mark_as_public(&mut self);

    /// Whether this has been marked as a public declaration or not.
    fn is_public(&self) -> bool;

    /// Set the documentation for the fragment.
    fn document(&mut self, documentation: String);

    /// Add an attribute to the declaration.
    fn add_attribute(&mut self, attribute: String);

    /// Set an implementation to go along with the declaration.
    fn set_body(&mut self, body: Rc<RefCell<dyn CodeFragment>>);
}

/// Fragment containing all universal modifiers for an item declaration.
#[derive(Clone)]
pub struct ItemDeclaration {
    /// Documentation string for the item.
    pub doc: Option<String>,
    /// Whether or not this item should be publicly exported out of the module.
    pub public: bool,
    /// Attributes to be added to the item.
    pub attributes: Vec<String>,
    /// Actual definition of the item, whether it be a variable, function, or module.
    pub definition: Rc<RefCell<dyn CodeFragment>>,
    /// Some items, such as functions and submodules, may have actual implementations that go along
    /// with their declaration.
    pub body: Option<Rc<RefCell<dyn CodeFragment>>>,
}

impl ItemDeclaration {
    /// Create a new item declaration.
    pub fn new(definition: Rc<RefCell<dyn CodeFragment>>) -> Self {
        Self {
            definition,
            ..Self::default()
        }
    }

    /// Set the fragment that defines this item.
    pub fn set_definition(&mut self, definition: Rc<RefCell<dyn CodeFragment>>) {
        self.definition = definition;
    }
}

impl Default for ItemDeclaration {
    fn default() -> Self {
        Self {
            doc: None,
            public: false,
            attributes: vec![],
            definition: Rc::new(RefCell::new(AtomicFragment::default())),
            body: None,
        }
    }
}

impl ItemDeclarationAPI for ItemDeclaration {
    fn mark_as_public(&mut self) {
        self.public = true;
    }

    fn is_public(&self) -> bool {
        self.public
    }

    fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }

    fn document(&mut self, documentation: String) {
        self.doc = Some(documentation);
    }

    fn set_body(&mut self, body: Rc<RefCell<dyn CodeFragment>>) {
        self.body = Some(body);
    }
}

impl CodeFragment for ItemDeclaration {
    fn body(&self) -> String {
        let doc = match &self.doc {
            Some(d) => into_docstring(&d, 0) + "\n",
            None => String::new(),
        };
        let public = if self.public { "pub " } else { "" };
        let mut attrs = self
            .attributes
            .iter()
            .map(|a| format!("#[{}]", a))
            .format("\n")
            .to_string();
        if !attrs.is_empty() {
            attrs.push('\n');
        }
        let preamble = format!(
            "{doc}{attrs}{public}{definition}",
            doc = doc,
            attrs = attrs,
            public = public,
            definition = self.definition.borrow().body(),
        )
        .trim()
        .to_owned();
        match &self.body {
            Some(actual_implementation) => NestedFragment {
                imports: vec![],
                preamble: format!("{} {{", preamble),
                nesting: Some(actual_implementation.clone()),
                postamble: "}".to_owned(),
            }
            .body(),
            None => format!("{};", preamble),
        }
    }

    fn imports(&self) -> Vec<String> {
        self.definition.borrow().imports()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn simple_declaration() -> ItemDeclaration {
        ItemDeclaration::new(Rc::new(RefCell::new(AtomicFragment::new(
            "fn foo() -> bool".to_owned(),
        ))))
    }

    #[test]
    fn test_simple_declaration() {
        let i = simple_declaration();

        assert_eq!(i.imports(), Vec::<String>::new());
        assert_eq!(i.body(), "fn foo() -> bool;");
    }

    #[test]
    fn test_public_declaration() {
        let mut i = simple_declaration();
        i.mark_as_public();

        assert_eq!(i.imports(), Vec::<String>::new());
        assert_eq!(i.body(), "pub fn foo() -> bool;");
    }

    #[test]
    fn test_documented_declaration() {
        let mut i = simple_declaration();
        i.document("Some bloody documentation for ya.".to_owned());

        assert_eq!(i.imports(), Vec::<String>::new());
        assert_eq!(
            i.body(),
            indoc! {"
            /// Some bloody documentation for ya.
            fn foo() -> bool;"}
        );
    }

    #[test]
    fn test_attributed_declaration() {
        let mut i = simple_declaration();
        i.add_attribute("allow(deprecated)".to_owned());

        assert_eq!(i.imports(), Vec::<String>::new());
        assert_eq!(
            i.body(),
            indoc! {"
            #[allow(deprecated)]
            fn foo() -> bool;"}
        );
    }

    #[test]
    fn test_combined_declaration() {
        let mut i = simple_declaration();
        i.mark_as_public();
        i.document("Some bloody documentation for ya.".to_owned());
        i.add_attribute("allow(deprecated)".to_owned());

        assert_eq!(i.imports(), Vec::<String>::new());
        assert_eq!(
            i.body(),
            indoc! {"
            /// Some bloody documentation for ya.
            #[allow(deprecated)]
            pub fn foo() -> bool;"}
        );
    }
}
