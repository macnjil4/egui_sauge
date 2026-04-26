//! Density preset: one knob that scales padding, gap and interact size
//! across every component that queries it.

/// UI density.
///
/// `Comfortable` is the default (32 px interactive height). `Compact`
/// tightens spacing / padding / size tokens by 0.75× for dense tables and
/// long forms; `Spacious` opens them up by 1.25× for tactile / touch-first
/// surfaces (kiosks, tablets, accessibility large-target mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Density {
    /// Generous (1.25×, 40 px target). Touch-friendly.
    Spacious,
    /// Default (1×, 32 px target).
    #[default]
    Comfortable,
    /// Dense (0.75×, 26 px target).
    Compact,
}

impl Density {
    /// Multiplier applied to spacing / padding tokens.
    pub fn scale(self) -> f32 {
        match self {
            Self::Spacious => 1.25,
            Self::Comfortable => 1.0,
            Self::Compact => 0.75,
        }
    }

    /// Interactive element height (minimum hit target) in pixels.
    pub fn interact_size(self) -> f32 {
        match self {
            Self::Spacious => 40.0,
            Self::Comfortable => 32.0,
            Self::Compact => 26.0,
        }
    }
}
