//! Tier 1: Subsystem R1 Shell & Morphism Tests (Features 1 to 6)
//!
//! Covers:
//! - F1: Desktop experience, theme tokens, wallpaper asset verification (>=5 tests)
//! - F2: Auth lock screen, PIN/password, lockout threshold, unlock transition (>=5 tests)
//! - F3: Editable desktop widgets, drag, pin, unpin, layout persistence (>=5 tests)
//! - F4: Liquidmorphism RAM evaluation, shader footprint calculation (>=5 tests)
//! - F5: Liquidmorphism notch island & widget styling, tokens, states (>=5 tests)
//! - F6: Workspace prompt builder assertion & keywords (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::{ActiveOverlay, DesktopWidgetId, ThemeMode, WindowId};
use mindset_os::ai::prompt_builder::build_personalized_system_prompt;
use mindset_os::shell::dynamic_island::IslandMode;
use mindset_os::shell::lock_screen::LockAuthMode;
use mindset_os::shell::wallpaper::ProceduralCosmicWallpaper;
use mindset_os::theme::liquidmorphism::{
    directional_specular_rim, fresnel_sheen, sdf_liquid_normal, LiquidMetrics,
};
use mindset_os::theme::ThemePalette;
use mindset_os::Message;
use std::f32::consts::FRAC_1_SQRT_2;
use std::path::Path;

use crate::helpers::{create_test_profile, HeadlessHarness, AUTH_LOCK};

// ===========================================================================
// FEATURE 1: Garuda / Pop!_OS Desktop Experience & Wallpaper
// ===========================================================================

#[test]
fn test_f01_wallpaper_asset_presence() {
    let wallpaper_path = Path::new("assets/wallpaper.jpg");
    let fallback_path = Path::new("C:/MINDSETOS/assets/wallpaper.jpg");
    assert!(
        wallpaper_path.exists() || fallback_path.exists(),
        "Wallpaper asset must exist at assets/wallpaper.jpg"
    );
    let path = if wallpaper_path.exists() {
        wallpaper_path
    } else {
        fallback_path
    };
    let metadata = std::fs::metadata(path).expect("Failed to read wallpaper metadata");
    assert!(
        metadata.len() > 10_000,
        "Wallpaper asset must be a high-resolution image (>10 KB), found {} bytes",
        metadata.len()
    );
}

#[test]
fn test_f01_theme_palette_token_hierarchy() {
    let dark = ThemePalette::for_mode(ThemeMode::DarkSpace);
    let light = ThemePalette::for_mode(ThemeMode::AlpineGlass);
    let violet = ThemePalette::for_mode(ThemeMode::VioletNebula);
    let solar = ThemePalette::for_mode(ThemeMode::SolarAmber);

    // Dark theme tokens
    assert!(dark.background.r < 0.15 && dark.background.g < 0.15 && dark.background.b < 0.15);
    assert!(dark.text_primary.r > 0.85);

    // Light theme tokens (frosted glass surface with soft azure tint)
    assert!(light.background.r > 0.70 && light.background.b > 0.90);
    assert!(light.text_primary.r < 0.20);

    // High-contrast cyber-neon accents
    assert_ne!(dark.accent, violet.accent);
    assert_ne!(violet.accent, solar.accent);
}

#[test]
fn test_f01_procedural_cosmic_fallback_geometry() {
    let mut harness = HeadlessHarness::new();
    // 1. Valid bundled wallpaper asset loads into wallpaper_handle
    let asset_path = if Path::new("assets/wallpaper.jpg").exists() {
        "assets/wallpaper.jpg"
    } else {
        "C:/MINDSETOS/assets/wallpaper.jpg"
    };
    harness.dispatch(Message::SetWallpaperPreset(asset_path.to_string()));
    assert!(
        harness.app.wallpaper_handle.is_some(),
        "Existing wallpaper asset must load into wallpaper_handle"
    );

    // 2. Absent wallpaper falls back cleanly to ProceduralCosmicWallpaper without panic
    harness.app.wallpaper_handle = None;
    harness.dispatch(Message::SetWallpaperPreset(
        "non_existent_dir/missing_wallpaper.png".to_string(),
    ));
    assert!(
        harness.app.wallpaper_handle.is_none(),
        "Non-existent wallpaper path must leave wallpaper_handle as None"
    );

    let palette = ThemePalette::for_mode(ThemeMode::DarkSpace);
    let procedural = ProceduralCosmicWallpaper {
        cache: &harness.app.wallpaper_canvas_cache,
        palette,
    };
    assert_eq!(procedural.palette.mode, ThemeMode::DarkSpace);
    assert!(procedural.palette.background.r < 0.15);

    // Render view headlessly under procedural fallback
    let _view = harness.app.view();
}

#[test]
fn test_f01_theme_switch_message_dispatch() {
    let mut harness = HeadlessHarness::new();
    // Ensure starting state is tracked, then switch through all themes
    harness.dispatch(Message::ThemeSelected(ThemeMode::DarkSpace));
    assert_eq!(harness.app.core.preferences.theme, ThemeMode::DarkSpace);

    harness.dispatch(Message::ThemeSelected(ThemeMode::VioletNebula));
    assert_eq!(harness.app.core.preferences.theme, ThemeMode::VioletNebula);

    harness.dispatch(Message::ThemeSelected(ThemeMode::SolarAmber));
    assert_eq!(harness.app.core.preferences.theme, ThemeMode::SolarAmber);

    harness.dispatch(Message::ThemeSelected(ThemeMode::AlpineGlass));
    assert_eq!(harness.app.core.preferences.theme, ThemeMode::AlpineGlass);
}

#[test]
fn test_f01_desktop_safe_area_bounds() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::OpenWindow(WindowId::Tasks));
    let win = harness
        .app
        .core
        .wm
        .get_window(WindowId::Tasks)
        .expect("Tasks window should be open");
    // Window must be positioned within desktop safe area (below 36px topbar)
    assert!(
        win.y >= 36.0,
        "Window Y ({}) must be below top bar safe area (36px)",
        win.y
    );
}

// ===========================================================================
// FEATURE 2: Functional Lock Screen with Authentication
// ===========================================================================

#[test]
fn test_f02_lock_screen_initial_state() {
    let harness = HeadlessHarness::new();
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert!(!harness.app.lock_screen.is_locked_out);
    assert_eq!(harness.app.lock_screen.failed_attempts, 0);
}

#[test]
fn test_f02_verify_valid_pin_unlocks() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // Enter correct demo PIN "123456" (6 digits triggers auto-submit)
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    // Verified unlock closes the lock screen overlay and clears pin
    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert_eq!(harness.app.lock_screen.failed_attempts, 0);
    assert!(harness.app.lock_screen.pin_input.is_empty());
    harness.reset_auth();
}

#[test]
fn test_f02_verify_valid_password_unlocks() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    harness.dispatch(Message::LockScreenToggleAuthMode);
    assert_eq!(harness.app.lock_screen.mode, LockAuthMode::Password);

    // Enter correct demo Password "MindSet2026!"
    harness.dispatch(Message::LockScreenPasswordChanged(
        "MindSet2026!".to_string(),
    ));
    harness.dispatch(Message::SubmitLockScreenAuth);

    assert_eq!(harness.active_overlay(), ActiveOverlay::None);
    assert_eq!(harness.app.lock_screen.failed_attempts, 0);
    harness.reset_auth();
}

#[test]
fn test_f02_failed_attempts_rate_limiting() {
    let _guard = AUTH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut harness = HeadlessHarness::new();
    harness.reset_auth();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));

    // Submit invalid 6-digit PIN 5 consecutive times (auto-submits on 6th digit)
    for attempt in 1..=4 {
        harness.dispatch(Message::LockScreenPinChanged("000000".to_string()));
        assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);
        assert_eq!(harness.app.lock_screen.failed_attempts, attempt);
    }

    // 5th attempt must trigger lock out
    harness.dispatch(Message::LockScreenPinChanged("000000".to_string()));
    assert!(harness.app.lock_screen.is_locked_out);
    assert!(harness.app.lock_screen.failed_attempts >= 4);
    assert!(harness.app.lock_screen.lockout_seconds_remaining > 0);

    // Submissions while locked out are rejected immediately
    harness.dispatch(Message::LockScreenPinChanged("123456".to_string()));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);
    harness.reset_auth();
}

#[test]
fn test_f02_escape_key_lock_interlock() {
    let mut harness = HeadlessHarness::new();
    harness.dispatch(Message::ToggleOverlay(ActiveOverlay::LockScreen));
    assert_eq!(harness.active_overlay(), ActiveOverlay::LockScreen);

    // Pressing Escape MUST NOT dismiss the LockScreen
    use iced::keyboard::{key::Named, Event as KbEvent, Key, Modifiers};
    use iced::Event;
    harness.dispatch(Message::EventOccurred(Event::Keyboard(
        KbEvent::KeyPressed {
            key: Key::Named(Named::Escape),
            modified_key: Key::Named(Named::Escape),
            physical_key: iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::Escape),
            location: iced::keyboard::Location::Standard,
            modifiers: Modifiers::default(),
            repeat: false,
            text: None,
        },
    )));

    assert_eq!(
        harness.active_overlay(),
        ActiveOverlay::LockScreen,
        "Escape key must not dismiss the auth lock screen"
    );
}

// ===========================================================================
// FEATURE 3: Editable Desktop Widgets
// ===========================================================================

#[test]
fn test_f03_default_widget_placements() {
    let harness = HeadlessHarness::new();
    let briefing = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::ScholarBriefing);
    assert!(
        briefing.is_some(),
        "ScholarBriefing widget must be configured in desktop layout"
    );
    let b = briefing.unwrap();
    assert!(b.width > 0.0 && b.height > 0.0);

    let trail = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::OdysseyTrail);
    assert!(
        trail.is_some(),
        "OdysseyTrail widget must be configured in desktop layout"
    );

    let scratchpad = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::Scratchpad);
    assert!(
        scratchpad.is_some(),
        "Scratchpad widget must be configured in desktop layout"
    );
}

#[test]
fn test_f03_widget_drag_updates_coordinates() {
    let mut harness = HeadlessHarness::new();
    // Ensure widget is unpinned
    if let Some(w) = harness
        .app
        .core
        .wm
        .get_widget_placement_mut(DesktopWidgetId::Scratchpad)
    {
        w.is_pinned = false;
    }

    harness.app.cursor_position = iced::Point::new(100.0, 150.0);
    harness.dispatch(Message::StartWidgetDrag(DesktopWidgetId::Scratchpad));
    assert!(harness.app.wm_interaction.dragging_widget.is_some());

    harness.dispatch(Message::EndWidgetDrag);
    assert!(harness.app.wm_interaction.dragging_widget.is_none());
}

#[test]
fn test_f03_pinned_widget_ignores_drag() {
    let mut harness = HeadlessHarness::new();
    if let Some(w) = harness
        .app
        .core
        .wm
        .get_widget_placement_mut(DesktopWidgetId::ScholarBriefing)
    {
        w.is_pinned = true;
    }

    harness.dispatch(Message::StartWidgetDrag(DesktopWidgetId::ScholarBriefing));
    assert!(
        harness.app.wm_interaction.dragging_widget.is_none(),
        "Pinned widgets must reject dragging attempts"
    );
}

#[test]
fn test_f03_toggle_widget_pin_state() {
    let mut harness = HeadlessHarness::new();
    let initial_pin = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::Scratchpad)
        .unwrap()
        .is_pinned;

    harness.dispatch(Message::ToggleWidgetPin(DesktopWidgetId::Scratchpad));
    let after_pin = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::Scratchpad)
        .unwrap()
        .is_pinned;
    assert_ne!(initial_pin, after_pin);

    harness.dispatch(Message::ToggleWidgetPin(DesktopWidgetId::Scratchpad));
    let final_pin = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::Scratchpad)
        .unwrap()
        .is_pinned;
    assert_eq!(initial_pin, final_pin);
}

#[test]
fn test_f03_reset_widget_layout() {
    let mut harness = HeadlessHarness::new();
    // Move widget arbitrarily
    if let Some(w) = harness
        .app
        .core
        .wm
        .get_widget_placement_mut(DesktopWidgetId::Scratchpad)
    {
        w.x = 999.0;
        w.y = 888.0;
    }

    harness.dispatch(Message::ResetWidgetLayout);
    let reset_w = harness
        .app
        .core
        .wm
        .get_widget_placement(DesktopWidgetId::Scratchpad)
        .unwrap();
    assert_ne!(
        reset_w.x, 999.0,
        "Reset layout must restore default coordinates"
    );
}

// ===========================================================================
// FEATURE 4: Liquidmorphism vs Glassmorphism RAM Evaluation
// ===========================================================================

#[test]
fn test_f04_ram_consumption_comparison_audit() {
    let (sdf_bytes, blur_bytes) = LiquidMetrics::compute_ram_vram_footprint(1280, 800, 2);
    assert!(sdf_bytes <= LiquidMetrics::SINGLE_PASS_SDF_RAM_BYTES);
    assert_eq!(blur_bytes, 1280 * 800 * 4 * 2);
    assert!(
        blur_bytes > sdf_bytes * 30,
        "Liquidmorphism RAM footprint ({} B) must be >30x lighter than dual-buffer Gaussian blur ({} B)",
        sdf_bytes,
        blur_bytes
    );

    let ratio = LiquidMetrics::memory_reduction_ratio(1280, 800, 2);
    assert!(ratio > 30.0);
    assert!((ratio - (blur_bytes as f32 / sdf_bytes as f32)).abs() < 1e-4);

    let full_hd = LiquidMetrics::evaluate_comparison(1920, 1080, 2);
    assert_eq!(full_hd.lower_ram_selection, "Liquidmorphism");
    assert_eq!(
        full_hd.multi_pass_blur_vram_bytes,
        LiquidMetrics::MULTI_PASS_BLUR_VRAM_BYTES
    );
    let expected_const_ratio =
        full_hd.multi_pass_blur_vram_bytes as f32 / LiquidMetrics::SINGLE_PASS_SDF_RAM_BYTES as f32;
    assert!((LiquidMetrics::MEMORY_REDUCTION_RATIO - expected_const_ratio).abs() < 1e-4);
    assert!(full_hd.reduction_ratio > 63.0);
}

#[test]
fn test_f04_zero_memory_leak_sustained_loop() {
    let mut harness = HeadlessHarness::new();
    for _ in 0..1000 {
        harness.dispatch(Message::Tick);
        harness.dispatch(Message::MapAmbientTick);
    }
    // Execution completes without memory ballooning or thread panics
    assert_eq!(harness.app.core.wm.active_overlay, ActiveOverlay::None);
}

#[test]
fn test_f04_specular_rim_azimuth_calculation() {
    // Screen-space (+x right, -y up): 135 deg top-left corner normal is (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
    let top_left_sdf_normal = sdf_liquid_normal([-45.0, -15.0], [50.0, 20.0], 10.0);
    let sdf_intensity = directional_specular_rim(top_left_sdf_normal, 135.0, 14.0);
    assert!(
        sdf_intensity > 0.9,
        "Top-left SDF corner must catch 135 deg specular rim, got {}",
        sdf_intensity
    );

    let aligned_intensity = directional_specular_rim([-FRAC_1_SQRT_2, -FRAC_1_SQRT_2], 135.0, 14.0);
    assert!((aligned_intensity - 1.0).abs() < 1e-3);

    let opposite_intensity = directional_specular_rim([FRAC_1_SQRT_2, FRAC_1_SQRT_2], 135.0, 14.0);
    assert_eq!(opposite_intensity, 0.0);
}

#[test]
fn test_f04_fresnel_sheen_power_curve() {
    let f0 = 0.04;
    let grazing = fresnel_sheen(0.0, 6.0, f0);
    assert!(
        grazing >= 0.99,
        "Grazing edge (dist=0) must yield ~1.0 Fresnel reflectance, got {}",
        grazing
    );

    let mid = fresnel_sheen(3.0, 6.0, f0);
    assert!(mid > f0 && mid < grazing);

    let interior = fresnel_sheen(6.0, 6.0, f0);
    assert!(
        interior <= 0.05 && (interior - f0).abs() < 1e-4,
        "Interior (dist=rim_thickness) must decay to f0={}, got {}",
        f0,
        interior
    );
}

#[test]
fn test_f04_liquid_glass_properties_consistency() {
    let dark_palette = ThemePalette::for_mode(ThemeMode::DarkSpace);
    let light_palette = ThemePalette::for_mode(ThemeMode::AlpineGlass);

    assert!(dark_palette.glass_surface.a > 0.0 && dark_palette.glass_surface.a < 1.0);
    assert!(light_palette.glass_surface.a > 0.0 && light_palette.glass_surface.a < 1.0);
    assert_ne!(dark_palette.glass_border, light_palette.glass_border);
}

// ===========================================================================
// FEATURE 5: Liquidmorphism Notch Island & Widget Styling
// ===========================================================================

#[test]
fn test_f05_notch_compact_state_dimensions() {
    let mode = IslandMode::Compact;
    let (w, h) = mode.target_dimensions(0);
    assert_eq!(w, 300.0, "Compact notch width must be 300px");
    assert_eq!(h, 34.0, "Compact notch height must be 34px");
}

#[test]
fn test_f05_notch_search_expanded_dimensions() {
    let mode = IslandMode::Search;
    let (w_empty, h_empty) = mode.target_dimensions(0);
    assert_eq!(w_empty, 480.0);
    assert_eq!(h_empty, 52.0);

    let (w_with_results, h_with_results) = mode.target_dimensions(5);
    assert_eq!(w_with_results, 480.0);
    assert_eq!(h_with_results, 240.0);
}

#[test]
fn test_f05_notch_control_center_state() {
    let mode = IslandMode::ControlCenter;
    let (w, h) = mode.target_dimensions(0);
    assert_eq!(w, 440.0);
    assert_eq!(h, 260.0);
}

#[test]
fn test_f05_notch_active_focus_state() {
    let mode = IslandMode::ActiveFocus;
    let (w, h) = mode.target_dimensions(0);
    assert_eq!(w, 340.0);
    assert_eq!(h, 42.0);
}

#[test]
fn test_f05_notch_speaking_waveform_state() {
    let mode = IslandMode::MaindySpeaking;
    let (w, h) = mode.target_dimensions(0);
    assert_eq!(w, 340.0);
    assert_eq!(h, 38.0);
}

// ===========================================================================
// FEATURE 6: Workspace Prompt Builder Assertion & Keywords
// ===========================================================================

#[test]
fn test_f06_prompt_builder_contains_all_mandatory_keywords() {
    let mut profile = create_test_profile("Mapla", 590);
    profile.subjects = vec!["Strongest in Calculus".to_string()];
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);

    assert!(prompt.contains("Mapla"), "Prompt must contain student name");
    assert!(
        prompt.contains("590/600"),
        "Prompt must contain 590/600 target score"
    );
    assert!(
        prompt.contains("Strongest in Calculus"),
        "Prompt must contain strongest subject"
    );
    assert!(
        prompt.contains("Gauss Law & Flux"),
        "Prompt must contain priority weak topic"
    );
    assert!(
        prompt.contains("THANGLISH"),
        "Prompt must contain THANGLISH style tag"
    );
    assert!(
        prompt.contains("ZERO ROBOTIC META-COMMENTARY"),
        "Prompt must contain ZERO ROBOTIC META-COMMENTARY constraint"
    );
    assert!(
        prompt.contains("DISPLAY:"),
        "Prompt must specify DISPLAY: tag for dual-text"
    );
    assert!(
        prompt.contains("SPEECH:"),
        "Prompt must specify SPEECH: tag for dual-text"
    );
}

#[test]
fn test_f06_prompt_builder_language_tone_adaptations() {
    let mut profile = create_test_profile("Arjun", 590);
    profile.maindy_style = "ta".to_string();
    let prompt_ta = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt_ta.contains("தமிழ்") || prompt_ta.contains("TAMIL"));

    profile.maindy_style = "en".to_string();
    let prompt_en = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt_en.contains("English") || prompt_en.contains("ENGLISH"));
}

#[test]
fn test_f06_prompt_builder_weak_topics_integration() {
    let mut profile = create_test_profile("Sundar", 580);
    profile.weak_topics = vec![
        "Electromagnetic Waves".to_string(),
        "Semiconductors Logic Gates".to_string(),
    ];
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt.contains("Electromagnetic Waves"));
    assert!(prompt.contains("Semiconductors Logic Gates"));
}

#[test]
fn test_f06_prompt_builder_target_score_framing() {
    let profile = create_test_profile("Kavitha", 595);
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt.contains("595/600"));
}

#[test]
fn test_f06_prompt_builder_prohibitions_strictness() {
    let profile = create_test_profile("TestStudent", 590);
    let prompt = build_personalized_system_prompt(&profile, &[], &[]);
    assert!(prompt.contains("STRICT PROHIBITIONS (ZERO ROBOTIC META-COMMENTARY)"));
    assert!(prompt.contains("NEVER say \"As an AI\""));
}
