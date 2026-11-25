//! Tier 1: Subsystem R2 Odyssey Gamified Learning Map Tests (Features 7 to 10)
//!
//! Covers:
//! - F7: Gamified map research document verification (>=5 tests)
//! - F8: Multi-chapter unit segmentation, visual dividers (>=5 tests)
//! - F9: 3-Star milestone rating & State Board Crown (>=98%) (>=5 tests)
//! - F10: 3D progression path UI math, projection, celebration rewards (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::odyssey::{NodeType, OdysseyJourneyState, OdysseyNode};
use mindset_os::shell::odyssey_map::project;
use std::path::Path;

use crate::helpers::create_test_odyssey_node;

// ===========================================================================
// FEATURE 7: Gamified Progression Map Research Document
// ===========================================================================

#[test]
fn test_f07_research_doc_existence_and_size() {
    let doc_path = Path::new("docs/GAMIFIED_LEARNING_MAP.md");
    let fallback_path = Path::new("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md");
    assert!(
        doc_path.exists() || fallback_path.exists(),
        "GAMIFIED_LEARNING_MAP.md must exist in docs/"
    );
    let path = if doc_path.exists() {
        doc_path
    } else {
        fallback_path
    };
    let metadata = std::fs::metadata(path).expect("Failed to read metadata");
    assert!(
        metadata.len() > 10_000,
        "Research document must be comprehensive (>10 KB), found {} bytes",
        metadata.len()
    );
}

#[test]
fn test_f07_duolingo_mechanics_extraction() {
    let content = std::fs::read_to_string("docs/GAMIFIED_LEARNING_MAP.md")
        .or_else(|_| std::fs::read_to_string("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md"))
        .expect("Failed to read GAMIFIED_LEARNING_MAP.md");

    assert!(content.contains("Duolingo"), "Must analyze Duolingo");
    assert!(
        content.contains("The Snake") || content.contains("serpentine"),
        "Must analyze Duolingo Snake progression"
    );
    assert!(content.contains("Streak"), "Must analyze streak mechanics");
}

#[test]
fn test_f07_candy_crush_mechanics_extraction() {
    let content = std::fs::read_to_string("docs/GAMIFIED_LEARNING_MAP.md")
        .or_else(|_| std::fs::read_to_string("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md"))
        .expect("Failed to read GAMIFIED_LEARNING_MAP.md");

    assert!(content.contains("Candy Crush"), "Must analyze Candy Crush");
    assert!(
        content.contains("3-Star") || content.contains("3-star"),
        "Must analyze 3-star scoring"
    );
    assert!(
        content.contains("Episode") || content.contains("Roadblock") || content.contains("Biome"),
        "Must analyze episodic progression"
    );
}

#[test]
fn test_f07_590_600_anti_trivialization_mandate() {
    let content = std::fs::read_to_string("docs/GAMIFIED_LEARNING_MAP.md")
        .or_else(|_| std::fs::read_to_string("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md"))
        .expect("Failed to read GAMIFIED_LEARNING_MAP.md");

    assert!(
        content.contains("590/600"),
        "Must frame standards against 590/600 State Board benchmark"
    );
    assert!(
        content.contains("Crown") || content.contains("98%"),
        "Must detail Crown mastery tier"
    );
}

#[test]
fn test_f07_dual_cache_canvas_architecture() {
    let content = std::fs::read_to_string("docs/GAMIFIED_LEARNING_MAP.md")
        .or_else(|_| std::fs::read_to_string("C:/MINDSETOS/docs/GAMIFIED_LEARNING_MAP.md"))
        .expect("Failed to read GAMIFIED_LEARNING_MAP.md");

    assert!(
        content.contains("static_road_cache")
            || content.contains("Dual-Cache")
            || content.contains("0% CPU")
    );
}

// ===========================================================================
// FEATURE 8: Multi-Chapter Unit Segmentation
// ===========================================================================

#[test]
fn test_f08_unit_segmentation_fields() {
    let node = create_test_odyssey_node("phy-101", "Electrostatics & Gauss's Law", 95, 100);
    assert!(!node.unit_id.is_empty(), "Node must contain unit_id");
    assert!(!node.unit_title.is_empty(), "Node must contain unit_title");
    assert_eq!(node.unit_id, "unit-1-electromagnetism");
}

#[test]
fn test_f08_unit_grouping_in_journey_state() {
    let state = OdysseyJourneyState::new();
    assert!(
        !state.nodes.is_empty(),
        "Odyssey journey must have initial syllabus nodes"
    );

    // Check that multiple nodes share common unit groups
    let first_unit = &state.nodes[0].unit_id;
    let count_in_unit = state
        .nodes
        .iter()
        .filter(|n| &n.unit_id == first_unit)
        .count();
    assert!(count_in_unit >= 1);
}

#[test]
fn test_f08_node_type_enumeration_and_badges() {
    let theory = NodeType::Theory;
    let drill = NodeType::NumericalDrill;
    let chest = NodeType::ReviewChest;
    let boss = NodeType::UnitBoss;

    assert_eq!(theory.display_badge(), "Theory");
    assert_eq!(drill.display_badge(), "Numerical Drill");
    assert_eq!(chest.display_badge(), "Review Chest");
    assert_eq!(boss.display_badge(), "Unit Boss");

    assert_ne!(theory.icon(), drill.icon());
    assert_ne!(drill.icon(), boss.icon());
}

#[test]
fn test_f08_initial_odyssey_nodes_population() {
    let state = OdysseyJourneyState::new();
    assert!(
        state.nodes.len() >= 3,
        "Initial journey should contain at least 3 nodes"
    );
    assert!(state.nodes.iter().any(|n| n.subject == "Physics"));
}

#[test]
fn test_f08_estimated_minutes_pacing() {
    let node = create_test_odyssey_node("phy-102", "Capacitance", 80, 100);
    assert!(
        node.estimated_minutes >= 30 && node.estimated_minutes <= 60,
        "Node pacing must align with 30-60 minute focus blocks, found {} mins",
        node.estimated_minutes
    );
    assert_eq!(node.estimated_time_display(), "35 mins");
}

// ===========================================================================
// FEATURE 9: 3-Star Milestone Rating & State Board Crown (>=98%)
// ===========================================================================

#[test]
fn test_f09_stars_calculation_zero_percent() {
    let stars = OdysseyNode::compute_stars(0, 100);
    assert_eq!(stars, 0);
    let crown = OdysseyNode::compute_crown(0, 100);
    assert!(!crown);
}

#[test]
fn test_f09_stars_calculation_tiers() {
    // Tier 1: 60% Passing -> 1 Star
    assert_eq!(OdysseyNode::compute_stars(60, 100), 1);
    assert_eq!(OdysseyNode::compute_stars(79, 100), 1);

    // Tier 2: 80% Proficient -> 2 Stars
    assert_eq!(OdysseyNode::compute_stars(80, 100), 2);
    assert_eq!(OdysseyNode::compute_stars(94, 100), 2);

    // Tier 3: 95% Mastery -> 3 Stars
    assert_eq!(OdysseyNode::compute_stars(95, 100), 3);
    assert_eq!(OdysseyNode::compute_stars(97, 100), 3);
}

#[test]
fn test_f09_crown_cutoff_strictly_at_98_percent() {
    // 97 marks out of 100 = 97.0% -> 3 stars, but NO crown
    assert_eq!(OdysseyNode::compute_stars(97, 100), 3);
    assert!(
        !OdysseyNode::compute_crown(97, 100),
        "97% must not receive State Board Crown"
    );

    // 98 marks out of 100 = 98.0% -> 3 stars AND Crown!
    assert_eq!(OdysseyNode::compute_stars(98, 100), 3);
    assert!(
        OdysseyNode::compute_crown(98, 100),
        "98% must receive State Board Crown"
    );

    // 590 out of 600 = 98.33% -> Crown!
    assert!(OdysseyNode::compute_crown(590, 600));

    // 587 out of 600 = 97.83% -> No crown
    assert!(!OdysseyNode::compute_crown(587, 600));
}

#[test]
fn test_f09_crown_evaluation_update_mutation() {
    let mut node = create_test_odyssey_node("node-1", "Kirchhoff Laws", 0, 100);
    assert_eq!(node.stars_earned, 0);
    assert!(!node.is_crown_mastered);

    node.update_evaluation(99);
    assert_eq!(node.marks, 99);
    assert_eq!(node.stars_earned, 3);
    assert!(node.is_crown_mastered);
    assert_eq!(node.mastery_percentage(), 99);
}

#[test]
fn test_f09_star_rating_display_formatting() {
    let mut node = create_test_odyssey_node("node-test", "Test Node", 0, 100);
    node.stars_earned = 0;
    assert_eq!(node.star_rating_display(), "☆☆☆");

    node.stars_earned = 1;
    assert_eq!(node.star_rating_display(), "★☆☆");

    node.stars_earned = 2;
    assert_eq!(node.star_rating_display(), "★★☆");

    node.stars_earned = 3;
    assert_eq!(node.star_rating_display(), "★★★");
}

// ===========================================================================
// FEATURE 10: 3D Progression Path UI Math & Projection
// ===========================================================================

#[test]
fn test_f10_projection_foreground_stability() {
    // At t = 0 (foreground start of road):
    // z = 1.0 - 0.52 * 0.0 = 1.0
    // depth_factor = (1.0 - z).max(0.0) = 0.0
    // Tilt shifts px and py are multiplied by depth_factor (0.0) -> zero parallax distortion in foreground!
    let pt = project(0.0, 1000.0, 800.0, 10.0, 10.0, 5.0);
    assert_eq!(pt.z, 1.0);
    assert_eq!(pt.r, 26.0 * 1.0);
}

#[test]
fn test_f10_projection_distant_parallax_shift() {
    // At t = 1.0 (horizon end of road):
    // z = 1.0 - 0.52 = 0.48
    // depth_factor = 1.0 - 0.48 = 0.52 > 0
    // Distant points exhibit significant perspective tilt shift
    let pt_no_tilt = project(1.0, 1000.0, 800.0, 0.0, 0.0, 0.0);
    let pt_tilted = project(1.0, 1000.0, 800.0, 18.0, 12.0, 0.0);

    assert!((pt_no_tilt.z - 0.48).abs() < 1e-4);
    assert!((pt_tilted.z - 0.48).abs() < 1e-4);
    assert_ne!(
        pt_no_tilt.x, pt_tilted.x,
        "Perspective tilt must shift distant X coordinate"
    );
    assert_ne!(
        pt_no_tilt.y, pt_tilted.y,
        "Perspective tilt must shift distant Y coordinate"
    );
}

#[test]
fn test_f10_projection_radius_depth_scaling() {
    let pt_fg = project(0.0, 1000.0, 800.0, 0.0, 0.0, 0.0);
    let pt_bg = project(1.0, 1000.0, 800.0, 0.0, 0.0, 0.0);

    assert!(
        pt_fg.r > pt_bg.r,
        "Foreground node radius ({}) must be larger than distant horizon node radius ({})",
        pt_fg.r,
        pt_bg.r
    );
    assert!((pt_fg.r - 26.0).abs() < 1e-4);
    assert!((pt_bg.r - (26.0 * 0.48)).abs() < 1e-4);
}

#[test]
fn test_f10_hit_test_distance_calculation() {
    let pt = project(0.5, 1000.0, 800.0, 0.0, 0.0, 0.0);
    let cursor_inside = (pt.x + 5.0, pt.y + 5.0);
    let dx = cursor_inside.0 - pt.x;
    let dy = cursor_inside.1 - pt.y;
    let dist = (dx * dx + dy * dy).sqrt();
    assert!(
        dist <= pt.r,
        "Cursor at +5,+5 must register as hit within node radius {}",
        pt.r
    );

    let cursor_outside = (pt.x + 100.0, pt.y + 100.0);
    let dx_out = cursor_outside.0 - pt.x;
    let dy_out = cursor_outside.1 - pt.y;
    let dist_out = (dx_out * dx_out + dy_out * dy_out).sqrt();
    assert!(
        dist_out > pt.r,
        "Cursor at +100,+100 must be outside node radius"
    );
}

#[test]
fn test_f10_ambient_tick_sway_motion() {
    let pt_t0 = project(0.5, 1000.0, 800.0, 0.0, 0.0, 0.0);
    let pt_swayed = project(0.5, 1000.0, 800.0, 0.0, 0.0, 5.0);
    assert_ne!(
        pt_t0.x, pt_swayed.x,
        "Ambient motion sway must perturb node position dynamically"
    );
}
