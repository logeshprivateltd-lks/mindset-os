//! MindSet OS — Focus & Pomodoro Engine (`pomodoro.rs`)
//! Pure Rust single-source-of-truth pomodoro timing, presets, and cycle state.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

pub const POMODORO_PRESETS: [u32; 4] = [25, 30, 45, 60];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PomodoroEngine {
    pub duration_mins: u32,
    pub break_mins: u32,
    pub remaining_secs: f32,
    pub is_running: bool,
    pub completed_sessions: u32,
    pub cycle_step: u8, // 1..=4
    pub is_break: bool,
}

impl Default for PomodoroEngine {
    fn default() -> Self {
        Self::new(25)
    }
}

impl PomodoroEngine {
    pub fn new(duration_mins: u32) -> Self {
        let valid_duration = if POMODORO_PRESETS.contains(&duration_mins) {
            duration_mins
        } else {
            25
        };
        Self {
            duration_mins: valid_duration,
            break_mins: 5,
            remaining_secs: (valid_duration * 60) as f32,
            is_running: false,
            completed_sessions: 0,
            cycle_step: 1,
            is_break: false,
        }
    }

    /// Cycle to the next duration preset (25 -> 30 -> 45 -> 60 -> 25).
    pub fn cycle_duration(&mut self) {
        let next = match self.duration_mins {
            25 => 30,
            30 => 45,
            45 => 60,
            _ => 25,
        };
        self.set_duration(next);
    }

    /// Set duration explicitly and reset timer if not running.
    pub fn set_duration(&mut self, mins: u32) {
        self.duration_mins = mins;
        if !self.is_running {
            self.remaining_secs = (mins * 60) as f32;
        }
    }

    /// Toggle play / pause state.
    pub fn toggle(&mut self) -> bool {
        self.is_running = !self.is_running;
        self.is_running
    }

    /// Start 25-min focus session explicitly (for [▶ Start] or [START FOCUS] CTA).
    pub fn start_focus_session(&mut self) {
        self.set_duration(25);
        self.remaining_secs = 25.0 * 60.0;
        self.is_running = true;
        self.is_break = false;
    }

    /// Advance cycle counter: 1/4 -> 2/4 -> 3/4 -> 4/4 -> resets to 1/4.
    pub fn advance_cycle(&mut self) {
        if self.cycle_step >= 4 {
            self.cycle_step = 1;
        } else {
            self.cycle_step += 1;
        }
    }

    /// Tick down by elapsed seconds. Returns true if session completed on this tick.
    pub fn tick(&mut self, dt_secs: f32) -> bool {
        if !self.is_running {
            return false;
        }

        self.remaining_secs -= dt_secs;
        if self.remaining_secs <= 0.0 {
            self.remaining_secs = 0.0;
            self.is_running = false;
            self.completed_sessions += 1;
            self.advance_cycle();
            true
        } else {
            false
        }
    }

    /// Progress ratio [0.0..=1.0] for circular/linear meters.
    pub fn progress(&self) -> f32 {
        let total = (self.duration_mins * 60) as f32;
        if total <= 0.0 {
            0.0
        } else {
            1.0 - (self.remaining_secs / total).clamp(0.0, 1.0)
        }
    }

    /// Formatted MM:SS display.
    pub fn format_time(&self) -> String {
        let total_secs = self.remaining_secs.max(0.0).ceil() as u32;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    /// Cycle text indicator (e.g. "1/4", "2/4", "3/4", "4/4").
    pub fn cycle_str(&self) -> String {
        format!("{}/4", self.cycle_step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pomodoro_duration_cycling() {
        let mut pomo = PomodoroEngine::new(25);
        assert_eq!(pomo.duration_mins, 25);
        pomo.cycle_duration();
        assert_eq!(pomo.duration_mins, 30);
        pomo.cycle_duration();
        assert_eq!(pomo.duration_mins, 45);
        pomo.cycle_duration();
        assert_eq!(pomo.duration_mins, 60);
        pomo.cycle_duration();
        assert_eq!(pomo.duration_mins, 25);
    }

    #[test]
    fn test_pomodoro_cycle_advance_and_reset() {
        let mut pomo = PomodoroEngine::new(25);
        assert_eq!(pomo.cycle_str(), "1/4");
        pomo.advance_cycle();
        assert_eq!(pomo.cycle_str(), "2/4");
        pomo.advance_cycle();
        assert_eq!(pomo.cycle_str(), "3/4");
        pomo.advance_cycle();
        assert_eq!(pomo.cycle_str(), "4/4");
        pomo.advance_cycle();
        assert_eq!(pomo.cycle_str(), "1/4");
    }

    #[test]
    fn test_pomodoro_tick_and_completion() {
        let mut pomo = PomodoroEngine::new(25);
        pomo.start_focus_session();
        assert!(pomo.is_running);
        assert_eq!(pomo.format_time(), "25:00");

        // Tick 60s
        let done = pomo.tick(60.0);
        assert!(!done);
        assert_eq!(pomo.format_time(), "24:00");

        // Fast forward remainder
        let completed = pomo.tick(24.0 * 60.0 + 1.0);
        assert!(completed);
        assert!(!pomo.is_running);
        assert_eq!(pomo.completed_sessions, 1);
        assert_eq!(pomo.cycle_str(), "2/4");
    }
}
