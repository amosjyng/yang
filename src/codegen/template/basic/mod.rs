/// Code fragment that can be combined with other code fragments.
mod appended;
/// Code fragment that cannot be broken down any further.
mod atomic;
/// Fragment for an entire code file.
mod file;
mod function;
mod item_declaration;
/// Fragment for a module declaration.
mod module;
/// Code fragment that nests another code fragment inside.
mod nested;
mod trait_fragment;

pub use appended::AppendedFragment;
pub use atomic::AtomicFragment;
pub use file::FileFragment;
pub use function::FunctionFragment;
pub use item_declaration::{ItemDeclaration, ItemDeclarationAPI};
pub use module::ModuleFragment;
pub use nested::NestedFragment;
pub use trait_fragment::TraitFragment;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4;

/// Represents a fragment of code that can be appended to or nested with other code fragements.
pub trait CodeFragment {
    /// Retrieve all imports used by this fragment.
    fn imports(&self) -> Vec<String>;
    /// Retrieve main body of code in this fragment.
    fn body(&self) -> String;
}
