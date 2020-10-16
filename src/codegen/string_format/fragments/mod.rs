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
pub use module::ModuleFragment;
pub use nested::NestedFragment;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4;

/// Serialize imports into a string.
pub fn imports_as_str(imports: &[String]) -> String {
    // this doesn't need to take into account self.tests because tests don't contribute to file
    // imports
    let mut result = String::new();
    for import in imports {
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
