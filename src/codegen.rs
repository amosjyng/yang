/// Format codegen.
mod string_format {
    /// Generate code for attribute concepts.
    pub mod attribute;
    /// Config values at the time of string generation.
    pub mod format_config;
    /// Import-related code generation.
    pub mod imports;
    /// Generate code for generic Tao concepts.
    pub mod tao;

    pub use format_config::FormatConfig;
    pub use imports::sort_imports;
}
/// Format documentation as rustdoc.
mod docstring;
/// Logic for ignoring autogenerated files in Git.
mod git_ignore;
/// Mark files as autogenerated.
mod mark_autogen;
/// Mark lines as not needing formatting.
///
/// Sometimes, it's just too hard to autogenerate perfectly formatted code.
mod mark_fmt;
/// Parsing and generation of names
mod name_transform;
/// Track autogenerated files.
pub mod track_autogen;

use crate::concepts::ImplementConfig;
use git_ignore::{git_ignore, git_rm};
use mark_autogen::add_autogeneration_comments;
pub use mark_autogen::{add_indent, count_indent};
pub use mark_fmt::add_fmt_skips;
pub use name_transform::NameTransform;
use path_abs::PathAbs;
use std::fs;
use std::path::Path;
use string_format::attribute::code_attribute;
use string_format::tao::code_tao;
use string_format::FormatConfig;
use track_autogen::track_autogen;

/// How many characters per line each autogenerated document should have.
const CODE_WIDTH: usize = 80;

/// Runtime options for code generation.
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

/// Do post-processing on generated code. Includes marking lines with autogeneration comments, or
/// marking lines as requiring formatting skips.
fn post_process_generation(code: &str, options: &CodegenConfig) -> String {
    let formatted = add_fmt_skips(&code);
    if options.comment_autogen && !options.release {
        add_autogeneration_comments(&formatted)
    } else {
        formatted
    }
}

/// Generate the final version of code, to be output to a file as-is.
fn code(implement: &ImplementConfig, options: &CodegenConfig) -> String {
    let format_cfg = FormatConfig::from_cfgs(implement, options);
    // todo: get attribute parents once Yin supports that
    let initial_code = if implement.parent_name == "Attribute" {
        code_attribute(&format_cfg)
    } else {
        code_tao(&format_cfg)
    };
    post_process_generation(&initial_code, options)
}

/// Output code to filename
pub fn output_code(implement: &ImplementConfig, options: &CodegenConfig) {
    let generated_code = code(implement, options);
    let folder = if implement.parent_name == "Attribute" {
        "src/concepts/attributes"
    } else {
        "src/concepts"
    };
    let file_relative = format!(
        "{}/{}.rs",
        folder,
        NameTransform::from_camel_case(&implement.name).to_snake_case()
    );
    let file_pathabs = PathAbs::new(Path::new(&file_relative))
        .unwrap_or_else(|_| panic!("Could not get absolute path for {}", file_relative));
    let file_absolute = file_pathabs.as_path().to_str().unwrap();
    let file_parent = file_pathabs
        .as_path()
        .parent()
        .unwrap_or_else(|| panic!("Could not get parent directory for {}", file_absolute));
    fs::create_dir_all(file_parent).unwrap_or_else(|_| {
        panic!(
            "Could not create intermediate directories for {}",
            file_absolute
        )
    });
    git_rm(&file_absolute);
    fs::write(file_absolute, generated_code)
        .unwrap_or_else(|_| panic!("Couldn't output generated code to {}", file_absolute));
    // track in .autogen for completeness, regardless of release options
    track_autogen(file_pathabs.as_path().to_str().unwrap().to_owned());
    if !options.release {
        // don't ignore so that files can be added to Git and compiled on docs.rs, because docs.rs
        // does not allow the yang binary to be downloaded
        git_ignore(&file_pathabs).unwrap_or_else(|_| panic!("Could not ignore {}", file_absolute));
    }
    if options.track_autogen {
        // tell cargo to regenerate autogenerated files when they're edited or removed
        println!("cargo:rerun-if-changed={}", file_relative);
    } else {
        println!("Generated {}", file_relative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::mark_autogen::AUTOGENERATION_MARKER;
    use crate::codegen::mark_fmt::FMT_SKIP_MARKER;

    #[test]
    fn test_post_process_comments() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: true,
            track_autogen: false,
            yin: false,
            release: false,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "dummy".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(AUTOGENERATION_MARKER));
        assert!(result.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_post_process_no_comments() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: false,
            track_autogen: false,
            yin: false,
            release: false,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "dummy".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains(AUTOGENERATION_MARKER));
    }

    #[test]
    fn test_post_process_yin() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: true,
            track_autogen: false,
            yin: true,
            release: false,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "dummy".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_post_process_fmt_not_skip() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: true,
            track_autogen: false,
            yin: false,
            release: false,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "short".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: true,
            track_autogen: false,
            yin: false,
            release: false,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip_release() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: true,
            track_autogen: false,
            yin: false,
            release: true,
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                parent_name: "doh".to_owned(),
                doc: None,
                id: 3,
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn integration_test_attribute_generation() {
        assert!(code(
            &ImplementConfig {
                name: "Target".to_owned(),
                parent_name: "Attribute".to_owned(),
                id: 1,
                doc: Some("The target of an implement command.".to_owned()),
            },
            &CodegenConfig::default()
        )
        .contains("Attribute"));
    }

    #[test]
    fn integration_test_non_attribute_generation() {
        assert!(!code(
            &ImplementConfig {
                name: "Data".to_owned(),
                parent_name: "Tao".to_owned(),
                id: 1,
                doc: None,
            },
            &CodegenConfig::default()
        )
        .contains("Attribute"));
    }
}
