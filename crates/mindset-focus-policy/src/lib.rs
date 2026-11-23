//! MindSet Focus Policy Engine — Distraction Rules, Timetable Evaluation & Lockdown Policies
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolicyEngineError {
    #[error("Failed to parse timetable rule: {0}")]
    ParseError(String),
    #[error("Conflicting focus profile constraints: {0}")]
    ConstraintConflict(String),
    #[error("Process inspection error: {0}")]
    InspectionError(String),
}

/// Categorization of external distractions subject to policy rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistractionCategory {
    SocialMedia,
    Gaming,
    VideoStreaming,
    InstantMessaging,
    NewsAndFeeds,
    CustomDomain,
}

/// Focus Mode operational profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusProfileMode {
    DeepWork,
    ExamPrep,
    Coding,
    Reading,
    Revision,
    CreativeFlow,
}

/// Enforcement action when a distraction rule is triggered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAction {
    WarnUser,
    MinimizeWindow,
    KillProcess,
    BlockDns,
    LockScreen,
}

/// Distraction filter rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistractionRule {
    pub id: String,
    pub category: DistractionCategory,
    pub match_pattern: String, // Process name or domain regex
    pub is_domain: bool,
    pub action: EnforcementAction,
    pub is_enabled: bool,
}

/// Time-windowed policy schedule entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleWindow {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub days_of_week: Vec<u8>, // 1 = Monday, 7 = Sunday
    pub profile_mode: FocusProfileMode,
    pub strict_lockdown: bool,
}

/// Comprehensive policy configuration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfiguration {
    pub active_mode: Option<FocusProfileMode>,
    pub strict_mode_active: bool,
    pub rules: Vec<DistractionRule>,
    pub schedule: Vec<ScheduleWindow>,
    pub break_interval_minutes: u32,
    pub max_session_minutes: u32,
}

impl Default for PolicyConfiguration {
    fn default() -> Self {
        Self {
            active_mode: None,
            strict_mode_active: false,
            rules: vec![
                DistractionRule {
                    id: "rule_social_yt".into(),
                    category: DistractionCategory::VideoStreaming,
                    match_pattern: "youtube.com".into(),
                    is_domain: true,
                    action: EnforcementAction::BlockDns,
                    is_enabled: true,
                },
                DistractionRule {
                    id: "rule_social_insta".into(),
                    category: DistractionCategory::SocialMedia,
                    match_pattern: "instagram.com".into(),
                    is_domain: true,
                    action: EnforcementAction::BlockDns,
                    is_enabled: true,
                },
                DistractionRule {
                    id: "rule_game_steam".into(),
                    category: DistractionCategory::Gaming,
                    match_pattern: "steam.exe".into(),
                    is_domain: false,
                    action: EnforcementAction::MinimizeWindow,
                    is_enabled: true,
                },
            ],
            schedule: Vec::new(),
            break_interval_minutes: 25,
            max_session_minutes: 90,
        }
    }
}

/// Distraction infraction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfractionEvent {
    pub timestamp_utc: DateTime<Utc>,
    pub rule_id: String,
    pub category: DistractionCategory,
    pub target_identifier: String,
    pub action_taken: EnforcementAction,
}

/// Policy evaluation engine
pub struct PolicyEvaluator {
    config: PolicyConfiguration,
}

impl PolicyEvaluator {
    pub fn new(config: PolicyConfiguration) -> Self {
        Self { config }
    }

    pub fn check_process(&self, process_name: &str) -> Option<&DistractionRule> {
        let lower = process_name.to_lowercase();
        self.config.rules.iter().find(|rule| {
            rule.is_enabled && !rule.is_domain && lower.contains(&rule.match_pattern.to_lowercase())
        })
    }

    pub fn check_domain(&self, domain: &str) -> Option<&DistractionRule> {
        let lower = domain.to_lowercase();
        self.config.rules.iter().find(|rule| {
            rule.is_enabled && rule.is_domain && lower.contains(&rule.match_pattern.to_lowercase())
        })
    }

    pub fn is_strict_mode(&self) -> bool {
        self.config.strict_mode_active
    }
}
