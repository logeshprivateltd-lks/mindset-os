//! Completion Micro-Celebration Particle Engine
//! Emits 40 radial particles with drag and alpha fading on task completion.

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub color: [f32; 4],
    pub size: f32,
    pub life: f32,
}

#[derive(Debug, Clone, Default)]
pub struct CelebrationBurst {
    pub origin: [f32; 2],
    pub particles: Vec<Particle>,
    pub elapsed: f32,
    pub active: bool,
}

impl CelebrationBurst {
    pub fn new() -> Self {
        Self {
            origin: [0.0, 0.0],
            particles: Vec::new(),
            elapsed: 0.0,
            active: false,
        }
    }

    pub fn trigger(&mut self, origin: [f32; 2], accent_color: [f32; 4]) {
        self.origin = origin;
        self.elapsed = 0.0;
        self.active = true;
        self.particles.clear();

        for i in 0..40 {
            let angle = (i as f32 / 40.0) * std::f32::consts::TAU + (i as f32 * 0.17).sin() * 0.2;
            let speed = 80.0 + ((i * 17) % 140) as f32;
            let vel = [angle.cos() * speed, angle.sin() * speed];
            let size = 3.0 + ((i * 7) % 4) as f32;
            let col = if i % 3 == 0 {
                [1.0, 1.0, 1.0, 1.0]
            } else if i % 2 == 0 {
                [accent_color[0], accent_color[1], accent_color[2], 1.0]
            } else {
                [0.98, 0.75, 0.14, 1.0] // Warm Solar Amber
            };

            self.particles.push(Particle {
                pos: origin,
                vel,
                color: col,
                size,
                life: 0.8,
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }
        self.elapsed += dt;
        let mut any_alive = false;

        for p in &mut self.particles {
            if p.life > 0.0 {
                p.pos[0] += p.vel[0] * dt;
                p.pos[1] += p.vel[1] * dt;
                p.vel[0] *= 0.96;
                p.vel[1] *= 0.96;
                p.life = (p.life - dt).max(0.0);
                if p.life > 0.0 {
                    any_alive = true;
                }
            }
        }

        self.active = any_alive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celebration_burst_lifecycle() {
        let mut burst = CelebrationBurst::new();
        assert!(!burst.active);

        burst.trigger([200.0, 200.0], [0.9, 0.6, 0.2, 1.0]);
        assert_eq!(burst.particles.len(), 40);
        assert!(burst.active);

        // Step 0.4s
        burst.update(0.4);
        assert!(burst.active);

        // Step past 0.9s total
        burst.update(0.6);
        assert!(!burst.active, "All 40 particles must be dead after 0.9s");
        assert!(burst.particles.iter().all(|p| p.life <= 0.0));
    }
}
