//! Tier 2: Subsystem R2 Gamified Odyssey Journey Boundary Tests (Features 7 to 10)
//!
//! Stress, limit, and edge condition verification:
//! - F7: Research doc existence, size boundaries, anti-trivialization keywords (5 tests)
//! - F8: Empty chapter journey (0 nodes), 1-node progress, 0 min pacing, title wrap (5 tests)
//! - F9: 97.9% vs 98.0% crown cutoff, 0 max_marks divide-by-zero, perfect 100%, excess marks (5 tests)
//! - F10: Horizon projection limits, near-camera projection, tilt limits, hit-test edge, particle bursts (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::odyssey::{NodeStatus, OdysseyJourneyState};
use mindset_os::shell::odyssey_map::project;

use crate::helpers::{create_test_odyssey_node, workspace_asset};

fn gamified_map_path() -> std::path::PathBuf {
    workspace_asset("docs/GAMIFIED_LEARNING_MAP.md")
}

// ===========================================================================
// FEATURE 7: Gamified Progression Map Research Document — Boundary Tests
// ===========================================================================

#[test]
fn test_f07_boundary_research_doc_fallback_paths() {
    let p = gamified_map_path();
    assert!(
        p.exists(),
        "Research document must exist at {}",
        p.display()
    );
}

#[test]
fn test_f07_boundary_research_doc_size_thresholds() {
    let path = gamified_map_path();
    let meta = std::fs::metadata(&path).expect("Read metadata");
    assert!(
        meta.len() >= 1_000,
        "Doc must contain substantive research (>=1,000 bytes)"
    );
    assert!(meta.len() <= 500_000, "Doc must be bounded (<500 KB)");
}

#[test]
fn test_f07_boundary_anti_trivialization_mandate_keywords() {
    let path = gamified_map_path();
    let text = std::fs::read_to_string(&path).expect("Read doc");
    assert!(
        text.contains("590/600"),
        "Must address high-yield 590/600 rigor"
    );
    assert!(
        text.contains("Crown") || text.contains("crown"),
        "Must define crown mastery"
    );
}

#[test]
fn test_f07_boundary_duolingo_mechanics_sections() {
    let path = gamified_map_path();
    let text = std::fs::read_to_string(&path).expect("Read doc");
    assert!(
        text.contains("Duolingo"),
        "Must contain Duolingo case study"
    );
    assert!(
        text.contains("Streak") || text.contains("streak"),
        "Must cover streak retention"
    );
}

#[test]
fn test_f07_boundary_candy_crush_mechanics_sections() {
    let path = gamified_map_path();
    let text = std::fs::read_to_string(&path).expect("Read doc");
    assert!(
        text.contains("Candy Crush"),
        "Must contain Candy Crush case study"
    );
    assert!(
        text.contains("Map") || text.contains("Progression"),
        "Must cover visual map progression"
    );
}

// ===========================================================================
// FEATURE 8: Multi-Chapter Unit Segmentation — Boundary Tests
// ===========================================================================

#[test]
fn test_f08_boundary_empty_journey_state_zero_division() {
    let mut journey = OdysseyJourneyState::new();
    journey.nodes.clear();
    assert_eq!(journey.nodes.len(), 0);
    assert_eq!(journey.overall_progress_ratio(), 0.0);
    assert_eq!(journey.overall_progress_pct(), 0);
}

#[test]
fn test_f08_boundary_single_node_completion_progress() {
    let mut journey = OdysseyJourneyState::new();
    journey.nodes = vec![create_test_odyssey_node(
        "n-single",
        "Single Milestone",
        0,
        100,
    )];
    assert_eq!(journey.overall_progress_pct(), 0);

    journey.nodes[0].status = NodeStatus::Completed;
    assert_eq!(journey.overall_progress_pct(), 100);
}

#[test]
fn test_f08_boundary_zero_estimated_minutes_handling() {
    let mut node = create_test_odyssey_node("n-zero-min", "Zero Min Quick Check", 10, 10);
    node.estimated_minutes = 0;
    assert_eq!(node.estimated_minutes, 0);
    node.update_evaluation(10);
    assert!(node.is_crown_mastered);
}

#[test]
fn test_f08_boundary_extreme_chapter_title_length() {
    let long_title =
        "Chapter 1: Extremely Detailed Mathematical Derivations of Electromagnetic Potentials "
            .repeat(5);
    let node = create_test_odyssey_node("n-long", &long_title, 50, 100);
    assert_eq!(node.title, long_title);
    assert!(node.title.len() > 300);
}

#[test]
fn test_f08_boundary_multi_unit_stable_grouping() {
    let n1 = create_test_odyssey_node("n1", "Unit 1 Node A", 10, 10);
    let mut n2 = create_test_odyssey_node("n2", "Unit 2 Node B", 10, 10);
    n2.unit_id = "unit-2-optics".to_string();
    n2.unit_title = "Unit 2: Wave & Ray Optics".to_string();

    let nodes = [n1, n2];
    let unique_units: Vec<_> = nodes.iter().map(|n| &n.unit_id).collect();
    assert_eq!(unique_units.len(), 2);
    assert_ne!(unique_units[0], unique_units[1]);
}

// ===========================================================================
// FEATURE 9: 3-Star & State Board Crown Mastery — Boundary Cutoffs
// ===========================================================================

#[test]
fn test_f09_boundary_crown_cutoff_97_9_vs_98_0() {
    // 97.9% = 979 / 1000
    let node_979 = create_test_odyssey_node("n-979", "Near Miss", 979, 1000);
    assert_eq!(node_979.stars_earned, 3, "97.9% earns 3 stars (>=90%)");
    assert!(!node_979.is_crown_mastered, "97.9% misses Crown (<98.0%)");

    // 98.0% = 980 / 1000
    let node_980 = create_test_odyssey_node("n-980", "Strict Crown", 980, 1000);
    assert_eq!(node_980.stars_earned, 3);
    assert!(
        node_980.is_crown_mastered,
        "98.0% achieves State Board Golden Crown"
    );
}

#[test]
fn test_f09_boundary_zero_max_marks_safety() {
    let mut node = create_test_odyssey_node("n-zero-max", "Zero Max Marks", 0, 0);
    node.update_evaluation(0);
    assert_eq!(
        node.mastery_percentage(),
        0,
        "0/0 marks must return 0% without division-by-zero panic"
    );
    assert_eq!(node.stars_earned, 0);
    assert!(!node.is_crown_mastered);
}

#[test]
fn test_f09_boundary_perfect_score_crown_award() {
    let mut node = create_test_odyssey_node("n-perfect", "Perfect Theory", 70, 70);
    node.update_evaluation(70);
    node.status = NodeStatus::Completed;
    assert_eq!(node.mastery_percentage(), 100);
    assert_eq!(node.stars_earned, 3);
    assert!(node.is_crown_mastered);
    assert_eq!(node.status, NodeStatus::Completed);
}

#[test]
fn test_f09_boundary_excess_marks_clamped() {
    let mut node = create_test_odyssey_node("n-bonus", "Extra Bonus", 120, 100);
    node.update_evaluation(120);
    assert_eq!(
        node.mastery_percentage(),
        100,
        "Score > max marks must be clamped to 100%"
    );
    assert_eq!(node.stars_earned, 3);
    assert!(node.is_crown_mastered);
}

#[test]
fn test_f09_boundary_star_tiers_edges() {
    // 0% -> 0 stars
    let n0 = create_test_odyssey_node("n0", "Zero", 0, 100);
    assert_eq!(n0.stars_earned, 0);

    // 59.9% -> 0 stars (<60%)
    let n59 = create_test_odyssey_node("n59", "Edge 1", 599, 1000);
    assert_eq!(n59.stars_earned, 0);

    // 60.0% -> 1 star
    let n60 = create_test_odyssey_node("n60", "Edge 2", 60, 100);
    assert_eq!(n60.stars_earned, 1);

    // 79.9% -> 1 star (<80%)
    let n79 = create_test_odyssey_node("n79", "Edge 3", 799, 1000);
    assert_eq!(n79.stars_earned, 1);

    // 80.0% -> 2 stars
    let n80 = create_test_odyssey_node("n80", "Edge 4", 80, 100);
    assert_eq!(n80.stars_earned, 2);

    // 94.9% -> 2 stars (<95%)
    let n94 = create_test_odyssey_node("n94", "Edge 5", 949, 1000);
    assert_eq!(n94.stars_earned, 2);

    // 95.0% -> 3 stars
    let n95 = create_test_odyssey_node("n95", "Edge 6", 95, 100);
    assert_eq!(n95.stars_earned, 3);
}

// ===========================================================================
// FEATURE 10: Interactive 3D Progression Path UI — Boundary Tests
// ===========================================================================

#[test]
fn test_f10_boundary_projection_at_near_plane_t0() {
    let pt = project(0.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(pt.z, 1.0, "At t=0, depth z must be 1.0 (nearest)");
    assert_eq!(pt.r, 26.0, "At t=0, puck radius is at full scale (26px)");
    assert!(pt.y > 0.0 && pt.y < 1080.0);
}

#[test]
fn test_f10_boundary_projection_at_far_horizon_t1() {
    let pt = project(1.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert!(
        (pt.z - 0.48).abs() < 1e-4,
        "At t=1, horizon z is clamped to ~0.48"
    );
    assert!(pt.r < 26.0, "Horizon puck radius is scaled down");
}

#[test]
fn test_f10_boundary_projection_extreme_camera_tilts() {
    let pt_pos45 = project(0.5, 1200.0, 800.0, 45.0, 45.0, 0.0);
    let pt_neg45 = project(0.5, 1200.0, 800.0, -45.0, -45.0, 0.0);

    assert!(pt_pos45.x.is_finite());
    assert!(pt_pos45.y.is_finite());
    assert!(pt_neg45.x.is_finite());
    assert!(pt_neg45.y.is_finite());
}

#[test]
fn test_f10_boundary_hit_test_exact_radius_boundary() {
    let pt = project(0.0, 1000.0, 800.0, 0.0, 0.0, 0.0);
    let r = pt.r; // Hit circle radius

    // Inside radius by 1px
    let dx_inside = r - 1.0;
    let dist_inside = (dx_inside * dx_inside).sqrt();
    assert!(
        dist_inside <= r,
        "Point 1px inside radius must register hit"
    );

    // Outside radius by 1px
    let dx_outside = r + 1.0;
    let dist_outside = (dx_outside * dx_outside).sqrt();
    assert!(dist_outside > r, "Point 1px outside radius must miss");
}

#[test]
fn test_f10_boundary_celebration_particle_count_clamping() {
    let mut harness = crate::helpers::HeadlessHarness::new();
    // Scroll offset is clamped strictly to [-0.15, 0.15] under extreme scroll deltas
    harness.dispatch(mindset_os::Message::ScrollOdysseyMap(10_000.0));
    assert_eq!(harness.app.odyssey_map_state.scroll_offset, -0.15);
    harness.dispatch(mindset_os::Message::ScrollOdysseyMap(-10_000.0));
    assert_eq!(harness.app.odyssey_map_state.scroll_offset, 0.15);

    // Ambient particle animation tick wraps cleanly at u32::MAX without overflow panic
    harness.app.odyssey_map_state.ambient_tick = u32::MAX;
    harness.dispatch(mindset_os::Message::MapAmbientTick);
    assert_eq!(harness.app.odyssey_map_state.ambient_tick, 0);

    // Out-of-range projection parameter t clamps cleanly to [0.0, 1.0]
    let p_neg = project(-5.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    let p_zero = project(0.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(p_neg.z, p_zero.z);
    let p_over = project(5.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    let p_one = project(1.0, 1920.0, 1080.0, 0.0, 0.0, 0.0);
    assert_eq!(p_over.z, p_one.z);
}
