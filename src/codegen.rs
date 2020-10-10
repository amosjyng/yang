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

use crate::concepts::Documentable;
use crate::concepts::Implement;
use attribute::code_attribute;
use docstring::into_docstring;
use git_ignore::{git_ignore, git_rm};
use mark_autogen::add_autogeneration_comments;
use path_abs::PathAbs;
use std::fs;
use std::path::Path;
use zamm_yin::graph::value_wrappers::unwrap_strong;
use zamm_yin::node_wrappers::CommonNodeTrait;

/// Configurable options for generating a concept code file.
pub struct OutputOptions<'a> {
    name: &'a str,
    doc: Option<&'a String>,
    id: usize,
    comment_autogen: bool,
    yin: bool,
}

/// Output code to filename
fn output_code<'a>(options: &OutputOptions<'a>) {
    let generated_code = code_attribute(options);
    let file_relative = format!("src/concepts/attributes/{}.rs", options.name.to_lowercase());
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
    // tell cargo to regenerate autogenerated files when they're edited or removed
    println!("cargo:rerun-if-changed={}", file_relative);
}

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, id: usize, comment_autogen: bool, yin: bool) {
    let target = request.target().unwrap();

    output_code(&OutputOptions {
        name: &target.internal_name().unwrap(),
        doc: unwrap_strong(&target.documentation()),
        id: id,
        comment_autogen: comment_autogen,
        yin: yin,
    })
}
