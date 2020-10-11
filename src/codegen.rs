/// Generate code for attribute files.
mod attribute;
/// Format documentation as rustdoc.
mod docstring;
/// Logic for ignoring autogenerated files in Git.
mod git_ignore;
/// Mark files as autogenerated.
mod mark_autogen;
/// Track autogenerated files.
pub mod track_autogen;

use crate::concepts::ImplementConfig;
use attribute::code_attribute;
use docstring::into_docstring;
use git_ignore::{git_ignore, git_rm};
use mark_autogen::add_autogeneration_comments;
use path_abs::PathAbs;
use std::fs;
use std::path::Path;

/// Runtime options for code generation.
pub struct CodegenConfig {
    /// Whether or not to mark each generated line of code with the autogeneration comment
    /// specified by `zamm_yang::codegen::mark_autogen::AUTOGENERATION_MARKER`.
    pub comment_autogen: bool,
    /// Whether or not we want Cargo to track autogenerated files and rebuild when they change.
    pub track_autogen: bool,
    /// Whether or not we're outputting code for Yin itself.
    pub yin: bool,
}

/// Output code to filename
pub fn output_code<'a>(implement: &ImplementConfig, options: &CodegenConfig) {
    let generated_code = code_attribute(implement, options);
    let file_relative = format!(
        "src/concepts/attributes/{}.rs",
        implement.name.to_lowercase()
    );
    let file_pathabs = PathAbs::new(Path::new(&file_relative))
        .expect(format!("Could not get absolute path for {}", file_relative).as_str());
    let file_absolute = file_pathabs.as_path().to_str().unwrap();
    let file_parent = file_pathabs
        .as_path()
        .parent()
        .expect(format!("Could not get parent directory for {}", file_absolute).as_str());
    fs::create_dir_all(file_parent).expect(
        format!(
            "Could not create intermediate directories for {}",
            file_absolute
        )
        .as_str(),
    );
    git_rm(&file_absolute);
    fs::write(file_absolute, generated_code)
        .expect(format!("Couldn't output generated code to {}", file_absolute).as_str());
    git_ignore(&file_pathabs).expect(format!("Could not ignore {}", file_absolute).as_str());
    if options.track_autogen {
        // tell cargo to regenerate autogenerated files when they're edited or removed
        println!("cargo:rerun-if-changed={}", file_relative);
    } else {
        println!("Generated {}", file_relative);
    }
}
