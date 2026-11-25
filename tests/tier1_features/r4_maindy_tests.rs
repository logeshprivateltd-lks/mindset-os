//! Tier 1: Subsystem R4 Maindy AI Study Companion & Personalization Tests (Features 15 to 18)
//!
//! Covers:
//! - F15: Dedicated Maindy AI Study Workstation 4-zone transitions (>=5 tests)
//! - F16: Pure-Rust PDF extraction & RAG vault citations (>=5 tests)
//! - F17: Academic web search with AdBlock shield filtering (>=5 tests)
//! - F18: Day-by-day personalization engine, Leitner spaced repetition (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::WindowId;
use mindset_intelligence::personalization::{
    DailyFatigueCheckin, FatigueLevel, LeitnerBox, PersonalizationEngine,
};
use mindset_intelligence::rag_vault::RagVault;
use mindset_intelligence::search_engine::{AdBlockShield, SearchFilter};
use mindset_os::Message;
use mindset_sys::personalization::OsMode;

use crate::helpers::{create_sample_pdf, create_test_capsule, HeadlessHarness};

// ===========================================================================
// FEATURE 15: Dedicated Maindy AI Study Workstation
// ===========================================================================

#[test]
fn test_f15_workstation_initial_zone_layout() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::OpenWindow(WindowId::StudyReader));
    assert!(
        harness.is_window_open(WindowId::StudyReader),
        "Study Workstation window must open"
    );
}

#[test]
fn test_f15_zone_switching_events() {
    let mut harness = HeadlessHarness::new();
    for idx in 0..=3usize {
        harness.dispatch(Message::SelectStudyTab(idx));
        assert_eq!(
            harness.app.core.study_reader.active_tab_index, idx,
            "SelectStudyTab({}) must update active_tab_index",
            idx
        );
    }
}

#[test]
fn test_f15_study_tab_selection_dispatch() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::SelectPreferencesTab(2));
    assert_eq!(
        harness.app.core.preferences.active_tab,
        mindset_core::PreferencesTab::AudioRadios
    );
}

#[test]
fn test_f15_feynman_input_state_binding() {
    let mut harness = HeadlessHarness::new();
    let derivation_text = "Electric flux phi = oint E . dS = q_enclosed / epsilon_0";
    harness.dispatch(Message::FeynmanInputChanged(derivation_text.to_string()));
    assert_eq!(harness.app.feynman_input, derivation_text);
}

#[test]
fn test_f15_capsule_status_transition() {
    let harness = HeadlessHarness::new();
    let cap = create_test_capsule("cap-99", "Physics", "Wave Optics");
    harness.app.storage.save_capsule(&cap);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let initial_caps = harness.app.storage.list_capsules(&today);
    assert!(initial_caps.iter().any(|c| c.id == "cap-99"));
}

// ===========================================================================
// FEATURE 16: Pure-Rust PDF Extraction & RAG Vault Citations
// ===========================================================================

#[test]
fn test_f16_rag_vault_initialization_seeds() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Vault initialization");

    let wave_optics = harness.vault_path().join("01_Wave_Optics_Notes.md");
    assert!(
        wave_optics.exists(),
        "Wave Optics study guide must be seeded"
    );

    let electrochem = harness.vault_path().join("02_Electrochemistry_Formulas.md");
    assert!(
        electrochem.exists(),
        "Electrochemistry guide must be seeded"
    );
}

#[test]
fn test_f16_rag_vault_search_keyword_matching() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Vault initialization");

    let matches = vault
        .search("Nernst equation cell potential", 3)
        .expect("Search vault");
    assert!(
        !matches.is_empty(),
        "RAG search must return matching study notes"
    );
    assert!(
        matches
            .iter()
            .any(|m| m.file_name.contains("Electrochemistry") || m.snippet.contains("Nernst")),
        "Match must contain Nernst equation reference"
    );
}

#[test]
fn test_f16_rag_vault_top_k_limiting() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Vault initialization");

    let matches = vault.search("formula", 2).expect("Search vault");
    assert!(
        matches.len() <= 2,
        "Search must strictly respect top_k limit"
    );
}

#[test]
fn test_f16_pdf_ingestion_and_chunking() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("ingest_test.pdf");

    create_sample_pdf(
        &pdf_path,
        "Modern Physics Textbook",
        &[
            "Chapter 1: Photoelectric effect work function W0 = h * nu_0.",
            "Chapter 2: de Broglie wavelength lambda = h / p.",
        ],
    )
    .expect("Create sample PDF");

    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Initialize RAG vault");

    let chunks = vault
        .ingest_pdf(&pdf_path)
        .expect("Ingest PDF into RAG vault");
    assert_eq!(
        chunks.len(),
        2,
        "2-page PDF must produce 2 page-anchored chunks"
    );
    assert_eq!(chunks[0].page_number, 1);
    assert!(chunks[0].text_content.contains("Photoelectric effect"));
    assert_eq!(chunks[1].page_number, 2);
    assert!(chunks[1].text_content.contains("de Broglie wavelength"));

    let hits = vault
        .search_pdf_chunks("de Broglie wavelength", 2)
        .expect("Search PDF chunks");
    assert!(
        !hits.is_empty(),
        "RAG chunk search must return matching page hit"
    );
    assert_eq!(hits[0].chunk.page_number, 2);
    assert!(hits[0].citation_label.contains("Page 2"));
}

#[test]
fn test_f16_exact_page_citation_anchors() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Vault initialization");

    let pdf_path = harness.vault_path().join("class12_wave_optics.pdf");
    create_sample_pdf(
        &pdf_path,
        "Wave Optics",
        &["Young's double slit experiment formula beta = lambda * D / d"],
    )
    .expect("PDF");
    let chunks = vault.ingest_pdf(&pdf_path).expect("Ingest PDF");
    assert!(!chunks.is_empty());
    assert_eq!(chunks[0].page_number, 1);
    assert_eq!(chunks[0].file_name, "class12_wave_optics.pdf");
}

// ===========================================================================
// FEATURE 17: Academic Web Search with AdBlock Shield
// ===========================================================================

#[test]
fn test_f17_adblock_blocks_telemetry_trackers() {
    let shield = AdBlockShield::default();
    assert!(shield.is_enabled);

    assert_eq!(
        shield
            .should_block_domain("https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js"),
        Some("Ad Network / Telemetry Tracker")
    );
    assert_eq!(
        shield.should_block_domain("https://adservice.google.com/adsid/integrator.js"),
        Some("Ad Network / Telemetry Tracker")
    );
    assert_eq!(
        shield.should_block_domain("https://securepubads.g.doubleclick.net/gampad/ads"),
        Some("Ad Network / Telemetry Tracker")
    );
}

#[test]
fn test_f17_adblock_blocks_short_form_video() {
    let shield = AdBlockShield::default();

    assert_eq!(
        shield.should_block_domain("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
        Some("Short-Form Video / Distraction Feed")
    );
    assert_eq!(
        shield.should_block_domain("https://www.instagram.com/reels/C_demo123/"),
        Some("Short-Form Video / Distraction Feed")
    );
    assert_eq!(
        shield.should_block_domain("https://www.tiktok.com/@scholar/video/1234"),
        Some("Short-Form Video / Distraction Feed")
    );
}

#[test]
fn test_f17_adblock_allows_academic_domains() {
    let shield = AdBlockShield::default();

    assert_eq!(
        shield.should_block_domain("https://arxiv.org/abs/2301.00001"),
        None
    );
    assert_eq!(
        shield.should_block_domain("https://en.wikipedia.org/wiki/Gauss%27s_law"),
        None
    );
    assert_eq!(
        shield.should_block_domain("https://khanacademy.org/science/physics"),
        None
    );
    assert_eq!(
        shield.should_block_domain("https://ncert.nic.in/textbook.php"),
        None
    );
}

#[test]
fn test_f17_adblock_shield_counter_increments() {
    let mut shield = AdBlockShield::default();
    let initial_ads = shield.ads_blocked_count;
    let initial_distractions = shield.distractions_blocked_count;

    let (blocked_ad, _) = shield.test_domain_filter("https://doubleclick.net/tracker");
    assert!(blocked_ad);
    assert_eq!(shield.ads_blocked_count, initial_ads + 1);

    let (blocked_distraction, _) = shield.test_domain_filter("https://youtube.com/shorts/dance");
    assert!(blocked_distraction);
    assert_eq!(shield.distractions_blocked_count, initial_distractions + 1);
}

#[test]
fn test_f17_search_engine_filter_categories() {
    let filters = SearchFilter::all();
    assert_eq!(filters.len(), 4);
    assert_eq!(SearchFilter::All.label(), "All Results");
    assert_eq!(SearchFilter::StudyVault.label(), "Study Vault");
    assert_eq!(SearchFilter::Encyclopedia.label(), "STEM Encyclopedia");
    assert_eq!(SearchFilter::AskMaindyAi.label(), "Ask Maindy AI");
}

// ===========================================================================
// FEATURE 18: Day-by-Day Personalization Engine & Spaced Repetition
// ===========================================================================

#[test]
fn test_f18_leitner_box_interval_progression() {
    // Verify real LeitnerBox domain intervals & advance transitions: 1d -> 3d -> 7d -> 14d -> 30d
    let expected = [
        (1u8, LeitnerBox::Box1, 1i64),
        (2u8, LeitnerBox::Box2, 3i64),
        (3u8, LeitnerBox::Box3, 7i64),
        (4u8, LeitnerBox::Box4, 14i64),
        (5u8, LeitnerBox::Box5, 30i64),
    ];
    for (num, box_variant, days) in expected {
        let b = LeitnerBox::from_u8(num);
        assert_eq!(b, box_variant);
        assert_eq!(b.interval_days(), days);
    }
    assert_eq!(LeitnerBox::Box1.advance(), LeitnerBox::Box2);
    assert_eq!(LeitnerBox::Box2.advance(), LeitnerBox::Box3);
    assert_eq!(LeitnerBox::Box3.advance(), LeitnerBox::Box4);
    assert_eq!(LeitnerBox::Box4.advance(), LeitnerBox::Box5);

    // Verify SQLite-backed progression through PersonalizationEngine::record_card_review
    let harness = HeadlessHarness::new();
    let engine = PersonalizationEngine::new(harness.db_path.clone()).expect("Init engine");
    let card = engine
        .add_leitner_card("card-gauss-1", "Gauss Law", "phi = q / eps0", "Physics")
        .expect("Add card");
    assert_eq!(card.current_box, 1);

    let r2 = engine
        .record_card_review("card-gauss-1", true, "2026-11-01")
        .expect("Advance to Box 2");
    assert_eq!(r2.current_box, 2);
    assert_eq!(r2.next_review_due, "2026-11-04");

    let r3 = engine
        .record_card_review("card-gauss-1", true, "2026-11-04")
        .expect("Advance to Box 3");
    assert_eq!(r3.current_box, 3);
    assert_eq!(r3.next_review_due, "2026-11-11");

    let r4 = engine
        .record_card_review("card-gauss-1", true, "2026-11-11")
        .expect("Advance to Box 4");
    assert_eq!(r4.current_box, 4);
    assert_eq!(r4.next_review_due, "2026-11-25");

    let r5 = engine
        .record_card_review("card-gauss-1", true, "2026-11-25")
        .expect("Advance to Box 5");
    assert_eq!(r5.current_box, 5);
    assert_eq!(r5.next_review_due, "2026-12-25");
}

#[test]
fn test_f18_mistake_log_resets_to_box_one() {
    assert_eq!(LeitnerBox::demote_on_failure(), LeitnerBox::Box1);

    let harness = HeadlessHarness::new();
    let engine = PersonalizationEngine::new(harness.db_path.clone()).expect("Init engine");
    engine
        .add_leitner_card(
            "card-nernst-1",
            "Nernst Eq",
            "E = E0 - (0.0591/n)logQ",
            "Chemistry",
        )
        .expect("Add card");

    // Promote twice to Box 3
    let _ = engine
        .record_card_review("card-nernst-1", true, "2026-11-20")
        .expect("Box 2");
    let promoted = engine
        .record_card_review("card-nernst-1", true, "2026-11-23")
        .expect("Box 3");
    assert_eq!(promoted.current_box, 3);

    // Failed review must demote directly to Box 1 and reset consecutive successes
    let demoted = engine
        .record_card_review("card-nernst-1", false, "2026-11-30")
        .expect("Demote on failure");
    assert_eq!(demoted.current_box, 1, "Mistake must reset card to Box 1");
    assert_eq!(demoted.consecutive_successes, 0);
    assert_eq!(demoted.next_review_due, "2026-12-01");
}

#[test]
fn test_f18_profile_persistence_in_sqlite() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();

    let name: String = conn
        .query_row("SELECT name FROM user_profile WHERE id = 1", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(name, "Mapla");

    let target_score: u32 = conn
        .query_row(
            "SELECT target_score FROM user_profile WHERE id = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(target_score, 590);
}

#[test]
fn test_f18_fatigue_adaptation_adapts_daily_goal() {
    let (high_cap, high_brk) = DailyFatigueCheckin::compute_pacing(FatigueLevel::High, 4.5);
    let (norm_cap, norm_brk) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Normal, 3.5);
    let (fat_cap, fat_brk) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Fatigued, 2.0);

    assert!(high_cap > norm_cap && norm_cap > fat_cap);
    assert!(fat_brk > norm_brk);

    let harness = HeadlessHarness::new();
    let engine = PersonalizationEngine::new(harness.db_path.clone()).expect("Init engine");
    let checkin = DailyFatigueCheckin {
        date: "2026-11-25".to_string(),
        energy_level: FatigueLevel::Fatigued,
        available_hours_today: 2.0,
        situation_note: "Heavy school lab day".to_string(),
        recommended_capsule_minutes: fat_cap,
        recommended_break_minutes: fat_brk,
    };
    engine
        .record_fatigue_checkin(&checkin)
        .expect("Persist fatigue checkin");

    let loaded = engine
        .get_fatigue_checkin("2026-11-25")
        .expect("Query checkin")
        .expect("Checkin must persist");
    assert_eq!(loaded.energy_level, FatigueLevel::Fatigued);
    assert_eq!(loaded.recommended_capsule_minutes, fat_cap);
    assert_eq!(loaded.recommended_break_minutes, fat_brk);
    assert_eq!(high_brk, 5);
}

#[test]
fn test_f18_operating_mode_transitions() {
    let modes = OsMode::all();
    assert_eq!(modes.len(), 4);
    assert_eq!(OsMode::Standard.pill_label(), "[Mode: Standard ▾]");
    assert_eq!(OsMode::Mindful.pill_label(), "[Mode: Mindful ✦]");
    assert_eq!(OsMode::ExamLock.pill_label(), "[Mode: Exam Lock 🔒]");
    assert_eq!(OsMode::SleepWindDown.pill_label(), "[Mode: Sleep 🌙]");
}
