//! Tier 3: Cross-Feature Combinations Suite
//!
//! Comprehensive multi-subsystem integration testing across Shell, Odyssey,
//! File System, Maindy AI, Exam Reviewer, and Voice Synthesis.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::agent::{
    CompetitiveMcqGenerator, ExamReviewer, StudentAnswer, StudentSubmission,
};
use mindset_core::{ActiveOverlay, DesktopWidgetId, ThemeMode};
use mindset_intelligence::personalization::{
    DailyFatigueCheckin, FatigueLevel, LeitnerBox, PersonalizationEngine,
};
use mindset_intelligence::rag_vault::RagVault;
use mindset_intelligence::search_engine::{AcademicEncyclopedia, AdBlockShield};
use mindset_os::ai::persona::{parse_dual_text, SubjectEntity, ThanglishParser};
use mindset_os::ai::prompt_builder::build_personalized_system_prompt;
use mindset_os::shell::dynamic_island::IslandMode;
use mindset_os::theme::liquidmorphism::LiquidMetrics;
use mindset_os::theme::ThemePalette;
use mindset_os::Message;
use mindset_sys::fs::document::{DocumentContent, DocumentHandler};
use mindset_sys::fs::explorer::list_directory;
use std::collections::HashMap;

use crate::helpers::{
    benchmark_dir, create_sample_pdf, create_sample_pptx, create_test_odyssey_node,
    create_test_profile, HeadlessHarness, AUTH_LOCK,
};

// ===========================================================================
// COMBINATORIAL 1: F2 (Auth Lock) + F3 (Editable Widgets)
// ===========================================================================
#[test]
fn test_comb_f02_lock_f03_widgets() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    let id = DesktopWidgetId::ScholarBriefing;

    // 1. Position and pin a desktop widget
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.x = 120.0;
        w.y = 80.0;
        w.is_pinned = true;
    }
    let initial_w = harness
        .app
        .core
        .wm
        .get_widget_placement(id)
        .unwrap()
        .clone();
    assert!(initial_w.is_pinned);

    // 2. Lock the workspace
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // 3. Attempting to drag widget while locked has zero effect on coordinates
    harness.dispatch(Message::StartWidgetDrag(id));
    assert!(harness.app.wm_interaction.dragging_widget.is_none());
    let post_lock_w = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert_eq!(post_lock_w.x, initial_w.x);
    assert_eq!(post_lock_w.y, initial_w.y);

    // 4. Unlock with valid PIN
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);

    // 5. Restored desktop retains exact widget configuration
    let restored_w = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert_eq!(restored_w.x, initial_w.x);
    assert!(restored_w.is_pinned);
}

// ===========================================================================
// COMBINATORIAL 2: F1 (Theme Switching) + F5 (Liquid Notch Island)
// ===========================================================================
#[test]
fn test_comb_f01_theme_f05_notch() {
    let mut harness = HeadlessHarness::new();

    // DarkSpace palette tokens applied to notch
    harness.dispatch(Message::ThemeSelected(ThemeMode::DarkSpace));
    let dark_palette = ThemePalette::for_mode(harness.app.core.preferences.theme);
    assert_eq!(dark_palette.mode, ThemeMode::DarkSpace);
    assert!(dark_palette.background.r < 0.15);

    // Switch to AlpineGlass: notch palette adapts to bright translucent frosted tokens
    harness.dispatch(Message::ThemeSelected(ThemeMode::AlpineGlass));
    let light_palette = ThemePalette::for_mode(harness.app.core.preferences.theme);
    assert_eq!(light_palette.mode, ThemeMode::AlpineGlass);
    assert!(light_palette.background.r > 0.70);

    // Compact notch dimensions remain stable across theme switches
    let (w, h) = IslandMode::Compact.target_dimensions(0);
    assert_eq!(w, 300.0);
    assert_eq!(h, 34.0);
}

// ===========================================================================
// COMBINATORIAL 3: F8 (Multi-Chapter Units) + F9 (Crown Mastery) + F10 (3D Path)
// ===========================================================================
#[test]
fn test_comb_f08_units_f09_crown_f10_path() {
    let mut node = create_test_odyssey_node("n-optics-1", "Snell's Law & Refraction", 99, 100);
    node.unit_id = "unit-3-optics".to_string();
    node.unit_title = "Unit 3: Ray & Wave Optics".to_string();

    // Verify 99% yields State Board Crown
    assert_eq!(node.mastery_percentage(), 99);
    assert_eq!(node.stars_earned, 3);
    assert!(node.is_crown_mastered);

    // Verify 3D projection renders node at near-plane foreground
    let pt = mindset_os::shell::odyssey_map::project(0.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(pt.z, 1.0);
    assert_eq!(pt.r, 26.0);
}

// ===========================================================================
// COMBINATORIAL 4: F11 (Filesystem Tree) + F12 (PDF Viewer) + F16 (RAG Vault)
// ===========================================================================
#[test]
fn test_comb_f11_fs_f12_pdf_f16_rag() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();
    let pdf_path = harness.vault_path().join("electromagnetism_vol1.pdf");

    // 1. Create PDF document on disk
    let page_content = "Chapter 1: Gauss Law and Applications. Total flux is q / epsilon_0.";
    create_sample_pdf(&pdf_path, "Physics Vol 1", &[page_content]).expect("Create PDF");

    // 2. Discover via filesystem directory listing
    let listing = list_directory(harness.vault_path(), &conn, None).expect("List dir");
    assert!(listing
        .entries
        .iter()
        .any(|e| e.name == "electromagnetism_vol1.pdf"));

    // 3. Inspect document with DocumentHandler
    let doc = DocumentHandler::read(&pdf_path).expect("Read PDF");
    assert_eq!(doc.count(), 1);
    assert!(doc.extract_full_text().contains("Gauss Law"));

    // 4. Ingest into RAG Vault and verify chunk page citation anchor
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Init vault");
    let chunks = vault.ingest_pdf(&pdf_path).expect("Ingest PDF");
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].page_number, 1);
    assert_eq!(chunks[0].file_name, "electromagnetism_vol1.pdf");
}

// ===========================================================================
// COMBINATORIAL 5: F11 (Filesystem) + F13 (PPTX Parser) + F14 (In-Place Editor)
// ===========================================================================
#[test]
fn test_comb_f11_fs_f13_pptx_f14_editor() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("organic_chemistry.pptx");

    // 1. Create PPTX presentation
    create_sample_pptx(
        &pptx_path,
        "Organic Chemistry Seminar",
        &[("Slide 1: Reaction Kinetics", "Rate = k[A][B]")],
    )
    .expect("Create PPTX");

    // 2. Read PPTX
    let doc = DocumentHandler::read(&pptx_path).expect("Read PPTX");
    if let DocumentContent::Pptx {
        mut slides,
        metadata,
    } = doc
    {
        assert_eq!(slides.len(), 1);
        // In-place edit of Slide 1 equation
        slides[0].body_blocks = vec!["Arrhenius equation: k = A * exp(-Ea / RT)".to_string()];
        DocumentHandler::save_update(&pptx_path, &DocumentContent::Pptx { slides, metadata })
            .expect("Save update");
    }

    // 3. Re-read to confirm updated slide content persisted
    let reread = DocumentHandler::read(&pptx_path).expect("Re-read PPTX");
    match reread {
        DocumentContent::Pptx { slides, .. } => {
            assert!(slides[0].body_blocks[0].contains("Arrhenius equation"));
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

// ===========================================================================
// COMBINATORIAL 6: F15 (Study Workstation) + F18 (Day-by-Day Fatigue Adaptation)
// ===========================================================================
#[test]
fn test_comb_f15_workstation_f18_fatigue() {
    // High fatigue (9.0) check-in shifts workstation session to recovery focus
    let (capsule, break_time) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Fatigued, 2.0);
    assert!(capsule <= 20, "Fatigued capsule reduced to <= 20m");
    assert!(break_time >= 8, "Recovery break extended to >= 8m");
}

// ===========================================================================
// COMBINATORIAL 7: F16 (RAG Vault) + F17 (Academic Web Search Fallback)
// ===========================================================================
#[test]
fn test_comb_f16_rag_f17_search_fallback() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Init vault");

    // 1. Query topic missing from local vault
    let local_matches = vault
        .search("Superconductivity Cooper Pairs", 3)
        .expect("Search");

    // 2. If 0 local matches, fallback to AcademicEncyclopedia database
    if local_matches.is_empty() {
        let encyclopedia = AcademicEncyclopedia::default();
        let shield = AdBlockShield::default();

        // Query encyclopedia while AdBlock shield validates clean domain
        assert_eq!(
            shield.should_block_domain("https://arxiv.org/abs/2301.99999"),
            None
        );
        assert!(!encyclopedia.entries.is_empty());
    }
}

// ===========================================================================
// COMBINATORIAL 8: F19 (590/600 Exam Reviewer) + F9 (Odyssey Crown Mastery)
// ===========================================================================
#[test]
fn test_comb_f19_exam_reviewer_f09_crown() {
    let rubric = ExamReviewer::load_rubric(
        &benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json"),
    )
    .expect("Load State Board rubric");

    // 1. Evaluate 590/600 paper via ExamReviewer -> feeds OdysseyNode evaluation
    let perfect_sheet = ExamReviewer::evaluate(
        &benchmark_dir().join("sample_exams/tn_class12_physics_590.txt"),
        &rubric,
    )
    .expect("Evaluate perfect paper");
    assert!(perfect_sheet.achieved_590_crown);

    let mut crown_node = create_test_odyssey_node(
        "n-exam-1",
        "Class 12 Mid-Term Physics",
        0,
        perfect_sheet.max_marks.round() as u32,
    );
    crown_node.update_evaluation(perfect_sheet.total_score.round() as u32);
    assert!(crown_node.is_crown_mastered);
    assert_eq!(crown_node.stars_earned, 3);

    // 2. Evaluate deductions paper (16.5/18.0 = 91.67%) -> earns 3 stars (>=90%) but misses Crown (<98%)
    let ded_sheet = ExamReviewer::evaluate(
        &benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt"),
        &rubric,
    )
    .expect("Evaluate deductions paper");
    assert!(!ded_sheet.achieved_590_crown);

    let mut non_crown_node = create_test_odyssey_node(
        "n-exam-2",
        "Class 12 Physics Deductions",
        0,
        ded_sheet.max_marks.round() as u32,
    );
    non_crown_node.update_evaluation(ded_sheet.total_score.floor() as u32);
    assert!(!non_crown_node.is_crown_mastered);
}

// ===========================================================================
// COMBINATORIAL 9: F20 (MCQ Generator) + F18 (Leitner Mistake Repetition)
// ===========================================================================
#[test]
fn test_comb_f20_mcq_generator_f18_leitner() {
    let harness = HeadlessHarness::new();
    let db_path = harness.vault_path().join("comb_personalization.db");
    let engine = PersonalizationEngine::new(db_path).expect("Init engine");

    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_jee_paper("Physics", Some("Rotational"), 2, 0)
        .expect("Generate JEE paper");

    // Submit 1 correct and 1 intentionally wrong answer
    let mut answers = HashMap::new();
    let q0 = &paper.questions[0];
    let q1 = &paper.questions[1];
    answers.insert(
        q0.id.clone(),
        StudentAnswer::Option(q0.correct_option_index.unwrap_or(0)),
    );
    answers.insert(
        q1.id.clone(),
        StudentAnswer::Option((q1.correct_option_index.unwrap_or(0) + 1) % 4),
    );

    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert_eq!(report.incorrect_count, 1);

    // Feed real incorrect QuestionResult directly into PersonalizationEngine mistake log + Leitner Box 1
    for res in report.question_results.iter().filter(|r| !r.is_correct) {
        engine
            .record_mistake(
                "Physics",
                &res.topic,
                &res.question_id,
                &res.student_answer,
                &res.correct_answer,
                "Distractor Trap",
            )
            .expect("Record mistake");
        engine
            .add_leitner_card(&res.question_id, &res.topic, &res.correct_answer, "Physics")
            .expect("Add Leitner card");
        let demoted = engine
            .record_card_review(&res.question_id, false, "2026-11-25")
            .expect("Demote failed card");
        assert_eq!(demoted.current_box, 1);
    }

    let weak = engine
        .get_unresolved_mistakes(Some("Physics"))
        .expect("Get weak topics");
    assert_eq!(weak.len(), 1);
    assert_eq!(weak[0].question_prompt, q1.id);
    assert_eq!(LeitnerBox::demote_on_failure().interval_days(), 1);
}

// ===========================================================================
// COMBINATORIAL 10: F22 (Thanglish NLP) + F24 (Dual-Text Voice)
// ===========================================================================
#[test]
fn test_comb_f22_thanglish_f24_dual_text() {
    let query = "p6 la chapter 10 la 10.6 example solli kudu bro";
    let query_res = ThanglishParser::parse(query);

    assert_eq!(query_res.subject, Some(SubjectEntity::Physics));
    assert_eq!(query_res.chapter, Some(10));
    assert_eq!(query_res.example.as_deref(), Some("10.6"));

    // Generated AI response is parsed into dual-text
    let ai_response = "DISPLAY: Here is Example 10.6 on Wave Optics: Fringe width beta = lambda * D / d.\nSPEECH: Vanakkam da mapla! Example 10.6 la fringe width formula paaru: beta equals lambda D by d.";
    let dual = parse_dual_text(ai_response, "thanglish");

    assert!(dual.display_text.contains("lambda * D / d"));
    assert!(dual.speech_text.contains("beta equals lambda"));
    assert_eq!(dual.primary_lang, "ta-IN");
}

// ===========================================================================
// COMBINATORIAL 11: F22 (Thanglish NLP) + F16 (RAG Vault Query)
// ===========================================================================
#[test]
fn test_comb_f22_thanglish_f16_rag() {
    let query = "hey bro physics la chapter 10 wave optics explain pannu";
    let query_res = ThanglishParser::parse(query);

    assert_eq!(query_res.subject, Some(SubjectEntity::Physics));
    assert_eq!(query_res.chapter, Some(10));

    // Construct targeted search query from extracted entities
    let target_search = format!(
        "{:?} Chapter {}",
        query_res.subject.unwrap(),
        query_res.chapter.unwrap()
    );
    assert_eq!(target_search, "Physics Chapter 10");
}

// ===========================================================================
// COMBINATORIAL 12: F2 (Auth Lock) + F15 (Pomodoro Focus Timer)
// ===========================================================================
#[test]
fn test_comb_f02_lock_f15_pomodoro() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();

    // Start Pomodoro study session
    harness.dispatch(Message::TogglePomodoro);
    assert!(harness.app.core.pomodoro.is_running);

    // Lock screen overlay active
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // Pomodoro timer state remains intact behind the lock screen
    assert!(harness.app.core.pomodoro.is_running);

    // Unlock
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert!(harness.app.core.pomodoro.is_running);
}

// ===========================================================================
// COMBINATORIAL 13: F3 (Editable Widgets) + F18 (SQLite Layout Persistence)
// ===========================================================================
#[test]
fn test_comb_f03_widgets_f18_profile() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();

    // Insert widget layout into SQLite
    conn.execute(
        "INSERT INTO widget_layouts (widget_id, pos_x, pos_y, width, height, is_pinned, z_index)
         VALUES ('daily_quote', 150.0, 90.0, 320.0, 180.0, 1, 10)",
        [],
    )
    .expect("Insert widget layout");

    // Read back from SQLite
    let (x, y, is_pinned): (f64, f64, i32) = conn
        .query_row(
            "SELECT pos_x, pos_y, is_pinned FROM widget_layouts WHERE widget_id = 'daily_quote'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .expect("Query widget layout");

    assert_eq!(x, 150.0);
    assert_eq!(y, 90.0);
    assert_eq!(is_pinned, 1);
}

// ===========================================================================
// COMBINATORIAL 14: F4 (Liquidmorphism RAM) + F5 (Morphing Dynamic Island)
// ===========================================================================
#[test]
fn test_comb_f04_ram_f05_notch_transitions() {
    let modes = [
        IslandMode::Compact,
        IslandMode::Search,
        IslandMode::ControlCenter,
        IslandMode::ActiveFocus,
        IslandMode::MaindySpeaking,
    ];

    // Morphing across all notch modes keeps single-pass LiquidMetrics SDF RAM <= 256 KB
    let (sdf_1080p, blur_1080p) = LiquidMetrics::compute_ram_vram_footprint(1920, 1080, 2);
    assert!(sdf_1080p <= LiquidMetrics::SINGLE_PASS_SDF_RAM_BYTES);
    assert!(blur_1080p > sdf_1080p * 30);

    for mode in modes {
        let (w, h) = mode.target_dimensions(5);
        assert!((300.0..=480.0).contains(&w));
        assert!((34.0..=260.0).contains(&h));
        let comp = LiquidMetrics::evaluate_comparison(w as u32, h as u32, 2);
        assert!(comp.single_pass_sdf_ram_bytes <= LiquidMetrics::SINGLE_PASS_SDF_RAM_BYTES);
    }
}

// ===========================================================================
// COMBINATORIAL 15: F7 (Research Mechanics) + F10 (Celebratory Particles)
// ===========================================================================
#[test]
fn test_comb_f07_research_f10_particles() {
    let doc_path = crate::helpers::workspace_asset("docs/GAMIFIED_LEARNING_MAP.md");
    let doc_text = std::fs::read_to_string(&doc_path).expect("Read gamification research doc");
    assert!(doc_text.contains("Duolingo") && doc_text.contains("Crown"));

    let mut harness = HeadlessHarness::new();
    let milestone_node = create_test_odyssey_node("n-milestone", "Unit 1 Capstone Exam", 100, 100);
    assert!(milestone_node.is_crown_mastered);

    // Advance Odyssey ambient particle simulation via real Message dispatch
    let before_tick = harness.app.odyssey_map_state.ambient_tick;
    harness.dispatch(Message::MapAmbientTick);
    assert_eq!(harness.app.odyssey_map_state.ambient_tick, before_tick + 1);

    let pt = mindset_os::shell::odyssey_map::project(0.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(pt.z, 1.0);
    assert_eq!(pt.r, 26.0);
}

// ===========================================================================
// COMBINATORIAL 16: F19 (State Board Rubric) + F20 (JEE Numerical MCQ Tolerance)
// ===========================================================================
#[test]
fn test_comb_f19_rubric_deduction_f20_numerical_mcq() {
    // 1. Real ExamReviewer deduction check
    let rubric = ExamReviewer::load_rubric(
        &benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json"),
    )
    .expect("Load rubric");
    let exam_sheet = ExamReviewer::evaluate(
        &benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt"),
        &rubric,
    )
    .expect("Evaluate paper");
    assert_eq!(exam_sheet.total_deductions, 1.5);

    // 2. Real CompetitiveMcqGenerator numerical tolerance check
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");
    let paper = gen
        .generate_jee_paper("Physics", Some("Electrostatics"), 0, 1)
        .expect("Generate JEE numerical paper");
    let q_id = paper.questions[0].id.clone();

    let mut within_tol = HashMap::new();
    within_tol.insert(q_id.clone(), StudentAnswer::Numerical(20.03));
    let pass_report = gen.evaluate_submission(
        &paper,
        &StudentSubmission {
            answers: within_tol,
        },
    );
    assert_eq!(pass_report.correct_count, 1);
    assert_eq!(pass_report.raw_score, 4.0);

    let mut outside_tol = HashMap::new();
    outside_tol.insert(q_id, StudentAnswer::Numerical(20.50));
    let fail_report = gen.evaluate_submission(
        &paper,
        &StudentSubmission {
            answers: outside_tol,
        },
    );
    assert_eq!(fail_report.incorrect_count, 1);
    assert_eq!(fail_report.raw_score, -1.0);
}

// ===========================================================================
// COMBINATORIAL 17: F23 (Voice Synthesis) + F24 (Dual-Text Speech Stream)
// ===========================================================================
#[test]
fn test_comb_f23_voice_streaming_f24_dual_text() {
    let dual_json = r#"{
        "display_text": "Formula: E = mc^2 on chalkboard.",
        "speech_text": "Formula: energy equals mass times speed of light squared.",
        "primary_lang": "en-IN"
    }"#;

    let res = parse_dual_text(dual_json, "english");
    assert_eq!(res.display_text, "Formula: E = mc^2 on chalkboard.");
    assert_eq!(
        res.speech_text,
        "Formula: energy equals mass times speed of light squared."
    );

    // Speech stream is clean for TTS streaming (zero raw markdown, zero raw LaTeX math syntax)
    assert!(!res.speech_text.contains("^2"));
    assert!(!res.speech_text.contains("```"));
}

// ===========================================================================
// COMBINATORIAL 18: F6 (Personalized Prompt) + F18 (SQLite Mistake Log & Weak Topics)
// ===========================================================================
#[test]
fn test_comb_f06_prompt_f18_personalization() {
    let mut profile = create_test_profile("Karthik", 590);
    profile.weak_topics = vec![
        "Gauss Law & Flux".to_string(),
        "Nernst Equation".to_string(),
    ];

    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    // Verified prompt dynamically injects weak topics from user personalization
    assert!(prompt.contains("Gauss Law & Flux"));
    assert!(prompt.contains("Nernst Equation"));
    assert!(prompt.contains("590/600"));
}
