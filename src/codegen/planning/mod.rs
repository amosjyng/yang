mod archetype;
mod id;
mod imports;
mod init;
mod module;

use id::grab_new_implementation_id;
use imports::{concept_to_struct, in_own_submodule};

pub use archetype::code_archetype;
pub use imports::{archetype_file_path, module_file_path};
pub use init::handle_init;
pub use module::code_module;
