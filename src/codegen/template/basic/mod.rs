/// Code fragment that can be combined with other code fragments.
mod appended;
mod assert;
/// Code fragment that cannot be broken down any further.
mod atomic;
/// Fragment for an entire code file.
mod file;
mod function;
mod function_call;
mod impl_fragment;
mod item_declaration;
/// Fragment for a module declaration.
mod module;
/// Code fragment that nests another code fragment inside.
mod nested;
mod trait_fragment;
mod type_specification;
mod vector;

pub use appended::{Appendable, AppendedFragment};
pub use assert::AssertFragment;
pub use atomic::AtomicFragment;
pub use file::FileFragment;
pub use function::{FunctionFragment, SelfReference};
pub use function_call::FunctionCallFragment;
pub use impl_fragment::ImplementationFragment;
pub use item_declaration::{ItemDeclaration, ItemDeclarationAPI};
pub use module::ModuleFragment;
pub use nested::NestedFragment;
pub use trait_fragment::TraitFragment;
pub use type_specification::TypeFragment;
pub use vector::VecFragment;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4; // todo: combined with INDENT_SIZE

/// Represents a fragment of code that can be appended to or nested with other code fragements.
pub trait CodeFragment {
    /// Retrieve all imports used by this fragment.
    fn imports(&self) -> Vec<String>;
    /// Retrieve main body of code in this fragment, restricted to the given line width.
    fn body(&self, line_width: usize) -> String;
}
