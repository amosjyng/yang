use itertools::Itertools;
use std::collections::HashMap;

/// Sort imports alphabetically.
pub fn sort_imports(imports: &str) -> String {
    let mut import_lines: Vec<&str> = imports.split('\n').collect();
    import_lines.sort_by_key(|s| {
        let n = s.len();
        if n > 0 {
            &s[..n - 1]
        } else {
            s
        }
    });
    import_lines
        .into_iter()
        .format("\n")
        .to_string()
        .trim()
        .to_owned()
}

/// Group imports together.
fn group_imports(imports: &[&str]) -> Vec<String> {
    let mut groups = HashMap::new();
    for import in imports {
        let mut path: Vec<&str> = import.split("::").collect();
        let name = path.pop().unwrap();
        groups
            .entry(path.iter().format("::").to_string())
            .or_insert_with(Vec::new)
            .push(name);
    }

    let mut final_imports = Vec::new();
    for (path, names) in &groups {
        let import = if names.len() > 1 {
            let mut sorted_names = names.clone();
            sorted_names.sort_unstable();
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
    fn test_sort_imports_crate() {
        assert_eq!(
            sort_imports(indoc! {"
            use std::convert::TryFrom;
            use std::fmt::{Debug, Formatter};
            use std::fmt;
            use std::rc::Rc;
            use crate::concepts::attributes::{Attribute, AttributeTrait};
            use crate::concepts::{ArchetypeTrait, FormTrait, Tao{imports}};
            use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
        "}),
            indoc! {"
            use crate::concepts::attributes::{Attribute, AttributeTrait};
            use crate::concepts::{ArchetypeTrait, FormTrait, Tao{imports}};
            use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
            use std::convert::TryFrom;
            use std::fmt;
            use std::fmt::{Debug, Formatter};
            use std::rc::Rc;
        "}
            .trim()
        );
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
