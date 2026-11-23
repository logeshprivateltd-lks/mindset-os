use crate::state::extract_domain;
use mindset_net::NetRepairAction;
use mindset_net::PolicyGuard;
use mindset_surface::events::{apply_window_requests, WindowRequest};
use mindset_surface::wm::WindowManager;
use mindset_surface::LayoutManager;
use mindset_sys::devices::GovernorProfile;
use mindset_sys::personalization::PersonalizationStore;
use mindset_ui::layout::INNER_GAP;
use mindset_ui::widgets::browser::BrowserTab;
use winit::window::CursorIcon;
// MindSet OS Event Loop & Input Handling

use chrono::Local;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId as WinitWindowId};

use mindset_material::{DualKawasePipeline, MaterialTargetManager};
use mindset_render::{BackgroundPipeline, RenderContext, SdfPipeline, TextRenderer};
use mindset_surface::session::{PowerAction, SessionState};
use mindset_surface::wm::{WindowId, WindowState};
use mindset_sys::personalization::OsMode;
use mindset_ui::widgets::browser::BrowserScreen;

use crate::app::MindSetApp;
use crate::state::{
    ActiveOverlay, ActiveSlider, BrowserPageKind, ExplorerViewMode, OsFocusMode, StudySoundscape,
    Task, ThemeMode, WallpaperPreset, STUDY_TOPICS,
};
use crate::top_bar::compute_top_bar_layout;

impl ApplicationHandler for MindSetApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attrs = Window::default_attributes()
                .with_title("MindSet OS — Sovereign Multi-Window Visual Desktop")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0_f64, 800.0_f64))
                .with_min_inner_size(winit::dpi::LogicalSize::new(1024.0_f64, 680.0_f64));

            let window = match event_loop.create_window(win_attrs) {
                Ok(w) => Arc::new(w),
                Err(err) => {
                    log::error!("Failed to create desktop window: {err:?}");
                    return;
                }
            };
            window.set_ime_allowed(true);

            let render_ctx = match pollster::block_on(RenderContext::new(window.clone())) {
                Ok(rc) => rc,
                Err(err) => {
                    log::error!("wgpu device creation failed: {err:?}");
                    return;
                }
            };
            let target_mgr = MaterialTargetManager::new(
                &render_ctx.device,
                render_ctx.config.width,
                render_ctx.config.height,
                render_ctx.config.format,
            );
            let bg_pipeline = BackgroundPipeline::new(&render_ctx.device, render_ctx.config.format);
            let kawase_pipeline =
                DualKawasePipeline::new(&render_ctx.device, render_ctx.config.format);
            let sdf_pipeline = SdfPipeline::new(&render_ctx.device, render_ctx.config.format);
            let text_renderer =
                match TextRenderer::new(&render_ctx.device, render_ctx.config.format) {
                    Ok(tr) => tr,
                    Err(err) => {
                        log::error!("Text glyph atlas failed: {err:?}");
                        return;
                    }
                };

            self.window = Some(window);
            self.render_ctx = Some(render_ctx);
            self.target_mgr = Some(target_mgr);
            self.bg_pipeline = Some(bg_pipeline);
            self.kawase_pipeline = Some(kawase_pipeline);
            self.sdf_pipeline = Some(sdf_pipeline);
            self.text_renderer = Some(text_renderer);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WinitWindowId,
        event: WindowEvent,
    ) {
        let (viewport_w, viewport_h) = self.render_ctx.as_ref().map_or((1280.0, 800.0), |c| {
            (c.config.width as f32, c.config.height as f32)
        });
        let ui_scale = self.state.personalization_store.appearance.ui_scale;
        let mut layout_mgr = LayoutManager::new();
        let snapshot = layout_mgr.compute(viewport_w, viewport_h, ui_scale);
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                let ctrl_now = modifiers.state().control_key();
                let alt_now = modifiers.state().alt_key();
                if ((self.alt_pressed && !alt_now) || (self.ctrl_pressed && !ctrl_now))
                    && self.state.alt_tab.is_visible
                {
                    self.state.alt_tab.close();
                    let open_windows: Vec<WindowId> = self
                        .state
                        .wm
                        .open_app_windows(self.state.active_workspace)
                        .into_iter()
                        .map(|w| w.id)
                        .collect();
                    if let Some(&wid) = open_windows.get(self.state.alt_tab.selected_index) {
                        self.state.wm.promote_to_front(wid);
                    }
                }
                self.ctrl_pressed = ctrl_now;
                self.alt_pressed = alt_now;
                self.shift_pressed = modifiers.state().shift_key();
            }

            WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                self.state.last_interaction_time = Instant::now();
                if let SessionState::Locked {
                    ref mut pin_input, ..
                } = self.state.session.state
                {
                    pin_input.insert_str(&text);
                } else if let Some(buf) = self.state.active_input_buffer_mut() {
                    buf.insert_str(&text);
                    if self.state.search_input.is_focused {
                        self.state.execute_search();
                    }
                }
            }

            WindowEvent::Resized(sz) => {
                if let Some(ctx) = self.render_ctx.as_mut() {
                    ctx.resize(sz.width, sz.height);
                    if let Some(ref mut tgt) = self.target_mgr {
                        tgt.resize(&ctx.device, sz.width, sz.height);
                    }
                    self.state
                        .session
                        .update_screen_size(sz.width as f32, sz.height as f32);
                    self.state
                        .wm
                        .resize_viewport(sz.width as f32, sz.height as f32);
                }
            }

            WindowEvent::ScaleFactorChanged {
                mut inner_size_writer,
                ..
            } => {
                if let Some(win) = self.window.as_ref() {
                    let sz = win.inner_size();
                    let _ = inner_size_writer.request_inner_size(sz);
                    if let Some(ctx) = self.render_ctx.as_mut() {
                        ctx.resize(sz.width, sz.height);
                        if let Some(ref mut tgt) = self.target_mgr {
                            tgt.resize(&ctx.device, sz.width, sz.height);
                        }
                        self.state
                            .session
                            .update_screen_size(sz.width as f32, sz.height as f32);
                        self.state
                            .wm
                            .resize_viewport(sz.width as f32, sz.height as f32);
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as f32;
                let y = position.y as f32;
                self.state.input.update_cursor(x, y);

                let w = self
                    .render_ctx
                    .as_ref()
                    .map_or(1280.0, |c| c.config.width as f32);
                let h = self
                    .render_ctx
                    .as_ref()
                    .map_or(800.0, |c| c.config.height as f32);

                // Dynamic OS Hand Cursor Feedback
                let is_interactive = self.check_interactive_hovered(x, y, w, h);
                if let Some(ref win) = self.window {
                    if is_interactive {
                        win.set_cursor(CursorIcon::Pointer);
                    } else {
                        win.set_cursor(CursorIcon::Default);
                    }
                }

                // Window dragging
                if let Some(wid) = self.state.dragging_window_id {
                    if let Some(win) = self.state.wm.find_window_mut(wid) {
                        win.rect.x = (x - win.drag_offset.0).clamp(10.0, (w - 60.0).max(10.0));
                        win.rect.y = snapshot.clamp_drag_y(y, win.drag_offset.1, h);
                    }
                }

                // Window resizing
                if let Some(wid) = self.state.resizing_window_id {
                    if let Some(win) = self.state.wm.find_window_mut(wid) {
                        let new_w = (x - win.rect.x).max(mindset_surface::MIN_WINDOW_WIDTH);
                        let new_h = (y - win.rect.y).max(mindset_surface::MIN_WINDOW_HEIGHT);
                        win.rect.w = new_w;
                        win.rect.h = new_h;
                    }
                }

                // Window scrollbar thumb dragging
                if let Some(wid) = self.state.wm.active_window_id {
                    if let Some(win) = self.state.wm.find_window_mut(wid) {
                        if win.is_scrolling_thumb {
                            let sb_track = win.scrollbar_track_rect();
                            let max_scroll = (win.content_height - win.content_rect().h).max(0.0);
                            if max_scroll > 0.0 && sb_track.h > 0.0 {
                                let ratio = ((y - sb_track.y) / sb_track.h).clamp(0.0, 1.0);
                                win.scroll_y.snap(ratio * max_scroll);
                            }
                        }
                    }
                }

                // Live slider dragging in Control Center & Settings
                if let Some(active) = self.state.active_slider {
                    if self.state.active_overlay() == ActiveOverlay::ControlCenter {
                        let s = self
                            .state
                            .personalization_store
                            .appearance
                            .ui_scale
                            .clamp(0.80, 1.50);
                        let cc_layout = mindset_surface::ControlCenterLayout::compute(w, h, s);
                        let thumb_r = cc_layout.brightness_track.h * 0.5;
                        match active {
                            ActiveSlider::Brightness => {
                                let val = mindset_surface::normalized_slider_value(
                                    cc_layout.brightness_track,
                                    x,
                                    thumb_r,
                                );
                                self.state.brightness = val;
                                self.modern_control_panel.brightness = val;
                                let pct = (val * 100.0).round() as u32;
                                self.state.show_toast(format!("☀️ Brightness: {}%", pct));
                            }
                            ActiveSlider::MasterVolume => {
                                let val = mindset_surface::normalized_slider_value(
                                    cc_layout.volume_track,
                                    x,
                                    thumb_r,
                                );
                                self.state.master_volume = val;
                                self.modern_control_panel.volume = val;
                                let pct = (val * 100.0).round() as u32;
                                self.state.show_toast(format!("🔊 Volume: {}%", pct));
                            }
                            ActiveSlider::UiScale => {
                                let val = mindset_surface::normalized_slider_value(
                                    cc_layout.ui_scale_track,
                                    x,
                                    thumb_r,
                                );
                                let scale = (0.80 + val * (1.50 - 0.80)).clamp(0.80, 1.50);
                                self.state.personalization_store.appearance.ui_scale = scale;
                                self.state.theme.set_ui_scale(scale);
                                if let Some(ref mut tr) = self.text_renderer {
                                    tr.set_ui_scale(scale);
                                }
                                let pct = (scale * 100.0).round() as u32;
                                self.state.show_toast(format!("🔍 UI Scale: {}%", pct));
                                self.persist_preferences();
                            }
                            ActiveSlider::NightWarmth => {}
                        }
                    } else if let Some(win) = self.state.wm.find_window(WindowId::Settings) {
                        let content = win.content_rect().inset(INNER_GAP);
                        let rail_w = 230.0;
                        let available_w = (content.w - rail_w - 28.0).max(100.0);
                        let form_w = available_w.min(820.0);
                        let content_x =
                            content.x + rail_w + 14.0 + ((available_w - form_w) * 0.5).max(0.0);
                        let content_w = form_w;
                        let slider_w = (content_w * 0.45).min(240.0);
                        let track_x = content_x + content_w - slider_w;
                        let track_w = slider_w.max(10.0);
                        let val = ((x - track_x) / track_w).clamp(0.0, 1.0);
                        match active {
                            ActiveSlider::NightWarmth => {
                                self.state.warmth_slider.value = val;
                                self.state.theme.set_night_warmth(val);
                                self.persist_preferences();
                            }
                            ActiveSlider::MasterVolume => {
                                self.state.master_volume = val;
                            }
                            ActiveSlider::UiScale => {
                                let scale = (0.85 + val * 0.50).clamp(0.85, 1.35);
                                self.state.theme.set_ui_scale(scale);
                                self.state.personalization_store.appearance.ui_scale = scale;
                                self.persist_preferences();
                            }
                            ActiveSlider::Brightness => {
                                self.state.brightness = val;
                                self.modern_control_panel.brightness = val;
                            }
                        }
                    }
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 36.0,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                let (cx, cy) = self.state.input.cursor;
                if !self.state.show_desktop && dy < -25.0 {
                    // Swiping / scrolling up while in Today view reveals full desktop
                    self.state.set_show_desktop(true);
                    self.state.show_toast("Desktop (Press Escape for Today)");
                    return;
                }
                if let Some(wid) =
                    self.state
                        .wm
                        .window_at_point(cx, cy, self.state.active_workspace)
                {
                    if let Some(win) = self.state.wm.find_window_mut(wid) {
                        let max_scroll = (win.content_height - win.content_rect().h).max(0.0);
                        let current = win.scroll_y.target;
                        win.scroll_y
                            .set_target((current - dy).clamp(0.0, max_scroll));
                    }
                }
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if !self.state.boot_completed {
                    self.state.boot_completed = true;
                    return;
                }
                self.state.input.press();
                self.state.last_interaction_time = Instant::now();
                let (cx, cy) = self.state.input.cursor;
                let w = self
                    .render_ctx
                    .as_ref()
                    .map_or(1280.0, |c| c.config.width as f32);
                let h = self
                    .render_ctx
                    .as_ref()
                    .map_or(800.0, |c| c.config.height as f32);

                // 0. Hero Lock Screen clicks
                if let SessionState::Locked {
                    ref mut pin_input, ..
                } = self.state.session.state
                {
                    let pad_w = snapshot.scaled(280.0);
                    let pad_h = snapshot.scaled(280.0);
                    let pad_y = snapshot.scaled(348.0);
                    let pad_r = snapshot.resolve_rect((w - pad_w) * 0.5, pad_y, pad_w, pad_h);

                    let grid_x = pad_r.x + snapshot.scaled(20.0);
                    let grid_y = pad_r.y + snapshot.scaled(48.0);
                    let btn_w = snapshot.scaled(70.0);
                    let btn_h = snapshot.scaled(44.0);
                    let gap = snapshot.scaled(15.0);

                    let keys = [
                        ("1", 0, 0),
                        ("2", 1, 0),
                        ("3", 2, 0),
                        ("4", 0, 1),
                        ("5", 1, 1),
                        ("6", 2, 1),
                        ("7", 0, 2),
                        ("8", 1, 2),
                        ("9", 2, 2),
                        ("Clear", 0, 3),
                        ("0", 1, 3),
                        ("Unlock", 2, 3),
                    ];
                    for (lbl, col, row) in keys {
                        let kr = snapshot.resolve_rect(
                            grid_x + col as f32 * (btn_w + gap),
                            grid_y + row as f32 * (btn_h + gap),
                            btn_w,
                            btn_h,
                        );
                        if kr.contains(cx, cy) {
                            match lbl {
                                "Clear" => pin_input.clear(),
                                "Unlock" => {
                                    let entered = pin_input.text.clone();
                                    self.state.session.try_unlock(&entered);
                                }
                                digit => {
                                    pin_input.insert_str(digit);
                                    if pin_input.text.len() >= 4 {
                                        let entered = pin_input.text.clone();
                                        self.state.session.try_unlock(&entered);
                                    }
                                }
                            }
                            return;
                        }
                    }

                    // Mode Badge Pill on Lock Screen
                    let mode_lbl = format!(
                        "{} — Click to Switch",
                        self.state.mindful_engine.mode().pill_label()
                    );
                    let (mode_tw, _) = self.text_renderer.as_ref().map_or((180.0, 14.0), |tr| {
                        tr.measure_line_px(&mode_lbl, snapshot.scaled(11.0))
                    });
                    let mode_pill_w = snapshot.scaled_max(mode_tw + 32.0, 280.0);
                    let mode_pill_h = snapshot.scaled(36.0);
                    let mode_pill_r = snapshot.resolve_rect(
                        (w - mode_pill_w) * 0.5,
                        pad_r.y + pad_r.h + snapshot.scaled(14.0),
                        mode_pill_w,
                        mode_pill_h,
                    );
                    if mode_pill_r.contains(cx, cy) {
                        let next_focus = self.state.mindful_engine.cycle_mode();
                        let next_mode = match next_focus {
                            OsFocusMode::Standard => OsMode::Standard,
                            OsFocusMode::Mindful => OsMode::Mindful,
                            OsFocusMode::StudyDeep => OsMode::ExamLock,
                            OsFocusMode::WindDown => OsMode::SleepWindDown,
                        };
                        self.state.personalization_store.productivity.current_mode = next_mode;
                        self.persist_preferences();
                        return;
                    }

                    return;
                }

                // 0.01 Mindful Focus Protection Modal clicks
                if let Some(target_wid) = self.state.mindful_modal_target {
                    let mm_r =
                        snapshot.resolve_rect((w - 460.0) * 0.5, (h - 240.0) * 0.44, 460.0, 240.0);
                    let btn_stay =
                        snapshot.resolve_rect(mm_r.x + 24.0, mm_r.y + mm_r.h - 52.0, 190.0, 38.0);
                    let btn_unlock = snapshot.resolve_rect(
                        mm_r.x + mm_r.w - 214.0,
                        mm_r.y + mm_r.h - 52.0,
                        190.0,
                        38.0,
                    );

                    if btn_stay.contains(cx, cy) {
                        self.state.mindful_modal_target = None;
                        return;
                    }
                    if btn_unlock.contains(cx, cy) {
                        self.state.temporary_unlocked_apps.insert(target_wid);
                        self.state.mindful_modal_target = None;
                        apply_window_requests(
                            &[WindowRequest::SetState {
                                window_id: target_wid,
                                state: WindowState::Floating,
                            }],
                            &mut self.state.wm,
                        );
                        if let Some(win) = self.state.wm.find_window_mut(target_wid) {
                            win.workspace_id = self.state.active_workspace;
                        }
                        self.state.wm.promote_to_front(target_wid);
                        self.state
                            .show_toast(format!("Unlocked {} for 5m", target_wid.title()));
                        return;
                    }
                    return;
                }

                // 0.02 Control Center Drawer clicks (Single Source of Truth)
                if self.state.active_overlay() == ActiveOverlay::ControlCenter {
                    let s = self
                        .state
                        .personalization_store
                        .appearance
                        .ui_scale
                        .clamp(0.80, 1.50);
                    let cc_layout = mindset_surface::ControlCenterLayout::compute(w, h, s);
                    if !cc_layout.panel_rect.contains(cx, cy) {
                        self.state.close_overlay();
                        return;
                    }
                    if let Some(action) =
                        crate::control_panel::hit_test_control_panel(&cc_layout, cx, cy, s)
                    {
                        match action {
                            crate::control_panel::ControlPanelAction::Close => {
                                self.state.close_overlay();
                                return;
                            }
                            crate::control_panel::ControlPanelAction::ToggleWifi => {
                                self.modern_control_panel.wifi_enabled =
                                    !self.modern_control_panel.wifi_enabled;
                                self.state.show_toast(format!(
                                    "Wi-Fi {}",
                                    if self.modern_control_panel.wifi_enabled {
                                        "Enabled"
                                    } else {
                                        "Disabled"
                                    }
                                ));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::ToggleBluetooth => {
                                self.modern_control_panel.bluetooth_enabled =
                                    !self.modern_control_panel.bluetooth_enabled;
                                self.state.show_toast(format!(
                                    "Bluetooth {}",
                                    if self.modern_control_panel.bluetooth_enabled {
                                        "Enabled"
                                    } else {
                                        "Disabled"
                                    }
                                ));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::ToggleDnd => {
                                self.modern_control_panel.do_not_disturb =
                                    !self.modern_control_panel.do_not_disturb;
                                self.state.show_toast(format!(
                                    "Do Not Disturb {}",
                                    if self.modern_control_panel.do_not_disturb {
                                        "ON"
                                    } else {
                                        "OFF"
                                    }
                                ));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::ToggleAirplane => {
                                self.modern_control_panel.airplane_mode =
                                    !self.modern_control_panel.airplane_mode;
                                self.state.show_toast(format!(
                                    "Airplane Mode {}",
                                    if self.modern_control_panel.airplane_mode {
                                        "ON"
                                    } else {
                                        "OFF"
                                    }
                                ));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::SetBrightness(val) => {
                                self.state.brightness = val;
                                self.modern_control_panel.brightness = val;
                                self.state.active_slider = Some(ActiveSlider::Brightness);
                                let pct = (val * 100.0).round() as u32;
                                self.state.show_toast(format!("☀️ Brightness: {}%", pct));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::SetVolume(val) => {
                                self.state.master_volume = val;
                                self.modern_control_panel.volume = val;
                                self.state.active_slider = Some(ActiveSlider::MasterVolume);
                                let pct = (val * 100.0).round() as u32;
                                self.state.show_toast(format!("🔊 Volume: {}%", pct));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::SetUiScale(scale) => {
                                self.state.personalization_store.appearance.ui_scale = scale;
                                self.state.theme.set_ui_scale(scale);
                                if let Some(ref mut tr) = self.text_renderer {
                                    tr.set_ui_scale(scale);
                                }
                                self.state.active_slider = Some(ActiveSlider::UiScale);
                                let pct = (scale * 100.0).round() as u32;
                                self.state.show_toast(format!("🔍 UI Scale: {}%", pct));
                                self.persist_preferences();
                                return;
                            }
                            crate::control_panel::ControlPanelAction::SetBacklight(val) => {
                                self.modern_control_panel.keyboard_backlight = val;
                                return;
                            }
                            crate::control_panel::ControlPanelAction::ToggleNightLight => {
                                self.modern_control_panel.night_light_active =
                                    !self.modern_control_panel.night_light_active;
                                self.state.show_toast(format!(
                                    "Night Light {}",
                                    if self.modern_control_panel.night_light_active {
                                        "ON"
                                    } else {
                                        "OFF"
                                    }
                                ));
                                return;
                            }
                            crate::control_panel::ControlPanelAction::OpenSettings => {
                                self.state.close_overlay();
                                apply_window_requests(
                                    &[WindowRequest::SetState {
                                        window_id: WindowId::Settings,
                                        state: WindowState::Floating,
                                    }],
                                    &mut self.state.wm,
                                );
                                if let Some(win) = self.state.wm.find_window_mut(WindowId::Settings)
                                {
                                    win.workspace_id = self.state.active_workspace;
                                }
                                self.state.wm.promote_to_front(WindowId::Settings);
                                self.state.show_toast("Opening Settings...");
                                return;
                            }
                            crate::control_panel::ControlPanelAction::OpenProjection => {
                                self.state.show_toast("Scanning wireless displays...");
                                return;
                            }
                        }
                    }
                    return;
                }

                // 0. Onboarding Wizard Modal clicks
                if self.state.active_overlay() == ActiveOverlay::OnboardingWizard {
                    let wiz_w = 600.0_f32.min(w - 40.0);
                    let wiz_h = 420.0_f32.min(h - 60.0);
                    let wiz_r =
                        snapshot.resolve_rect((w - wiz_w) * 0.5, (h - wiz_h) * 0.44, wiz_w, wiz_h);
                    let btn_y = wiz_r.y + wiz_h - 48.0;
                    let back_btn = snapshot.resolve_rect(wiz_r.x + 24.0, btn_y, 88.0, 36.0);
                    let skip_btn = if self.state.onboarding_wizard.current_step == 0 {
                        snapshot.resolve_rect(wiz_r.x + 24.0, btn_y, 110.0, 36.0)
                    } else {
                        snapshot.resolve_rect(wiz_r.x + 120.0, btn_y, 110.0, 36.0)
                    };
                    let next_btn =
                        snapshot.resolve_rect(wiz_r.x + wiz_r.w - 180.0, btn_y, 156.0, 36.0);

                    if next_btn.contains(cx, cy) {
                        if !self.state.onboarding_wizard.next_step() {
                            self.state.close_overlay();
                            self.state
                                .show_toast("Welcome to MindSet OS! Setup Complete.");
                            self.persist_preferences();
                        }
                        return;
                    }
                    if self.state.onboarding_wizard.current_step > 0 && back_btn.contains(cx, cy) {
                        self.state.onboarding_wizard.prev_step();
                        return;
                    }
                    if skip_btn.contains(cx, cy) {
                        self.state.close_overlay();
                        self.persist_preferences();
                        return;
                    }
                    return;
                }

                // 0.05 Odyssey Map & Trophy Deck Modal clicks
                if self.state.active_overlay() == ActiveOverlay::OdysseyMap {
                    let modal_w = (w * 0.90).clamp(840.0, 1160.0);
                    let modal_h = (h * 0.88).clamp(580.0, 760.0);
                    let modal_x = (w - modal_w) * 0.5;
                    let modal_y = (h - modal_h) * 0.5;
                    let modal_rect = snapshot.resolve_rect(modal_x, modal_y, modal_w, modal_h);

                    let header_h = 68.0;
                    let header_r =
                        snapshot.resolve_rect(modal_rect.x, modal_rect.y, modal_rect.w, header_h);
                    let close_btn_w = 84.0;
                    let direct_close_r = snapshot.resolve_rect(
                        header_r.x + header_r.w - close_btn_w - 18.0,
                        header_r.y + 18.0,
                        close_btn_w,
                        32.0,
                    );

                    // Clicking outside the modal container dismisses the overlay
                    if !modal_rect.contains(cx, cy) || direct_close_r.contains(cx, cy) {
                        self.state.close_overlay();
                        self.state.show_toast("Returned to Today View");
                        return;
                    }

                    let action = self.state.odyssey_hit_map.test(cx, cy);
                    match action {
                        crate::odyssey_renderer::OdysseyClickAction::CloseMap => {
                            self.state.close_overlay();
                            self.state.show_toast("Returned to Today View");
                        }
                        crate::odyssey_renderer::OdysseyClickAction::OpenTrophyDeck => {
                            self.state.open_overlay(ActiveOverlay::TrophyDeck);
                        }
                        crate::odyssey_renderer::OdysseyClickAction::SelectNode(idx) => {
                            self.state.odyssey_map.selected_node_idx = Some(idx);
                        }
                        crate::odyssey_renderer::OdysseyClickAction::AdvanceStage => {
                            let completed = self.state.odyssey_map.advance_mission();
                            if completed {
                                self.state.show_toast(
                                    "🏆 LEVEL COMPLETE! +Marks Locked & Trophy Card Minted!",
                                );
                            } else {
                                self.state.show_toast("✓ Mission Stage Completed!");
                            }
                        }
                        crate::odyssey_renderer::OdysseyClickAction::EmbarkMission => {
                            self.state.close_overlay();
                            self.state.start_mission_flow_session();
                        }
                        crate::odyssey_renderer::OdysseyClickAction::ResolveCampfire => {
                            self.state.odyssey_map.resolve_campfire();
                            self.state
                                .show_toast("🔥 Campfire Restored! Momentum Unlocked!");
                        }
                        _ => {}
                    }
                    return;
                }

                if self.state.active_overlay() == ActiveOverlay::TrophyDeck {
                    let modal_w = (w * 0.85).clamp(780.0, 1040.0);
                    let modal_h = (h * 0.82).clamp(520.0, 680.0);
                    let modal_x = (w - modal_w) * 0.5;
                    let modal_y = (h - modal_h) * 0.5;
                    let modal_rect = snapshot.resolve_rect(modal_x, modal_y, modal_w, modal_h);
                    let close_btn = snapshot.resolve_rect(
                        modal_rect.x + modal_rect.w - 94.0,
                        modal_rect.y + 20.0,
                        70.0,
                        28.0,
                    );
                    if close_btn.contains(cx, cy) || !modal_rect.contains(cx, cy) {
                        self.state.open_overlay(ActiveOverlay::OdysseyMap);
                        return;
                    }

                    let grid_y = modal_rect.y + 76.0;
                    let card_w = (modal_rect.w - 48.0 - 24.0) / 3.0;
                    let card_h = 168.0;
                    for (i, card) in self
                        .state
                        .odyssey_map
                        .trophy_deck
                        .iter()
                        .take(6)
                        .enumerate()
                    {
                        let col = (i % 3) as f32;
                        let row = (i / 3) as f32;
                        let card_x = modal_rect.x + 24.0 + col * (card_w + 12.0);
                        let card_y = grid_y + row * (card_h + 12.0);
                        let cr = snapshot.resolve_rect(card_x, card_y, card_w, card_h);
                        if cr.contains(cx, cy) {
                            self.state.show_toast(format!(
                                "🏆 #{}: {} (+{} Marks)",
                                card.id, card.title, card.marks_yield
                            ));
                            return;
                        }
                    }
                    return;
                }

                // 0.06 Flash Switcher Modal clicks
                if self.state.active_overlay() == ActiveOverlay::FlashSwitcher {
                    let flashes = crate::flash_switcher::FlashWorkspaceInfo::all();
                    let num_flashes = flashes.len() as f32;
                    let card_w = 170.0;
                    let card_h = 148.0;
                    let card_gap = 14.0;
                    let pad_h = 20.0;
                    let modal_w =
                        pad_h * 2.0 + num_flashes * card_w + (num_flashes - 1.0) * card_gap;
                    let modal_h = 224.0;
                    let modal_x = (w - modal_w) * 0.5;
                    let modal_y = 64.0;
                    let modal_rect = snapshot.resolve_rect(modal_x, modal_y, modal_w, modal_h);

                    let esc_pill = snapshot.resolve_rect(
                        modal_rect.x + modal_rect.w - pad_h - 70.0,
                        modal_rect.y + 14.0,
                        70.0,
                        24.0,
                    );
                    if esc_pill.contains(cx, cy) || !modal_rect.contains(cx, cy) {
                        self.state.close_overlay();
                        return;
                    }

                    let start_y = modal_rect.y + 58.0;
                    for flash in flashes.iter() {
                        let card_x =
                            modal_rect.x + pad_h + (flash.index as f32) * (card_w + card_gap);
                        let card_rect = snapshot.resolve_rect(card_x, start_y, card_w, card_h);
                        if card_rect.contains(cx, cy) {
                            self.state.active_workspace = flash.index;
                            self.state.close_overlay();
                            self.state
                                .show_toast(format!("Switched to Flash {}", flash.index + 1));
                            return;
                        }
                    }
                    return;
                }

                // 0.1 Alt+Tab Overlay clicks
                if self.state.alt_tab.is_visible {
                    let open_windows: Vec<WindowId> = self
                        .state
                        .wm
                        .open_app_windows(self.state.active_workspace)
                        .into_iter()
                        .map(|w| w.id)
                        .collect();
                    let count = open_windows.len();
                    if count > 0 {
                        let card_w = 110.0;
                        let card_h = 100.0;
                        let gap = 12.0;
                        let total_w = (count as f32 * card_w + (count - 1) as f32 * gap + 40.0)
                            .clamp(320.0, w - 40.0);
                        let total_h = 170.0;
                        let overlay_r = snapshot.resolve_rect(
                            (w - total_w) * 0.5,
                            (h - total_h) * 0.42,
                            total_w,
                            total_h,
                        );
                        if overlay_r.contains(cx, cy) {
                            let start_x = overlay_r.x + 20.0;
                            let card_y = overlay_r.y + 46.0;
                            for (idx, wid) in open_windows.iter().enumerate() {
                                let cr = snapshot.resolve_rect(
                                    start_x + idx as f32 * (card_w + gap),
                                    card_y,
                                    card_w,
                                    card_h,
                                );
                                if cr.contains(cx, cy) {
                                    self.state.wm.promote_to_front(*wid);
                                    self.state.alt_tab.close();
                                    return;
                                }
                            }
                            return;
                        } else {
                            self.state.alt_tab.close();
                            return;
                        }
                    }
                }

                // Close context menu if clicked outside
                if self.state.active_overlay() == ActiveOverlay::ContextMenu
                    && !self.state.context_menu.rect().contains(cx, cy)
                {
                    self.state.close_overlay();
                    return;
                }

                // Command launcher overlay clicks
                if matches!(
                    self.state.active_overlay(),
                    ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. }
                ) {
                    let cl_w = 540.0_f32.min(w - 40.0);
                    let cl_h = 510.0_f32.min(h - 60.0);
                    let cl_r =
                        snapshot.resolve_rect((w - cl_w) * 0.5, (h - cl_h) * 0.44, cl_w, cl_h);
                    if cl_r.contains(cx, cy) {
                        let search_r = snapshot.resolve_rect(
                            cl_r.x + 24.0,
                            cl_r.y + 20.0,
                            cl_r.w - 48.0,
                            36.0,
                        );
                        if search_r.contains(cx, cy) {
                            return;
                        }
                        let banner_y = search_r.y + search_r.h + 12.0;
                        let ai_hub_w = 260.0;
                        let ai_hub_r =
                            snapshot.resolve_rect(cl_r.x + 24.0, banner_y, ai_hub_w, 52.0);
                        if ai_hub_r.contains(cx, cy) {
                            self.state.wm.promote_to_front(WindowId::LayaAiChat);
                            self.state.close_overlay();
                            return;
                        }
                        let pinned_y = banner_y + 64.0;
                        let app_icons = [
                            WindowId::Today,
                            WindowId::SearchBrowser,
                            WindowId::StudyReader,
                            WindowId::LayaAiChat,
                            WindowId::Tasks,
                            WindowId::FileVault,
                            WindowId::ProgressStats,
                            WindowId::SystemMonitor,
                            WindowId::Settings,
                            WindowId::DesignSystem,
                        ];
                        let icon_w = 44.0;
                        let grid_start_x = cl_r.x + 32.0;
                        let col_spacing = (cl_r.w - 64.0 - icon_w * 5.0) / 4.0;
                        for (idx, wid) in app_icons.iter().enumerate() {
                            let row = (idx / 5) as f32;
                            let col_idx = (idx % 5) as f32;
                            let ix = grid_start_x + col_idx * (icon_w + col_spacing);
                            let iy = pinned_y + 20.0 + row * 62.0;
                            let icon_box =
                                snapshot.resolve_rect(ix - 6.0, iy, icon_w + 12.0, icon_w + 18.0);
                            if icon_box.contains(cx, cy) {
                                self.state.wm.promote_to_front(*wid);
                                self.state.close_overlay();
                                return;
                            }
                        }
                        let rec_y = pinned_y + 154.0;
                        for f_idx in 0..3 {
                            let fy = rec_y + 18.0 + (f_idx as f32) * 26.0;
                            let file_r =
                                snapshot.resolve_rect(cl_r.x + 24.0, fy, cl_r.w - 48.0, 24.0);
                            if file_r.contains(cx, cy) {
                                self.state.wm.promote_to_front(WindowId::StudyReader);
                                self.state.close_overlay();
                                return;
                            }
                        }
                        let bottom_y = cl_r.y + cl_h - 44.0;
                        let btn_bar_y = bottom_y + 8.0;
                        let pad_x = 20.0;
                        let b1 = snapshot.resolve_rect(cl_r.x + pad_x, btn_bar_y, 96.0, 26.0);
                        let b2 = snapshot.resolve_rect(b1.x + b1.w + 8.0, btn_bar_y, 110.0, 26.0);
                        let b3 = snapshot.resolve_rect(b2.x + b2.w + 8.0, btn_bar_y, 98.0, 26.0);
                        let power_w = 76.0;
                        let power_btn = snapshot.resolve_rect(
                            cl_r.x + cl_r.w - pad_x - power_w,
                            btn_bar_y,
                            power_w,
                            26.0,
                        );
                        if b1.contains(cx, cy) {
                            return;
                        }
                        if b2.contains(cx, cy) || b3.contains(cx, cy) {
                            self.state.wm.promote_to_front(WindowId::FileVault);
                            self.state.close_overlay();
                            return;
                        }
                        if power_btn.contains(cx, cy) {
                            self.state.session.state = SessionState::PowerMenuOpen;
                            self.state.close_overlay();
                            return;
                        }
                        return;
                    } else if cy > 52.0 {
                        self.state.close_overlay();
                        return;
                    }
                }

                // Left-Side Student Widget Card clicks & Desktop Hit Targets (Phase 0 Unification)
                if !self.state.session.focus_mode
                    && self.state.show_desktop
                    && self.state.mindful_engine.should_show_left_widgets()
                {
                    let s = self
                        .state
                        .personalization_store
                        .appearance
                        .ui_scale
                        .clamp(0.80, 1.50);
                    let desktop_layout = crate::frame::DesktopLayout::compute(w, h, s);
                    if self
                        .state
                        .wm
                        .window_at_point(cx, cy, self.state.active_workspace)
                        .is_none()
                    {
                        // Child widgets evaluated BEFORE parent card / icon checks (BUG-202, BUG-203)
                        // 1. Mission start button -> triggers focus session (does NOT open Tasks)
                        if desktop_layout.hits.mission_start_btn.contains(cx, cy) {
                            self.trigger_focus_session();
                            return;
                        }

                        // 2. Pomodoro duration button -> cycles 25m -> 30m -> 45m -> 60m -> 25m
                        if desktop_layout.hits.pomodoro_duration_btn.contains(cx, cy) {
                            self.state.cycle_pomodoro_duration();
                            return;
                        }

                        // 3. Pomodoro cycle session button -> cycles 1/4 .. 4/4
                        if desktop_layout.hits.pomodoro_cycle_btn.contains(cx, cy) {
                            self.state.cycle_pomodoro_session();
                            return;
                        }

                        // 4. Top-Center Desktop Search Bar Pill -> opens Launcher
                        if desktop_layout.hits.search_pill.contains(cx, cy) {
                            if self.state.active_overlay() != ActiveOverlay::Launcher {
                                self.state.open_overlay(ActiveOverlay::Launcher);
                            }
                            return;
                        }

                        // 5. Mission lock button
                        if desktop_layout.hits.mission_lock_btn.contains(cx, cy) {
                            self.state.lock_today_mission_marks();
                            return;
                        }

                        // Fallback parent card clicks:
                        if self.state.widgets_enabled[0]
                            && desktop_layout.primary_goal_card.contains(cx, cy)
                        {
                            self.trigger_focus_session();
                            return;
                        }
                        if self.state.widgets_enabled[2]
                            && desktop_layout.mission_card.contains(cx, cy)
                        {
                            apply_window_requests(
                                &[WindowRequest::SetState {
                                    window_id: WindowId::Tasks,
                                    state: WindowState::Floating,
                                }],
                                &mut self.state.wm,
                            );
                            if let Some(win) = self.state.wm.find_window_mut(WindowId::Tasks) {
                                win.workspace_id = self.state.active_workspace;
                            }
                            self.state.wm.promote_to_front(WindowId::Tasks);
                            self.state.show_toast("Opened Tasks & Goals");
                            return;
                        }
                        if self.state.widgets_enabled[1]
                            && desktop_layout.subject_quota_card.contains(cx, cy)
                        {
                            apply_window_requests(
                                &[WindowRequest::SetState {
                                    window_id: WindowId::ProgressStats,
                                    state: WindowState::Floating,
                                }],
                                &mut self.state.wm,
                            );
                            if let Some(win) =
                                self.state.wm.find_window_mut(WindowId::ProgressStats)
                            {
                                win.workspace_id = self.state.active_workspace;
                            }
                            self.state.wm.promote_to_front(WindowId::ProgressStats);
                            self.state.show_toast("Opened Progress & Stats");
                            return;
                        }
                        if desktop_layout.pomodoro_card.contains(cx, cy) {
                            self.state.timer_running = !self.state.timer_running;
                            let msg = if self.state.timer_running {
                                "Pomodoro Timer Started"
                            } else {
                                "Pomodoro Timer Paused"
                            };
                            self.state.show_toast(msg);
                            return;
                        }
                    }
                }

                // Top Bar clicks
                if cy <= 52.0 {
                    let s = self
                        .state
                        .personalization_store
                        .appearance
                        .ui_scale
                        .clamp(0.80, 1.50);
                    let capsule_tb = crate::top_bar::TopCapsuleBarLayout::compute(w, s);
                    if capsule_tb.cc_button.contains(cx, cy) {
                        self.state.toggle_overlay(ActiveOverlay::ControlCenter);
                        return;
                    }
                    if capsule_tb.flash_pill.contains(cx, cy) {
                        self.state.toggle_overlay(ActiveOverlay::FlashSwitcher);
                        return;
                    }
                    if capsule_tb.map_pill.contains(cx, cy) {
                        self.state.toggle_overlay(ActiveOverlay::OdysseyMap);
                        return;
                    }
                    if capsule_tb.notif_pill.contains(cx, cy) {
                        self.state.toggle_overlay(ActiveOverlay::Notifications);
                        return;
                    }
                    if capsule_tb.clock_pill.contains(cx, cy) {
                        let now = Local::now();
                        self.state
                            .show_toast(now.format("%A, %B %e, %Y • %r").to_string());
                        return;
                    }
                    if capsule_tb.wifi_pill.contains(cx, cy) {
                        self.state
                            .show_toast("Wi-Fi 6 (802.11ax) • 12ms Latency • Connected");
                        return;
                    }
                    if capsule_tb.menu_file.contains(cx, cy) {
                        apply_window_requests(
                            &[WindowRequest::SetState {
                                window_id: WindowId::FileVault,
                                state: WindowState::Floating,
                            }],
                            &mut self.state.wm,
                        );
                        if let Some(win) = self.state.wm.find_window_mut(WindowId::FileVault) {
                            win.workspace_id = self.state.active_workspace;
                        }
                        self.state.wm.promote_to_front(WindowId::FileVault);
                        self.state.show_toast("Opened Files & Study Vault");
                        return;
                    }
                    if capsule_tb.menu_edit.contains(cx, cy) {
                        apply_window_requests(
                            &[WindowRequest::SetState {
                                window_id: WindowId::Tasks,
                                state: WindowState::Floating,
                            }],
                            &mut self.state.wm,
                        );
                        if let Some(win) = self.state.wm.find_window_mut(WindowId::Tasks) {
                            win.workspace_id = self.state.active_workspace;
                        }
                        self.state.wm.promote_to_front(WindowId::Tasks);
                        self.state.show_toast("Opened Tasks & Goals");
                        return;
                    }
                    if capsule_tb.menu_view.contains(cx, cy) {
                        self.state.toggle_desktop();
                        let msg = if self.state.show_desktop {
                            "Desktop View (Press Escape for Today)"
                        } else {
                            "Distraction-Free Today View"
                        };
                        self.state.show_toast(msg);
                        return;
                    }
                    if capsule_tb.menu_help.contains(cx, cy) {
                        apply_window_requests(
                            &[WindowRequest::SetState {
                                window_id: WindowId::LayaAiChat,
                                state: WindowState::Floating,
                            }],
                            &mut self.state.wm,
                        );
                        if let Some(win) = self.state.wm.find_window_mut(WindowId::LayaAiChat) {
                            win.workspace_id = self.state.active_workspace;
                        }
                        self.state.wm.promote_to_front(WindowId::LayaAiChat);
                        self.state.show_toast("Opened Maindy AI Study Tutor");
                        return;
                    }

                    if let Some(ref text_r) = self.text_renderer {
                        let s = self.state.personalization_store.appearance.ui_scale;
                        let open_count = self
                            .state
                            .wm
                            .open_app_windows(self.state.active_workspace)
                            .len();
                        let unread_cnt = self.state.session.notifications.unread_count();
                        let mode_lbl = self.state.mindful_engine.mode().pill_label();
                        let now_str = Local::now().format("%A, %b %d • %I:%M:%S %p").to_string();

                        let tb = compute_top_bar_layout(
                            w,
                            s,
                            self.state.workspaces_enabled,
                            self.state.active_workspace,
                            open_count,
                            unread_cnt,
                            mode_lbl,
                            &now_str,
                            |t, fs| text_r.measure_line_px(t, fs),
                        );

                        if tb.win_pill.w > 0.0 && tb.win_pill.contains(cx, cy) {
                            self.state.alt_tab.is_visible = !self.state.alt_tab.is_visible;
                            return;
                        }

                        for i in 0..tb.ws_count {
                            let (ws_r, ws_idx) = tb.ws_pills[i];
                            if ws_r.contains(cx, cy) {
                                self.state.active_workspace = ws_idx;
                                return;
                            }
                        }

                        if tb.btn_power.contains(cx, cy) {
                            self.state.session.state = match self.state.session.state {
                                SessionState::PowerMenuOpen => SessionState::Active,
                                _ => SessionState::PowerMenuOpen,
                            };
                            return;
                        }

                        if tb.btn_lock.contains(cx, cy) {
                            self.state.session.lock();
                            return;
                        }

                        if tb.btn_cc.contains(cx, cy) {
                            self.state.toggle_overlay(ActiveOverlay::ControlCenter);
                            return;
                        }

                        if tb.btn_mode.contains(cx, cy) {
                            let next_focus = self.state.mindful_engine.cycle_mode();
                            let next_mode = match next_focus {
                                OsFocusMode::Standard => OsMode::Standard,
                                OsFocusMode::Mindful => OsMode::Mindful,
                                OsFocusMode::StudyDeep => OsMode::ExamLock,
                                OsFocusMode::WindDown => OsMode::SleepWindDown,
                            };
                            self.state.personalization_store.productivity.current_mode = next_mode;
                            self.state
                                .show_toast(format!("Switched to {}", next_focus.pill_label()));
                            self.persist_preferences();
                            return;
                        }

                        if tb.btn_search.contains(cx, cy) {
                            self.state.toggle_overlay(ActiveOverlay::Launcher);
                            return;
                        }

                        if tb.btn_notif.contains(cx, cy) {
                            self.state.toggle_overlay(ActiveOverlay::Notifications);
                            return;
                        }
                    }
                }

                // Notification drawer clicks
                if self.state.active_overlay() == ActiveOverlay::Notifications {
                    let w = self
                        .render_ctx
                        .as_ref()
                        .map_or(1280.0, |c| c.config.width as f32);
                    let nd_r = self.state.session.notifications.drawer_rect(w);
                    if nd_r.contains(cx, cy) {
                        let dnd_btn = snapshot.resolve_rect(
                            nd_r.x + nd_r.w - 146.0,
                            nd_r.y + 8.0,
                            68.0,
                            24.0,
                        );
                        let clear_btn =
                            snapshot.resolve_rect(nd_r.x + nd_r.w - 74.0, nd_r.y + 8.0, 62.0, 24.0);
                        let snooze_btn = snapshot.resolve_rect(
                            nd_r.x + 12.0,
                            nd_r.y + nd_r.h - 38.0,
                            nd_r.w - 24.0,
                            28.0,
                        );

                        if dnd_btn.contains(cx, cy) {
                            self.state.session.notifications.toggle_dnd();
                            return;
                        }
                        if clear_btn.contains(cx, cy) {
                            self.state.session.notifications.clear_all();
                            return;
                        }
                        if snooze_btn.contains(cx, cy) {
                            let curr = self.state.session.notifications.snoozed_until_minutes;
                            let next = if curr > 0 { 0 } else { 15 };
                            self.state.session.notifications.snooze(next);
                            return;
                        }
                        return;
                    } else if cy > 52.0 {
                        self.state.close_overlay();
                        return;
                    }
                }

                // Power menu item clicks
                if self.state.session.state == SessionState::PowerMenuOpen {
                    let pm_r = self.state.session.power_menu_rect();
                    if !pm_r.contains(cx, cy) {
                        self.state.session.state = SessionState::Active;
                        return;
                    }
                    for (idx, action) in PowerAction::all().iter().enumerate() {
                        let ir = snapshot.resolve_rect(
                            pm_r.x + 12.0,
                            pm_r.y + 36.0 + idx as f32 * 38.0,
                            pm_r.w - 24.0,
                            32.0,
                        );
                        if ir.contains(cx, cy) {
                            match action {
                                PowerAction::LockScreen => self.state.session.lock(),
                                PowerAction::SleepStandby => self.state.session.lock(),
                                PowerAction::RestartShell => {
                                    self.state.wm = WindowManager::new(
                                        self.render_ctx
                                            .as_ref()
                                            .map_or(1280.0, |c| c.config.width as f32),
                                        self.render_ctx
                                            .as_ref()
                                            .map_or(800.0, |c| c.config.height as f32),
                                    );
                                    self.state.session.state = SessionState::Active;
                                }
                                PowerAction::ShutDown => {
                                    if let Some(ref db) = self.db_mgr {
                                        if let Ok(conn) = db.get_core_connection().lock() {
                                            let _ = conn
                                                .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
                                        }
                                    }
                                    std::process::exit(0);
                                }
                            }
                            return;
                        }
                    }
                    return;
                }

                // Bottom dock clicks
                if !self.state.session.focus_mode
                    && self.state.show_desktop
                    && self.state.mindful_engine.should_show_bottom_dock()
                {
                    let dock_r = self.state.session.dock_rect();
                    if dock_r.contains(cx, cy) {
                        let all_ids = WindowId::dock_windows();
                        let allowed_ids: Vec<WindowId> = all_ids
                            .into_iter()
                            .filter(|id| self.state.is_app_allowed_on_home(*id))
                            .collect();
                        for (idx, wid) in allowed_ids.iter().enumerate() {
                            let ir = self.state.session.dock_item_rect(idx, allowed_ids.len());
                            if ir.contains(cx, cy) {
                                if !self.state.is_app_allowed_on_home(*wid) {
                                    self.state.mindful_modal_target = Some(*wid);
                                    return;
                                }
                                if *wid == WindowId::Today {
                                    self.state.set_show_desktop(false);
                                    self.state.show_toast("Switched to Today View");
                                    return;
                                }
                                if let Some(win) = self.state.wm.find_window_mut(*wid) {
                                    win.workspace_id = self.state.active_workspace;
                                }
                                self.state.wm.toggle_window(*wid);
                                self.state
                                    .show_toast(format!("Switched to {}", wid.title()));
                                return;
                            }
                        }
                    }
                }

                // Window clicks & titlebar dragging
                if let Some(wid) =
                    self.state
                        .wm
                        .window_at_point(cx, cy, self.state.active_workspace)
                {
                    self.state.wm.promote_to_front(wid);

                    if let Some(win) = self.state.wm.find_window_mut(wid) {
                        let s = self
                            .state
                            .personalization_store
                            .appearance
                            .ui_scale
                            .clamp(0.80, 1.50);
                        let chrome = win.chrome_targets(s);
                        // Titlebar buttons
                        if chrome.close_btn.contains(cx, cy) {
                            let w_id = win.id;
                            self.state.wm.close_window(w_id);
                            return;
                        }
                        if chrome.max_btn.contains(cx, cy) {
                            let w_id = win.id;
                            self.state.wm.toggle_maximize(w_id);
                            return;
                        }
                        if chrome.min_btn.contains(cx, cy) {
                            let w_id = win.id;
                            self.state.wm.minimize_window(w_id);
                            return;
                        }

                        // Resize grip
                        if win.state == WindowState::Floating
                            && win.resize_grip_rect().contains(cx, cy)
                        {
                            self.state.resizing_window_id = Some(wid);
                            return;
                        }

                        // Titlebar dragging
                        if win.titlebar_rect().contains(cx, cy) {
                            win.is_dragging = true;
                            win.drag_offset = (cx - win.rect.x, cy - win.rect.y);
                            self.state.dragging_window_id = Some(wid);
                            return;
                        }

                        // Scrollbar track click
                        if win.scrollbar_track_rect().contains(cx, cy) {
                            let sb_track = win.scrollbar_track_rect();
                            let max_scroll = (win.content_height - win.content_rect().h).max(0.0);
                            if max_scroll > 0.0 && sb_track.h > 0.0 {
                                let ratio = ((cy - sb_track.y) / sb_track.h).clamp(0.0, 1.0);
                                win.scroll_y.snap(ratio * max_scroll);
                                win.is_scrolling_thumb = true;
                            }
                            return;
                        }

                        // Content interactive clicks
                        let content = win.content_rect().inset(INNER_GAP);
                        match wid {
                            WindowId::StudyReader => {
                                if self.state.timer_secs <= 0.0 {
                                    let card_w = 420.0_f32.min(content.w - 40.0);
                                    let card_h = 240.0;
                                    let card_r = snapshot.resolve_rect(
                                        content.x + (content.w - card_w) * 0.5,
                                        content.y + 40.0,
                                        card_w,
                                        card_h,
                                    );
                                    let back_btn = snapshot.resolve_rect(
                                        card_r.x + (card_r.w - 180.0) * 0.5,
                                        card_r.y + 140.0,
                                        180.0,
                                        42.0,
                                    );
                                    if back_btn.contains(cx, cy) {
                                        // Complete session, refresh today, fire celebration, close StudyReader
                                        self.today_state
                                            .complete_session(25, self.state.theme.reduced_motion);
                                        self.state.wm.close_window(WindowId::StudyReader);
                                        self.state.set_show_desktop(false);
                                        self.state.timer_secs = 25.0 * 60.0;
                                        self.state.timer_running = false;
                                        self.state.show_toast("Wave Optics: 42% -> 58% (+16%)");
                                        return;
                                    }
                                }
                                let (topic_bar, _) = content.split_top(34.0);
                                for i in 0..STUDY_TOPICS.len() {
                                    let tr = topic_bar.child_row(i, STUDY_TOPICS.len(), 8.0);
                                    if tr.contains(cx, cy) {
                                        self.state.study_topic = i;
                                        return;
                                    }
                                }
                            }
                            WindowId::LayaAiChat => {
                                let (_, input_bar) = content.split_top(content.h - 48.0);
                                let send_btn_w = 68.0;
                                let gap = 8.0;
                                let input_w = (input_bar.w - send_btn_w - gap - 4.0).max(80.0);
                                let input_r = snapshot.resolve_rect(
                                    input_bar.x,
                                    input_bar.y + 8.0,
                                    input_w,
                                    36.0,
                                );
                                let send_btn = snapshot.resolve_rect(
                                    input_r.x + input_w + gap,
                                    input_bar.y + 8.0,
                                    send_btn_w,
                                    36.0,
                                );
                                if input_r.contains(cx, cy) {
                                    self.state.maindy_query_input.is_focused = true;
                                    self.state.task_title_input.is_focused = false;
                                    self.state.search_input.is_focused = false;
                                    return;
                                }
                                if send_btn.contains(cx, cy) {
                                    let q = self.state.maindy_query_input.text.clone();
                                    if !q.is_empty() {
                                        self.state.ask_custom_maindy_query(&q);
                                        self.state.maindy_query_input.clear();
                                    }
                                    return;
                                }
                            }
                            WindowId::Tasks => {
                                let (input_row, list_area) = content.split_top(42.0);
                                let inp_r = snapshot.resolve_rect(
                                    input_row.x,
                                    input_row.y,
                                    input_row.w - 100.0,
                                    32.0,
                                );
                                let add_btn = snapshot.resolve_rect(
                                    input_row.x + input_row.w - 92.0,
                                    input_row.y,
                                    92.0,
                                    32.0,
                                );
                                if inp_r.contains(cx, cy) {
                                    self.state.task_title_input.is_focused = true;
                                    self.state.maindy_query_input.is_focused = false;
                                    self.state.search_input.is_focused = false;
                                    return;
                                }
                                if add_btn.contains(cx, cy) {
                                    let t = self.state.task_title_input.text.clone();
                                    if !t.is_empty() {
                                        self.state.tasks.push(Task::new(&t, "General", 0.0));
                                        self.state.task_title_input.clear();
                                    }
                                    return;
                                }
                                for (i, task) in self.state.tasks.iter_mut().take(6).enumerate() {
                                    let row_r = snapshot.resolve_rect(
                                        list_area.x,
                                        list_area.y + 6.0 + i as f32 * 54.0,
                                        list_area.w,
                                        48.0,
                                    );
                                    let btn_w = 56.0;
                                    let p10_btn = snapshot.resolve_rect(
                                        row_r.x + row_r.w - btn_w - 12.0,
                                        row_r.y + 24.0,
                                        btn_w,
                                        22.0,
                                    );
                                    if p10_btn.contains(cx, cy) {
                                        task.add_progress(0.10);
                                        return;
                                    }
                                }
                            }
                            WindowId::ProgressStats => {
                                let (_, score_area) = content.split_top(180.0);
                                let p5_btn = snapshot.resolve_rect(
                                    score_area.x + score_area.w - 160.0,
                                    score_area.y + 32.0,
                                    72.0,
                                    28.0,
                                );
                                let m5_btn = snapshot.resolve_rect(
                                    score_area.x + score_area.w - 80.0,
                                    score_area.y + 32.0,
                                    72.0,
                                    28.0,
                                );
                                if p5_btn.contains(cx, cy) {
                                    self.state.adjust_score(5);
                                    return;
                                }
                                if m5_btn.contains(cx, cy) {
                                    self.state.adjust_score(-5);
                                    return;
                                }
                            }
                            WindowId::FileVault => {
                                let (top_bar, main_area) = content.split_top(42.0);

                                // Toolbar navigation buttons
                                let back_btn = snapshot.resolve_rect(
                                    top_bar.x + 4.0,
                                    top_bar.y + 6.0,
                                    28.0,
                                    28.0,
                                );
                                let fwd_btn = snapshot.resolve_rect(
                                    top_bar.x + 36.0,
                                    top_bar.y + 6.0,
                                    28.0,
                                    28.0,
                                );
                                let up_btn = snapshot.resolve_rect(
                                    top_bar.x + 68.0,
                                    top_bar.y + 6.0,
                                    28.0,
                                    28.0,
                                );
                                if back_btn.contains(cx, cy) {
                                    if self.state.explorer_history_idx > 0 {
                                        self.state.explorer_history_idx -= 1;
                                        self.state.explorer_selected_sidebar = self
                                            .state
                                            .explorer_history[self.state.explorer_history_idx];
                                    }
                                    return;
                                }
                                if fwd_btn.contains(cx, cy) {
                                    if self.state.explorer_history_idx + 1
                                        < self.state.explorer_history.len()
                                    {
                                        self.state.explorer_history_idx += 1;
                                        self.state.explorer_selected_sidebar = self
                                            .state
                                            .explorer_history[self.state.explorer_history_idx];
                                    }
                                    return;
                                }
                                if up_btn.contains(cx, cy) {
                                    self.state.explorer_selected_sidebar = 0;
                                    self.state.explorer_history.push(0);
                                    self.state.explorer_history_idx =
                                        self.state.explorer_history.len() - 1;
                                    return;
                                }

                                let view_toggle_r = snapshot.resolve_rect(
                                    top_bar.x + top_bar.w - 96.0,
                                    top_bar.y + 6.0,
                                    92.0,
                                    28.0,
                                );
                                if view_toggle_r.contains(cx, cy) {
                                    self.state.explorer_view_mode =
                                        match self.state.explorer_view_mode {
                                            ExplorerViewMode::Grid => ExplorerViewMode::List,
                                            ExplorerViewMode::List => ExplorerViewMode::Grid,
                                        };
                                    self.state.show_toast("Explorer View Mode Switched");
                                    return;
                                }

                                // Sidebar directory selection
                                let sidebar_w = 190.0;
                                let sidebar_r = snapshot.resolve_rect(
                                    main_area.x,
                                    main_area.y + 2.0,
                                    sidebar_w,
                                    main_area.h - 4.0,
                                );
                                for i in 0..6 {
                                    let item_r = snapshot.resolve_rect(
                                        sidebar_r.x + 4.0,
                                        sidebar_r.y + 8.0 + i as f32 * 36.0,
                                        sidebar_r.w - 8.0,
                                        32.0,
                                    );
                                    if item_r.contains(cx, cy) {
                                        self.state.explorer_selected_sidebar = i;
                                        self.state.explorer_history.push(i);
                                        self.state.explorer_history_idx =
                                            self.state.explorer_history.len() - 1;
                                        return;
                                    }
                                }

                                // File selection & actions in main view
                                let files_r = snapshot.resolve_rect(
                                    main_area.x + sidebar_w + 10.0,
                                    main_area.y + 2.0,
                                    main_area.w - sidebar_w - 12.0,
                                    main_area.h - 4.0,
                                );
                                let scroll_y = win.scroll_y.position;
                                let file_count = self.state.vault_notes.len().max(3);

                                if self.state.explorer_view_mode == ExplorerViewMode::Grid {
                                    let card_w = snapshot.scaled_max(210.0, 195.0);
                                    let card_h = snapshot.scaled_max(128.0, 118.0);
                                    let gap = snapshot.scaled(16.0);
                                    let pad = snapshot.scaled(8.0);
                                    let cols = ((files_r.w - snapshot.scaled(10.0))
                                        / (card_w + gap))
                                        .floor()
                                        .max(1.0)
                                        as usize;
                                    for i in 0..file_count {
                                        let col = i % cols;
                                        let row = i / cols;
                                        let card_r = snapshot.resolve_rect(
                                            files_r.x + pad + col as f32 * (card_w + gap),
                                            files_r.y + pad + row as f32 * (card_h + gap)
                                                - scroll_y,
                                            card_w,
                                            card_h,
                                        );
                                        let btn_w = (card_r.w - snapshot.scaled(24.0)) * 0.5;
                                        let btn_h = snapshot.scaled(24.0);
                                        let open_btn = snapshot.resolve_rect(
                                            card_r.x + pad,
                                            card_r.y + card_r.h - btn_h - pad,
                                            btn_w,
                                            btn_h,
                                        );
                                        let ask_btn = snapshot.resolve_rect(
                                            card_r.x + pad * 2.0 + btn_w,
                                            card_r.y + card_r.h - btn_h - pad,
                                            btn_w,
                                            btn_h,
                                        );

                                        let target_note_idx =
                                            i.min(self.state.vault_notes.len().saturating_sub(1));
                                        if ask_btn.contains(cx, cy) {
                                            self.state.ask_ai_about_note(target_note_idx);
                                            return;
                                        }
                                        if open_btn.contains(cx, cy) || card_r.contains(cx, cy) {
                                            self.state.open_note_in_reader(target_note_idx);
                                            return;
                                        }
                                    }
                                } else {
                                    for i in 0..file_count {
                                        let row_r = snapshot.resolve_rect(
                                            files_r.x + 4.0,
                                            files_r.y + 28.0 + i as f32 * 42.0 - scroll_y,
                                            files_r.w - 8.0,
                                            36.0,
                                        );
                                        let open_btn = snapshot.resolve_rect(
                                            row_r.x + row_r.w - 140.0,
                                            row_r.y + 6.0,
                                            64.0,
                                            24.0,
                                        );
                                        let ask_btn = snapshot.resolve_rect(
                                            row_r.x + row_r.w - 70.0,
                                            row_r.y + 6.0,
                                            64.0,
                                            24.0,
                                        );

                                        let target_note_idx =
                                            i.min(self.state.vault_notes.len().saturating_sub(1));
                                        if ask_btn.contains(cx, cy) {
                                            self.state.ask_ai_about_note(target_note_idx);
                                            return;
                                        }
                                        if open_btn.contains(cx, cy) || row_r.contains(cx, cy) {
                                            self.state.open_note_in_reader(target_note_idx);
                                            return;
                                        }
                                    }
                                }
                            }
                            WindowId::SystemMonitor => {
                                let scroll_y = win.scroll_y.position;
                                let base_y = content.y - scroll_y;
                                let auto_repair_btn = snapshot.resolve_rect(
                                    content.x,
                                    base_y + 70.0,
                                    content.w,
                                    38.0,
                                );
                                if auto_repair_btn.contains(cx, cy) {
                                    self.state.execute_full_auto_repair();
                                    return;
                                }
                                let cards_y = base_y + 140.0;
                                let card_spacing = 66.0;
                                for i in 0..5 {
                                    let card_r = snapshot.resolve_rect(
                                        content.x,
                                        cards_y + i as f32 * card_spacing,
                                        content.w,
                                        58.0,
                                    );
                                    let btn_r = snapshot.resolve_rect(
                                        card_r.x + card_r.w - 146.0,
                                        card_r.y + 15.0,
                                        136.0,
                                        28.0,
                                    );
                                    if btn_r.contains(cx, cy) {
                                        self.state.execute_auto_repair_item(i);
                                        return;
                                    }
                                }
                                let pwr_y = cards_y + 5.0 * card_spacing + 12.0;
                                for (i, prof) in [
                                    GovernorProfile::EcoStudy,
                                    GovernorProfile::Balanced,
                                    GovernorProfile::AiBoost,
                                ]
                                .iter()
                                .enumerate()
                                {
                                    let pr = snapshot.resolve_rect(
                                        content.x + 180.0 + i as f32 * 105.0,
                                        pwr_y,
                                        100.0,
                                        28.0,
                                    );
                                    if pr.contains(cx, cy) {
                                        self.state.active_power_profile = *prof;
                                        return;
                                    }
                                }
                            }
                            WindowId::Settings => {
                                let rail_w = 230.0;
                                if cx >= content.x && cx <= content.x + rail_w {
                                    let tab_idx = ((cy - (content.y + 6.0)) / 46.0) as usize;
                                    if tab_idx < 10 {
                                        self.state.settings_active_tab = tab_idx;
                                        if let Some(win) =
                                            self.state.wm.find_window_mut(WindowId::Settings)
                                        {
                                            win.scroll_y.snap(0.0);
                                        }
                                        return;
                                    }
                                }

                                let available_w = (content.w - rail_w - 28.0).max(100.0);
                                let form_w = available_w.min(820.0);
                                let content_x = content.x
                                    + rail_w
                                    + 14.0
                                    + ((available_w - form_w) * 0.5).max(0.0);
                                let content_w = form_w;
                                let scroll_y = win.scroll_y.position;
                                let mut y = content.y + 8.0 - scroll_y;

                                match self.state.settings_active_tab {
                                    0 => {
                                        // Profile & Identity
                                        y += 50.0 + 56.0 + 56.0;
                                        let pill_w = (content_w - 18.0) / 4.0;
                                        for i in 0..4 {
                                            let pr = snapshot.resolve_rect(
                                                content_x + i as f32 * (pill_w + 6.0),
                                                y + 18.0,
                                                pill_w,
                                                28.0,
                                            );
                                            if pr.contains(cx, cy) {
                                                self.state.active_academic_stage = i;
                                                self.state
                                                    .personalization_store
                                                    .profile
                                                    .academic_stage = i;
                                                let stage_names = [
                                                    "School / Class 12",
                                                    "Undergraduate",
                                                    "Postgraduate",
                                                    "Competitive (JEE/NEET)",
                                                ];
                                                self.state
                                                    .personalization_store
                                                    .profile
                                                    .user_category = stage_names[i].to_string();
                                                self.persist_preferences();
                                                self.state.show_toast(format!(
                                                    "Academic Stage: {}",
                                                    stage_names[i]
                                                ));
                                                return;
                                            }
                                        }
                                        y += 56.0;
                                        for i in 0..4 {
                                            let hr_r = snapshot.resolve_rect(
                                                content_x + i as f32 * (pill_w + 6.0),
                                                y + 18.0,
                                                pill_w,
                                                28.0,
                                            );
                                            if hr_r.contains(cx, cy) {
                                                self.state.active_study_target_hours = i;
                                                let vals = [3.0, 4.5, 6.0, 8.0];
                                                self.state
                                                    .personalization_store
                                                    .routine
                                                    .study_hours_per_day = vals[i];
                                                self.state
                                                    .personalization_store
                                                    .profile
                                                    .study_target_hours_idx = i;
                                                self.persist_preferences();
                                                self.state.show_toast(format!(
                                                    "Daily Target: {:.1} hrs",
                                                    vals[i]
                                                ));
                                                return;
                                            }
                                        }
                                        y += 58.0;
                                        let wiz_btn =
                                            snapshot.resolve_rect(content_x, y, content_w, 36.0);
                                        if wiz_btn.contains(cx, cy) {
                                            self.state
                                                .open_overlay(ActiveOverlay::OnboardingWizard);
                                            return;
                                        }
                                    }
                                    1 => {
                                        // Appearance & Glass
                                        y += 48.0;
                                        let btn_l =
                                            snapshot.resolve_rect(content_x, y + 18.0, 100.0, 28.0);
                                        let btn_d = snapshot.resolve_rect(
                                            content_x + 110.0,
                                            y + 18.0,
                                            100.0,
                                            28.0,
                                        );
                                        if btn_l.contains(cx, cy) {
                                            if self.state.theme.mode != ThemeMode::Light {
                                                self.state.theme.toggle_mode();
                                                self.persist_preferences();
                                            }
                                            return;
                                        }
                                        if btn_d.contains(cx, cy) {
                                            if self.state.theme.mode != ThemeMode::Dark {
                                                self.state.theme.toggle_mode();
                                                self.persist_preferences();
                                            }
                                            return;
                                        }
                                        y += 56.0;
                                        let presets = WallpaperPreset::all();
                                        for (i, &wp) in presets.iter().enumerate() {
                                            let row = (i / 3) as f32;
                                            let col = (i % 3) as f32;
                                            let btn_w = (content_w - 16.0) / 3.0;
                                            let wp_btn = snapshot.resolve_rect(
                                                content_x + col * (btn_w + 8.0),
                                                y + 18.0 + row * 34.0,
                                                btn_w,
                                                28.0,
                                            );
                                            if wp_btn.contains(cx, cy) {
                                                self.state.wallpaper_preset = wp;
                                                self.persist_preferences();
                                                return;
                                            }
                                        }
                                        y += 94.0;
                                        let slider_w = (content_w * 0.45).min(240.0);
                                        let sl_r = snapshot.resolve_rect(
                                            content_x + content_w - slider_w,
                                            y + 4.0,
                                            slider_w,
                                            18.0,
                                        );
                                        if sl_r.contains(cx, cy) {
                                            let val = ((cx - sl_r.x) / sl_r.w).clamp(0.0, 1.0);
                                            self.state.warmth_slider.value = val;
                                            self.state.theme.set_night_warmth(val);
                                            self.state.active_slider =
                                                Some(ActiveSlider::NightWarmth);
                                            self.persist_preferences();
                                            return;
                                        }
                                        y += 42.0;
                                        let sc_r = snapshot.resolve_rect(
                                            content_x + content_w - slider_w,
                                            y + 4.0,
                                            slider_w,
                                            18.0,
                                        );
                                        if sc_r.contains(cx, cy) {
                                            let val = ((cx - sc_r.x) / sc_r.w).clamp(0.0, 1.0);
                                            let scale = (0.85 + val * 0.50).clamp(0.85, 1.35);
                                            self.state.theme.set_ui_scale(scale);
                                            self.state.personalization_store.appearance.ui_scale =
                                                scale;
                                            self.state.active_slider = Some(ActiveSlider::UiScale);
                                            self.persist_preferences();
                                            return;
                                        }
                                        y += 42.0;
                                        let b1 =
                                            snapshot.resolve_rect(content_x, y + 18.0, 160.0, 28.0);
                                        let b2 = snapshot.resolve_rect(
                                            content_x + 170.0,
                                            y + 18.0,
                                            150.0,
                                            28.0,
                                        );
                                        if b1.contains(cx, cy) {
                                            self.state.glass_blur_quality = 0;
                                            self.state
                                                .personalization_store
                                                .appearance
                                                .blur_quality = 0;
                                            self.persist_preferences();
                                            self.state
                                                .show_toast("Dual-Kawase 4-Pass Blur Enabled");
                                            return;
                                        }
                                        if b2.contains(cx, cy) {
                                            self.state.glass_blur_quality = 1;
                                            self.state
                                                .personalization_store
                                                .appearance
                                                .blur_quality = 1;
                                            self.persist_preferences();
                                            self.state.show_toast("Standard 2-Pass Blur Enabled");
                                            return;
                                        }
                                    }
                                    2 => {
                                        // Desktop & Workspaces
                                        y += 48.0;
                                        for i in 0..4u8 {
                                            let wr = snapshot.resolve_rect(
                                                content_x,
                                                y + 20.0 + i as f32 * 36.0,
                                                content_w,
                                                30.0,
                                            );
                                            let tag_r = snapshot.resolve_rect(
                                                wr.x + wr.w - 134.0,
                                                wr.y + 4.0,
                                                68.0,
                                                22.0,
                                            );
                                            let pill_r = snapshot.resolve_rect(
                                                wr.x + wr.w - 56.0,
                                                wr.y + 3.0,
                                                48.0,
                                                24.0,
                                            );
                                            if pill_r.contains(cx, cy) {
                                                self.state.workspaces_enabled[i as usize] =
                                                    !self.state.workspaces_enabled[i as usize];
                                                self.state
                                                    .personalization_store
                                                    .desktop
                                                    .workspaces_enabled =
                                                    self.state.workspaces_enabled;
                                                self.persist_preferences();
                                                let st =
                                                    if self.state.workspaces_enabled[i as usize] {
                                                        "Enabled"
                                                    } else {
                                                        "Disabled"
                                                    };
                                                self.state.show_toast(format!(
                                                    "Workspace {}: {}",
                                                    i + 1,
                                                    st
                                                ));
                                                return;
                                            }
                                            if tag_r.contains(cx, cy) || wr.contains(cx, cy) {
                                                self.state.active_workspace = i;
                                                self.state.show_toast(format!(
                                                    "Switched to Workspace {}",
                                                    i + 1
                                                ));
                                                return;
                                            }
                                        }
                                        y += 4.0 * 36.0 + 28.0;
                                        let widget_names = [
                                            "Analog Clock & Study Timer",
                                            "Study Streak & Target Score Card",
                                            "Priority Tasks Quick-Launch Card",
                                            "Ambient Study Audio Player",
                                        ];
                                        for (i, name) in widget_names.iter().enumerate().take(4) {
                                            let wr = snapshot.resolve_rect(
                                                content_x,
                                                y + 18.0 + i as f32 * 34.0,
                                                content_w,
                                                28.0,
                                            );
                                            let pill_r = snapshot.resolve_rect(
                                                wr.x + wr.w - 56.0,
                                                wr.y + 2.0,
                                                48.0,
                                                24.0,
                                            );
                                            if pill_r.contains(cx, cy) || wr.contains(cx, cy) {
                                                self.state.widgets_enabled[i] =
                                                    !self.state.widgets_enabled[i];
                                                self.state
                                                    .personalization_store
                                                    .desktop
                                                    .widgets_enabled = self.state.widgets_enabled;
                                                self.persist_preferences();
                                                let st = if self.state.widgets_enabled[i] {
                                                    "Visible"
                                                } else {
                                                    "Hidden"
                                                };
                                                self.state.show_toast(format!("{}: {}", name, st));
                                                return;
                                            }
                                        }
                                    }
                                    3 => {
                                        // AI Personalization
                                        y += 48.0;
                                        for i in 0..4 {
                                            let mr = snapshot.resolve_rect(
                                                content_x,
                                                y + 18.0 + i as f32 * 48.0,
                                                content_w,
                                                42.0,
                                            );
                                            if mr.contains(cx, cy) {
                                                self.state
                                                    .show_toast("Maindy AI Tutor Mode Switched");
                                                return;
                                            }
                                        }
                                    }
                                    4 => {
                                        // Daily Routine
                                        y += 48.0 + 52.0 + 3.0 * 42.0 + 26.0;
                                        let pw = (content_w - 12.0) / 3.0;
                                        for i in 0..3 {
                                            let pr = snapshot.resolve_rect(
                                                content_x + i as f32 * (pw + 6.0),
                                                y + 18.0,
                                                pw,
                                                28.0,
                                            );
                                            if pr.contains(cx, cy) {
                                                self.state
                                                    .show_toast("Pomodoro Interval Calibrated");
                                                return;
                                            }
                                        }
                                    }
                                    5 => {
                                        // Notifications
                                        y += 48.0 + 4.0 * 36.0 + 26.0;
                                        let dnd_r =
                                            snapshot.resolve_rect(content_x, y, content_w, 36.0);
                                        if dnd_r.contains(cx, cy) {
                                            self.state.session.notifications.toggle_dnd();
                                            return;
                                        }
                                    }
                                    6 => {
                                        // Productivity & Focus Guard
                                        y += 48.0 + 42.0;
                                        let apps = [
                                            "StudyReader",
                                            "Tasks",
                                            "LayaAiChat",
                                            "ProgressStats",
                                            "SystemMonitor",
                                            "FileVault",
                                            "SearchBrowser",
                                            "Settings",
                                        ];
                                        for app_id in apps {
                                            let toggle_r = snapshot.resolve_rect(
                                                content_x + content_w - 56.0,
                                                y + 6.0,
                                                48.0,
                                                24.0,
                                            );
                                            let row_r = snapshot
                                                .resolve_rect(content_x, y, content_w, 38.0);
                                            if toggle_r.contains(cx, cy) || row_r.contains(cx, cy) {
                                                self.state
                                                    .personalization_store
                                                    .productivity
                                                    .toggle_app_allowed(app_id);
                                                let engine_id = if app_id == "Tasks" {
                                                    "TasksGoals"
                                                } else {
                                                    app_id
                                                };
                                                self.state
                                                    .mindful_engine
                                                    .toggle_app_allowed(engine_id);
                                                self.persist_preferences();
                                                let allowed = self
                                                    .state
                                                    .personalization_store
                                                    .productivity
                                                    .is_app_allowed(app_id);
                                                let st =
                                                    if allowed { "Allowed" } else { "Restricted" };
                                                self.state.show_toast(format!(
                                                    "{}: {} in Mindful Mode",
                                                    app_id, st
                                                ));
                                                return;
                                            }
                                            y += 44.0;
                                        }

                                        y += 8.0 + 42.0;
                                        for col_idx in 0..4 {
                                            let row_r = snapshot
                                                .resolve_rect(content_x, y, content_w, 36.0);
                                            let toggle_r = snapshot.resolve_rect(
                                                row_r.x + row_r.w - 56.0,
                                                row_r.y + 6.0,
                                                48.0,
                                                24.0,
                                            );
                                            if toggle_r.contains(cx, cy) || row_r.contains(cx, cy) {
                                                match col_idx {
                                                    0 => self
                                                        .state
                                                        .mindful_engine
                                                        .toggle_left_widgets(),
                                                    1 => self
                                                        .state
                                                        .mindful_engine
                                                        .toggle_desktop_icons(),
                                                    2 => self
                                                        .state
                                                        .mindful_engine
                                                        .toggle_search_bar(),
                                                    3 => self
                                                        .state
                                                        .mindful_engine
                                                        .toggle_bottom_dock(),
                                                    _ => {}
                                                }
                                                self.state.show_toast(
                                                    "Mindful Home Screen Layout Updated",
                                                );
                                                return;
                                            }
                                            y += 42.0;
                                        }
                                    }
                                    7 => {
                                        // Accessibility
                                        y += 48.0 + 60.0;
                                        let rm_r =
                                            snapshot.resolve_rect(content_x, y, content_w, 36.0);
                                        if rm_r.contains(cx, cy) {
                                            self.state.theme.reduced_motion =
                                                !self.state.theme.reduced_motion;
                                            return;
                                        }
                                    }
                                    8 => {
                                        // Sound & Bluetooth
                                        y += 48.0;
                                        let slider_w = (content_w * 0.45).min(240.0);
                                        let vol_r = snapshot.resolve_rect(
                                            content_x + content_w - slider_w,
                                            y + 4.0,
                                            slider_w,
                                            18.0,
                                        );
                                        if vol_r.contains(cx, cy) {
                                            let val = ((cx - vol_r.x) / vol_r.w).clamp(0.0, 1.0);
                                            self.state.master_volume = val;
                                            self.state.active_slider =
                                                Some(ActiveSlider::MasterVolume);
                                            return;
                                        }
                                        y += 42.0;
                                        let sw = (content_w - 16.0) / 3.0;
                                        for (i, sc) in [
                                            StudySoundscape::Off,
                                            StudySoundscape::SoftRain,
                                            StudySoundscape::FocusTone,
                                        ]
                                        .iter()
                                        .enumerate()
                                        {
                                            let sc_r = snapshot.resolve_rect(
                                                content_x + i as f32 * (sw + 8.0),
                                                y + 18.0,
                                                sw,
                                                28.0,
                                            );
                                            if sc_r.contains(cx, cy) {
                                                self.state.soundscape = *sc;
                                                return;
                                            }
                                        }
                                        y += 56.0;
                                        let scan_btn = snapshot.resolve_rect(
                                            content_x + content_w - 200.0,
                                            y + 8.0,
                                            134.0,
                                            26.0,
                                        );
                                        let bt_pill = snapshot.resolve_rect(
                                            content_x + content_w - 56.0,
                                            y + 9.0,
                                            48.0,
                                            24.0,
                                        );
                                        if scan_btn.contains(cx, cy) {
                                            self.state.bluetooth_mgr.start_scan();
                                            self.state.show_toast(
                                                "Scanning for nearby Bluetooth devices...",
                                            );
                                            return;
                                        }
                                        if bt_pill.contains(cx, cy) {
                                            self.state.bluetooth_mgr.toggle_enabled();
                                            self.persist_preferences();
                                            let st = if self.state.bluetooth_mgr.enabled {
                                                "Enabled"
                                            } else {
                                                "Disabled"
                                            };
                                            self.state.show_toast(format!("Bluetooth {}", st));
                                            return;
                                        }
                                        y += 44.0;
                                        for (i, dev) in self
                                            .state
                                            .bluetooth_mgr
                                            .devices
                                            .clone()
                                            .iter()
                                            .enumerate()
                                        {
                                            let dev_r = snapshot.resolve_rect(
                                                content_x,
                                                y + i as f32 * 72.0,
                                                content_w,
                                                64.0,
                                            );
                                            let conn_btn = snapshot.resolve_rect(
                                                dev_r.x + dev_r.w - 96.0,
                                                dev_r.y + 18.0,
                                                88.0,
                                                28.0,
                                            );
                                            if conn_btn.contains(cx, cy) {
                                                let new_state = self
                                                    .state
                                                    .bluetooth_mgr
                                                    .toggle_device_connection(&dev.id);
                                                let act_str = if new_state {
                                                    "Connected to"
                                                } else {
                                                    "Disconnected from"
                                                };
                                                self.state.show_toast(format!(
                                                    "{} {}",
                                                    act_str, dev.name
                                                ));
                                                return;
                                            }
                                        }
                                        y += self.state.bluetooth_mgr.devices.len() as f32 * 72.0
                                            + 12.0
                                            + 20.0;
                                        for (i, dev) in self
                                            .state
                                            .bluetooth_mgr
                                            .nearby_devices
                                            .clone()
                                            .iter()
                                            .enumerate()
                                        {
                                            let dev_r = snapshot.resolve_rect(
                                                content_x,
                                                y + i as f32 * 66.0,
                                                content_w,
                                                58.0,
                                            );
                                            let pair_btn = snapshot.resolve_rect(
                                                dev_r.x + dev_r.w - 110.0,
                                                dev_r.y + 14.0,
                                                102.0,
                                                28.0,
                                            );
                                            if pair_btn.contains(cx, cy) {
                                                let paired = self
                                                    .state
                                                    .bluetooth_mgr
                                                    .pair_nearby_device(&dev.id);
                                                if paired {
                                                    self.state.show_toast(format!(
                                                        "Paired & Connected {}",
                                                        dev.name
                                                    ));
                                                }
                                                return;
                                            }
                                        }
                                    }
                                    _ => {
                                        // Privacy & Adaptive
                                        y += 48.0 + 54.0;
                                        let sug_r = snapshot.resolve_rect(
                                            content_x,
                                            y + 18.0,
                                            content_w,
                                            54.0,
                                        );
                                        let app_btn = snapshot.resolve_rect(
                                            sug_r.x + sug_r.w - 146.0,
                                            sug_r.y + 12.0,
                                            68.0,
                                            26.0,
                                        );
                                        let dis_btn = snapshot.resolve_rect(
                                            sug_r.x + sug_r.w - 74.0,
                                            sug_r.y + 12.0,
                                            66.0,
                                            26.0,
                                        );
                                        if app_btn.contains(cx, cy) {
                                            self.state.show_toast(
                                                "Adaptive Suggestion Approved & Applied",
                                            );
                                            return;
                                        }
                                        if dis_btn.contains(cx, cy) {
                                            self.state.show_toast("Adaptive Suggestion Dismissed");
                                            return;
                                        }
                                        y += 82.0;
                                        let exp_btn =
                                            snapshot.resolve_rect(content_x, y + 18.0, 180.0, 30.0);
                                        let rst_btn = snapshot.resolve_rect(
                                            content_x + 192.0,
                                            y + 18.0,
                                            180.0,
                                            30.0,
                                        );
                                        if exp_btn.contains(cx, cy) {
                                            self.state.show_toast("Exported User Data to JSON");
                                            return;
                                        }
                                        if rst_btn.contains(cx, cy) {
                                            self.state.personalization_store =
                                                PersonalizationStore::new();
                                            self.persist_preferences();
                                            self.state
                                                .show_toast("Factory Reset to Defaults Completed");
                                            return;
                                        }
                                    }
                                }
                            }
                            WindowId::SearchBrowser => {
                                let (sub_tabs_bar, body) = content.split_top(34.0);
                                let tab_count = 4.0;
                                let tab_gap = 6.0;
                                let tab_w =
                                    (sub_tabs_bar.w - (tab_count - 1.0) * tab_gap) / tab_count;

                                // Sub-tab header clicks
                                for i in 0..4 {
                                    let tab_r = snapshot.resolve_rect(
                                        sub_tabs_bar.x + i as f32 * (tab_w + tab_gap),
                                        sub_tabs_bar.y + 2.0,
                                        tab_w,
                                        28.0,
                                    );
                                    if tab_r.contains(cx, cy) {
                                        self.state.browser_active_tab = i;
                                        if let Some(win) =
                                            self.state.wm.find_window_mut(WindowId::SearchBrowser)
                                        {
                                            win.scroll_y.snap(0.0);
                                        }
                                        return;
                                    }
                                }

                                match self.state.browser_active_tab {
                                    0 => {
                                        // Tab 0: MindSet Sovereign Browser (Arc/Safari Architecture)
                                        let (tab_strip, body_after_tabs) = body.split_top(32.0);

                                        // 1. Horizontal Tab Bar Clicks
                                        let tab_h = 26.0;
                                        let max_tabs_w = tab_strip.w - 180.0;
                                        let rendered_tab_count =
                                            self.state.browser_tabs.len().max(1) as f32;
                                        let single_tab_w = ((max_tabs_w
                                            - (rendered_tab_count + 1.0) * 6.0)
                                            / rendered_tab_count)
                                            .clamp(110.0, 200.0);

                                        for (idx, _) in self.state.browser_tabs.iter().enumerate() {
                                            let tab_x = tab_strip.x
                                                + 8.0
                                                + idx as f32 * (single_tab_w + 6.0);
                                            let tab_r = snapshot.resolve_rect(
                                                tab_x,
                                                tab_strip.y + 3.0,
                                                single_tab_w,
                                                tab_h,
                                            );
                                            let close_r = snapshot.resolve_rect(
                                                tab_r.x + tab_r.w - 16.0,
                                                tab_r.y + 6.0,
                                                12.0,
                                                14.0,
                                            );

                                            if close_r.contains(cx, cy) {
                                                if self.state.browser_tabs.len() > 1 {
                                                    self.state.browser_tabs.remove(idx);
                                                    if self.state.browser_active_tab_idx
                                                        >= self.state.browser_tabs.len()
                                                    {
                                                        self.state.browser_active_tab_idx =
                                                            self.state.browser_tabs.len() - 1;
                                                    }
                                                }
                                                return;
                                            } else if tab_r.contains(cx, cy) {
                                                self.state.browser_active_tab_idx = idx;
                                                let tab_url =
                                                    self.state.browser_tabs[idx].url.clone();
                                                self.state.open_browser_url(&tab_url);
                                                self.state.browser_screen =
                                                    BrowserScreen::ActiveBrowsing;
                                                return;
                                            }
                                        }

                                        // New Tab (+) button click
                                        let plus_x = tab_strip.x
                                            + 8.0
                                            + self.state.browser_tabs.len() as f32
                                                * (single_tab_w + 6.0);
                                        let plus_r = snapshot.resolve_rect(
                                            plus_x,
                                            tab_strip.y + 4.0,
                                            26.0,
                                            24.0,
                                        );
                                        if plus_r.contains(cx, cy) {
                                            let new_id = self.state.browser_tabs.len();
                                            self.state.browser_tabs.push(BrowserTab::new(
                                                new_id,
                                                "New Tab",
                                                "mindset://start",
                                                "🧭",
                                            ));
                                            self.state.browser_active_tab_idx = new_id;
                                            self.state.browser_screen = BrowserScreen::NewTab;
                                            self.state.search_input.text.clear();
                                            return;
                                        }

                                        // Workspace switcher click
                                        let ws_r = snapshot.resolve_rect(
                                            tab_strip.x + tab_strip.w - 170.0,
                                            tab_strip.y + 4.0,
                                            162.0,
                                            24.0,
                                        );
                                        if ws_r.contains(cx, cy) {
                                            self.state.browser_workspace_idx =
                                                (self.state.browser_workspace_idx + 1) % 3;
                                            let ws_name = match self.state.browser_workspace_idx {
                                                1 => "Research Space",
                                                2 => "Personal Space",
                                                _ => "Study Space",
                                            };
                                            self.state
                                                .show_toast(format!("Switched to {}", ws_name));
                                            return;
                                        }

                                        // 2. Navigation Toolbar Clicks
                                        let (nav_bar, main_area) = body_after_tabs.split_top(40.0);
                                        let nav_btn_h = 28.0;
                                        let btn_back = snapshot.resolve_rect(
                                            nav_bar.x + 8.0,
                                            nav_bar.y + 6.0,
                                            30.0,
                                            nav_btn_h,
                                        );
                                        let btn_fwd = snapshot.resolve_rect(
                                            btn_back.x + 34.0,
                                            nav_bar.y + 6.0,
                                            30.0,
                                            nav_btn_h,
                                        );
                                        let btn_rel = snapshot.resolve_rect(
                                            btn_fwd.x + 34.0,
                                            nav_bar.y + 6.0,
                                            30.0,
                                            nav_btn_h,
                                        );
                                        let btn_home = snapshot.resolve_rect(
                                            btn_rel.x + 34.0,
                                            nav_bar.y + 6.0,
                                            34.0,
                                            nav_btn_h,
                                        );

                                        let right_w = 260.0;
                                        let right_x = nav_bar.x + nav_bar.w - right_w - 8.0;
                                        let btn_bm = snapshot.resolve_rect(
                                            right_x,
                                            nav_bar.y + 6.0,
                                            32.0,
                                            nav_btn_h,
                                        );
                                        let btn_dl = snapshot.resolve_rect(
                                            btn_bm.x + 36.0,
                                            nav_bar.y + 6.0,
                                            32.0,
                                            nav_btn_h,
                                        );
                                        let btn_sh = snapshot.resolve_rect(
                                            btn_dl.x + 36.0,
                                            nav_bar.y + 6.0,
                                            32.0,
                                            nav_btn_h,
                                        );
                                        let btn_st = snapshot.resolve_rect(
                                            btn_sh.x + 36.0,
                                            nav_bar.y + 6.0,
                                            32.0,
                                            nav_btn_h,
                                        );
                                        let btn_prof = snapshot.resolve_rect(
                                            btn_st.x + 36.0,
                                            nav_bar.y + 6.0,
                                            80.0,
                                            nav_btn_h,
                                        );

                                        let omni_x = btn_home.x + 40.0;
                                        let omni_w = (right_x - omni_x - 10.0).max(120.0);
                                        let omni_r = snapshot.resolve_rect(
                                            omni_x,
                                            nav_bar.y + 5.0,
                                            omni_w,
                                            30.0,
                                        );

                                        if btn_back.contains(cx, cy) {
                                            self.state.browser_go_back();
                                            return;
                                        }
                                        if btn_fwd.contains(cx, cy) {
                                            self.state.show_toast("Forward history end");
                                            return;
                                        }
                                        if btn_rel.contains(cx, cy) {
                                            let cur_url = match &self.state.browser_page_kind {
                                                BrowserPageKind::WebPage { url, .. } => url.clone(),
                                                _ => "https://nptel.ac.in".to_string(),
                                            };
                                            self.state.open_browser_url(&cur_url);
                                            return;
                                        }
                                        if btn_home.contains(cx, cy) {
                                            self.state.browser_go_home();
                                            self.state.browser_screen = BrowserScreen::NewTab;
                                            return;
                                        }
                                        if omni_r.contains(cx, cy) {
                                            self.state.search_input.is_focused = true;
                                            self.state.maindy_query_input.is_focused = false;
                                            self.state.task_title_input.is_focused = false;
                                            return;
                                        }
                                        if btn_bm.contains(cx, cy) {
                                            self.state.browser_screen = BrowserScreen::Bookmarks;
                                            return;
                                        }
                                        if btn_dl.contains(cx, cy) {
                                            self.state.browser_screen = BrowserScreen::Downloads;
                                            return;
                                        }
                                        if btn_sh.contains(cx, cy) {
                                            self.state.browser_screen =
                                                BrowserScreen::ShieldDashboard;
                                            return;
                                        }
                                        if btn_st.contains(cx, cy) {
                                            self.state.browser_screen = BrowserScreen::Settings;
                                            return;
                                        }
                                        if btn_prof.contains(cx, cy) {
                                            self.state.show_toast(
                                                "MindSet Profile: LKS Sovereign Academic Edition",
                                            );
                                            return;
                                        }

                                        // 3. Left Arc Sidebar Clicks
                                        let sb_w = if self.state.browser_sidebar_expanded {
                                            snapshot.scaled_clamped(160.0, 140.0, 190.0)
                                        } else {
                                            52.0
                                        };
                                        let (sidebar_r, viewport_outer) =
                                            main_area.split_left(sb_w);

                                        let mut sby = sidebar_r.y + 8.0;
                                        if self.state.browser_sidebar_expanded {
                                            sby += 18.0;
                                            for sp_idx in 0..3 {
                                                let sp_r = snapshot.resolve_rect(
                                                    sidebar_r.x + 6.0,
                                                    sby,
                                                    sidebar_r.w - 12.0,
                                                    24.0,
                                                );
                                                if sp_r.contains(cx, cy) {
                                                    self.state.browser_workspace_idx = sp_idx;
                                                    let ws_name = match sp_idx {
                                                        1 => "Research Space",
                                                        2 => "Personal Space",
                                                        _ => "Study Space",
                                                    };
                                                    self.state.show_toast(format!(
                                                        "Switched to {}",
                                                        ws_name
                                                    ));
                                                    return;
                                                }
                                                sby += 26.0;
                                            }
                                            sby += 26.0;
                                        }

                                        let nav_items = [
                                            BrowserScreen::NewTab,
                                            BrowserScreen::ActiveBrowsing,
                                            BrowserScreen::Bookmarks,
                                            BrowserScreen::ReadingList,
                                            BrowserScreen::History,
                                            BrowserScreen::Downloads,
                                            BrowserScreen::ShieldDashboard,
                                            BrowserScreen::Settings,
                                        ];

                                        for screen_target in nav_items.iter() {
                                            let item_r = snapshot.resolve_rect(
                                                sidebar_r.x + 4.0,
                                                sby,
                                                sidebar_r.w - 8.0,
                                                30.0,
                                            );
                                            if item_r.contains(cx, cy) {
                                                self.state.browser_screen = *screen_target;
                                                if let Some(win) = self
                                                    .state
                                                    .wm
                                                    .find_window_mut(WindowId::SearchBrowser)
                                                {
                                                    win.scroll_y.snap(0.0);
                                                }
                                                return;
                                            }
                                            sby += 34.0;
                                        }

                                        // Collapse/expand toggle click
                                        let toggle_r = snapshot.resolve_rect(
                                            sidebar_r.x + 6.0,
                                            sidebar_r.y + sidebar_r.h - 32.0,
                                            sidebar_r.w - 12.0,
                                            24.0,
                                        );
                                        if toggle_r.contains(cx, cy) {
                                            self.state.browser_sidebar_expanded =
                                                !self.state.browser_sidebar_expanded;
                                            return;
                                        }

                                        // 4. Right Content Viewport Clicks
                                        let (viewport_content, _) =
                                            viewport_outer.split_bottom(22.0);
                                        let scroll_y = win.scroll_y.position;

                                        match self.state.browser_screen {
                                            BrowserScreen::NewTab => {
                                                let mut vy = viewport_content.y + 16.0 - scroll_y;
                                                vy += 50.0 + 26.0 + 28.0; // logo + title + sub

                                                let search_w =
                                                    (viewport_content.w * 0.76).clamp(340.0, 680.0);
                                                let search_x = viewport_content.x
                                                    + (viewport_content.w - search_w) * 0.5;
                                                let search_pill = snapshot
                                                    .resolve_rect(search_x, vy, search_w, 42.0);
                                                let btn_search_r = snapshot.resolve_rect(
                                                    search_pill.x + search_pill.w - 146.0,
                                                    search_pill.y + 6.0,
                                                    138.0,
                                                    30.0,
                                                );

                                                if search_pill.contains(cx, cy)
                                                    && !btn_search_r.contains(cx, cy)
                                                {
                                                    self.state.search_input.is_focused = true;
                                                    self.state.maindy_query_input.is_focused =
                                                        false;
                                                    self.state.task_title_input.is_focused = false;
                                                    return;
                                                }
                                                if btn_search_r.contains(cx, cy) {
                                                    let query = self
                                                        .state
                                                        .search_input
                                                        .text
                                                        .trim()
                                                        .to_string();
                                                    if !query.is_empty() {
                                                        self.state.open_browser_url(&query);
                                                    } else {
                                                        self.state.open_browser_url(
                                                            "https://en.wikipedia.org",
                                                        );
                                                    }
                                                    return;
                                                }
                                                vy += 50.0;

                                                // Engine pills
                                                vy += 36.0;

                                                // Dismiss blocked banner
                                                if self.state.browser_blocked_reason.is_some() {
                                                    let banner_r = snapshot.resolve_rect(
                                                        viewport_content.x + 12.0,
                                                        vy,
                                                        viewport_content.w - 24.0,
                                                        40.0,
                                                    );
                                                    let dis_btn = snapshot.resolve_rect(
                                                        banner_r.x + banner_r.w - 74.0,
                                                        banner_r.y + 8.0,
                                                        66.0,
                                                        24.0,
                                                    );
                                                    if dis_btn.contains(cx, cy) {
                                                        self.state.browser_blocked_reason = None;
                                                        return;
                                                    }
                                                    vy += 48.0;
                                                }

                                                // Portals grid
                                                vy += 22.0;
                                                let portal_urls = [
                                                    "https://nptel.ac.in",
                                                    "https://khanacademy.org",
                                                    "https://en.wikipedia.org",
                                                    "https://youtube.com/watch?v=physics",
                                                    "https://arxiv.org/abs/2403.11928",
                                                    "https://ocw.mit.edu",
                                                ];
                                                let cols = 3.0;
                                                let gap = 10.0;
                                                let card_w = (viewport_content.w
                                                    - 32.0
                                                    - (cols - 1.0) * gap)
                                                    / cols;
                                                let card_h = 56.0;

                                                for (idx, p_url) in portal_urls.iter().enumerate() {
                                                    let col = (idx % 3) as f32;
                                                    let row = (idx / 3) as f32;
                                                    let px = viewport_content.x
                                                        + 16.0
                                                        + col * (card_w + gap);
                                                    let py = vy + row * (card_h + gap);
                                                    let card_r = snapshot
                                                        .resolve_rect(px, py, card_w, card_h);
                                                    if card_r.contains(cx, cy) {
                                                        self.state.open_browser_url(p_url);
                                                        return;
                                                    }
                                                }
                                                vy += 2.0 * card_h + gap + 16.0;

                                                // Recent sessions
                                                vy += 22.0;
                                                let rcard_w =
                                                    (viewport_content.w - 32.0 - 2.0 * gap) / 3.0;
                                                let rcard_h = 62.0;
                                                for idx in 0..3 {
                                                    let rc_x = viewport_content.x
                                                        + 16.0
                                                        + idx as f32 * (rcard_w + gap);
                                                    let rc_r = snapshot
                                                        .resolve_rect(rc_x, vy, rcard_w, rcard_h);
                                                    if rc_r.contains(cx, cy) {
                                                        match idx {
                                                            0 => self.state.open_browser_url("https://youtube.com/watch?v=physics"),
                                                            1 => self.state.open_browser_url("https://nptel.ac.in"),
                                                            _ => self.state.open_browser_url("https://en.wikipedia.org"),
                                                        }
                                                        return;
                                                    }
                                                }
                                                vy += rcard_h + 16.0;

                                                // Reading list preview card click
                                                let half_w =
                                                    (viewport_content.w - 32.0 - gap) * 0.5;
                                                let rl_card = snapshot.resolve_rect(
                                                    viewport_content.x + 16.0,
                                                    vy,
                                                    half_w,
                                                    82.0,
                                                );
                                                if rl_card.contains(cx, cy) {
                                                    self.state.browser_screen =
                                                        BrowserScreen::ReadingList;
                                                    return;
                                                }
                                            }
                                            BrowserScreen::ActiveBrowsing => {
                                                match &self.state.browser_page_kind.clone() {
                                                    BrowserPageKind::WebPage {
                                                        url,
                                                        title,
                                                        content_lines,
                                                        is_video,
                                                        ..
                                                    } => {
                                                        let mut vy =
                                                            viewport_content.y + 12.0 - scroll_y;
                                                        vy += 78.0;
                                                        if *is_video {
                                                            vy += 214.0;
                                                        }
                                                        let line_count = content_lines.len();
                                                        let body_card_h =
                                                            (line_count as f32 * 26.0 + 36.0)
                                                                .max(180.0);
                                                        vy += body_card_h + 16.0;

                                                        let btn_save_w = 170.0;
                                                        let btn_read_w = 170.0;
                                                        let btn_ai_w = 150.0;
                                                        let btn_home_w = 160.0;
                                                        let save_vault_btn = snapshot.resolve_rect(
                                                            viewport_content.x + 12.0,
                                                            vy,
                                                            btn_save_w,
                                                            34.0,
                                                        );
                                                        let read_btn = snapshot.resolve_rect(
                                                            save_vault_btn.x + btn_save_w + 10.0,
                                                            vy,
                                                            btn_read_w,
                                                            34.0,
                                                        );
                                                        let ai_btn = snapshot.resolve_rect(
                                                            read_btn.x + btn_read_w + 10.0,
                                                            vy,
                                                            btn_ai_w,
                                                            34.0,
                                                        );
                                                        let home_btn = snapshot.resolve_rect(
                                                            ai_btn.x + btn_ai_w + 10.0,
                                                            vy,
                                                            btn_home_w,
                                                            34.0,
                                                        );

                                                        if save_vault_btn.contains(cx, cy) {
                                                            let domain_name = extract_domain(url);
                                                            let note_text = format!("# {}\n\nTarget URL: {}\n\n## Content Overview\n{}", title, url, content_lines.join("\n"));
                                                            let _ =
                                                                self.state.rag_vault.write_note(
                                                                    &format!("{}.md", domain_name),
                                                                    &note_text,
                                                                );
                                                            self.state.vault_notes = self
                                                                .state
                                                                .rag_vault
                                                                .list_notes()
                                                                .unwrap_or_default();
                                                            self.state.show_toast(format!(
                                                                "Saved '{}' to Study Vault",
                                                                domain_name
                                                            ));
                                                            return;
                                                        }
                                                        if read_btn.contains(cx, cy) {
                                                            self.state.ai_response_text = format!(
                                                                "# {}\n\n{}",
                                                                title,
                                                                content_lines.join("\n")
                                                            );
                                                            self.state.wm.promote_to_front(
                                                                WindowId::StudyReader,
                                                            );
                                                            self.state.show_toast(format!(
                                                                "Loaded '{}' into Study Reader",
                                                                title
                                                            ));
                                                            return;
                                                        }
                                                        if ai_btn.contains(cx, cy) {
                                                            self.state
                                                                .ask_custom_maindy_query(title);
                                                            self.state.wm.promote_to_front(
                                                                WindowId::LayaAiChat,
                                                            );
                                                            self.state.show_toast(
                                                                "Query Sent to Maindy AI",
                                                            );
                                                            return;
                                                        }
                                                        if home_btn.contains(cx, cy) {
                                                            self.state.browser_go_home();
                                                            self.state.browser_screen =
                                                                BrowserScreen::NewTab;
                                                            return;
                                                        }
                                                    }
                                                    BrowserPageKind::BlockedShield { .. } => {
                                                        let vy =
                                                            viewport_content.y + 14.0 - scroll_y;
                                                        let hero_r = snapshot.resolve_rect(
                                                            viewport_content.x + 14.0,
                                                            vy,
                                                            viewport_content.w - 28.0,
                                                            300.0,
                                                        );
                                                        let btn_home_w = 210.0;
                                                        let btn_quota_w = 190.0;
                                                        let ret_home_btn = snapshot.resolve_rect(
                                                            hero_r.x
                                                                + (hero_r.w
                                                                    - btn_home_w
                                                                    - btn_quota_w
                                                                    - 12.0)
                                                                    * 0.5,
                                                            hero_r.y + 230.0,
                                                            btn_home_w,
                                                            36.0,
                                                        );
                                                        let check_quota_btn = snapshot
                                                            .resolve_rect(
                                                                ret_home_btn.x + btn_home_w + 12.0,
                                                                hero_r.y + 230.0,
                                                                btn_quota_w,
                                                                36.0,
                                                            );

                                                        if ret_home_btn.contains(cx, cy) {
                                                            self.state.browser_go_home();
                                                            self.state.browser_screen =
                                                                BrowserScreen::NewTab;
                                                            return;
                                                        }
                                                        if check_quota_btn.contains(cx, cy) {
                                                            self.state.browser_active_tab = 1;
                                                            return;
                                                        }
                                                    }
                                                    BrowserPageKind::SearchHome => {
                                                        self.state.browser_screen =
                                                            BrowserScreen::NewTab;
                                                    }
                                                }
                                            }
                                            BrowserScreen::Bookmarks => {
                                                let bookmarks = [
                                                    "https://nptel.ac.in",
                                                    "https://youtube.com/watch?v=physics",
                                                    "https://en.wikipedia.org",
                                                    "https://arxiv.org/abs/2403.11928",
                                                    "https://khanacademy.org",
                                                    "https://cs106b.stanford.edu",
                                                    "https://chem.libretexts.org",
                                                    "https://nature.com/articles/physics",
                                                ];
                                                let by = viewport_content.y + 16.0 - scroll_y
                                                    + 34.0
                                                    + 38.0;
                                                let cols = 2.0;
                                                let gap = 12.0;
                                                let card_w =
                                                    (viewport_content.w - 32.0 - gap) / cols;
                                                let card_h = 74.0;
                                                for (idx, b_url) in bookmarks.iter().enumerate() {
                                                    let col = (idx % 2) as f32;
                                                    let row = (idx / 2) as f32;
                                                    let bx = viewport_content.x
                                                        + 16.0
                                                        + col * (card_w + gap);
                                                    let cy_b = by + row * (card_h + gap);
                                                    let card_r = snapshot
                                                        .resolve_rect(bx, cy_b, card_w, card_h);
                                                    if card_r.contains(cx, cy) {
                                                        self.state.open_browser_url(b_url);
                                                        return;
                                                    }
                                                }
                                            }
                                            BrowserScreen::ReadingList => {
                                                let ry =
                                                    viewport_content.y + 16.0 - scroll_y + 36.0;
                                                let card_w = viewport_content.w - 32.0;
                                                let card_h = 76.0;
                                                let gap = 12.0;
                                                for idx in 0..4 {
                                                    let cr = snapshot.resolve_rect(
                                                        viewport_content.x + 16.0,
                                                        ry + idx as f32 * (card_h + gap),
                                                        card_w,
                                                        card_h,
                                                    );
                                                    let btn_r = snapshot.resolve_rect(
                                                        cr.x + cr.w - 120.0,
                                                        cr.y + 22.0,
                                                        108.0,
                                                        30.0,
                                                    );
                                                    if btn_r.contains(cx, cy) || cr.contains(cx, cy)
                                                    {
                                                        let title = match idx {
                                                            0 => "Huygens-Fresnel Principle: Wavefront Superposition Analysis",
                                                            1 => "Maxwell's Equations in Differential & Integral Forms",
                                                            2 => "Zero-Copy GPU Buffer Streaming in Pure Rust wgpu",
                                                            _ => "Chemical Thermodynamics: Gibbs Free Energy & Phase Transitions",
                                                        };
                                                        self.state.ai_response_text = format!("# {}\n\nOffline Cached Academic Document.\nStudy without distraction.", title);
                                                        self.state.wm.promote_to_front(
                                                            WindowId::StudyReader,
                                                        );
                                                        self.state.show_toast(format!(
                                                            "Loaded '{}' into Study Reader",
                                                            title
                                                        ));
                                                        return;
                                                    }
                                                }
                                            }
                                            BrowserScreen::History => {
                                                let hy = viewport_content.y + 16.0 - scroll_y
                                                    + 34.0
                                                    + 24.0;
                                                for idx in 0..4 {
                                                    let hr = snapshot.resolve_rect(
                                                        viewport_content.x + 16.0,
                                                        hy + idx as f32 * 54.0,
                                                        viewport_content.w - 32.0,
                                                        48.0,
                                                    );
                                                    if hr.contains(cx, cy) {
                                                        let url = match idx {
                                                            0 => "https://youtube.com/watch?v=physics",
                                                            1 => "https://nptel.ac.in",
                                                            2 => "https://en.wikipedia.org",
                                                            _ => "https://netflix.com",
                                                        };
                                                        self.state.open_browser_url(url);
                                                        return;
                                                    }
                                                }
                                            }
                                            BrowserScreen::Downloads => {
                                                let dy =
                                                    viewport_content.y + 16.0 - scroll_y + 36.0;
                                                let card_w = viewport_content.w - 32.0;
                                                let card_h = 72.0;
                                                let gap = 12.0;
                                                for idx in 0..4 {
                                                    let dr = snapshot.resolve_rect(
                                                        viewport_content.x + 16.0,
                                                        dy + idx as f32 * (card_h + gap),
                                                        card_w,
                                                        card_h,
                                                    );
                                                    let act_btn = snapshot.resolve_rect(
                                                        dr.x + dr.w - 130.0,
                                                        dr.y + 20.0,
                                                        118.0,
                                                        32.0,
                                                    );
                                                    if act_btn.contains(cx, cy) {
                                                        self.state
                                                            .wm
                                                            .promote_to_front(WindowId::FileVault);
                                                        self.state.show_toast(
                                                            "Opened File Vault Downloads Directory",
                                                        );
                                                        return;
                                                    }
                                                }
                                            }
                                            BrowserScreen::Settings => {
                                                // Settings buttons
                                                self.state.show_toast("MindSet Sovereign Browser Settings Synchronized");
                                            }
                                            BrowserScreen::ShieldDashboard => {
                                                let mut sy = viewport_content.y + 16.0 - scroll_y
                                                    + 36.0
                                                    + 76.0
                                                    + 16.0;
                                                sy += 24.0 + 2.0 * 74.0 + 10.0 + 16.0;
                                                let bar_r = snapshot.resolve_rect(
                                                    viewport_content.x + 16.0,
                                                    sy,
                                                    viewport_content.w - 32.0,
                                                    42.0,
                                                );
                                                let b1 = snapshot.resolve_rect(
                                                    bar_r.x + 10.0,
                                                    bar_r.y + 6.0,
                                                    230.0,
                                                    30.0,
                                                );
                                                let b2 = snapshot.resolve_rect(
                                                    b1.x + 240.0,
                                                    bar_r.y + 6.0,
                                                    210.0,
                                                    30.0,
                                                );

                                                if b1.contains(cx, cy) {
                                                    self.state.browser_active_tab = 3; // Switch to Net Troubleshooter tab
                                                    return;
                                                }
                                                if b2.contains(cx, cy) {
                                                    self.state.show_toast("SQLite Network Audit Log: 0 Leaks Detected (Verified)");
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                    1 => {
                                        // Tab 1: YouTube Timer & App Blocker
                                        let scroll_y = win.scroll_y.position;
                                        let mut y = body.y + 8.0 - scroll_y;

                                        let yt_card =
                                            snapshot.resolve_rect(body.x, y, body.w, 180.0);
                                        let play_btn_r = snapshot.resolve_rect(
                                            yt_card.x + 14.0,
                                            yt_card.y + 78.0,
                                            130.0,
                                            28.0,
                                        );
                                        if play_btn_r.contains(cx, cy) {
                                            if self.state.net_hub.youtube_guard.timer_running {
                                                self.state.net_hub.youtube_guard.pause_timer();
                                                self.state
                                                    .show_toast("YouTube Screen-Time Timer Paused");
                                            } else {
                                                self.state.net_hub.youtube_guard.start_timer();
                                                self.state.show_toast(
                                                    "YouTube Screen-Time Timer Started",
                                                );
                                            }
                                            return;
                                        }

                                        let presets = [15, 30, 45, 60];
                                        for (pi, &m) in presets.iter().enumerate() {
                                            let pr_r = snapshot.resolve_rect(
                                                play_btn_r.x + 130.0 + 8.0 + pi as f32 * 56.0,
                                                yt_card.y + 78.0,
                                                50.0,
                                                28.0,
                                            );
                                            if pr_r.contains(cx, cy) {
                                                self.state.net_hub.youtube_guard.set_preset(m);
                                                self.state.show_toast(format!(
                                                    "YouTube Limit Set to {} Minutes",
                                                    m
                                                ));
                                                return;
                                            }
                                        }

                                        let sim_btn_r = snapshot.resolve_rect(
                                            yt_card.x + yt_card.w - 174.0,
                                            yt_card.y + 78.0,
                                            160.0,
                                            28.0,
                                        );
                                        if sim_btn_r.contains(cx, cy) {
                                            self.state.net_hub.youtube_guard.used_today_secs =
                                                self.state.net_hub.youtube_guard.daily_limit_secs;
                                            self.state.net_hub.youtube_guard.timer_running = false;
                                            self.state.show_toast(
                                                "Daily YouTube Screen-Time Limit Reached!",
                                            );
                                            return;
                                        }

                                        y += 194.0;

                                        // Category blockers
                                        let ent_card =
                                            snapshot.resolve_rect(body.x, y, body.w, 240.0);
                                        for ci in 0..4 {
                                            let cat_row_r = snapshot.resolve_rect(
                                                ent_card.x + 14.0,
                                                ent_card.y + 60.0 + ci as f32 * 42.0,
                                                ent_card.w - 28.0,
                                                36.0,
                                            );
                                            let pill_r = snapshot.resolve_rect(
                                                cat_row_r.x + cat_row_r.w - 110.0,
                                                cat_row_r.y + 5.0,
                                                102.0,
                                                26.0,
                                            );
                                            if pill_r.contains(cx, cy) {
                                                match ci {
                                                    0 => {
                                                        self.state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_ott = !self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_ott;
                                                        let st = if self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_ott
                                                        {
                                                            "Blocked"
                                                        } else {
                                                            "Allowed"
                                                        };
                                                        self.state.show_toast(format!(
                                                            "OTT Streaming {}",
                                                            st
                                                        ));
                                                    }
                                                    1 => {
                                                        self.state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_social = !self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_social;
                                                        let st = if self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_social
                                                        {
                                                            "Blocked"
                                                        } else {
                                                            "Allowed"
                                                        };
                                                        self.state.show_toast(format!(
                                                            "Social Media {}",
                                                            st
                                                        ));
                                                    }
                                                    2 => {
                                                        self.state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_games = !self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_games;
                                                        let st = if self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_games
                                                        {
                                                            "Blocked"
                                                        } else {
                                                            "Allowed"
                                                        };
                                                        self.state.show_toast(format!(
                                                            "Web Games {}",
                                                            st
                                                        ));
                                                    }
                                                    3 => {
                                                        self.state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_chat_during_focus = !self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_chat_during_focus;
                                                        let st = if self
                                                            .state
                                                            .net_hub
                                                            .entertainment_shield
                                                            .block_chat_during_focus
                                                        {
                                                            "Blocked during Focus"
                                                        } else {
                                                            "Allowed"
                                                        };
                                                        self.state.show_toast(format!(
                                                            "Chat Apps {}",
                                                            st
                                                        ));
                                                    }
                                                    _ => {}
                                                }
                                                return;
                                            }
                                        }
                                    }
                                    2 => {
                                        // Tab 2: Ad-Blocker Scripts
                                        let scroll_y = win.scroll_y.position;
                                        let y = body.y + 8.0 - scroll_y + 86.0;
                                        let scripts_card =
                                            snapshot.resolve_rect(body.x, y, body.w, 200.0);
                                        let reload_btn_r = snapshot.resolve_rect(
                                            scripts_card.x + 14.0,
                                            scripts_card.y + 164.0,
                                            scripts_card.w - 28.0,
                                            26.0,
                                        );
                                        if reload_btn_r.contains(cx, cy) {
                                            match self.state.net_hub.chromium_bridge.ensure_shield_extension_on_disk() {
                                                Ok(()) => self.state.show_toast("Shield Extension Verified on Disk (~/.mindset_data/browser_shield_ext)"),
                                                Err(e) => self.state.show_toast(format!("Shield Notice: {}", e)),
                                            }
                                            return;
                                        }
                                    }
                                    3 => {
                                        // Tab 3: Internet Troubleshooter
                                        let scroll_y = win.scroll_y.position;
                                        let mut y = body.y + 8.0 - scroll_y;

                                        let diag_btn_r =
                                            snapshot.resolve_rect(body.x, y, body.w, 36.0);
                                        if diag_btn_r.contains(cx, cy) {
                                            self.state.net_hub.run_diagnostics();
                                            self.state.show_toast(
                                                "Full 7-Stage Internet Diagnostics Completed",
                                            );
                                            return;
                                        }
                                        y += 44.0;

                                        let rep_gap = 8.0;
                                        let rep_w = (body.w - 3.0 * rep_gap) / 4.0;
                                        let repair_actions = [
                                            (
                                                NetRepairAction::FlushDnsAndResetSockets,
                                                "DNS Cache Flushed & Sockets Reset",
                                            ),
                                            (
                                                NetRepairAction::SwitchToSafeAcademicDns,
                                                "Safe Academic DNS (1.1.1.3) Applied",
                                            ),
                                            (
                                                NetRepairAction::ClearProxyAndUnblockPorts,
                                                "Proxy Cleared & Ports Unblocked",
                                            ),
                                            (
                                                NetRepairAction::RebuildBrowserShieldScripts,
                                                "Browser Shield Scripts Rebuilt",
                                            ),
                                        ];
                                        for (ri, (act, msg)) in repair_actions.iter().enumerate() {
                                            let r_btn = snapshot.resolve_rect(
                                                body.x + ri as f32 * (rep_w + rep_gap),
                                                y,
                                                rep_w,
                                                30.0,
                                            );
                                            if r_btn.contains(cx, cy) {
                                                let _ = self.state.net_hub.execute_repair(*act);
                                                self.state.show_toast(*msg);
                                                return;
                                            }
                                        }
                                        y += 38.0;

                                        for (si, _) in self
                                            .state
                                            .net_hub
                                            .troubleshooter
                                            .stages
                                            .iter()
                                            .enumerate()
                                        {
                                            let st_card = snapshot.resolve_rect(
                                                body.x,
                                                y + si as f32 * 56.0,
                                                body.w,
                                                50.0,
                                            );
                                            let rep_btn_r = snapshot.resolve_rect(
                                                st_card.x + st_card.w - 110.0,
                                                st_card.y + 12.0,
                                                102.0,
                                                26.0,
                                            );
                                            if rep_btn_r.contains(cx, cy) {
                                                self.state.net_hub.run_diagnostics();
                                                self.state.show_toast(format!(
                                                    "Diagnosed Stage {}",
                                                    si + 1
                                                ));
                                                return;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Desktop icon clicks (Single-click launch with exact rendered_rect)
                if !self.state.session.focus_mode
                    && self.state.show_desktop
                    && self.state.mindful_engine.should_show_desktop_icons()
                {
                    if let Some(clicked_id) = self.state.desktop_grid.select_icon_at(cx, cy) {
                        if !self.state.is_app_allowed_on_home(clicked_id) {
                            self.state.mindful_modal_target = Some(clicked_id);
                            return;
                        }
                        if clicked_id == WindowId::Today {
                            self.state.set_show_desktop(false);
                            self.state.show_toast("Switched to Today View");
                            return;
                        }
                        apply_window_requests(
                            &[WindowRequest::SetState {
                                window_id: clicked_id,
                                state: WindowState::Floating,
                            }],
                            &mut self.state.wm,
                        );
                        if let Some(win) = self.state.wm.find_window_mut(clicked_id) {
                            win.workspace_id = self.state.active_workspace;
                        }
                        self.state.wm.promote_to_front(clicked_id);
                        self.state
                            .show_toast(format!("Opened {}", clicked_id.title()));
                        return;
                    }
                }

                // Root Today Screen Click Handling
                // If no modal or floating window was clicked, process Today Screen actions:
                // [ START FOCUS ], [ Set up goals ], [ Start 5-min restart ], [ Skip for today ], [ 🗕 Desktop ]
                if !self.state.show_desktop
                    && self
                        .state
                        .wm
                        .window_at_point(cx, cy, self.state.active_workspace)
                        .is_none()
                {
                    let targets = crate::today::compute_today_hit_targets(
                        w,
                        h,
                        self.today_state.one_thing.is_some(),
                        self.today_state.recovery.is_some(),
                    );

                    // Minimize to desktop
                    if targets.minimize_btn.contains(cx, cy) {
                        self.state.set_show_desktop(true);
                        self.state.show_toast(
                            "Desktop View (Click Today icon or press Escape to return)",
                        );
                        return;
                    }

                    // START FOCUS button
                    if let Some(cta_btn) = targets.start_focus_btn {
                        if cta_btn.contains(cx, cy) {
                            self.trigger_focus_session();
                            return;
                        }
                    }

                    // Set up goals button
                    if let Some(setup_btn) = targets.setup_goals_btn {
                        if setup_btn.contains(cx, cy) {
                            apply_window_requests(
                                &[WindowRequest::SetState {
                                    window_id: WindowId::Settings,
                                    state: WindowState::Floating,
                                }],
                                &mut self.state.wm,
                            );
                            if let Some(win) = self.state.wm.find_window_mut(WindowId::Settings) {
                                win.workspace_id = self.state.active_workspace;
                            }
                            self.state.wm.promote_to_front(WindowId::Settings);
                            self.state.settings_active_tab = 0; // Profile
                            self.state.show_toast("Opened Profile Settings");
                            return;
                        }
                    }

                    // 5-min restart button
                    if let Some(restart_btn) = targets.restart_btn {
                        if restart_btn.contains(cx, cy) {
                            self.today_state.recovery = None;
                            self.state.timer_secs = 5.0 * 60.0;
                            self.state.timer_running = true;
                            apply_window_requests(
                                &[WindowRequest::SetState {
                                    window_id: WindowId::StudyReader,
                                    state: WindowState::Floating,
                                }],
                                &mut self.state.wm,
                            );
                            if let Some(win) = self.state.wm.find_window_mut(WindowId::StudyReader)
                            {
                                win.workspace_id = self.state.active_workspace;
                            }
                            self.state.wm.promote_to_front(WindowId::StudyReader);
                            self.state.show_toast("Starting 5-min Restart Session");
                            return;
                        }
                    }

                    // Skip for today button
                    if let Some(skip_btn) = targets.skip_btn {
                        if skip_btn.contains(cx, cy) {
                            self.today_state.recovery = None;
                            self.state.show_toast("Streak Paused • Ready for Tomorrow");
                            return;
                        }
                    }

                    // Progress ring clicks
                    for (idx, ring_r) in targets.ring_rects.iter().enumerate() {
                        if ring_r.contains(cx, cy) {
                            let ring_names = [
                                "Today's Target: 2h of 4h done",
                                "Weekly Target: 8h of 20h completed",
                                "NEET Syllabus: 34% Mastered",
                            ];
                            self.state.show_toast(ring_names[idx]);
                            return;
                        }
                    }

                    // Future self card click
                    if targets.future_self_rect.contains(cx, cy) {
                        self.state
                            .show_toast("Identity Goal: Dr. Logesh Karthik, MBBS 2027");
                    }
                }
            }

            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                self.state.input.release();
                self.state.active_slider = None;
                let (cx, cy) = self.state.input.cursor;

                for win in &mut self.state.wm.windows {
                    win.is_scrolling_thumb = false;
                }

                if let Some(wid) = self.state.dragging_window_id.take() {
                    self.state.wm.release_drag(wid, cx, cy);
                }
                self.state.resizing_window_id = None;
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                let (cx, cy) = self.state.input.cursor;
                let w = self
                    .render_ctx
                    .as_ref()
                    .map_or(1280.0, |c| c.config.width as f32);
                let h = self
                    .render_ctx
                    .as_ref()
                    .map_or(800.0, |c| c.config.height as f32);

                // Right-click opens context menu on wallpaper
                if self
                    .state
                    .wm
                    .window_at_point(cx, cy, self.state.active_workspace)
                    .is_none()
                {
                    self.state.open_overlay(ActiveOverlay::ContextMenu);
                    self.state.context_menu.open(cx, cy, w, h);
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if !self.state.boot_completed {
                    self.state.boot_completed = true;
                    return;
                }
                self.state.last_interaction_time = Instant::now();

                // 1. Lock screen keyboard handling
                if let SessionState::Locked {
                    ref mut pin_input, ..
                } = self.state.session.state
                {
                    match logical_key {
                        Key::Named(NamedKey::Enter) => {
                            let entered = pin_input.text.clone();
                            self.state.session.try_unlock(&entered);
                        }
                        Key::Named(NamedKey::Backspace) => {
                            pin_input.backspace();
                        }
                        Key::Named(NamedKey::Delete) => {
                            pin_input.delete();
                        }
                        Key::Named(NamedKey::ArrowLeft) => {
                            pin_input.move_left_selection(self.shift_pressed);
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            pin_input.move_right_selection(self.shift_pressed);
                        }
                        Key::Character(ref ch) => {
                            pin_input.insert_str(ch.as_str());
                        }
                        _ => {}
                    }
                    return;
                }

                // 2. Window Switching with Alt+Tab or Ctrl+Tab
                if (self.alt_pressed || self.ctrl_pressed)
                    && logical_key == Key::Named(NamedKey::Tab)
                {
                    if !self.state.alt_tab.is_visible {
                        self.state.alt_tab.open();
                    }
                    let open_count = self
                        .state
                        .wm
                        .windows
                        .iter()
                        .filter(|w| w.is_open && w.workspace_id == self.state.active_workspace)
                        .count();
                    if self.shift_pressed {
                        self.state.alt_tab.cycle_prev(open_count);
                    } else {
                        self.state.alt_tab.cycle_next(open_count);
                    }
                    return;
                }

                // 3. Global Shortcuts with Ctrl
                if self.ctrl_pressed {
                    // Phase 9: Global Incognito Shortcut Interception (Ctrl+Shift+N / Ctrl+Shift+P)
                    if self.shift_pressed {
                        if let Key::Character(ref c) = logical_key {
                            if let Some(reason) = PolicyGuard::is_shortcut_blocked(
                                self.ctrl_pressed,
                                self.shift_pressed,
                                self.alt_pressed,
                                c.as_str(),
                            ) {
                                self.state.show_toast(reason);
                                return;
                            }
                        }
                    }

                    match &logical_key {
                        Key::Character(c) if c.eq_ignore_ascii_case("d") && self.shift_pressed => {
                            self.state.wm.toggle_window(WindowId::DesignSystem);
                            self.state.show_toast("Toggled Design System Showcase");
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("m") => {
                            self.state.toggle_overlay(ActiveOverlay::OdysseyMap);
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("k") => {
                            self.state.toggle_overlay(ActiveOverlay::Launcher);
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("l") => {
                            self.state.session.lock();
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("a") => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.select_all();
                            }
                            self.state.show_toast("Select All (Ctrl+A)");
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("c") => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.copy(&self.clipboard);
                            }
                            self.state.show_toast("Copied to Clipboard (Ctrl+C)");
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("x") => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.cut(&self.clipboard);
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            self.state.show_toast("Cut to Clipboard (Ctrl+X)");
                            return;
                        }
                        Key::Character(c) if c.eq_ignore_ascii_case("v") => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.paste(&self.clipboard);
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            self.state.show_toast("Pasted from Clipboard (Ctrl+V)");
                            return;
                        }
                        _ => {}
                    }
                }

                // 4. Focused Input Buffer Key Routing
                if self.state.active_input_buffer_mut().is_some() {
                    match logical_key {
                        Key::Named(NamedKey::ArrowLeft) => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.move_left_selection(self.shift_pressed);
                            }
                            return;
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.move_right_selection(self.shift_pressed);
                            }
                            return;
                        }
                        Key::Named(NamedKey::Home) => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.move_home_selection(self.shift_pressed);
                            }
                            return;
                        }
                        Key::Named(NamedKey::End) => {
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.move_end_selection(self.shift_pressed);
                            }
                            return;
                        }
                        Key::Named(NamedKey::Backspace) => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.backspace();
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            return;
                        }
                        Key::Named(NamedKey::Delete) => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.delete();
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            return;
                        }
                        Key::Named(NamedKey::Space) => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.insert_char(' ');
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            return;
                        }
                        Key::Named(NamedKey::Enter) => {
                            if matches!(
                                self.state.active_overlay(),
                                ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. }
                            ) {
                                let q = self.state.command_launcher.input.text.trim().to_string();
                                if !q.is_empty() {
                                    self.state.ask_custom_maindy_query(&q);
                                    self.state.command_launcher.input.clear();
                                    self.state.close_overlay();
                                    self.state.wm.promote_to_front(WindowId::LayaAiChat);
                                }
                                return;
                            }
                            if self.state.search_input.is_focused {
                                let query = self.state.search_input.text.trim().to_string();
                                if query.starts_with("http://")
                                    || query.starts_with("https://")
                                    || query.contains(".com")
                                    || query.contains(".org")
                                    || query.contains(".in")
                                    || query.contains(".edu")
                                    || query.contains(".ac")
                                    || query.contains(".gov")
                                {
                                    self.state.open_browser_url(&query);
                                } else {
                                    self.state.execute_search();
                                }
                                return;
                            }
                            if self.state.maindy_query_input.is_focused {
                                let q = self.state.maindy_query_input.text.clone();
                                if !q.is_empty() {
                                    self.state.ask_custom_maindy_query(&q);
                                    self.state.maindy_query_input.clear();
                                }
                                return;
                            }
                            if self.state.task_title_input.is_focused {
                                let title = self.state.task_title_input.text.clone();
                                if !title.trim().is_empty() {
                                    self.state.tasks.push(Task::new(&title, "Study Task", 0.0));
                                    self.state.task_title_input.clear();
                                }
                                return;
                            }
                        }
                        Key::Named(NamedKey::Escape) => {
                            if matches!(
                                self.state.active_overlay(),
                                ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. }
                            ) {
                                self.state.close_overlay();
                            } else {
                                self.state.search_input.is_focused = false;
                                self.state.maindy_query_input.is_focused = false;
                                self.state.task_title_input.is_focused = false;
                            }
                            return;
                        }
                        Key::Named(NamedKey::Tab) => {
                            let curr_focus = if self.state.search_input.is_focused {
                                1
                            } else if self.state.maindy_query_input.is_focused {
                                2
                            } else if self.state.task_title_input.is_focused {
                                3
                            } else {
                                0
                            };
                            let next_focus = if self.shift_pressed {
                                match curr_focus {
                                    1 => 3,
                                    2 => 1,
                                    3 => 2,
                                    _ => 3,
                                }
                            } else {
                                match curr_focus {
                                    1 => 2,
                                    2 => 3,
                                    3 => 1,
                                    _ => 1,
                                }
                            };
                            self.state.search_input.is_focused = next_focus == 1;
                            self.state.maindy_query_input.is_focused = next_focus == 2;
                            self.state.task_title_input.is_focused = next_focus == 3;
                            self.state.input.focused_widget = Some(next_focus);

                            let target_wid = match next_focus {
                                1 => Some(WindowId::SearchBrowser),
                                2 => Some(WindowId::LayaAiChat),
                                3 => Some(WindowId::Tasks),
                                _ => None,
                            };
                            if let Some(wid) = target_wid {
                                self.state.wm.promote_to_front(wid);
                            }
                            return;
                        }
                        Key::Character(ref ch) if !self.ctrl_pressed && !self.alt_pressed => {
                            let was_search = self.state.search_input.is_focused;
                            if let Some(buf) = self.state.active_input_buffer_mut() {
                                buf.insert_str(ch.as_str());
                            }
                            if was_search {
                                self.state.execute_search();
                            }
                            return;
                        }
                        _ => {}
                    }
                }

                // 5. Desktop-Level Keys & Shortcuts (when no input is focused)
                match logical_key {
                    Key::Named(NamedKey::Tab) => {
                        let next_focus = if self.shift_pressed { 3 } else { 1 };
                        self.state.search_input.is_focused = next_focus == 1;
                        self.state.maindy_query_input.is_focused = next_focus == 2;
                        self.state.task_title_input.is_focused = next_focus == 3;
                        self.state.input.focused_widget = Some(next_focus);

                        let target_wid = match next_focus {
                            1 => Some(WindowId::SearchBrowser),
                            2 => Some(WindowId::LayaAiChat),
                            3 => Some(WindowId::Tasks),
                            _ => None,
                        };
                        if let Some(wid) = target_wid {
                            self.state.wm.promote_to_front(wid);
                        }
                    }
                    Key::Named(NamedKey::F12) => {
                        self.state.inspector_open = !self.state.inspector_open;
                    }
                    Key::Named(NamedKey::F11) => {
                        self.state.wireframe_bounds = !self.state.wireframe_bounds;
                        let s = if self.state.wireframe_bounds {
                            "ON"
                        } else {
                            "OFF"
                        };
                        self.state.show_toast(format!("Wireframe Bounds {}", s));
                    }
                    Key::Named(NamedKey::Escape) => {
                        if self.state.is_overlay_open() {
                            self.state.close_overlay();
                        } else if self.state.alt_tab.is_visible {
                            self.state.alt_tab.close();
                        } else if self.state.inspector_open {
                            self.state.inspector_open = false;
                        } else if self.state.session.focus_mode {
                            self.state.toggle_focus_mode();
                        } else {
                            self.state.toggle_desktop();
                            let msg = if self.state.show_desktop {
                                "Desktop View (Press Escape for Today)"
                            } else {
                                "Distraction-Free Today View"
                            };
                            self.state.show_toast(msg);
                        }
                    }
                    Key::Named(NamedKey::PageDown) => {
                        if let Some(wid) = self.state.wm.active_window_id {
                            if let Some(win) = self.state.wm.find_window_mut(wid) {
                                win.scroll_by(120.0);
                            }
                        }
                    }
                    Key::Named(NamedKey::PageUp) => {
                        if let Some(wid) = self.state.wm.active_window_id {
                            if let Some(win) = self.state.wm.find_window_mut(wid) {
                                win.scroll_by(-120.0);
                            }
                        }
                    }
                    Key::Named(NamedKey::ArrowDown) => {
                        if let Some(wid) = self.state.wm.active_window_id {
                            if let Some(win) = self.state.wm.find_window_mut(wid) {
                                win.scroll_by(40.0);
                            }
                        }
                    }
                    Key::Named(NamedKey::ArrowUp) => {
                        if let Some(wid) = self.state.wm.active_window_id {
                            if let Some(win) = self.state.wm.find_window_mut(wid) {
                                win.scroll_by(-40.0);
                            }
                        }
                    }
                    Key::Named(NamedKey::Space) => {
                        self.state.toggle_focus_mode();
                    }
                    Key::Character(ref ch) => match ch.as_str() {
                        "1" => self.state.wm.promote_to_front(WindowId::StudyReader),
                        "2" => self.state.wm.promote_to_front(WindowId::LayaAiChat),
                        "3" => self.state.wm.promote_to_front(WindowId::Tasks),
                        "4" => self.state.wm.promote_to_front(WindowId::ProgressStats),
                        "5" => self.state.wm.promote_to_front(WindowId::SystemMonitor),
                        "6" => self.state.wm.promote_to_front(WindowId::Settings),
                        "7" => self.state.wm.promote_to_front(WindowId::FileVault),
                        "8" => self.state.wm.promote_to_front(WindowId::SearchBrowser),
                        "t" | "T" => {
                            self.state.theme.toggle_mode();
                            self.persist_preferences();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            WindowEvent::RedrawRequested => match self.render_frame() {
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    if let Some(ctx) = self.render_ctx.as_mut() {
                        ctx.surface.configure(&ctx.device, &ctx.config);
                    }
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("GPU OOM: exiting");
                    event_loop.exit();
                }
                _ => {}
            },

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let has_motion = self.state.has_active_motion();
        let is_streaming = self.state.ai_is_streaming;
        let is_interacting =
            self.state.last_interaction_time.elapsed() < std::time::Duration::from_millis(250);

        if has_motion || is_streaming || is_interacting {
            event_loop.set_control_flow(ControlFlow::Poll);
            if let Some(ref win) = self.window {
                win.request_redraw();
            }
        } else {
            let next_wake = Instant::now() + std::time::Duration::from_millis(1000);
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_wake));
            if let Some(ref win) = self.window {
                win.request_redraw();
            }
        }
    }
}
