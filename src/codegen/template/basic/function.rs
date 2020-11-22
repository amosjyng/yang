use super::{AppendedFragment, AtomicFragment, CodeFragment, ItemDeclaration, ItemDeclarationAPI};
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// The type of reference to &self in a function argument.
pub enum SelfReference {
    /// When this function makes no reference to a containing struct. Equivalent to no "self" in
    /// the arguments list
    None,
    /// Equivalent to "&self" at the beginning of the arguments list.
    Immutable,
    /// Equivalent to "&mut self" at the beginning of the arguments list.
    Mutable,
}

impl Default for SelfReference {
    fn default() -> Self {
        Self::None
    }
}

/// Function argument.
pub struct FunctionArgument {
    /// Argument name.
    pub name: String,
    /// Argument type.
    pub arg_type: String,
}

impl FunctionArgument {
    /// Create a new argument with the given name and type.
    pub fn new(name: String, arg_type: String) -> Self {
        Self { name, arg_type }
    }
}

/// Fragment for a function.
pub struct FunctionFragment {
    /// Name of the function.
    name: String,
    /// Declaration fragment.
    declaration: ItemDeclaration,
    /// The type of reference to &self in a function argument.
    reference: SelfReference,
    /// Arguments of the function.
    args: Vec<FunctionArgument>,
    /// Type of data that the function returns.
    return_type: Option<String>,
    /// Any optional imports for the args or return type. todo: imports should be tied to the type
    imports: Vec<String>,
    /// Actual internal code fragments for the function.
    content: Rc<RefCell<AppendedFragment>>,
}

impl FunctionFragment {
    /// Create a new function with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    /// Set the function to be a test function.
    pub fn mark_as_test(&mut self) {
        self.declaration.add_attribute("test".to_owned());
    }

    /// Set the return type for the function.
    pub fn set_return(&mut self, return_type: String) {
        self.return_type = Some(return_type);
    }

    /// Set the self reference type for the function, if it's part of a struct or a trait.
    pub fn set_self_reference(&mut self, reference: SelfReference) {
        self.reference = reference;
    }

    /// Add a new argument to the function.
    pub fn add_arg(&mut self, name: String, arg_type: String) {
        self.args.push(FunctionArgument::new(name, arg_type));
    }

    /// Add a new import for the args or return type.
    pub fn add_import(&mut self, import: String) {
        self.imports.push(import);
    }

    /// Add a fragment to the internals of this module.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }
}

impl Default for FunctionFragment {
    fn default() -> Self {
        let mut declaration = ItemDeclaration::default();
        let content = Rc::new(RefCell::new(AppendedFragment::new_with_separator("\n")));
        declaration.set_body(content.clone());
        Self {
            name: String::default(),
            declaration,
            reference: SelfReference::default(),
            args: vec![],
            return_type: None,
            imports: vec![],
            content,
        }
    }
}

impl ItemDeclarationAPI for FunctionFragment {
    fn mark_as_public(&mut self) {
        self.declaration.mark_as_public();
    }

    fn is_public(&self) -> bool {
        self.declaration.is_public()
    }

    fn add_attribute(&mut self, attribute: String) {
        self.declaration.add_attribute(attribute);
    }

    fn document(&mut self, documentation: String) {
        self.declaration.document(documentation);
    }

    fn set_body(&mut self, body: Rc<RefCell<dyn CodeFragment>>) {
        self.declaration.set_body(body);
    }

    fn mark_as_declare_only(&mut self) {
        self.declaration.mark_as_declare_only();
    }

    fn mark_for_full_implementation(&mut self) {
        self.declaration.mark_for_full_implementation();
    }
}

impl CodeFragment for FunctionFragment {
    fn body(&self) -> String {
        let mut args = self
            .args
            .iter()
            .map(|a| format!("{}: {}", a.name, a.arg_type))
            .collect::<Vec<String>>();
        match self.reference {
            SelfReference::None => (),
            SelfReference::Immutable => args.insert(0, "&self".to_owned()),
            SelfReference::Mutable => args.insert(0, "&mut self".to_owned()),
        };
        let args_str = args.iter().format(", ").to_string();
        let return_type = match &self.return_type {
            Some(actual_return_type) => format!(" -> {}", actual_return_type),
            None => String::default(),
        };
        let mut declaration = self.declaration.clone();
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "fn {name}({args}){return_type}",
            name = self.name,
            args = args_str,
            return_type = return_type
        )))));
        declaration.body()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.imports.clone();
        imports.append(&mut self.content.borrow().imports());
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_function() {
        let f = FunctionFragment::new("foo".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                fn foo() {
                }"}
        );
    }

    #[test]
    fn test_function_declare_only() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.mark_as_declare_only();

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(f.body(), "fn foo();");
    }

    #[test]
    fn test_documented_function() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.document("This is a function.".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                /// This is a function.
                fn foo() {
                }"}
        );
    }

    #[test]
    fn test_public_function() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.mark_as_public();

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                pub fn foo() {
                }"}
        );
    }

    #[test]
    fn test_test_function() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.mark_as_test();

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                #[test]
                fn foo() {
                }"}
        );
    }

    #[test]
    fn test_function_return() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.mark_as_public();
        f.set_return("()".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                pub fn foo() -> () {
                }"}
        );
    }

    #[test]
    fn test_function_statements_return() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.set_return("i64".to_owned());
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::<String>::default(),
            atom: "4".to_owned(),
        })));

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                fn foo() -> i64 {
                    4
                }"} // chosen by fair dice roll. guaranteed to be random.
        );
    }

    #[test]
    fn test_function_args() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.set_return("i64".to_owned());
        f.add_arg("x".to_owned(), "i64".to_owned());
        f.add_arg("y".to_owned(), "u64".to_owned());
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::<String>::default(),
            atom: "x + y".to_owned(),
        })));

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                fn foo(x: i64, y: u64) -> i64 {
                    x + y
                }"}
        );
    }

    #[test]
    fn test_function_self() {
        let mut f = FunctionFragment::new("check".to_owned());
        f.set_self_reference(SelfReference::Immutable);
        f.append(Rc::new(RefCell::new(AtomicFragment::new(
            "assert(self.value > 0);".to_owned(),
        ))));

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                fn check(&self) {
                    assert(self.value > 0);
                }"}
        );
    }

    #[test]
    fn test_function_self_with_other_args() {
        let mut f = FunctionFragment::new("replace".to_owned());
        f.set_self_reference(SelfReference::Mutable);
        f.add_arg("new_value".to_owned(), "i64".to_owned());
        f.append(Rc::new(RefCell::new(AtomicFragment::new(
            "self.value = new_value;".to_owned(),
        ))));

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                fn replace(&mut self, new_value: i64) {
                    self.value = new_value;
                }"}
        );
    }

    #[test]
    fn test_function_imports() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.document("This function adds two custom numbers together.".to_owned());
        f.mark_as_public();
        f.add_import("crate::MyNum".to_owned());
        f.set_return("MyNum".to_owned());
        f.add_arg("x".to_owned(), "MyNum".to_owned());
        f.add_arg("y".to_owned(), "MyNum".to_owned());
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec!["crate::operators::plus".to_owned()],
            atom: "x + y".to_owned(),
        })));

        assert_eq!(f.imports(), vec!["crate::MyNum", "crate::operators::plus"]);
        assert_eq!(
            f.body(),
            indoc! {"
                /// This function adds two custom numbers together.
                pub fn foo(x: MyNum, y: MyNum) -> MyNum {
                    x + y
                }"}
        );
    }
}
