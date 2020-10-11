use super::Implement;
use crate::codegen::{output_code, CodegenConfig};

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, cfg: &CodegenConfig) {
    let impl_cfg = request.config().unwrap();
    output_code(&impl_cfg, cfg);
}
