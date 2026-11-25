//! Tier 4: Student Workflows (Scenarios S1 to S8)
//!
//! Realistic multi-step end-to-end user journeys through MindSet OS.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use mindset_core::agent::{
    CompetitiveMcqGenerator, ExamReviewer, StudentAnswer, StudentSubmission,
};
use mindset_core::{ActiveOverlay, DesktopWidgetId};
use mindset_intelligence::personalization::{
    DailyFatigueCheckin, FatigueLevel, LeitnerBox, PersonalizationEngine,
};
use mindset_intelligence::rag_vault::RagVault;
use mindset_os::ai::persona::{parse_dual_text, AcademicIntent, SubjectEntity, ThanglishParser};
use mindset_os::shell::dynamic_island::IslandMode;
use mindset_os::theme::ThemePalette;
use mindset_os::Message;
use mindset_sys::fs::document::{DocumentContent, DocumentHandler};
use mindset_sys::fs::explorer::list_directory;

use crate::helpers::{
    benchmark_dir, create_sample_pdf, create_sample_pptx, create_test_odyssey_node,
    create_test_profile, HeadlessHarness, AUTH_LOCK,
};

// ===========================================================================
// SCENARIO S1: Morning Lock & Study Wakeup
// ===========================================================================
// Exercised Features: F1 (Cosmic Shell), F2 (Lock Auth), F3 (Editable Widgets),
//                     F5 (Liquid Notch), F18 (Fatigue Personalization)
#[test]
fn test_scenario_s1_morning_lock_and_study_wakeup() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    let id = DesktopWidgetId::ScholarBriefing;

    // 1. System boots with LockScreen overlay active
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // 2. Student enters PIN "123456" to unlock
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert_eq!(harness.app.lock_screen.failed_attempts, 0);

    // 3. Desktop appears with custom wallpaper & dark cyber-neon palette
    let palette = ThemePalette::for_mode(harness.app.core.preferences.theme);
    assert!(palette.glass_surface.a > 0.50);

    // 4. Student arranges desktop widgets
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.x = 100.0;
        w.y = 60.0;
        w.is_pinned = true;
    }
    let quote_widget = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert_eq!(quote_widget.x, 100.0);
    assert!(quote_widget.is_pinned);

    // 5. Daily fatigue check-in: student reports normal energy (4.0 hours)
    let (capsule, break_time) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Normal, 4.0);
    assert_eq!(capsule, 30);
    assert_eq!(break_time, 5);

    // 6. Notch reflects compact glanceable morning state
    let (w, h) = IslandMode::Compact.target_dimensions(0);
    assert_eq!(w, 300.0);
    assert_eq!(h, 34.0);
}

// ===========================================================================
// SCENARIO S2: Physics Chapter Odyssey Progression
// ===========================================================================
// Exercised Features: F7 (Research Docs), F8 (Multi-Chapter Units),
//                     F9 (Crown Mastery), F10 (3D Progression Path)
#[test]
fn test_scenario_s2_physics_chapter_odyssey_progression() {
    // 1. Verify research documents underpin progression logic
    let doc_path = Path::new("docs/GAMIFIED_LEARNING_MAP.md");
    let fallback_path = Path::new("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md");
    assert!(doc_path.exists() || fallback_path.exists());

    // 2. Student navigates Unit 1: Electromagnetism & Potentials
    let mut node = create_test_odyssey_node("node-gauss-law", "Gauss's Law & Electric Flux", 0, 70);
    node.unit_id = "unit-1-electromagnetism".to_string();
    assert_eq!(node.stars_earned, 0);
    assert!(!node.is_crown_mastered);

    // 3. Student completes practice test and scores 69/70 (98.57%)
    node.update_evaluation(69);

    // 4. Milestone awards 3 stars and Golden Crown (>=98%)
    assert_eq!(node.mastery_percentage(), 99);
    assert_eq!(node.stars_earned, 3);
    assert!(node.is_crown_mastered);

    // 5. 3D path projection renders completed milestone at near-plane foreground
    let projection = mindset_os::shell::odyssey_map::project(0.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(projection.z, 1.0);
    assert_eq!(projection.r, 26.0);
}

// ===========================================================================
// SCENARIO S3: Textbook Ingestion & Page Citation Q&A
// ===========================================================================
// Exercised Features: F11 (Filesystem), F12 (PDF Viewer), F16 (RAG Vault),
//                     F15 (Maindy Study Workstation)
#[test]
fn test_scenario_s3_textbook_ingestion_and_citation_qa() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("tn_physics_class12_vol1.pdf");

    // 1. Student downloads/places textbook PDF into vault folder
    let page1_text = "Chapter 1: Electrostatics. Coulomb's Law states that electrostatic force between two point charges is proportional to product of charges.";
    let page2_text = "Electric Field and Dipoles: Torque tau = p x E. Work done in rotating dipole W = pE(1 - cos theta).";
    create_sample_pdf(&pdf_path, "TN Physics Vol 1", &[page1_text, page2_text])
        .expect("Create PDF");

    // 2. Inspects directory hierarchy in File Explorer
    let conn = harness.open_test_conn();
    let listing = list_directory(harness.vault_path(), &conn, None).expect("List dir");
    assert!(listing
        .entries
        .iter()
        .any(|e| e.name == "tn_physics_class12_vol1.pdf"));

    // 3. DocumentHandler opens and validates 2 pages
    let doc = DocumentHandler::read(&pdf_path).expect("Read PDF");
    assert_eq!(doc.count(), 2);

    // 4. Ingests into Maindy RAG Vault
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Init vault");
    let chunks = vault.ingest_pdf(&pdf_path).expect("Ingest PDF");
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].page_number, 1);
    assert_eq!(chunks[1].page_number, 2);

    // 5. Student asks question about dipoles in Study Workstation
    let results = vault.search("Torque tau = p x E", 1).expect("Search");
    assert!(!results.is_empty());
    assert_eq!(results[0].file_name, "tn_physics_class12_vol1.pdf");
}

// ===========================================================================
// SCENARIO S4: Presentation Revision & Slide Update
// ===========================================================================
// Exercised Features: F11 (Filesystem), F13 (PPTX Parser), F14 (In-Place Editor)
#[test]
fn test_scenario_s4_presentation_revision_and_slide_update() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("chemistry_electrochem.pptx");

    // 1. Student has seminar presentation
    create_sample_pptx(
        &pptx_path,
        "Electrochemistry Seminar",
        &[
            (
                "Slide 1: Galvanic Cells",
                "Zinc-Copper Daniell cell. E_cell = 1.10 V.",
            ),
            ("Slide 2: Nernst Equation", "Old draft formula text"),
        ],
    )
    .expect("Create PPTX");

    // 2. Opens presentation in Document Viewer
    let doc = DocumentHandler::read(&pptx_path).expect("Read PPTX");
    if let DocumentContent::Pptx {
        mut slides,
        metadata,
    } = doc
    {
        assert_eq!(slides.len(), 2);

        // 3. Student edits Slide 2 in-place with rigorous Nernst equation
        slides[1].body_blocks =
            vec!["E_cell = E0_cell - (0.0591 / n) * log10(Q) at 298 K".to_string()];
        DocumentHandler::save_update(&pptx_path, &DocumentContent::Pptx { slides, metadata })
            .expect("Save in-place");
    }

    // 4. Re-opens from filesystem to verify persistent update
    let updated_doc = DocumentHandler::read(&pptx_path).expect("Re-read PPTX");
    match updated_doc {
        DocumentContent::Pptx { slides, .. } => {
            assert!(slides[1].body_blocks[0].contains("E_cell = E0_cell - (0.0591 / n)"));
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

// ===========================================================================
// SCENARIO S5: State Board Strict Exam Paper Evaluation
// ===========================================================================
// Exercised Features: F12 (Document Viewer), F16 (RAG Vault), F19 (590/600 Exam Reviewer),
//                     F21 (Workspace Verification Benchmarks)
#[test]
fn test_scenario_s5_state_board_strict_exam_evaluation() {
    // 1. Load State Board 590/600 strict rubric benchmark via ExamReviewer
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load State Board rubric");
    assert_eq!(rubric.board, "Tamil Nadu State Board");

    // 2. Load student exam paper with genuine structural mistakes (zero marker comments)
    let deductions_paper_path =
        benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt");
    let paper_text = fs::read_to_string(&deductions_paper_path).expect("Read student paper");
    assert!(!paper_text.contains("Rule DED-"));

    // 3. Evaluate via genuine ExamReviewer engine
    let sheet = ExamReviewer::evaluate(&deductions_paper_path, &rubric)
        .expect("Evaluate student exam paper");
    assert_eq!(sheet.total_score, 16.5);
    assert_eq!(sheet.max_marks, 18.0);
    assert_eq!(sheet.total_deductions, 1.5);
    assert!(
        !sheet.achieved_590_crown,
        "91.67% misses 98% State Board Crown"
    );
    assert!((90.0..98.0).contains(&sheet.percentage));
    assert!(!sheet.red_pen_annotations.is_empty());
    assert!(!sheet.gap_analysis_590.actionable_recommendations.is_empty());
}

// ===========================================================================
// SCENARIO S6: Competitive Exam MCQ Drill Session
// ===========================================================================
// Exercised Features: F15 (Study Workstation), F20 (MCQ Generator),
//                     F21 (Benchmarks), F18 (Personalization & Mistake Log)
#[test]
fn test_scenario_s6_competitive_mcq_drill_session() {
    let harness = HeadlessHarness::new();
    let db_path = harness.vault_path().join("s6_personalization.db");
    let engine = PersonalizationEngine::new(db_path).expect("Init engine");

    // 1. Instantiate CompetitiveMcqGenerator & load workspace benchmarks
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load MCQ benchmarks");

    // 2. Generate 3-question JEE Physics Rotational Dynamics drill (2 SingleCorrect + 1 Numerical)
    let paper = gen
        .generate_jee_paper("Physics", Some("Rotational"), 2, 1)
        .expect("Generate 3-question JEE paper");
    assert_eq!(paper.total_questions, 3);

    // 3. Construct real StudentSubmission: Q1 correct (+4), Q2 wrong (-1), Q3 numerical correct (+4)
    let mut answers = HashMap::new();
    for (i, q) in paper.questions.iter().enumerate() {
        if let Some(correct_idx) = q.correct_option_index {
            let chosen = if i == 1 {
                (correct_idx + 1) % 4
            } else {
                correct_idx
            };
            answers.insert(q.id.clone(), StudentAnswer::Option(chosen));
        } else if let Some(val) = q.numerical_correct_value {
            answers.insert(q.id.clone(), StudentAnswer::Numerical(val));
        }
    }

    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert_eq!(report.correct_count, 2);
    assert_eq!(report.incorrect_count, 1);
    assert_eq!(report.raw_score, 7.0);

    // 4. Feed incorrect QuestionResult into PersonalizationEngine mistake log & Leitner Box 1
    for res in report.question_results.iter().filter(|r| !r.is_correct) {
        engine
            .record_mistake(
                "Physics",
                &res.topic,
                &res.question_id,
                &res.student_answer,
                &res.correct_answer,
                "Calculation / Concept Error",
            )
            .expect("Record mistake");
        engine
            .add_leitner_card(&res.question_id, &res.topic, &res.correct_answer, "Physics")
            .expect("Create Leitner card");
        let demoted = engine
            .record_card_review(&res.question_id, false, "2026-11-25")
            .expect("Demote card");
        assert_eq!(demoted.current_box, 1);
    }

    let mistakes = engine
        .get_unresolved_mistakes(Some("Physics"))
        .expect("Query mistakes");
    assert_eq!(mistakes.len(), 1);
    assert_eq!(LeitnerBox::demote_on_failure().interval_days(), 1);
}

// ===========================================================================
// SCENARIO S7: Conversational Thanglish Voice Assistant
// ===========================================================================
// Exercised Features: F6 (Persona Prompt), F22 (Thanglish NLP),
//                     F23 (Voice Synthesis), F24 (Dual-Text Generation)
#[test]
fn test_scenario_s7_conversational_thanglish_voice_assistant() {
    let student_query = "hey brow , bro p6 la chapter 10 la 10.6 example solli kudu bro";

    // 1. NLP extracts subject, chapter, example, and academic intent
    let entities = ThanglishParser::parse(student_query);
    assert_eq!(entities.subject, Some(SubjectEntity::Physics));
    assert_eq!(entities.chapter, Some(10));
    assert_eq!(entities.example.as_deref(), Some("10.6"));
    assert_eq!(entities.intent, AcademicIntent::TeachConcept);

    // 2. Dual-Text Generator produces formatted math for screen + speech text for voice
    let raw_ai_reply = r#"DISPLAY: # Example 10.6: Wave Optics
Given: Wavelength \lambda = 500\text{ nm}, Slit distance d = 1\text{ mm}, Screen distance D = 1\text{ m}.
Fringe width \beta = \frac{\lambda D}{d} = \frac{500 \times 10^{-9} \times 1}{1 \times 10^{-3}} = 0.5\text{ mm}.

SPEECH: Vanakkam da mapla! Example 10.6 romba simple. Fringe width formula beta equals lambda D by d. Values substitute panna 0.5 millimeter varum da."#;

    let dual = parse_dual_text(raw_ai_reply, "thanglish");
    assert!(dual.display_text.contains(r#"\beta = \frac{\lambda D}{d}"#));
    assert!(dual.speech_text.contains("beta equals lambda"));
    assert_eq!(dual.primary_lang, "ta-IN");

    // 3. Speech stream is clean and formatted for audio synthesis buffer
    assert!(!dual.speech_text.contains(r#"\frac"#));
}

// ===========================================================================
// SCENARIO S8: Full Student Day-in-the-Life Workload (F1 to F24)
// ===========================================================================
// Comprehensive integration pipeline:
// Unlock -> Setup Widgets -> Study Odyssey Milestone -> Ingest Textbook PDF ->
// Ask Thanglish Q&A -> Submit Exam Paper for Strict Evaluation -> Persist Mistake Log
#[test]
fn test_scenario_s8_full_student_day_in_the_life_workload() {
    let mut harness = HeadlessHarness::new();

    // STEP 1: Morning Unlock
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);

    // STEP 2: Configure Workspace Widgets & Liquid Notch
    let id = DesktopWidgetId::ScholarBriefing;
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.x = 100.0;
        w.y = 80.0;
        w.is_pinned = true;
    }
    assert_eq!(IslandMode::Compact.target_dimensions(0), (300.0, 34.0));

    // STEP 3: Complete Odyssey Milestone (Snell's Law)
    let mut odyssey_node =
        create_test_odyssey_node("n-day-1", "Snell's Law & Internal Reflection", 99, 100);
    odyssey_node.update_evaluation(99);
    assert!(odyssey_node.is_crown_mastered);
    assert_eq!(odyssey_node.stars_earned, 3);

    // STEP 4: Ingest Textbook PDF into RAG Vault
    let textbook_path = harness.vault_path().join("class12_physics_full.pdf");
    create_sample_pdf(
        &textbook_path,
        "Class 12 Physics",
        &["Chapter 10: Wave Optics. Interference and diffraction fringes."],
    )
    .expect("Create PDF");

    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Init vault");
    let chunks = vault.ingest_pdf(&textbook_path).expect("Ingest PDF");
    assert_eq!(chunks[0].page_number, 1);

    // STEP 5: Thanglish Voice Interaction
    let query = "p6 la chapter 10 solli kudu mapla";
    let entities = ThanglishParser::parse(query);
    assert_eq!(entities.subject, Some(SubjectEntity::Physics));
    assert_eq!(entities.chapter, Some(10));

    let ai_response =
        "DISPLAY: Wave optics interference logic.\nSPEECH: Wave optics romba easy da mapla.";
    let dual = parse_dual_text(ai_response, "thanglish");
    assert_eq!(dual.primary_lang, "ta-IN");

    // STEP 6: Strict 590/600 Exam Grading via real ExamReviewer
    let rubric = ExamReviewer::load_rubric(
        &benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json"),
    )
    .expect("Load State Board rubric");
    let exam_sheet = ExamReviewer::evaluate(
        &benchmark_dir().join("sample_exams/tn_class12_physics_590.txt"),
        &rubric,
    )
    .expect("Evaluate 590 paper");
    assert_eq!(exam_sheet.total_score, 18.0);
    assert_eq!(exam_sheet.percentage, 100.0);
    assert!(
        exam_sheet.achieved_590_crown,
        "Protected 590/600 trajectory maintained"
    );

    // STEP 7: Nightly Persistence in SQLite
    let conn = harness.open_test_conn();
    let profile = create_test_profile("Mapla", 590);
    conn.execute(
        "UPDATE user_profile SET target_score = ?1 WHERE id = 1",
        [profile.target_score_value],
    )
    .expect("Persist profile");

    let saved_target: u32 = conn
        .query_row(
            "SELECT target_score FROM user_profile WHERE id = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(saved_target, 590);
}
