//! MindSet OS Core Application State and Lifecycle Manager

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Instant;
use sysinfo::{Pid, ProcessesToUpdate, System};
use winit::window::Window;

use mindset_material::{DualKawasePipeline, MaterialTargetManager};
use mindset_render::{BackgroundPipeline, RenderContext, SdfPipeline, TextRenderer};

use mindset_surface::wm::WindowState;
use mindset_sys::db::DatabaseManager;
use mindset_sys::storage::MindSetPaths;

use crate::control_panel::ModernControlPanelState;
use crate::state::{
    ActiveOverlay, PersonalizationStore, SessionState, ShellState, SystemClipboard, ThemeMode,
    WallpaperPreset, WindowId,
};
use crate::widgets;

pub struct MindSetApp {
    pub window: Option<Arc<Window>>,
    pub render_ctx: Option<RenderContext>,
    pub target_mgr: Option<MaterialTargetManager>,
    pub bg_pipeline: Option<BackgroundPipeline>,
    pub kawase_pipeline: Option<DualKawasePipeline>,
    pub sdf_pipeline: Option<SdfPipeline>,
    pub text_renderer: Option<TextRenderer>,

    pub db_mgr: Option<DatabaseManager>,
    pub sysinfo: System,
    pub current_pid: Pid,

    pub start_time: Instant,
    pub last_frame: Instant,
    pub frame_count: u64,
    pub current_fps: f32,
    pub frame_time_ms: f32,

    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
    pub shift_pressed: bool,
    pub clipboard: SystemClipboard,
    pub state: ShellState,

    pub telemetry_rx: Receiver<widgets::TelemetrySnapshot>,
    pub telemetry_snapshot: widgets::TelemetrySnapshot,
    pub subsystems: Vec<widgets::DiagnosticRow>,
    pub control_center_state: widgets::ControlCenterState,
    pub modern_control_panel: ModernControlPanelState,
    pub dock_items: Vec<widgets::DockIconItem>,
    pub dock_hovered_idx: Option<usize>,
    pub today_state: crate::today::TodayState,
}

impl Default for MindSetApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MindSetApp {
    pub fn new() -> Self {
        let sys = System::new_all();
        let pid = Pid::from_u32(std::process::id());

        let (
            db_mgr,
            initial_score,
            saved_theme_dark,
            saved_warmth,
            saved_font,
            saved_wallpaper,
            saved_p_store,
        ) = Self::load_persistence();

        let mut state = ShellState::with_wallpaper(
            1280.0,
            800.0,
            saved_theme_dark,
            saved_warmth,
            saved_font,
            initial_score,
            saved_wallpaper,
        );
        state.personalization_store = saved_p_store;
        state.active_academic_stage = state.personalization_store.profile.academic_stage;
        state.active_study_target_hours =
            state.personalization_store.profile.study_target_hours_idx;
        state.glass_blur_quality = state.personalization_store.appearance.blur_quality;
        state.workspaces_enabled = state.personalization_store.desktop.workspaces_enabled;
        state.widgets_enabled = state.personalization_store.desktop.widgets_enabled;

        // Boot into completely stripped-down, distraction-free Today view
        state.wallpaper_preset = WallpaperPreset::AlpineLake;
        state.close_overlay();
        state.show_desktop = false;
        state.wm.promote_to_front(WindowId::Today);

        // Async background worker (tokio/thread + sysinfo) polling every 1000ms
        let (telem_tx, telem_rx) = std::sync::mpsc::channel();
        let pid_num = std::process::id();
        std::thread::spawn(move || {
            let mut bg_sys = System::new_all();
            let bg_pid = Pid::from_u32(pid_num);
            loop {
                std::thread::sleep(std::time::Duration::from_millis(1000));
                bg_sys.refresh_processes(ProcessesToUpdate::Some(&[bg_pid]), true);
                bg_sys.refresh_memory();
                let rss = bg_sys
                    .process(bg_pid)
                    .map(|p| p.memory() as f32 / (1024.0 * 1024.0))
                    .unwrap_or(28.4);
                let free = (bg_sys.available_memory() / (1024 * 1024)) as f32;

                let port_online = std::net::TcpStream::connect_timeout(
                    &std::net::SocketAddr::from(([127, 0, 0, 1], 11434)),
                    std::time::Duration::from_millis(50),
                )
                .is_ok();

                let snapshot = widgets::TelemetrySnapshot {
                    rss_mb: rss,
                    free_ram_mb: free,
                    fps: 60,
                    frame_time_ms: 16.6,
                    health_percent: 99.4,
                    is_full_repairing: false,
                    port_11434_online: port_online,
                };
                if telem_tx.send(snapshot).is_err() {
                    break;
                }
            }
        });

        let telemetry_snapshot = widgets::TelemetrySnapshot::default();
        let subsystems = widgets::DiagnosticRow::default_rows();
        let control_center_state = widgets::ControlCenterState::default();
        let modern_control_panel = ModernControlPanelState::default();
        let dock_items = widgets::DockIconItem::default_items();

        Self {
            window: None,
            render_ctx: None,
            target_mgr: None,
            bg_pipeline: None,
            kawase_pipeline: None,
            sdf_pipeline: None,
            text_renderer: None,

            db_mgr,
            sysinfo: sys,
            current_pid: pid,

            start_time: Instant::now(),
            last_frame: Instant::now(),
            frame_count: 0,
            current_fps: 120.0,
            frame_time_ms: 8.33,

            ctrl_pressed: false,
            alt_pressed: false,
            shift_pressed: false,
            clipboard: SystemClipboard::new(),
            state,

            telemetry_rx: telem_rx,
            telemetry_snapshot,
            subsystems,
            control_center_state,
            modern_control_panel,
            dock_items,
            dock_hovered_idx: None,
            today_state: {
                let mut ts = crate::today::TodayState::new();
                ts.refresh();
                ts
            },
        }
    }

    pub fn load_persistence() -> (
        Option<DatabaseManager>,
        i32,
        bool,
        f32,
        f32,
        u32,
        PersonalizationStore,
    ) {
        let paths = match MindSetPaths::resolve() {
            Ok(p) => p,
            Err(_) => return (None, 590, true, 0.0, 1.0, 0, PersonalizationStore::new()),
        };
        let _ = paths.initialize_filesystem();
        let db = match DatabaseManager::initialize(&paths) {
            Ok(d) => d,
            Err(_) => return (None, 590, true, 0.0, 1.0, 0, PersonalizationStore::new()),
        };

        let conn_arc = db.get_core_connection();
        let conn = match conn_arc.lock() {
            Ok(c) => c,
            Err(_) => {
                return (
                    Some(db),
                    590,
                    true,
                    0.0,
                    1.0,
                    0,
                    PersonalizationStore::new(),
                )
            }
        };

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS class12_target_score \
             (id INTEGER PRIMARY KEY, target_score INTEGER NOT NULL, max_score INTEGER NOT NULL);",
            [],
        );
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM class12_target_score WHERE id=1",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        let score = if count == 0 {
            let _ = conn.execute("INSERT INTO class12_target_score VALUES (1,590,600)", []);
            590
        } else {
            conn.query_row(
                "SELECT target_score FROM class12_target_score WHERE id=1",
                [],
                |r| r.get(0),
            )
            .unwrap_or(590)
        };

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS user_preferences \
             (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
            [],
        );
        let get_pref = |k: &str, def: &str| -> String {
            conn.query_row(
                "SELECT value FROM user_preferences WHERE key=?1",
                [k],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| def.to_owned())
        };
        let dark = get_pref("theme_mode", "dark") == "dark";
        let warmth = get_pref("night_warmth", "0.0")
            .parse::<f32>()
            .unwrap_or(0.0);
        let font_scl = get_pref("font_scale", "1.0").parse::<f32>().unwrap_or(1.0);
        let wp_style = get_pref("wallpaper_style", "0").parse::<u32>().unwrap_or(0);
        let p_store = PersonalizationStore::load_from_sqlite(&conn)
            .unwrap_or_else(|_| PersonalizationStore::new());

        drop(conn);
        (Some(db), score, dark, warmth, font_scl, wp_style, p_store)
    }

    #[allow(dead_code)]
    pub fn persist_score(&self, score: i32) {
        if let Some(ref db) = self.db_mgr {
            if let Ok(conn) = db.get_core_connection().lock() {
                let _ = conn.execute(
                    "UPDATE class12_target_score SET target_score=?1 WHERE id=1",
                    rusqlite::params![score],
                );
            }
        }
    }

    pub fn persist_preferences(&mut self) {
        if let Some(ref db) = self.db_mgr {
            if let Ok(conn) = db.get_core_connection().lock() {
                let upsert = |k: &str, v: &str| {
                    let _ = conn.execute(
                        "INSERT INTO user_preferences(key,value) VALUES(?1,?2) \
                         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                        rusqlite::params![k, v],
                    );
                };
                upsert(
                    "theme_mode",
                    if self.state.theme.mode == ThemeMode::Dark {
                        "dark"
                    } else {
                        "light"
                    },
                );
                upsert("night_warmth", &self.state.theme.night_warmth.to_string());
                upsert("font_scale", &self.state.theme.font_scale.to_string());
                upsert("ui_scale", &self.state.theme.ui_scale.to_string());
                upsert(
                    "wallpaper_style",
                    &(self.state.wallpaper_preset as u32).to_string(),
                );
                upsert(
                    "academic_stage",
                    &self.state.active_academic_stage.to_string(),
                );
                upsert(
                    "study_target_hours",
                    &self.state.active_study_target_hours.to_string(),
                );
                upsert(
                    "glass_blur_quality",
                    &self.state.glass_blur_quality.to_string(),
                );
                self.state.personalization_store.profile.academic_stage =
                    self.state.active_academic_stage;
                self.state
                    .personalization_store
                    .profile
                    .study_target_hours_idx = self.state.active_study_target_hours;
                self.state.personalization_store.appearance.blur_quality =
                    self.state.glass_blur_quality;
                self.state.personalization_store.desktop.workspaces_enabled =
                    self.state.workspaces_enabled;
                self.state.personalization_store.desktop.widgets_enabled =
                    self.state.widgets_enabled;
                let _ = self.state.personalization_store.save_to_sqlite(&conn);
            }
        }
    }

    pub fn get_telemetry(&mut self) -> (String, String) {
        self.sysinfo
            .refresh_processes(ProcessesToUpdate::Some(&[self.current_pid]), true);
        self.sysinfo.refresh_memory();
        let rss = self
            .sysinfo
            .process(self.current_pid)
            .map(|p| p.memory() as f64 / (1024.0 * 1024.0))
            .unwrap_or(24.5);
        let free = self.sysinfo.available_memory() / (1024 * 1024);
        (
            format!("RSS {:.1} MB • Free {} MB", rss, free),
            format!("{:.0} FPS • {:.2} ms", self.current_fps, self.frame_time_ms),
        )
    }

    pub fn check_interactive_hovered(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        // Top bar buttons (floating capsule islands at top)
        if y <= 52.0 {
            let capsule_tb = crate::top_bar::TopCapsuleBarLayout::calculate(w, h);
            if capsule_tb.left_capsule.contains(x, y) || capsule_tb.right_capsule.contains(x, y) {
                return true;
            }
        }
        // Today Screen buttons when in distraction-free Today view
        if !self.state.show_desktop {
            let targets = crate::today::compute_today_hit_targets(
                w,
                h,
                self.today_state.one_thing.is_some(),
                self.today_state.recovery.is_some(),
            );
            if targets.minimize_btn.contains(x, y) {
                return true;
            }
            if let Some(btn) = targets.start_focus_btn {
                if btn.contains(x, y) {
                    return true;
                }
            }
            if let Some(btn) = targets.setup_goals_btn {
                if btn.contains(x, y) {
                    return true;
                }
            }
            if let Some(btn) = targets.restart_btn {
                if btn.contains(x, y) {
                    return true;
                }
            }
            if let Some(btn) = targets.skip_btn {
                if btn.contains(x, y) {
                    return true;
                }
            }
            for rr in &targets.ring_rects {
                if rr.contains(x, y) {
                    return true;
                }
            }
        }
        // Dock buttons
        if !self.state.session.focus_mode
            && self.state.show_desktop
            && self.state.mindful_engine.should_show_bottom_dock()
            && self.state.session.dock_rect().contains(x, y)
        {
            return true;
        }
        // Desktop icons
        if !self.state.session.focus_mode
            && self.state.show_desktop
            && self.state.mindful_engine.should_show_desktop_icons()
            && self
                .state
                .desktop_grid
                .icons
                .iter()
                .any(|icon| icon.rendered_rect.contains(x, y))
        {
            return true;
        }
        // Left-side student widget cards
        let s = self.state.personalization_store.appearance.ui_scale;
        if !self.state.session.focus_mode
            && self.state.show_desktop
            && self.state.mindful_engine.should_show_left_widgets()
            && x >= 20.0 * s
            && x <= 265.0 * s
            && y >= 50.0 * s
            && y <= 420.0 * s
        {
            return true;
        }
        // Window titlebars, close/min/max buttons, resize grips, scrollbars
        if let Some(wid) = self
            .state
            .wm
            .window_at_point(x, y, self.state.active_workspace)
        {
            if let Some(win) = self.state.wm.find_window(wid) {
                if win.titlebar_rect().contains(x, y) {
                    return true;
                }
                if win.state == WindowState::Floating && win.resize_grip_rect().contains(x, y) {
                    return true;
                }
                if win.scrollbar_track_rect().contains(x, y) {
                    return true;
                }
                if win.content_rect().contains(x, y) {
                    return true;
                }
            }
        }
        // Overlays (Rule 3: Compile-Time Modal Exclusivity)
        if self.state.is_overlay_open()
            || self.state.session.state == SessionState::PowerMenuOpen
            || self.state.mindful_modal_target.is_some()
            || matches!(self.state.session.state, SessionState::Locked { .. })
        {
            return true;
        }
        false
    }

    /// Returns the currently active modal overlay.
    pub fn active_overlay(&self) -> ActiveOverlay {
        self.state.active_overlay()
    }

    /// Atomically opens the given overlay and closes all others.
    pub fn open_overlay(&mut self, overlay: ActiveOverlay) {
        self.state.open_overlay(overlay);
    }

    /// Closes any active modal overlay.
    pub fn close_overlay(&mut self) {
        self.state.close_overlay();
    }

    /// Returns true if any modal overlay is open.
    pub fn is_overlay_open(&self) -> bool {
        self.state.is_overlay_open()
    }

    /// Triggers a 25-minute focus session in StudyReader (zero-panic)
    pub fn trigger_focus_session(&mut self) {
        self.state.trigger_focus_session();
    }
}
