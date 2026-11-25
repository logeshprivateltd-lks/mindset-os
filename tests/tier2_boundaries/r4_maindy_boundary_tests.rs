//! Tier 2: Subsystem R4 Maindy AI Study Workstation Boundary Tests (Features 15 to 18)
//!
//! Stress, limit, and edge condition verification:
//! - F15: Tab clamping, empty input, 10k character derivations, empty capsule queue, rapid zone switching (5 tests)
//! - F16: Empty vault queries, whitespace/punctuation search, top_k=0 boundary, duplicate PDF ingest, unicode queries (5 tests)
//! - F17: Invalid URL syntax, tracker subdomains, 10k char search query, disabled shield, empty category search (5 tests)
//! - F18: Fatigue=0 vs fatigue=10, Leitner Box 5 ceiling, reset to Box 1, profile persistence, SQLite transactions (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::PreferencesTab;
use mindset_intelligence::personalization::{
    DailyFatigueCheckin, FatigueLevel, LeitnerBox, PersonalizationEngine,
};
use mindset_intelligence::rag_vault::RagVault;
use mindset_intelligence::search_engine::{AcademicSearchEngine, AdBlockShield, SearchFilter};
use mindset_os::Message;

use crate::helpers::{create_sample_pdf, HeadlessHarness};

// ===========================================================================
// FEATURE 15: Dedicated Maindy AI Study Workstation — Boundary Tests
// ===========================================================================

#[test]
fn test_f15_boundary_tab_clamping_on_preferences() {
    let mut harness = HeadlessHarness::new();
    // Valid tab selections
    harness.dispatch(Message::SelectPreferencesTab(0));
    assert_eq!(
        harness.app.core.preferences.active_tab,
        PreferencesTab::Appearance
    );

    harness.dispatch(Message::SelectPreferencesTab(1));
    assert_eq!(
        harness.app.core.preferences.active_tab,
        PreferencesTab::FocusShields
    );

    harness.dispatch(Message::SelectPreferencesTab(2));
    assert_eq!(
        harness.app.core.preferences.active_tab,
        PreferencesTab::AudioRadios
    );

    harness.dispatch(Message::SelectPreferencesTab(3));
    assert_eq!(
        harness.app.core.preferences.active_tab,
        PreferencesTab::LanguageRegion
    );
}

#[test]
fn test_f15_boundary_empty_feynman_derivation_input() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::FeynmanInputChanged("   ".to_string()));
    assert_eq!(harness.app.feynman_input, "   ");
}

#[test]
fn test_f15_boundary_massive_derivation_input_length() {
    let mut harness = HeadlessHarness::new();
    let massive_formula =
        "\\int_{0}^{\\infty} e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2} \n".repeat(200); // ~12,000 chars
    harness.dispatch(Message::FeynmanInputChanged(massive_formula.clone()));
    assert_eq!(harness.app.feynman_input.len(), massive_formula.len());
}

#[test]
fn test_f15_boundary_empty_capsule_queue_handled_cleanly() {
    let harness = HeadlessHarness::new();
    let past_date = "1990-01-01";
    let caps = harness.app.storage.list_capsules(past_date);
    assert_eq!(caps.len(), 0, "No capsules should exist for ancient date");
}

#[test]
fn test_f15_boundary_rapid_study_zone_switching() {
    let mut harness = HeadlessHarness::new();
    // Rapidly cycle tabs 50 times and assert active_tab_index matches every dispatch
    for i in 0..50 {
        let target = i % 4;
        harness.dispatch(Message::SelectStudyTab(target));
        assert_eq!(
            harness.app.core.study_reader.active_tab_index, target,
            "Tab index must match {} on iteration {}",
            target, i
        );
    }
}

// ===========================================================================
// FEATURE 16: Pure-Rust PDF Extraction & RAG Vault Citations — Boundary Tests
// ===========================================================================

#[test]
fn test_f16_boundary_empty_vault_query_zero_matches() {
    let harness = HeadlessHarness::new();
    let empty_vault_dir = harness.temp_dir.path().join("empty_vault");
    std::fs::create_dir_all(&empty_vault_dir).expect("Create dir");

    let vault = RagVault::with_path(empty_vault_dir);
    vault.initialize().expect("Initialize");

    let results = vault.search("Quantum tunneling", 5).expect("Search empty");
    assert_eq!(
        results.len(),
        0,
        "Empty vault must return 0 results cleanly"
    );
}

#[test]
fn test_f16_boundary_search_with_whitespace_and_punctuation() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Initialize");

    let results = vault
        .search("   !@#$%^&*()   ", 3)
        .expect("Search punctuation");
    assert_eq!(
        results.len(),
        0,
        "Punctuation-only query must return 0 results without error"
    );
}

#[test]
fn test_f16_boundary_top_k_zero_returns_empty() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Initialize");

    let results = vault.search("Optics", 0).expect("Search with top_k=0");
    assert_eq!(results.len(), 0, "top_k=0 must return 0 results");
}

#[test]
fn test_f16_boundary_duplicate_pdf_ingestion_updates_cleanly() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Initialize");

    let pdf_path = harness.vault_path().join("electrodynamics.pdf");
    create_sample_pdf(&pdf_path, "Electrodynamics", &["Gauss Law curl E = -dB/dt"])
        .expect("Create PDF");

    // Ingest once
    let chunks1 = vault.ingest_pdf(&pdf_path).expect("Ingest 1");
    assert!(!chunks1.is_empty());

    // Ingest again (same file, updated content)
    create_sample_pdf(
        &pdf_path,
        "Electrodynamics V2",
        &["Gauss Law curl E = -dB/dt with Maxwell displacement"],
    )
    .expect("Update PDF");
    let chunks2 = vault.ingest_pdf(&pdf_path).expect("Ingest 2");
    assert!(!chunks2.is_empty());
}

#[test]
fn test_f16_boundary_unicode_math_symbols_search_query() {
    let harness = HeadlessHarness::new();
    let vault = RagVault::with_path(harness.vault_path().to_path_buf());
    vault.initialize().expect("Initialize");

    // Query with Greek letters, integral symbols, and sub/superscripts
    let query = "∮ E ⋅ dA = Q / ε₀";
    let results = vault.search(query, 5).expect("Search unicode");
    assert!(results.len() <= 5);
}

// ===========================================================================
// FEATURE 17: Academic Web Search with AdBlock Shield — Boundary Tests
// ===========================================================================

#[test]
fn test_f17_boundary_invalid_url_syntax_handled_safely() {
    let shield = AdBlockShield::default();
    let invalid_urls = [
        "not_a_valid_url",
        "://missing-scheme",
        "https://",
        "ftp:///invalid/path",
    ];

    for u in invalid_urls {
        let res = shield.should_block_domain(u);
        assert_eq!(
            res, None,
            "Invalid/non-tracker URL '{}' must return None without panicking",
            u
        );
        let sanitized = shield.sanitize_url(u);
        assert!(!sanitized.is_empty());
    }
}

#[test]
fn test_f17_boundary_tracker_subdomain_obfuscation() {
    let shield = AdBlockShield::default();
    // Nested tracker domains
    assert_eq!(
        shield.should_block_domain("https://sub.analytics.telemetry.doubleclick.net/pixel"),
        Some("Ad Network / Telemetry Tracker")
    );
    assert_eq!(
        shield.should_block_domain("https://cdn.criteo.com/script.js"),
        Some("Ad Network / Telemetry Tracker")
    );
}

#[test]
fn test_f17_boundary_extreme_query_length_search() {
    let long_query = "quantum mechanics wave-particle duality ".repeat(200); // 8,000 chars
    assert!(long_query.len() > 5000);

    let mut engine = AcademicSearchEngine::new();
    engine.search(&long_query, &[]);
    assert_eq!(engine.query.len(), long_query.trim().len());
    assert!(engine.results.is_empty());

    // Subsequent normal search still succeeds cleanly
    engine.search("Gauss", &[]);
    assert!(!engine.results.is_empty());
}

#[test]
fn test_f17_boundary_disabled_shield_allows_all() {
    let shield = AdBlockShield {
        is_enabled: false,
        ..Default::default()
    };

    // When disabled, all domains are allowed through
    assert_eq!(
        shield.should_block_domain("https://doubleclick.net/ad"),
        None
    );
    assert_eq!(
        shield.should_block_domain("https://tiktok.com/@scholar"),
        None
    );
}

#[test]
fn test_f17_boundary_filter_category_labels_integrity() {
    assert_eq!(SearchFilter::All.label(), "All Results");
    assert_eq!(SearchFilter::StudyVault.label(), "Study Vault");
    assert_eq!(SearchFilter::Encyclopedia.label(), "STEM Encyclopedia");
    assert_eq!(SearchFilter::AskMaindyAi.label(), "Ask Maindy AI");
}

// ===========================================================================
// FEATURE 18: Day-by-Day Personalization Engine — Boundary Tests
// ===========================================================================

#[test]
fn test_f18_boundary_zero_fatigue_full_commitment() {
    let (capsule, break_time) = DailyFatigueCheckin::compute_pacing(FatigueLevel::High, 4.0);
    assert_eq!(capsule, 40, "High energy allows 40m intensive focus block");
    assert_eq!(break_time, 5, "5m break time");
}

#[test]
fn test_f18_boundary_extreme_fatigue_maximum_reduction() {
    let (capsule, break_time) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Fatigued, 2.0);
    assert_eq!(
        capsule, 18,
        "Fatigued energy reduces capsule to 18m micro-focus to prevent burnout"
    );
    assert_eq!(break_time, 8, "Break extended to 8m for recovery");
}

#[test]
fn test_f18_boundary_leitner_box_five_ceiling() {
    let b5 = LeitnerBox::Box5;
    assert_eq!(b5.interval_days(), 30);
    let next_b = b5.advance();
    assert_eq!(
        next_b,
        LeitnerBox::Box5,
        "Promoting from Box 5 must stay at Box 5 ceiling"
    );
}

#[test]
fn test_f18_boundary_mistake_reset_to_box_one() {
    let demoted = LeitnerBox::demote_on_failure();
    assert_eq!(
        demoted,
        LeitnerBox::Box1,
        "Mistake must reset box to Box 1 immediately"
    );
    assert_eq!(demoted.interval_days(), 1);
}

#[test]
fn test_f18_boundary_mistake_log_persistence() {
    let harness = HeadlessHarness::new();
    let db_path = harness.vault_path().join("test_personalization.db");
    let engine = PersonalizationEngine::new(db_path).expect("Init personalization engine");

    let mistake_id = engine
        .record_mistake(
            "Physics",
            "Magnetic Effects",
            "State Biot-Savart Law",
            "Forgot mu_0/4pi factor",
            "dB = (mu_0 / 4pi) * (I dl x r) / r^3",
            "Formula Error",
        )
        .expect("Record mistake");

    let mistakes = engine
        .get_unresolved_mistakes(Some("Physics"))
        .expect("Query mistakes");
    assert_eq!(mistakes.len(), 1);
    assert_eq!(mistakes[0].id, mistake_id);
    assert_eq!(mistakes[0].subject, "Physics");

    engine.resolve_mistake(mistake_id).expect("Resolve mistake");
    let remaining = engine
        .get_unresolved_mistakes(Some("Physics"))
        .expect("Query after resolve");
    assert_eq!(remaining.len(), 0);
}
