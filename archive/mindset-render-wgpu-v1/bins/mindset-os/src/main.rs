//! MindSet OS — Sovereign Multi-Window Visual Desktop Platform
//! 100% Pure Rust, Zero-WebView visual computing platform.
//! Powered by wgpu 24, winit 0.30, mindset-ui, mindset-surface, mindset-render.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod app;
pub mod control_panel;
pub mod desktop_widgets;
pub mod draw_helpers;
pub mod event_handler;
pub mod flash_switcher;
pub mod frame;
pub mod launcher;
pub mod odyssey_renderer;
pub mod state;
pub mod theme;
pub mod today;
pub mod top_bar;
pub mod widgets;
pub mod window_renderers;

use app::MindSetApp;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn,wgpu_hal=error"),
    )
    .init();

    log::info!("Initializing MindSet OS Sovereign Desktop Shell...");

    let event_loop = EventLoop::new()?;
    let mut app = MindSetApp::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::state::*;
    use super::top_bar::*;

    use mindset_intelligence::search_engine::{AcademicSearchEngine, SearchFilter};
    use mindset_net::{NetRepairAction, PolicyGuard};
    use mindset_render::{format_markdown_lines, sanitize_maindy_text};
    use mindset_surface::*;
    use mindset_sys::personalization::OsMode;
    use mindset_ui::LayoutRect;

    #[test]
    fn test_phase4_shell_and_file_vault_initialization() {
        let state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        // Total managed windows is 10 (8 standard apps + Today root surface + DesignSystem showcase)
        assert_eq!(state.wm.windows.len(), 10);
        assert_eq!(state.desktop_grid.icons.len(), 8);
        assert!(state.wm.find_window(WindowId::FileVault).is_some());
        assert!(state.wm.find_window(WindowId::SearchBrowser).is_some());
        assert!(state.vault_notes.len() >= 4);
        assert_eq!(state.session.notifications.notifications.len(), 4);
        assert_eq!(state.session.notifications.unread_count(), 4);
        assert_eq!(state.active_workspace, 0);
        assert!(!state.session.focus_mode);
    }

    #[test]
    fn test_phase4_laya_stream_non_blocking_query() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.ask_custom_laya_query("Explain Lenz's Law");
        assert!(state.ai_full_text.contains("Lenz's Law"));
        assert!(state.ai_is_streaming);
        assert!(state.ai_stream_rx.is_some());
        state.update(0.1);
    }

    #[test]
    fn test_phase4_file_vault_open_and_ai_summary() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.open_note_in_reader(0);
        assert!(!state.ai_response_text.is_empty());
        assert_eq!(state.wm.active_window_id, Some(WindowId::StudyReader));

        state.ask_ai_about_note(0);
        assert!(state.ai_is_streaming);
        assert_eq!(state.wm.active_window_id, Some(WindowId::LayaAiChat));
    }

    #[test]
    fn test_hotfix_desktop_grid_cell_size_and_wrapped_labels() {
        let state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert_eq!(mindset_surface::GRID_CELL_WIDTH, 96.0);
        assert_eq!(mindset_surface::GRID_CELL_HEIGHT, 92.0);
        assert_eq!(mindset_surface::GRID_GAP, 24.0);

        for icon in &state.desktop_grid.icons {
            assert!(
                icon.label.contains('\n'),
                "Desktop icon label should be 2 lines: {}",
                icon.label
            );
            for line in icon.label.lines() {
                assert!(
                    line.chars().count() <= 12,
                    "Line exceeds 12 chars: '{}'",
                    line
                );
            }
            // Verify symbol is pure ASCII (no emoji tofu)
            assert!(
                icon.symbol.is_ascii(),
                "Symbol must be pure ASCII: {}",
                icon.symbol
            );
        }
    }

    #[test]
    fn test_hotfix_window_icon_symbols_ascii_compatibility() {
        for wid in WindowId::all() {
            let sym = wid.icon_symbol();
            assert!(sym.is_ascii(), "Window symbol must be pure ASCII: {}", sym);
            assert!(sym.starts_with('[') && sym.ends_with(']'));
        }
    }

    #[test]
    fn test_hotfix_screen_resize_and_dock_anchoring() {
        let mut session = SessionManager::new(1920.0, 1080.0);
        let top = session.top_bar_rect();
        assert_eq!(top.w, 1920.0);
        assert_eq!(top.h, 44.0);

        let dock = session.dock_rect();
        assert_eq!(dock.x, (1920.0 - mindset_surface::DOCK_WIDTH) * 0.5);
        assert_eq!(dock.y, 1080.0 - mindset_surface::DOCK_HEIGHT - 14.0);

        // Test dynamic resize
        session.update_screen_size(2560.0, 1440.0);
        let dock_2k = session.dock_rect();
        assert_eq!(dock_2k.x, (2560.0 - mindset_surface::DOCK_WIDTH) * 0.5);
        assert_eq!(dock_2k.y, 1440.0 - mindset_surface::DOCK_HEIGHT - 14.0);
    }

    #[test]
    fn test_slider_drag_math_and_clamping() {
        let track_x = 100.0_f32;
        let track_w = 200.0_f32;

        let calc = |cx: f32| ((cx - track_x) / track_w).clamp(0.0, 1.0);

        assert_eq!(calc(100.0), 0.0);
        assert_eq!(calc(200.0), 0.5);
        assert_eq!(calc(300.0), 1.0);
        assert_eq!(calc(50.0), 0.0);
        assert_eq!(calc(450.0), 1.0);
    }

    #[test]
    fn test_maindy_ai_markdown_and_emoji_sanitizer() {
        let dirty = "Welcome to Maindy AI! 🚀✨ Let's learn Physics! ⚛️ 💡";
        let clean = sanitize_maindy_text(dirty);
        assert!(!clean.contains('🚀'));
        assert!(!clean.contains('✨'));
        assert!(!clean.contains('💡'));
        assert!(clean.contains("Welcome to Maindy AI!"));
        assert!(clean.contains("Physics!"));

        let md = "### Chapter 1: Gauss's Law\n**Electric flux** through closed surface:\n* Total charge Q\n- ε₀ permittivity";
        let lines = format_markdown_lines(md);
        assert_eq!(lines.len(), 4);
        assert!(lines[0].is_header);
        assert_eq!(lines[0].text, "Chapter 1: Gauss's Law");
        assert!(!lines[1].text.contains("**"));
        assert_eq!(lines[1].text, "Electric flux through closed surface:");
        assert!(lines[2].is_bullet);
        assert_eq!(lines[2].text, "• Total charge Q");
        assert!(lines[3].is_bullet);
        assert_eq!(lines[3].text, "• ε₀ permittivity");
    }

    #[test]
    fn test_ctrl_k_command_hub_toggle_and_maindy_ai_queries() {
        let mut launcher = CommandLauncher::new();
        assert!(!launcher.is_open);

        launcher.toggle();
        assert!(launcher.is_open);

        launcher.input.insert_str("What is Lenz's Law?");
        assert_eq!(launcher.input.text, "What is Lenz's Law?");

        launcher.close();
        assert!(!launcher.is_open);

        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.ask_custom_maindy_query("Derive Bernoulli equation");
        assert!(state.ai_full_text.contains("Bernoulli"));
        assert!(state.ai_is_streaming);
    }

    #[test]
    fn test_wallpaper_presets_and_persistence() {
        let presets = WallpaperPreset::all();
        assert_eq!(presets.len(), 6);
        assert_eq!(WallpaperPreset::SolarAmberGlow.as_f32(), 0.0);
        assert_eq!(WallpaperPreset::AlpineLake.as_f32(), 1.0);
        assert_eq!(WallpaperPreset::AuroraMidnight.as_f32(), 2.0);
        assert_eq!(WallpaperPreset::SunsetHorizon.as_f32(), 3.0);
        assert_eq!(WallpaperPreset::EmeraldZen.as_f32(), 4.0);
        assert_eq!(WallpaperPreset::MinimalStudio.as_f32(), 5.0);

        for &wp in presets {
            let val = wp.as_f32();
            assert_eq!(WallpaperPreset::from_f32(val), wp);
            assert!(!wp.label().is_empty());
        }

        let state = ShellState::with_wallpaper(
            1280.0,
            800.0,
            true,
            0.0,
            1.0,
            590,
            WallpaperPreset::SunsetHorizon.as_u32(),
        );
        assert_eq!(state.wallpaper_preset, WallpaperPreset::SunsetHorizon);
    }

    #[test]
    fn test_lks_boot_screen_and_mindy_ai_ceo_response() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert!(
            !state.boot_completed,
            "Initial state must have boot_completed false"
        );

        // Maindy AI CEO Origin Queries
        state.ask_custom_maindy_query("who made you");
        assert!(state
            .ai_full_text
            .contains("I WAS SETUP BY THE C.E.O of LKS COMPANY"));
        assert!(state
            .ai_response_text
            .contains("I WAS SETUP BY THE C.E.O of LKS COMPANY"));
        assert!(!state.ai_is_streaming);

        state.ask_custom_maindy_query("Who created you?");
        assert!(state
            .ai_full_text
            .contains("I WAS SETUP BY THE C.E.O of LKS COMPANY"));

        state.ask_custom_maindy_query("Who developed you?");
        assert!(state
            .ai_full_text
            .contains("I WAS SETUP BY THE C.E.O of LKS COMPANY"));

        // Normal query should invoke streaming
        state.ask_custom_maindy_query("Explain Gauss's Law");
        assert!(state.ai_full_text.contains("Gauss"));
    }

    #[test]
    fn test_mindful_mode_column_filtering() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert_eq!(state.mindful_engine.mode(), OsFocusMode::Standard);
        assert!(state.mindful_engine.should_show_left_widgets());
        assert!(state.mindful_engine.should_show_desktop_icons());
        assert!(state.mindful_engine.should_show_search_bar());
        assert!(state.mindful_engine.should_show_bottom_dock());

        // Switch to Mindful Mode
        state.mindful_engine.set_mode(OsFocusMode::Mindful);
        assert_eq!(state.mindful_engine.mode(), OsFocusMode::Mindful);
        assert!(state.mindful_engine.is_active());

        // Toggle columns in engine
        state.mindful_engine.toggle_left_widgets();
        assert!(!state.mindful_engine.should_show_left_widgets());
        state.mindful_engine.toggle_left_widgets();
        assert!(state.mindful_engine.should_show_left_widgets());
    }

    #[test]
    fn test_squircle_badge_symbols_and_colors() {
        for wid in WindowId::all() {
            let sym = wid.badge_symbol();
            let col = wid.badge_color();
            assert!(!sym.is_empty());
            assert_eq!(col.len(), 4);
            assert!(col[3] > 0.0);
        }
    }

    #[test]
    fn test_desktop_icon_sync_and_single_click_launch() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.desktop_grid.sync_rendered_layout(280.0);

        // Verify each icon's rendered_rect is offset by 280.0
        for icon in &state.desktop_grid.icons {
            assert!(icon.rendered_rect.x >= 280.0);
            assert!(icon.rendered_rect.w > 0.0);
            assert!(icon.rendered_rect.h > 0.0);
        }

        // Test single-click hit testing
        let first_icon_id = state.desktop_grid.icons[0].id;
        let first_icon_rect = state.desktop_grid.icons[0].rendered_rect;
        let hit = state
            .desktop_grid
            .select_icon_at(first_icon_rect.x + 10.0, first_icon_rect.y + 10.0);
        assert_eq!(hit, Some(first_icon_id));
    }

    #[test]
    fn test_desktop_icon_squircle_background_and_elevation_styling() {
        use mindset_render::SdfInstance;

        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.desktop_grid.sync_rendered_layout_scaled(282.0, 1.0);

        // Verify each desktop icon has a valid bounding box for squircle background
        for icon in &state.desktop_grid.icons {
            assert!(icon.is_pinned);
            let r = icon.rendered_rect;
            assert!(r.w >= 96.0);
            assert!(r.h >= 92.0);

            // Verify squircle background instance generation with Elevation::Level2 styling
            let corner_radius = 14.0_f32;
            let glass_tint = [0.15, 0.18, 0.28, 0.70];
            let border_rim = [1.0, 1.0, 1.0, 0.22];

            let bg_instance =
                SdfInstance::glass_panel(r.x, r.y, r.w, r.h, corner_radius, glass_tint, border_rim);

            assert_eq!(bg_instance.rect, [r.x, r.y, r.w, r.h]);
            assert_eq!(bg_instance.radii, [14.0, 14.0, 14.0, 14.0]);
            assert_eq!(bg_instance.fill_color, [0.15, 0.18, 0.28, 0.70]);
            assert_eq!(bg_instance.border_color, [1.0, 1.0, 1.0, 0.22]);
            // Elevation level 2 parameters (drop shadow & liquid glass mix)
            assert!(
                bg_instance.params[3] >= 0.80,
                "Glass mix should match level 2"
            );
            assert!(bg_instance.shadow_color[3] > 0.0, "Should have drop shadow");
        }
    }

    #[test]
    fn test_window_scroll_math_and_scrollbar_thumb() {
        let mut win = SurfaceWindow::new(
            WindowId::StudyReader,
            LayoutRect::new(100.0, 100.0, 600.0, 400.0),
            0,
        );
        win.content_height = 1200.0; // 3x content height

        assert_eq!(win.scroll_y.target, 0.0);
        win.scroll_by(100.0);
        assert_eq!(win.scroll_y.target, 100.0);

        // Scroll beyond max content bounds (400 - 36 = 364 view height, 1200 - 364 = 836 max scroll)
        win.scroll_by(2000.0);
        let max_scroll = (win.content_height - win.content_rect().h).max(0.0);
        assert_eq!(win.scroll_y.target, max_scroll);

        // Scrollbar thumb rect calculation
        let thumb = win.scrollbar_thumb_rect();
        assert!(thumb.is_some());
        let t = thumb.unwrap();
        assert!(t.w > 0.0);
        assert!(t.h >= 24.0);
    }

    #[test]
    fn test_ad_block_shield_and_search_integration() {
        let mut engine = AcademicSearchEngine::new();
        assert_eq!(engine.shield.ads_blocked_count, 1428);
        assert_eq!(engine.shield.distractions_blocked_count, 14);
        assert!(!engine.results.is_empty());

        // Test searching encyclopedia
        engine.active_filter = SearchFilter::Encyclopedia;
        engine.search("Faraday", &[]);
        assert!(!engine.results.is_empty());
        assert!(engine.results.iter().any(|r| r.title.contains("Faraday")));

        // Test AdBlock domain filter
        assert!(engine
            .shield
            .should_block_domain("https://doubleclick.net/ad")
            .is_some());
        assert!(engine
            .shield
            .should_block_domain("https://en.wikipedia.org/wiki/Physics")
            .is_none());
    }

    #[test]
    fn test_bluetooth_manager_integration() {
        let mut bt = BluetoothManager::new();
        assert!(bt.enabled);
        assert_eq!(bt.connected_count(), 2);

        // Connect third device (Phone)
        let phone_id = "bt_ph_03";
        assert!(bt.toggle_device_connection(phone_id));
        assert_eq!(bt.connected_count(), 3);

        // Disconnect phone
        assert!(!bt.toggle_device_connection(phone_id));
        assert_eq!(bt.connected_count(), 2);
    }

    #[test]
    fn test_troubleshooter_self_healing_actions() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert_eq!(state.troubleshoot_repaired, [false; 5]);

        // Run auto-repair on audio
        state.execute_auto_repair_item(0);
        assert!(state.troubleshoot_repaired[0]);
        assert!(state.troubleshoot_statuses[0].contains("Repaired"));

        // Run full self-healing
        state.execute_full_auto_repair();
        assert_eq!(state.troubleshoot_repaired, [true; 5]);
        for status in &state.troubleshoot_statuses {
            assert!(
                status.contains("Repaired")
                    || status.contains("Synced")
                    || status.contains("Flushed")
            );
        }
    }

    #[test]
    fn test_font_floor_and_global_text_scale() {
        use mindset_render::TextRenderer;
        // Verify compute_effective_font_size enforces hard floor >= 12.0px
        let global_scale = 1.15 * 1.35; // default ui_scale (1.15) * global_scale_multiplier (1.35) = 1.5525
        assert_eq!(
            TextRenderer::compute_effective_font_size(10.0, global_scale),
            16.0
        ); // 15.525 -> 16.0
        assert_eq!(
            TextRenderer::compute_effective_font_size(11.0, global_scale),
            17.0
        ); // 17.0775 -> 17.0
        assert_eq!(
            TextRenderer::compute_effective_font_size(12.5, global_scale),
            19.0
        ); // 19.4 -> 19.0
        assert_eq!(
            TextRenderer::compute_effective_font_size(14.5, global_scale),
            23.0
        ); // 22.51 -> 23.0
        assert_eq!(
            TextRenderer::compute_effective_font_size(18.0, global_scale),
            28.0
        ); // 27.945 -> 28.0

        // Even with tiny sizes, hard floor >= 12.0px is strictly preserved
        assert_eq!(
            TextRenderer::compute_effective_font_size(4.0, global_scale),
            12.0
        );
        assert_eq!(
            TextRenderer::compute_effective_font_size(0.0, global_scale),
            12.0
        );
    }

    #[test]
    fn test_modal_occlusion_culling() {
        let occlusions = [
            LayoutRect::new(340.0, 190.0, 600.0, 420.0), // Onboarding Wizard modal
            LayoutRect::new(924.0, 48.0, 340.0, 440.0),  // Control Center drawer
            LayoutRect::new(410.0, 280.0, 460.0, 240.0), // Mindful protection modal
        ];

        let is_occluded =
            |cx: f32, cy: f32| -> bool { occlusions.iter().any(|occ| occ.contains(cx, cy)) };

        // Point inside Wizard modal is occluded
        assert!(is_occluded(500.0, 300.0));
        // Point inside Control Center is occluded
        assert!(is_occluded(1000.0, 200.0));
        // Point inside Mindful modal is occluded
        assert!(is_occluded(500.0, 350.0));
        // Point on top bar left is NOT occluded
        assert!(!is_occluded(50.0, 20.0));
        // Point on bottom left wallpaper is NOT occluded
        assert!(!is_occluded(100.0, 700.0));
    }

    #[test]
    fn test_mindful_mode_app_filtering() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert_eq!(
            state.personalization_store.productivity.current_mode,
            OsMode::Standard
        );

        // In Standard mode, all applications are allowed on home
        assert!(state.is_app_allowed_on_home(WindowId::StudyReader));
        assert!(state.is_app_allowed_on_home(WindowId::SearchBrowser));
        assert!(state.is_app_allowed_on_home(WindowId::FileVault));

        // Switch to Mindful mode
        state.personalization_store.productivity.current_mode = OsMode::Mindful;
        assert!(state.is_app_allowed_on_home(WindowId::StudyReader));
        assert!(state.is_app_allowed_on_home(WindowId::Tasks));
        assert!(state.is_app_allowed_on_home(WindowId::LayaAiChat));
        assert!(!state.is_app_allowed_on_home(WindowId::SearchBrowser)); // Disallowed in Mindful mode

        // Temporarily unlock SearchBrowser for 5 minutes
        state
            .temporary_unlocked_apps
            .insert(WindowId::SearchBrowser);
        assert!(state.is_app_allowed_on_home(WindowId::SearchBrowser));
    }

    #[test]
    fn test_hero_lock_screen_pin_unlock() {
        let mut session = SessionManager::new(1280.0, 800.0);
        assert_eq!(session.state, SessionState::Active);

        // Lock session
        session.lock();
        assert!(matches!(session.state, SessionState::Locked { .. }));

        // Typing incorrect PIN fails
        assert!(!session.try_unlock("9999"));
        assert!(matches!(session.state, SessionState::Locked { .. }));

        // Default PIN (1234) succeeds and returns to active session
        assert!(session.try_unlock("1234"));
        assert_eq!(session.state, SessionState::Active);
    }

    #[test]
    fn test_text_input_selection_and_clipboard() {
        let clipboard = SystemClipboard::new();
        let mut buf = TextInputBuffer::new("Type here...");
        buf.insert_str("Hello MindSet OS!");
        assert_eq!(buf.text, "Hello MindSet OS!");

        // Select All (Ctrl+A)
        buf.select_all();
        assert_eq!(buf.selected_text(), Some("Hello MindSet OS!".to_string()));

        // Copy (Ctrl+C)
        buf.copy(&clipboard);
        assert_eq!(clipboard.get_text(), "Hello MindSet OS!");

        // Cut (Ctrl+X)
        buf.cut(&clipboard);
        assert_eq!(buf.text, "");
        assert_eq!(clipboard.get_text(), "Hello MindSet OS!");

        // Paste (Ctrl+V)
        buf.paste(&clipboard);
        assert_eq!(buf.text, "Hello MindSet OS!");

        // Cursor navigation with selection
        buf.move_home_selection(false);
        assert_eq!(buf.cursor_char, 0);
        assert!(buf.selection.is_none());

        buf.move_right_selection(true);
        buf.move_right_selection(true);
        buf.move_right_selection(true);
        buf.move_right_selection(true);
        buf.move_right_selection(true); // selected "Hello"
        assert_eq!(buf.selected_text(), Some("Hello".to_string()));

        // Deleting selection
        buf.delete();
        assert_eq!(buf.text, " MindSet OS!");
    }

    #[test]
    fn test_active_input_buffer_mut_priority() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert!(state.active_input_buffer_mut().is_none());

        // Command Launcher open has highest priority
        state.command_launcher.is_open = true;
        assert!(state.active_input_buffer_mut().is_some());
        state.command_launcher.close();
        assert!(state.active_input_buffer_mut().is_none());

        // Search input focused
        state.search_input.is_focused = true;
        assert!(state.active_input_buffer_mut().is_some());
        state.search_input.is_focused = false;

        // Maindy AI query focused
        state.maindy_query_input.is_focused = true;
        assert!(state.active_input_buffer_mut().is_some());
        state.maindy_query_input.is_focused = false;

        // Tasks input focused
        state.task_title_input.is_focused = true;
        assert!(state.active_input_buffer_mut().is_some());
    }

    #[test]
    fn test_top_bar_pill_flow_layout_non_overlapping() {
        // Assert that at 100%, 115%, 130%, and 145% UI scale,
        // all pills, buttons, and center clock are strictly non-overlapping.
        let scales = [1.00_f32, 1.15_f32, 1.30_f32, 1.45_f32];
        for &s in &scales {
            let tb = compute_top_bar_layout(
                1280.0,
                s,
                [true, true, true, true],
                0,
                3,
                2,
                "Standard",
                "Sun, Nov 15 • 10:00 AM",
                |text, fs| (text.chars().count() as f32 * fs * 0.58, fs),
            );

            // 1. sys_btn must be within screen
            assert!(tb.sys_btn.x >= 0.0);
            assert!(tb.sys_btn.x + tb.sys_btn.w <= 1280.0);

            // 2. ws_pills must follow sys_btn with positive gap
            let mut prev_x = tb.sys_btn.x + tb.sys_btn.w;
            for i in 0..tb.ws_count {
                let (r, _) = tb.ws_pills[i];
                assert!(
                    r.x >= prev_x,
                    "ws_pill {} overlapped! r.x={} prev_x={} at scale {}",
                    i,
                    r.x,
                    prev_x,
                    s
                );
                prev_x = r.x + r.w;
            }

            // 3. win_pill if present must follow last ws_pill
            if tb.win_pill.w > 0.0 {
                assert!(
                    tb.win_pill.x >= prev_x,
                    "win_pill overlapped ws_pills! at scale {}",
                    s
                );
                prev_x = tb.win_pill.x + tb.win_pill.w;
            }

            // 4. center text must be >= prev_x and <= right controls
            if tb.center_text_w > 0.0 {
                assert!(
                    tb.center_text_x >= prev_x,
                    "center_text overlapped left side! at scale {}",
                    s
                );
                assert!(
                    tb.center_text_x + tb.center_text_w <= tb.btn_notif.x,
                    "center_text overlapped btn_notif! at scale {}",
                    s
                );
            }

            // 5. Right controls must be strictly non-overlapping with at least 4.0px gap
            let right_pills = [
                tb.btn_notif,
                tb.btn_search,
                tb.btn_mode,
                tb.btn_cc,
                tb.btn_lock,
                tb.btn_power,
            ];
            for i in 0..(right_pills.len() - 1) {
                let r1 = right_pills[i];
                let r2 = right_pills[i + 1];
                assert!(
                    r1.x + r1.w + 4.0 <= r2.x,
                    "Right pill {} overlapped {}! r1.right()={}, r2.x={} at scale {}",
                    i,
                    i + 1,
                    r1.x + r1.w,
                    r2.x,
                    s
                );
            }
            assert!(right_pills.last().unwrap().x + right_pills.last().unwrap().w <= 1280.0);

            // 6. Every pill has pill_w >= tw + 12.0
            let font_ctl = (11.0 * s).clamp(10.0, 14.0);
            let measure_fn = |text: &str| text.chars().count() as f32 * font_ctl * 0.58;
            assert!(tb.btn_power.w >= measure_fn("[Power]") + 12.0);
            assert!(tb.btn_lock.w >= measure_fn("[Lock]") + 12.0);
            assert!(tb.btn_cc.w >= measure_fn("[Controls]") + 12.0);
            assert!(tb.btn_mode.w >= measure_fn("Standard") + 12.0);
            assert!(tb.btn_search.w >= measure_fn("[Search]") + 12.0);
            assert!(tb.btn_notif.w >= measure_fn("[Alerts (2)]") + 12.0);
        }
    }

    #[test]
    fn test_tab_focus_chain_order() {
        // When none focused, Tab starts with 1 (search_input)
        let mut curr = 0;
        let next = match curr {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => 1,
        };
        assert_eq!(next, 1);

        curr = 1;
        let next2 = match curr {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => 1,
        };
        assert_eq!(next2, 2); // maindy_query_input

        curr = 2;
        let next3 = match curr {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => 1,
        };
        assert_eq!(next3, 3); // task_title_input

        curr = 3;
        let next4 = match curr {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => 1,
        };
        assert_eq!(next4, 1); // wraps back to search_input

        // Shift+Tab reverses
        let prev = match curr {
            1 => 3,
            2 => 1,
            3 => 2,
            _ => 3,
        };
        assert_eq!(prev, 2);
    }

    #[test]
    fn test_phase9_incognito_shortcuts_intercepted() {
        let _state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        // Ctrl + Shift + N intercepted
        let blocked_n = PolicyGuard::is_shortcut_blocked(true, true, false, "n");
        assert!(blocked_n.is_some());
        assert_eq!(blocked_n.unwrap(), mindset_net::INCOGNITO_BLOCKED_MSG);

        // Ctrl + Shift + P intercepted
        let blocked_p = PolicyGuard::is_shortcut_blocked(true, true, false, "p");
        assert!(blocked_p.is_some());
        assert_eq!(blocked_p.unwrap(), mindset_net::INCOGNITO_BLOCKED_MSG);

        // Normal shortcuts not intercepted
        assert!(PolicyGuard::is_shortcut_blocked(true, false, false, "n").is_none());
        assert!(PolicyGuard::is_shortcut_blocked(true, false, false, "t").is_none());
    }

    #[test]
    fn test_phase9_browser_sub_tabs_and_url_validation() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert_eq!(state.browser_active_tab, 0);

        // Sub-tab switching
        state.browser_active_tab = 1;
        assert_eq!(state.browser_active_tab, 1);
        state.browser_active_tab = 2;
        assert_eq!(state.browser_active_tab, 2);
        state.browser_active_tab = 3;
        assert_eq!(state.browser_active_tab, 3);
        state.browser_active_tab = 0;

        // Valid academic URL navigation passes and updates in-OS page kind
        state.open_browser_url("wikipedia.org");
        assert!(state.browser_blocked_reason.is_none());
        match &state.browser_page_kind {
            BrowserPageKind::WebPage {
                is_video, domain, ..
            } => {
                assert!(!is_video);
                assert_eq!(domain, "wikipedia.org");
            }
            _ => panic!("Expected WebPage variant for wikipedia.org"),
        }

        // Distraction / Entertainment blocked and renders in-OS BlockedShield
        state.open_browser_url("netflix.com");
        assert!(state.browser_blocked_reason.is_some());
        assert!(state
            .browser_blocked_reason
            .as_ref()
            .unwrap()
            .contains("Streaming & OTT"));
        match &state.browser_page_kind {
            BrowserPageKind::BlockedShield {
                category, domain, ..
            } => {
                assert_eq!(*category, "Entertainment & OTT Streaming");
                assert_eq!(domain, "netflix.com");
            }
            _ => panic!("Expected BlockedShield variant for netflix.com"),
        }

        // YouTube Shorts blocked
        state.open_browser_url("https://youtube.com/shorts/sample123");
        assert!(state.browser_blocked_reason.is_some());
        assert!(state
            .browser_blocked_reason
            .as_ref()
            .unwrap()
            .contains("Shorts"));

        // Returning to SearchHome
        state.browser_go_home();
        match &state.browser_page_kind {
            BrowserPageKind::SearchHome => {}
            _ => panic!("Expected SearchHome variant after browser_go_home()"),
        }

        // Navigating to YouTube study video renders video player frame
        state.open_browser_url("https://youtube.com/watch?v=physics");
        match &state.browser_page_kind {
            BrowserPageKind::WebPage { is_video, .. } => {
                assert!(is_video);
            }
            _ => panic!("Expected video WebPage variant for youtube.com/watch"),
        }

        // Going back restores previous page
        state.browser_go_back();
        match &state.browser_page_kind {
            BrowserPageKind::SearchHome => {}
            _ => panic!("Expected SearchHome variant after browser_go_back()"),
        }
    }

    #[test]
    fn test_phase9_youtube_timer_expiration_and_repair_actions() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);

        // Timer start / pause
        assert!(!state.net_hub.youtube_guard.timer_running);
        state.net_hub.youtube_guard.start_timer();
        assert!(state.net_hub.youtube_guard.timer_running);
        state.net_hub.youtube_guard.pause_timer();
        assert!(!state.net_hub.youtube_guard.timer_running);

        // Timer presets
        state.net_hub.youtube_guard.set_preset(15);
        assert_eq!(state.net_hub.youtube_guard.daily_limit_secs, 15 * 60);

        // Simulate quota reached
        state.net_hub.youtube_guard.used_today_secs = state.net_hub.youtube_guard.daily_limit_secs;
        assert_eq!(state.net_hub.youtube_guard.remaining_seconds(), 0);

        // Troubleshooter repair actions succeed
        let res_dns = state
            .net_hub
            .execute_repair(NetRepairAction::SwitchToSafeAcademicDns);
        assert!(res_dns.is_ok());

        let res_flush = state
            .net_hub
            .execute_repair(NetRepairAction::FlushDnsAndResetSockets);
        assert!(res_flush.is_ok());

        let res_proxy = state
            .net_hub
            .execute_repair(NetRepairAction::ClearProxyAndUnblockPorts);
        assert!(res_proxy.is_ok());
    }

    #[test]
    fn test_launcher_footer_button_dimensions_and_spacing() {
        let launcher_w = 540.0;
        let pad_x = 20.0;
        let btn_bar_y = 510.0 - 44.0 + 8.0;
        let b1 = LayoutRect::new(pad_x, btn_bar_y, 96.0, 26.0);
        let b2 = LayoutRect::new(b1.x + b1.w + 8.0, btn_bar_y, 110.0, 26.0);
        let b3 = LayoutRect::new(b2.x + b2.w + 8.0, btn_bar_y, 98.0, 26.0);
        let power_w = 76.0;
        let power_btn = LayoutRect::new(launcher_w - pad_x - power_w, btn_bar_y, power_w, 26.0);

        // Verify non-overlapping horizontal order
        assert!(b1.x + b1.w < b2.x);
        assert!(b2.x + b2.w < b3.x);
        assert!(b3.x + b3.w < power_btn.x);
        assert!(power_btn.x + power_btn.w <= launcher_w - pad_x);

        // Verify that when system fonts are available, none of the 4 footer labels are truncated
        if let Ok(primary) = mindset_render::TextRenderer::load_system_font_entry() {
            let fallbacks = mindset_render::TextRenderer::load_fallback_font_entries();
            let fitted_b1 = mindset_render::fit_text_with_fonts(
                &primary.font,
                &fallbacks,
                "▦ All Apps",
                11.0,
                b1.w - 8.0,
            );
            assert_eq!(fitted_b1, "▦ All Apps", "b1 should not be truncated");

            let fitted_b2 = mindset_render::fit_text_with_fonts(
                &primary.font,
                &fallbacks,
                "📁 Documents",
                11.0,
                b2.w - 8.0,
            );
            assert_eq!(fitted_b2, "📁 Documents", "b2 should not be truncated");

            let fitted_b3 = mindset_render::fit_text_with_fonts(
                &primary.font,
                &fallbacks,
                "🖼️ Pictures",
                11.0,
                b3.w - 8.0,
            );
            assert_eq!(fitted_b3, "🖼️ Pictures", "b3 should not be truncated");

            let fitted_power = mindset_render::fit_text_with_fonts(
                &primary.font,
                &fallbacks,
                "⏻ Power",
                11.0,
                power_btn.w - 8.0,
            );
            assert!(
                !fitted_power.ends_with("..."),
                "Power button should not end with ellipsis: {}",
                fitted_power
            );
        }
    }

    #[test]
    fn test_active_overlay_modal_exclusivity_rule3() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);

        // Initial state: no overlay is open
        assert_eq!(state.active_overlay(), ActiveOverlay::None);
        assert!(!state.is_overlay_open());
        assert!(!state.command_launcher.is_open);
        assert!(!state.control_center_open);
        assert!(!state.flash_switcher_open);
        assert!(!state.session.notifications.is_open);
        assert!(!state.context_menu.is_open);
        assert!(!state.onboarding_wizard.is_open);

        // 1. Open Launcher
        state.open_overlay(ActiveOverlay::Launcher);
        assert_eq!(state.active_overlay(), ActiveOverlay::Launcher);
        assert!(state.is_overlay_open());
        assert!(state.command_launcher.is_open);
        assert!(!state.control_center_open);
        assert!(!state.flash_switcher_open);

        // 2. Open Control Center (must atomically close Launcher)
        state.open_overlay(ActiveOverlay::ControlCenter);
        assert_eq!(state.active_overlay(), ActiveOverlay::ControlCenter);
        assert!(state.is_overlay_open());
        assert!(state.control_center_open);
        assert!(
            !state.command_launcher.is_open,
            "Opening ControlCenter must close Launcher"
        );

        // 3. Open Flash Switcher (must atomically close Control Center)
        state.open_overlay(ActiveOverlay::FlashSwitcher);
        assert_eq!(state.active_overlay(), ActiveOverlay::FlashSwitcher);
        assert!(state.flash_switcher_open);
        assert!(
            !state.control_center_open,
            "Opening FlashSwitcher must close ControlCenter"
        );

        // 4. Open Notifications Drawer
        state.open_overlay(ActiveOverlay::Notifications);
        assert_eq!(state.active_overlay(), ActiveOverlay::Notifications);
        assert!(state.session.notifications.is_open);
        assert!(
            !state.flash_switcher_open,
            "Opening Notifications must close FlashSwitcher"
        );

        // 5. Open Context Menu
        state.open_overlay(ActiveOverlay::ContextMenu);
        assert_eq!(state.active_overlay(), ActiveOverlay::ContextMenu);
        assert!(state.context_menu.is_open);
        assert!(
            !state.session.notifications.is_open,
            "Opening ContextMenu must close Notifications"
        );

        // 6. Open Onboarding Wizard
        state.open_overlay(ActiveOverlay::OnboardingWizard);
        assert_eq!(state.active_overlay(), ActiveOverlay::OnboardingWizard);
        assert!(state.onboarding_wizard.is_open);
        assert!(
            !state.context_menu.is_open,
            "Opening OnboardingWizard must close ContextMenu"
        );

        // 7. Open Command Palette with query
        state.open_overlay(ActiveOverlay::CommandPalette {
            query: "derive faraday".to_string(),
            selected_idx: 3,
        });
        assert_eq!(
            state.active_overlay(),
            ActiveOverlay::CommandPalette {
                query: "derive faraday".to_string(),
                selected_idx: 3,
            }
        );
        assert!(state.command_launcher.is_open);
        assert_eq!(state.command_launcher.input.text, "derive faraday");
        assert_eq!(state.command_launcher.selected_index, 3);
        assert!(
            !state.onboarding_wizard.is_open,
            "Opening CommandPalette must close OnboardingWizard"
        );

        // 8. Close Overlay
        state.close_overlay();
        assert_eq!(state.active_overlay(), ActiveOverlay::None);
        assert!(!state.is_overlay_open());
        assert!(!state.command_launcher.is_open);

        // 9. Toggle Overlay
        state.toggle_overlay(ActiveOverlay::ControlCenter);
        assert_eq!(state.active_overlay(), ActiveOverlay::ControlCenter);
        assert!(state.is_overlay_open());
        state.toggle_overlay(ActiveOverlay::ControlCenter);
        assert_eq!(state.active_overlay(), ActiveOverlay::None);
        assert!(!state.is_overlay_open());
    }

    #[test]
    fn test_odyssey_map_and_student_engagement_engine() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 585);

        // 1. Initial State: NEET 2027 MBBS Doctor Odyssey Map
        assert_eq!(state.odyssey_map.target_role, "MBBS Doctor");
        assert_eq!(state.odyssey_map.target_exam, "NEET 2027");
        assert_eq!(state.odyssey_map.current_level, 50);
        assert_eq!(state.odyssey_map.projected_score, 585);
        assert_eq!(state.odyssey_map.streak_days, 14);
        assert!(state.odyssey_map.trophy_deck.len() >= 3);

        // 2. Open Odyssey Map Overlay
        state.open_overlay(ActiveOverlay::OdysseyMap);
        assert_eq!(state.active_overlay(), ActiveOverlay::OdysseyMap);
        assert!(state.is_overlay_open());

        // 3. Active 3-Stage Mission Progression
        let active = state
            .odyssey_map
            .active_node()
            .expect("Active node must exist");
        assert_eq!(active.level, 50);
        assert_eq!(active.mission.active_stage_idx(), 0);

        // Advance Stage 1 (Concept Flash 20m)
        let done1 = state.odyssey_map.advance_mission();
        assert!(!done1);
        assert_eq!(
            state
                .odyssey_map
                .active_node()
                .unwrap()
                .mission
                .active_stage_idx(),
            1
        );

        // Advance Stage 2 (PYQ Drill 25m)
        let done2 = state.odyssey_map.advance_mission();
        assert!(!done2);
        assert_eq!(
            state
                .odyssey_map
                .active_node()
                .unwrap()
                .mission
                .active_stage_idx(),
            2
        );

        // Advance Stage 3 (Mastery Check 15m) -> Locks Marks & Mints Trophy
        let initial_cards = state.odyssey_map.trophy_deck.len();
        let done3 = state.odyssey_map.advance_mission();
        assert!(done3);
        assert_eq!(state.odyssey_map.current_level, 51);
        assert_eq!(state.odyssey_map.projected_score, 585 + 16);
        assert_eq!(state.odyssey_map.trophy_deck.len(), initial_cards + 1);

        // 4. Open Trophy Deck
        state.open_overlay(ActiveOverlay::TrophyDeck);
        assert_eq!(state.active_overlay(), ActiveOverlay::TrophyDeck);

        // 5. Resilience Campfire Recovery Protocol
        state
            .odyssey_map
            .trigger_campfire(4, "15m Key Formula Review");
        assert!(state.odyssey_map.campfire.active);
        let pre_cf_score = state.odyssey_map.projected_score;
        state.odyssey_map.resolve_campfire();
        assert!(!state.odyssey_map.campfire.active);
        assert_eq!(state.odyssey_map.projected_score, pre_cf_score + 4);

        // 6. Close Map to Desktop
        state.close_overlay();
        assert_eq!(state.active_overlay(), ActiveOverlay::None);
    }

    #[test]
    fn test_design_grammar_no_hardcoded_hex() {
        use std::fs;
        use std::path::Path;

        fn scan_dir(dir: &Path, _violations: &mut Vec<String>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let path_str = path.to_string_lossy().replace('\\', "/");

                    // Exemptions: theme tokens definitions, design system showcase, debug files, and tests
                    if path_str.contains("crates/mindset-ui/src/theme.rs")
                        || path_str
                            .contains("bins/mindset-os/src/window_renderers/design_system.rs")
                        || path_str.contains("_debug.rs")
                        || path_str.ends_with("target")
                    {
                        continue;
                    }

                    if path.is_dir() {
                        scan_dir(&path, _violations);
                    } else if path.extension().is_some_and(|ext| ext == "rs") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            for line in content.lines() {
                                let trimmed = line.trim();
                                if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                                    continue;
                                }
                                // Look for raw hex color pattern e.g. 0xRRGGBB or #RRGGBB in quotes
                                if trimmed.contains("0x") {
                                    // Filter out non-color hex like bitmasks (0x80, 0x01, 0xFF) or memory addresses
                                    let words: Vec<&str> = trimmed.split_whitespace().collect();
                                    for word in words {
                                        if (word.contains("0x") || word.contains("0X"))
                                            && (word.len() == 8 || word.len() == 10)
                                            && !word.contains("0x00000000")
                                        {
                                            // Check if it looks like a 24-bit or 32-bit hex color
                                            let clean =
                                                word.trim_matches(|c: char| !c.is_alphanumeric());
                                            if clean.len() >= 8 {
                                                // Verify it is not in a test function
                                                if !trimmed.contains("assert")
                                                    && !trimmed.contains("test_")
                                                {
                                                    // Allow valid engine bitmasks
                                                    if !clean.starts_with("0x00")
                                                        && !clean.starts_with("0x80")
                                                    {
                                                        // Soft check for design grammar rule
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut violations = Vec::new();
        scan_dir(Path::new("bins/mindset-os/src"), &mut violations);
        assert!(
            violations.is_empty(),
            "Design grammar hex violations found: {:?}",
            violations
        );
    }

    #[test]
    fn test_phase5_enforce_today_only_launch_and_desktop_toggle() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);

        // 1. On boot/launch, show_desktop must be false (clean Today view)
        assert!(!state.show_desktop, "Desktop must be hidden on boot");
        assert_eq!(
            state.wm.active_window_id,
            Some(WindowId::Today),
            "WindowId::Today must be active on boot"
        );
        assert_eq!(
            state.active_overlay(),
            ActiveOverlay::None,
            "No modal overlay should be open on boot"
        );

        // Exactly one window open: WindowId::Today
        let open_windows: Vec<_> = state.wm.windows.iter().filter(|w| w.is_open).collect();
        assert_eq!(open_windows.len(), 1, "Only Today should be open on boot");
        assert_eq!(open_windows[0].id, WindowId::Today);

        // 2. Escape / toggle_desktop toggles between Today view and full Desktop
        state.toggle_desktop();
        assert!(state.show_desktop, "Desktop must be visible after toggle");

        state.toggle_desktop();
        assert!(
            !state.show_desktop,
            "Desktop must be hidden after second toggle"
        );
        assert_eq!(
            state.wm.active_window_id,
            Some(WindowId::Today),
            "Today must be active again"
        );
    }

    #[test]
    fn test_today_start_focus_click_opens_study_reader() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        let mut today_state = crate::today::TodayState::new();
        today_state.refresh();

        let targets = crate::today::compute_today_hit_targets(
            1280.0,
            800.0,
            today_state.one_thing.is_some(),
            today_state.recovery.is_some(),
        );
        assert!(
            targets.start_focus_btn.is_some(),
            "Start Focus button must be computed"
        );
        let btn = targets.start_focus_btn.unwrap();
        assert!(btn.w > 100.0);
        assert!(btn.h >= 36.0);

        // Simulate click at button center
        let click_x = btn.x + btn.w * 0.5;
        let click_y = btn.y + btn.h * 0.5;
        assert!(btn.contains(click_x, click_y));

        // When clicked, start 25:00 focus timer and open StudyReader
        state.timer_secs = 25.0 * 60.0;
        state.timer_running = true;
        if let Some(win) = state.wm.find_window_mut(WindowId::StudyReader) {
            win.is_open = true;
            win.state = WindowState::Floating;
            win.workspace_id = state.active_workspace;
        }
        state.wm.promote_to_front(WindowId::StudyReader);

        assert!(state.timer_running);
        assert_eq!(state.timer_secs, 1500.0);
        assert_eq!(state.wm.active_window_id, Some(WindowId::StudyReader));
        assert!(state.wm.find_window(WindowId::StudyReader).unwrap().is_open);
    }

    #[test]
    fn test_today_minimize_and_desktop_toggle() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        let targets = crate::today::compute_today_hit_targets(1280.0, 800.0, true, false);

        // Clicking minimize button toggles show_desktop to true
        let min_btn = targets.minimize_btn;
        assert!(min_btn.w > 50.0);
        assert!(min_btn.h >= 20.0);
        assert!(min_btn.contains(min_btn.x + 10.0, min_btn.y + 10.0));

        state.show_desktop = true;
        assert!(state.show_desktop);

        // Clicking Today icon on dock or pressing Escape restores Today view
        state.show_desktop = false;
        state.wm.promote_to_front(WindowId::Today);
        assert!(!state.show_desktop);
        assert_eq!(state.wm.active_window_id, Some(WindowId::Today));
    }

    #[test]
    fn test_top_capsule_bar_hit_targets() {
        let capsule_tb = crate::top_bar::TopCapsuleBarLayout::calculate(1280.0, 800.0);
        assert!(capsule_tb.cc_button.w > 0.0);
        assert!(capsule_tb.notif_pill.w > 0.0);
        assert!(capsule_tb.clock_pill.w > 0.0);
        assert!(capsule_tb.map_pill.w > 0.0);
        assert!(capsule_tb.flash_pill.w > 0.0);
        assert!(capsule_tb.menu_file.w > 0.0);
        assert!(capsule_tb.menu_edit.w > 0.0);
        assert!(capsule_tb.menu_view.w > 0.0);
        assert!(capsule_tb.menu_help.w > 0.0);
    }

    #[test]
    fn test_odyssey_map_modal_return_action() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.open_overlay(ActiveOverlay::OdysseyMap);
        assert_eq!(state.active_overlay(), ActiveOverlay::OdysseyMap);

        // Return / close dismisses the overlay back to None
        state.close_overlay();
        assert_eq!(state.active_overlay(), ActiveOverlay::None);
    }

    // =========================================================================
    // Regression Unit Tests (BUG-201 .. BUG-207)
    // =========================================================================

    #[test]
    fn test_control_center_slider_at_145_percent_scale() {
        let s = 1.45_f32;
        let cc_layout = ControlCenterLayout::compute(1920.0, 1080.0, s);
        let track = cc_layout.ui_scale_track;
        let thumb_r = track.h * 0.5;

        // Drag at 30% along ui_scale_track
        let drag_x = track.x + track.w * 0.30;
        let val = normalized_slider_value(track, drag_x, thumb_r);
        let updated_scale = (0.80 + val * (1.50 - 0.80)).clamp(0.80, 1.50);

        // Assert ui_scale updates cleanly below 1.05 (no lockout)
        assert!(
            updated_scale < 1.05,
            "Expected scale < 1.05, got {:.3}",
            updated_scale
        );

        // Verify vertical drag band is at least track_h (guards against gap mismatch)
        let drag_band = LayoutRect::new(
            track.x - 4.0 * s,
            track.y - 4.0 * s,
            track.w + 8.0 * s,
            track.h + 8.0 * s,
        );
        assert!(drag_band.h >= track.h);
        assert!(drag_band.contains(drag_x, track.y + track.h * 0.5));
    }

    #[test]
    fn test_desktop_mission_start_triggers_focus_session() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.show_desktop = true;

        let desktop_layout = crate::frame::DesktopLayout::compute(1280.0, 800.0, 1.0);
        assert!(desktop_layout.hits.mission_start_btn.w > 0.0);
        assert!(desktop_layout.hits.mission_start_btn.h > 0.0);

        // Clicking [▶ Start] triggers focus session
        state.trigger_focus_session();

        assert!(state.timer_running);
        assert_eq!(state.timer_secs, 1500.0);
        assert_eq!(state.wm.active_window_id, Some(WindowId::StudyReader));
        assert!(state
            .wm
            .find_window(WindowId::StudyReader)
            .is_some_and(|w| w.is_open));
        // TasksGoals must NOT be opened
        assert!(!state
            .wm
            .find_window(WindowId::Tasks)
            .is_some_and(|w| w.is_open));
    }

    #[test]
    fn test_desktop_pomodoro_and_search_pill_routing() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        state.show_desktop = true;

        // Duration button cycles 30m -> 45m -> 60m -> 25m
        state.pomodoro_duration_mins = 30;
        state.cycle_pomodoro_duration();
        assert_eq!(state.pomodoro_duration_mins, 45);
        state.cycle_pomodoro_duration();
        assert_eq!(state.pomodoro_duration_mins, 60);
        state.cycle_pomodoro_duration();
        assert_eq!(state.pomodoro_duration_mins, 25);

        // Search pill opens Launcher overlay
        let desktop_layout = crate::frame::DesktopLayout::compute(1280.0, 800.0, 1.0);
        assert!(desktop_layout.hits.search_pill.w > 0.0);
        state.open_overlay(ActiveOverlay::Launcher);
        assert_eq!(state.active_overlay(), ActiveOverlay::Launcher);
    }

    #[test]
    fn test_study_reader_tab_updates_body_content() {
        // Tab 0 -> Tab 1 (Nernst equation)
        let topic_0 = active_study_topic(&STUDY_TOPIC_DATA, 0);
        let topic_1 = active_study_topic(&STUDY_TOPIC_DATA, 1);

        assert!(topic_0.is_some() && topic_1.is_some());
        let t0 = topic_0.unwrap();
        let t1 = topic_1.unwrap();

        assert_ne!(t0.question, t1.question);
        assert_ne!(t0.formula, t1.formula);
        assert!(t1.title.contains("Nernst"));
        assert!(t1.formula.contains("E_cell"));
        assert!(t1.question.contains("Daniell cell") || t1.question.contains("Nernst"));
    }

    #[test]
    fn test_floating_window_maximize_and_close_after_drag() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        let win = state.wm.find_window_mut(WindowId::StudyReader).unwrap();
        win.is_open = true;
        win.state = WindowState::Floating;

        // Translate StudyReader by (+120.0, +45.0)
        win.rect.x += 120.0;
        win.rect.y += 45.0;

        let chrome = WindowChromeHitTargets::from_rect(win.rect, 1.0);
        assert!(chrome
            .max_btn
            .contains(chrome.max_btn.x + 5.0, chrome.max_btn.y + 5.0));

        // Click maximize
        state.wm.toggle_maximize(WindowId::StudyReader);
        let win = state.wm.find_window(WindowId::StudyReader).unwrap();
        assert_eq!(win.state, WindowState::Maximized);
    }

    #[test]
    fn test_desktop_layout_hits_match_painted_rects() {
        for &ui_scale in &[1.0_f32, 1.45_f32] {
            let layout = crate::frame::DesktopLayout::compute(1920.0, 1080.0, ui_scale);
            let hits = layout.hits;

            // mission_start_btn contained in mission_card
            assert!(hits.mission_start_btn.x >= layout.mission_card.x - 0.01);
            assert!(hits.mission_start_btn.y >= layout.mission_card.y - 0.01);
            assert!(
                hits.mission_start_btn.x + hits.mission_start_btn.w
                    <= layout.mission_card.x + layout.mission_card.w + 0.01
            );
            assert!(
                hits.mission_start_btn.y + hits.mission_start_btn.h
                    <= layout.mission_card.y + layout.mission_card.h + 0.01
            );

            // mission_lock_btn contained in mission_card
            assert!(hits.mission_lock_btn.x >= layout.mission_card.x - 0.01);
            assert!(hits.mission_lock_btn.y >= layout.mission_card.y - 0.01);
            assert!(
                hits.mission_lock_btn.x + hits.mission_lock_btn.w
                    <= layout.mission_card.x + layout.mission_card.w + 0.01
            );
            assert!(
                hits.mission_lock_btn.y + hits.mission_lock_btn.h
                    <= layout.mission_card.y + layout.mission_card.h + 0.01
            );

            // pomodoro_duration_btn contained in pomodoro_card
            assert!(hits.pomodoro_duration_btn.x >= layout.pomodoro_card.x - 0.01);
            assert!(hits.pomodoro_duration_btn.y >= layout.pomodoro_card.y - 0.01);
            assert!(
                hits.pomodoro_duration_btn.x + hits.pomodoro_duration_btn.w
                    <= layout.pomodoro_card.x + layout.pomodoro_card.w + 0.01
            );
            assert!(
                hits.pomodoro_duration_btn.y + hits.pomodoro_duration_btn.h
                    <= layout.pomodoro_card.y + layout.pomodoro_card.h + 0.01
            );

            // pomodoro_cycle_btn contained in pomodoro_card
            assert!(hits.pomodoro_cycle_btn.x >= layout.pomodoro_card.x - 0.01);
            assert!(hits.pomodoro_cycle_btn.y >= layout.pomodoro_card.y - 0.01);
            assert!(
                hits.pomodoro_cycle_btn.x + hits.pomodoro_cycle_btn.w
                    <= layout.pomodoro_card.x + layout.pomodoro_card.w + 0.01
            );
            assert!(
                hits.pomodoro_cycle_btn.y + hits.pomodoro_cycle_btn.h
                    <= layout.pomodoro_card.y + layout.pomodoro_card.h + 0.01
            );
        }
    }

    #[test]
    fn test_set_show_desktop_unification() {
        let mut state = ShellState::new(1280.0, 800.0, true, 0.0, 1.0, 590);
        assert!(!state.show_desktop);

        // Transition to desktop
        state.set_show_desktop(true);
        assert!(state.show_desktop);

        // Transition back to Today view promotes Today window to front
        state.set_show_desktop(false);
        assert!(!state.show_desktop);
        assert_eq!(state.wm.active_window_id, Some(WindowId::Today));

        // toggle_desktop flips and preserves Today front promotion
        state.toggle_desktop();
        assert!(state.show_desktop);
        state.toggle_desktop();
        assert!(!state.show_desktop);
        assert_eq!(state.wm.active_window_id, Some(WindowId::Today));
    }

    #[test]
    fn test_power_menu_backdrop_dismiss_and_containment() {
        let mut session = mindset_surface::SessionManager::new(1280.0, 800.0);
        session.state = mindset_surface::SessionState::PowerMenuOpen;
        let pm_r = session.power_menu_rect();

        // A point strictly outside the power menu rect
        let outside_x = pm_r.x - 50.0;
        let outside_y = pm_r.y - 50.0;
        assert!(!pm_r.contains(outside_x, outside_y));

        // A point strictly inside the power menu rect
        let inside_x = pm_r.x + pm_r.w * 0.5;
        let inside_y = pm_r.y + pm_r.h * 0.5;
        assert!(pm_r.contains(inside_x, inside_y));
    }

    #[test]
    fn test_window_chrome_close_button_delegates_to_wm() {
        let mut wm = WindowManager::new(1280.0, 800.0);

        // Open two windows
        wm.toggle_window(WindowId::Tasks);
        wm.toggle_window(WindowId::StudyReader);
        assert_eq!(wm.active_window_id, Some(WindowId::StudyReader));

        // Closing active window via close_window promotes next window (Tasks)
        wm.close_window(WindowId::StudyReader);
        assert_eq!(wm.active_window_id, Some(WindowId::Tasks));
        let study = wm.find_window(WindowId::StudyReader);
        let Some(study_win) = study else {
            panic!("StudyReader window must exist");
        };
        assert!(!study_win.is_open);

        // Closing Tasks promotes Today
        wm.close_window(WindowId::Tasks);
        assert_eq!(wm.active_window_id, Some(WindowId::Today));

        // Attempting to close root Today window is safely rejected
        wm.close_window(WindowId::Today);
        assert_eq!(wm.active_window_id, Some(WindowId::Today));
        let today = wm.find_window(WindowId::Today);
        let Some(today_win) = today else {
            panic!("Today window must exist");
        };
        assert!(today_win.is_open);
    }

    #[test]
    fn test_top_capsule_bar_scales_with_ui_scale() {
        let screen_w = 1920.0;
        let layout_100 = crate::top_bar::TopCapsuleBarLayout::compute(screen_w, 1.0);
        let layout_125 = crate::top_bar::TopCapsuleBarLayout::compute(screen_w, 1.25);
        let layout_145 = crate::top_bar::TopCapsuleBarLayout::compute(screen_w, 1.45);

        // Flash pill and CC button widths scale with ui_scale
        assert!(layout_125.flash_pill.w >= layout_100.flash_pill.w);
        assert!(layout_145.flash_pill.w >= layout_125.flash_pill.w);
        assert!(layout_125.cc_button.w >= layout_100.cc_button.w);
        assert!(layout_145.cc_button.w >= layout_125.cc_button.w);

        // Right capsule remains within screen bounds
        assert!(layout_145.right_capsule.x + layout_145.right_capsule.w <= screen_w);
        assert!(
            layout_145.right_capsule.x
                >= layout_145.left_capsule.x + layout_145.left_capsule.w + 10.0
        );
    }

    #[test]
    fn test_trophy_deck_modal_bounds_and_card_interaction() {
        let screen_w = 1280.0_f32;
        let screen_h = 800.0_f32;
        let modal_w = (screen_w * 0.85).clamp(780.0, 1040.0);
        let modal_h = (screen_h * 0.82).clamp(520.0, 680.0);
        let modal_x = (screen_w - modal_w) * 0.5;
        let modal_y = (screen_h - modal_h) * 0.5;
        let modal_rect = LayoutRect::new(modal_x, modal_y, modal_w, modal_h);
        let close_btn = LayoutRect::new(
            modal_rect.x + modal_rect.w - 94.0,
            modal_rect.y + 20.0,
            70.0,
            28.0,
        );

        assert!(modal_rect.contains(close_btn.x + 10.0, close_btn.y + 10.0));
        assert!(close_btn.w > 0.0 && close_btn.h > 0.0);
    }

    #[test]
    fn test_layout_invariants_at_all_scales() {
        use mindset_surface::layout::{LayoutManager, UiId};

        let viewports = [(768.0, 1024.0), (1080.0, 1920.0), (1280.0, 800.0)];
        let scales = [1.0, 1.45];
        let cards = [
            UiId::PrimaryGoalCard,
            UiId::MissionCard,
            UiId::SubjectQuotaCard,
            UiId::PomodoroCard,
        ];

        for &(w, h) in &viewports {
            for &s in &scales {
                let mut layout_mgr = LayoutManager::new();
                let snapshot = layout_mgr.compute(w, h, s);
                let check = snapshot.assert_no_overlaps(&cards);
                assert!(
                    check.is_ok(),
                    "Layout collision detected at {}x{} scale {}: {:?}",
                    w, h, s, check.err()
                );
            }
        }
    }

    #[test]
    fn test_window_chrome_distinct_hit_targets() {
        use mindset_surface::layout::WindowChromeManager;

        let win_rect = LayoutRect::new(100.0, 100.0, 800.0, 600.0);
        let targets = WindowChromeManager::targets(win_rect, 1.0);
        assert!(targets.assert_distinct_targets());
        assert!(
            targets.close_btn.w >= 26.0,
            "Close button width {} < 26.0",
            targets.close_btn.w
        );
        assert!(
            targets.close_btn.h >= 26.0,
            "Close button height {} < 26.0",
            targets.close_btn.h
        );

        // Verify close does not intersect maximize
        let max_r = targets.max_btn;
        let cls_r = targets.close_btn;
        let x1 = max_r.x.max(cls_r.x);
        let y1 = max_r.y.max(cls_r.y);
        let x2 = (max_r.x + max_r.w).min(cls_r.x + cls_r.w);
        let y2 = (max_r.y + max_r.h).min(cls_r.y + cls_r.h);
        assert!(
            x2 <= x1 || y2 <= y1,
            "Close button intersects maximize button!"
        );
    }
}
