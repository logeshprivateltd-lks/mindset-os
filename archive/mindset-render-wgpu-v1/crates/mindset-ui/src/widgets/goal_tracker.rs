//! MindSet OS — Declarative Goal & Neuroscience Productivity Widgets (`goal_tracker.rs`)
//! High-engagement, dopamine-reinforcing UI primitives for competitive exams & mastery targets.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

/// Timer mode selection conforming to student preferences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FocusTimerMode {
    #[default]
    Ultradian90, // 90m deep focus + 20m consolidation
    Pomodoro25,  // 25m focus + 5m break
    PowerHour60, // 60m high-yield drill
}

impl FocusTimerMode {
    pub fn duration_secs(&self) -> f32 {
        match self {
            Self::Ultradian90 => 90.0 * 60.0,
            Self::Pomodoro25 => 25.0 * 60.0,
            Self::PowerHour60 => 60.0 * 60.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Ultradian90 => "90m Ultradian",
            Self::Pomodoro25 => "25m Pomodoro",
            Self::PowerHour60 => "60m Hour Drill",
        }
    }
}

/// Goal priority classification with Maindy AI weightage allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GoalPriority {
    #[default]
    Primary, // Receives 90% Maindy AI cognition & daily rail focus
    Secondary, // Receives 20% passive background tracking
}

/// Visual status of an academic chapter in the reverse breakdown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ChapterYieldStatus {
    #[default]
    NextUp, // Queued for today / tomorrow
    InProgress,    // Concepts reviewed, practice test pending
    Locked,        // Marks secured, verified in mock tests (Dopamine lock)
    StrategicSkip, // Low ROI (e.g. < 1.0 mark/hr), safely skipped if time-crunched
}

impl ChapterYieldStatus {
    pub fn badge_label(&self) -> &'static str {
        match self {
            Self::Locked => "🔒 Locked",
            Self::InProgress => "⏳ Active",
            Self::NextUp => "🎯 Next Up",
            Self::StrategicSkip => "⚡ Skip",
        }
    }
}

/// A chapter mapped to its exam mark weightage and estimated hours
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterYieldItem {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub weightage_marks: u32,
    pub estimated_hours: f32,
    pub status: ChapterYieldStatus,
    pub progress: f32,
}

impl ChapterYieldItem {
    pub fn new(
        id: &str,
        title: &str,
        subject: &str,
        marks: u32,
        hours: f32,
        status: ChapterYieldStatus,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            subject: subject.to_string(),
            weightage_marks: marks,
            estimated_hours: hours,
            status,
            progress: if status == ChapterYieldStatus::Locked {
                1.0
            } else {
                0.0
            },
        }
    }

    /// Return On Investment (Marks earned per study hour)
    pub fn roi_marks_per_hour(&self) -> f32 {
        if self.estimated_hours <= 0.0 {
            0.0
        } else {
            self.weightage_marks as f32 / self.estimated_hours
        }
    }
}

/// Subject quota allocation in total exam score
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectQuotaProgress {
    pub name: String,
    pub target_marks: u32,
    pub max_marks: u32,
    pub locked_marks: u32,
    pub in_progress_marks: u32,
}

impl SubjectQuotaProgress {
    pub fn progress_ratio(&self) -> f32 {
        if self.target_marks == 0 {
            0.0
        } else {
            (self.locked_marks as f32 / self.target_marks as f32).clamp(0.0, 1.0)
        }
    }
}

/// Complete reverse-engineered exam goal blueprint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcademicExamGoal {
    pub title: String,
    pub dream_role: String,
    pub target_date_str: String,
    pub days_remaining: u32,
    pub target_score: u32,
    pub max_score: u32,
    pub priority: GoalPriority,
    pub subjects: Vec<SubjectQuotaProgress>,
    pub chapters: Vec<ChapterYieldItem>,
    pub today_mission_idx: usize,
    pub timer_mode: FocusTimerMode,
    pub soundscape_active: bool,
}

impl AcademicExamGoal {
    /// Out-of-the-box flagship goal for aspiring doctors: NEET 2027
    pub fn neet_2027_doctor_preset() -> Self {
        Self {
            title: "NEET 2027 (MBBS Medical Entrance)".to_string(),
            dream_role: "Doctor (Govt Medical College)".to_string(),
            target_date_str: "Sunday, April 5, 2027".to_string(),
            days_remaining: 504,
            target_score: 680,
            max_score: 720,
            priority: GoalPriority::Primary,
            subjects: vec![
                SubjectQuotaProgress {
                    name: "Biology".to_string(),
                    target_marks: 340,
                    max_marks: 360,
                    locked_marks: 320,
                    in_progress_marks: 15,
                },
                SubjectQuotaProgress {
                    name: "Chemistry".to_string(),
                    target_marks: 160,
                    max_marks: 180,
                    locked_marks: 142,
                    in_progress_marks: 8,
                },
                SubjectQuotaProgress {
                    name: "Physics".to_string(),
                    target_marks: 150,
                    max_marks: 180,
                    locked_marks: 138,
                    in_progress_marks: 12,
                },
            ],
            chapters: vec![
                ChapterYieldItem::new(
                    "ph_01",
                    "Modern Physics: Photoelectric & Bohr",
                    "Physics",
                    16,
                    4.0,
                    ChapterYieldStatus::NextUp,
                ),
                ChapterYieldItem::new(
                    "ph_02",
                    "Ray Optics & Optical Instruments",
                    "Physics",
                    12,
                    4.0,
                    ChapterYieldStatus::InProgress,
                ),
                ChapterYieldItem::new(
                    "ch_01",
                    "Chemical Bonding & Molecular Structure",
                    "Chemistry",
                    20,
                    5.0,
                    ChapterYieldStatus::Locked,
                ),
                ChapterYieldItem::new(
                    "ch_02",
                    "Coordination Compounds",
                    "Chemistry",
                    12,
                    3.5,
                    ChapterYieldStatus::InProgress,
                ),
                ChapterYieldItem::new(
                    "bi_01",
                    "Human Physiology: Neural Control",
                    "Biology",
                    24,
                    6.0,
                    ChapterYieldStatus::Locked,
                ),
                ChapterYieldItem::new(
                    "bi_02",
                    "Genetics & Molecular Basis",
                    "Biology",
                    32,
                    8.0,
                    ChapterYieldStatus::Locked,
                ),
                ChapterYieldItem::new(
                    "ch_03",
                    "Surface Chemistry & Metallurgy",
                    "Chemistry",
                    4,
                    6.0,
                    ChapterYieldStatus::StrategicSkip,
                ),
            ],
            today_mission_idx: 0,
            timer_mode: FocusTimerMode::Ultradian90,
            soundscape_active: true,
        }
    }

    /// Secondary Goal Preset: CBSE Class 12 Board Exam 500/500
    pub fn board_exam_preset(priority: GoalPriority) -> Self {
        Self {
            title: "CBSE Class 12 Science Board Exam".to_string(),
            dream_role: "Top 0.1% Merit Certificate (500/500)".to_string(),
            target_date_str: "March 1, 2027".to_string(),
            days_remaining: 469,
            target_score: 490,
            max_score: 500,
            priority,
            subjects: vec![
                SubjectQuotaProgress {
                    name: "Physics".to_string(),
                    target_marks: 98,
                    max_marks: 100,
                    locked_marks: 90,
                    in_progress_marks: 5,
                },
                SubjectQuotaProgress {
                    name: "Chemistry".to_string(),
                    target_marks: 98,
                    max_marks: 100,
                    locked_marks: 92,
                    in_progress_marks: 4,
                },
                SubjectQuotaProgress {
                    name: "Biology".to_string(),
                    target_marks: 99,
                    max_marks: 100,
                    locked_marks: 95,
                    in_progress_marks: 3,
                },
            ],
            chapters: vec![
                ChapterYieldItem::new(
                    "bd_01",
                    "Electromagnetic Induction & AC",
                    "Physics",
                    8,
                    2.5,
                    ChapterYieldStatus::Locked,
                ),
                ChapterYieldItem::new(
                    "bd_02",
                    "Electrochemistry & Nernst Equation",
                    "Chemistry",
                    9,
                    3.0,
                    ChapterYieldStatus::Locked,
                ),
                ChapterYieldItem::new(
                    "bd_03",
                    "Wave Optics & Interference",
                    "Physics",
                    7,
                    2.0,
                    ChapterYieldStatus::NextUp,
                ),
            ],
            today_mission_idx: 2,
            timer_mode: FocusTimerMode::Pomodoro25,
            soundscape_active: false,
        }
    }

    /// Calculate total secured marks across all subjects
    pub fn total_locked_marks(&self) -> u32 {
        self.subjects.iter().map(|s| s.locked_marks).sum()
    }

    /// Calculate total pending in-progress marks
    pub fn total_in_progress_marks(&self) -> u32 {
        self.subjects.iter().map(|s| s.in_progress_marks).sum()
    }

    /// Calculate projected total score if current in-progress chapters are locked
    pub fn projected_score(&self) -> u32 {
        (self.total_locked_marks() + self.total_in_progress_marks()).min(self.max_score)
    }

    /// Returns the currently active mission chapter
    pub fn today_mission(&self) -> Option<&ChapterYieldItem> {
        self.chapters.get(self.today_mission_idx)
    }

    /// Locks in marks for the current mission (Dopamine prediction error reward)
    pub fn lock_today_mission(&mut self) -> u32 {
        if let Some(ch) = self.chapters.get_mut(self.today_mission_idx) {
            ch.status = ChapterYieldStatus::Locked;
            ch.progress = 1.0;
            let earned = ch.weightage_marks;
            // Attribute to matching subject
            if let Some(subj) = self
                .subjects
                .iter_mut()
                .find(|s| s.name.eq_ignore_ascii_case(&ch.subject))
            {
                subj.locked_marks = (subj.locked_marks + earned).min(subj.max_marks);
                if subj.in_progress_marks >= earned {
                    subj.in_progress_marks -= earned;
                } else {
                    subj.in_progress_marks = 0;
                }
            }
            earned
        } else {
            0
        }
    }
}

impl Default for AcademicExamGoal {
    fn default() -> Self {
        Self::neet_2027_doctor_preset()
    }
}
