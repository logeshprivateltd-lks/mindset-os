use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Configuration options for running the whole inspector plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorConfig {
    /// Workspace root directory to inspect.
    pub workspace_root: PathBuf,
    /// Whether to run live `cargo check`, `cargo test`, `cargo clippy` if cargo is available.
    pub run_cargo: bool,
    /// Optional file path to output the markdown report.
    pub output_md: Option<PathBuf>,
    /// Optional file path to output the JSON report.
    pub output_json: Option<PathBuf>,
    /// Verbose console logging during inspection.
    pub verbose: bool,
}

impl Default for InspectorConfig {
    fn default() -> Self {
        Self {
            workspace_root: PathBuf::from("."),
            run_cargo: false,
            output_md: None,
            output_json: None,
            verbose: false,
        }
    }
}

/// Status of an individual audited capability or feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityStatus {
    Pass,
    Partial,
    Mock,
    Missing,
}

impl CapabilityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Partial => "PARTIAL",
            Self::Mock => "MOCK",
            Self::Missing => "MISSING",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::Pass => "✅ PASS",
            Self::Partial => "⚠️ PARTIAL",
            Self::Mock => "🎭 MOCK",
            Self::Missing => "❌ MISSING",
        }
    }
}

/// Entry in the master Capability Matrix Table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMatrixEntry {
    pub feature: String,
    pub category: String, // "Normal OS", "ISO", "Student OS", "AI", "UI"
    pub status: CapabilityStatus,
    pub evidence: String, // Exact file:line reference or concise proof
    pub notes: String,
}

/// Severity classification for remediation tasks in the prioritized action queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionSeverity {
    Critical, // Must fix before Debian 13 ISO / live boot
    High,     // Must fix before production demo
    Medium,   // Architectural hygiene or UX optimization
    Low,      // Minor polish or documentation detail
}

impl ActionSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL (Must Fix Before ISO)",
            Self::High => "HIGH (Fix Before Demo)",
            Self::Medium => "MEDIUM (Architectural Hygiene)",
            Self::Low => "LOW (Cosmetic Polish)",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::Critical => "🚨 CRITICAL",
            Self::High => "⚠️ HIGH",
            Self::Medium => "ℹ️ MEDIUM",
            Self::Low => "💡 LOW",
        }
    }
}

/// Single actionable remediation item with exact location coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizedAction {
    pub severity: ActionSeverity,
    pub title: String,
    pub location: String, // Exact file:line reference
    pub finding: String,
    pub remediation: String,
}

/// Diagnostic results and metrics from a single inspection module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleAuditResult {
    pub module_id: String,
    pub module_name: String,
    pub score: u32, // 0..=100
    pub summary: String,
    pub entries: Vec<CapabilityMatrixEntry>,
    pub actions: Vec<PrioritizedAction>,
    pub metrics: HashMap<String, String>,
}

/// High-level executive scoring and readiness summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    /// Score 0..=100: Can MindSet OS function as a normal daily desktop OS?
    pub normal_os_score: u32,
    /// Score 0..=100: Can the project build and boot clean Debian 13 ISO on PC & VM?
    pub iso_readiness_score: u32,
    /// Score 0..=100: Are all student features, UI polish, and permission-gated Maindy AI complete?
    pub student_os_ai_score: u32,
    /// Aggregated overall system readiness score (0..=100).
    pub overall_score: u32,
    /// High-level verdict string ("DEMO / OS READY", "NON-CRITICAL WARNINGS", "CRITICAL BLOCKERS FOUND").
    pub verdict_status: String,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub total_features_checked: usize,
    pub pass_count: usize,
    pub partial_count: usize,
    pub mock_count: usize,
    pub missing_count: usize,
}

/// Complete diagnostic and architectural verification report for MindSet OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WholeInspectionReport {
    pub timestamp: String,
    pub workspace_root: String,
    pub executive_summary: ExecutiveSummary,
    pub module_results: Vec<ModuleAuditResult>,
    pub capability_matrix: Vec<CapabilityMatrixEntry>,
    pub action_queue: Vec<PrioritizedAction>,
    /// Suggested process exit code:
    /// - 0: Demo / OS Ready (no critical blockers, all scores >= 80)
    /// - 1: Non-critical warnings (no critical blockers, scores >= 60)
    /// - 2: Critical blockers found
    pub exit_code: i32,
}
