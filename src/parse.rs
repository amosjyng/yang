/// Parses Markdown into concepts.
pub mod markdown;
/// Parses YAML into concepts.
pub mod yaml;

pub use markdown::parse_markdown;
pub use yaml::parse_yaml;
