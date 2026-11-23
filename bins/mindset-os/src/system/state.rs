//! Shared System State, Local Weather, Persistent Scratchpad & Study Metrics

use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OllamaStatus {
    #[default]
    Idle,
    Online,
    Loading,
    Offline,
}

impl OllamaStatus {
    pub fn badge_text(&self) -> &'static str {
        match self {
            OllamaStatus::Online => "🟢 LLM Online",
            OllamaStatus::Idle => "🟡 LLM Idle",
            OllamaStatus::Loading => "🔄 LLM Loading",
            OllamaStatus::Offline => "⚪ LLM Offline",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherInfo {
    pub location: String,
    pub temperature: String,
    pub condition: String,
    pub high_low: String,
}

impl Default for WeatherInfo {
    fn default() -> Self {
        Self {
            location: "Madurai, TN".to_string(),
            temperature: "31°C".to_string(),
            condition: "Clear Skies • Deep Focus".to_string(),
            high_low: "H: 34°  L: 24°".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedCapsule {
    pub title: &'static str,
    pub subject: &'static str,
    pub time_ago: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Clone)]
pub struct SystemState {
    pub weather: WeatherInfo,
    pub scratchpad_text: String,
    pub scratchpad_path: PathBuf,
    pub ollama_status: OllamaStatus,
    pub study_screen_time_mins: u32,
    pub study_goal_mins: u32,
    pub recommended_capsules: Vec<RecommendedCapsule>,
    pub scratchpad_save_status: &'static str,
    pub scratchpad_revision: u64,
}

impl Default for SystemState {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemState {
    pub fn new() -> Self {
        Self::new_with_scratchpad(Self::default_scratchpad())
    }

    pub fn new_with_scratchpad(scratchpad_text: String) -> Self {
        let scratchpad_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("mindset_os_scratchpad.txt");

        Self {
            weather: WeatherInfo::default(),
            scratchpad_text,
            scratchpad_path,
            ollama_status: OllamaStatus::Idle,
            study_screen_time_mins: 142,
            study_goal_mins: 180,
            recommended_capsules: vec![
                RecommendedCapsule {
                    title: "Gauss_Law_Derivations.notes",
                    subject: "Physics • Inverse Square Law",
                    time_ago: "Today",
                    icon: "📄",
                },
                RecommendedCapsule {
                    title: "Electrochem_Nernst_Formula.capsule",
                    subject: "Chemistry • Equilibrium Potential",
                    time_ago: "Yesterday",
                    icon: "📝",
                },
                RecommendedCapsule {
                    title: "Matrix_Eigenvalues_Revision.md",
                    subject: "Mathematics • Linear Algebra",
                    time_ago: "2 Days Ago",
                    icon: "📊",
                },
                RecommendedCapsule {
                    title: "DNA_Transcription_Notes.md",
                    subject: "Biology • Molecular Genetics",
                    time_ago: "3 Days Ago",
                    icon: "🧬",
                },
            ],
            scratchpad_save_status: "Auto-saved to SQLite",
            scratchpad_revision: 0,
        }
    }

    pub fn default_scratchpad() -> String {
        "// MindSet OS Quick Scratchpad (Auto-saved to SQLite)\n\
         • Gauss's Law: Φ = ∮ E·dA = q_enc / ε₀\n\
         • Nernst Eq: E = E° - (RT/nF) ln Q\n\
         • Target: 590/600 | Syllabus Sprint Active"
            .to_string()
    }

    pub fn set_scratchpad(&mut self, text: String) {
        self.scratchpad_text = text;
        self.scratchpad_revision = self.scratchpad_revision.wrapping_add(1);
        self.scratchpad_save_status = "Saving...";
    }

    /// Dynamic Greeting based on 24-hour clock:
    /// - 05:00 - 11:59 -> "Good Morning"
    /// - 12:00 - 16:59 -> "Good Afternoon"
    /// - 17:00 - 21:59 -> "Good Evening"
    /// - 22:00 - 04:59 -> "Night Focus"
    pub fn current_greeting() -> &'static str {
        let hour = Local::now().hour();
        match hour {
            5..=11 => "Good Morning",
            12..=16 => "Good Afternoon",
            17..=21 => "Good Evening",
            _ => "Night Focus",
        }
    }

    /// Formatted current time string "%I:%M %p"
    pub fn formatted_clock() -> String {
        Local::now().format("%I:%M %p").to_string()
    }

    /// Formatted current date string "%A, %B %e"
    pub fn formatted_date() -> String {
        Local::now().format("%A, %B %e").to_string()
    }
}
