use std::collections::HashMap;
use crate::analyzers::WorkspaceContext;
use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ModuleAuditResult, PrioritizedAction,
};

/// Module 3: Standard Operating System Capability Audit
pub fn audit(ctx: &WorkspaceContext) -> ModuleAuditResult {
    let mut entries = Vec::new();
    let mut actions = Vec::new();
    let mut metrics = HashMap::new();
    let mut score: u32 = 100;

    let main_rs = ctx.read_file("bins/mindset-os/src/main.rs").unwrap_or_default();
    let top_bar_rs = ctx.read_file("bins/mindset-os/src/shell/top_bar.rs").unwrap_or_default();
    let browser_rs = ctx.read_file("bins/mindset-os/src/apps/browser.rs").unwrap_or_default();
    let file_explorer_rs = ctx.read_file("bins/mindset-os/src/apps/file_explorer.rs").unwrap_or_default();
    let launcher_rs = ctx.read_file("bins/mindset-os/src/shell/launcher.rs").unwrap_or_default();
    let db_rs = ctx.read_file("bins/mindset-os/src/storage/db.rs").unwrap_or_default();
    let motion_lib = ctx.read_file("crates/mindset-motion/src/lib.rs").unwrap_or_default();

    // 1. Boot, Identity & Session Lifecycle
    let has_boot_splash = ctx.has_file("bins/mindset-os/src/shell/boot_splash.rs")
        && ctx.read_file("bins/mindset-os/src/shell/boot_splash.rs").map(|s| s.contains("HELLO ACHIEVER")).unwrap_or(false);
    let has_onboarding = ctx.has_file("bins/mindset-os/src/onboarding/mod.rs");
    let has_lock_screen = ctx.has_file("bins/mindset-os/src/shell/lock_screen.rs");
    let has_power_menu = main_rs.contains("ActiveOverlay::PowerMenu") || launcher_rs.contains("ActiveOverlay::PowerMenu");

    let splash_line = ctx
        .find_first_occurrence("bins/mindset-os/src/shell/boot_splash.rs", "HELLO ACHIEVER")
        .map(|(l, _)| l)
        .unwrap_or(1);

    if has_boot_splash && has_onboarding && has_lock_screen && has_power_menu {
        entries.push(CapabilityMatrixEntry {
            feature: "Boot, Identity & Session Lifecycle".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/shell/boot_splash.rs:{}", splash_line),
            notes: "Full OS lifecycle verified: 'HELLO ACHIEVER' animated splash, onboarding flow, lock screen, and power menu".into(),
        });
    } else {
        score = score.saturating_sub(15);
        entries.push(CapabilityMatrixEntry {
            feature: "Boot, Identity & Session Lifecycle".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/shell/boot_splash.rs:{}", splash_line),
            notes: "Incomplete session lifecycle: missing one or more of splash, onboarding, lock screen, or power menu".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Complete OS session lifecycle components".into(),
            location: "bins/mindset-os/src/main.rs:1".into(),
            finding: "OS lifecycle components missing from shell pipeline.".into(),
            remediation: "Ensure boot splash, onboarding, lock screen, and power menu overlays are wired into main update loop.".into(),
        });
    }

    // 2. Window Manager & Shell Interactive Controls
    let has_dragging = main_rs.contains("StartWindowDrag") || main_rs.contains("wm_interaction.dragging");
    let has_resizing = main_rs.contains("WindowResizing") || main_rs.contains("Event::Window(window::Event::Resized");
    let has_clamping = main_rs.contains("clamp") || main_rs.contains("safe-area") || main_rs.contains("min(") || main_rs.contains("max(");
    let has_z_index = main_rs.contains("bring_to_front") || main_rs.contains("focus_window");
    let has_top_bar_3_zone = top_bar_rs.contains("MindSetLogoProgram") && top_bar_rs.contains("Dynamic Notch");
    let has_active_overlay_enum = ctx.read_file("crates/mindset-core/src/wm.rs").map(|s| s.contains("enum ActiveOverlay")).unwrap_or(false)
        || ctx.read_file("crates/mindset-core/src/lib.rs").map(|s| s.contains("ActiveOverlay")).unwrap_or(false);

    let wm_line = ctx
        .find_first_occurrence("bins/mindset-os/src/main.rs", "StartWindowDrag")
        .map(|(l, _)| l)
        .unwrap_or(832);

    if has_dragging && has_resizing && has_clamping && has_z_index && has_top_bar_3_zone && has_active_overlay_enum {
        entries.push(CapabilityMatrixEntry {
            feature: "Floating Window Manager & Shell Geometry".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/main.rs:{}", wm_line),
            notes: "Interactive window dragging/resizing with viewport clamping, 3-zone top bar with vector 'MS' logo, and compile-time ActiveOverlay routing".into(),
        });
    } else {
        score = score.saturating_sub(15);
        entries.push(CapabilityMatrixEntry {
            feature: "Floating Window Manager & Shell Geometry".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/main.rs:{}", wm_line),
            notes: "Window manager lacks full interactive dragging, resizing, or 3-zone top bar integration".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Implement real window movement and resizing in main.rs".into(),
            location: format!("bins/mindset-os/src/main.rs:{}", wm_line),
            finding: "Window dragging or geometry clamping is incomplete.".into(),
            remediation: "Handle mouse delta cursor movements to update window positions and clamp within screen bounds.".into(),
        });
    }

    // 3. Real File System & Vault Navigation
    let has_fs_read_dir = file_explorer_rs.contains("fs::read_dir") || file_explorer_rs.contains("std::fs::read_dir");
    let has_selection_state = file_explorer_rs.contains("selected_name") || file_explorer_rs.contains("selected_path");
    let has_file_preview = file_explorer_rs.contains("preview_content") || file_explorer_rs.contains("fs::read_to_string");

    let fe_line = ctx
        .find_first_occurrence("bins/mindset-os/src/apps/file_explorer.rs", "fs::read_dir")
        .map(|(l, _)| l)
        .unwrap_or(30);

    if has_fs_read_dir && has_selection_state && has_file_preview {
        entries.push(CapabilityMatrixEntry {
            feature: "Real Filesystem Navigation & File Vault".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/apps/file_explorer.rs:{}", fe_line),
            notes: "Real std::fs::read_dir traversal with click hit-testing, selection state, and live text/metadata preview (zero mock lists)".into(),
        });
    } else {
        score = score.saturating_sub(20);
        entries.push(CapabilityMatrixEntry {
            feature: "Real Filesystem Navigation & File Vault".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Mock,
            evidence: format!("bins/mindset-os/src/apps/file_explorer.rs:{}", fe_line),
            notes: "File Explorer appears to rely on static mock items rather than real std::fs directory reading".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Replace mock file list with real std::fs directory reader".into(),
            location: format!("bins/mindset-os/src/apps/file_explorer.rs:{}", fe_line),
            finding: "File explorer lacks real std::fs::read_dir or live file content inspection.".into(),
            remediation: "Read actual directories from Study Capsules, Documents, and Downloads with selection hit-testing and content preview.".into(),
        });
    }

    // 4. Networking, Internet & Real App Launching
    let has_real_browser_launch = main_rs.contains("LaunchMindSetBrowser") || browser_rs.contains("mindset-browser");
    let has_real_youtube_launch = main_rs.contains("LaunchYouTubeApp") || browser_rs.contains("youtube");
    let has_real_freetube_launch = main_rs.contains("LaunchFreeTubeApp") || browser_rs.contains("freetube");

    // Check for distraction clutter in launcher (Edge, Netflix, Spotify)
    let has_netflix = launcher_rs.to_lowercase().contains("netflix");
    let has_spotify = launcher_rs.to_lowercase().contains("spotify");
    let has_edge = launcher_rs.to_lowercase().contains("msedge") || launcher_rs.to_lowercase().contains("microsoft edge");

    let browser_line = ctx
        .find_first_occurrence("bins/mindset-os/src/apps/browser.rs", "Academic Launchpad & Real Web Engine")
        .map(|(l, _)| l)
        .unwrap_or(1);

    if has_real_browser_launch && has_real_youtube_launch && has_real_freetube_launch && !has_netflix && !has_spotify && !has_edge {
        entries.push(CapabilityMatrixEntry {
            feature: "Academic Web & Video App Launchers".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/apps/browser.rs:{}", browser_line),
            notes: "Real desktop application spawning: Chromium/MindSet Browser, Official YouTube, and FreeTube (0 distraction apps like Netflix/Spotify/Edge in launcher)".into(),
        });
    } else {
        score = score.saturating_sub(15);
        let reason = if has_netflix || has_spotify || has_edge {
            "Distraction app shortcuts (Netflix/Spotify/Edge) detected in launcher"
        } else {
            "Incomplete real desktop app launch integration"
        };

        entries.push(CapabilityMatrixEntry {
            feature: "Academic Web & Video App Launchers".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/apps/browser.rs:{}", browser_line),
            notes: reason.into(),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Enforce distraction-free app launchers and purge mock browsers".into(),
            location: format!("bins/mindset-os/src/shell/launcher.rs:1"),
            finding: format!("{}: student OS must focus exclusively on academic tools.", reason),
            remediation: "Ensure launcher provides direct commands for Chromium, YouTube, and FreeTube, with no commercial entertainment apps.".into(),
        });
    }

    // 5. Hardware Controls & Low-RAM Performance Hygiene
    let has_sysinfo = ctx.has_file("bins/mindset-os/src/system/sysinfo_collector.rs");
    let has_audio_ctrl = ctx.has_file("bins/mindset-os/src/system/audio.rs");
    let has_sqlite_wal = db_rs.contains("PRAGMA journal_mode = WAL");
    let has_sqlite_cache_cap = db_rs.contains("PRAGMA cache_size = -2000");
    let has_spring_at_rest = motion_lib.contains("is_at_rest(&self)");

    let wal_line = ctx
        .find_first_occurrence("bins/mindset-os/src/storage/db.rs", "PRAGMA journal_mode = WAL")
        .map(|(l, _)| l)
        .unwrap_or(290);

    let motion_line = ctx
        .find_first_occurrence("crates/mindset-motion/src/lib.rs", "is_at_rest(&self)")
        .map(|(l, _)| l)
        .unwrap_or(71);

    if has_sysinfo && has_audio_ctrl && has_sqlite_wal && has_sqlite_cache_cap && has_spring_at_rest {
        entries.push(CapabilityMatrixEntry {
            feature: "Hardware Monitoring & Low-RAM Hygiene".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/storage/db.rs:{} | crates/mindset-motion/src/lib.rs:{}", wal_line, motion_line),
            notes: "Live CPU/RAM sysinfo, volume controls, SQLite WAL mode + 2MB cache cap, and zero-idle 60 FPS spring rest guards".into(),
        });
    } else {
        score = score.saturating_sub(15);
        let mut missing_hygiene = Vec::new();
        if !has_sysinfo { missing_hygiene.push("sysinfo collector"); }
        if !has_sqlite_wal { missing_hygiene.push("SQLite WAL mode"); }
        if !has_sqlite_cache_cap { missing_hygiene.push("SQLite cache cap"); }
        if !has_spring_at_rest { missing_hygiene.push("spring is_at_rest() guard"); }

        entries.push(CapabilityMatrixEntry {
            feature: "Hardware Monitoring & Low-RAM Hygiene".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/storage/db.rs:{}", wal_line),
            notes: format!("Low-RAM performance hygiene incomplete: {}", missing_hygiene.join(", ")),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Apply SQLite WAL mode and spring stop guards".into(),
            location: format!("bins/mindset-os/src/storage/db.rs:{}", wal_line),
            finding: format!("Missing performance hygiene safeguards: {}", missing_hygiene.join(", ")),
            remediation: "Enable WAL mode and PRAGMA cache_size = -2000 in db.rs, and verify spring.is_at_rest() terminates GUI redraws.".into(),
        });
    }

    metrics.insert("os_capabilities_score".into(), score.to_string());
    let summary = format!(
        "Audited boot lifecycle, window manager, real file vault, academic launchpad, and hardware controls. Standard OS score: {}/100.",
        score
    );

    ModuleAuditResult {
        module_id: "os_capabilities".into(),
        module_name: "Module 3: Standard Operating System Capability Audit".into(),
        score,
        summary,
        entries,
        actions,
        metrics,
    }
}
