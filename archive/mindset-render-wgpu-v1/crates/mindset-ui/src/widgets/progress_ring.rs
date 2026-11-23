//! Progress Ring Component
//! Anti-aliased SDF progress arc with FluidPanel spring animation.

use mindset_motion::{Spring1D, SpringPreset};

pub struct ProgressRing {
    pub progress: Spring1D,
    pub label: String,
    pub value_text: String,
}

impl ProgressRing {
    pub fn new(label: impl Into<String>, value_text: impl Into<String>) -> Self {
        Self {
            progress: Spring1D::with_preset(0.0, SpringPreset::FluidPanel),
            label: label.into(),
            value_text: value_text.into(),
        }
    }

    pub fn set_progress(&mut self, frac: f32, reduced_motion: bool) {
        let clamped = frac.clamp(0.0, 1.0);
        if reduced_motion {
            self.progress.snap(clamped);
        } else {
            self.progress.set_target(clamped);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress.update(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_spring_converges() {
        let mut ring = ProgressRing::new("Today", "2h/4h");
        ring.set_progress(0.58, false);

        // Simulate 0.5s at 60Hz
        for _ in 0..30 {
            ring.update(1.0 / 60.0);
        }

        assert!(!ring.progress.position.is_nan(), "Position must not be NaN");
        assert!(
            (ring.progress.position - 0.58).abs() < 0.05,
            "Ring spring must converge to 0.58 within 0.5s"
        );

        // With reduced motion, snaps immediately
        ring.set_progress(0.90, true);
        assert_eq!(
            ring.progress.position, 0.90,
            "Reduced motion must snap instantly"
        );
    }
}
