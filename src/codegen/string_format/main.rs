use super::fragments::{AtomicFragment, FileFragment, FunctionFragment};
use std::cell::RefCell;
use std::rc::Rc;

/// Output config for main function code generation.
#[derive(Eq, PartialEq, Debug)]
pub struct MainConfig {
    /// Imports that the main function uses
    pub imports: Vec<String>,
    /// Lines of code to include in the main function. Should really be additional fragments to
    /// include.
    pub lines: Vec<String>,
}

/// Generate code for a main function.
pub fn code_main(cfg: &MainConfig) -> String {
    let mut file = FileFragment::default();
    let mut main_function = FunctionFragment::new("main".to_owned());

    for import in &cfg.imports {
        main_function.add_import(import.clone());
    }
    for line in &cfg.lines {
        main_function.append(Rc::new(RefCell::new(AtomicFragment::new(line.clone()))));
    }

    file.append(Rc::new(RefCell::new(main_function)));
    file.generate_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_main() {
        assert_eq!(
            code_main(&MainConfig {
                imports: vec![],
                lines: vec![],
            }),
            indoc! {"
                fn main() {
                }
                "}
        );
    }

    #[test]
    fn test_some_main() {
        assert_eq!(
            code_main(&MainConfig {
                imports: vec!["crate::mymod::Thing".to_owned()],
                lines: vec![
                    "let t = Thing::new();".to_owned(),
                    "t.do_something();".to_owned()
                ],
            }),
            indoc! {"
                use crate::mymod::Thing;

                fn main() {
                    let t = Thing::new();
                    t.do_something();
                }
                "}
        );
    }
}
