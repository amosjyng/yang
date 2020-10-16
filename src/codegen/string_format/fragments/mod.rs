use crate::codegen::string_format::sort_imports;

/// Code fragment that can be combined with other code fragments.
mod appended;
/// Code fragment that cannot be broken down any further.
mod atomic;
/// Fragment for an entire code file.
mod file;
/// Fragment for a module declaration.
mod module;
/// Code fragment that nests another code fragment inside.
mod nested;

pub use appended::AppendedFragment;
pub use atomic::AtomicFragment;
pub use file::FileFragment;
use itertools::Itertools;
pub use module::ModuleFragment;
pub use nested::NestedFragment;
use std::collections::HashMap;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4;

/// Group imports together.
fn group_imports(imports: &[&str]) -> Vec<String> {
    let mut groups = HashMap::new();
    for import in imports {
        let mut path: Vec<&str> = import.split("::").collect();
        let name = path.pop().unwrap();
        groups
            .entry(path.iter().format("::").to_string())
            .or_insert(Vec::new())
            .push(name);
    }

    let mut final_imports = Vec::new();
    for (path, names) in &groups {
        let import = if names.len() > 1 {
            let mut sorted_names = names.clone();
            sorted_names.sort();
            format!(
                "{}::{{{}}}",
                path,
                sorted_names.iter().format(", ").to_string()
            )
        } else {
            format!("{}::{}", path, names.first().unwrap())
        };
        final_imports.push(import);
    }
    final_imports
}

/// Serialize imports into a string.
pub fn imports_as_str(imports: &[&str]) -> String {
    // this doesn't need to take into account self.tests because tests don't contribute to file
    // imports
    let mut result = String::new();
    for import in group_imports(imports) {
        result += &format!("use {};\n", import);
    }
    sort_imports(&result).trim().to_owned()
}

/// Represents a fragment of code that can be appended to or nested with other code fragements.
pub trait CodeFragment {
    /// Retrieve all imports used by this fragment.
    fn imports(&self) -> Vec<String>;
    /// Retrieve main body of code in this fragment.
    fn body(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::collections::HashSet;

    /// Convert a vec to a set. todo: deduplicate with same thing in Yin's cypher_graph file
    fn vec_as_set(inputs: Vec<String>) -> HashSet<String> {
        let mut set = HashSet::new();
        for input in inputs {
            set.insert(input);
        }
        set
    }

    macro_rules! assert_unordered_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(vec_as_set($a), vec_as_set($b));
        };
    }

    #[test]
    fn test_group_import_none() {
        assert_eq!(group_imports(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_group_import_uniques() {
        assert_unordered_eq!(
            group_imports(&["std::cell::Cell", "std::rc::Rc"]),
            vec!["std::cell::Cell".to_owned(), "std::rc::Rc".to_owned()]
        );
    }

    #[test]
    fn test_group_import_groups() {
        assert_unordered_eq!(
            group_imports(&["std::cell::RefCell", "std::rc::Rc", "std::cell::Cell"]),
            vec![
                "std::cell::{Cell, RefCell}".to_owned(),
                "std::rc::Rc".to_owned()
            ]
        );
    }

    #[test]
    fn test_imports_as_str() {
        assert_eq!(
            imports_as_str(&["std::cell::RefCell", "std::rc::Rc", "std::cell::Cell"]),
            indoc! {"
                use std::cell::{Cell, RefCell};
                use std::rc::Rc;"}
        );
    }
}
