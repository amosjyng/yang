mod archetype;
mod id;
pub mod imports;
mod init;
mod module;

use id::grab_new_implementation_id;
use imports::concept_to_struct;

pub use archetype::{code_archetype, code_archetype_trait};
pub use imports::{archetype_file_path, in_own_submodule, module_file_path};
pub use init::handle_init;
pub use module::code_module;
