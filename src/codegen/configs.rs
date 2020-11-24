/// Runtime options for code generation.
#[derive(Copy, Clone, Debug)]
pub struct CodegenConfig {
    /// Whether or not to mark each generated line of code with the autogeneration comment
    /// specified by `zamm_yang::codegen::mark_autogen::AUTOGENERATION_MARKER`.
    pub comment_autogen: bool,
    /// Whether or not to add rustfmt attributes to prevent rustfmt from acting on certain lines.
    pub add_rustfmt_attributes: bool,
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
            add_rustfmt_attributes: true,
            track_autogen: false,
            yin: false,
            release: false,
        }
    }
}

/// Config representing an imported struct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructConfig {
    /// Name of the Struct.
    pub name: String,
    /// Import path for this Struct.
    pub import: String,
}

impl StructConfig {
    /// Get the StructConfig symbol from just the import string.
    pub fn new(import: String) -> Self {
        Self {
            name: import.split("::").last().unwrap().to_owned(),
            import,
        }
    }
}

impl Default for StructConfig {
    fn default() -> Self {
        Self {
            name: "Dummy".to_owned(),
            import: "zamm_yin::tao::Dummy".to_owned(),
        }
    }
}
