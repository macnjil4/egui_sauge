//! Numerical design tokens: spacing grid and border radii.

/// 4-point spacing scale.
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    /// 4 px.
    pub s1: f32,
    /// 8 px.
    pub s2: f32,
    /// 12 px.
    pub s3: f32,
    /// 16 px.
    pub s4: f32,
    /// 24 px.
    pub s5: f32,
    /// 32 px.
    pub s6: f32,
    /// 48 px.
    pub s7: f32,
    /// 64 px.
    pub s8: f32,
}

/// The canonical spacing scale (4, 8, 12, 16, 24, 32, 48, 64).
pub const SPACING: Spacing = Spacing {
    s1: 4.0,
    s2: 8.0,
    s3: 12.0,
    s4: 16.0,
    s5: 24.0,
    s6: 32.0,
    s7: 48.0,
    s8: 64.0,
};

/// Border radius scale.
#[derive(Debug, Clone, Copy)]
pub struct Radius {
    /// 4 px — inputs, small badges.
    pub sm: f32,
    /// 8 px — buttons, cards (default).
    pub md: f32,
    /// 12 px — modals, popovers.
    pub lg: f32,
    /// 16 px — large surfaces.
    pub xl: f32,
    /// 9999 px — avatars, pills, toggles.
    pub full: f32,
}

/// The canonical radius scale.
pub const RADIUS: Radius = Radius {
    sm: 4.0,
    md: 8.0,
    lg: 12.0,
    xl: 16.0,
    full: 9999.0,
};
