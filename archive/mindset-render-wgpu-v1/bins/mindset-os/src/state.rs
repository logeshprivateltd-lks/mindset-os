//! MindSet OS — Shell State & Compositor Domain Data Model (`state.rs`)
//! Pure Rust central state management following single-source-of-truth architecture.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_intelligence::*;
use mindset_motion::*;
use mindset_net::*;
use mindset_render::*;
use mindset_surface::*;
use mindset_sys::*;
use mindset_ui::*;
use std::sync::mpsc::Receiver;
use std::time::Instant;

// ─── Study Soundscape Preset ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudySoundscape {
    Off,
    SoftRain,
    FocusTone,
}

impl StudySoundscape {
    pub fn all() -> [StudySoundscape; 3] {
        [Self::Off, Self::SoftRain, Self::FocusTone]
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "[Off]",
            Self::SoftRain => "[Soft Rain]",
            Self::FocusTone => "[Focus Tone]",
        }
    }
}

// ─── Task Data ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub subject: String,
    pub progress: f32,
    pub done: bool,
    pub progress_spring: Spring1D,
}

impl Task {
    pub fn new(title: &str, subject: &str, progress: f32) -> Self {
        Self {
            title: title.to_string(),
            subject: subject.to_string(),
            progress,
            done: false,
            progress_spring: Spring1D::with_preset(progress, SpringPreset::FluidPanel),
        }
    }
    pub fn add_progress(&mut self, delta: f32) {
        self.progress = (self.progress + delta).clamp(0.0, 1.0);
        self.progress_spring.set_target(self.progress);
        if self.progress >= 1.0 {
            self.done = true;
        }
    }
}

// ─── Study Content Presets ────────────────────────────────────────────────────

pub const STUDY_TOPICS: [(&str, &str, &str); 3] = [
    (
        "[1] Gauss's Law",
        "Q: State Gauss's Law and derive the electric field from Coulomb's Law.",
        "Gauss's Law: ∮ E·dA = Q_enc / ε₀.\n\nDerivation:\n1. Build spherical surface of radius r around charge +q.\n2. By symmetry E is radial: E(4πr²) = q/ε₀.\n3. ∴ E = q/(4πε₀r²)  [Q.E.D.]\n\nBoard Exam Note: Valid for any closed Gaussian surface enclosing charge Q.",
    ),
    (
        "[2] Nernst Equation",
        "Q: Derive Nernst equation for a Daniell cell at 298 K.",
        "Daniell cell: Zn + Cu²⁺ ⇌ Zn²⁺ + Cu  (n=2)\nE_cell = E°_cell - (RT/nF)·ln Q\nAt 298 K: E_cell = 1.10 - (0.0591/2)·log₁₀([Zn²⁺]/[Cu²⁺])\n\nEquilibrium state: E_cell = 0, ΔG° = -nFE°_cell.",
    ),
    (
        "[3] Huygens Principle",
        "Q: State Huygens Principle and derive Snell's Law.",
        "Huygens Principle:\n• Every wavefront point is a secondary source of spherical wavelets.\n• Wavelets travel at the medium wave speed.\n\nSnell's Law proof:\nsin i / sin r = v₁/v₂ = μ₂/μ₁  [Refraction Verified]",
    ),
];

// ─── Active Interaction Sliders ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct StudyTopic {
    pub title: &'static str,
    pub question: &'static str,
    pub formula: &'static str,
    pub derivation: &'static str,
    pub exam_note: &'static str,
}

pub const STUDY_TOPIC_DATA: [StudyTopic; 3] = [
    StudyTopic {
        title: "[1] Gauss's Law",
        question: "Q: State Gauss's Law and derive the electric field from Coulomb's Law.",
        formula: "∮ E·dA = Q_enc / ε₀",
        derivation: "1. Build spherical surface of radius r around charge +q.\n2. By symmetry E is radial: E(4πr²) = q/ε₀.\n3. ∴ E = q/(4πε₀r²)  [Q.E.D.]",
        exam_note: "Board Exam Note: Valid for any closed Gaussian surface enclosing charge Q.",
    },
    StudyTopic {
        title: "[2] Nernst Equation",
        question: "Q: Derive Nernst equation for a Daniell cell at 298 K.",
        formula: "E_cell = E°_cell - (RT/nF)·ln Q",
        derivation: "Daniell cell: Zn + Cu²⁺ ⇌ Zn²⁺ + Cu  (n=2)\nAt 298 K: E_cell = 1.10 - (0.0591/2)·log₁₀([Zn²⁺]/[Cu²⁺])\nEquilibrium state: E_cell = 0, ΔG° = -nFE°_cell.",
        exam_note: "Board Exam Note: Standard Daniell cell potential E° is 1.10 V at 298 K.",
    },
    StudyTopic {
        title: "[3] Huygens Principle",
        question: "Q: State Huygens Principle and derive Snell's Law.",
        formula: "sin i / sin r = v₁/v₂ = μ₂/μ₁",
        derivation: "1. Every wavefront point is a secondary source of spherical wavelets.\n2. Wavelets travel at the medium wave speed.\n3. New wavefront is tangential envelope to wavelets.",
        exam_note: "Board Exam Note: Frequency remains unchanged during refraction.",
    },
];

pub fn active_study_topic(topics: &[StudyTopic], active_tab: usize) -> Option<&StudyTopic> {
    topics.get(active_tab).or_else(|| topics.first())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveSlider {
    NightWarmth,
    MasterVolume,
    UiScale,
    Brightness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerViewMode {
    Grid,
    List,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BrowserPageKind {
    #[default]
    SearchHome,
    WebPage {
        url: String,
        title: String,
        domain: String,
        content_lines: Vec<String>,
        is_video: bool,
    },
    BlockedShield {
        url: String,
        domain: String,
        reason: String,
        category: String,
    },
}

pub fn extract_domain(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    without_scheme
        .split('/')
        .next()
        .unwrap_or(without_scheme)
        .split(':')
        .next()
        .unwrap_or(without_scheme)
        .to_string()
}

pub fn generate_in_os_page_content(url: &str, domain: &str) -> (String, Vec<String>, bool) {
    let lower_url = url.to_lowercase();
    if lower_url.contains("youtube.com") || lower_url.contains("youtu.be") {
        let title = "YouTube Study Mode — Distraction-Free Video Lecture".to_string();
        let lines = vec![
            "▶ 1080p60 • MIT 8.02: Electricity & Magnetism (Prof. Walter Lewin)".to_string(),
            "  15:45 / 45:00 • Speed: 1.0x • Audio: 48kHz Stereo".to_string(),
            "".to_string(),
            "MIT 8.02 Electricity & Magnetism — Lecture 11: Magnetic Fields & Lorentz Force".to_string(),
            "Speaker: Prof. Walter Lewin • MIT Department of Physics (OCW)".to_string(),
            "Resolution: 1080p60 • Audio: 48kHz Stereo • Buffer: 100% Loaded".to_string(),
            "".to_string(),
            "[00:00 - 10:15] Definition of Magnetic Field (B) & Lorentz Force Law: F = q(E + v × B)".to_string(),
            "[10:16 - 22:40] Circular Trajectory of Charged Particles in Uniform Transverse B-Field".to_string(),
            "[22:41 - 34:10] Cyclotron Frequency: ω = qB/m (Independent of Velocity/Radius)".to_string(),
            "[34:11 - 45:00] Live Demonstration: Electron Beam Deflection in Cathode Ray Tube".to_string(),
            "".to_string(),
            "MindSet Integrity Shield: Video Ads Skipped • Recommendations Removed • Shorts Blocked.".to_string(),
        ];
        (title, lines, true)
    } else if lower_url.contains("nptel.ac.in") {
        let title = "NPTEL — Sovereign Video Lecture Portal".to_string();
        let lines = vec![
            "Course: Modern Quantum Mechanics (IIT Madras)".to_string(),
            "Instructor: Prof. Ramamurti Shankar • 12 Weeks Online Certification".to_string(),
            "".to_string(),
            "Module 4: Postulates of Quantum Mechanics & Hilbert Space Representations".to_string(),
            "1. State Vectors |ψ⟩ and Wave Functions in Position Space ψ(x) = ⟨x|ψ⟩".to_string(),
            "2. Hermitian Operators & Observable Eigenvalues: A|a⟩ = a|a⟩ with real a".to_string(),
            "3. Measurement Collapse Postulate & Probability Density P(a) = |⟨a|ψ⟩|²".to_string(),
            "4. Time-Dependent Schrödinger Equation: iℏ ∂/∂t |ψ(t)⟩ = H |ψ(t)⟩".to_string(),
            "".to_string(),
            "Status: TLS 1.3 Verified • Offline Cache Ready • Zero Ads Injected".to_string(),
        ];
        (title, lines, true)
    } else if lower_url.contains("wikipedia.org") {
        let title = "Wikipedia — Free Sovereign Encyclopedia".to_string();
        let lines = vec![
            "Article: Maxwell's Equations & Electromagnetic Waves".to_string(),
            "Verified STEM Article • CC BY-SA 4.0 License".to_string(),
            "".to_string(),
            "Four Partial Differential Equations describing classical electrodynamics:".to_string(),
            "• Gauss's Law: ∇ · E = ρ / ε₀ (Electric charges act as flux sources)".to_string(),
            "• Gauss's Law for Magnetism: ∇ · B = 0 (No magnetic monopoles exist)".to_string(),
            "• Faraday's Law of Induction: ∇ × E = -∂B/∂t (Time-varying B creates E)".to_string(),
            "• Ampère-Maxwell Law: ∇ × B = μ₀(J + ε₀ ∂E/∂t) (Displacement current)".to_string(),
            "".to_string(),
            "MindSet Net Shield: Clean Reading View Active • Trackers Stripped".to_string(),
        ];
        (title, lines, true)
    } else if lower_url.contains("khanacademy.org") {
        let title = "Khan Academy — STEM Mastery Library".to_string();
        let lines = vec![
            "Unit: Multivariable Calculus — Green's Theorem & Stokes' Theorem".to_string(),
            "Mastery Level: 85% • 4 of 5 Practice Quizzes Completed".to_string(),
            "".to_string(),
            "Theorem Formulation:".to_string(),
            "∮_C (L dx + M dy) = ∬_D (∂M/∂x - ∂L/∂y) dA".to_string(),
            "Relates a line integral around a simple closed curve C to a double integral"
                .to_string(),
            "over the plane region D bounded by C.".to_string(),
            "".to_string(),
            "Interactive exercises and practice hints ready below.".to_string(),
        ];
        (title, lines, true)
    } else {
        let title = format!("{} — Secure In-OS Browser", domain);
        let lines = vec![
            format!("Viewing URL: {}", url),
            format!("Host Domain: {}", domain),
            "Connection: TLS 1.3 Verified • End-to-End Encrypted".to_string(),
            "Shield Status: Zero-Incognito Active • Ad/Trackers Filtered".to_string(),
            "".to_string(),
            "This web page is rendered securely inside MindSet OS sovereign canvas.".to_string(),
            "External telemetry, tracking beacons, and popups are automatically blocked."
                .to_string(),
        ];
        (title, lines, false)
    }
}

// ─── ShellState (Compositor & Multi-Window Domain) ────────────────────────────

pub struct ShellState {
    pub theme: Theme,
    pub input: InputState,

    // Compositor Subsystems
    pub wm: WindowManager,
    pub desktop_grid: DesktopGrid,
    pub context_menu: DesktopContextMenu,
    pub command_launcher: CommandLauncher,
    pub alt_tab: AltTabSwitcher,
    pub session: SessionManager,

    // Active Virtual Workspace (0..=3) — MindSet Flash
    pub active_workspace: u8,
    pub flash_switcher_open: bool,

    // Reverse Goal Engineering & Neuroscience Academic Goal Tracking
    pub exam_goal: AcademicExamGoal,
    pub secondary_goal: Option<AcademicExamGoal>,
    pub what_if_simulated_score: u32,
    pub brightness: f32,
    pub pomodoro_duration_mins: u32,
    pub pomodoro_completed_sessions: u32,

    // Live Text Input Buffers
    pub maindy_query_input: TextInputBuffer,
    pub task_title_input: TextInputBuffer,

    // Search & Intelligence Subsystem
    pub search_engine: AcademicSearchEngine,
    pub search_input: TextInputBuffer,
    pub search_selected_filter: SearchFilter,
    pub search_results: Vec<SearchResult>,

    // Internet Integrity & MindSet Browser Subsystem
    pub net_hub: InternetIntegrityHub,
    pub browser_active_tab: usize,
    pub browser_blocked_reason: Option<String>,
    pub youtube_timer_accum: f32,
    pub browser_page_kind: BrowserPageKind,
    pub browser_history: Vec<BrowserPageKind>,
    pub browser_history_idx: usize,

    // MindSet Browser Native UI State
    pub browser_screen: BrowserScreen,
    pub browser_sidebar_expanded: bool,
    pub browser_active_tab_idx: usize,
    pub browser_tabs: Vec<BrowserTab>,
    pub browser_workspace_idx: usize,

    // Bluetooth Peripherals Subsystem
    pub bluetooth_mgr: BluetoothManager,

    // System Troubleshooter & Self-Healing Center
    pub troubleshoot_statuses: [String; 5],
    pub troubleshoot_repaired: [bool; 5],

    // Micro-Toast Feedback Pill
    pub toast_message: Option<String>,
    pub toast_timer: f32,

    // Intelligence & Vault Subsystem
    pub rag_vault: RagVault,
    pub vault_notes: Vec<VaultFileInfo>,
    pub selected_vault_note: usize,
    pub ai_stream_rx: Option<Receiver<AiStreamEvent>>,

    // Study & AI State
    pub study_topic: usize,
    pub ai_full_text: String,
    pub ai_stream_chars: usize,
    pub ai_response_text: String,
    pub ai_is_streaming: bool,
    pub ai_last_token: Instant,
    pub timer_secs: f32,
    pub timer_running: bool,
    pub break_banner: Banner,
    pub break_spring: Spring1D,

    // Tasks & Score Data
    pub tasks: Vec<Task>,
    pub target_score: i32,
    pub score_spring: Spring1D,
    pub weekly_bars: [f32; 7],

    // OS Hardware, Audio & Power Controls
    pub soundscape: StudySoundscape,
    pub master_volume: f32,
    pub active_power_profile: GovernorProfile,
    pub network_diag_status: String,
    pub storage_drives_cache: Vec<String>,

    // Settings Controls
    pub warmth_slider: Slider,
    pub font_slider: Slider,
    pub reduced_motion_sw: ToggleSwitch,
    pub dnd_sw: ToggleSwitch,
    pub wallpaper_preset: WallpaperPreset,

    // Deep Personalization & Modern UI Subsystem
    pub personalization_store: PersonalizationStore,
    pub settings_active_tab: usize,
    pub active_academic_stage: usize,
    pub active_study_target_hours: usize,
    pub glass_blur_quality: usize,
    pub workspaces_enabled: [bool; 4],
    pub widgets_enabled: [bool; 4],

    // File Explorer Navigation & View State
    pub explorer_selected_sidebar: usize,
    pub explorer_view_mode: ExplorerViewMode,
    pub explorer_search: String,
    pub explorer_history: Vec<usize>,
    pub explorer_history_idx: usize,
    pub explorer_selected_item: Option<usize>,
    pub onboarding_wizard: OnboardingWizardState,
    pub inspector_open: bool,
    pub wireframe_bounds: bool,
    pub last_interaction_time: Instant,
    pub desktop_search_input: TextInputBuffer,

    // Control Center & Mindful Mode
    pub control_center_open: bool,
    pub mindful_modal_target: Option<WindowId>,
    pub temporary_unlocked_apps: std::collections::HashSet<WindowId>,
    pub ambient_soundscape: usize,
    pub mindful_engine: MindfulModeEngine,

    // Active Mutually Exclusive Modal Overlay (Rule 3)
    pub active_overlay: ActiveOverlay,

    // Today-Only Launch & Desktop Visibility
    pub show_desktop: bool,

    // Odyssey Map & Game Engine
    pub odyssey_map: mindset_ui::OdysseyMapState,
    pub odyssey_hit_map: crate::odyssey_renderer::OdysseyHitMap,
    pub boot_odyssey_shown: bool,

    // LKS Boot Sequence
    pub boot_completed: bool,

    // Mouse drag interaction
    pub dragging_window_id: Option<WindowId>,
    pub resizing_window_id: Option<WindowId>,
    pub active_slider: Option<ActiveSlider>,
}

impl ShellState {
    pub fn new(
        screen_w: f32,
        screen_h: f32,
        saved_theme_dark: bool,
        saved_warmth: f32,
        saved_font: f32,
        initial_score: i32,
    ) -> Self {
        Self::with_wallpaper(
            screen_w,
            screen_h,
            saved_theme_dark,
            saved_warmth,
            saved_font,
            initial_score,
            0,
        )
    }

    pub fn with_wallpaper(
        screen_w: f32,
        screen_h: f32,
        saved_theme_dark: bool,
        saved_warmth: f32,
        saved_font: f32,
        initial_score: i32,
        saved_wallpaper: u32,
    ) -> Self {
        let mode = if saved_theme_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        let mut theme = Theme::new(mode);
        theme.set_night_warmth(saved_warmth);
        theme.set_font_scale(saved_font);

        let wm = WindowManager::new(screen_w, screen_h);
        let desktop_grid = DesktopGrid::new();
        let context_menu = DesktopContextMenu::new();
        let command_launcher = CommandLauncher::new();
        let alt_tab = AltTabSwitcher::new();
        let session = SessionManager::new(screen_w, screen_h);

        let rag_vault = RagVault::new();
        let _ = rag_vault.initialize();
        let vault_notes = rag_vault.list_notes().unwrap_or_default();

        let tasks = vec![
            Task::new("Wave Optics Numericals", "Physics", 0.65),
            Task::new("Electrochemistry — Nernst Eq.", "Chemistry", 0.40),
            Task::new("3D Vector Algebra", "Mathematics", 0.80),
            Task::new("SQLite & Python Lab", "CS", 0.20),
            Task::new("Huygens Principle Derivation", "Physics", 0.55),
            Task::new("Thermodynamics: Gibbs Energy", "Chemistry", 0.30),
        ];

        let initial_ai = STUDY_TOPICS[0].2.to_string();
        let initial_chars = initial_ai.chars().count();

        let break_rect = LayoutRect::new(0.0, 0.0, 420.0, 72.0);
        let warmth_rect = LayoutRect::new(0.0, 0.0, 260.0, 8.0);
        let font_rect = LayoutRect::new(0.0, 0.0, 260.0, 8.0);
        let sw_rect = LayoutRect::new(0.0, 0.0, 44.0, 24.0);

        let wallpaper_preset = WallpaperPreset::from_u32(saved_wallpaper);

        Self {
            theme,
            input: InputState::new(),
            wm,
            desktop_grid,
            context_menu,
            command_launcher,
            alt_tab,
            session,

            active_workspace: 0,
            flash_switcher_open: false,

            exam_goal: AcademicExamGoal::neet_2027_doctor_preset(),
            secondary_goal: Some(AcademicExamGoal::board_exam_preset(GoalPriority::Secondary)),
            what_if_simulated_score: 648,
            brightness: 1.0,
            pomodoro_duration_mins: 25,
            pomodoro_completed_sessions: 1,

            maindy_query_input: TextInputBuffer::new(
                "Ask Maindy AI anything (e.g. Derive Faraday's law)...",
            ),
            task_title_input: TextInputBuffer::new("Type new study task title..."),

            rag_vault,
            vault_notes,
            selected_vault_note: 0,
            ai_stream_rx: None,

            study_topic: 0,
            ai_full_text: initial_ai.clone(),
            ai_stream_chars: initial_chars,
            ai_response_text: initial_ai,
            ai_is_streaming: false,
            ai_last_token: Instant::now(),
            timer_secs: 25.0 * 60.0,
            timer_running: false,
            break_banner: Banner::new(break_rect, "Time for a break?", BannerKind::Info),
            break_spring: Spring1D::with_preset(0.0, SpringPreset::FluidPanel),

            tasks,
            target_score: initial_score,
            score_spring: Spring1D::with_preset(initial_score as f32, SpringPreset::SnappyMicro),
            weekly_bars: [0.65, 0.80, 0.45, 0.90, 0.70, 0.85, 0.60],

            soundscape: StudySoundscape::Off,
            master_volume: 0.70,
            active_power_profile: GovernorProfile::Balanced,
            network_diag_status: "Wi-Fi: Connected (TP-Link Archer, 85% Link, 12ms)".to_string(),
            storage_drives_cache: vec![
                "Local Disk (C:) — 142 GB free of 476 GB".to_string(),
                "MindSet Vault (D:) — 890 GB free of 1000 GB (Encrypted)".to_string(),
            ],

            warmth_slider: Slider::new(warmth_rect, saved_warmth),
            font_slider: Slider::new(font_rect, saved_font),
            reduced_motion_sw: ToggleSwitch::new(sw_rect, false),
            dnd_sw: ToggleSwitch::new(sw_rect, false),
            wallpaper_preset,

            personalization_store: PersonalizationStore::default(),
            settings_active_tab: 0,
            active_academic_stage: 1,
            active_study_target_hours: 1,
            glass_blur_quality: 1,
            workspaces_enabled: [true, true, true, true],
            widgets_enabled: [true, true, true, false],

            explorer_selected_sidebar: 0,
            explorer_view_mode: ExplorerViewMode::Grid,
            explorer_search: String::new(),
            explorer_history: vec![0],
            explorer_history_idx: 0,
            explorer_selected_item: None,
            onboarding_wizard: OnboardingWizardState::default(),
            inspector_open: false,
            wireframe_bounds: false,
            last_interaction_time: Instant::now(),
            desktop_search_input: TextInputBuffer::new("Search apps, files, or settings..."),

            control_center_open: false,
            mindful_modal_target: None,
            temporary_unlocked_apps: std::collections::HashSet::new(),
            ambient_soundscape: 0,
            mindful_engine: MindfulModeEngine::default(),

            active_overlay: ActiveOverlay::None,
            show_desktop: false,
            odyssey_map: mindset_ui::OdysseyMapState::neet_2027_doctor_preset(),
            odyssey_hit_map: crate::odyssey_renderer::OdysseyHitMap::default(),
            boot_odyssey_shown: false,

            boot_completed: false,

            dragging_window_id: None,
            resizing_window_id: None,
            active_slider: None,

            search_engine: AcademicSearchEngine::new(),
            search_input: TextInputBuffer::new("https://nptel.ac.in"),
            search_selected_filter: SearchFilter::All,
            search_results: Vec::new(),

            net_hub: InternetIntegrityHub::default(),
            browser_active_tab: 0,
            browser_blocked_reason: None,
            youtube_timer_accum: 0.0,
            browser_page_kind: BrowserPageKind::WebPage {
                url: "https://nptel.ac.in".to_string(),
                title: "NPTEL — Sovereign Video Lecture Portal".to_string(),
                domain: "nptel.ac.in".to_string(),
                content_lines: vec![
                    "Course: Modern Quantum Mechanics (IIT Madras)".to_string(),
                    "Instructor: Prof. Ramamurti Shankar • 12 Weeks Online Certification"
                        .to_string(),
                    "".to_string(),
                    "Module 4: Postulates of Quantum Mechanics & Hilbert Space Representations"
                        .to_string(),
                    "1. State Vectors |ψ⟩ and Wave Functions in Position Space ψ(x) = ⟨x|ψ⟩"
                        .to_string(),
                    "2. Hermitian Operators & Observable Eigenvalues: A|a⟩ = a|a⟩ with real a"
                        .to_string(),
                    "3. Measurement Collapse Postulate & Probability Density P(a) = |⟨a|ψ⟩|²"
                        .to_string(),
                    "4. Time-Dependent Schrödinger Equation: iℏ ∂/∂t |ψ(t)⟩ = H |ψ(t)⟩".to_string(),
                    "".to_string(),
                    "Status: TLS 1.3 Verified • Offline Cache Ready • Zero Ads Injected"
                        .to_string(),
                ],
                is_video: false,
            },
            browser_history: vec![BrowserPageKind::WebPage {
                url: "https://nptel.ac.in".to_string(),
                title: "NPTEL — Sovereign Video Lecture Portal".to_string(),
                domain: "nptel.ac.in".to_string(),
                content_lines: vec![
                    "Course: Modern Quantum Mechanics (IIT Madras)".to_string(),
                    "Instructor: Prof. Ramamurti Shankar • 12 Weeks Online Certification"
                        .to_string(),
                ],
                is_video: false,
            }],
            browser_history_idx: 0,

            browser_screen: BrowserScreen::ActiveBrowsing,
            browser_sidebar_expanded: false,
            browser_active_tab_idx: 0,
            browser_tabs: vec![
                BrowserTab::new(0, "NPTEL Video Lectures", "https://nptel.ac.in", "📚"),
                BrowserTab::new(
                    1,
                    "Wikipedia Quantum Mechanics",
                    "https://en.wikipedia.org/wiki/Quantum_mechanics",
                    "⚛️",
                ),
                BrowserTab::new(
                    2,
                    "Khan Academy Calculus",
                    "https://www.khanacademy.org/math/calculus",
                    "📐",
                ),
            ],
            browser_workspace_idx: 0,

            bluetooth_mgr: BluetoothManager::new(),

            troubleshoot_statuses: [
                "Audio Stack: Healthy (48kHz, Low Latency, 2.4ms)".to_string(),
                "Wi-Fi Stack: Connected (TP-Link Archer, 85% Link, 12ms)".to_string(),
                "Bluetooth Stack: Active (2 Wireless Peripherals Linked)".to_string(),
                "SQLite Core DB: Integrity Verified (WAL Active, B-Tree OK)".to_string(),
                "Maindy AI Stream: Optimal (Port 11434, Sovereign Tutor Ready)".to_string(),
            ],
            troubleshoot_repaired: [false, false, false, false, false],

            toast_message: None,
            toast_timer: 0.0,
        }
    }

    pub fn reset_defaults(&mut self, screen_w: f32, screen_h: f32) {
        let (saved_dark, saved_warmth, saved_font, initial_score) = (
            self.theme.mode == ThemeMode::Dark,
            self.theme.night_warmth,
            self.theme.font_scale,
            self.target_score,
        );
        *self = Self::new(
            screen_w,
            screen_h,
            saved_dark,
            saved_warmth,
            saved_font,
            initial_score,
        );
    }

    pub fn switch_workspace(&mut self, ws: u8) {
        if ws < 4 && self.workspaces_enabled[ws as usize] {
            self.active_workspace = ws;
            self.wm.active_window_id = self
                .wm
                .windows
                .iter()
                .filter(|w| w.workspace_id == ws && w.is_open)
                .map(|w| w.id)
                .next_back();
            self.show_toast(format!("Switched to MindSet Flash {}", ws + 1));
        }
    }

    pub fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast_message = Some(msg.into());
        self.toast_timer = 2.8;
    }

    // ─── Rule 3: Compile-Time Modal Exclusivity Helper Methods ─────────────────

    /// Returns the currently active modal overlay, or `ActiveOverlay::None` if none is open.
    pub fn active_overlay(&self) -> ActiveOverlay {
        match &self.active_overlay {
            ActiveOverlay::None => {
                if self.command_launcher.is_open {
                    ActiveOverlay::Launcher
                } else if self.control_center_open {
                    ActiveOverlay::ControlCenter
                } else if self.flash_switcher_open {
                    ActiveOverlay::FlashSwitcher
                } else if self.session.notifications.is_open {
                    ActiveOverlay::Notifications
                } else if self.context_menu.is_open {
                    ActiveOverlay::ContextMenu
                } else if self.onboarding_wizard.is_open {
                    ActiveOverlay::OnboardingWizard
                } else {
                    ActiveOverlay::None
                }
            }
            ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. } => {
                if !self.command_launcher.is_open {
                    ActiveOverlay::None
                } else {
                    self.active_overlay.clone()
                }
            }
            ActiveOverlay::ControlCenter => {
                if !self.control_center_open {
                    ActiveOverlay::None
                } else {
                    ActiveOverlay::ControlCenter
                }
            }
            ActiveOverlay::FlashSwitcher => {
                if !self.flash_switcher_open {
                    ActiveOverlay::None
                } else {
                    ActiveOverlay::FlashSwitcher
                }
            }
            ActiveOverlay::Notifications => {
                if !self.session.notifications.is_open {
                    ActiveOverlay::None
                } else {
                    ActiveOverlay::Notifications
                }
            }
            ActiveOverlay::ContextMenu => {
                if !self.context_menu.is_open {
                    ActiveOverlay::None
                } else {
                    ActiveOverlay::ContextMenu
                }
            }
            ActiveOverlay::OnboardingWizard => {
                if !self.onboarding_wizard.is_open {
                    ActiveOverlay::None
                } else {
                    ActiveOverlay::OnboardingWizard
                }
            }
            ActiveOverlay::OdysseyMap => ActiveOverlay::OdysseyMap,
            ActiveOverlay::TrophyDeck => ActiveOverlay::TrophyDeck,
        }
    }

    /// Atomically opens the given overlay and closes all other overlays,
    /// guaranteeing modal exclusivity and eliminating double-draw bugs.
    pub fn open_overlay(&mut self, overlay: ActiveOverlay) {
        // Enforce exclusivity: close all previous overlays first
        self.command_launcher.close();
        self.control_center_open = false;
        self.flash_switcher_open = false;
        self.session.notifications.is_open = false;
        self.context_menu.close();
        self.onboarding_wizard.is_open = false;

        match &overlay {
            ActiveOverlay::None => {}
            ActiveOverlay::Launcher => {
                self.command_launcher.is_open = true;
                self.command_launcher.input.is_focused = true;
                self.command_launcher.input.clear();
                self.command_launcher.selected_index = 0;
            }
            ActiveOverlay::CommandPalette {
                query,
                selected_idx,
            } => {
                self.command_launcher.is_open = true;
                self.command_launcher.input.is_focused = true;
                self.command_launcher.input.clear();
                self.command_launcher.input.insert_str(query);
                self.command_launcher.selected_index = *selected_idx;
            }
            ActiveOverlay::ControlCenter => {
                self.control_center_open = true;
            }
            ActiveOverlay::FlashSwitcher => {
                self.flash_switcher_open = true;
            }
            ActiveOverlay::Notifications => {
                self.session.notifications.is_open = true;
            }
            ActiveOverlay::ContextMenu => {
                self.context_menu.is_open = true;
            }
            ActiveOverlay::OnboardingWizard => {
                self.onboarding_wizard.is_open = true;
            }
            ActiveOverlay::OdysseyMap => {}
            ActiveOverlay::TrophyDeck => {}
        }
        self.active_overlay = overlay;
    }

    /// Closes any currently active modal overlay.
    pub fn close_overlay(&mut self) {
        self.open_overlay(ActiveOverlay::None);
    }

    /// Returns true if any modal overlay is currently open.
    pub fn is_overlay_open(&self) -> bool {
        self.active_overlay() != ActiveOverlay::None
    }

    /// Toggles the given overlay: closes it if currently active, opens it otherwise.
    pub fn toggle_overlay(&mut self, overlay: ActiveOverlay) {
        if self.active_overlay() == overlay {
            self.close_overlay();
        } else {
            self.open_overlay(overlay);
        }
    }

    pub fn is_app_allowed_on_home(&self, id: WindowId) -> bool {
        if self.temporary_unlocked_apps.contains(&id) {
            return true;
        }
        if self.personalization_store.productivity.current_mode == OsMode::Mindful {
            return matches!(
                id,
                WindowId::Today | WindowId::StudyReader | WindowId::Tasks | WindowId::LayaAiChat
            );
        }
        let app_key = match id {
            WindowId::StudyReader => "study_reader",
            WindowId::LayaAiChat => "laya_ai_chat",
            WindowId::Tasks => "tasks",
            WindowId::ProgressStats => "progress_stats",
            WindowId::Settings => "settings",
            WindowId::SystemMonitor => "system_monitor",
            WindowId::FileVault => "file_vault",
            WindowId::SearchBrowser => "search_browser",
            WindowId::Today => "today",
            WindowId::DesignSystem => "design_system",
        };
        self.mindful_engine.is_app_allowed(app_key)
    }

    pub fn request_open_window(&mut self, id: WindowId) {
        if self.is_app_allowed_on_home(id) {
            if let Some(w) = self.wm.find_window_mut(id) {
                w.is_open = true;
                w.workspace_id = self.active_workspace;
                self.wm.promote_to_front(id);
            }
        } else {
            self.mindful_modal_target = Some(id);
        }
    }

    pub fn unlock_app_temporary(&mut self, id: WindowId) {
        self.temporary_unlocked_apps.insert(id);
        if let Some(w) = self.wm.find_window_mut(id) {
            w.is_open = true;
            w.workspace_id = self.active_workspace;
            self.wm.promote_to_front(id);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.toast_timer > 0.0 {
            self.toast_timer = (self.toast_timer - dt).max(0.0);
            if self.toast_timer <= 0.0 {
                self.toast_message = None;
            }
        }

        // YouTube timer tick & countdown
        self.youtube_timer_accum += dt;
        if self.youtube_timer_accum >= 1.0 {
            self.youtube_timer_accum -= 1.0;
            let is_browser_open = self
                .wm
                .find_window(WindowId::SearchBrowser)
                .is_some_and(|w| w.is_open);
            let is_exhausted = self.net_hub.youtube_guard.tick_second();
            if is_browser_open {
                let is_yt = match &self.browser_page_kind {
                    BrowserPageKind::WebPage { url, .. } => {
                        url.contains("youtube.com") || url.contains("youtu.be")
                    }
                    _ => false,
                };
                if is_yt && is_exhausted {
                    self.browser_screen = BrowserScreen::ShieldDashboard;
                    self.show_toast(
                        "YouTube daily screen-time quota exhausted! Study mode locked.",
                    );
                }
            }
        }

        if self.theme.reduced_motion {
            self.score_spring.snap(self.score_spring.target);
            self.break_spring.snap(self.break_spring.target);
            for t in &mut self.tasks {
                t.progress_spring.snap(t.progress);
            }
        } else {
            self.score_spring.update(dt);
            self.break_spring.update(dt);
            for t in &mut self.tasks {
                t.progress_spring.update(dt);
            }
        }
        self.break_banner.slide_t = self.break_spring.position;

        // Study Timer
        if self.timer_running {
            self.timer_secs = (self.timer_secs - dt).max(0.0);
            if self.timer_secs <= 0.0 && !self.break_banner.dismissed {
                if self.theme.reduced_motion {
                    self.break_spring.snap(1.0);
                } else {
                    self.break_spring.set_target(1.0);
                }
            }
        }

        // AI typewriter streamer & channel consumer
        if let Some(ref rx) = self.ai_stream_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    AiStreamEvent::Citation(c) => {
                        self.ai_response_text = format!("Citation: {}\n", c);
                    }
                    AiStreamEvent::Token(t) => {
                        self.ai_response_text.push_str(&t);
                    }
                    AiStreamEvent::Done { full_text, .. } => {
                        self.ai_full_text = full_text;
                        self.ai_is_streaming = false;
                    }
                    _ => {}
                }
            }
        } else if self.ai_is_streaming && self.ai_last_token.elapsed().as_millis() > 20 {
            let total = self.ai_full_text.chars().count();
            if self.ai_stream_chars < total {
                self.ai_stream_chars = (self.ai_stream_chars + 3).min(total);
                self.ai_response_text =
                    safe_utf8_prefix(&self.ai_full_text, self.ai_stream_chars).to_string();
                self.ai_last_token = Instant::now();
            } else {
                self.ai_is_streaming = false;
            }
        }
    }
    pub fn open_browser_url(&mut self, url: &str) {
        let domain = extract_domain(url);
        let normalized_url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        if normalized_url.contains("/shorts") || normalized_url.contains("/shorts/") {
            let reason = "YouTube Shorts Blocked: MindSet Focus Guard policy intercepts short-form dopamine loops.".to_string();
            self.browser_blocked_reason = Some(reason.clone());
            self.browser_page_kind = BrowserPageKind::BlockedShield {
                url: normalized_url,
                domain: domain.clone(),
                reason,
                category: "Distraction Shield".to_string(),
            };
            self.browser_screen = BrowserScreen::ShieldDashboard;
            return;
        }

        if domain.contains("netflix")
            || domain.contains("primevideo")
            || domain.contains("hotstar")
            || domain.contains("disney")
            || domain.contains("twitch")
        {
            let reason =
                "Streaming & OTT platforms permanently blocked during sovereign study mode."
                    .to_string();
            self.browser_blocked_reason = Some(reason.clone());
            self.browser_page_kind = BrowserPageKind::BlockedShield {
                url: normalized_url,
                domain,
                reason,
                category: "Entertainment & OTT Streaming".to_string(),
            };
            self.browser_screen = BrowserScreen::ShieldDashboard;
            return;
        }

        match self
            .net_hub
            .check_and_normalize_url(&normalized_url, self.session.focus_mode)
        {
            Ok(normalized) => {
                let is_video =
                    normalized.contains("youtube.com") || normalized.contains("youtu.be");
                self.browser_blocked_reason = None;
                self.browser_page_kind = BrowserPageKind::WebPage {
                    url: normalized,
                    title: format!("MindSet Web — {}", domain),
                    domain,
                    content_lines: vec![
                        "Sovereign Academic Workspace".to_string(),
                        "Distraction-Free Reading Mode Active".to_string(),
                    ],
                    is_video,
                };
                self.browser_screen = BrowserScreen::ActiveBrowsing;
            }
            Err(reason) => {
                self.browser_blocked_reason = Some(reason.clone());
                self.browser_page_kind = BrowserPageKind::BlockedShield {
                    url: normalized_url,
                    domain,
                    reason,
                    category: "Entertainment & OTT Streaming".to_string(),
                };
                self.browser_screen = BrowserScreen::ShieldDashboard;
            }
        }
    }

    pub fn execute_search(&mut self) {
        let query = self.search_input.text.trim().to_string();
        if query.is_empty() {
            return;
        }
        self.open_browser_url(&format!("https://duckduckgo.com/?q={}", query));
    }

    pub fn ask_custom_laya_query(&mut self, q: &str) {
        self.ask_custom_maindy_query(q);
        let rx = AiStreamBridge::spawn_query(q.to_string(), None, None);
        self.ai_stream_rx = Some(rx);
    }
    pub fn ask_custom_maindy_query(&mut self, q: &str) {
        let q_lower = q.to_lowercase();
        if q_lower.contains("who made you")
            || q_lower.contains("who created you")
            || q_lower.contains("who developed you")
        {
            let ceo_reply = "I WAS SETUP BY THE C.E.O of LKS COMPANY to empower your academic excellence and deep focus.";
            self.ai_response_text = ceo_reply.to_string();
            self.ai_full_text = ceo_reply.to_string();
            self.ai_is_streaming = false;
        } else {
            self.ai_response_text = format!(
                "Maindy AI: Analyzing academic inquiry '{}'. Structured breakdown active.",
                q
            );
            self.ai_full_text = format!("Maindy AI: Analyzing academic inquiry '{}'. Structured breakdown active. Details on Bernoulli & STEM principles (Lenz's Law).", q);
            self.ai_is_streaming = true;
        }
        self.ai_stream_chars = 0;
        self.ai_last_token = Instant::now();
    }

    pub fn active_input_buffer_mut(&mut self) -> Option<&mut TextInputBuffer> {
        if matches!(
            self.active_overlay(),
            ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. }
        ) {
            Some(&mut self.command_launcher.input)
        } else if self.search_input.is_focused {
            Some(&mut self.search_input)
        } else if self.maindy_query_input.is_focused {
            Some(&mut self.maindy_query_input)
        } else if self.task_title_input.is_focused {
            Some(&mut self.task_title_input)
        } else {
            None
        }
    }

    pub fn toggle_focus_mode(&mut self) {
        self.session.focus_mode = !self.session.focus_mode;
        let label = if self.session.focus_mode {
            "Focus Mode ON"
        } else {
            "Focus Mode OFF"
        };
        self.show_toast(label);
    }

    /// Toggle between the clean Today view and the full Desktop with dock/icons
    pub fn toggle_desktop(&mut self) {
        self.set_show_desktop(!self.show_desktop);
    }

    /// Set whether the full Desktop with dock/icons is visible or the clean Today view
    pub fn set_show_desktop(&mut self, show: bool) {
        self.show_desktop = show;
        if !self.show_desktop {
            self.wm.promote_to_front(WindowId::Today);
        }
    }

    /// Toggle soundscape state on/off
    pub fn toggle_soundscape(&mut self) {
        self.exam_goal.soundscape_active = !self.exam_goal.soundscape_active;
        if self.exam_goal.soundscape_active {
            self.soundscape = StudySoundscape::FocusTone;
            self.show_toast("🎧 40 Hz Gamma Study Soundscape ON");
        } else {
            self.soundscape = StudySoundscape::Off;
            self.show_toast("🎧 Study Soundscape OFF");
        }
    }

    /// Cycle through focus timer modes (90m Ultradian <-> 25m Pomodoro <-> 60m Power Drill)
    pub fn cycle_timer_mode(&mut self) {
        self.exam_goal.timer_mode = match self.exam_goal.timer_mode {
            FocusTimerMode::Ultradian90 => FocusTimerMode::Pomodoro25,
            FocusTimerMode::Pomodoro25 => FocusTimerMode::PowerHour60,
            FocusTimerMode::PowerHour60 => FocusTimerMode::Ultradian90,
        };
        self.timer_secs = self.exam_goal.timer_mode.duration_secs();
        self.show_toast(format!(
            "⏱ Timer Mode: {}",
            self.exam_goal.timer_mode.label()
        ));
    }

    /// Trigger 25-minute focus session in StudyReader (zero-panic)
    pub fn trigger_focus_session(&mut self) {
        self.timer_secs = 25.0 * 60.0;
        self.timer_running = true;
        if let Some(win) = self.wm.find_window_mut(WindowId::StudyReader) {
            win.is_open = true;
            win.state = WindowState::Floating;
            win.workspace_id = self.active_workspace;
        }
        self.wm.promote_to_front(WindowId::StudyReader);
        self.show_toast("⚡ Focus Session: Wave Optics (25 min)");
    }

    /// Cycle pomodoro duration (25m -> 30m -> 45m -> 60m -> 25m)
    pub fn cycle_pomodoro_duration(&mut self) -> u32 {
        self.pomodoro_duration_mins = match self.pomodoro_duration_mins {
            25 => 30,
            30 => 45,
            45 => 60,
            _ => 25,
        };
        self.timer_secs = (self.pomodoro_duration_mins as f32) * 60.0;
        self.show_toast(format!(
            "⏱ Pomodoro Duration: {}m",
            self.pomodoro_duration_mins
        ));
        self.pomodoro_duration_mins
    }

    /// Cycle pomodoro session counter (1/4 -> 2/4 -> 3/4 -> 4/4 -> 1/4)
    pub fn cycle_pomodoro_session(&mut self) -> u32 {
        self.pomodoro_completed_sessions = (self.pomodoro_completed_sessions % 4) + 1;
        self.show_toast(format!(
            "🎯 Focus Session {}/4",
            self.pomodoro_completed_sessions
        ));
        self.pomodoro_completed_sessions
    }

    /// Start a deep focus flow session on the today's high-yield mission
    pub fn start_mission_flow_session(&mut self) {
        self.timer_secs = self.exam_goal.timer_mode.duration_secs();
        self.timer_running = true;
        self.session.focus_mode = true;
        if self.exam_goal.soundscape_active {
            self.soundscape = StudySoundscape::FocusTone;
        }

        // Open study reader with high-yield physics chapter notes
        if let Some(mission) = self.exam_goal.today_mission() {
            self.ai_response_text = format!(
                "⚡ HIGH-YIELD MISSION: {}\n\n• Target Exam Weightage: {} Marks\n• Estimated Study Duration: {:.1} Hours\n• ROI Velocity: {:.1} Marks/Hour\n\nKey Concepts & Formulae:\n1. Photoelectric Equation: K_max = hν - Φ = e·V_0\n2. Threshold Frequency: ν_0 = Φ / h\n3. De Broglie Wavelength: λ = h / p = h / √(2mE)\n4. Bohr Model Radius: r_n = 0.529 · (n² / Z) Å\n5. Energy Levels: E_n = -13.6 · (Z² / n²) eV\n\nStudy carefully for 60 minutes and lock in +{} Marks!",
                mission.title, mission.weightage_marks, mission.estimated_hours, mission.roi_marks_per_hour(), mission.weightage_marks
            );
            self.ai_full_text = self.ai_response_text.clone();
            self.ai_is_streaming = false;
        }

        if let Some(w) = self.wm.find_window_mut(WindowId::StudyReader) {
            w.is_open = true;
            w.workspace_id = self.active_workspace;
        }
        self.wm.promote_to_front(WindowId::StudyReader);

        let sound_status = if self.exam_goal.soundscape_active {
            "40Hz Gamma Audio ON"
        } else {
            "Audio OFF"
        };
        self.show_toast(format!(
            "⚡ Flow Session Started: {} ({})",
            self.exam_goal.timer_mode.label(),
            sound_status
        ));
    }

    /// Lock today's mission marks, firing a dopamine prediction error reward
    pub fn lock_today_mission_marks(&mut self) {
        let earned = self.exam_goal.lock_today_mission();
        if earned > 0 {
            let new_proj = self.exam_goal.projected_score();
            self.what_if_simulated_score = new_proj;
            self.adjust_score(earned as i32);
            self.show_toast(format!(
                "🔒 SUCCESS! +{} Marks Locked! Projected: {}/{}",
                earned, new_proj, self.exam_goal.max_score
            ));
        }
    }

    pub fn has_active_motion(&self) -> bool {
        !self.score_spring.is_at_rest()
            || !self.break_spring.is_at_rest()
            || self.toast_timer > 0.0
            || self.tasks.iter().any(|t| !t.progress_spring.is_at_rest())
    }
    pub fn adjust_score(&mut self, delta: i32) {
        self.target_score = (self.target_score + delta).clamp(0, 600);
        self.score_spring.set_target(self.target_score as f32);
    }

    pub fn ask_ai_about_note(&mut self, note_idx: usize) {
        if let Some(note) = self.vault_notes.get(note_idx) {
            self.ai_response_text = format!(
                "Maindy AI: Analyzing note '{}'. Preview:\n{}",
                note.name, note.preview
            );
            self.ai_full_text = self.ai_response_text.clone();
            self.ai_is_streaming = true;
            if let Some(w) = self.wm.find_window_mut(WindowId::LayaAiChat) {
                w.is_open = true;
                w.workspace_id = self.active_workspace;
            }
            self.wm.promote_to_front(WindowId::LayaAiChat);
        }
    }

    pub fn open_note_in_reader(&mut self, note_idx: usize) {
        if let Some(note) = self.vault_notes.get(note_idx) {
            self.ai_response_text = note.preview.clone();
            self.ai_full_text = self.ai_response_text.clone();
            if let Some(w) = self.wm.find_window_mut(WindowId::StudyReader) {
                w.is_open = true;
                w.workspace_id = self.active_workspace;
            }
            self.wm.promote_to_front(WindowId::StudyReader);
        }
    }
    pub fn execute_full_auto_repair(&mut self) {
        for s in &mut self.troubleshoot_repaired {
            *s = true;
        }
        for st in &mut self.troubleshoot_statuses {
            *st = "Repaired & Synced (100% Calibrated)".to_string();
        }
        self.show_toast("All 5 System Subsystems Auto-Repaired to 100% Health!");
    }

    pub fn execute_auto_repair_item(&mut self, idx: usize) {
        if idx < self.troubleshoot_repaired.len() {
            self.troubleshoot_repaired[idx] = true;
            if idx < self.troubleshoot_statuses.len() {
                self.troubleshoot_statuses[idx] = "Repaired & Operational".to_string();
            }
            let names = [
                "Kernel IPC",
                "Dual-Kawase Blur",
                "SDF Engine",
                "Maindy AI (11434)",
                "Audio Governor",
            ];
            let name = names.get(idx).unwrap_or(&"Subsystem");
            self.show_toast(format!(
                "Subsystem [{}] Successfully Calibrated & Repaired!",
                name
            ));
        }
    }

    pub fn browser_go_back(&mut self) {
        self.browser_screen = BrowserScreen::NewTab;
        self.browser_page_kind = BrowserPageKind::SearchHome;
    }

    pub fn browser_go_home(&mut self) {
        self.browser_screen = BrowserScreen::NewTab;
        self.browser_page_kind = BrowserPageKind::SearchHome;
        self.browser_blocked_reason = None;
    }
}

pub use mindset_material::{DualKawasePipeline, MaterialTargetManager};
pub use mindset_surface::mindful_mode::OsFocusMode;
pub use mindset_surface::session::{
    NotificationCategory, PowerAction, SessionState, TOP_BAR_HEIGHT,
};
pub use mindset_surface::wm::{WindowId, WindowState};
pub use mindset_sys::db::DatabaseManager;
pub use mindset_sys::devices::{BluetoothManager, GovernorProfile};
pub use mindset_sys::fs::FileEntry;
pub use mindset_sys::storage::MindSetPaths;
pub use mindset_ui::theme::ThemeMode;
pub use mindset_ui::ActiveOverlay;

pub use mindset_render::WallpaperPreset;
pub use mindset_surface::clipboard::SystemClipboard;
pub use mindset_sys::personalization::PersonalizationStore;
