/// Config for wrapper functions.
pub struct WrapperConfig {
    /// Immutable deref accessor.
    pub deref: &'static str,
    /// Mutable deref accessor.
    pub deref_mut: &'static str,
}

impl Default for WrapperConfig {
    fn default() -> Self {
        YIN_0_1_X
    }
}

/// Unwrap functions for Yin 0.1.x versions.
pub const YIN_0_1_X: WrapperConfig = WrapperConfig {
    deref: "essence",
    deref_mut: "essence_mut",
};

/// Unwrap functions for Yin 0.2.x versions.
pub const YIN_0_2_X: WrapperConfig = WrapperConfig {
    deref: "deref",
    deref_mut: "deref_mut",
};
