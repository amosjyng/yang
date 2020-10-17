/// High-level code generation logic.
mod code;
/// Format documentation as rustdoc.
mod docstring;
/// Finalized code generation.
mod postprocessing;

/// Actual changes to the filesystem.
pub mod filesystem;
/// Parsing and generation of names
mod name_transform;
/// Format codegen.
pub mod string_format;
/// Track autogenerated files.
pub mod track_autogen;

pub use code::{code, CodeConfig};
pub use name_transform::NameTransform;
pub use postprocessing::mark_autogen::{add_indent, count_indent};
pub use postprocessing::mark_fmt::add_fmt_skips;

/// How many characters per line each autogenerated document should have.
const CODE_WIDTH: usize = 80;

/// Runtime options for code generation.
#[derive(Copy, Clone)]
pub struct CodegenConfig {
    /// Whether or not to mark each generated line of code with the autogeneration comment
    /// specified by `zamm_yang::codegen::mark_autogen::AUTOGENERATION_MARKER`.
    pub comment_autogen: bool,
    /// Whether or not we want Cargo to track autogenerated files and rebuild when they change.
    pub track_autogen: bool,
    /// Whether or not we're outputting code for Yin itself.
    pub yin: bool,
    /// Whether or not we're outputting code for release.
    ///
    /// If we are, the implications are:
    ///
    ///  * No autogeneration comments, so that documentation looks good on docs.rs
    ///  * No `build.rs`, because there's no network access for builds on docs.rs anyways
    ///  * Autogenerated files will be committed instead of ignored, because they can't be built
    ///    without `build.rs` to do it
    ///  * A release branch will be created, ready for cargo publishing
    pub release: bool,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            comment_autogen: true,
            track_autogen: false,
            yin: false,
            release: false,
        }
    }
}
