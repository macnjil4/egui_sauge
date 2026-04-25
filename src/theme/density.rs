//! Density preset: one knob that scales padding, gap and interact size
//! across every component that queries it.

/// UI density.
///
/// `Comfortable` is the default (generous spacing, 32 px interactive height).
/// `Compact` tightens every spacing / padding / size token by 0.75×, giving
/// dense tables and long forms without giving up the same visual rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Density {
    /// Generous. Default.
    #[default]
    Comfortable,
    /// Dense. ~0.75× the Comfortable sizes.
    Compact,
}

impl Density {
    /// Multiplier applied to spacing / padding / interact size tokens.
    pub fn scale(self) -> f32 {
        match self {
            Self::Comfortable => 1.0,
            Self::Compact => 0.75,
        }
    }

    /// Interactive element height (minimum hit target) in pixels.
    pub fn interact_size(self) -> f32 {
        match self {
            Self::Comfortable => 32.0,
            Self::Compact => 26.0,
        }
    }
}
