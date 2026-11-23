//! # mindset-motion
//!
//! Allocation-free critically damped and configurable spring physics engine
//! engineered for MindSet OS neuroaesthetic UI animations.

/// Neuroscience-inspired motion presets for distinct UI interaction tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringPreset {
    /// High stiffness (420.0), moderate damping (32.0) for button hover and dock icon scaling.
    SnappyMicro,
    /// Balanced stiffness (260.0), damping (26.0) for dragging and snapping floating glass windows.
    FluidPanel,
    /// Gentle stiffness (90.0), calm damping (18.0) for calm peripheral dimming and focus mode transitions.
    DeepFocus,
    /// Alias for DeepFocus transition
    DeepFocusTransition,
}

impl SpringPreset {
    /// Returns `(stiffness, damping)` values for this preset.
    pub const fn params(&self) -> (f32, f32) {
        match self {
            Self::SnappyMicro => (420.0, 32.0),
            Self::FluidPanel => (260.0, 26.0),
            Self::DeepFocus | Self::DeepFocusTransition => (90.0, 18.0),
        }
    }
}

/// A 1-dimensional damped harmonic oscillator with sub-stepped numerical integration.
#[derive(Debug, Clone, Copy)]
pub struct Spring1D {
    pub position: f32,
    pub velocity: f32,
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring1D {
    /// Creates a new spring with explicit stiffness and damping.
    pub fn new(initial: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            position: initial,
            velocity: 0.0,
            target: initial,
            stiffness,
            damping,
        }
    }

    /// Creates a new spring initialized with a neuroscience preset.
    pub fn with_preset(initial: f32, preset: SpringPreset) -> Self {
        let (stiffness, damping) = preset.params();
        Self::new(initial, stiffness, damping)
    }

    /// Sets a new target destination without resetting current velocity.
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Instantly teleports both position and target to `val`, zeroing velocity.
    pub fn snap(&mut self, val: f32) {
        self.position = val;
        self.target = val;
        self.velocity = 0.0;
    }

    /// Checks if the oscillator has settled within the rest threshold.
    pub fn is_at_rest(&self) -> bool {
        (self.position - self.target).abs() < 0.0005 && self.velocity.abs() < 0.0005
    }

    /// Advances the spring simulation by `dt` seconds using sub-stepped semi-implicit Euler integration.
    ///
    /// Clamps `dt` to `[0.0, 0.05]` max (prevents explosion on frame skips/tab switches)
    /// and sub-steps at `1.0 / 240.0` seconds for unconditional stability at any frame rate.
    /// Also guards against NaN, infinite, or negative dt values.
    pub fn update(&mut self, dt: f32) -> f32 {
        if self.is_at_rest() {
            self.position = self.target;
            self.velocity = 0.0;
            return self.position;
        }

        if dt.is_nan() || dt.is_infinite() || dt <= 0.0 {
            return self.position;
        }

        let clamped_dt = dt.clamp(0.0, 0.05);
        if clamped_dt <= 0.0 {
            return self.position;
        }

        let sub_step = 1.0 / 240.0;
        let mut time_accum = clamped_dt;

        while time_accum > 0.000001 {
            let step = time_accum.min(sub_step);

            // Hooke's law + linear viscous damping: F = -k*(x - target) - c*v
            let spring_force = -self.stiffness * (self.position - self.target);
            let damping_force = -self.damping * self.velocity;
            let acceleration = spring_force + damping_force; // assuming mass = 1.0

            // Semi-implicit Euler integration (symplectic)
            self.velocity += acceleration * step;
            self.position += self.velocity * step;

            time_accum -= step;
        }

        // Defensive validation against NaN / Inf propagation
        if self.position.is_nan() || self.position.is_infinite() {
            self.position = self.target;
            self.velocity = 0.0;
        }
        if self.velocity.is_nan() || self.velocity.is_infinite() {
            self.velocity = 0.0;
        }

        // Snap to target if within rest threshold to eliminate perpetual micro-jitter
        if (self.position - self.target).abs() < 0.0005 && self.velocity.abs() < 0.0005 {
            self.position = self.target;
            self.velocity = 0.0;
        }

        self.position
    }
}

/// A 2-dimensional spring animator for positions, sizes, and layout boxes.
#[derive(Debug, Clone, Copy)]
pub struct Spring2D {
    pub x: Spring1D,
    pub y: Spring1D,
}

impl Spring2D {
    pub fn new(x: f32, y: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            x: Spring1D::new(x, stiffness, damping),
            y: Spring1D::new(y, stiffness, damping),
        }
    }

    pub fn with_preset(x: f32, y: f32, preset: SpringPreset) -> Self {
        Self {
            x: Spring1D::with_preset(x, preset),
            y: Spring1D::with_preset(y, preset),
        }
    }

    pub fn set_target(&mut self, target_x: f32, target_y: f32) {
        self.x.set_target(target_x);
        self.y.set_target(target_y);
    }

    pub fn snap(&mut self, x: f32, y: f32) {
        self.x.snap(x);
        self.y.snap(y);
    }

    pub fn is_at_rest(&self) -> bool {
        self.x.is_at_rest() && self.y.is_at_rest()
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x.position, self.y.position)
    }

    pub fn update(&mut self, dt: f32) -> (f32, f32) {
        (self.x.update(dt), self.y.update(dt))
    }
}

/// A 4-channel spring animator for smooth RGBA color transitions.
#[derive(Debug, Clone, Copy)]
pub struct SpringColor {
    pub r: Spring1D,
    pub g: Spring1D,
    pub b: Spring1D,
    pub a: Spring1D,
}

impl SpringColor {
    pub fn new(rgba: [f32; 4], stiffness: f32, damping: f32) -> Self {
        Self {
            r: Spring1D::new(rgba[0], stiffness, damping),
            g: Spring1D::new(rgba[1], stiffness, damping),
            b: Spring1D::new(rgba[2], stiffness, damping),
            a: Spring1D::new(rgba[3], stiffness, damping),
        }
    }

    pub fn with_preset(rgba: [f32; 4], preset: SpringPreset) -> Self {
        Self {
            r: Spring1D::with_preset(rgba[0], preset),
            g: Spring1D::with_preset(rgba[1], preset),
            b: Spring1D::with_preset(rgba[2], preset),
            a: Spring1D::with_preset(rgba[3], preset),
        }
    }

    pub fn set_target(&mut self, rgba: [f32; 4]) {
        self.r.set_target(rgba[0]);
        self.g.set_target(rgba[1]);
        self.b.set_target(rgba[2]);
        self.a.set_target(rgba[3]);
    }

    pub fn snap(&mut self, rgba: [f32; 4]) {
        self.r.snap(rgba[0]);
        self.g.snap(rgba[1]);
        self.b.snap(rgba[2]);
        self.a.snap(rgba[3]);
    }

    pub fn is_at_rest(&self) -> bool {
        self.r.is_at_rest() && self.g.is_at_rest() && self.b.is_at_rest() && self.a.is_at_rest()
    }

    pub fn color(&self) -> [f32; 4] {
        [
            self.r.position,
            self.g.position,
            self.b.position,
            self.a.position,
        ]
    }

    pub fn update(&mut self, dt: f32) -> [f32; 4] {
        [
            self.r.update(dt),
            self.g.update(dt),
            self.b.update(dt),
            self.a.update(dt),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring1d_convergence() {
        let mut spring = Spring1D::with_preset(0.0, SpringPreset::SnappyMicro);
        spring.set_target(100.0);

        // Simulate 2 seconds of 60 FPS frames (dt = 0.0166)
        let dt = 1.0 / 60.0;
        for _ in 0..120 {
            spring.update(dt);
        }

        assert!(spring.is_at_rest());
        assert!((spring.position - 100.0).abs() < 0.001);
        assert_eq!(spring.velocity, 0.0);
    }

    #[test]
    fn test_spring1d_stability_no_nan() {
        let mut spring = Spring1D::new(0.0, 500.0, 10.0);
        spring.set_target(1000.0);

        // Large irregular dt steps, negative dt, NaNs and Infinities to test clamping and sub-stepping stability
        let steps = [
            0.1,
            0.5,
            0.001,
            0.033,
            0.05,
            0.0001,
            -0.016,
            f32::NAN,
            f32::INFINITY,
        ];
        for &dt in &steps {
            let pos = spring.update(dt);
            assert!(!pos.is_nan(), "Position produced NaN!");
            assert!(!pos.is_infinite(), "Position produced Infinite!");
            assert!(!spring.velocity.is_nan(), "Velocity produced NaN!");
        }
    }

    #[test]
    fn test_spring2d_motion() {
        let mut spring2d = Spring2D::with_preset(10.0, 20.0, SpringPreset::FluidPanel);
        spring2d.set_target(50.0, 100.0);

        let dt = 1.0 / 60.0;
        for _ in 0..180 {
            spring2d.update(dt);
        }

        assert!(spring2d.is_at_rest());
        let (x, y) = spring2d.position();
        assert!((x - 50.0).abs() < 0.001);
        assert!((y - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_spring_color_interpolation() {
        let mut color = SpringColor::with_preset([0.0, 0.0, 0.0, 1.0], SpringPreset::DeepFocus);
        color.set_target([1.0, 0.5, 0.25, 0.8]);

        let dt = 1.0 / 60.0;
        for _ in 0..240 {
            color.update(dt);
        }

        assert!(color.is_at_rest());
        let res = color.color();
        assert!((res[0] - 1.0).abs() < 0.001);
        assert!((res[1] - 0.5).abs() < 0.001);
        assert!((res[2] - 0.25).abs() < 0.001);
        assert!((res[3] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_snap_teleport() {
        let mut spring = Spring1D::with_preset(0.0, SpringPreset::SnappyMicro);
        spring.set_target(50.0);
        spring.update(0.016);
        assert!(spring.velocity > 0.0);

        spring.snap(200.0);
        assert_eq!(spring.position, 200.0);
        assert_eq!(spring.target, 200.0);
        assert_eq!(spring.velocity, 0.0);
        assert!(spring.is_at_rest());
    }
}
