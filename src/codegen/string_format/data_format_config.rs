use super::FormatConfig;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct DataFormatConfig {
    /// Regular concept config.
    pub tao_cfg: FormatConfig,
    /// Rust primitive that this concept represents.
    pub rust_primitive_name: Rc<String>,
    /// Rust code representation of the default value of this concept.
    pub default_value: Rc<String>,
}
