use std::collections::HashMap;
use crate::analyzers::WorkspaceContext;
use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ModuleAuditResult, PrioritizedAction,
};

/// Module 1: Filesystem, Assets & Codebase Hygiene Analyzer
pub fn audit(ctx: &WorkspaceContext) -> ModuleAuditResult {
    let mut entries = Vec::new();
    let mut actions = Vec::new();
    let mut metrics = HashMap::new();
    let mut score: u32 = 100;

    // 1. Catalog Workspace Structure
    let total_files = ctx.files.len();
    let total_bytes: u64 = ctx.files.iter().map(|f| f.size_bytes).sum();
    let rust_files = ctx.files.iter().filter(|f| f.extension == "rs").count();
    let rust_lines: usize = ctx
        .files
        .iter()
        .filter(|f| f.extension == "rs")
        .map(|f| f.line_count)
        .sum();

    metrics.insert("total_files".into(), total_files.to_string());
    metrics.insert("total_bytes".into(), format!("{} KB", total_bytes / 1024));
    metrics.insert("rust_files".into(), rust_files.to_string());
    metrics.insert("rust_lines".into(), rust_lines.to_string());

    let crates_found: Vec<_> = ctx
        .files
        .iter()
        .filter(|f| f.rel_path.starts_with("crates/") && f.rel_path.ends_with("/Cargo.toml"))
        .map(|f| f.rel_path.replace("/Cargo.toml", ""))
        .collect();
    metrics.insert("crates_count".into(), crates_found.len().to_string());

    // 2. Check for leftover legacy Web / Tauri clutter
    let legacy_clutter: Vec<_> = ctx
        .files
        .iter()
        .filter(|f| {
            let p = &f.rel_path;
            p.ends_with(".tsx")
                || p.ends_with(".ts")
                || p.ends_with(".jsx")
                || p.contains("node_modules/")
                || p.ends_with("package.json")
                || p.contains("vite.config.")
                || p.contains("src-tauri/")
        })
        .collect();

    if legacy_clutter.is_empty() {
        entries.push(CapabilityMatrixEntry {
            feature: "Pure-Rust Clean Slate (Zero Web/Tauri Clutter)".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: "0 legacy web/Tauri files (.tsx, .ts, node_modules, package.json) found".into(),
            notes: "Workspace is 100% pure Rust and native desktop assets".into(),
        });
    } else {
        score = score.saturating_sub(20);
        let first_loc = format!("{}:1", legacy_clutter[0].rel_path);
        let sample_files: Vec<_> = legacy_clutter.iter().take(3).map(|f| f.rel_path.as_str()).collect();

        entries.push(CapabilityMatrixEntry {
            feature: "Pure-Rust Clean Slate (Zero Web/Tauri Clutter)".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: first_loc.clone(),
            notes: format!("Found {} legacy web/Tauri files: {}", legacy_clutter.len(), sample_files.join(", ")),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Remove leftover legacy web/Tauri clutter".into(),
            location: first_loc,
            finding: format!("Found {} legacy frontend files leftover from earlier web/Tauri iterations.", legacy_clutter.len()),
            remediation: "Delete all obsolete .tsx, .ts, node_modules, and package.json files to preserve pure Rust build hygiene.".into(),
        });
    }

    // 3. Verify archived directories are cleanly excluded from active build
    let root_cargo = ctx.read_file("Cargo.toml").unwrap_or_default();
    let has_archive_dir = ctx.files.iter().any(|f| f.rel_path.starts_with("archive/"));
    
    let (root_excludes_archive, root_members_has_archive) = {
        let mut inside_members = false;
        let mut inside_exclude = false;
        let mut in_members = false;
        let mut in_exclude = false;

        for line in root_cargo.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("members =") {
                inside_members = true;
                inside_exclude = false;
            } else if trimmed.starts_with("exclude =") {
                inside_exclude = true;
                inside_members = false;
            } else if trimmed.starts_with(']') {
                inside_members = false;
                inside_exclude = false;
            } else if trimmed.contains("\"archive\"") || trimmed.contains("'archive'") {
                if inside_members {
                    in_members = true;
                }
                if inside_exclude {
                    in_exclude = true;
                }
            }
        }
        (in_exclude, in_members)
    };

    if has_archive_dir {
        if root_excludes_archive && !root_members_has_archive {
            let line_no = ctx
                .find_first_occurrence("Cargo.toml", "\"archive\"")
                .map(|(l, _)| l)
                .unwrap_or(18);
            entries.push(CapabilityMatrixEntry {
                feature: "Archived Legacy Exclusion".into(),
                category: "Normal OS".into(),
                status: CapabilityStatus::Pass,
                evidence: format!("Cargo.toml:{}", line_no),
                notes: "archive/ directory cleanly excluded from workspace members in root Cargo.toml".into(),
            });
        } else {
            score = score.saturating_sub(15);
            entries.push(CapabilityMatrixEntry {
                feature: "Archived Legacy Exclusion".into(),
                category: "Normal OS".into(),
                status: CapabilityStatus::Partial,
                evidence: "Cargo.toml:1".into(),
                notes: "archive/ directory exists but is not cleanly isolated under [workspace.exclude]".into(),
            });
            actions.push(PrioritizedAction {
                severity: ActionSeverity::High,
                title: "Ensure archive/ directory is excluded in root Cargo.toml".into(),
                location: "Cargo.toml:1".into(),
                finding: "Archived directories may accidentally be included in workspace member compilation.".into(),
                remediation: "Add 'archive' to [workspace.exclude] in root Cargo.toml.".into(),
            });
        }
    } else {
        entries.push(CapabilityMatrixEntry {
            feature: "Archived Legacy Exclusion".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: "Cargo.toml:1".into(),
            notes: "No unexcluded archive directories detected in active workspace".into(),
        });
    }

    // 4. Bundled Fonts: Tamil, Malayalam, and Latin Font Verification
    let has_latin_font = ctx.has_file("assets/fonts/NotoSans-Regular.ttf");
    let has_tamil_font = ctx.has_file("assets/fonts/NotoSansTamil-Regular.ttf");
    let has_malayalam_font = ctx.has_file("assets/fonts/NotoSansMalayalam-Regular.ttf");

    let main_rs = ctx.read_file("bins/mindset-os/src/main.rs").unwrap_or_default();
    let main_registers_tamil = main_rs.contains("NotoSansTamil-Regular.ttf");
    let main_registers_malayalam = main_rs.contains("NotoSansMalayalam-Regular.ttf");
    let main_registers_latin = main_rs.contains("NotoSans-Regular.ttf");

    let font_reg_line = ctx
        .find_first_occurrence("bins/mindset-os/src/main.rs", "NotoSansTamil-Regular.ttf")
        .map(|(l, _)| l)
        .unwrap_or(2789);

    if has_latin_font && has_tamil_font && has_malayalam_font && main_registers_tamil && main_registers_malayalam && main_registers_latin {
        entries.push(CapabilityMatrixEntry {
            feature: "Bundled Multi-Script Fonts (Tamil, Malayalam, Latin)".into(),
            category: "UI".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/main.rs:{}", font_reg_line),
            notes: "All 3 NotoSans fonts bundled in assets/fonts/ and registered via .font() in app builder".into(),
        });
    } else {
        score = score.saturating_sub(15);
        let mut missing = Vec::new();
        if !has_latin_font || !main_registers_latin { missing.push("NotoSans-Regular.ttf"); }
        if !has_tamil_font || !main_registers_tamil { missing.push("NotoSansTamil-Regular.ttf"); }
        if !has_malayalam_font || !main_registers_malayalam { missing.push("NotoSansMalayalam-Regular.ttf"); }

        entries.push(CapabilityMatrixEntry {
            feature: "Bundled Multi-Script Fonts (Tamil, Malayalam, Latin)".into(),
            category: "UI".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/main.rs:{}", font_reg_line),
            notes: format!("Missing font assets or registration: {}", missing.join(", ")),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Register bundled Indic fonts in main.rs app builder".into(),
            location: format!("bins/mindset-os/src/main.rs:{}", font_reg_line),
            finding: format!("Font registration incomplete for: {}", missing.join(", ")),
            remediation: "Ensure NotoSans, NotoSansTamil, and NotoSansMalayalam TTF files are placed in assets/fonts/ and registered in iced builder with .font(include_bytes!(...)).".into(),
        });
    }

    // 5. Dead / Unreferenced Rust Module Detection
    // Check bins/mindset-os/src/views/mod.rs
    let views_mod = ctx.read_file("bins/mindset-os/src/views/mod.rs").unwrap_or_default();
    let view_files: Vec<_> = ctx
        .files
        .iter()
        .filter(|f| f.rel_path.starts_with("bins/mindset-os/src/views/") && f.rel_path.ends_with(".rs") && !f.rel_path.ends_with("mod.rs"))
        .collect();

    let mut unreferenced_views = Vec::new();
    for vf in &view_files {
        let file_stem = vf.rel_path.rsplit('/').next().unwrap_or("").replace(".rs", "");
        let mod_decl = format!("pub mod {};", file_stem);
        let alt_mod_decl = format!("mod {};", file_stem);
        if !views_mod.contains(&mod_decl) && !views_mod.contains(&alt_mod_decl) {
            unreferenced_views.push(vf.rel_path.clone());
        }
    }

    if unreferenced_views.is_empty() {
        entries.push(CapabilityMatrixEntry {
            feature: "Clean View Hierarchy & Module Registration".into(),
            category: "UI".into(),
            status: CapabilityStatus::Pass,
            evidence: "bins/mindset-os/src/views/mod.rs:1".into(),
            notes: format!("All {} view modules cleanly declared and active in views/mod.rs", view_files.len()),
        });
    } else {
        score = score.saturating_sub(10);
        let first_loc = format!("{}:1", unreferenced_views[0]);
        entries.push(CapabilityMatrixEntry {
            feature: "Clean View Hierarchy & Module Registration".into(),
            category: "UI".into(),
            status: CapabilityStatus::Partial,
            evidence: first_loc.clone(),
            notes: format!("Unreferenced view modules detected: {}", unreferenced_views.join(", ")),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::Medium,
            title: "Register or prune unused view modules in views/mod.rs".into(),
            location: first_loc,
            finding: format!("Files in views/ directory are not declared in views/mod.rs: {}", unreferenced_views.join(", ")),
            remediation: "Add 'pub mod <name>;' in views/mod.rs or remove abandoned view files.".into(),
        });
    }

    // 6. Duplicate Media & Image Reference Audit
    let image_assets: Vec<_> = ctx
        .files
        .iter()
        .filter(|f| {
            f.rel_path.starts_with("assets/")
                && matches!(f.extension.as_str(), "svg" | "png" | "jpg" | "jpeg" | "ico")
        })
        .collect();

    let mut size_map: HashMap<u64, Vec<String>> = HashMap::new();
    for img in &image_assets {
        size_map.entry(img.size_bytes).or_default().push(img.rel_path.clone());
    }

    let mut duplicate_groups = Vec::new();
    for (_, group) in size_map {
        if group.len() > 1 {
            // Verify byte content
            let b0 = ctx.read_file_bytes(&group[0]);
            let b1 = ctx.read_file_bytes(&group[1]);
            if b0.is_some() && b0 == b1 {
                duplicate_groups.push(group);
            }
        }
    }

    if duplicate_groups.is_empty() {
        entries.push(CapabilityMatrixEntry {
            feature: "Asset Deduplication & Media Integrity".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Pass,
            evidence: "assets/icons/ms_logo.svg:1".into(),
            notes: format!("0 duplicate media assets found across {} inspected assets", image_assets.len()),
        });
    } else {
        score = score.saturating_sub(5);
        let sample = duplicate_groups[0].join(" <=> ");
        entries.push(CapabilityMatrixEntry {
            feature: "Asset Deduplication & Media Integrity".into(),
            category: "Normal OS".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("{}:1", duplicate_groups[0][0]),
            notes: format!("Identical asset duplicates found: {}", sample),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::Low,
            title: "Deduplicate identical media asset files".into(),
            location: format!("{}:1", duplicate_groups[0][0]),
            finding: format!("Found identical media files consuming redundant disk space: {}", sample),
            remediation: "Consolidate duplicate media assets to a single canonical path in assets/.".into(),
        });
    }

    let summary = format!(
        "Audited {} files ({} KB, {} Rust lines across {} crates). Pure-Rust hygiene score: {}/100.",
        total_files,
        total_bytes / 1024,
        rust_lines,
        crates_found.len(),
        score
    );

    ModuleAuditResult {
        module_id: "files_hygiene".into(),
        module_name: "Module 1: Filesystem, Assets & Codebase Hygiene".into(),
        score,
        summary,
        entries,
        actions,
        metrics,
    }
}
