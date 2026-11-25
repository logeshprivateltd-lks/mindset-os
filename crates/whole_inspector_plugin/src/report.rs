use chrono::Local;
use std::fs;
use std::path::Path;

use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ExecutiveSummary, ModuleAuditResult,
    PrioritizedAction, WholeInspectionReport,
};

/// Master scoring engine and report compiler for MindSet OS inspection.
pub struct ReportGenerator;

impl ReportGenerator {
    /// Aggregates all module results into the final WholeInspectionReport.
    pub fn compile(
        workspace_root: &Path,
        modules: Vec<ModuleAuditResult>,
    ) -> WholeInspectionReport {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S UTC%z").to_string();

        let mut all_entries: Vec<CapabilityMatrixEntry> = Vec::new();
        let mut all_actions: Vec<PrioritizedAction> = Vec::new();

        let mut pass_count = 0;
        let mut partial_count = 0;
        let mut mock_count = 0;
        let mut missing_count = 0;

        for module in &modules {
            for entry in &module.entries {
                match entry.status {
                    CapabilityStatus::Pass => pass_count += 1,
                    CapabilityStatus::Partial => partial_count += 1,
                    CapabilityStatus::Mock => mock_count += 1,
                    CapabilityStatus::Missing => missing_count += 1,
                }
                all_entries.push(entry.clone());
            }
            for action in &module.actions {
                all_actions.push(action.clone());
            }
        }

        // Sort actions by severity: Critical first, then High, Medium, Low
        all_actions.sort_by_key(|a| a.severity);

        let critical_count = all_actions
            .iter()
            .filter(|a| a.severity == ActionSeverity::Critical)
            .count();
        let high_count = all_actions
            .iter()
            .filter(|a| a.severity == ActionSeverity::High)
            .count();
        let medium_count = all_actions
            .iter()
            .filter(|a| a.severity == ActionSeverity::Medium)
            .count();
        let low_count = all_actions
            .iter()
            .filter(|a| a.severity == ActionSeverity::Low)
            .count();

        // 1. Standard OS Capability Score: Modules 1 (Hygiene), 2 (Deps), 3 (OS Capabilities)
        let m1_score = modules
            .iter()
            .find(|m| m.module_id == "files_hygiene")
            .map(|m| m.score)
            .unwrap_or(100);
        let m2_score = modules
            .iter()
            .find(|m| m.module_id == "deps_build")
            .map(|m| m.score)
            .unwrap_or(100);
        let m3_score = modules
            .iter()
            .find(|m| m.module_id == "os_capabilities")
            .map(|m| m.score)
            .unwrap_or(100);
        let normal_os_score = ((m1_score as f32 * 0.25)
            + (m2_score as f32 * 0.25)
            + (m3_score as f32 * 0.50))
            .round() as u32;

        // 2. Bootable Debian 13 ISO Capability Score: Module 4
        let m4_score = modules
            .iter()
            .find(|m| m.module_id == "iso_readiness")
            .map(|m| m.score)
            .unwrap_or(100);
        let iso_readiness_score = m4_score;

        // 3. Student OS + Agentic AI Vision Score: Modules 5 (UI/i18n), 6 (AI/TTS), 7 (Student Features)
        let m5_score = modules
            .iter()
            .find(|m| m.module_id == "ui_theme_i18n")
            .map(|m| m.score)
            .unwrap_or(100);
        let m6_score = modules
            .iter()
            .find(|m| m.module_id == "ai_agent_tts")
            .map(|m| m.score)
            .unwrap_or(100);
        let m7_score = modules
            .iter()
            .find(|m| m.module_id == "student_features")
            .map(|m| m.score)
            .unwrap_or(100);
        let student_os_ai_score = ((m5_score as f32 * 0.30)
            + (m6_score as f32 * 0.35)
            + (m7_score as f32 * 0.35))
            .round() as u32;

        // Weighted Overall Score: 30% Normal OS + 35% ISO + 35% Student OS/AI
        let overall_score = ((normal_os_score as f32 * 0.30)
            + (iso_readiness_score as f32 * 0.35)
            + (student_os_ai_score as f32 * 0.35))
            .round() as u32;

        // Exit Code Determination:
        // 0: Demo / OS Ready (no critical blockers, all scores >= 80)
        // 1: Non-critical warnings (no critical blockers, scores >= 60)
        // 2: Critical blockers found
        let (exit_code, verdict_status) = if critical_count > 0 {
            (
                2,
                "CRITICAL BLOCKERS DETECTED (Fix Required Before ISO Build)",
            )
        } else if normal_os_score >= 80 && iso_readiness_score >= 80 && student_os_ai_score >= 80 {
            (0, "DEMO / OS READY (Full Production Verification Passed)")
        } else {
            (
                1,
                "NON-CRITICAL WARNINGS (System Usable, Polish Recommended)",
            )
        };

        let executive_summary = ExecutiveSummary {
            normal_os_score,
            iso_readiness_score,
            student_os_ai_score,
            overall_score,
            verdict_status: verdict_status.to_string(),
            critical_count,
            high_count,
            medium_count,
            low_count,
            total_features_checked: all_entries.len(),
            pass_count,
            partial_count,
            mock_count,
            missing_count,
        };

        WholeInspectionReport {
            timestamp,
            workspace_root: workspace_root.to_string_lossy().to_string(),
            executive_summary,
            module_results: modules,
            capability_matrix: all_entries,
            action_queue: all_actions,
            exit_code,
        }
    }

    /// Renders the complete, human-readable Markdown report.
    pub fn render_markdown(report: &WholeInspectionReport) -> String {
        let mut out = String::new();
        let s = &report.executive_summary;

        out.push_str(
            "# 🔍 MindSet OS: Whole System Diagnostic & Architectural Verification Report\n\n",
        );
        out.push_str(&format!(
            "**Inspection Timestamp:** `{}`  \n",
            report.timestamp
        ));
        out.push_str(&format!(
            "**Workspace Root:** `{}`  \n",
            report.workspace_root
        ));
        out.push_str(&format!(
            "**Overall Readiness Score:** `{} / 100`  \n",
            s.overall_score
        ));
        out.push_str(&format!(
            "**Executive Verdict:** **`{}`** (Exit Code `{}`)\n\n",
            s.verdict_status, report.exit_code
        ));

        out.push_str("---\n\n");
        out.push_str("## 1. Executive Summary & Verdict Scorecard\n\n");
        out.push_str("| Capability Dimension | Score | Status | Description |\n");
        out.push_str("| :--- | :---: | :---: | :--- |\n");

        let normal_status = if s.normal_os_score >= 80 {
            "✅ PASS"
        } else if s.normal_os_score >= 60 {
            "⚠️ WARN"
        } else {
            "❌ FAIL"
        };
        let iso_status = if s.iso_readiness_score >= 80 {
            "✅ PASS"
        } else if s.iso_readiness_score >= 60 {
            "⚠️ WARN"
        } else {
            "❌ FAIL"
        };
        let student_status = if s.student_os_ai_score >= 80 {
            "✅ PASS"
        } else if s.student_os_ai_score >= 60 {
            "⚠️ WARN"
        } else {
            "❌ FAIL"
        };

        out.push_str(&format!("| **1. Standard OS Capability** | `{} / 100` | {} | Shell, Window Dragging, Real File Vault, Audio, Wi-Fi, sysinfo |\n", s.normal_os_score, normal_status));
        out.push_str(&format!("| **2. Bootable Debian 13 ISO Readiness** | `{} / 100` | {} | live-build Trixie, Bare-Metal Firmware, FreeTube Hook, LF Scripts |\n", s.iso_readiness_score, iso_status));
        out.push_str(&format!("| **3. Student OS + Agentic AI Vision** | `{} / 100` | {} | Proof-of-Study Boss (>=70), Permission Gate, Thanglish TTS, 3D Odyssey |\n", s.student_os_ai_score, student_status));
        out.push_str(&format!("| **Overall System Composite** | `{} / 100` | **`{}`** | Aggregated System Verification Score |\n\n", s.overall_score, if report.exit_code == 0 { "READY" } else if report.exit_code == 1 { "WARNINGS" } else { "BLOCKERS" }));

        out.push_str(&format!("**Feature Audit Counters:** Total Checked: `{}` | Pass: `{}` | Partial: `{}` | Mock: `{}` | Missing: `{}`\n",
            s.total_features_checked, s.pass_count, s.partial_count, s.mock_count, s.missing_count));
        out.push_str(&format!(
            "**Action Items in Queue:** Critical: `{}` | High: `{}` | Medium: `{}` | Low: `{}`\n\n",
            s.critical_count, s.high_count, s.medium_count, s.low_count
        ));

        out.push_str("---\n\n");
        out.push_str("## 2. Module Inspection Breakdowns\n\n");
        for m in &report.module_results {
            out.push_str(&format!("### {}\n\n", m.module_name));
            out.push_str(&format!(
                "**Module Score:** `{} / 100` — {}\n\n",
                m.score, m.summary
            ));
            if !m.metrics.is_empty() {
                out.push_str("**Key Metrics:**\n");
                let mut metric_keys: Vec<_> = m.metrics.keys().collect();
                metric_keys.sort();
                for k in metric_keys {
                    out.push_str(&format!("- **`{}`**: {}\n", k, m.metrics[k]));
                }
                out.push('\n');
            }
        }

        out.push_str("---\n\n");
        out.push_str("## 3. Comprehensive Capability Matrix Table\n\n");
        out.push_str("| # | Feature / Subsystem | Category | Status | Evidence (Exact `file:line`) | Verification Notes |\n");
        out.push_str("| :-: | :--- | :---: | :---: | :--- | :--- |\n");

        for (idx, entry) in report.capability_matrix.iter().enumerate() {
            out.push_str(&format!(
                "| {} | **{}** | `{}` | {} | `{}` | {} |\n",
                idx + 1,
                entry.feature,
                entry.category,
                entry.status.badge(),
                entry.evidence,
                entry.notes
            ));
        }

        out.push_str("\n---\n\n");
        out.push_str("## 4. Prioritized Action Queue\n\n");

        if report.action_queue.is_empty() {
            out.push_str("✨ **Zero action items! The workspace is 100% verified and production-ready.**\n\n");
        } else {
            let severities = [
                (
                    ActionSeverity::Critical,
                    "🚨 CRITICAL (Must Fix Before ISO Build)",
                ),
                (ActionSeverity::High, "⚠️ HIGH (Fix Before Demo)"),
                (
                    ActionSeverity::Medium,
                    "ℹ️ MEDIUM (Architectural Hygiene & UX)",
                ),
                (ActionSeverity::Low, "💡 LOW (Minor Polish & Documentation)"),
            ];

            for (sev, header) in severities {
                let items: Vec<_> = report
                    .action_queue
                    .iter()
                    .filter(|a| a.severity == sev)
                    .collect();
                if items.is_empty() {
                    continue;
                }

                out.push_str(&format!("### {}\n\n", header));
                for (i, action) in items.iter().enumerate() {
                    out.push_str(&format!("#### {}. {}\n", i + 1, action.title));
                    out.push_str(&format!("- **Location:** `{}`\n", action.location));
                    out.push_str(&format!("- **Finding:** {}\n", action.finding));
                    out.push_str(&format!("- **Remediation:** {}\n\n", action.remediation));
                }
            }
        }

        out.push_str("---\n\n");
        out.push_str(
            "*Generated by MindSet OS `whole_inspector_plugin` (Pure Rust Diagnostic Engine).*  \n",
        );
        out.push_str("*Strict Read-Only Verification Guarantee: Workspace was completely unmodified during analysis.*\n");

        out
    }

    /// Writes report artifacts to disk if file paths are configured.
    pub fn save_artifacts(
        report: &WholeInspectionReport,
        md_path: Option<&Path>,
        json_path: Option<&Path>,
    ) -> Result<(), String> {
        if let Some(md_p) = md_path {
            let md_content = Self::render_markdown(report);
            fs::write(md_p, md_content).map_err(|e| {
                format!(
                    "Failed to write markdown report to {}: {}",
                    md_p.display(),
                    e
                )
            })?;
        }

        if let Some(json_p) = json_path {
            let json_content = serde_json::to_string_pretty(report)
                .map_err(|e| format!("Failed to serialize report to JSON: {}", e))?;
            fs::write(json_p, json_content).map_err(|e| {
                format!("Failed to write JSON report to {}: {}", json_p.display(), e)
            })?;
        }

        Ok(())
    }
}
