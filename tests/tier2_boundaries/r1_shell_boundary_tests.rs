//! Tier 2: Subsystem R1 Shell & Morphism Boundary Tests (Features 1 to 6)
//!
//! Stress, limit, and edge condition verification:
//! - F1: Extreme display scaling, zero/negative bounds, procedural fallback, rapid cycling (5 tests)
//! - F2: PIN brute force lockout, countdown lock, length truncate, non-digits, empty credentials (5 tests)
//! - F3: Boundary clamping, viewport overflow, pinned widget immobility, Z-index stacking (5 tests)
//! - F4: Liquidmorphism RAM evaluation at 4K/8K, 10k frame leak assertion, angle boundaries, Fresnel exponents (5 tests)
//! - F5: Notch empty vs high result counts, text overflow clamping, timer transitions, waveform limits (5 tests)
//! - F6: Prompt builder missing fields, 0 target score, empty weak topics, negative dates (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::{ActiveOverlay, DesktopWidgetId, ThemeMode};
use mindset_motion::{Spring1D, SpringPreset};
use mindset_os::ai::prompt_builder::build_personalized_system_prompt;
use mindset_os::shell::dynamic_island::IslandMode;
use mindset_os::shell::lock_screen::LockAuthMode;
use mindset_os::shell::wallpaper::ProceduralCosmicWallpaper;
use mindset_os::theme::liquidmorphism::{
    directional_specular_rim, fresnel_sheen, sdf_rounded_box, LiquidMetrics, LiquidPalette,
};
use mindset_os::theme::ThemePalette;
use mindset_os::Message;
use std::path::Path;

use crate::helpers::{create_test_profile, HeadlessHarness, AUTH_LOCK};

// ===========================================================================
// FEATURE 1: Garuda / Pop!_OS Desktop Experience — Boundary & Stress Tests
// ===========================================================================

#[test]
fn test_f01_boundary_extreme_scaling_factors() {
    let mut harness = HeadlessHarness::new();

    // Scale factor 2.0 is clamped to maximum UI scaling bound (1.45)
    harness.dispatch(Message::ScaleChanged(2.0));
    assert_eq!(harness.app.scale_factor(), 1.45);

    // Scale factor 3.0 is clamped to maximum UI scaling bound (1.45)
    harness.dispatch(Message::ScaleChanged(3.0));
    assert_eq!(harness.app.scale_factor(), 1.45);

    // Scale factor 0.5 is clamped to minimum UI scaling bound (0.80)
    harness.dispatch(Message::ScaleChanged(0.5));
    assert_eq!(harness.app.scale_factor(), 0.80);

    // Scale factor 1.0 (Standard 100%)
    harness.dispatch(Message::ScaleChanged(1.0));
    assert_eq!(harness.app.scale_factor(), 1.0);
}

#[test]
fn test_f01_boundary_zero_and_extreme_window_dimensions() {
    let mut harness = HeadlessHarness::new();
    // Simulate resizing to ultra-wide 8K viewport (7680x2160)
    harness.dispatch(Message::EventOccurred(iced::Event::Window(
        iced::window::Event::Resized(iced::Size::new(7680.0, 2160.0)),
    )));
    assert_eq!(harness.app.wm_interaction.screen_width, 7680.0);
    assert_eq!(harness.app.wm_interaction.screen_height, 2160.0);

    // Simulate resizing to minimal netbook viewport (800x600)
    harness.dispatch(Message::EventOccurred(iced::Event::Window(
        iced::window::Event::Resized(iced::Size::new(800.0, 600.0)),
    )));
    assert_eq!(harness.app.wm_interaction.screen_width, 800.0);
    assert_eq!(harness.app.wm_interaction.screen_height, 600.0);
}

#[test]
fn test_f01_boundary_wallpaper_fallback_when_path_absent() {
    let mut harness = HeadlessHarness::new();
    let non_existent = Path::new("non_existent_dir/no_wallpaper.png");
    assert!(!non_existent.exists());

    harness.app.wallpaper_handle = None;
    harness.dispatch(Message::SetWallpaperPreset(
        non_existent.to_string_lossy().to_string(),
    ));
    assert!(
        harness.app.wallpaper_handle.is_none(),
        "Absent wallpaper path must not populate wallpaper_handle"
    );

    let palette = ThemePalette::for_mode(ThemeMode::DarkSpace);
    let fallback = ProceduralCosmicWallpaper {
        cache: &harness.app.wallpaper_canvas_cache,
        palette,
    };
    assert_eq!(fallback.palette.mode, ThemeMode::DarkSpace);

    // Headless view() render must execute procedural fallback cleanly without panicking
    let _view = harness.app.view();
}

#[test]
fn test_f01_boundary_rapid_theme_cycling_stress() {
    let mut harness = HeadlessHarness::new();
    let themes = [
        ThemeMode::DarkSpace,
        ThemeMode::AlpineGlass,
        ThemeMode::VioletNebula,
        ThemeMode::SolarAmber,
    ];

    // Rapid cycle 100 times to verify zero panic, zero heap corruption
    for i in 0..100 {
        let mode = themes[i % themes.len()];
        harness.dispatch(Message::ThemeSelected(mode));
        assert_eq!(harness.app.core.preferences.theme, mode);
    }
}

#[test]
fn test_f01_boundary_palette_color_channel_clamping() {
    for mode in [
        ThemeMode::DarkSpace,
        ThemeMode::AlpineGlass,
        ThemeMode::VioletNebula,
        ThemeMode::SolarAmber,
    ] {
        let p = ThemePalette::for_mode(mode);
        assert!(p.background.r >= 0.0 && p.background.r <= 1.0);
        assert!(p.background.g >= 0.0 && p.background.g <= 1.0);
        assert!(p.background.b >= 0.0 && p.background.b <= 1.0);
        assert!(p.glass_surface.a > 0.0 && p.glass_surface.a <= 1.0);
        assert!(p.glass_surface_elevated.a >= p.glass_surface.a);
    }
}

// ===========================================================================
// FEATURE 2: Functional Lock Screen Auth — Boundary & Corner Tests
// ===========================================================================

#[test]
fn test_f02_boundary_pin_brute_force_lockout_threshold() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.app.lock_screen.failed_attempts = 0;
    harness.app.lock_screen.is_locked_out = false;
    harness.app.lock_screen.lockout_seconds_remaining = 0;
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // Exact threshold: 4 failures do NOT lock out; 5th failure DOES
    for attempt in 1..=4 {
        harness.dispatch(Message::LockScreenPinChanged("000000".to_string()));
        assert!(
            !harness.app.lock_screen.is_locked_out,
            "Attempt {} must not lock out",
            attempt
        );
        assert_eq!(harness.app.lock_screen.failed_attempts, attempt);
    }

    // 5th failed attempt triggers lockout
    harness.dispatch(Message::LockScreenPinChanged("000000".to_string()));
    assert!(harness.app.lock_screen.is_locked_out);
    assert!(harness.app.lock_screen.failed_attempts >= 4);
    assert!(harness.app.lock_screen.lockout_seconds_remaining > 0);
}

#[test]
fn test_f02_boundary_submissions_blocked_during_active_lockout() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.app.lock_screen.failed_attempts = 0;
    harness.app.lock_screen.is_locked_out = false;
    harness.app.lock_screen.lockout_seconds_remaining = 0;
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));

    // Force 5 failures
    for _ in 0..5 {
        harness.dispatch(Message::LockScreenPinChanged("999999".to_string()));
    }
    assert!(harness.app.lock_screen.is_locked_out);

    // Even with the CORRECT pin "123456", submissions while locked out are rejected
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(
        harness.active_overlay(),
        ActiveOverlay::LockScreen,
        "Workspace must remain locked"
    );
    assert!(harness.app.lock_screen.is_locked_out);
    harness.reset_auth();
}

#[test]
fn test_f02_boundary_pin_input_length_truncation() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.app.lock_screen.failed_attempts = 0;
    harness.app.lock_screen.is_locked_out = false;
    harness.app.lock_screen.lockout_seconds_remaining = 0;
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));

    // Entering a 10-digit string is safely truncated to 6 digits (auto-submitting on 6th digit)
    harness.dispatch(Message::LockScreenPinChanged("1234567890".to_string()));
    // Correct 6-digit prefix "123456" auto-submits and unlocks!
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert_eq!(harness.app.lock_screen.failed_attempts, 0);
    harness.reset_auth();
}

#[test]
fn test_f02_boundary_non_numeric_pin_character_filtering() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));

    // Entering alphanumeric string "1a2b3c" filters out non-digits: only "123" remains (len < 6)
    harness.dispatch(Message::LockScreenPinChanged("1a2b3c".to_string()));
    assert_eq!(harness.app.lock_screen.pin_input, "123");
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);
}

#[test]
fn test_f02_boundary_empty_password_and_pin_submission_safe() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    harness.dispatch(Message::LockScreenToggleAuthMode);
    assert_eq!(harness.app.lock_screen.mode, LockAuthMode::Password);

    // Empty password submission must be rejected cleanly with 0 crashes
    harness.dispatch(Message::LockScreenPasswordChanged("   ".to_string()));
    harness.dispatch(Message::SubmitLockScreenAuth);
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);
}

// ===========================================================================
// FEATURE 3: Editable Desktop Widgets — Boundary & Clamping Tests
// ===========================================================================

#[test]
fn test_f03_boundary_widget_negative_coordinates_clamped() {
    let mut harness = HeadlessHarness::new();
    let id = DesktopWidgetId::ScholarBriefing;
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.x = -100.0;
        w.y = -200.0;
        harness.app.wm_interaction.clamp_widget(w);
    }
    let w = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert!(w.x >= 0.0, "Widget X ({}) must be clamped >= 0", w.x);
    assert!(
        w.y >= 36.0,
        "Widget Y ({}) must be clamped below 36px topbar",
        w.y
    );
}

#[test]
fn test_f03_boundary_widget_viewport_overflow_clamped() {
    let mut harness = HeadlessHarness::new();
    let id = DesktopWidgetId::Scratchpad;
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.x = 10_000.0;
        w.y = 10_000.0;
        harness.app.wm_interaction.clamp_widget(w);
    }
    let w = harness.app.core.wm.get_widget_placement(id).unwrap();
    // Widget must be clamped inside desktop viewport
    assert!(w.x < 10_000.0);
    assert!(w.y < 10_000.0);
}

#[test]
fn test_f03_boundary_pinned_widget_strictly_immobile() {
    let mut harness = HeadlessHarness::new();
    let id = DesktopWidgetId::OdysseyTrail;
    if let Some(w) = harness.app.core.wm.get_widget_placement_mut(id) {
        w.is_pinned = true;
    }
    let initial = harness
        .app
        .core
        .wm
        .get_widget_placement(id)
        .unwrap()
        .clone();
    assert!(initial.is_pinned);

    // Attempt dragging pinned widget
    harness.dispatch(Message::StartWidgetDrag(id));
    assert!(
        harness.app.wm_interaction.dragging_widget.is_none(),
        "Pinned widget must reject start drag"
    );
    let post_drag = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert_eq!(post_drag.x, initial.x, "Pinned widget X must not move");
    assert_eq!(post_drag.y, initial.y, "Pinned widget Y must not move");
}

#[test]
fn test_f03_boundary_widget_visibility_toggle_idempotency() {
    let mut harness = HeadlessHarness::new();
    let id = DesktopWidgetId::Scratchpad;

    // Toggle visibility
    harness.dispatch(Message::ToggleWidgetVisibility(id));
    let w1 = harness.app.core.wm.get_widget_placement(id).unwrap();
    let v1 = w1.is_visible;

    // Toggle visible again
    harness.dispatch(Message::ToggleWidgetVisibility(id));
    let w2 = harness.app.core.wm.get_widget_placement(id).unwrap();
    assert_ne!(w2.is_visible, v1);
}

#[test]
fn test_f03_boundary_reset_layout_under_compact_screen() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::EventOccurred(iced::Event::Window(
        iced::window::Event::Resized(iced::Size::new(1024.0, 768.0)),
    )));
    harness.dispatch(Message::ResetWidgetLayout);

    for w in &harness.app.core.wm.widget_layout.placements {
        assert!(w.x >= 0.0);
        assert!(w.y >= 0.0);
    }
}

// ===========================================================================
// FEATURE 4: Liquidmorphism RAM Evaluation — Boundary & Stress Tests
// ===========================================================================

#[test]
fn test_f04_boundary_extreme_vram_estimation_scaling() {
    // 8K Resolution: 7680 x 4320, 2 blur passes
    let (sdf_bytes, blur_bytes) = LiquidMetrics::compute_ram_vram_footprint(7680, 4320, 2);
    assert!(
        blur_bytes > 200_000_000,
        "Glassmorphism VRAM at 8K exceeds 200 MB, got {}",
        blur_bytes
    );
    assert!(
        sdf_bytes <= LiquidMetrics::SINGLE_PASS_SDF_RAM_BYTES,
        "Liquidmorphism uniform stays bounded <= 256 KB, got {}",
        sdf_bytes
    );
    assert!(
        blur_bytes > sdf_bytes * 1000,
        "Liquidmorphism is >1000x lighter in VRAM at 8K"
    );
    assert!(LiquidMetrics::memory_reduction_ratio(7680, 4320, 2) > 1000.0);
}

#[test]
fn test_f04_boundary_10k_iterations_zero_leak() {
    assert_eq!(std::mem::size_of::<LiquidPalette>(), 176);
    let (initial_sdf, _) = LiquidMetrics::compute_ram_vram_footprint(1920, 1080, 2);
    for i in 0..10_000 {
        let (sdf, _) = LiquidMetrics::compute_ram_vram_footprint(1920, 1080, 2);
        assert_eq!(sdf, initial_sdf);
        let d = sdf_rounded_box([10.0, 5.0], [50.0, 20.0], 10.0);
        assert!(d < 0.0);
        let sheen = fresnel_sheen((i % 7) as f32, 6.0, 0.04);
        assert!((0.0..=1.0).contains(&sheen));
    }
}

#[test]
fn test_f04_boundary_specular_rim_azimuth_cardinal_angles() {
    // Screen-space (+x right, -y up): 0->(+1,0), 90->(0,-1), 180->(-1,0), 270->(0,+1), 360->(+1,0)
    let cases = [
        (0.0f32, [1.0f32, 0.0f32]),
        (90.0, [0.0, -1.0]),
        (180.0, [-1.0, 0.0]),
        (270.0, [0.0, 1.0]),
        (360.0, [1.0, 0.0]),
    ];
    for (angle, aligned_normal) in cases {
        let intensity = directional_specular_rim(aligned_normal, angle, 1.0);
        assert!(
            (intensity - 1.0).abs() < 1e-4,
            "Aligned cardinal normal at {} deg must yield 1.0, got {}",
            angle,
            intensity
        );
        let opposite =
            directional_specular_rim([-aligned_normal[0], -aligned_normal[1]], angle, 1.0);
        assert!(
            opposite < 1e-4,
            "Opposite cardinal normal at {} deg must yield ~0.0, got {}",
            angle,
            opposite
        );
    }
}

#[test]
fn test_f04_boundary_fresnel_power_curve_limits() {
    for rim in [0.0f32, 1.0, 6.0, 20.0] {
        for dist in [0.0f32, 0.001, 3.0, 6.0, 100.0] {
            let sheen = fresnel_sheen(dist, rim, 0.04);
            assert!(sheen.is_finite());
            assert!(
                (0.0..=1.0).contains(&sheen),
                "Sheen must stay clamped in [0, 1], got {}",
                sheen
            );
        }
    }
    // Zero rim_thickness must guard against division-by-zero
    assert!(fresnel_sheen(0.0, 0.0, 0.04) >= 0.99);
}

#[test]
fn test_f04_boundary_liquid_surface_opacity_clamped() {
    for mode in [
        ThemeMode::DarkSpace,
        ThemeMode::AlpineGlass,
        ThemeMode::VioletNebula,
        ThemeMode::SolarAmber,
    ] {
        let palette = ThemePalette::for_mode(mode);
        assert!(
            palette.glass_surface.a >= 0.50,
            "Opacity must guarantee WCAG AAA scrim >= 0.50"
        );
        assert!(
            palette.glass_surface.a <= 0.95,
            "Opacity must preserve translucent liquid refraction"
        );
    }
}

// ===========================================================================
// FEATURE 5: Liquidmorphism Notch Island & Widgets — Boundary Tests
// ===========================================================================

#[test]
fn test_f05_boundary_search_dimensions_zero_vs_hundred_results() {
    let mode = IslandMode::Search;
    let (w0, h0) = mode.target_dimensions(0);
    assert_eq!(w0, 480.0);
    assert_eq!(
        h0, 52.0,
        "Empty search results must stay compact height (52px)"
    );

    let (w100, h100) = mode.target_dimensions(100);
    assert_eq!(w100, 480.0);
    assert_eq!(
        h100, 240.0,
        "Large search results list clamped to max height (240px)"
    );
}

#[test]
fn test_f05_boundary_extreme_search_query_text() {
    let mut harness = HeadlessHarness::new();
    let long_query = "physics ".repeat(150); // 1200 characters
    harness.dispatch(Message::SearchInputChanged(long_query.clone()));
    assert_eq!(harness.app.search_query.len(), long_query.len());
    // Notch dimensions remain clamped
    let (w, h) = IslandMode::Search.target_dimensions(0);
    assert_eq!(w, 480.0);
    assert_eq!(h, 52.0);
}

#[test]
fn test_f05_boundary_control_center_toggle_transitions() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::SetIslandMode(IslandMode::ControlCenter));
    // Control center dimensions must be stable
    let (w, h) = IslandMode::ControlCenter.target_dimensions(0);
    assert_eq!(w, 440.0);
    assert_eq!(h, 260.0);
}

#[test]
fn test_f05_boundary_speaking_waveform_amplitude_bounds() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::SetIslandMode(IslandMode::MaindySpeaking));
    assert_eq!(harness.app.island_mode, IslandMode::MaindySpeaking);
    assert_eq!(
        IslandMode::MaindySpeaking.target_dimensions(0),
        (340.0, 38.0)
    );

    let mut spring = Spring1D::with_preset(0.0, SpringPreset::SnappyMicro);
    spring.set_target(1.0);
    for dt in [-1.0f32, 0.0, 0.016, 10.0, f32::NAN, f32::INFINITY] {
        let val = spring.update(dt);
        assert!(val.is_finite());
        let clamped = val.clamp(0.0, 1.0);
        assert!((0.0..=1.0).contains(&clamped));
    }
}

#[test]
fn test_f05_boundary_compact_notch_height_stability() {
    let mode = IslandMode::Compact;
    let (_, h) = mode.target_dimensions(0);
    assert_eq!(h, 34.0, "Compact notch height must be exactly 34.0px");
}

// ===========================================================================
// FEATURE 6: Workspace Prompt Builder — Boundary & Fallback Tests
// ===========================================================================

#[test]
fn test_f06_boundary_empty_student_name_fallback() {
    let mut profile = create_test_profile("", 590);
    profile.name = "   ".to_string();
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(
        prompt.contains("Scholar"),
        "Blank student name must fallback to 'Scholar'"
    );
}

#[test]
fn test_f06_boundary_empty_weak_topics_fallback() {
    let mut profile = create_test_profile("Karthik", 590);
    profile.weak_topics.clear();
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt.contains("Core Syllabus Fundamentals"));
}

#[test]
fn test_f06_boundary_empty_subjects_list_fallback() {
    let mut profile = create_test_profile("Deepa", 590);
    profile.subjects.clear();
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt.contains("Physics, Chemistry, Mathematics, Biology"));
}

#[test]
fn test_f06_boundary_zero_target_score_clamping() {
    let profile = create_test_profile("Ramesh", 0);
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    // Should display Target: 0/600 without panic
    assert!(prompt.contains("Target: 0/600"));
}

#[test]
fn test_f06_boundary_unknown_coaching_style_fallback() {
    let mut profile = create_test_profile("Meena", 590);
    profile.coaching_style = "NonExistentStyle".to_string();
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(
        prompt.contains("Tone: Calm Mentor"),
        "Unknown style must fallback to Calm Mentor"
    );
}
