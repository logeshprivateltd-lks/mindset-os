//! MindSet OS — Today Screen & Identity State Model (`today.rs`)
//! Pure Rust representation of neuroscience-designed root distraction-free view.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSnapshot {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub subtopic: String,
    pub duration_minutes: u32,
    pub why_line: String,
    pub marks_delta: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RingProgress {
    pub label: String,
    pub value_text: String,
    pub current_hours: f32,
    pub target_hours: f32,
    pub fraction: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FutureSelfSnapshot {
    pub target_identity_label: String, // e.g. "Dr. Logesh Karthik, MBBS 2027"
    pub quote: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreakSnapshot {
    pub count: u32,
    pub freezes_remaining: u32,
    pub freeze_used: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodayState {
    pub one_thing: Option<TaskSnapshot>,
    pub rings: [RingProgress; 3], // Today, Week, Syllabus
    pub future_self: FutureSelfSnapshot,
    pub streak: StreakSnapshot,
    pub tomorrow_preview: Vec<String>,
}

impl Default for TodayState {
    fn default() -> Self {
        Self::new()
    }
}

impl TodayState {
    pub fn new() -> Self {
        let ring_today = RingProgress {
            label: "Today".to_string(),
            value_text: "2h / 4h".to_string(),
            current_hours: 2.0,
            target_hours: 4.0,
            fraction: 0.50,
        };
        let ring_week = RingProgress {
            label: "Week".to_string(),
            value_text: "8h / 20h".to_string(),
            current_hours: 8.0,
            target_hours: 20.0,
            fraction: 0.40,
        };
        let ring_syllabus = RingProgress {
            label: "Syllabus".to_string(),
            value_text: "34%".to_string(),
            current_hours: 34.0,
            target_hours: 100.0,
            fraction: 0.34,
        };

        let one_thing = Some(TaskSnapshot {
            id: "task_nernst_01".to_string(),
            title: "Electrochemistry — Nernst Equation Derivation".to_string(),
            topic: "Physical Chemistry".to_string(),
            subtopic: "Equilibrium Potentials".to_string(),
            duration_minutes: 25,
            why_line: "Guarantees 5 marks in Section C & anchors high-yield exam retention."
                .to_string(),
            marks_delta: Some(5.0),
        });

        Self {
            one_thing,
            rings: [ring_today, ring_week, ring_syllabus],
            future_self: FutureSelfSnapshot {
                target_identity_label: "Dr. Logesh Karthik, MBBS 2027".to_string(),
                quote: "Today's 25 min = 1 step closer to your white coat.".to_string(),
            },
            streak: StreakSnapshot {
                count: 14,
                freezes_remaining: 2,
                freeze_used: false,
            },
            tomorrow_preview: vec![
                "Optics: Wavefront & Huygens Principle (45m)".to_string(),
                "Inorganic: Coordination Compounds (30m)".to_string(),
            ],
        }
    }

    /// Add completed study minutes to today's ring and recalculate fractions.
    pub fn log_study_minutes(&mut self, minutes: u32) {
        let hrs_delta = minutes as f32 / 60.0;
        self.rings[0].current_hours += hrs_delta;
        self.rings[0].fraction =
            (self.rings[0].current_hours / self.rings[0].target_hours).clamp(0.0, 1.0);
        self.rings[0].value_text = format!(
            "{:.1}h / {:.0}h",
            self.rings[0].current_hours, self.rings[0].target_hours
        );

        self.rings[1].current_hours += hrs_delta;
        self.rings[1].fraction =
            (self.rings[1].current_hours / self.rings[1].target_hours).clamp(0.0, 1.0);
        self.rings[1].value_text = format!(
            "{:.1}h / {:.0}h",
            self.rings[1].current_hours, self.rings[1].target_hours
        );

        // Small bump to syllabus
        self.rings[2].current_hours = (self.rings[2].current_hours + 0.25).min(100.0);
        self.rings[2].fraction = (self.rings[2].current_hours / 100.0).clamp(0.0, 1.0);
        self.rings[2].value_text = format!("{:.0}%", self.rings[2].current_hours);
    }

    /// Contextual greeting string based on current local time.
    pub fn greeting_str(&self) -> String {
        let hour = Local::now().hour();
        if hour < 12 {
            "Good Morning, Scholar".to_string()
        } else if hour < 17 {
            "Good Afternoon, Scholar".to_string()
        } else {
            "Good Evening, Scholar".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today_initial_state() {
        let today = TodayState::new();
        assert!(today.one_thing.is_some());
        assert_eq!(today.rings.len(), 3);
        assert_eq!(today.streak.count, 14);
        assert_eq!(
            today.future_self.target_identity_label,
            "Dr. Logesh Karthik, MBBS 2027"
        );
    }

    #[test]
    fn test_today_log_minutes_progress() {
        let mut today = TodayState::new();
        let prev_today_hrs = today.rings[0].current_hours;
        today.log_study_minutes(60); // 1 hour
        assert!((today.rings[0].current_hours - (prev_today_hrs + 1.0)).abs() < 0.01);
    }
}
