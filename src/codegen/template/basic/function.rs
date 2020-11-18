use super::{AppendedFragment, CodeFragment, NestedFragment};
use crate::codegen::docstring::into_docstring;
use indoc::formatdoc;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

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
#[derive(Default)]
pub struct FunctionFragment {
    /// Name of the function.
    name: String,
    /// Documentation string for the function.
    doc: Option<String>,
    /// Whether or not this function should be publicly exported out of the function.
    public: bool,
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
            doc: None,
            public: false,
            args: Vec::<FunctionArgument>::new(),
            return_type: None,
            imports: Vec::<String>::new(),
            content: Rc::new(RefCell::new(AppendedFragment::new_with_separator("\n"))),
        }
    }

    /// Set documentation for the function.
    pub fn set_documentation(&mut self, doc: String) {
        self.doc = Some(doc);
    }

    /// Set the function to be public.
    pub fn make_public(&mut self) {
        self.public = true;
    }

    /// Set the return type for the function.
    pub fn set_return(&mut self, return_type: String) {
        self.return_type = Some(return_type);
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

impl CodeFragment for FunctionFragment {
    fn body(&self) -> String {
        let doc = match &self.doc {
            Some(d) => into_docstring(&d, 0),
            None => String::new(),
        };
        let public = if self.public { "pub " } else { "" };
        let args = self
            .args
            .iter()
            .map(|a| format!("{}: {}", a.name, a.arg_type))
            .format(", ");
        let return_type = match &self.return_type {
            Some(actual_return_type) => format!(" -> {}", actual_return_type),
            None => String::default(),
        };
        let nested = NestedFragment {
            imports: Vec::new(), // todo: should NestedFragment be a trait instead?
            preamble: formatdoc! {"
                {doc}
                {public}fn {name}({args}){return_type} {{",
                doc = doc,
                public = public,
                name = self.name,
                args = args,
                return_type = return_type
            },
            nesting: Some(self.content.clone()),
            postamble: "}".to_owned(),
        };
        nested.body()
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
    fn test_documented_function() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.set_documentation("This is a function.".to_owned());

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
        f.make_public();

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(),
            indoc! {"
                pub fn foo() {
                }"}
        );
    }

    #[test]
    fn test_function_return() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.make_public();
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
    fn test_function_imports() {
        let mut f = FunctionFragment::new("foo".to_owned());
        f.set_documentation("This function adds two custom numbers together.".to_owned());
        f.make_public();
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