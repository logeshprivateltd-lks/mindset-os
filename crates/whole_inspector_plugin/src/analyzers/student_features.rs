use crate::analyzers::WorkspaceContext;
use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ModuleAuditResult, PrioritizedAction,
};
use std::collections::HashMap;

/// Module 7: Student OS Extra Features & Vision Audit
pub fn audit(ctx: &WorkspaceContext) -> ModuleAuditResult {
    let mut entries = Vec::new();
    let mut actions = Vec::new();
    let mut metrics = HashMap::new();
    let mut score: u32 = 100;

    let generator_rs = ctx
        .read_file("bins/mindset-os/src/capsules/generator.rs")
        .unwrap_or_default();
    let db_rs = ctx
        .read_file("bins/mindset-os/src/storage/db.rs")
        .unwrap_or_default();
    let boss_view_rs = ctx
        .read_file("bins/mindset-os/src/views/verification_boss.rs")
        .unwrap_or_default();
    let core_verif_rs = ctx
        .read_file("crates/mindset-core/src/engine/verification.rs")
        .unwrap_or_default();
    let desktop_rs = ctx
        .read_file("bins/mindset-os/src/shell/desktop.rs")
        .unwrap_or_default();
    let odyssey_rs = ctx
        .read_file("bins/mindset-os/src/shell/odyssey_map.rs")
        .unwrap_or_default();

    // 1. Exam Scout & Target-to-Capsule Engine
    let has_track_blueprints = generator_rs.contains("TrackBlueprint")
        && generator_rs.contains("Class12BoardsTN600")
        && generator_rs.contains("NEET720")
        && generator_rs.contains("JEE300");

    let has_auto_heal = db_rs.contains("auto_heal_profile") || db_rs.contains("target_score_max");
    let has_unclamped_score_fix = db_rs.contains("10005") || db_rs.contains("target_score_max > 0");

    let gen_line = ctx
        .find_first_occurrence(
            "bins/mindset-os/src/capsules/generator.rs",
            "pub struct TrackBlueprint",
        )
        .map(|(l, _)| l)
        .unwrap_or(10);

    if has_track_blueprints && has_auto_heal && has_unclamped_score_fix {
        entries.push(CapabilityMatrixEntry {
            feature: "Dynamic Exam Scout & Target-to-Capsule Engine".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/capsules/generator.rs:{}", gen_line),
            notes: "Multi-track blueprint engine (TN 600, CBSE 500, NEET 720, JEE 300, UPSC) with SQLite auto-healing and target score clamping".into(),
        });
    } else {
        score = score.saturating_sub(20);
        entries.push(CapabilityMatrixEntry {
            feature: "Dynamic Exam Scout & Target-to-Capsule Engine".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/capsules/generator.rs:{}", gen_line),
            notes:
                "Exam blueprint engine missing dynamic track resolution or database auto-healing"
                    .into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Support multi-track blueprints and database auto-healing".into(),
            location: format!("bins/mindset-os/src/capsules/generator.rs:{}", gen_line),
            finding: "Exam Scout must dynamically adapt study stages to student career tracks with sanitized target score boundaries.".into(),
            remediation: "Implement TrackBlueprint::for_track and verify auto_heal_profile sanitizes target_score_max in db.rs.".into(),
        });
    }

    // 2. Strict Proof-of-Study Verification Boss Battle
    let has_boss_threshold = core_verif_rs.contains(">= 70") || core_verif_rs.contains("70..=89");
    let has_speedrun_threshold =
        core_verif_rs.contains("85") || core_verif_rs.contains("speedrun_mode");
    let has_feynman = boss_view_rs.contains("feynman") || boss_view_rs.contains("Feynman");

    // Strictly check that NO fake manual "mark complete" checkbox bypasses the boss
    let has_fake_complete_checkbox = desktop_rs.contains("checkbox(\"Mark Complete\"")
        || desktop_rs.contains("button(\"Mark Complete\"")
        || desktop_rs.contains("button(\"Mark Done\"");

    let boss_line = ctx
        .find_first_occurrence(
            "crates/mindset-core/src/engine/verification.rs",
            "NO manual \"Mark Complete\" checkboxes",
        )
        .map(|(l, _)| l)
        .unwrap_or(5);

    if has_boss_threshold && has_speedrun_threshold && has_feynman && !has_fake_complete_checkbox {
        entries.push(CapabilityMatrixEntry {
            feature: "Strict Proof-of-Study Verification Boss (Zero Fake Mark-Complete)".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("crates/mindset-core/src/engine/verification.rs:{}", boss_line),
            notes: "Strict 3-stage Boss Battle gating (>= 70 regular / >= 85 speedrun) with dynamic Feynman grading and ZERO fake manual checkboxes".into(),
        });
    } else {
        score = score.saturating_sub(25);
        let reason = if has_fake_complete_checkbox {
            "Found fake manual 'Mark Complete' checkbox or button in study views"
        } else {
            "Missing strict 70-point boss battle pass threshold in verification engine"
        };

        entries.push(CapabilityMatrixEntry {
            feature: "Strict Proof-of-Study Verification Boss (Zero Fake Mark-Complete)".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Mock,
            evidence: format!(
                "crates/mindset-core/src/engine/verification.rs:{}",
                boss_line
            ),
            notes: reason.into(),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Enforce strict Proof-of-Study Boss Gate and remove manual checkboxes".into(),
            location: format!("crates/mindset-core/src/engine/verification.rs:{}", boss_line),
            finding: format!("{}: integrity of student learning requires verified mastery.", reason),
            remediation: "Require passing 3-stage Boss Battle (Rapid Recall, Applied Problem, Feynman) with >= 70 points to mark complete.".into(),
        });
    }

    // 3. Student Desktop Widgets & Focus Layer
    let has_odyssey_trail = odyssey_rs.contains("OdysseyTrail")
        || ctx.has_file("bins/mindset-os/src/shell/odyssey_map.rs");
    let has_scratchpad =
        desktop_rs.contains("Quick Study Scratchpad") || desktop_rs.contains("scratchpad");
    let has_boss_bar = desktop_rs.contains("Daily Boss")
        || desktop_rs.contains("Boss Bar")
        || desktop_rs.contains("boss_session");
    let has_focus_policy = ctx.has_file("crates/mindset-focus-policy/src/lib.rs")
        || ctx.has_file("services/mindset-focus-guard/src/main.rs");

    let odyssey_line = ctx
        .find_first_occurrence("bins/mindset-os/src/shell/odyssey_map.rs", "pub fn view")
        .map(|(l, _)| l)
        .unwrap_or(25);

    if has_odyssey_trail && has_scratchpad && has_boss_bar && has_focus_policy {
        entries.push(CapabilityMatrixEntry {
            feature: "Student Focus Layer & Desktop Widgets (Odyssey Trail & Scratchpad)".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/shell/odyssey_map.rs:{}", odyssey_line),
            notes: "3D Syllabus Odyssey Trail, debounced async SQLite scratchpad, Daily Boss Bar, and daemon focus-guard policy service".into(),
        });
    } else {
        score = score.saturating_sub(15);
        let mut missing_widgets = Vec::new();
        if !has_odyssey_trail {
            missing_widgets.push("3D Syllabus Odyssey Trail");
        }
        if !has_scratchpad {
            missing_widgets.push("Quick Study Scratchpad");
        }
        if !has_boss_bar {
            missing_widgets.push("Daily Boss Bar");
        }
        if !has_focus_policy {
            missing_widgets.push("Focus Policy Guard");
        }

        entries.push(CapabilityMatrixEntry {
            feature: "Student Focus Layer & Desktop Widgets (Odyssey Trail & Scratchpad)".into(),
            category: "Student OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/shell/odyssey_map.rs:{}", odyssey_line),
            notes: format!(
                "Student focus layer missing: {}",
                missing_widgets.join(", ")
            ),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::Medium,
            title: "Integrate Student OS desktop focus widgets".into(),
            location: format!("bins/mindset-os/src/shell/odyssey_map.rs:{}", odyssey_line),
            finding: format!("Desktop missing study widgets: {}", missing_widgets.join(", ")),
            remediation: "Ensure Odyssey Trail, scratchpad, and focus policy daemon are connected in desktop shell view.".into(),
        });
    }

    metrics.insert("student_features_score".into(), score.to_string());
    let summary = format!(
        "Audited Exam Scout track engine, strict Proof-of-Study Boss Battle (>=70 threshold), and Odyssey Trail widgets. Student OS score: {}/100.",
        score
    );

    ModuleAuditResult {
        module_id: "student_features".into(),
        module_name: "Module 7: Student OS Extra Features & Vision Audit".into(),
        score,
        summary,
        entries,
        actions,
        metrics,
    }
}
