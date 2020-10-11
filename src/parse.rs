/// Parses Markdown into concepts.
pub mod markdown;
/// Parses YAML into concepts.
pub mod yaml;

pub use markdown::parse_md;
pub use yaml::parse_yaml;
