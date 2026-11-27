pub mod ai;
pub mod apps;
pub mod capsules;
pub mod i18n;
pub mod onboarding;
pub mod shell;
pub mod storage;
pub mod system;
pub mod theme;
pub mod views;
pub mod wm;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use iced::futures::SinkExt;
use iced::keyboard::{self, key::Named, Key};
use iced::widget::{column, container, mouse_area, row, stack};
use iced::{event, mouse, window, Alignment, Element, Event, Length, Size, Subscription, Task};
use mindset_core::agent::ollama_grader::FeynmanGrader;
use mindset_core::engine::verification::{
    CapsuleRank, CapsuleState, ProofOfStudyGate, VerificationScoreBreakdown,
};
use mindset_core::{ActiveOverlay, MindSetCoreState, ThemeMode, WindowId, WindowMode};
use theme::{h_space, v_space, ThemePalette};

/// Startup shell configuration parsed from CLI flags (used by `mindset-shell` and headless verification).
#[derive(Debug, Clone)]
pub struct LaunchOptions {
    pub window_size: Size,
    pub start_maximized: bool,
    pub fullscreen: bool,
    pub theme_override: Option<ThemeMode>,
    pub wallpaper_override: Option<String>,
    pub skip_boot: bool,
    pub smoke_frames: Option<u32>,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            window_size: Size::new(1280.0, 800.0),
            start_maximized: true,
            fullscreen: false,
            theme_override: None,
            wallpaper_override: None,
            skip_boot: false,
            smoke_frames: None,
        }
    }
}

impl LaunchOptions {
    pub fn from_env() -> Self {
        let args: Vec<String> = std::env::args().collect();
        Self::parse_args(&args[1..])
    }

    pub fn parse_args(args: &[String]) -> Self {
        let mut opts = Self::default();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--windowed" => {
                    opts.start_maximized = false;
                    opts.fullscreen = false;
                }
                "--fullscreen" => {
                    opts.fullscreen = true;
                    opts.start_maximized = true;
                }
                "--skip-boot" => {
                    opts.skip_boot = true;
                }
                "--size" => {
                    if let Some(val) = args.get(i + 1) {
                        if let Some((w_str, h_str)) = val.split_once('x') {
                            if let (Ok(w), Ok(h)) = (w_str.parse::<f32>(), h_str.parse::<f32>()) {
                                opts.window_size = Size::new(w.max(640.0), h.max(480.0));
                                opts.start_maximized = false;
                                opts.fullscreen = false;
                            }
                        }
                        i += 1;
                    }
                }
                "--theme" => {
                    if let Some(val) = args.get(i + 1) {
                        opts.theme_override = match val.to_lowercase().as_str() {
                            "alpine" | "light" => Some(ThemeMode::AlpineGlass),
                            "dark" => Some(ThemeMode::DarkSpace),
                            "violet" | "nebula" => Some(ThemeMode::VioletNebula),
                            "amber" | "solar" => Some(ThemeMode::SolarAmber),
                            _ => None,
                        };
                        i += 1;
                    }
                }
                "--wallpaper" => {
                    if let Some(val) = args.get(i + 1) {
                        opts.wallpaper_override = Some(val.clone());
                        i += 1;
                    }
                }
                "--smoke-frames" => {
                    if let Some(val) = args.get(i + 1) {
                        if let Ok(frames) = val.parse::<u32>() {
                            opts.smoke_frames = Some(frames.max(1));
                            opts.skip_boot = true;
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        opts
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    Tick,
    SysinfoTick,
    ScaleChanged(f32),
    BrightnessChanged(f32),
    VolumeChanged(f32),
    ThemeSelected(ThemeMode),
    ToggleWifi,
    ToggleBluetooth,
    ToggleAirplaneMode,
    ToggleDnd,
    ToggleDesktop,
    OpenWindow(WindowId),
    CloseWindow(WindowId),
    MinimizeWindow(WindowId),
    MaximizeWindow(WindowId),
    FocusWindow(WindowId),
    StartWindowDrag(WindowId),
    EndWindowDrag,
    StartWidgetDrag(mindset_core::DesktopWidgetId),
    EndWidgetDrag,
    ToggleWidgetPin(mindset_core::DesktopWidgetId),
    ResetWidgetLayout,
    ToggleOverlay(ActiveOverlay),
    CloseOverlays,
    ToggleSideHud,
    SystemLogout,
    SystemReboot,
    SystemPowerOff,
    StartFocusSession,
    TogglePomodoro,
    CyclePomodoroPreset,
    ResetPomodoro,
    Log10mTask(String),
    ToggleTask(String),
    NewTaskInputChanged(String),
    AddTask(String, String, u32),
    SelectStudyTab(usize),
    SelectPreferencesTab(usize),
    VaultSelectSidebar(String),
    VaultEntryClicked(String, bool),
    VaultOpenFolder(String),
    VaultNavigateUp,
    VaultSearchChanged(String),
    VaultToggleTreeNode(String),
    VaultPdfPrevPage,
    VaultPdfNextPage,
    VaultPdfJumpPage(String),
    VaultPptxSelectSlide(usize),
    VaultPptxTitleChanged(String),
    VaultPptxBodyChanged(String),
    VaultPptxSaveSlide,
    VaultTextEditChanged(String),
    VaultTextSave,
    EmbarkNode(String),
    CompleteStage(String),
    MindfulStayFocused,
    MindfulUnlock5m,
    SimulateDistraction,
    WizardNext,
    WizardPrev,
    WizardClose,
    SearchInputChanged(String),
    SelectSearchResult(WindowId),
    ChatInputChanged(String),
    SendChatMessage,
    AiStreamToken(ai::StreamToken),
    CancelAiStream,
    ModelsDiscovered(Vec<String>),
    SelectModel(String),
    UnloadOllamaModel,
    ModelUnloaded(Result<(), String>),
    NewChatThread,
    SelectChatThread(String),
    GenerateDailyCapsules,
    ScratchpadChanged(String),
    SaveScratchpadDebounced {
        revision: u64,
        text: String,
    },
    ScratchpadSaved {
        revision: u64,
        result: Result<(), String>,
    },
    CopyScratchpad,
    ClearScratchpad,
    DismissToast(u64),
    TriggerNotificationAction(u64, mindset_core::NotificationAction),
    MaindyToggleLeftRail,
    MaindyToggleRightDrawer,
    MaindyToggleEnginePopover,
    MaindySelectDrawerTab(apps::ai_tutor::MaindyDrawerTab),
    SetMaindyScreen(apps::ai_tutor::MaindyScreen),
    OpenStudyCapsule(String),
    SelectCapsulePhase(usize),
    ToggleCapsuleHint,
    ExplainCapsuleStepInTutor(String),
    StartCapsuleFocusSprint(String, u16),
    PinCapsuleToDesktopHud(String),
    RunQuickOrbCommand(String),
    ConfirmDestructiveOsAction,
    CancelDestructiveOsAction,
    OpenExamReport,
    OpenVerificationBoss,
    SelectBossStage1(usize, usize),
    SelectBossStage2(usize, usize),
    FeynmanInputChanged(String),
    SubmitVerificationBoss,
    BossGraded {
        recall_score: u8,
        applied_score: u8,
        feynman_res: mindset_core::FeynmanGradeResult,
        weak_topic: String,
    },
    StartOllama,
    RetryBossSession,
    TriggerExcuseRoast(String),
    AnswerExcuseFormula(usize),
    OpenBreakArcade,
    SelectArcadeCard(usize),
    // Memory Subsystem
    DeleteUserMemory(i64),
    NewMemoryInputChanged(String),
    AddUserMemory,
    // Boot Sequence & First-Boot Onboarding
    BootSplashTick,
    FinishBootSplash,
    RerunOnboarding,
    OnboardingNext,
    OnboardingPrev,
    OnboardingSetName(String),
    OnboardingSetClassDegree(String),
    OnboardingSetLanguage(String),
    OnboardingSelectTrack(String),
    OnboardingSetTargetScore(u32),
    OnboardingSetExamDate(String),
    OnboardingSetWeekdayHours(u32),
    OnboardingSetStrongSubject(String),
    OnboardingSetToughestSubject(String),
    OnboardingSetCoachingStyle(String),
    OnboardingSetWeakTopics(String),
    OnboardingSetWakeTime(String),
    OnboardingSetBedTime(String),
    OnboardingSetSchoolHours(String),
    OnboardingToggleObstacle(String),
    OnboardingSetRamProfile(String),
    OnboardingComplete,
    DockHovered(Option<usize>),
    MaindyOrbHovered(bool),
    ToggleMaindyDrawer,
    SetIslandMode(shell::dynamic_island::IslandMode),
    IslandSearchChanged(String),
    SetThemeMode(String),
    SetMildMotion(bool),
    SetWallpaperPreset(String),
    SelectCustomWallpaper,
    CustomWallpaperSelected(Option<String>),
    SetMapTheme(String),
    HoverOdysseyNode(Option<usize>),
    SelectOdysseyNode(Option<usize>),
    ScrollOdysseyMap(f32),
    LaunchOdysseyNode(String),
    CloseOdysseyPopover,
    MapAmbientTick,
    AnimationFrame(std::time::Instant),
    ToggleAudio,
    ToggleTunnelVision,
    SetLanguage(String),
    SetMaindyStyle(String),
    SetAiApiKey(String),
    SetAiCloudProvider(String),
    SetAiModel(String),
    SetAiBaseUrl(String),
    TestAiConnection,
    AiConnectionTested(Result<String, String>),
    SpeakMessage(String),
    StopTts,
    ToggleAutoSpeak,
    // Agent Tools & Permissions
    AgentPermissionResponse(ai::agent::AgentPermissionDecision),
    RevokeAllAgentPermissions,
    ClearAiActivityLog,
    // Real Browser & YouTube Study
    BrowserUrlChanged(String),
    BrowserNavigate(String),
    LaunchBrowserUrl(String),
    BrowserLaunched(Result<String, String>),
    ToggleBrowserFocusShield,
    BrowserBack,
    BrowserForward,
    OpenYouTubeStudy,
    LaunchYouTubeApp,
    LaunchFreeTubeApp,
    LaunchMindSetBrowser(Option<String>),
    // Wi-Fi & Network Manager
    RefreshNetworkStatus,
    NetworkStatusUpdated(mindset_sys::network::NetworkStatus),
    ScanWifiNetworks,
    WifiNetworksScanned(Vec<mindset_sys::network::WifiNetwork>),
    ToggleWifiMenu,
    SelectWifiNetwork(String),
    WifiPasswordInputChanged(String),
    SubmitWifiConnect,
    CancelWifiConnect,
    WifiConnectResult(Result<String, String>),
    // Lock Screen Auth
    LockScreenPinChanged(String),
    LockScreenPasswordChanged(String),
    LockScreenToggleAuthMode,
    LockScreenToggleShowPassword,
    LockScreenKeypadDigit(char),
    LockScreenKeypadBackspace,
    LockScreenKeypadClear,
    SubmitLockScreenAuth,
    // Widget Management & Overlay Docking
    ToggleWidgetVisibility(mindset_core::DesktopWidgetId),
    DockOverlayAsWidget(mindset_core::ActiveOverlay),
    // Capsule Engine Scheduling
    AdaptTodaySchedule(String, f32),
    AutoHealMissedDay,
    // Native MS OS YouTube Study
    YouTubeSearchChanged(String),
    YouTubeSearchSubmit,
    YouTubeSearchResults(Vec<apps::youtube_study::StudyVideoItem>, String),
    YouTubeSelectVideo(usize),
    YouTubeNotesChanged(String),
    // MindStore (.ms Package Manager)
    MindStoreSearchChanged(String),
    MindStoreSelectCategory(String),
    MindStoreInstallCatalogApp(String),
    MindStoreLaunchApp(String),
    MindStoreUninstallApp(String),
    MindStorePickLocalPackage,
    MindStoreLocalPackageSelected(Option<std::path::PathBuf>),
    // System Installer
    InstallMindSetToDisk,
    // RAG Vault (Zone 3)
    PickVaultPdfToIngest,
    IngestPdfPath(Option<std::path::PathBuf>),
    Zone3PrevPage,
    Zone3NextPage,
    Zone3SelectPdf(String),
    Zone3SearchChanged(String),
    // Academic Search (Zone 4)
    Zone4SearchInputChanged(String),
    Zone4SelectProvider(mindset_intelligence::SearchProviderType),
    Zone4ExecuteSearch,
    Zone4SearchCompleted(Result<Vec<mindset_intelligence::SearchResult>, String>),
    // Personalization Engine
    SubmitFatigueCheckin(mindset_intelligence::FatigueLevel, f32),
    ReviewLeitnerCard {
        id: String,
        success: bool,
    },
    AddLeitnerCard {
        front: String,
        back: String,
        subject: String,
    },
    ResolveStudentMistake(i64),
    // 590/600 Exam Paper Reviewer
    PickExamPaperToGrade,
    EvaluateExamPaperPath(Option<std::path::PathBuf>),
    // Competitive Exam MCQ Generator
    GenerateCompetitiveMcq {
        target: mindset_core::agent::ExamTarget,
        subject: String,
        topic: Option<String>,
        count: usize,
    },
    SelectCompetitiveMcqOption(String, usize),
    SubmitCompetitiveMcq,
    EventOccurred(Event),
}

pub struct MindSetApp {
    pub core: MindSetCoreState,
    pub system: system::SystemState,
    pub storage: storage::StorageManager,
    pub sysinfo: system::SysinfoCollector,
    pub wm_interaction: wm::WindowInteractionState,
    pub search_query: String,
    pub chat_input: String,
    pub active_thread_id: String,
    pub threads: Vec<storage::ChatThread>,
    pub active_messages: Vec<storage::ChatMessageRecord>,
    pub is_generating: bool,
    pub active_model: String,
    pub installed_models: Vec<String>,
    pub cancel_flag: Option<Arc<AtomicBool>>,
    pub new_task_input: String,
    pub feynman_input: String,
    pub stage1_selections: Vec<Option<usize>>,
    pub stage2_selections: Vec<Option<usize>>,
    pub excuse_selected_answer: Option<usize>,
    pub wallpaper_handle: Option<iced::widget::image::Handle>,
    pub profile: storage::db::UserProfile,
    pub capsules: Vec<storage::db::CapsuleRecord>,
    pub memories: Vec<storage::db::UserMemory>,
    pub new_memory_input: String,
    pub onboarding_data: onboarding::OnboardingData,
    pub youtube_study: apps::youtube_study::YouTubeStudyState,
    pub mindstore_state: apps::mindstore::MindStoreState,
    pub dock_springs: [mindset_motion::Spring1D; 14],
    pub hovered_dock_index: Option<usize>,
    pub hud_visible: bool,
    pub smoke_frames_remaining: Option<u32>,
    pub maindy_spring: mindset_motion::Spring1D,
    pub maindy_drawer_open: bool,
    pub island_mode: shell::dynamic_island::IslandMode,
    pub island_width_spring: mindset_motion::Spring1D,
    pub island_height_spring: mindset_motion::Spring1D,
    pub island_search_query: String,
    pub odyssey_map_state: shell::odyssey_map::OdysseyMapState,
    pub tunnel_vision_active: bool,
    pub tunnel_vision_spring: mindset_motion::Spring1D,
    pub audio_player: system::AudioPlayer,
    pub last_anim_instant: Option<std::time::Instant>,
    pub cursor_position: iced::Point,
    pub i18n: i18n::I18n,
    pub auto_speak_enabled: bool,
    pub boot_splash: shell::boot_splash::BootSplashState,
    pub pending_agent_action: Option<mindset_intelligence::agent_tools::AgentAction>,
    pub session_allowed_folders: std::collections::HashSet<String>,
    pub ai_activity_log: Vec<storage::db::AiActivityRecord>,
    pub browser_url: String,
    pub browser_focus_shield: bool,
    pub browser_history: Vec<String>,
    pub browser_history_idx: usize,
    pub browser_status_note: Option<String>,
    pub network_status: mindset_sys::network::NetworkStatus,
    pub nearby_networks: Vec<mindset_sys::network::WifiNetwork>,
    pub wifi_menu_open: bool,
    pub wifi_connecting_ssid: Option<String>,
    pub wifi_password_input: String,
    pub lock_screen: shell::lock_screen::LockScreenState,
    pub wallpaper_canvas_cache: iced::widget::canvas::Cache,
    // Wired Intelligence & Exam Engines
    pub rag_vault: mindset_intelligence::RagVault,
    pub rag_pdf_chunks: Vec<mindset_intelligence::PdfDocumentChunk>,
    pub rag_active_file: Option<String>,
    pub rag_current_page: usize,
    pub rag_query_input: String,
    pub rag_citations: Vec<mindset_intelligence::PdfCitationMatch>,
    pub academic_search: mindset_intelligence::AcademicSearchEngine,
    pub academic_search_input: String,
    pub academic_search_provider: mindset_intelligence::SearchProviderType,
    pub academic_search_loading: bool,
    pub academic_search_error: Option<String>,
    pub personalization: mindset_intelligence::PersonalizationEngine,
    pub today_checkin: Option<mindset_intelligence::DailyFatigueCheckin>,
    pub due_leitner_cards: Vec<mindset_intelligence::LeitnerCard>,
    pub unresolved_mistakes: Vec<mindset_intelligence::StudentMistakeRecord>,
    pub last_parsed_query: Option<ai::persona::ParsedQuery>,
    pub active_exam_evaluation: Option<mindset_core::agent::EvaluationSheet>,
    pub exam_eval_status: Option<String>,
    pub mcq_generator: mindset_core::agent::CompetitiveMcqGenerator,
    pub active_mcq_paper: Option<mindset_core::agent::TestPaper>,
    pub mcq_answers: std::collections::HashMap<String, mindset_core::agent::StudentAnswer>,
    pub mcq_report: Option<mindset_core::agent::TestEvaluationReport>,
    // Screen 1-4 Maindy Tutor Studio, Capsule Theater & Destructive Confirm Guard
    pub maindy_screen: apps::ai_tutor::MaindyScreen,
    pub maindy_drawer_tab: apps::ai_tutor::MaindyDrawerTab,
    pub maindy_left_rail_open: bool,
    pub maindy_right_drawer_open: bool,
    pub maindy_engine_popover_open: bool,
    pub study_capsules: Vec<ai::maindy_system_bridge::StudyCapsule>,
    pub active_capsule_id: Option<String>,
    pub active_capsule_phase: usize,
    pub capsule_hint_open: bool,
    pub pending_destructive_action: Option<ai::maindy_system_bridge::MaindyOsAction>,
}

impl Default for MindSetApp {
    fn default() -> Self {
        let storage = storage::StorageManager::new();
        let rag_vault = mindset_intelligence::RagVault::new();
        Self::init_with_storage_and_vault(storage, rag_vault)
    }
}

impl MindSetApp {
    pub fn with_storage_path(db_path: std::path::PathBuf) -> Self {
        let vault_dir = db_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("rag_vault");
        let storage = storage::StorageManager::with_path(db_path);
        let rag_vault = mindset_intelligence::RagVault::with_path(vault_dir);
        Self::init_with_storage_and_vault(storage, rag_vault)
    }

    fn init_with_storage_and_vault(
        storage: storage::StorageManager,
        rag_vault: mindset_intelligence::RagVault,
    ) -> Self {
        let scratchpad_path = storage
            .db_path()
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("mindset_os_scratchpad.txt");
        let initial_scratchpad = storage.load_or_migrate_scratchpad(&scratchpad_path);
        let system = system::SystemState::new_with_scratchpad(initial_scratchpad);

        let threads = storage.list_threads();
        let active_thread_id = threads
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "main-session".to_string());
        let active_messages = storage.load_recent_messages(&active_thread_id, 30);
        let wallpaper_handle = [
            std::path::PathBuf::from("assets/wallpaper.jpg"),
            std::path::PathBuf::from("/usr/share/mindset-os/wallpaper.jpg"),
            std::path::PathBuf::from("/opt/mindset-os/assets/wallpaper.jpg"),
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/wallpaper.jpg"),
        ]
        .into_iter()
        .find(|p| p.is_file())
        .map(iced::widget::image::Handle::from_path);

        let profile = storage.load_profile();
        let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut capsules = storage.list_capsules(&today_str);
        if capsules.is_empty() {
            let initial = capsules::generator::generate_capsules_for_blueprint(
                &profile.career_track,
                &profile.weak_topics,
            );
            for cap in &initial {
                storage.save_capsule(cap);
            }
            capsules = storage.list_capsules(&today_str);
            if capsules.is_empty() {
                capsules = initial;
            }
        }
        let memories = storage.list_memories();
        let onboarding_data = onboarding::OnboardingData::new_with_profile(profile.clone());

        let mut core = MindSetCoreState::new();
        if let Some(mut saved_plan) = storage.load_capsule_plan() {
            let healed =
                mindset_core::CapsulePlanner::reconcile_elapsed_days(&mut saved_plan, &today_str);
            if healed > 0 {
                let _ = storage.save_capsule_plan(&saved_plan);
            }
            core.exam_plan = saved_plan;
        }
        if profile.onboarding_completed {
            core.today = mindset_core::TodayState::fresh_for_student(
                profile.weekday_study_hours,
                &profile.target_label,
            );
        }
        let saved_layouts = storage.load_widget_layouts();
        let i18n = i18n::I18n::new(i18n::AppLanguage::from_code(&profile.language_code));
        let ai_activity_log = storage.list_ai_activities(50);

        let _ = rag_vault.initialize();
        let rag_pdf_chunks = rag_vault.load_pdf_chunks_index();
        let rag_active_file = rag_pdf_chunks.first().map(|c| c.file_name.clone());

        let personalization =
            mindset_intelligence::PersonalizationEngine::new(storage.db_path().to_path_buf())
                .unwrap_or_default();
        let today_checkin = personalization
            .get_fatigue_checkin(&today_str)
            .ok()
            .flatten();
        let due_leitner_cards = personalization
            .get_due_leitner_cards(&today_str)
            .unwrap_or_default();
        let unresolved_mistakes = personalization
            .get_unresolved_mistakes(None)
            .unwrap_or_default();

        let mut mcq_generator = mindset_core::agent::CompetitiveMcqGenerator::new();
        let bench_dir = std::path::Path::new("tests/benchmarks/mcq_datasets");
        if bench_dir.exists() {
            let _ = mcq_generator.load_benchmarks(bench_dir);
        }
        let study_capsules = ai::MaindySystemBridge::load_saved_capsules();
        let active_capsule_id = study_capsules.first().map(|c| c.id.clone());
        for (w_id_str, layout) in saved_layouts {
            let id = match w_id_str.as_str() {
                "ScholarBriefing" => Some(mindset_core::DesktopWidgetId::ScholarBriefing),
                "OdysseyMap" | "OdysseyTrail" => Some(mindset_core::DesktopWidgetId::OdysseyTrail),
                "Scratchpad" => Some(mindset_core::DesktopWidgetId::Scratchpad),
                "VerificationBoss" => Some(mindset_core::DesktopWidgetId::VerificationBoss),
                "QuickTutor" => Some(mindset_core::DesktopWidgetId::QuickTutor),
                _ => None,
            };
            if let Some(id) = id {
                if let Some(placement) = core.wm.get_widget_placement_mut(id) {
                    placement.x = layout.pos_x;
                    placement.y = layout.pos_y;
                    placement.width = layout.width;
                    placement.height = layout.height;
                    placement.is_pinned = layout.is_pinned;
                }
            }
        }

        let installed_ms_ids: Vec<String> = storage
            .list_installed_ms_apps()
            .into_iter()
            .map(|(id, _, _, _, _, _)| id)
            .collect();
        let mindstore_state = apps::mindstore::MindStoreState {
            installed_ids: installed_ms_ids,
            ..Default::default()
        };

        Self {
            core,
            system,
            storage,
            sysinfo: system::SysinfoCollector::new(),
            wm_interaction: wm::WindowInteractionState::new(),
            youtube_study: apps::youtube_study::YouTubeStudyState::default(),
            mindstore_state,
            search_query: String::new(),
            chat_input: String::new(),
            active_thread_id,
            threads,
            active_messages,
            is_generating: false,
            active_model: "gemma2:2b".to_string(),
            installed_models: vec!["gemma2:2b".to_string()],
            cancel_flag: None,
            new_task_input: String::new(),
            feynman_input: String::new(),
            stage1_selections: vec![None, None, None],
            stage2_selections: vec![None, None],
            excuse_selected_answer: None,
            wallpaper_handle,
            profile,
            capsules,
            memories,
            new_memory_input: String::new(),
            onboarding_data,
            dock_springs: std::array::from_fn(|_| {
                mindset_motion::Spring1D::with_preset(
                    1.0,
                    mindset_motion::SpringPreset::SnappyMicro,
                )
            }),
            hovered_dock_index: None,
            hud_visible: true,
            smoke_frames_remaining: None,
            maindy_spring: mindset_motion::Spring1D::with_preset(
                1.0,
                mindset_motion::SpringPreset::SnappyMicro,
            ),
            maindy_drawer_open: false,
            island_mode: shell::dynamic_island::IslandMode::Compact,
            island_width_spring: mindset_motion::Spring1D::with_preset(
                290.0,
                mindset_motion::SpringPreset::SnappyMicro,
            ),
            island_height_spring: mindset_motion::Spring1D::with_preset(
                36.0,
                mindset_motion::SpringPreset::SnappyMicro,
            ),
            island_search_query: String::new(),
            odyssey_map_state: shell::odyssey_map::OdysseyMapState::default(),
            tunnel_vision_active: false,
            tunnel_vision_spring: mindset_motion::Spring1D::with_preset(
                1.0,
                mindset_motion::SpringPreset::DeepFocus,
            ),
            audio_player: system::AudioPlayer::new(),
            last_anim_instant: None,
            cursor_position: iced::Point::ORIGIN,
            i18n,
            auto_speak_enabled: false,
            boot_splash: shell::boot_splash::BootSplashState::new(),
            pending_agent_action: None,
            session_allowed_folders: std::collections::HashSet::new(),
            ai_activity_log,
            browser_url: "https://www.youtube.com".to_string(),
            browser_focus_shield: true,
            browser_history: vec!["https://www.youtube.com".to_string()],
            browser_history_idx: 0,
            browser_status_note: None,
            network_status: mindset_sys::network::NetworkManager::get_status(),
            nearby_networks: Vec::new(),
            wifi_menu_open: false,
            wifi_connecting_ssid: None,
            wifi_password_input: String::new(),
            lock_screen: shell::lock_screen::LockScreenState::default(),
            wallpaper_canvas_cache: iced::widget::canvas::Cache::default(),
            rag_vault,
            rag_pdf_chunks,
            rag_active_file,
            rag_current_page: 1,
            rag_query_input: String::new(),
            rag_citations: Vec::new(),
            academic_search: mindset_intelligence::AcademicSearchEngine::new(),
            academic_search_input: String::new(),
            academic_search_provider: mindset_intelligence::SearchProviderType::Encyclopedia,
            academic_search_loading: false,
            academic_search_error: None,
            personalization,
            today_checkin,
            due_leitner_cards,
            unresolved_mistakes,
            last_parsed_query: None,
            active_exam_evaluation: None,
            exam_eval_status: None,
            mcq_generator,
            active_mcq_paper: None,
            mcq_answers: std::collections::HashMap::new(),
            mcq_report: None,
            maindy_screen: apps::ai_tutor::MaindyScreen::TutorStudio,
            maindy_drawer_tab: apps::ai_tutor::MaindyDrawerTab::Capsules,
            maindy_left_rail_open: true,
            maindy_right_drawer_open: true,
            maindy_engine_popover_open: false,
            study_capsules,
            active_capsule_id,
            active_capsule_phase: 0,
            capsule_hint_open: false,
            pending_destructive_action: None,
        }
    }

    pub fn new() -> (Self, Task<Message>) {
        Self::new_with_options(LaunchOptions::default())
    }

    pub fn new_with_options(opts: LaunchOptions) -> (Self, Task<Message>) {
        let mut app = Self::default();
        app.wm_interaction.screen_width = opts.window_size.width;
        app.wm_interaction.screen_height = opts.window_size.height;
        app.smoke_frames_remaining = opts.smoke_frames;

        if opts.skip_boot {
            app.boot_splash.skip();
            app.profile.onboarding_completed = true;
        }
        if let Some(theme_mode) = opts.theme_override {
            app.core.preferences.theme = theme_mode;
            app.profile.theme_mode = match theme_mode {
                ThemeMode::AlpineGlass => "AlpineMatteLight".to_string(),
                _ => "ObsidianMatteDark".to_string(),
            };
        }
        if let Some(wp) = opts.wallpaper_override {
            match wp.as_str() {
                "none" | "procedural" | "bright" | "dark" | "busy" => {
                    app.wallpaper_handle = None;
                    app.wallpaper_canvas_cache.clear();
                    if wp == "bright" && opts.theme_override.is_none() {
                        app.core.preferences.theme = ThemeMode::AlpineGlass;
                        app.profile.theme_mode = "AlpineMatteLight".to_string();
                    } else if wp == "dark" && opts.theme_override.is_none() {
                        app.core.preferences.theme = ThemeMode::DarkSpace;
                        app.profile.theme_mode = "ObsidianMatteDark".to_string();
                    }
                }
                path_str => {
                    if std::path::Path::new(path_str).exists() {
                        app.wallpaper_handle =
                            Some(iced::widget::image::Handle::from_path(path_str));
                    }
                }
            }
        }

        // Asynchronously discover installed Ollama models on boot
        let discover_task = Task::perform(
            async {
                ai::OllamaEngine::list_models("127.0.0.1:11434")
                    .await
                    .unwrap_or_default()
            },
            Message::ModelsDiscovered,
        );
        let network_task = Task::perform(
            async { mindset_sys::network::NetworkManager::get_status() },
            Message::NetworkStatusUpdated,
        );

        (app, Task::batch([discover_task, network_task]))
    }

    pub fn title(&self) -> String {
        "MindSet OS — Uninterrupted Deep Focus".to_string()
    }

    pub fn scale_factor(&self) -> f32 {
        self.core.preferences.ui_scale
    }

    pub fn execute_agent_action(
        &mut self,
        action: mindset_intelligence::agent_tools::AgentAction,
    ) -> Task<Message> {
        match action {
            mindset_intelligence::agent_tools::AgentAction::CreateAndShowCapsule {
                subject,
                topic,
                duration_mins,
                priority,
                notes_summary,
            } => {
                let id = format!("capsule-{}", uuid::Uuid::new_v4().simple());
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let cap = storage::db::CapsuleRecord {
                    id: id.clone(),
                    target_date: today_str.clone(),
                    subject: subject.clone(),
                    unit: topic.clone(),
                    estimated_minutes: duration_mins,
                    xp_reward: 50,
                    status: "In Progress".to_string(),
                    stage: "Heavy Lift".to_string(),
                };
                self.storage.save_capsule(&cap);
                self.capsules = self.storage.list_capsules(&today_str);

                // Synchronize with 3D Syllabus Odyssey Journey
                let node_id = format!("node-{}", uuid::Uuid::new_v4().simple());
                let odyssey_node = mindset_core::odyssey::OdysseyNode {
                    id: node_id,
                    unit_id: "unit-active-capsules".to_string(),
                    unit_title: "Active Study Capsules".to_string(),
                    title: topic.clone(),
                    subject: subject.clone(),
                    description: notes_summary.clone(),
                    node_type: mindset_core::odyssey::NodeType::Theory,
                    status: mindset_core::odyssey::NodeStatus::InProgress,
                    marks: 0,
                    max_marks: 100,
                    xp: 50,
                    stars_earned: 0,
                    is_crown_mastered: false,
                    estimated_minutes: duration_mins,
                };
                self.core.odyssey.nodes.push(odyssey_node);
                self.odyssey_map_state.static_road_cache.clear();
                self.odyssey_map_state.dynamic_ambient_cache.clear();

                // Launch Study Session Reader immediately
                self.core.wm.open(WindowId::StudyReader);

                // Post confirmation into chat
                let confirmation = format!(
                    "✨ **Study Capsule Created & Launched!**\n\n\
                    • **Topic:** {}\n\
                    • **Subject:** {}\n\
                    • **Target Duration:** {} mins\n\
                    • **Priority:** {}\n\n\
                    *Capsule has been added to today's Queue and the 3D Syllabus Odyssey Trail.*",
                    topic, subject, duration_mins, priority
                );
                self.storage
                    .save_message(&self.active_thread_id, "assistant", &confirmation);
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);

                self.core.toast_queue.push(
                    "Capsule Ready",
                    format!("{} ({}) is now active in Study Reader.", topic, subject),
                    "⚡",
                );
            }
            mindset_intelligence::agent_tools::AgentAction::ListFolder { path } => {
                let listing_res = ai::agent::safe_list_folder(&path);
                match listing_res {
                    Ok(files) => {
                        let reply = if files.is_empty() {
                            format!("📂 **Folder Listing:** `{}` is empty.", path)
                        } else {
                            let mut s = format!(
                                "📂 **Folder Listing:** `{}` ({} items):\n\n",
                                path,
                                files.len()
                            );
                            for f in files {
                                s.push_str(&format!("• {}\n", f));
                            }
                            s
                        };
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                    Err(err) => {
                        self.core.toast_queue.warning(
                            "Study Vault Folder Unavailable",
                            format!("Could not list `{path}`: {err}"),
                            Some("Open Vault Folder"),
                            Some(mindset_core::NotificationAction::OpenVaultFolder),
                        );
                    }
                }
            }
            mindset_intelligence::agent_tools::AgentAction::ReadDocument { file_path } => {
                let read_res = ai::agent::safe_read_document(&file_path);
                match read_res {
                    Ok((content, is_truncated)) => {
                        let trunc_note = if is_truncated {
                            "\n*(Content capped at 16 KB)*"
                        } else {
                            ""
                        };
                        let reply = format!(
                            "📄 **Document Read Complete:** `{}` ({} bytes read){}\n\n\
                            ```text\n{}\n```\n\n\
                            💡 *I've loaded this document into context and can now answer any questions about it!*",
                            file_path,
                            content.len(),
                            trunc_note,
                            content
                        );
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                    Err(err) => {
                        self.core.toast_queue.warning(
                            "Study Vault File Missing",
                            format!("Could not open `{file_path}`: {err}"),
                            Some("Open Vault Folder"),
                            Some(mindset_core::NotificationAction::OpenVaultFolder),
                        );
                    }
                }
            }
            mindset_intelligence::agent_tools::AgentAction::OpenApp { app_id } => {
                let lower = app_id.to_lowercase();
                match lower.as_str() {
                    "youtube" | "youtube_study" | "yt" => {
                        match mindset_net::ChromiumEngineBridge::spawn_youtube_app() {
                            Ok(msg) => {
                                let reply = format!("📺 Launched **Official YouTube App**: {msg}");
                                self.storage.save_message(
                                    &self.active_thread_id,
                                    "assistant",
                                    &reply,
                                );
                                self.active_messages = self
                                    .storage
                                    .load_recent_messages(&self.active_thread_id, 30);
                            }
                            Err(e) => {
                                self.core.toast_queue.warning(
                                    "YouTube App Unavailable",
                                    format!("Could not launch YouTube app: {e}"),
                                    Some("Open Settings"),
                                    Some(mindset_core::NotificationAction::OpenSettings),
                                );
                            }
                        }
                    }
                    "freetube" => match mindset_net::ChromiumEngineBridge::spawn_freetube_app() {
                        Ok(msg) => {
                            let reply = format!("🛡️ Launched **FreeTube (Ad-Free)**: {msg}");
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", &reply);
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
                        }
                        Err(e) => {
                            self.core.toast_queue.warning(
                                "FreeTube Unavailable",
                                format!("Could not launch FreeTube: {e}"),
                                Some("Open Settings"),
                                Some(mindset_core::NotificationAction::OpenSettings),
                            );
                        }
                    },
                    "browser" | "mindset_browser" | "chromium" => {
                        match mindset_net::ChromiumEngineBridge::spawn_mindset_browser(None) {
                            Ok(msg) => {
                                let reply = format!("🌐 Launched **MindSet Browser**: {msg}");
                                self.storage.save_message(
                                    &self.active_thread_id,
                                    "assistant",
                                    &reply,
                                );
                                self.active_messages = self
                                    .storage
                                    .load_recent_messages(&self.active_thread_id, 30);
                            }
                            Err(e) => {
                                self.core.toast_queue.warning(
                                    "Browser Launch Unavailable",
                                    format!("Could not launch external browser: {e}"),
                                    Some("Open Settings"),
                                    Some(mindset_core::NotificationAction::OpenSettings),
                                );
                            }
                        }
                    }
                    _ => {
                        let win_id = match lower.as_str() {
                            "file_vault" | "filevault" | "vault" | "files" => {
                                Some(WindowId::FileVault)
                            }
                            "system_monitor" | "resource_monitor" | "sysinfo" => {
                                Some(WindowId::SystemMonitor)
                            }
                            "settings" | "preferences" => Some(WindowId::Settings),
                            "study_reader" | "reader" | "study" => Some(WindowId::StudyReader),
                            "tasks" | "todo" => Some(WindowId::Tasks),
                            "search" => Some(WindowId::SearchBrowser),
                            "design_system" => Some(WindowId::DesignSystem),
                            _ => None,
                        };

                        if let Some(wid) = win_id {
                            self.core.wm.open(wid);
                            let reply = format!("🚀 Opened **{}** application window.", app_id);
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", &reply);
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
                        } else {
                            self.core.toast_queue.info(
                                "App Not Registered",
                                format!("No built-in MindSet window matches `{app_id}`."),
                            );
                        }
                    }
                }
            }
            mindset_intelligence::agent_tools::AgentAction::ControlSystem { action } => {
                match action {
                    mindset_intelligence::agent_tools::SystemControlAction::ToggleTheme(
                        theme_name,
                    ) => {
                        let lower = theme_name.to_lowercase();
                        if lower.contains("light") || lower.contains("alpine") {
                            self.core.preferences.theme = mindset_core::ThemeMode::AlpineGlass;
                        } else if lower.contains("violet") || lower.contains("nebula") {
                            self.core.preferences.theme = mindset_core::ThemeMode::VioletNebula;
                        } else if lower.contains("amber") || lower.contains("solar") {
                            self.core.preferences.theme = mindset_core::ThemeMode::SolarAmber;
                        } else {
                            self.core.preferences.theme = mindset_core::ThemeMode::DarkSpace;
                        }
                        let reply = format!(
                            "🎨 System theme switched to **{:?}**.",
                            self.core.preferences.theme
                        );
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                    mindset_intelligence::agent_tools::SystemControlAction::SwitchLanguage(
                        lang,
                    ) => {
                        self.profile.maindy_style = lang.clone();
                        let _ = self.storage.save_profile(&self.profile);
                        let reply =
                            format!("🌐 Maindy AI language preference set to **{}**.", lang);
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                    mindset_intelligence::agent_tools::SystemControlAction::StartPomodoro {
                        duration_mins,
                    } => {
                        self.core.pomodoro.set_duration(duration_mins);
                        self.core.pomodoro.remaining_secs = (duration_mins * 60) as f32;
                        self.core.pomodoro.is_running = true;
                        self.core.pomodoro.is_break = false;
                        let reply = format!(
                            "⏱️ Focus Pomodoro session started for **{} minutes**.",
                            duration_mins
                        );
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                    mindset_intelligence::agent_tools::SystemControlAction::OpenBrowserUrl(url) => {
                        match mindset_net::ChromiumEngineBridge::spawn_mindset_browser(Some(&url)) {
                            Ok(msg) => {
                                let reply = format!(
                                    "🌐 Launched MindSet Browser to destination `{url}` ({msg})"
                                );
                                self.storage.save_message(
                                    &self.active_thread_id,
                                    "assistant",
                                    &reply,
                                );
                                self.active_messages = self
                                    .storage
                                    .load_recent_messages(&self.active_thread_id, 30);
                            }
                            Err(e) => {
                                self.core.toast_queue.warning(
                                    "Browser Launch Unavailable",
                                    format!("Could not open `{url}`: {e}"),
                                    Some("Open Settings"),
                                    Some(mindset_core::NotificationAction::OpenSettings),
                                );
                            }
                        }
                    }
                }
            }
        }
        Task::none()
    }

    pub fn handle_maindy_os_action(
        &mut self,
        action: ai::MaindyOsAction,
        clean_reply: Option<String>,
        execute_destructive: bool,
    ) -> Task<Message> {
        if action.is_destructive() && !execute_destructive {
            self.pending_destructive_action = Some(action.clone());
            self.core.wm.open(WindowId::LayaAiChat);
            self.maindy_screen = apps::ai_tutor::MaindyScreen::TutorStudio;
            let msg_text = clean_reply.unwrap_or_else(|| {
                format!(
                    "Please review the confirmation card below before running `{}`.",
                    action.human_label()
                )
            });
            self.storage
                .save_message(&self.active_thread_id, "assistant", &msg_text);
            self.active_messages = self
                .storage
                .load_recent_messages(&self.active_thread_id, 30);
            return Task::none();
        }

        let mut suppress_clean_reply = false;

        match action {
            ai::MaindyOsAction::OpenSettings { section } => {
                let tab_idx = match section {
                    Some(ai::SettingsSection::FocusGuard) => 1,
                    Some(ai::SettingsSection::Wifi)
                    | Some(ai::SettingsSection::Bluetooth)
                    | Some(ai::SettingsSection::Audio) => 2,
                    Some(ai::SettingsSection::Display)
                    | Some(ai::SettingsSection::General)
                    | None => 0,
                };
                self.core.preferences.set_active_tab_index(tab_idx);
                self.core.wm.open(WindowId::Settings);
                self.core.wm.close_overlays();
                self.core.toast_queue.success(
                    "Settings Opened",
                    "Maindy navigated directly to MindSet Preferences.",
                );
            }
            ai::MaindyOsAction::OpenFiles { path } => {
                let target = path.unwrap_or_else(ai::maindy_system_bridge::vault_dir);
                apps::file_explorer::open_directory(&target);
                self.core.wm.open(WindowId::FileVault);
                self.core.wm.close_overlays();
                self.core.toast_queue.success(
                    "Study Vault Ready",
                    format!("Opened `{}` in Student File Vault.", target.display()),
                );
            }
            ai::MaindyOsAction::LaunchApp { app_id } => {
                let lower = app_id.to_lowercase();
                match lower.as_str() {
                    "youtube" | "youtube_study" | "yt" => {
                        match mindset_net::ChromiumEngineBridge::spawn_youtube_app() {
                            Ok(_) => {
                                self.core.toast_queue.success(
                                    "YouTube Launched",
                                    "Opened dedicated distraction-free YouTube Study player.",
                                );
                            }
                            Err(e) => {
                                suppress_clean_reply = true;
                                self.core.toast_queue.warning(
                                    "YouTube App Unavailable",
                                    format!("Could not launch external YouTube app: {e}"),
                                    Some("Open Settings"),
                                    Some(mindset_core::NotificationAction::OpenSettings),
                                );
                            }
                        }
                    }
                    "google" | "browser" | "search" | "mindset_browser" => {
                        self.core.wm.open(WindowId::SearchBrowser);
                        self.core.wm.close_overlays();
                        self.core.toast_queue.success(
                            "MindStrict Browser Ready",
                            "Safe distraction-shielded academic browser opened.",
                        );
                    }
                    "study_reader" | "reader" => {
                        self.core.wm.open(WindowId::StudyReader);
                        self.core.wm.close_overlays();
                        self.core
                            .toast_queue
                            .success("Study Reader Opened", "Interactive reader is now active.");
                    }
                    "odyssey_map" | "odyssey" => {
                        self.core.wm.open(WindowId::OdysseyMap);
                        self.core.wm.close_overlays();
                        self.core.toast_queue.success(
                            "3D Odyssey Opened",
                            "Navigated to your syllabus milestone trail.",
                        );
                    }
                    "scratchpad" => {
                        self.core.wm.open(WindowId::Scratchpad);
                        self.core.wm.close_overlays();
                        self.core
                            .toast_queue
                            .success("Scratchpad Ready", "Quick study notes opened.");
                    }
                    "app_center" | "tasks" => {
                        self.core.wm.open(WindowId::Tasks);
                        self.core.wm.close_overlays();
                        self.core.toast_queue.success(
                            "Syllabus Goals Ready",
                            "Opened your daily targets and study backlog.",
                        );
                    }
                    "file_vault" | "files" => {
                        let vault = ai::maindy_system_bridge::vault_dir();
                        apps::file_explorer::open_directory(&vault);
                        self.core.wm.open(WindowId::FileVault);
                        self.core.wm.close_overlays();
                        self.core.toast_queue.success(
                            "File Vault Opened",
                            "Opened your Linux Study Vault workspace.",
                        );
                    }
                    "settings" | "preferences" => {
                        self.core.wm.open(WindowId::Settings);
                        self.core.wm.close_overlays();
                        self.core
                            .toast_queue
                            .success("Settings Opened", "Opened MindSet OS Preferences.");
                    }
                    other => {
                        suppress_clean_reply = true;
                        self.core.toast_queue.info(
                            "App Not Found",
                            format!("No MindSet application named `{other}` is installed."),
                        );
                    }
                }
            }
            ai::MaindyOsAction::ToggleSystemControl { control, enabled } => {
                match control {
                    ai::SystemControl::Wifi => {
                        self.core.preferences.wifi_enabled = enabled;
                    }
                    ai::SystemControl::Bluetooth => {
                        self.core.preferences.bluetooth_enabled = enabled;
                    }
                    ai::SystemControl::DoNotDisturb => {
                        self.core.preferences.focus_dnd = enabled;
                    }
                    ai::SystemControl::AirplaneMode => {
                        self.core.preferences.airplane_mode = enabled;
                        if enabled {
                            self.core.preferences.wifi_enabled = false;
                            self.core.preferences.bluetooth_enabled = false;
                        }
                    }
                    ai::SystemControl::DarkTheme => {
                        self.core.preferences.theme = if enabled {
                            mindset_core::ThemeMode::DarkSpace
                        } else {
                            mindset_core::ThemeMode::AlpineGlass
                        };
                    }
                    ai::SystemControl::AudioMute => {
                        self.core.preferences.volume = if enabled { 0.0 } else { 0.8 };
                    }
                }
                ai::MaindySystemBridge::apply_debian_dbus_control(control, enabled);
                self.core.toast_queue.success(
                    "System Setting Updated",
                    format!(
                        "{control:?} switched {}.",
                        if enabled { "ON" } else { "OFF" }
                    ),
                );
            }
            ai::MaindyOsAction::CreateAndPresentCapsule {
                topic,
                subject,
                duration_mins,
            } => {
                let cap = ai::MaindySystemBridge::build_capsule(
                    &topic,
                    subject.as_deref(),
                    duration_mins,
                );
                if let Err(err) = ai::MaindySystemBridge::save_capsule(&cap) {
                    self.core.toast_queue.error(
                        "Capsule Storage Notice",
                        err,
                        Some("Open Vault Folder"),
                        Some(mindset_core::NotificationAction::OpenVaultFolder),
                    );
                }

                let db_cap = cap.to_db_record();
                self.storage.save_capsule(&db_cap);
                self.capsules.retain(|c| c.id != db_cap.id);
                self.capsules.insert(0, db_cap);

                self.study_capsules.retain(|c| c.id != cap.id);
                self.study_capsules.insert(0, cap.clone());
                self.active_capsule_id = Some(cap.id.clone());
                self.active_capsule_phase = 0;
                self.capsule_hint_open = false;
                self.maindy_screen = apps::ai_tutor::MaindyScreen::CapsuleTheater;
                self.core.wm.open(WindowId::LayaAiChat);
                self.core.wm.close_overlays();

                self.core.toast_queue.push_with_action(
                    mindset_core::NotificationSeverity::Success,
                    "Study Capsule Ready",
                    format!(
                        "{} ({} mins) launched in Capsule Theater.",
                        cap.title, cap.duration_mins
                    ),
                    "⚡",
                    Some("Open Capsule"),
                    Some(mindset_core::NotificationAction::OpenCapsule(
                        cap.id.clone(),
                    )),
                );
            }
            ai::MaindyOsAction::StartFocusSprint {
                capsule_id,
                minutes,
            } => {
                let mins = minutes.clamp(5, 120) as u32;
                self.core.pomodoro.set_duration(mins);
                self.core.pomodoro.remaining_secs = (mins * 60) as f32;
                self.core.pomodoro.is_running = true;
                self.core.pomodoro.is_break = false;
                self.core.preferences.focus_dnd = true;
                self.island_mode = shell::dynamic_island::IslandMode::ActiveFocus;
                let (tw, th) = self.island_mode.target_dimensions(0);
                self.island_width_spring.set_target(tw);
                self.island_height_spring.set_target(th);
                if self.study_capsules.iter().any(|c| c.id == capsule_id) {
                    self.active_capsule_id = Some(capsule_id);
                }
                self.core.toast_queue.success(
                    "Focus Sprint Active",
                    format!("{mins}-minute focus timer running with Do Not Disturb ON."),
                );
            }
            ai::MaindyOsAction::ReadVaultFile { file_name } => {
                suppress_clean_reply = true;
                if let Some(resolved) = ai::MaindySystemBridge::resolve_vault_file(&file_name) {
                    match std::fs::read_to_string(&resolved) {
                        Ok(content) => {
                            let preview: String = content.chars().take(3000).collect();
                            let reply = format!(
                                "📄 **Loaded Note:** `{}`\n\n```text\n{}\n```",
                                file_name, preview
                            );
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", &reply);
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
                            self.core.toast_queue.success(
                                "Study Note Loaded",
                                format!("Maindy read `{}` from your Linux Vault.", file_name),
                            );
                        }
                        Err(err) => {
                            self.core.toast_queue.warning(
                                "Cannot Read Vault File",
                                format!("Could not read `{}`: {err}", file_name),
                                Some("Open Vault Folder"),
                                Some(mindset_core::NotificationAction::OpenVaultFolder),
                            );
                        }
                    }
                } else {
                    self.core.toast_queue.warning(
                        "Study Vault File Missing",
                        format!(
                            "`{}` was not found in ~/.local/share/mindset-os/vault/.",
                            file_name
                        ),
                        Some("Open Vault Folder"),
                        Some(mindset_core::NotificationAction::OpenVaultFolder),
                    );
                }
            }
            ai::MaindyOsAction::DeleteVaultFile { path } => {
                suppress_clean_reply = true;
                self.pending_destructive_action = None;
                if path.exists() {
                    match std::fs::remove_file(&path) {
                        Ok(()) => {
                            self.core.toast_queue.success(
                                "Vault File Deleted",
                                format!("Removed `{}`.", path.display()),
                            );
                        }
                        Err(e) => {
                            self.core.toast_queue.error(
                                "File Delete Failed",
                                format!("Could not delete `{}`: {e}", path.display()),
                                Some("Open Vault Folder"),
                                Some(mindset_core::NotificationAction::OpenVaultFolder),
                            );
                        }
                    }
                } else {
                    self.core.toast_queue.warning(
                        "File Not Present",
                        format!("`{}` was already removed or missing.", path.display()),
                        Some("Open Vault Folder"),
                        Some(mindset_core::NotificationAction::OpenVaultFolder),
                    );
                }
            }
            ai::MaindyOsAction::PowerOffConfirm => {
                self.pending_destructive_action = None;
                return self.update(Message::SystemPowerOff);
            }
        }

        if !suppress_clean_reply {
            if let Some(reply_text) = clean_reply {
                let trimmed = reply_text.trim();
                if !trimmed.is_empty() {
                    self.storage
                        .save_message(&self.active_thread_id, "assistant", trimmed);
                    self.active_messages = self
                        .storage
                        .load_recent_messages(&self.active_thread_id, 30);
                }
            }
        }

        Task::none()
    }

    pub fn collapse_island(&mut self) {
        if self.island_mode != shell::dynamic_island::IslandMode::Compact {
            self.island_mode = if self.tunnel_vision_active {
                shell::dynamic_island::IslandMode::ActiveFocus
            } else {
                shell::dynamic_island::IslandMode::Compact
            };
            let (tw, th) = self.island_mode.target_dimensions(0);
            self.island_width_spring.set_target(tw);
            self.island_height_spring.set_target(th);
            self.island_search_query.clear();
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => {}
            Message::Tick => {
                self.core.tick(1);
                if self.lock_screen.lockout_seconds_remaining > 0 {
                    self.lock_screen.lockout_seconds_remaining -= 1;
                    if self.lock_screen.lockout_seconds_remaining == 0 {
                        self.lock_screen.is_locked_out = false;
                        self.lock_screen.failed_attempts = 0;
                        self.lock_screen.error_message =
                            Some("Account unlocked. You may try again.".into());
                    } else {
                        self.lock_screen.error_message = Some(format!(
                            "Account locked due to 5 failed attempts. Retry in {}s",
                            self.lock_screen.lockout_seconds_remaining
                        ));
                    }
                }
                if self.island_mode == shell::dynamic_island::IslandMode::MaindySpeaking
                    && !ai::TtsEngine::is_speaking()
                {
                    self.island_mode = shell::dynamic_island::IslandMode::Compact;
                    let (tw, th) = self.island_mode.target_dimensions(0);
                    self.island_width_spring.set_target(tw);
                    self.island_height_spring.set_target(th);
                }
            }
            Message::SysinfoTick => {
                self.sysinfo.refresh();
                if self.sysinfo.metrics.ollama_detected {
                    self.system.ollama_status = system::OllamaStatus::Online;
                } else if self.is_generating {
                    self.system.ollama_status = system::OllamaStatus::Loading;
                } else {
                    self.system.ollama_status = system::OllamaStatus::Idle;
                }
                return Task::perform(
                    async { mindset_sys::network::NetworkManager::get_status() },
                    Message::NetworkStatusUpdated,
                );
            }
            Message::ScaleChanged(scale) => {
                self.core.preferences.set_ui_scale(scale);
            }
            Message::BrightnessChanged(val) => {
                self.core.preferences.set_brightness(val);
            }
            Message::VolumeChanged(val) => {
                self.core.preferences.set_volume(val);
            }
            Message::ThemeSelected(mode) => {
                self.core.preferences.theme = mode;
                self.profile.theme_mode = match mode {
                    mindset_core::ThemeMode::AlpineGlass => "AlpineMatteLight".to_string(),
                    mindset_core::ThemeMode::DarkSpace => "DarkSpace".to_string(),
                    mindset_core::ThemeMode::VioletNebula => "VioletNebula".to_string(),
                    mindset_core::ThemeMode::SolarAmber => "SolarAmber".to_string(),
                };
                let _ = self.storage.save_profile(&self.profile);
            }
            Message::ToggleWifi => {
                self.core.preferences.toggle_wifi();
                let enabled = self.core.preferences.wifi_enabled;
                let _ = mindset_sys::network::NetworkManager::set_wifi_enabled(enabled);
                return Task::perform(
                    async { mindset_sys::network::NetworkManager::get_status() },
                    Message::NetworkStatusUpdated,
                );
            }
            Message::ToggleBluetooth => {
                self.core.preferences.toggle_bluetooth();
            }
            Message::ToggleAirplaneMode => {
                self.core.preferences.toggle_airplane_mode();
                let is_on = self.core.preferences.airplane_mode;
                self.core.toast_queue.push(
                    if is_on {
                        "Airplane Mode Enabled"
                    } else {
                        "Airplane Mode Disabled"
                    },
                    if is_on {
                        "Wi-Fi & Bluetooth radios powered down"
                    } else {
                        "Radios restored"
                    },
                    "✈️",
                );
            }
            Message::ToggleDnd => {
                self.core.preferences.toggle_dnd();
            }
            Message::ToggleDesktop => {
                self.core.preferences.toggle_desktop();
            }
            Message::OpenWindow(id) => {
                self.core.wm.open(id);
                self.core.wm.close_overlays();
                if let Some(win) = self.core.wm.get_window_mut(id) {
                    self.wm_interaction.clamp_window(win);
                }
                if id == WindowId::StudyReader && !self.tunnel_vision_active {
                    self.tunnel_vision_active = true;
                    self.tunnel_vision_spring.set_target(0.0);
                }
            }
            Message::CloseWindow(id) => {
                self.core.wm.close(id);
                if id == WindowId::StudyReader && self.tunnel_vision_active {
                    self.tunnel_vision_active = false;
                    self.tunnel_vision_spring.set_target(1.0);
                    self.island_mode = shell::dynamic_island::IslandMode::Compact;
                    let (w, h) = self.island_mode.target_dimensions(0);
                    self.island_width_spring.set_target(w);
                    self.island_height_spring.set_target(h);
                    self.core.toast_queue.push(
                        "Tunnel Vision Disengaged",
                        "Study session ended. Peripheral widgets restored.",
                        "🎯",
                    );
                }
            }
            Message::MinimizeWindow(id) => {
                self.core.wm.minimize(id);
                if id == WindowId::StudyReader && self.tunnel_vision_active {
                    self.tunnel_vision_active = false;
                    self.tunnel_vision_spring.set_target(1.0);
                    self.island_mode = shell::dynamic_island::IslandMode::Compact;
                    let (w, h) = self.island_mode.target_dimensions(0);
                    self.island_width_spring.set_target(w);
                    self.island_height_spring.set_target(h);
                }
            }
            Message::MaximizeWindow(id) => {
                let w = self.wm_interaction.screen_width;
                let h = self.wm_interaction.screen_height;
                self.core.wm.toggle_maximize(id, w, h);
            }
            Message::FocusWindow(id) => {
                self.core.wm.bring_to_front(id);
                self.core.wm.close_overlays();
            }
            Message::StartWindowDrag(id) => {
                self.core.wm.bring_to_front(id);
                self.core.wm.close_overlays();
                if let Some(win) = self.core.wm.get_window_mut(id) {
                    if win.mode == WindowMode::Normal {
                        let offset_x = (self.cursor_position.x - win.x).max(0.0);
                        let offset_y = (self.cursor_position.y - win.y).max(0.0);
                        self.wm_interaction.dragging = Some((id, offset_x, offset_y));
                    }
                }
            }
            Message::EndWindowDrag => {
                if let Some((id, _, _)) = self.wm_interaction.dragging.take() {
                    if let Some(win) = self.core.wm.get_window_mut(id) {
                        self.wm_interaction.clamp_window(win);
                    }
                }
            }
            Message::StartWidgetDrag(widget_id) => {
                if let Some(widget) = self.core.wm.get_widget_placement_mut(widget_id) {
                    if !widget.is_pinned {
                        let offset_x = (self.cursor_position.x - widget.x).max(0.0);
                        let offset_y = (self.cursor_position.y - widget.y).max(0.0);
                        self.wm_interaction.dragging_widget = Some((widget_id, offset_x, offset_y));
                        self.core.wm.widget_layout.free_placement = true;
                    }
                }
            }
            Message::EndWidgetDrag => {
                if let Some((widget_id, _, _)) = self.wm_interaction.dragging_widget.take() {
                    if let Some(widget) = self.core.wm.get_widget_placement_mut(widget_id) {
                        self.wm_interaction.clamp_widget(widget);
                        let record = storage::WidgetLayoutRecord {
                            widget_id: format!("{:?}", widget_id),
                            pos_x: widget.x,
                            pos_y: widget.y,
                            width: widget.width,
                            height: widget.height,
                            is_pinned: widget.is_pinned,
                            z_index: 1,
                        };
                        let _ = self.storage.save_widget_layout(&record);
                    }
                }
            }
            Message::ToggleWidgetPin(id) => {
                if let Some(widget) = self.core.wm.get_widget_placement_mut(id) {
                    widget.is_pinned = !widget.is_pinned;
                    let record = storage::WidgetLayoutRecord {
                        widget_id: format!("{:?}", id),
                        pos_x: widget.x,
                        pos_y: widget.y,
                        width: widget.width,
                        height: widget.height,
                        is_pinned: widget.is_pinned,
                        z_index: 1,
                    };
                    let _ = self.storage.save_widget_layout(&record);
                }
            }
            Message::ResetWidgetLayout => {
                self.core.wm.widget_layout.reset();
            }
            Message::ToggleWidgetVisibility(widget_id) => {
                if let Some(w) = self.core.wm.get_widget_placement_mut(widget_id) {
                    w.is_visible = !w.is_visible;
                } else {
                    self.core.wm.widget_layout.placements.push(
                        mindset_core::DesktopWidgetPlacement::new(
                            widget_id, 410.0, 120.0, 460.0, 200.0,
                        ),
                    );
                }
                if let Some(w) = self.core.wm.get_widget_placement(widget_id) {
                    let record = storage::WidgetLayoutRecord {
                        widget_id: format!("{:?}", widget_id),
                        pos_x: w.x,
                        pos_y: w.y,
                        width: w.width,
                        height: w.height,
                        is_pinned: w.is_pinned,
                        z_index: 1,
                    };
                    let _ = self.storage.save_widget_layout(&record);
                }
            }
            Message::DockOverlayAsWidget(overlay) => {
                self.core.wm.close_overlays();
                let widget_id = match overlay {
                    mindset_core::ActiveOverlay::VerificationBoss => {
                        mindset_core::DesktopWidgetId::VerificationBoss
                    }
                    mindset_core::ActiveOverlay::Launcher => {
                        mindset_core::DesktopWidgetId::QuickTutor
                    }
                    _ => return Task::none(),
                };
                if let Some(p) = self.core.wm.get_widget_placement_mut(widget_id) {
                    p.is_visible = true;
                } else {
                    self.core.wm.widget_layout.placements.push(
                        mindset_core::DesktopWidgetPlacement::new(
                            widget_id, 380.0, 120.0, 480.0, 220.0,
                        ),
                    );
                }
                self.core.toast_queue.push(
                    "Widget Docked",
                    "Overlay docked onto editable desktop canvas.",
                    "📌",
                );
            }
            Message::LockScreenPinChanged(pin) => {
                let filtered: String = pin.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
                self.lock_screen.pin_input = filtered;
                if self.lock_screen.pin_input.len() == 6 {
                    return self.update(Message::SubmitLockScreenAuth);
                }
            }
            Message::LockScreenPasswordChanged(pwd) => {
                self.lock_screen.password_input = pwd;
            }
            Message::LockScreenToggleAuthMode => {
                self.lock_screen.mode = match self.lock_screen.mode {
                    shell::lock_screen::LockAuthMode::Pin => {
                        shell::lock_screen::LockAuthMode::Password
                    }
                    shell::lock_screen::LockAuthMode::Password => {
                        shell::lock_screen::LockAuthMode::Pin
                    }
                };
                self.lock_screen.pin_input.clear();
                self.lock_screen.password_input.clear();
                self.lock_screen.error_message = None;
            }
            Message::LockScreenToggleShowPassword => {
                self.lock_screen.show_password = !self.lock_screen.show_password;
            }
            Message::LockScreenKeypadDigit(d) => {
                if !self.lock_screen.is_locked_out && self.lock_screen.pin_input.len() < 6 {
                    self.lock_screen.pin_input.push(d);
                    if self.lock_screen.pin_input.len() == 6 {
                        return self.update(Message::SubmitLockScreenAuth);
                    }
                }
            }
            Message::LockScreenKeypadBackspace => {
                self.lock_screen.pin_input.pop();
            }
            Message::LockScreenKeypadClear => {
                self.lock_screen.pin_input.clear();
            }
            Message::SubmitLockScreenAuth => {
                if self.lock_screen.is_locked_out || self.lock_screen.lockout_seconds_remaining > 0
                {
                    return Task::none();
                }

                match self.lock_screen.mode {
                    shell::lock_screen::LockAuthMode::Pin => {
                        if self.lock_screen.pin_input.len() < 6
                            || !self
                                .lock_screen
                                .pin_input
                                .chars()
                                .all(|c| c.is_ascii_digit())
                        {
                            self.lock_screen.error_message =
                                Some("Enter a 6-digit numeric PIN.".into());
                            return Task::none();
                        }
                    }
                    shell::lock_screen::LockAuthMode::Password => {
                        if self.lock_screen.password_input.trim().is_empty() {
                            self.lock_screen.error_message =
                                Some("Password cannot be blank.".into());
                            return Task::none();
                        }
                    }
                }

                let user_id = "scholar";

                if let Ok(conn) = self.storage.open_connection() {
                    let auth_result = match self.lock_screen.mode {
                        shell::lock_screen::LockAuthMode::Pin => {
                            mindset_sys::auth::CredentialsManager::verify_pin(
                                &conn,
                                user_id,
                                &self.lock_screen.pin_input,
                            )
                        }
                        shell::lock_screen::LockAuthMode::Password => {
                            mindset_sys::auth::CredentialsManager::verify_password(
                                &conn,
                                user_id,
                                &self.lock_screen.password_input,
                            )
                        }
                    };

                    match auth_result {
                        Ok(true) => {
                            self.lock_screen.pin_input.clear();
                            self.lock_screen.password_input.clear();
                            self.lock_screen.error_message = None;
                            self.lock_screen.failed_attempts = 0;
                            self.core.wm.close_overlays();
                            self.core.toast_queue.push(
                                "Workspace Unlocked",
                                "Welcome back, Scholar. Study momentum active.",
                                "🔓",
                            );
                        }
                        Err(mindset_sys::auth::CredentialsError::AccountLocked(secs)) => {
                            self.lock_screen.lockout_seconds_remaining = secs;
                            self.lock_screen.is_locked_out = true;
                            self.lock_screen.error_message = Some(format!(
                                "Account locked due to 5 failed attempts. Retry in {}s",
                                secs
                            ));
                        }
                        Ok(false)
                        | Err(mindset_sys::auth::CredentialsError::InvalidCredentials) => {
                            self.lock_screen.failed_attempts += 1;
                            let remaining = (mindset_sys::auth::credentials::MAX_FAILED_ATTEMPTS
                                - self.lock_screen.failed_attempts)
                                .max(0);
                            self.lock_screen.pin_input.clear();
                            self.lock_screen.password_input.clear();
                            if remaining > 0 {
                                self.lock_screen.error_message = Some(format!(
                                    "Invalid credentials. {} attempt{} remaining.",
                                    remaining,
                                    if remaining == 1 { "" } else { "s" }
                                ));
                            } else {
                                self.lock_screen.lockout_seconds_remaining =
                                    mindset_sys::auth::credentials::LOCKOUT_DURATION_SECS;
                                self.lock_screen.is_locked_out = true;
                                self.lock_screen.error_message = Some(
                                    "Account locked for 30s due to consecutive failed attempts."
                                        .into(),
                                );
                            }
                        }
                        Err(e) => {
                            self.lock_screen.error_message = Some(format!("Auth Error: {}", e));
                        }
                    }
                }
            }
            Message::ToggleOverlay(overlay) => {
                self.core.wm.toggle_overlay(overlay);
                if self.core.wm.active_overlay != ActiveOverlay::None {
                    self.collapse_island();
                    self.maindy_drawer_open = false;
                }
            }
            Message::CloseOverlays => {
                self.core.wm.close_overlays();
                self.collapse_island();
                self.maindy_drawer_open = false;
            }
            Message::ToggleSideHud => {
                self.hud_visible = !self.hud_visible;
                self.core.wm.close_overlays();
            }
            Message::SystemLogout => {
                self.lock_screen.pin_input.clear();
                self.lock_screen.password_input.clear();
                self.lock_screen.error_message = None;
                let _ = self.storage.save_profile(&self.profile);
                self.collapse_island();
                self.maindy_drawer_open = false;
                self.core.wm.close_overlays();
                self.core.wm.toggle_overlay(ActiveOverlay::LockScreen);
                self.core.toast_queue.push(
                    "Session Logged Out",
                    "Workspace locked and study state persisted.",
                    "🔒",
                );
            }
            Message::SystemReboot => {
                let _ = self.storage.save_profile(&self.profile);
                self.core.wm.close_overlays();
                self.collapse_island();
                #[cfg(target_os = "linux")]
                {
                    let _ = std::process::Command::new("systemctl")
                        .arg("reboot")
                        .spawn();
                }
                self.boot_splash = shell::boot_splash::BootSplashState::new();
                self.core.toast_queue.push(
                    "System Reboot Initiated",
                    "Restarting MindSet OS shell cleanly.",
                    "🔄",
                );
            }
            Message::SystemPowerOff => {
                let _ = self.storage.save_profile(&self.profile);
                self.core.wm.close_overlays();
                #[cfg(target_os = "linux")]
                {
                    let _ = std::process::Command::new("systemctl")
                        .arg("poweroff")
                        .spawn();
                }
                return iced::exit();
            }
            Message::StartFocusSession => {
                self.core.pomodoro.start_focus_session();
                if !self.tunnel_vision_active {
                    self.tunnel_vision_active = true;
                    self.tunnel_vision_spring.set_target(0.0);
                }
                self.core.toast_queue.push(
                    "Deep Focus Started",
                    "Mindful Guard active. Syllabus session initiated.",
                    "🎯",
                );
            }
            Message::TogglePomodoro => {
                self.core.pomodoro.toggle();
            }
            Message::CyclePomodoroPreset => {
                self.core.pomodoro.cycle_duration();
            }
            Message::ResetPomodoro => {
                let duration = self.core.pomodoro.duration_mins;
                self.core.pomodoro.set_duration(duration);
            }
            Message::Log10mTask(target) => {
                self.core.tasks.log_10m_for_task(&target);
                self.core.today.log_study_minutes(10);
                self.system.study_screen_time_mins += 10;
                self.core.toast_queue.push(
                    "+10m Logged",
                    format!("Study time recorded for {}", target),
                    "⏱️",
                );
            }
            Message::ToggleTask(id) => {
                self.core.tasks.toggle_task(&id);
            }
            Message::NewTaskInputChanged(val) => {
                self.new_task_input = val;
            }
            Message::AddTask(title, subject, minutes) => {
                if !title.trim().is_empty() {
                    self.core.tasks.add_task(&title, &subject, minutes);
                    self.new_task_input.clear();
                }
            }
            Message::SelectStudyTab(idx) => {
                self.core.study_reader.active_tab_index = idx;
            }
            Message::SelectPreferencesTab(idx) => {
                self.core.preferences.set_active_tab_index(idx);
            }
            Message::VaultSelectSidebar(name) => {
                apps::file_explorer::select_sidebar(&name);
            }
            Message::VaultEntryClicked(name, is_dir) => {
                apps::file_explorer::entry_clicked(&name, is_dir);
            }
            Message::VaultOpenFolder(name) => {
                apps::file_explorer::open_folder(&name);
            }
            Message::VaultNavigateUp => {
                apps::file_explorer::navigate_up();
            }
            Message::VaultSearchChanged(q) => {
                apps::file_explorer::set_search(&q);
            }
            Message::VaultToggleTreeNode(path) => {
                apps::file_explorer::toggle_tree_node(&path);
            }
            Message::VaultPdfPrevPage => {
                apps::file_explorer::pdf_prev_page();
            }
            Message::VaultPdfNextPage => {
                apps::file_explorer::pdf_next_page();
            }
            Message::VaultPdfJumpPage(val) => {
                apps::file_explorer::pdf_jump_page(&val);
            }
            Message::VaultPptxSelectSlide(idx) => {
                apps::file_explorer::pptx_select_slide(idx);
            }
            Message::VaultPptxTitleChanged(val) => {
                apps::file_explorer::pptx_title_changed(&val);
            }
            Message::VaultPptxBodyChanged(val) => {
                apps::file_explorer::pptx_body_changed(&val);
            }
            Message::VaultPptxSaveSlide => {
                apps::file_explorer::pptx_save_slide();
            }
            Message::VaultTextEditChanged(val) => {
                apps::file_explorer::text_edit_changed(&val);
            }
            Message::VaultTextSave => {
                apps::file_explorer::text_save();
            }
            Message::EmbarkNode(id) => {
                self.core.odyssey.embark(&id);
                if let Some(idx) = self.core.odyssey.nodes.iter().position(|n| n.id == id) {
                    self.odyssey_map_state.selected_node = Some(idx);
                }
            }
            Message::CompleteStage(node_id) => {
                if self.core.odyssey.complete_stage(&node_id) {
                    self.audio_player.play_chime();
                    self.odyssey_map_state.static_road_cache.clear();
                    self.odyssey_map_state.dynamic_ambient_cache.clear();
                }
            }
            Message::MindfulStayFocused => {
                self.core.wm.close_overlays();
            }
            Message::MindfulUnlock5m => {
                self.core.mindful_guard.unlock_5m();
                self.core.wm.close_overlays();
                self.core.toast_queue.push(
                    "Temporary 5m Pass",
                    "Mindful Guard paused for 300 seconds.",
                    "⏳",
                );
            }
            Message::SimulateDistraction => {
                self.core
                    .mindful_guard
                    .intercept("Social / Entertaining Feed");
                self.core.wm.toggle_overlay(ActiveOverlay::MindfulGuard);
            }
            Message::WizardNext => {
                self.core.onboarding.next_step();
            }
            Message::WizardPrev => {
                self.core.onboarding.prev_step();
            }
            Message::WizardClose => {
                self.core.onboarding.close();
                self.core.wm.close_overlays();
            }
            Message::SearchInputChanged(query) => {
                self.search_query = query;
            }
            Message::SelectSearchResult(id) => {
                self.core.wm.open(id);
                self.core.wm.close_overlays();
                self.search_query.clear();
            }
            Message::ChatInputChanged(val) => {
                self.chat_input = val;
            }
            Message::SendChatMessage => {
                if !self.chat_input.trim().is_empty() && !self.is_generating {
                    let prompt = self.chat_input.clone();
                    self.chat_input.clear();

                    // 1. Save user query in SQLite
                    self.storage
                        .save_message(&self.active_thread_id, "user", &prompt);
                    self.active_messages = self
                        .storage
                        .load_recent_messages(&self.active_thread_id, 30);

                    // 2. Tier-1 Deterministic OS Control & Capsule Fast-Path (<5ms, zero 2B failures)
                    if let Some(fast) = ai::MaindySystemBridge::route_fast_path(&prompt) {
                        if let Some(action) = fast.action {
                            return self.handle_maindy_os_action(action, fast.clean_reply, false);
                        }
                        if let Some(reply) = fast.clean_reply {
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", &reply);
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
                            return Task::none();
                        }
                    }

                    // 3. Insert placeholder assistant message for live streaming
                    self.active_messages.push(storage::ChatMessageRecord {
                        id: 0,
                        thread_id: self.active_thread_id.clone(),
                        role: "assistant".into(),
                        content: String::new(),
                        created_at: chrono::Local::now().to_rfc3339(),
                    });

                    self.is_generating = true;
                    let parsed_q = ai::persona::ThanglishParser::parse(&prompt);
                    self.last_parsed_query = Some(parsed_q.clone());
                    let grounded_prefix =
                        ai::persona::ThanglishParser::format_academic_prompt(&parsed_q);
                    let rag_query = if let Some(sec) = &parsed_q.section {
                        format!("{} {}", sec, prompt)
                    } else {
                        prompt.clone()
                    };
                    let rag_context = self.rag_vault.build_context_citations(&rag_query, 3);

                    let base_system_prompt = ai::prompt_builder::build_personalized_system_prompt(
                        &self.profile,
                        &self.memories,
                        &self.capsules,
                    );
                    let system_prompt = if rag_context.trim().is_empty() {
                        format!("{}\n\n{}", grounded_prefix, base_system_prompt)
                    } else {
                        format!(
                            "{}\n\n[RAG Textbook Context Citations]\n{}\n{}",
                            grounded_prefix, rag_context, base_system_prompt
                        )
                    };
                    let model = self.active_model.clone();

                    let history_tuples: Vec<(String, String)> = self
                        .active_messages
                        .iter()
                        .take(self.active_messages.len().saturating_sub(1))
                        .map(|m| (m.role.clone(), m.content.clone()))
                        .collect();

                    let ai_config = self.storage.load_ai_config();
                    let (mut rx, cancel) = ai::provider::HybridAiEngine::spawn_hybrid_stream(
                        ai_config,
                        model,
                        system_prompt,
                        history_tuples,
                        2048,
                        self.profile.name.clone(),
                        self.profile.track_name().to_string(),
                        self.profile.maindy_style.clone(),
                    );
                    self.cancel_flag = Some(cancel);

                    // Stream tokens into UI
                    return Task::run(
                        iced::stream::channel(100, async move |mut output| {
                            while let Some(token) = rx.recv().await {
                                let _ = output.send(Message::AiStreamToken(token)).await;
                            }
                        }),
                        std::convert::identity,
                    );
                }
            }
            Message::AiStreamToken(token) => match token {
                ai::StreamToken::Chunk(piece) => {
                    if let Some(last) = self.active_messages.last_mut() {
                        last.content.push_str(&piece);
                    }
                }
                ai::StreamToken::SystemAlert {
                    severity,
                    title,
                    body,
                } => {
                    let retry_hint = if title.to_lowercase().contains("engine")
                        || title.to_lowercase().contains("ollama")
                        || body.to_lowercase().contains("ollama")
                    {
                        (
                            Some("Retry"),
                            Some(mindset_core::NotificationAction::RetryOllama),
                        )
                    } else {
                        (None, None)
                    };
                    match severity.as_str() {
                        "error" => {
                            self.core
                                .toast_queue
                                .error(title, body, retry_hint.0, retry_hint.1)
                        }
                        "warning" => {
                            self.core
                                .toast_queue
                                .warning(title, body, retry_hint.0, retry_hint.1)
                        }
                        "success" => self.core.toast_queue.success(title, body),
                        _ => self.core.toast_queue.info(title, body),
                    }
                }
                ai::StreamToken::Done => {
                    self.is_generating = false;
                    self.cancel_flag = None;
                    let mut pending_os_action = None;
                    let mut pending_agent_action = None;
                    let mut remove_empty_last = false;

                    if let Some(last) = self.active_messages.last_mut() {
                        let raw_snapshot = last.content.clone();
                        let (agent_act_opt, cleaned_agent) =
                            mindset_intelligence::agent_tools::extract_agent_action(&raw_snapshot);
                        let (os_act_opt, cleaned) = if agent_act_opt.is_some() {
                            (None, cleaned_agent)
                        } else {
                            ai::MaindySystemBridge::extract_llm_os_action(&raw_snapshot)
                        };

                        let parsed =
                            ai::persona::parse_dual_text(&cleaned, &self.profile.maindy_style);
                        if parsed.display_text.trim().is_empty() {
                            remove_empty_last = true;
                        } else {
                            last.content = parsed.display_text.clone();
                            self.storage.save_message(
                                &self.active_thread_id,
                                "assistant",
                                &last.content,
                            );
                        }

                        if self.auto_speak_enabled && !parsed.speech_text.trim().is_empty() {
                            ai::TtsEngine::speak(parsed.speech_text, parsed.primary_lang);
                            self.island_mode = shell::dynamic_island::IslandMode::MaindySpeaking;
                            let (tw, th) = self.island_mode.target_dimensions(0);
                            self.island_width_spring.set_target(tw);
                            self.island_height_spring.set_target(th);
                        }

                        pending_os_action = os_act_opt;
                        pending_agent_action = agent_act_opt;
                    }

                    if remove_empty_last {
                        self.active_messages.pop();
                    }

                    if let Some(os_act) = pending_os_action {
                        return self.handle_maindy_os_action(os_act, None, false);
                    }

                    if let Some(action) = pending_agent_action {
                        if mindset_intelligence::agent_tools::requires_permission(&action) {
                            let target_path = match &action {
                                mindset_intelligence::agent_tools::AgentAction::ListFolder {
                                    path,
                                } => Some(path.as_str()),
                                mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                                    file_path,
                                } => Some(file_path.as_str()),
                                _ => None,
                            };
                            let is_permitted = if let Some(p) = target_path {
                                ai::agent::is_folder_permitted(p, &self.session_allowed_folders)
                            } else {
                                false
                            };

                            if is_permitted {
                                let (_, target) =
                                    mindset_intelligence::agent_tools::action_description(&action);
                                let action_type = match &action {
                                    mindset_intelligence::agent_tools::AgentAction::ListFolder { .. } => "ListFolder",
                                    mindset_intelligence::agent_tools::AgentAction::ReadDocument { .. } => "ReadDocument",
                                    mindset_intelligence::agent_tools::AgentAction::ControlSystem { .. } => "ControlSystem",
                                    _ => "Other",
                                };
                                self.storage.log_ai_activity(
                                    action_type,
                                    &target,
                                    "Approved (Session)",
                                    "Authorized by session permission",
                                );
                                self.ai_activity_log = self.storage.list_ai_activities(50);
                                return self.execute_agent_action(action);
                            } else {
                                let (_, target) =
                                    mindset_intelligence::agent_tools::action_description(&action);
                                let action_type = match &action {
                                    mindset_intelligence::agent_tools::AgentAction::ListFolder { .. } => "ListFolder",
                                    mindset_intelligence::agent_tools::AgentAction::ReadDocument { .. } => "ReadDocument",
                                    mindset_intelligence::agent_tools::AgentAction::ControlSystem { .. } => "ControlSystem",
                                    _ => "Other",
                                };
                                self.storage.log_ai_activity(
                                    action_type,
                                    &target,
                                    "Pending",
                                    "Awaiting user approval",
                                );
                                self.ai_activity_log = self.storage.list_ai_activities(50);
                                self.pending_agent_action = Some(action);
                                self.core.wm.toggle_overlay(ActiveOverlay::AgentPermission);
                            }
                        } else {
                            let (_, target) =
                                mindset_intelligence::agent_tools::action_description(&action);
                            let action_type = match &action {
                                mindset_intelligence::agent_tools::AgentAction::CreateAndShowCapsule { .. } => "CreateAndShowCapsule",
                                mindset_intelligence::agent_tools::AgentAction::OpenApp { .. } => "OpenApp",
                                _ => "AutoAction",
                            };
                            self.storage.log_ai_activity(
                                action_type,
                                &target,
                                "Auto",
                                "Auto-executed safe action",
                            );
                            self.ai_activity_log = self.storage.list_ai_activities(50);
                            return self.execute_agent_action(action);
                        }
                    }
                }
                ai::StreamToken::Error(err) => {
                    self.is_generating = false;
                    self.cancel_flag = None;
                    if self
                        .active_messages
                        .last()
                        .map(|m| m.role == "assistant" && m.content.trim().is_empty())
                        .unwrap_or(false)
                    {
                        self.active_messages.pop();
                    }
                    self.core.toast_queue.error(
                        "Maindy Engine Interrupted",
                        err,
                        Some("Retry"),
                        Some(mindset_core::NotificationAction::RetryOllama),
                    );
                }
            },
            Message::CancelAiStream => {
                if let Some(flag) = &self.cancel_flag {
                    flag.store(true, Ordering::SeqCst);
                }
                self.is_generating = false;
                self.cancel_flag = None;
            }
            Message::ModelsDiscovered(models) => {
                if !models.is_empty() {
                    self.active_model = models[0].clone();
                    self.installed_models = models;
                }
            }
            Message::SelectModel(model) => {
                self.active_model = model;
            }
            Message::UnloadOllamaModel => {
                let model = self.active_model.clone();
                return Task::perform(
                    async move {
                        ai::OllamaEngine::unload_model_from_ram("127.0.0.1:11434", &model).await
                    },
                    Message::ModelUnloaded,
                );
            }
            Message::ModelUnloaded(res) => match res {
                Ok(()) => {
                    self.core.toast_queue.success(
                        "RAM Evicted",
                        "Ollama model evicted from memory (keep_alive: 0).",
                    );
                }
                Err(err) => {
                    self.core
                        .toast_queue
                        .warning("Unload Skipped", err, None, None);
                }
            },
            Message::NewChatThread => {
                let id = self.storage.create_thread("New Study Session");
                self.active_thread_id = id;
                self.threads = self.storage.list_threads();
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);
            }
            Message::SelectChatThread(id) => {
                self.active_thread_id = id;
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);
            }
            Message::GenerateDailyCapsules => {
                let initial = capsules::generator::generate_capsules_for_blueprint(
                    &self.profile.career_track,
                    &self.profile.weak_topics,
                );
                for cap in &initial {
                    self.storage.save_capsule(cap);
                }
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                self.capsules = self.storage.list_capsules(&today_str);
                if self.capsules.is_empty() {
                    self.capsules = initial;
                }
                self.core.toast_queue.push(
                    "Study Capsules Ready",
                    format!(
                        "Maindy AI delivered {} tailored capsules for today.",
                        self.capsules.len()
                    ),
                    "⚡",
                );
            }
            Message::DeleteUserMemory(id) => {
                self.storage.delete_memory(id);
                self.memories = self.storage.list_memories();
                self.core
                    .toast_queue
                    .push("Memory Removed", "Maindy AI context updated.", "🗑️");
            }
            Message::NewMemoryInputChanged(val) => {
                self.new_memory_input = val;
            }
            Message::AddUserMemory => {
                let trimmed = self.new_memory_input.trim();
                if !trimmed.is_empty() {
                    self.storage.add_memory(trimmed, "Student Context");
                    self.new_memory_input.clear();
                    self.memories = self.storage.list_memories();
                    self.core.toast_queue.push(
                        "Memory Saved",
                        "Maindy AI will remember this fact.",
                        "🧠",
                    );
                }
            }
            Message::BootSplashTick => {
                let dt = if std::env::var("MINDSET_LOW_OVERHEAD").ok().as_deref() == Some("1") {
                    0.045
                } else {
                    0.033
                };
                let finished = self.boot_splash.update(dt);
                if finished && self.profile.onboarding_completed {
                    self.core.toast_queue.push(
                        format!("Welcome back, {}!", self.profile.name),
                        "Your Focus World is active. Let's conquer today's goals!",
                        "✨",
                    );
                }
            }
            Message::FinishBootSplash => {
                self.boot_splash.skip();
                if self.profile.onboarding_completed {
                    self.core.toast_queue.push(
                        format!("Welcome back, {}!", self.profile.name),
                        "Your Focus World is active. Let's conquer today's goals!",
                        "✨",
                    );
                }
            }
            Message::RerunOnboarding => {
                self.profile.onboarding_completed = false;
                let _ = self.storage.save_profile(&self.profile);
                self.onboarding_data =
                    onboarding::OnboardingData::new_with_profile(self.profile.clone());
                for win in &mut self.core.wm.windows {
                    win.is_open = false;
                }
                self.core.wm.z_stack.clear();
                self.core.wm.close_overlays();
            }
            Message::OnboardingNext => {
                self.onboarding_data.next();
            }
            Message::OnboardingPrev => {
                self.onboarding_data.prev();
            }
            Message::OnboardingSetName(name) => {
                self.onboarding_data.name_input = name;
            }
            Message::OnboardingSetClassDegree(cls) => {
                self.onboarding_data.class_degree_input = cls;
            }
            Message::OnboardingSetLanguage(lang) => {
                self.onboarding_data.selected_language = lang;
            }
            Message::OnboardingSelectTrack(track) => {
                self.onboarding_data.apply_track(&track);
            }
            Message::OnboardingSetTargetScore(score) => {
                let max = self.onboarding_data.target_score_max;
                self.onboarding_data.target_score_value = score.clamp(0, max);
            }
            Message::OnboardingSetExamDate(date) => {
                self.onboarding_data.exam_date_input = date;
            }
            Message::OnboardingSetWeekdayHours(h) => {
                self.onboarding_data.daily_study_hours = h;
            }
            Message::OnboardingSetStrongSubject(subj) => {
                self.onboarding_data.strongest_subject = subj;
            }
            Message::OnboardingSetToughestSubject(subj) => {
                self.onboarding_data.toughest_subject = subj;
            }
            Message::OnboardingSetCoachingStyle(style) => {
                self.onboarding_data.coaching_tone = style;
            }
            Message::OnboardingSetWeakTopics(topics) => {
                self.onboarding_data.toughest_subject = topics.clone();
            }
            Message::OnboardingSetWakeTime(t) => {
                self.onboarding_data.profile.wake_time = t;
            }
            Message::OnboardingSetBedTime(t) => {
                self.onboarding_data.profile.bed_time = t;
            }
            Message::OnboardingSetSchoolHours(h) => {
                self.onboarding_data.profile.school_or_work_hours = h;
            }
            Message::OnboardingToggleObstacle(_obs) => {}
            Message::OnboardingSetRamProfile(ram) => {
                self.onboarding_data.profile.ram_profile = ram;
            }
            Message::OnboardingComplete => {
                self.onboarding_data.commit_to_profile();
                self.profile = self.onboarding_data.profile.clone();
                self.profile.onboarding_completed = true;
                let _ = self.storage.save_profile(&self.profile);
                self.storage.seed_localized_greeting(
                    &self.active_thread_id,
                    &self.profile.name,
                    &self.profile.maindy_style,
                    &self.profile.coaching_style,
                );
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);
                self.core.today = mindset_core::TodayState::fresh_for_student(
                    self.profile.weekday_study_hours,
                    &self.profile.target_label,
                );
                let _ = self.storage.save_capsule_plan(&self.core.exam_plan);

                // Persist 3 tailored memories into SQLite user_memories
                self.storage.add_memory(
                    &format!(
                        "Strongest academic subject: {}",
                        self.onboarding_data.strongest_subject
                    ),
                    "Academic Strength",
                );
                self.storage.add_memory(
                    &format!(
                        "Toughest priority area for reinforcement: {}",
                        self.onboarding_data.toughest_subject
                    ),
                    "Growth Area",
                );
                self.storage.add_memory(
                    &format!(
                        "Targeting {} in {} on {} with {}h daily study",
                        self.profile.target_score_value,
                        self.profile.target_exam_display(),
                        self.profile.exam_date,
                        self.profile.weekday_study_hours,
                    ),
                    "Core Mission",
                );
                self.memories = self.storage.list_memories();

                let initial_caps = capsules::generator::generate_capsules_for_blueprint(
                    &self.profile.career_track,
                    &self.profile.weak_topics,
                );
                for cap in &initial_caps {
                    self.storage.save_capsule(cap);
                }
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                self.capsules = self.storage.list_capsules(&today_str);
                if self.capsules.is_empty() {
                    self.capsules = initial_caps;
                }
                self.core.wm.open(WindowId::Today);
                self.core.toast_queue.push(
                    format!("Welcome, {}!", self.profile.name),
                    format!(
                        "Track initialized: {}. Let's conquer your goals.",
                        self.profile.target_label
                    ),
                    "🚀",
                );
            }
            Message::ScratchpadChanged(text) => {
                self.system.set_scratchpad(text.clone());
                let revision = self.system.scratchpad_revision;
                return Task::perform(
                    async move {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        (revision, text)
                    },
                    move |(rev, txt)| Message::SaveScratchpadDebounced {
                        revision: rev,
                        text: txt,
                    },
                );
            }
            Message::SaveScratchpadDebounced { revision, text } => {
                if revision == self.system.scratchpad_revision {
                    let storage = self.storage.clone();
                    return Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                storage.save_scratchpad(&text).map_err(|e| e.to_string())
                            })
                            .await
                            .map_err(|e| e.to_string())
                            .and_then(|r| r)
                        },
                        move |res| Message::ScratchpadSaved {
                            revision,
                            result: res,
                        },
                    );
                }
            }
            Message::ScratchpadSaved { revision, result } => {
                if revision == self.system.scratchpad_revision {
                    if result.is_ok() {
                        self.system.scratchpad_save_status = "Auto-saved to SQLite";
                    } else {
                        self.system.scratchpad_save_status = "Save error";
                    }
                }
            }
            Message::CopyScratchpad => {
                self.system.scratchpad_save_status = "Copied to clipboard!";
                return iced::clipboard::write(self.system.scratchpad_text.clone());
            }
            Message::ClearScratchpad => {
                return self.update(Message::ScratchpadChanged(String::new()));
            }
            Message::DismissToast(id) => {
                self.core.toast_queue.dismiss(id);
            }
            Message::TriggerNotificationAction(toast_id, action) => {
                self.core.toast_queue.dismiss(toast_id);
                match action {
                    mindset_core::NotificationAction::OpenVaultFolder => {
                        return self.handle_maindy_os_action(
                            ai::MaindyOsAction::OpenFiles { path: None },
                            None,
                            false,
                        );
                    }
                    mindset_core::NotificationAction::RetryOllama => {
                        return Task::batch([
                            self.update(Message::StartOllama),
                            Task::perform(
                                async {
                                    ai::OllamaEngine::list_models("127.0.0.1:11434")
                                        .await
                                        .unwrap_or_default()
                                },
                                Message::ModelsDiscovered,
                            ),
                        ]);
                    }
                    mindset_core::NotificationAction::OpenSettings => {
                        self.core.wm.open(WindowId::Settings);
                        self.core.wm.close_overlays();
                    }
                    mindset_core::NotificationAction::OpenCapsule(cid) => {
                        return self.update(Message::OpenStudyCapsule(cid));
                    }
                }
            }
            Message::MaindyToggleLeftRail => {
                self.maindy_left_rail_open = !self.maindy_left_rail_open;
            }
            Message::MaindyToggleRightDrawer => {
                self.maindy_right_drawer_open = !self.maindy_right_drawer_open;
            }
            Message::MaindyToggleEnginePopover => {
                self.maindy_engine_popover_open = !self.maindy_engine_popover_open;
            }
            Message::MaindySelectDrawerTab(tab) => {
                self.maindy_drawer_tab = tab;
                self.maindy_right_drawer_open = true;
            }
            Message::SetMaindyScreen(screen) => {
                self.maindy_screen = screen;
            }
            Message::OpenStudyCapsule(id) => {
                let resolved_id = if id.trim().is_empty() {
                    self.active_capsule_id
                        .clone()
                        .or_else(|| self.study_capsules.first().map(|c| c.id.clone()))
                } else {
                    Some(id)
                };
                if let Some(cid) = resolved_id {
                    self.active_capsule_id = Some(cid);
                    self.active_capsule_phase = 0;
                    self.capsule_hint_open = false;
                    self.maindy_screen = apps::ai_tutor::MaindyScreen::CapsuleTheater;
                    self.core.wm.open(WindowId::LayaAiChat);
                    self.core.wm.close_overlays();
                } else {
                    self.core.toast_queue.info(
                        "Create Your First Capsule",
                        "Ask Maindy to synthesize a Study Capsule for any syllabus topic.",
                    );
                }
            }
            Message::SelectCapsulePhase(idx) => {
                self.active_capsule_phase = idx.min(3);
                self.capsule_hint_open = false;
            }
            Message::ToggleCapsuleHint => {
                self.capsule_hint_open = !self.capsule_hint_open;
            }
            Message::ExplainCapsuleStepInTutor(prompt) => {
                self.maindy_screen = apps::ai_tutor::MaindyScreen::TutorStudio;
                self.core.wm.open(WindowId::LayaAiChat);
                self.chat_input = prompt;
                return self.update(Message::SendChatMessage);
            }
            Message::StartCapsuleFocusSprint(cap_id, mins) => {
                return self.handle_maindy_os_action(
                    ai::MaindyOsAction::StartFocusSprint {
                        capsule_id: cap_id,
                        minutes: mins,
                    },
                    None,
                    false,
                );
            }
            Message::PinCapsuleToDesktopHud(cap_id) => {
                self.hud_visible = true;
                if let Some(p) = self
                    .core
                    .wm
                    .get_widget_placement_mut(mindset_core::DesktopWidgetId::ScholarBriefing)
                {
                    p.is_visible = true;
                    p.is_pinned = true;
                }
                if let Some(sc) = self.study_capsules.iter().find(|c| c.id == cap_id).cloned() {
                    let db_cap = sc.to_db_record();
                    self.storage.save_capsule(&db_cap);
                    self.capsules.retain(|c| c.id != db_cap.id);
                    self.capsules.insert(0, db_cap);
                    self.core.toast_queue.success(
                        "Pinned to Desktop HUD",
                        format!("`{}` is now pinned on your active Desktop HUD.", sc.title),
                    );
                } else {
                    self.core.toast_queue.success(
                        "Desktop HUD Enabled",
                        "Today's active study card is pinned on your desktop.",
                    );
                }
            }
            Message::RunQuickOrbCommand(cmd) => {
                self.chat_input = cmd;
                return self.update(Message::SendChatMessage);
            }
            Message::ConfirmDestructiveOsAction => {
                if let Some(act) = self.pending_destructive_action.take() {
                    return self.handle_maindy_os_action(act, None, true);
                }
            }
            Message::CancelDestructiveOsAction => {
                self.pending_destructive_action = None;
                self.core.toast_queue.info(
                    "Action Cancelled",
                    "No files or system power states were changed.",
                );
            }
            Message::OpenExamReport => {
                self.core.open_exam_report();
            }
            Message::OpenVerificationBoss => {
                self.core.start_verification_boss();
                self.stage1_selections = vec![None, None, None];
                self.stage2_selections = vec![None, None];
                self.feynman_input.clear();
            }
            Message::SelectBossStage1(q_idx, opt_idx) => {
                if q_idx < self.stage1_selections.len() {
                    self.stage1_selections[q_idx] = Some(opt_idx);
                    if let Some(session) = &mut self.core.boss_session {
                        session.apply_damage(10.0);
                    }
                }
            }
            Message::SelectBossStage2(q_idx, opt_idx) => {
                if q_idx < self.stage2_selections.len() {
                    self.stage2_selections[q_idx] = Some(opt_idx);
                    if let Some(session) = &mut self.core.boss_session {
                        session.apply_damage(15.0);
                    }
                }
            }
            Message::FeynmanInputChanged(text) => {
                self.feynman_input = text;
            }
            Message::SubmitVerificationBoss => {
                let mut recall_score = 0u8;
                let mut applied_score = 0u8;

                if let Some(exam) = &self.core.active_exam {
                    for (i, &choice) in self.stage1_selections.iter().enumerate() {
                        if let Some(opt_idx) = choice {
                            if i < exam.recall_questions.len()
                                && opt_idx == exam.recall_questions[i].correct_option_index
                            {
                                recall_score += exam.recall_questions[i].points;
                            }
                        }
                    }
                    for (i, &choice) in self.stage2_selections.iter().enumerate() {
                        if let Some(opt_idx) = choice {
                            if i < exam.applied_questions.len()
                                && opt_idx == exam.applied_questions[i].correct_index
                            {
                                applied_score += exam.applied_questions[i].points;
                            }
                        }
                    }
                }

                let feynman_prompt = if let Some(exam) = &self.core.active_exam {
                    exam.feynman_prompt.clone()
                } else {
                    let topic = self
                        .core
                        .exam_plan
                        .capsules
                        .get(self.core.exam_plan.current_capsule_index)
                        .map(|c| c.title.clone())
                        .unwrap_or_else(|| "Electrochemistry & Nernst Equation".to_string());
                    mindset_core::DynamicExamGenerator::generate_for_topic(&topic, 1).feynman_prompt
                };

                let student_input = self.feynman_input.clone();
                let model = self.active_model.clone();
                let recall = recall_score;
                let applied = applied_score;
                let weak_topic = feynman_prompt.target_concept.clone();

                self.core.toast_queue.push(
                    "Grading Submission...",
                    "Evaluating proof of study with local Ollama AI model...",
                    "🧠",
                );

                return Task::perform(
                    async move {
                        let res = FeynmanGrader::grade_with_ollama(
                            &student_input,
                            &feynman_prompt,
                            "http://127.0.0.1:11434/api/generate",
                            &model,
                        )
                        .await;
                        (recall, applied, res, weak_topic)
                    },
                    |(recall, applied, feynman_res, weak_topic)| Message::BossGraded {
                        recall_score: recall,
                        applied_score: applied,
                        feynman_res,
                        weak_topic,
                    },
                );
            }
            Message::BossGraded {
                recall_score,
                applied_score,
                feynman_res,
                weak_topic,
            } => {
                let feynman_score = feynman_res.points_awarded;
                let breakdown = VerificationScoreBreakdown::compute(
                    recall_score,
                    applied_score,
                    feynman_score,
                    0,
                    false,
                    vec![weak_topic],
                );

                let passed = ProofOfStudyGate::is_passing(breakdown.total_points, false);

                if let Some(session) = &mut self.core.boss_session {
                    if passed {
                        session.apply_damage(100.0);
                        session.is_victory = true;
                    } else {
                        session.is_defeated_by_boss = true;
                    }
                }

                let cap_idx = self.core.exam_plan.current_capsule_index;
                let mark_val = self
                    .core
                    .exam_plan
                    .capsules
                    .get(cap_idx)
                    .map(|c| c.mark_value)
                    .unwrap_or(5.0);

                if passed {
                    let rank = if breakdown.total_points >= 90 {
                        CapsuleRank::Platinum
                    } else {
                        CapsuleRank::Gold
                    };

                    if let Some(cap) = self.core.exam_plan.capsules.get_mut(cap_idx) {
                        cap.state = CapsuleState::Completed {
                            final_score: breakdown.total_points,
                            rank,
                            marks_locked: mark_val,
                            xp_earned: 150,
                            verified_at_epoch: chrono::Utc::now().timestamp() as u64,
                        };
                    }
                    self.core.exam_plan.predicted_exam_score += mark_val;
                    self.core.exam_plan.mindset_xp += 150;
                    self.core.toast_queue.push(
                        "Boss Cleared! Capsule Verified",
                        format!(
                            "Passed with {}/100 points (Feynman: {}/30, {}).",
                            breakdown.total_points,
                            feynman_score,
                            if feynman_res.is_offline_fallback {
                                "Offline Rubric"
                            } else {
                                "Live Ollama"
                            }
                        ),
                        "🏆",
                    );
                } else {
                    if let Some(cap) = self.core.exam_plan.capsules.get_mut(cap_idx) {
                        cap.state = CapsuleState::NeedsRemediation {
                            last_score: breakdown.total_points,
                            weak_topics: breakdown.weak_subtopics.clone(),
                            cooldown_until_epoch: (chrono::Utc::now().timestamp() + 900) as u64,
                            attempt: 1,
                        };
                    }
                    let weak_topic_label = breakdown
                        .weak_subtopics
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "Core Concept".to_string());
                    let _ = self.personalization.record_mistake(
                        "Physics",
                        &weak_topic_label,
                        "Verification Boss Feynman",
                        &self.feynman_input,
                        "Complete causal explanation with formula & SI units required",
                        "Conceptual Gap",
                    );
                    self.unresolved_mistakes = self
                        .personalization
                        .get_unresolved_mistakes(None)
                        .unwrap_or_default();
                    self.core.toast_queue.push(
                        "Verification Gate Failed",
                        format!(
                            "Scored {}/100. Score >= 70 required to verify capsule.",
                            breakdown.total_points
                        ),
                        "⚠️",
                    );
                }
            }
            Message::StartOllama => {
                let res = ai::OllamaEngine::start_local_ollama();
                if res.is_ok() {
                    self.core.toast_queue.push(
                        "Launching Ollama",
                        "Started 'ollama serve' in background.",
                        "⚡",
                    );
                } else {
                    self.core.toast_queue.push(
                        "Ollama Launch Failed",
                        "Could not launch 'ollama serve'. Ensure Ollama CLI is installed.",
                        "⚠️",
                    );
                }
            }
            Message::RetryBossSession => {
                self.core.start_verification_boss();
                self.stage1_selections = vec![None, None, None];
                self.stage2_selections = vec![None, None];
                self.feynman_input.clear();
            }
            Message::TriggerExcuseRoast(excuse) => {
                self.core.start_excuse_roast(&excuse);
                self.excuse_selected_answer = None;
            }
            Message::AnswerExcuseFormula(ans_idx) => {
                self.excuse_selected_answer = Some(ans_idx);
                let passed = if let Some(roast) = &mut self.core.excuse_roast {
                    mindset_core::ExcuseRoastEngine::evaluate_answer(roast, ans_idx)
                } else {
                    false
                };
                if passed {
                    self.core.toast_queue.push(
                        "Formula Verified!",
                        "5-minute mindful pass unlocked.",
                        "✅",
                    );
                }
            }
            Message::OpenBreakArcade => {
                self.core.start_break_arcade();
            }
            Message::SelectArcadeCard(idx) => {
                if let Some(arcade) = &mut self.core.break_arcade {
                    arcade.select_card(idx);
                }
            }
            Message::DockHovered(idx_opt) => {
                self.hovered_dock_index = idx_opt;
                for (i, spring) in self.dock_springs.iter_mut().enumerate() {
                    let target = match idx_opt {
                        Some(hovered) => {
                            let diff = (i as isize - hovered as isize).abs();
                            if diff == 0 {
                                1.35
                            } else if diff == 1 {
                                1.15
                            } else {
                                1.0
                            }
                        }
                        None => 1.0,
                    };
                    spring.set_target(target);
                }
            }
            Message::MaindyOrbHovered(hovered) => {
                let target = if hovered { 1.08 } else { 1.0 };
                self.maindy_spring.set_target(target);
            }
            Message::ToggleMaindyDrawer => {
                self.maindy_drawer_open = !self.maindy_drawer_open;
                if self.maindy_drawer_open {
                    self.core.wm.close_overlays();
                    self.collapse_island();
                }
                self.maindy_spring.set_target(0.92);
            }
            Message::SetIslandMode(mode) => {
                self.island_mode = mode;
                if !matches!(
                    mode,
                    shell::dynamic_island::IslandMode::Compact
                        | shell::dynamic_island::IslandMode::ActiveFocus
                        | shell::dynamic_island::IslandMode::MaindySpeaking
                ) {
                    self.core.wm.close_overlays();
                    self.maindy_drawer_open = false;
                }
                let count = if mode == shell::dynamic_island::IslandMode::Search {
                    if self.island_search_query.trim().is_empty() {
                        0
                    } else {
                        1
                    }
                } else {
                    0
                };
                let (w, h) = mode.target_dimensions(count);
                self.island_width_spring.set_target(w);
                self.island_height_spring.set_target(h);
                if mode == shell::dynamic_island::IslandMode::Compact {
                    self.island_search_query.clear();
                }
            }
            Message::IslandSearchChanged(q) => {
                self.island_search_query = q;
                let count = if self.island_search_query.trim().is_empty() {
                    0
                } else {
                    1
                };
                let (w, h) = shell::dynamic_island::IslandMode::Search.target_dimensions(count);
                self.island_width_spring.set_target(w);
                self.island_height_spring.set_target(h);
            }
            Message::SetThemeMode(theme_str) => {
                self.profile.theme_mode = theme_str.clone();
                if theme_str == "AlpineMatteLight" {
                    self.core.preferences.theme = mindset_core::ThemeMode::AlpineGlass;
                } else {
                    self.core.preferences.theme = mindset_core::ThemeMode::DarkSpace;
                }
                let _ = self.storage.save_profile(&self.profile);
            }
            Message::SetMildMotion(enabled) => {
                self.profile.mild_motion_enabled = enabled;
                let _ = self.storage.save_profile(&self.profile);
            }
            Message::SetWallpaperPreset(path_str) => {
                self.profile.wallpaper_path = path_str.clone();
                let _ = self.storage.save_profile(&self.profile);
                if std::path::Path::new(&path_str).exists() {
                    self.wallpaper_handle = Some(iced::widget::image::Handle::from_path(&path_str));
                }
            }
            Message::SelectCustomWallpaper => {
                return Task::perform(
                    async {
                        let file = rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["jpg", "jpeg", "png", "webp"])
                            .set_title("Select Wallpaper Image")
                            .pick_file()
                            .await;
                        file.map(|f| f.path().to_string_lossy().to_string())
                    },
                    Message::CustomWallpaperSelected,
                );
            }
            Message::CustomWallpaperSelected(opt_path) => {
                if let Some(path) = opt_path {
                    self.profile.wallpaper_path = path.clone();
                    let _ = self.storage.save_profile(&self.profile);
                    self.wallpaper_handle = Some(iced::widget::image::Handle::from_path(path));
                }
            }
            Message::SetMapTheme(theme_name) => {
                self.profile.map_theme = theme_name;
                let _ = self.storage.save_profile(&self.profile);
                self.odyssey_map_state.static_road_cache.clear();
                self.odyssey_map_state.dynamic_ambient_cache.clear();
            }
            Message::HoverOdysseyNode(opt) => {
                if self.odyssey_map_state.hovered_node != opt {
                    self.odyssey_map_state.hovered_node = opt;
                    self.odyssey_map_state.static_road_cache.clear();
                    self.odyssey_map_state.dynamic_ambient_cache.clear();
                }
            }
            Message::SelectOdysseyNode(opt) => {
                self.odyssey_map_state.selected_node = opt;
            }
            Message::ScrollOdysseyMap(delta) => {
                self.odyssey_map_state.scroll_offset =
                    (self.odyssey_map_state.scroll_offset - delta * 0.02).clamp(-0.15, 0.15);
                self.odyssey_map_state.static_road_cache.clear();
            }
            Message::LaunchOdysseyNode(node_id) => {
                self.core.odyssey.embark(&node_id);
                self.audio_player.play_chime();
                self.core.wm.open(WindowId::StudyReader);
                if !self.tunnel_vision_active {
                    let _ = self.update(Message::ToggleTunnelVision);
                }
                self.odyssey_map_state.selected_node = None;
            }
            Message::CloseOdysseyPopover => {
                self.odyssey_map_state.selected_node = None;
            }
            Message::MapAmbientTick => {
                self.odyssey_map_state.ambient_tick =
                    self.odyssey_map_state.ambient_tick.wrapping_add(1);
                self.odyssey_map_state.dynamic_ambient_cache.clear();
            }
            Message::AnimationFrame(now) => {
                let dt = if let Some(last) = self.last_anim_instant {
                    (now - last).as_secs_f32().clamp(0.0, 0.05)
                } else {
                    1.0 / 60.0
                };
                self.last_anim_instant = Some(now);

                for spring in &mut self.dock_springs {
                    if !spring.is_at_rest() {
                        spring.update(dt);
                    }
                }
                if !self.maindy_spring.is_at_rest() {
                    self.maindy_spring.update(dt);
                }
                if !self.island_width_spring.is_at_rest() {
                    self.island_width_spring.update(dt);
                }
                if !self.island_height_spring.is_at_rest() {
                    self.island_height_spring.update(dt);
                }
                if !self.tunnel_vision_spring.is_at_rest() {
                    self.tunnel_vision_spring.update(dt);
                }
                if let Some(rem) = self.smoke_frames_remaining.as_mut() {
                    if *rem <= 1 {
                        self.smoke_frames_remaining = Some(0);
                        eprintln!(
                            "[mindset-shell] SMOKE_FRAMES_COMPLETE: {}x{} zero-panic render",
                            self.wm_interaction.screen_width, self.wm_interaction.screen_height
                        );
                        return iced::exit();
                    } else {
                        *rem -= 1;
                    }
                }
            }
            Message::ToggleAudio => {
                let is_playing = self.audio_player.toggle();
                self.core.toast_queue.push(
                    if is_playing {
                        "Ambient Rainfall Active"
                    } else {
                        "Ambient Audio Muted"
                    },
                    if is_playing {
                        "Procedural brown-noise rain soundscape (<1 MB RAM)"
                    } else {
                        "Ambient soundscape paused"
                    },
                    if is_playing { "🎧" } else { "🔇" },
                );
            }
            Message::ToggleTunnelVision => {
                self.tunnel_vision_active = !self.tunnel_vision_active;
                let target = if self.tunnel_vision_active { 0.0 } else { 1.0 };
                self.tunnel_vision_spring.set_target(target);
                if self.tunnel_vision_active {
                    self.island_mode = shell::dynamic_island::IslandMode::ActiveFocus;
                    let (w, h) = self.island_mode.target_dimensions(0);
                    self.island_width_spring.set_target(w);
                    self.island_height_spring.set_target(h);
                } else {
                    self.island_mode = shell::dynamic_island::IslandMode::Compact;
                    let (w, h) = self.island_mode.target_dimensions(0);
                    self.island_width_spring.set_target(w);
                    self.island_height_spring.set_target(h);
                }
                self.core.toast_queue.push(
                    if self.tunnel_vision_active {
                        "Tunnel Vision Active"
                    } else {
                        "Tunnel Vision Disengaged"
                    },
                    if self.tunnel_vision_active {
                        "Peripheral widgets and dock hidden. Press Esc to exit."
                    } else {
                        "All peripheral widgets restored."
                    },
                    "🎯",
                );
            }
            Message::SetLanguage(lang) => {
                let app_lang = i18n::AppLanguage::from_code(&lang);
                self.i18n.set_language(app_lang);
                self.profile.language_code = app_lang.code().to_string();
                let _ = self.storage.save_profile(&self.profile);
                self.core.toast_queue.push(
                    "Language Updated",
                    format!("Interface language set to {}.", app_lang.display_name()),
                    "🌐",
                );
            }
            Message::SetMaindyStyle(style) => {
                self.profile.maindy_style = style.clone();
                let _ = self.storage.save_profile(&self.profile);
                self.core.toast_queue.push(
                    "Maindy Style Updated",
                    format!("Maindy conversation style set to {}.", style),
                    "🧠",
                );
            }
            Message::SetAiApiKey(key) => {
                let mut cfg = self.storage.load_ai_config();
                cfg.cloud_api_key = key;
                let _ = self.storage.save_ai_config(&cfg);
            }
            Message::SetAiCloudProvider(prov) => {
                let mut cfg = self.storage.load_ai_config();
                cfg.cloud_provider = prov;
                let _ = self.storage.save_ai_config(&cfg);
            }
            Message::SetAiModel(model) => {
                let mut cfg = self.storage.load_ai_config();
                cfg.cloud_model = model;
                let _ = self.storage.save_ai_config(&cfg);
            }
            Message::SetAiBaseUrl(url) => {
                let mut cfg = self.storage.load_ai_config();
                cfg.cloud_base_url = url;
                let _ = self.storage.save_ai_config(&cfg);
            }
            Message::TestAiConnection => {
                let cfg = self.storage.load_ai_config();
                return Task::perform(
                    async move {
                        match ai::provider::HybridAiEngine::health_check(&cfg).await {
                            Ok(status) => Ok(status.message),
                            Err(err) => Err(err),
                        }
                    },
                    Message::AiConnectionTested,
                );
            }
            Message::AiConnectionTested(res) => match res {
                Ok(msg) => {
                    self.core.toast_queue.push("API Test Success", msg, "✅");
                }
                Err(err) => {
                    self.core.toast_queue.push("API Test Notice", err, "ℹ️");
                }
            },
            Message::SpeakMessage(content) => {
                let parsed = ai::persona::parse_dual_text(&content, &self.profile.maindy_style);
                ai::TtsEngine::speak(parsed.speech_text, parsed.primary_lang);
                self.island_mode = shell::dynamic_island::IslandMode::MaindySpeaking;
                let (tw, th) = self.island_mode.target_dimensions(0);
                self.island_width_spring.set_target(tw);
                self.island_height_spring.set_target(th);
            }
            Message::StopTts => {
                ai::TtsEngine::stop();
                self.island_mode = shell::dynamic_island::IslandMode::Compact;
                let (tw, th) = self.island_mode.target_dimensions(0);
                self.island_width_spring.set_target(tw);
                self.island_height_spring.set_target(th);
            }
            Message::ToggleAutoSpeak => {
                self.auto_speak_enabled = !self.auto_speak_enabled;
                self.core.toast_queue.push(
                    "Auto-Speak",
                    if self.auto_speak_enabled {
                        "Auto-Speak enabled"
                    } else {
                        "Auto-Speak disabled"
                    },
                    "🔊",
                );
            }
            Message::AgentPermissionResponse(decision) => {
                self.core.wm.close_overlays();
                match decision {
                    ai::agent::AgentPermissionDecision::AllowOnce => {
                        if let Some(action) = self.pending_agent_action.take() {
                            let (_, target) =
                                mindset_intelligence::agent_tools::action_description(&action);
                            let action_type = match &action {
                                mindset_intelligence::agent_tools::AgentAction::ListFolder {
                                    ..
                                } => "ListFolder",
                                mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                                    ..
                                } => "ReadDocument",
                                mindset_intelligence::agent_tools::AgentAction::ControlSystem {
                                    ..
                                } => "ControlSystem",
                                _ => "Other",
                            };
                            self.storage.log_ai_activity(
                                action_type,
                                &target,
                                "Approved (Once)",
                                "Granted single-use permission",
                            );
                            self.ai_activity_log = self.storage.list_ai_activities(50);
                            return self.execute_agent_action(action);
                        }
                    }
                    ai::agent::AgentPermissionDecision::AllowSession => {
                        if let Some(action) = self.pending_agent_action.take() {
                            let target_path = match &action {
                                mindset_intelligence::agent_tools::AgentAction::ListFolder {
                                    path,
                                } => Some(path.as_str()),
                                mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                                    file_path,
                                } => Some(file_path.as_str()),
                                _ => None,
                            };
                            if let Some(p) = target_path {
                                let scope = ai::agent::extract_folder_scope(p);
                                self.session_allowed_folders.insert(scope);
                            }
                            let (_, target) =
                                mindset_intelligence::agent_tools::action_description(&action);
                            let action_type = match &action {
                                mindset_intelligence::agent_tools::AgentAction::ListFolder {
                                    ..
                                } => "ListFolder",
                                mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                                    ..
                                } => "ReadDocument",
                                mindset_intelligence::agent_tools::AgentAction::ControlSystem {
                                    ..
                                } => "ControlSystem",
                                _ => "Other",
                            };
                            self.storage.log_ai_activity(
                                action_type,
                                &target,
                                "Approved (Session)",
                                "Granted whole-session permission for folder",
                            );
                            self.ai_activity_log = self.storage.list_ai_activities(50);
                            return self.execute_agent_action(action);
                        }
                    }
                    ai::agent::AgentPermissionDecision::Deny => {
                        if let Some(action) = self.pending_agent_action.take() {
                            let (_, target) =
                                mindset_intelligence::agent_tools::action_description(&action);
                            let action_type = match &action {
                                mindset_intelligence::agent_tools::AgentAction::ListFolder {
                                    ..
                                } => "ListFolder",
                                mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                                    ..
                                } => "ReadDocument",
                                mindset_intelligence::agent_tools::AgentAction::ControlSystem {
                                    ..
                                } => "ControlSystem",
                                _ => "Other",
                            };
                            self.storage.log_ai_activity(
                                action_type,
                                &target,
                                "Denied",
                                "User denied permission",
                            );
                            self.ai_activity_log = self.storage.list_ai_activities(50);
                            self.core.toast_queue.push(
                                "Permission Denied",
                                "Maindy AI was not granted access to this resource.",
                                "🔒",
                            );
                            let denial_msg =
                                "🔒 *Action cancelled: Permission was denied by user.*";
                            self.storage.save_message(
                                &self.active_thread_id,
                                "assistant",
                                denial_msg,
                            );
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
                        }
                    }
                }
            }
            Message::RevokeAllAgentPermissions => {
                self.session_allowed_folders.clear();
                self.pending_agent_action = None;
                self.storage.log_ai_activity(
                    "RevokeAll",
                    "Session Permissions",
                    "Revoked",
                    "All session folder authorizations have been revoked by user",
                );
                self.ai_activity_log = self.storage.list_ai_activities(50);
                self.core.toast_queue.push(
                    "Permissions Revoked",
                    "All session folder authorizations have been revoked.",
                    "🛡️",
                );
            }
            Message::ClearAiActivityLog => {
                self.storage.clear_ai_activities();
                self.ai_activity_log.clear();
                self.core.toast_queue.push(
                    "Log Cleared",
                    "AI activity history has been cleared.",
                    "🧹",
                );
            }
            // Browser & YouTube Study
            Message::BrowserUrlChanged(url) => {
                self.browser_url = url;
            }
            Message::BrowserNavigate(url) => {
                let mut target = url.trim().to_string();
                if target.is_empty() {
                    target = "https://www.youtube.com".to_string();
                } else if !target.contains("://") {
                    if target.contains('.') && !target.contains(' ') {
                        target = format!("https://{target}");
                    } else {
                        let query = target.replace(' ', "+");
                        target = format!("https://www.youtube.com/results?search_query={query}");
                    }
                }
                self.browser_url = target.clone();
                if self.browser_history.last() != Some(&target) {
                    self.browser_history.push(target);
                    self.browser_history_idx = self.browser_history.len() - 1;
                }
            }
            Message::LaunchBrowserUrl(url) => {
                let focus_shield = self.browser_focus_shield;
                let target_url = if url.trim().is_empty() {
                    self.browser_url.clone()
                } else {
                    url
                };
                self.browser_status_note = Some("Spawning shielded Chromium window...".to_string());
                return Task::perform(
                    async move {
                        let mut entertainment = mindset_net::EntertainmentShield::new();
                        let mut youtube = mindset_net::YoutubeGuard::new();
                        let bridge = mindset_net::ChromiumEngineBridge::new();
                        bridge.launch_shielded_url(
                            &target_url,
                            &mut entertainment,
                            &mut youtube,
                            focus_shield,
                        )
                    },
                    Message::BrowserLaunched,
                );
            }
            Message::BrowserLaunched(res) => match res {
                Ok(msg) => {
                    self.browser_status_note = Some(msg.clone());
                    self.core.toast_queue.push("Browser Launched", msg, "🚀");
                }
                Err(err) => {
                    self.browser_status_note = Some(format!("Error: {err}"));
                    self.core.toast_queue.push("Browser Notice", err, "⚠️");
                }
            },
            Message::ToggleBrowserFocusShield => {
                self.browser_focus_shield = !self.browser_focus_shield;
                self.core.toast_queue.push(
                    "Focus Shield Toggled",
                    if self.browser_focus_shield {
                        "Anti-Distraction Shield Activated (Shorts Redirect ON)"
                    } else {
                        "Standard Web Browsing (Focus Shield OFF)"
                    },
                    "🛡️",
                );
            }
            Message::BrowserBack => {
                if self.browser_history_idx > 0 {
                    self.browser_history_idx -= 1;
                    if let Some(prev) = self.browser_history.get(self.browser_history_idx) {
                        self.browser_url = prev.clone();
                    }
                }
            }
            Message::BrowserForward => {
                if self.browser_history_idx + 1 < self.browser_history.len() {
                    self.browser_history_idx += 1;
                    if let Some(next) = self.browser_history.get(self.browser_history_idx) {
                        self.browser_url = next.clone();
                    }
                }
            }
            Message::OpenYouTubeStudy => {
                self.browser_url = "https://www.youtube.com".to_string();
                if self.browser_history.last() != Some(&self.browser_url) {
                    self.browser_history.push(self.browser_url.clone());
                    self.browser_history_idx = self.browser_history.len() - 1;
                }
                self.core.wm.open(WindowId::SearchBrowser);
                self.core.wm.close_overlays();
                if let Some(win) = self.core.wm.get_window_mut(WindowId::SearchBrowser) {
                    self.wm_interaction.clamp_window(win);
                }
                return Task::perform(
                    async { mindset_net::ChromiumEngineBridge::spawn_youtube_app() },
                    Message::BrowserLaunched,
                );
            }
            Message::LaunchYouTubeApp => {
                self.browser_status_note =
                    Some("Spawning real YouTube standalone app...".to_string());
                return Task::perform(
                    async { mindset_net::ChromiumEngineBridge::spawn_youtube_app() },
                    Message::BrowserLaunched,
                );
            }
            Message::AdaptTodaySchedule(energy_level, hours) => {
                mindset_core::CapsulePlanner::recalibrate_for_checkin(
                    &mut self.core.exam_plan,
                    &energy_level,
                    hours,
                );
                let _ = self.storage.save_capsule_plan(&self.core.exam_plan);
                self.core.toast_queue.push(
                    "Schedule Adapted",
                    format!("Today's capsule budget tuned for {energy_level} ({hours:.1}h)."),
                    "📅",
                );
            }
            Message::AutoHealMissedDay => {
                mindset_core::CapsulePlanner::auto_heal_missed_day(&mut self.core.exam_plan, 1);
                let _ = self.storage.save_capsule_plan(&self.core.exam_plan);
                self.core.toast_queue.push(
                    "Zero-Guilt Auto-Heal",
                    format!(
                        "Missed day absorbed cleanly. {} buffer days remaining.",
                        self.core.exam_plan.buffer_days_remaining
                    ),
                    "🛡️",
                );
            }
            Message::YouTubeSearchChanged(q) => {
                self.youtube_study.search_query = q;
            }
            Message::YouTubeSearchSubmit => {
                let q = self.youtube_study.search_query.clone();
                self.youtube_study.is_searching = true;
                return Task::perform(
                    apps::youtube_study::search_educational_videos(q),
                    |(items, mirror)| Message::YouTubeSearchResults(items, mirror),
                );
            }
            Message::YouTubeSearchResults(items, mirror) => {
                self.youtube_study.is_searching = false;
                self.youtube_study.active_mirror = mirror;
                self.youtube_study.selected_video = items.first().cloned();
                self.youtube_study.results = items;
            }
            Message::YouTubeSelectVideo(idx) => {
                self.youtube_study.selected_video = self.youtube_study.results.get(idx).cloned();
            }
            Message::YouTubeNotesChanged(notes) => {
                self.youtube_study.lecture_notes = notes;
            }
            Message::MindStoreSearchChanged(q) => {
                self.mindstore_state.search_query = q;
            }
            Message::MindStoreSelectCategory(cat) => {
                self.mindstore_state.selected_category = cat;
            }
            Message::MindStoreInstallCatalogApp(app_id) => {
                if let Some(entry) = self
                    .mindstore_state
                    .catalog
                    .iter()
                    .find(|e| e.manifest.id == app_id)
                    .cloned()
                {
                    let apps_root = mindset_sys::store::MsPackageManager::default_apps_root();
                    let tmp_ms = std::env::temp_dir().join(format!("{app_id}.ms"));
                    let payload = serde_json::to_vec_pretty(&entry.manifest).unwrap_or_default();
                    if let Ok(digest) = mindset_sys::store::MsPackageManager::build_ms_archive(
                        &tmp_ms,
                        &entry.manifest,
                        &entry.manifest.entrypoint,
                        &payload,
                    ) {
                        match mindset_sys::store::MsPackageManager::install_ms_archive(
                            &tmp_ms,
                            &apps_root,
                            Some(&digest),
                        ) {
                            Ok(inst) => {
                                let manifest_json =
                                    serde_json::to_string(&inst.manifest).unwrap_or_default();
                                let _ = self.storage.save_installed_ms_app(
                                    &inst.manifest.id,
                                    &inst.manifest.name,
                                    &inst.manifest.version,
                                    inst.manifest.kind.label(),
                                    &manifest_json,
                                    &inst.install_dir.to_string_lossy(),
                                );
                                if !self.mindstore_state.installed_ids.contains(&app_id) {
                                    self.mindstore_state.installed_ids.push(app_id.clone());
                                }
                                self.mindstore_state.status_message = Some(format!(
                                    "Installed {} (.ms verified SHA-256)",
                                    inst.manifest.name
                                ));
                                self.core.toast_queue.push(
                                    "MindStore App Installed",
                                    format!("{} is ready to launch.", inst.manifest.name),
                                    "🛍️",
                                );
                            }
                            Err(e) => {
                                self.mindstore_state.status_message = Some(e);
                            }
                        }
                        let _ = std::fs::remove_file(&tmp_ms);
                    }
                }
            }
            Message::MindStoreLaunchApp(app_id) => {
                if let Some(entry) = self
                    .mindstore_state
                    .catalog
                    .iter()
                    .find(|e| e.manifest.id == app_id)
                {
                    if let Some(url) = &entry.manifest.default_url {
                        return self.update(Message::LaunchMindSetBrowser(Some(url.clone())));
                    }
                    self.core.toast_queue.push(
                        format!("Launching {}", entry.manifest.name),
                        "Opened native .ms study module.".to_string(),
                        "🛍️",
                    );
                }
            }
            Message::MindStoreUninstallApp(app_id) => {
                let apps_root = mindset_sys::store::MsPackageManager::default_apps_root();
                let _ = mindset_sys::store::MsPackageManager::uninstall_app(&app_id, &apps_root);
                let _ = self.storage.remove_installed_ms_app(&app_id);
                self.mindstore_state
                    .installed_ids
                    .retain(|id| id != &app_id);
                self.mindstore_state.status_message = Some(format!("Uninstalled {app_id}."));
            }
            Message::MindStorePickLocalPackage => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("MindSet App Package", &["ms"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::MindStoreLocalPackageSelected,
                );
            }
            Message::MindStoreLocalPackageSelected(maybe_path) => {
                if let Some(path) = maybe_path {
                    let apps_root = mindset_sys::store::MsPackageManager::default_apps_root();
                    match mindset_sys::store::MsPackageManager::install_ms_archive(
                        &path, &apps_root, None,
                    ) {
                        Ok(inst) => {
                            let manifest_json =
                                serde_json::to_string(&inst.manifest).unwrap_or_default();
                            let _ = self.storage.save_installed_ms_app(
                                &inst.manifest.id,
                                &inst.manifest.name,
                                &inst.manifest.version,
                                inst.manifest.kind.label(),
                                &manifest_json,
                                &inst.install_dir.to_string_lossy(),
                            );
                            if !self
                                .mindstore_state
                                .installed_ids
                                .contains(&inst.manifest.id)
                            {
                                self.mindstore_state
                                    .installed_ids
                                    .push(inst.manifest.id.clone());
                            }
                            self.mindstore_state.status_message = Some(format!(
                                "Installed local .ms package: {}",
                                inst.manifest.name
                            ));
                        }
                        Err(e) => {
                            self.mindstore_state.status_message = Some(e);
                        }
                    }
                }
            }
            Message::InstallMindSetToDisk => {
                self.core.wm.close_overlays();
                #[cfg(target_os = "linux")]
                {
                    let res = std::process::Command::new("sh")
                        .arg("-c")
                        .arg("command -v calamares >/dev/null && sudo -E calamares &")
                        .spawn();
                    match res {
                        Ok(_) => self.core.toast_queue.push(
                            "MindSet OS V1.3 Installer",
                            "Launching disk installer so your data persists across reboots."
                                .to_string(),
                            "💿",
                        ),
                        Err(e) => self.core.toast_queue.warning(
                            "Installer Unavailable",
                            format!("Could not start Calamares: {e}"),
                            None,
                            None,
                        ),
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    self.core.toast_queue.push(
                        "Disk Installer (Linux Live Only)",
                        "Boot the MindSet OS V1.3 .iso on a VM or PC to install to disk."
                            .to_string(),
                        "💿",
                    );
                }
            }
            Message::LaunchFreeTubeApp => {
                self.browser_status_note = Some("Spawning FreeTube ad-free player...".to_string());
                return Task::perform(
                    async { mindset_net::ChromiumEngineBridge::spawn_freetube_app() },
                    Message::BrowserLaunched,
                );
            }
            Message::LaunchMindSetBrowser(maybe_url) => {
                let target = maybe_url.or_else(|| {
                    if self.browser_url.is_empty()
                        || self.browser_url == "https://ncert.nic.in/textbook.php"
                    {
                        None
                    } else {
                        Some(self.browser_url.clone())
                    }
                });
                self.browser_status_note =
                    Some("Spawning real MindSet Browser (Chromium)...".to_string());
                return Task::perform(
                    async move {
                        mindset_net::ChromiumEngineBridge::spawn_mindset_browser(target.as_deref())
                    },
                    Message::BrowserLaunched,
                );
            }

            // Wi-Fi & Network Manager
            Message::RefreshNetworkStatus => {
                return Task::perform(
                    async { mindset_sys::network::NetworkManager::get_status() },
                    Message::NetworkStatusUpdated,
                );
            }
            Message::NetworkStatusUpdated(status) => {
                self.network_status = status;
            }
            Message::ScanWifiNetworks => {
                return Task::perform(
                    async { mindset_sys::network::NetworkManager::scan_networks() },
                    Message::WifiNetworksScanned,
                );
            }
            Message::WifiNetworksScanned(nets) => {
                self.nearby_networks = nets;
            }
            Message::ToggleWifiMenu => {
                self.wifi_menu_open = !self.wifi_menu_open;
                if self.wifi_menu_open {
                    return Task::perform(
                        async { mindset_sys::network::NetworkManager::scan_networks() },
                        Message::WifiNetworksScanned,
                    );
                }
            }
            Message::SelectWifiNetwork(ssid) => {
                self.wifi_connecting_ssid = Some(ssid);
                self.wifi_password_input.clear();
            }
            Message::WifiPasswordInputChanged(pwd) => {
                self.wifi_password_input = pwd;
            }
            Message::CancelWifiConnect => {
                self.wifi_connecting_ssid = None;
                self.wifi_password_input.clear();
            }
            Message::SubmitWifiConnect => {
                if let Some(ssid) = self.wifi_connecting_ssid.clone() {
                    let password = if self.wifi_password_input.is_empty() {
                        None
                    } else {
                        Some(self.wifi_password_input.clone())
                    };
                    return Task::perform(
                        async move {
                            mindset_sys::network::NetworkManager::connect(
                                &ssid,
                                password.as_deref(),
                            )
                        },
                        Message::WifiConnectResult,
                    );
                }
            }
            Message::WifiConnectResult(res) => {
                self.wifi_connecting_ssid = None;
                self.wifi_password_input.clear();
                match res {
                    Ok(msg) => {
                        self.core.toast_queue.push("Wi-Fi Connected", msg, "📶");
                    }
                    Err(err) => {
                        self.core.toast_queue.push("Wi-Fi Notice", err, "⚠️");
                    }
                }
                return Task::perform(
                    async { mindset_sys::network::NetworkManager::get_status() },
                    Message::NetworkStatusUpdated,
                );
            }
            Message::EventOccurred(Event::Window(window::Event::Resized(size))) => {
                self.wm_interaction
                    .set_screen_dimensions(size.width, size.height);
                for win in &mut self.core.wm.windows {
                    if win.mode == WindowMode::Normal {
                        self.wm_interaction.clamp_window(win);
                    }
                }
                for widget in &mut self.core.wm.widget_layout.placements {
                    self.wm_interaction.clamp_widget(widget);
                }
            }
            Message::EventOccurred(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                self.cursor_position = position;
                // 1. Draggable Window Header with safe-area bounds clamping
                if let Some((id, offset_x, offset_y)) = self.wm_interaction.dragging {
                    if let Some(win) = self.core.wm.get_window_mut(id) {
                        if win.mode == WindowMode::Normal {
                            win.x = position.x - offset_x;
                            win.y = position.y - offset_y;
                            self.wm_interaction.clamp_window(win);
                        }
                    }
                }
                // 2. Desktop Widgets Free Placement with safe-area bounds clamping
                if let Some((widget_id, offset_x, offset_y)) = self.wm_interaction.dragging_widget {
                    if let Some(widget) = self.core.wm.get_widget_placement_mut(widget_id) {
                        widget.x = position.x - offset_x;
                        widget.y = position.y - offset_y;
                        self.wm_interaction.clamp_widget(widget);
                    }
                }
            }
            Message::EventOccurred(Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Left,
            ))) => {
                if self.boot_splash.is_active {
                    self.boot_splash.skip();
                    if self.profile.onboarding_completed {
                        self.core.toast_queue.push(
                            format!("Welcome back, {}!", self.profile.name),
                            "Your Focus World is active. Let's conquer today's goals!",
                            "✨",
                        );
                    }
                }

                if let Some((id, _, _)) = self.wm_interaction.dragging.take() {
                    if let Some(win) = self.core.wm.get_window_mut(id) {
                        self.wm_interaction.clamp_window(win);
                    }
                }
                if let Some((widget_id, _, _)) = self.wm_interaction.dragging_widget.take() {
                    if let Some(widget) = self.core.wm.get_widget_placement_mut(widget_id) {
                        self.wm_interaction.clamp_widget(widget);
                        let record = storage::WidgetLayoutRecord {
                            widget_id: format!("{:?}", widget_id),
                            pos_x: widget.x,
                            pos_y: widget.y,
                            width: widget.width,
                            height: widget.height,
                            is_pinned: widget.is_pinned,
                            z_index: 1,
                        };
                        let _ = self.storage.save_widget_layout(&record);
                    }
                }
            }
            Message::EventOccurred(Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            })) => {
                if self.boot_splash.is_active {
                    match key {
                        Key::Named(Named::Enter) | Key::Named(Named::Space) => {
                            self.boot_splash.skip();
                            if self.profile.onboarding_completed {
                                self.core.toast_queue.push(
                                    format!("Welcome back, {}!", self.profile.name),
                                    "Your Focus World is active. Let's conquer today's goals!",
                                    "✨",
                                );
                            }
                            return Task::none();
                        }
                        Key::Named(Named::Escape) => {
                            self.boot_splash.skip();
                        }
                        _ => {}
                    }
                }

                // Ctrl+K -> Toggle Dynamic Island Search
                if modifiers.control() {
                    if let Key::Character(c) = &key {
                        if c.as_str() == "k" || c.as_str() == "K" {
                            if self.island_mode == shell::dynamic_island::IslandMode::Search {
                                self.island_mode = shell::dynamic_island::IslandMode::Compact;
                                self.island_search_query.clear();
                            } else {
                                self.island_mode = shell::dynamic_island::IslandMode::Search;
                                self.core.wm.close_overlays();
                                self.maindy_drawer_open = false;
                            }
                            let count = if self.island_search_query.trim().is_empty() {
                                0
                            } else {
                                1
                            };
                            let (w, h) = self.island_mode.target_dimensions(count);
                            self.island_width_spring.set_target(w);
                            self.island_height_spring.set_target(h);
                        }
                    }
                }
                // Ctrl+Space -> Toggle Launcher (strict mutual exclusion)
                if modifiers.control() {
                    if let Key::Named(Named::Space) = key {
                        self.core.wm.toggle_overlay(ActiveOverlay::Launcher);
                        if self.core.wm.active_overlay != ActiveOverlay::None {
                            self.collapse_island();
                            self.maindy_drawer_open = false;
                        }
                    }
                }
                // Alt+Tab -> Toggle Window Switcher
                if modifiers.alt() {
                    if let Key::Named(Named::Tab) = key {
                        self.core.wm.toggle_overlay(ActiveOverlay::WindowSwitcher);
                        if self.core.wm.active_overlay != ActiveOverlay::None {
                            self.collapse_island();
                            self.maindy_drawer_open = false;
                        }
                    }
                }
                // Super+D -> Toggle Desktop, Super+L -> Lock Screen
                if modifiers.logo() {
                    if let Key::Character(c) = &key {
                        if c.as_str() == "d" || c.as_str() == "D" {
                            self.core.preferences.toggle_desktop();
                        } else if c.as_str() == "l" || c.as_str() == "L" {
                            self.core.wm.toggle_overlay(ActiveOverlay::LockScreen);
                            self.collapse_island();
                            self.maindy_drawer_open = false;
                        }
                    }
                }
                // Enter -> Submit Lock Screen auth if active
                if self.core.wm.active_overlay == ActiveOverlay::LockScreen {
                    if let Key::Named(Named::Enter) = key {
                        return self.update(Message::SubmitLockScreenAuth);
                    }
                }
                // Esc -> Exit Tunnel Vision FIRST, or Close Dynamic Island / Overlays / Drawer
                if let Key::Named(Named::Escape) = key {
                    // Security Interlock: Escape MUST NOT dismiss the Lock Screen!
                    if self.core.wm.active_overlay == ActiveOverlay::LockScreen {
                        return Task::none();
                    }
                    if self.tunnel_vision_active {
                        self.tunnel_vision_active = false;
                        self.tunnel_vision_spring.set_target(1.0);
                        self.island_mode = shell::dynamic_island::IslandMode::Compact;
                        let (w, h) = self.island_mode.target_dimensions(0);
                        self.island_width_spring.set_target(w);
                        self.island_height_spring.set_target(h);
                        self.island_search_query.clear();
                        self.core.toast_queue.push(
                            "Tunnel Vision Disengaged",
                            "Peripheral widgets restored.",
                            "🎯",
                        );
                    } else if self.island_mode != shell::dynamic_island::IslandMode::Compact {
                        self.island_mode = shell::dynamic_island::IslandMode::Compact;
                        let (w, h) = self.island_mode.target_dimensions(0);
                        self.island_width_spring.set_target(w);
                        self.island_height_spring.set_target(h);
                        self.island_search_query.clear();
                    } else if self.core.wm.active_overlay != ActiveOverlay::None {
                        self.core.wm.close_overlays();
                    } else if self.maindy_drawer_open {
                        self.maindy_drawer_open = false;
                    }
                }
            }
            // ---------------------------------------------------------------
            // RAG Vault (Zone 3)
            // ---------------------------------------------------------------
            Message::PickVaultPdfToIngest => {
                return Task::perform(
                    async {
                        let file = rfd::AsyncFileDialog::new()
                            .add_filter("PDF Documents", &["pdf"])
                            .set_title("Select Textbook PDF to Ingest")
                            .pick_file()
                            .await;
                        file.map(|f| f.path().to_path_buf())
                    },
                    Message::IngestPdfPath,
                );
            }
            Message::IngestPdfPath(opt_path) => {
                if let Some(path) = opt_path {
                    match self.rag_vault.ingest_pdf(&path) {
                        Ok(chunks) => {
                            let fname = chunks.first().map(|c| c.file_name.clone()).or_else(|| {
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|s| s.to_string())
                            });
                            self.rag_pdf_chunks = self.rag_vault.load_pdf_chunks_index();
                            if let Some(name) = fname {
                                self.rag_active_file = Some(name);
                            }
                            self.rag_current_page = 1;
                            self.core.toast_queue.push(
                                "PDF Indexed into RAG Vault",
                                format!("Extracted {} page chunks with citations.", chunks.len()),
                                "📖",
                            );
                        }
                        Err(e) => {
                            self.core.toast_queue.push(
                                "PDF Ingestion Error",
                                format!("Could not index PDF: {}", e),
                                "⚠️",
                            );
                        }
                    }
                }
            }
            Message::Zone3PrevPage => {
                if let Some(active) = &self.rag_active_file {
                    let has_chunks = self.rag_pdf_chunks.iter().any(|c| &c.file_name == active);
                    if has_chunks {
                        if self.rag_current_page > 1 {
                            self.rag_current_page -= 1;
                        }
                    } else if self.core.study_reader.active_tab_index > 0 {
                        self.core.study_reader.active_tab_index -= 1;
                    }
                } else if self.core.study_reader.active_tab_index > 0 {
                    self.core.study_reader.active_tab_index -= 1;
                }
            }
            Message::Zone3NextPage => {
                if let Some(active) = &self.rag_active_file {
                    let max_pg = self
                        .rag_pdf_chunks
                        .iter()
                        .filter(|c| &c.file_name == active)
                        .map(|c| c.page_number as usize)
                        .max();
                    if let Some(max_p) = max_pg {
                        if self.rag_current_page < max_p {
                            self.rag_current_page += 1;
                        }
                    } else if self.core.study_reader.active_tab_index + 1
                        < mindset_core::STUDY_TOPICS.len()
                    {
                        self.core.study_reader.active_tab_index += 1;
                    }
                } else if self.core.study_reader.active_tab_index + 1
                    < mindset_core::STUDY_TOPICS.len()
                {
                    self.core.study_reader.active_tab_index += 1;
                }
            }
            Message::Zone3SelectPdf(name) => {
                self.rag_active_file = Some(name);
                self.rag_current_page = 1;
            }
            Message::Zone3SearchChanged(q) => {
                self.rag_query_input = q;
                if self.rag_query_input.trim().is_empty() {
                    self.rag_citations.clear();
                } else {
                    self.rag_citations = self
                        .rag_vault
                        .search_pdf_chunks(&self.rag_query_input, 5)
                        .unwrap_or_default();
                }
            }

            // ---------------------------------------------------------------
            // Academic Search (Zone 4)
            // ---------------------------------------------------------------
            Message::Zone4SearchInputChanged(q) => {
                self.academic_search_input = q;
            }
            Message::Zone4SelectProvider(prov) => {
                self.academic_search_provider = prov;
            }
            Message::Zone4ExecuteSearch => {
                let query = self.academic_search_input.trim().to_string();
                match self.academic_search_provider {
                    mindset_intelligence::SearchProviderType::Encyclopedia => {
                        self.academic_search_loading = false;
                        self.academic_search_error = None;
                        self.academic_search.search(&query, &[]);
                    }
                    mindset_intelligence::SearchProviderType::Google => {
                        self.academic_search_loading = true;
                        self.academic_search_error = None;
                        return Task::perform(
                            async move {
                                use mindset_intelligence::WebSearchProvider;
                                let conn = mindset_intelligence::GoogleSearchConnector::new();
                                conn.execute_query(&query).await
                            },
                            Message::Zone4SearchCompleted,
                        );
                    }
                    mindset_intelligence::SearchProviderType::DuckDuckGo => {
                        self.academic_search_loading = true;
                        self.academic_search_error = None;
                        return Task::perform(
                            async move {
                                use mindset_intelligence::WebSearchProvider;
                                let conn = mindset_intelligence::DuckDuckGoConnector::new();
                                conn.execute_query(&query).await
                            },
                            Message::Zone4SearchCompleted,
                        );
                    }
                    mindset_intelligence::SearchProviderType::ArXiv => {
                        self.academic_search_loading = true;
                        self.academic_search_error = None;
                        return Task::perform(
                            async move {
                                use mindset_intelligence::WebSearchProvider;
                                let conn = mindset_intelligence::ArXivConnector::new();
                                conn.execute_query(&query).await
                            },
                            Message::Zone4SearchCompleted,
                        );
                    }
                }
            }
            Message::Zone4SearchCompleted(res) => {
                self.academic_search_loading = false;
                match res {
                    Ok(raw) => {
                        let clean = self.academic_search.shield.filter_and_sanitize_results(raw);
                        self.academic_search.results = clean;
                        self.academic_search_error = None;
                    }
                    Err(err) => {
                        self.academic_search_error = Some(err);
                    }
                }
            }

            // ---------------------------------------------------------------
            // Personalization Engine
            // ---------------------------------------------------------------
            Message::SubmitFatigueCheckin(energy, hours) => {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let (cap_mins, brk_mins) =
                    mindset_intelligence::DailyFatigueCheckin::compute_pacing(energy, hours);
                let checkin = mindset_intelligence::DailyFatigueCheckin {
                    date: today_str,
                    energy_level: energy,
                    available_hours_today: hours,
                    situation_note: format!("{} energy check-in", energy.as_str()),
                    recommended_capsule_minutes: cap_mins,
                    recommended_break_minutes: brk_mins,
                };
                if self
                    .personalization
                    .record_fatigue_checkin(&checkin)
                    .is_ok()
                {
                    self.today_checkin = Some(checkin);
                    self.core.pomodoro.set_duration(cap_mins);
                    self.core.toast_queue.push(
                        "Pacing Adapted",
                        format!(
                            "{}m focus / {}m break set for {} energy.",
                            cap_mins,
                            brk_mins,
                            energy.as_str()
                        ),
                        "⚡",
                    );
                }
            }
            Message::ReviewLeitnerCard { id, success } => {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let _ = self
                    .personalization
                    .record_card_review(&id, success, &today_str);
                self.due_leitner_cards = self
                    .personalization
                    .get_due_leitner_cards(&today_str)
                    .unwrap_or_default();
            }
            Message::AddLeitnerCard {
                front,
                back,
                subject,
            } => {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let id = format!("card-{}", uuid::Uuid::new_v4().simple());
                let _ = self
                    .personalization
                    .add_leitner_card(&id, &front, &back, &subject);
                self.due_leitner_cards = self
                    .personalization
                    .get_due_leitner_cards(&today_str)
                    .unwrap_or_default();
            }
            Message::ResolveStudentMistake(mistake_id) => {
                let _ = self.personalization.resolve_mistake(mistake_id);
                self.unresolved_mistakes = self
                    .personalization
                    .get_unresolved_mistakes(None)
                    .unwrap_or_default();
            }

            // ---------------------------------------------------------------
            // 590/600 Exam Paper Reviewer
            // ---------------------------------------------------------------
            Message::PickExamPaperToGrade => {
                return Task::perform(
                    async {
                        let file = rfd::AsyncFileDialog::new()
                            .add_filter("Student Exam Paper", &["pdf", "txt"])
                            .set_title("Select Student Answer Sheet (PDF or TXT)")
                            .pick_file()
                            .await;
                        file.map(|f| f.path().to_path_buf())
                    },
                    Message::EvaluateExamPaperPath,
                );
            }
            Message::EvaluateExamPaperPath(opt_path) => {
                if let Some(path) = opt_path {
                    let rubric_path = std::path::Path::new(
                        "tests/benchmarks/exam_rubrics/tn_stateboard_590_rubric.json",
                    );
                    match mindset_core::agent::ExamReviewer::load_rubric(rubric_path) {
                        Ok(rubric) => {
                            match mindset_core::agent::ExamReviewer::evaluate(&path, &rubric) {
                                Ok(sheet) => {
                                    self.exam_eval_status = Some(format!(
                                        "Graded '{}' — {:.1}/{:.1} ({:.1}%)",
                                        path.file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("paper"),
                                        sheet.total_score,
                                        sheet.max_marks,
                                        sheet.percentage
                                    ));
                                    self.active_exam_evaluation = Some(sheet);
                                    if self.core.wm.active_overlay != ActiveOverlay::ExamReport {
                                        self.core.wm.toggle_overlay(ActiveOverlay::ExamReport);
                                    }
                                }
                                Err(e) => {
                                    self.exam_eval_status = Some(format!("Grading error: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            self.exam_eval_status = Some(format!("Rubric load error: {}", e));
                        }
                    }
                }
            }

            // ---------------------------------------------------------------
            // Competitive Exam MCQ Generator
            // ---------------------------------------------------------------
            Message::GenerateCompetitiveMcq {
                target,
                subject,
                topic,
                count,
            } => {
                let paper_res = match target {
                    mindset_core::agent::ExamTarget::JeeMain
                    | mindset_core::agent::ExamTarget::JeeAdvanced => {
                        let sc = count.max(1);
                        self.mcq_generator
                            .generate_jee_paper(&subject, topic.as_deref(), sc, 1)
                    }
                    mindset_core::agent::ExamTarget::TnpscGroup1 => self
                        .mcq_generator
                        .generate_tnpsc_paper(topic.as_deref(), count.max(1)),
                    mindset_core::agent::ExamTarget::UpscCse => self
                        .mcq_generator
                        .generate_upsc_paper(topic.as_deref(), count.max(1)),
                };
                match paper_res {
                    Ok(paper) => {
                        self.active_mcq_paper = Some(paper);
                        self.mcq_answers.clear();
                        self.mcq_report = None;
                    }
                    Err(e) => {
                        self.core.toast_queue.push("MCQ Generator", e, "⚠️");
                    }
                }
            }
            Message::SelectCompetitiveMcqOption(q_id, opt_idx) => {
                self.mcq_answers
                    .insert(q_id, mindset_core::agent::StudentAnswer::Option(opt_idx));
            }
            Message::SubmitCompetitiveMcq => {
                if let Some(paper) = &self.active_mcq_paper {
                    let submission = mindset_core::agent::StudentSubmission {
                        answers: self.mcq_answers.clone(),
                    };
                    let report = self.mcq_generator.evaluate_submission(paper, &submission);
                    for qr in &report.question_results {
                        if !qr.is_correct && qr.student_answer != "Unattempted" {
                            let _ = self.personalization.record_mistake(
                                &paper.subject,
                                &qr.topic,
                                &qr.question_id,
                                &qr.student_answer,
                                &qr.correct_answer,
                                qr.distractor_or_elimination_insight
                                    .as_deref()
                                    .unwrap_or("MCQ Trap"),
                            );
                        }
                    }
                    self.unresolved_mistakes = self
                        .personalization
                        .get_unresolved_mistakes(None)
                        .unwrap_or_default();
                    self.mcq_report = Some(report);
                }
            }
            Message::EventOccurred(_) => {}
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let palette = ThemePalette::for_mode(self.core.preferences.theme);

        // 0. Liquidmorphism "HELLO ACHIEVER" Boot Sequence Gate
        if self.boot_splash.is_active {
            return shell::boot_splash::view(&self.boot_splash);
        }

        // 1. First-Boot Personalization Wizard (OOBE) Gate
        if !self.profile.onboarding_completed {
            return onboarding::view(&self.onboarding_data, palette);
        }

        // 1. Dynamic Top Menu Bar
        let is_audio_playing = self.audio_player.is_playing;
        let top_bar = shell::top_bar::view(
            &self.core,
            palette,
            &self.profile.language_code,
            is_audio_playing,
            self.tunnel_vision_active,
            &self.network_status,
        );

        // 2. Base View: Student OS Desktop vs Fullscreen Today Dashboard
        let base_content: Element<'_, Message> = if !self.core.preferences.show_desktop
            && self.core.wm.is_open(WindowId::Today)
            && self.core.wm.active_window() == Some(WindowId::Today)
        {
            apps::today::view(&self.core, &self.profile, &self.capsules, palette)
        } else {
            shell::desktop::view(
                &self.core,
                &self.system,
                &self.profile,
                &self.capsules,
                &self.odyssey_map_state,
                &self.search_query,
                self.wallpaper_handle.as_ref(),
                palette,
                self.tunnel_vision_active,
                self.hud_visible,
            )
        };

        // 3. Floating Application Windows with Exact Safe-Area Positioning & Z-Stack Layering
        let mut workspace_stack = stack![base_content];

        for &id in &self.core.wm.z_stack {
            if id != WindowId::Today || self.core.preferences.show_desktop {
                if let Some(win) = self.core.wm.get_window(id) {
                    if win.is_open && win.mode != WindowMode::Minimized {
                        let win_view = apps::render_window(
                            win,
                            &self.core,
                            &self.system,
                            &self.odyssey_map_state,
                            &self.profile,
                            &self.capsules,
                            palette,
                            &self.chat_input,
                            &self.new_task_input,
                            &self.sysinfo.metrics,
                            &self.threads,
                            &self.active_messages,
                            &self.active_thread_id,
                            self.is_generating,
                            &self.active_model,
                            &self.memories,
                            &self.new_memory_input,
                            self.auto_speak_enabled,
                            &self.ai_activity_log,
                            &self.browser_url,
                            self.browser_focus_shield,
                            self.browser_status_note.as_deref(),
                            &self.network_status,
                            &self.rag_pdf_chunks,
                            self.rag_active_file.as_deref(),
                            self.rag_current_page,
                            &self.rag_query_input,
                            &self.rag_citations,
                            &self.academic_search,
                            &self.academic_search_input,
                            self.academic_search_provider,
                            self.academic_search_loading,
                            self.academic_search_error.as_deref(),
                            self.today_checkin.as_ref(),
                            &self.due_leitner_cards,
                            &self.unresolved_mistakes,
                            self.last_parsed_query.as_ref(),
                            self.active_mcq_paper.as_ref(),
                            &self.mcq_answers,
                            self.mcq_report.as_ref(),
                            &self.installed_models,
                            self.maindy_screen,
                            self.maindy_drawer_tab,
                            self.maindy_left_rail_open,
                            self.maindy_right_drawer_open,
                            self.maindy_engine_popover_open,
                            &self.study_capsules,
                            self.active_capsule_id.as_deref(),
                            self.active_capsule_phase,
                            self.capsule_hint_open,
                            self.pending_destructive_action.as_ref(),
                            &self.youtube_study,
                            &self.mindstore_state,
                        );

                        let positioned_win: Element<'_, Message> =
                            if win.mode == WindowMode::Maximized {
                                win_view
                            } else {
                                let rel_y = (win.y - wm::TOP_BAR_SAFE_AREA).max(0.0);
                                let rel_x = win.x.max(0.0);
                                container(win_view)
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .padding(iced::Padding {
                                        top: rel_y,
                                        left: rel_x,
                                        right: 0.0,
                                        bottom: 0.0,
                                    })
                                    .into()
                            };

                        workspace_stack = workspace_stack.push(positioned_win);
                    }
                }
            }
        }

        let neu = crate::theme::neumorphism::NeuPalette::for_theme(self.core.preferences.theme);

        // 4. Floating Bottom Dock (Unmounted in Tunnel Vision)
        let dock_scales: Vec<f32> = self.dock_springs.iter().map(|s| s.position).collect();
        let dock: Element<'_, Message> =
            if self.tunnel_vision_active && self.tunnel_vision_spring.position <= 0.01 {
                container(row![]).into()
            } else {
                container(
                    row![
                        h_space(),
                        shell::dock::view(
                            &self.core,
                            &dock_scales,
                            self.hovered_dock_index,
                            palette,
                            &neu,
                        ),
                        h_space(),
                    ]
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 0.0,
                    right: 0.0,
                    bottom: 12.0,
                    left: 0.0,
                })
                .into()
            };

        // 4.1 Floating Claymorphic Maindy AI Orb (Unmounted in Tunnel Vision)
        let maindy_orb_element: Element<'_, Message> =
            if self.tunnel_vision_active && self.tunnel_vision_spring.position <= 0.01 {
                container(row![]).into()
            } else {
                container(shell::maindy_orb::view(
                    self.maindy_spring.position,
                    self.maindy_drawer_open,
                    &self.chat_input,
                    &self.active_messages,
                    self.is_generating,
                    &neu,
                ))
                .padding(iced::Padding {
                    top: 0.0,
                    right: 28.0,
                    bottom: 84.0,
                    left: 0.0,
                })
                .align_x(Alignment::End)
                .align_y(Alignment::End)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            };

        workspace_stack = workspace_stack
            .push(column![v_space(), dock])
            .push(maindy_orb_element);

        // 5. Stack Main Workspace
        let workspace = column![
            top_bar,
            container(workspace_stack)
                .width(Length::Fill)
                .height(Length::Fill),
        ];

        // 6. Modal Backdrop & Overlays
        let modal_layer = views::modals::view(
            self.core.wm.active_overlay,
            &self.core,
            palette,
            &self.search_query,
            &self.stage1_selections,
            &self.stage2_selections,
            &self.feynman_input,
            self.excuse_selected_answer,
            &self.pending_agent_action,
            &self.network_status,
            &self.nearby_networks,
            self.wifi_menu_open,
            self.wifi_connecting_ssid.as_deref(),
            &self.wifi_password_input,
            &self.lock_screen,
            &self.profile,
            self.active_exam_evaluation.as_ref(),
            self.exam_eval_status.as_deref(),
            self.hud_visible,
        );

        // 7. Toast Notifications
        let toast_layer = views::toasts::view(&self.core.toast_queue, palette);

        // 8. Dynamic Island Morphing Notch Layer
        let island_layer = container(shell::dynamic_island::view(
            self.island_mode,
            self.island_width_spring.position,
            self.island_height_spring.position,
            &self.core,
            &self.profile,
            &self.capsules,
            &self.island_search_query,
            self.audio_player.is_playing,
            palette,
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Start)
        .padding(iced::Padding {
            top: 6.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        });

        // 9. Cosmic Wallpaper Background Layer (Cached Handle or Procedural Fallback)
        let bg_element: Element<'_, Message> = if let Some(handle) = &self.wallpaper_handle {
            iced::widget::image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(iced::ContentFit::Cover)
                .into()
        } else {
            iced::widget::canvas(shell::wallpaper::ProceduralCosmicWallpaper {
                cache: &self.wallpaper_canvas_cache,
                palette,
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        };

        let mut root_stack = stack![bg_element, workspace]
            .width(Length::Fill)
            .height(Length::Fill);

        // Transparent backdrop click-catcher when Dynamic Island or Maindy drawer is expanded
        let transient_shell_popover_open = matches!(
            self.island_mode,
            shell::dynamic_island::IslandMode::Search
                | shell::dynamic_island::IslandMode::ControlCenter
        ) || self.maindy_drawer_open;
        if transient_shell_popover_open && self.core.wm.active_overlay == ActiveOverlay::None {
            let dismiss_layer =
                mouse_area(container(row![]).width(Length::Fill).height(Length::Fill))
                    .on_press(Message::CloseOverlays);
            root_stack = root_stack.push(dismiss_layer);
        }

        root_stack = root_stack.push(island_layer);

        if let Some(m) = modal_layer {
            root_stack = root_stack.push(m);
        }
        if let Some(t) = toast_layer {
            root_stack = root_stack.push(t);
        }

        // 10. Hardware/Software Brightness & Night Light Click-Through Scrim Overlay
        let scrim = shell::scrim::scrim_overlay(
            self.core.preferences.brightness,
            self.core.preferences.focus_dnd,
        );
        root_stack = root_stack.push(scrim);

        container(root_stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(palette.background)),
                text_color: Some(palette.text_primary),
                ..Default::default()
            })
            .into()
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Zero-Idle-Waste: Run time subscription only at 1 Hz for clock/pomodoro,
        // and every 4 seconds for sysinfo. window::frames() is subscribed ONLY when
        // springs are actively in motion (!is_at_rest()). When springs settle,
        // the frame subscription drops, maintaining strict 0% CPU idle usage.
        let mut subs = vec![
            iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick),
            iced::time::every(Duration::from_secs(4)).map(|_| Message::SysinfoTick),
            event::listen().map(Message::EventOccurred),
        ];

        let low_overhead = std::env::var("MINDSET_LOW_OVERHEAD").ok().as_deref() == Some("1");

        // Liquidmorphic boot animation timer (only while boot splash is active)
        if self.boot_splash.is_active {
            let boot_ms = if low_overhead { 45 } else { 33 };
            subs.push(
                iced::time::every(Duration::from_millis(boot_ms)).map(|_| Message::BootSplashTick),
            );
        }

        // 18 FPS (~55 ms) mild ambient motion for 3D Odyssey path ONLY when the Odyssey map
        // is actually visible on screen, preventing background CPU burn in VMs at idle.
        let odyssey_visible = self.core.wm.widget_layout.free_placement
            || self.core.wm.is_open(WindowId::OdysseyMap)
            || self.core.wm.active_overlay == ActiveOverlay::OdysseyMap;
        if self.profile.mild_motion_enabled && !self.tunnel_vision_active && odyssey_visible {
            let map_ms = if low_overhead { 75 } else { 55 };
            subs.push(
                iced::time::every(Duration::from_millis(map_ms)).map(|_| Message::MapAmbientTick),
            );
        }

        let springs_moving = !self.tunnel_vision_spring.is_at_rest()
            || !self.maindy_spring.is_at_rest()
            || !self.island_width_spring.is_at_rest()
            || !self.island_height_spring.is_at_rest()
            || self.dock_springs.iter().any(|s| !s.is_at_rest());

        if springs_moving || self.smoke_frames_remaining.is_some() {
            subs.push(window::frames().map(Message::AnimationFrame));
        }

        Subscription::batch(subs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    fn temp_app(tag: &str) -> (MindSetApp, std::path::PathBuf) {
        let dir = std::env::temp_dir().join(format!(
            "mindset_os_lib_test_{}_{}",
            tag,
            uuid::Uuid::new_v4()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let db_path = dir.join("mindset_os.db");
        (MindSetApp::with_storage_path(db_path), dir)
    }

    #[test]
    fn test_open_exam_report_modal() {
        let (mut app, dir) = temp_app("exam_report");
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);

        let _ = app.update(Message::OpenExamReport);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ExamReport);

        let _ = app.update(Message::CloseOverlays);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_open_verification_boss_and_battle_lifecycle() {
        let (mut app, dir) = temp_app("boss");
        let _ = app.update(Message::OpenVerificationBoss);

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::VerificationBoss);
        assert!(app.core.boss_session.is_some());
        assert!(app.core.active_exam.is_some());

        let initial_hp = app.core.boss_session.as_ref().unwrap().current_hp;
        assert_eq!(initial_hp, 100.0);

        let _ = app.update(Message::SelectBossStage1(0, 0));
        assert_eq!(app.stage1_selections[0], Some(0));
        assert!(app.core.boss_session.as_ref().unwrap().current_hp < initial_hp);

        let explanation = "Nernst equation connects cell potential with reaction quotient because standard potentials shift when ion concentration changes.";
        let _ = app.update(Message::FeynmanInputChanged(explanation.to_string()));
        assert_eq!(app.feynman_input, explanation);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_excuse_roast_and_bailout_flow() {
        let (mut app, dir) = temp_app("excuse");
        let _ = app.update(Message::TriggerExcuseRoast(
            "I need 5 minutes of Instagram reels".to_string(),
        ));

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ExcuseRoast);
        assert!(app.core.excuse_roast.is_some());

        let _ = app.update(Message::AnswerExcuseFormula(0));
        assert_eq!(app.excuse_selected_answer, Some(0));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_break_arcade_card_interaction() {
        let (mut app, dir) = temp_app("arcade");
        let _ = app.update(Message::OpenBreakArcade);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::BreakArcade);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_airplane_mode_interlock_in_app() {
        let (mut app, dir) = temp_app("airplane");
        assert!(app.core.preferences.wifi_enabled);
        assert!(app.core.preferences.bluetooth_enabled);

        let _ = app.update(Message::ToggleAirplaneMode);
        assert!(app.core.preferences.airplane_mode);
        assert!(!app.core.preferences.wifi_enabled);
        assert!(!app.core.preferences.bluetooth_enabled);

        let _ = app.update(Message::ToggleAirplaneMode);
        assert!(!app.core.preferences.airplane_mode);
        assert!(app.core.preferences.wifi_enabled);
        assert!(app.core.preferences.bluetooth_enabled);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scratchpad_persistence() {
        let (mut app, dir) = temp_app("scratchpad");
        let test_note = "E = mc^2 • Photoelectric effect hf = W + KE";
        let _ = app.update(Message::ScratchpadChanged(test_note.to_string()));
        assert_eq!(app.system.scratchpad_text, test_note);

        let _ = app.update(Message::ClearScratchpad);
        assert_eq!(app.system.scratchpad_text, "");

        let _ = app.update(Message::CopyScratchpad);
        assert_eq!(app.system.scratchpad_save_status, "Copied to clipboard!");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_odyssey_embark_and_selection() {
        let (mut app, dir) = temp_app("odyssey");
        assert_eq!(app.core.odyssey.current_stage_index, 0);
        let _ = app.update(Message::EmbarkNode("phy-101".to_string()));
        assert_eq!(app.odyssey_map_state.selected_node, Some(0));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_daily_capsule_generation() {
        let (mut app, dir) = temp_app("capsules");
        let _ = app.update(Message::GenerateDailyCapsules);
        assert!(!app.core.toast_queue.toasts.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_preferences_tab_navigation() {
        let (mut app, dir) = temp_app("prefs");
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::Appearance
        );

        let _ = app.update(Message::SelectPreferencesTab(1));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::FocusShields
        );

        let _ = app.update(Message::SelectPreferencesTab(2));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::AudioRadios
        );

        let _ = app.update(Message::SelectPreferencesTab(3));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::LanguageRegion
        );

        let _ = app.update(Message::SelectPreferencesTab(4));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::MaindyAiKeys
        );

        let _ = app.update(Message::SelectPreferencesTab(5));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::HardwarePower
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_control_center_live_bindings() {
        let (mut app, dir) = temp_app("cc");

        let _ = app.update(Message::BrightnessChanged(0.65));
        assert!((app.core.preferences.brightness - 0.65).abs() < f32::EPSILON);

        let _ = app.update(Message::VolumeChanged(0.40));
        assert!((app.core.preferences.volume - 0.40).abs() < f32::EPSILON);

        let _ = app.update(Message::ThemeSelected(mindset_core::ThemeMode::SolarAmber));
        assert_eq!(
            app.core.preferences.theme,
            mindset_core::ThemeMode::SolarAmber
        );

        assert!(!app.core.preferences.focus_dnd);
        let _ = app.update(Message::ToggleDnd);
        assert!(app.core.preferences.focus_dnd);

        assert!(app.core.preferences.wifi_enabled);
        let _ = app.update(Message::ToggleWifi);
        assert!(!app.core.preferences.wifi_enabled);

        assert!(app.core.preferences.bluetooth_enabled);
        let _ = app.update(Message::ToggleBluetooth);
        assert!(!app.core.preferences.bluetooth_enabled);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_vault_actions() {
        let (mut app, dir) = temp_app("vault");

        let _ = app.update(Message::VaultSelectSidebar("Documents".to_string()));
        let _ = app.update(Message::VaultSearchChanged("Gauss".to_string()));
        let _ = app.update(Message::VaultEntryClicked(
            "README_STUDY_VAULT.txt".to_string(),
            false,
        ));
        let _ = app.update(Message::VaultNavigateUp);
        let _ = app.update(Message::VaultToggleTreeNode("Study Capsules".to_string()));
        let _ = app.update(Message::VaultPdfPrevPage);
        let _ = app.update(Message::VaultPdfNextPage);
        let _ = app.update(Message::VaultPdfJumpPage("1".to_string()));
        let _ = app.update(Message::VaultPptxSelectSlide(1));
        let _ = app.update(Message::VaultPptxTitleChanged("New Title".to_string()));
        let _ = app.update(Message::VaultPptxBodyChanged(
            "New Content Body".to_string(),
        ));
        let _ = app.update(Message::VaultPptxSaveSlide);
        let _ = app.update(Message::VaultTextEditChanged(
            "Edited text line".to_string(),
        ));
        let _ = app.update(Message::VaultTextSave);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_window_drag_lifecycle() {
        let (mut app, dir) = temp_app("drag");
        let id = WindowId::StudyReader;
        let _ = app.update(Message::OpenWindow(id));
        assert!(app.core.wm.is_open(id));

        let initial_win = app.core.wm.get_window(id).unwrap().clone();
        app.cursor_position = Point::new(initial_win.x + 50.0, initial_win.y + 15.0);

        let _ = app.update(Message::StartWindowDrag(id));
        assert!(app.wm_interaction.dragging.is_some());
        let (drag_id, off_x, off_y) = app.wm_interaction.dragging.unwrap();
        assert_eq!(drag_id, id);
        assert!((off_x - 50.0).abs() < f32::EPSILON);
        assert!((off_y - 15.0).abs() < f32::EPSILON);

        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(200.0, 80.0),
            },
        )));

        let moved_win = app.core.wm.get_window(id).unwrap();
        assert_eq!(moved_win.x, 200.0 - 50.0);
        assert_eq!(moved_win.y, 80.0 - 15.0);

        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(300.0, 500.0),
            },
        )));
        let clamped_win = app.core.wm.get_window(id).unwrap();
        assert_eq!(
            clamped_win.y,
            app.wm_interaction.screen_height - wm::DOCK_SAFE_AREA - clamped_win.height
        );

        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::ButtonReleased(mouse::Button::Left),
        )));
        assert!(app.wm_interaction.dragging.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_focus_window_and_bring_to_front() {
        let (mut app, dir) = temp_app("focus");
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        let _ = app.update(Message::OpenWindow(WindowId::Tasks));
        let _ = app.update(Message::OpenWindow(WindowId::FileVault));

        assert_eq!(app.core.wm.active_window(), Some(WindowId::FileVault));

        let _ = app.update(Message::FocusWindow(WindowId::StudyReader));
        assert_eq!(app.core.wm.active_window(), Some(WindowId::StudyReader));
        assert_eq!(app.core.wm.z_stack.last(), Some(&WindowId::StudyReader));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_desktop_widget_drag_and_repositioning() {
        let (mut app, dir) = temp_app("wdrag");
        app.core.wm.widget_layout.reset();
        let widget_id = mindset_core::DesktopWidgetId::ScholarBriefing;

        let initial_widget = app.core.wm.get_widget_placement(widget_id).unwrap().clone();

        app.cursor_position = Point::new(initial_widget.x + 20.0, initial_widget.y + 10.0);

        let _ = app.update(Message::StartWidgetDrag(widget_id));
        assert!(app.wm_interaction.dragging_widget.is_some());
        assert!(app.core.wm.widget_layout.free_placement);

        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(500.0, 150.0),
            },
        )));

        let moved_widget = app.core.wm.get_widget_placement(widget_id).unwrap();
        assert_eq!(moved_widget.x, 500.0 - 20.0);
        assert_eq!(moved_widget.y, 150.0 - 10.0);

        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::ButtonReleased(mouse::Button::Left),
        )));
        assert!(app.wm_interaction.dragging_widget.is_none());

        let _ = app.update(Message::ResetWidgetLayout);
        let reset_widget = app.core.wm.get_widget_placement(widget_id).unwrap();
        assert_eq!(reset_widget.x, initial_widget.x);
        assert_eq!(reset_widget.y, initial_widget.y);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_tunnel_vision_disengage_on_close_study_reader() {
        let (mut app, dir) = temp_app("tv_close");
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        assert!(app.tunnel_vision_active);
        assert_eq!(app.tunnel_vision_spring.target, 0.0);

        let _ = app.update(Message::CloseWindow(WindowId::StudyReader));
        assert!(!app.tunnel_vision_active);
        assert_eq!(app.tunnel_vision_spring.target, 1.0);
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_tunnel_vision_disengage_on_escape() {
        use iced::keyboard::key::Named;
        use iced::keyboard::{Event as KbEvent, Key, Modifiers};

        let (mut app, dir) = temp_app("tv_esc");
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        assert!(app.tunnel_vision_active);

        let _ = app.update(Message::EventOccurred(Event::Keyboard(
            KbEvent::KeyPressed {
                key: Key::Named(Named::Escape),
                modified_key: Key::Named(Named::Escape),
                physical_key: iced::keyboard::key::Physical::Code(
                    iced::keyboard::key::Code::Escape,
                ),
                location: iced::keyboard::Location::Standard,
                modifiers: Modifiers::default(),
                repeat: false,
                text: None,
            },
        )));

        assert!(!app.tunnel_vision_active);
        assert_eq!(app.tunnel_vision_spring.target, 1.0);
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_single_ctrl_k_toggles_island_search() {
        use iced::keyboard::{Event as KbEvent, Key, Modifiers};

        let (mut app, dir) = temp_app("ctrl_k");
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);

        let ctrl_k = || {
            Message::EventOccurred(Event::Keyboard(KbEvent::KeyPressed {
                key: Key::Character("k".into()),
                modified_key: Key::Character("k".into()),
                physical_key: iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::KeyK),
                location: iced::keyboard::Location::Standard,
                modifiers: Modifiers::CTRL,
                repeat: false,
                text: None,
            }))
        };

        let _ = app.update(ctrl_k());
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Search);

        let _ = app.update(ctrl_k());
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_lock_screen_lockout_reset_and_blank_validation() {
        let (mut app, dir) = temp_app("lock_reset");
        app.lock_screen.is_locked_out = true;
        app.lock_screen.failed_attempts = 5;
        app.lock_screen.lockout_seconds_remaining = 1;

        let _ = app.update(Message::Tick);
        assert!(!app.lock_screen.is_locked_out);
        assert_eq!(app.lock_screen.lockout_seconds_remaining, 0);
        assert_eq!(app.lock_screen.failed_attempts, 0);

        // Short PIN rejected before bcrypt / failed_attempts increment
        app.lock_screen.mode = shell::lock_screen::LockAuthMode::Pin;
        app.lock_screen.pin_input = "123".to_string();
        let _ = app.update(Message::SubmitLockScreenAuth);
        assert_eq!(app.lock_screen.failed_attempts, 0);
        assert!(app.lock_screen.error_message.is_some());

        // Blank password rejected before bcrypt / failed_attempts increment
        app.lock_screen.mode = shell::lock_screen::LockAuthMode::Password;
        app.lock_screen.password_input = "   ".to_string();
        let _ = app.update(Message::SubmitLockScreenAuth);
        assert_eq!(app.lock_screen.failed_attempts, 0);
        assert!(app.lock_screen.error_message.is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_personalization_and_mcq_wiring() {
        let (mut app, dir) = temp_app("wiring");

        // 1. Fatigue Checkin
        let _ = app.update(Message::SubmitFatigueCheckin(
            mindset_intelligence::FatigueLevel::Fatigued,
            2.0,
        ));
        assert!(app.today_checkin.is_some());
        assert_eq!(
            app.today_checkin
                .as_ref()
                .unwrap()
                .recommended_capsule_minutes,
            18
        );
        assert_eq!(app.core.pomodoro.duration_mins, 18);

        // 2. Leitner card add & review
        let _ = app.update(Message::AddLeitnerCard {
            front: "Snell's Law".to_string(),
            back: "n1 sin i = n2 sin r".to_string(),
            subject: "Physics".to_string(),
        });
        assert_eq!(app.due_leitner_cards.len(), 1);
        let card_id = app.due_leitner_cards[0].id.clone();
        let _ = app.update(Message::ReviewLeitnerCard {
            id: card_id,
            success: true,
        });
        assert!(app.due_leitner_cards.is_empty());

        // 3. Competitive MCQ generation & submission
        let _ = app.update(Message::GenerateCompetitiveMcq {
            target: mindset_core::agent::ExamTarget::JeeMain,
            subject: "Physics".to_string(),
            topic: None,
            count: 2,
        });
        assert!(app.active_mcq_paper.is_some());
        let first_q_id = app.active_mcq_paper.as_ref().unwrap().questions[0]
            .id
            .clone();
        let _ = app.update(Message::SelectCompetitiveMcqOption(first_q_id, 0));
        let _ = app.update(Message::SubmitCompetitiveMcq);
        assert!(app.mcq_report.is_some());

        // 4. Zone 4 Encyclopedia search
        let _ = app.update(Message::Zone4SearchInputChanged("Gauss".to_string()));
        let _ = app.update(Message::Zone4ExecuteSearch);
        assert!(!app.academic_search.results.is_empty());

        // 5. Resolve student mistake roundtrip
        let _ = app.personalization.record_mistake(
            "Physics",
            "Wave Optics",
            "Fringe width dependence on d",
            "proportional",
            "inversely proportional",
            "Proportionality",
        );
        app.unresolved_mistakes = app
            .personalization
            .get_unresolved_mistakes(None)
            .unwrap_or_default();
        assert!(!app.unresolved_mistakes.is_empty());
        let m_id = app.unresolved_mistakes[0].id;
        let _ = app.update(Message::ResolveStudentMistake(m_id));
        assert!(app.unresolved_mistakes.iter().all(|m| m.id != m_id));

        // 6. Exam Reviewer evaluation wiring (when benchmark exists)
        let sample_exam =
            std::path::PathBuf::from("tests/benchmarks/sample_exams/tn_class12_physics_590.txt");
        if sample_exam.exists() {
            let _ = app.update(Message::EvaluateExamPaperPath(Some(sample_exam)));
            assert!(app.active_exam_evaluation.is_some());
            assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ExamReport);
        }

        // 7. Zone 3 RAG PDF ingestion, pagination & search
        let pdf_path = dir.join("sample_textbook.pdf");
        let pdf_content = mindset_sys::fs::document::DocumentContent::Pdf {
            pages: vec![
                mindset_sys::fs::document::PdfPage {
                    page_number: 1,
                    text: "Chapter 1: Electrostatics\nFormula: Phi_E = Q / epsilon_0".to_string(),
                },
                mindset_sys::fs::document::PdfPage {
                    page_number: 2,
                    text: "Chapter 10: Wave Optics\nFormula: beta = lambda * D / d".to_string(),
                },
            ],
            metadata: mindset_sys::fs::document::PdfMetadata::default(),
        };
        mindset_sys::fs::document::DocumentHandler::save_update(&pdf_path, &pdf_content).unwrap();
        let _ = app.update(Message::IngestPdfPath(Some(pdf_path)));
        assert_eq!(app.rag_active_file.as_deref(), Some("sample_textbook.pdf"));
        assert_eq!(app.rag_current_page, 1);
        let _ = app.update(Message::Zone3NextPage);
        assert_eq!(app.rag_current_page, 2);
        let _ = app.update(Message::Zone3PrevPage);
        assert_eq!(app.rag_current_page, 1);
        let _ = app.update(Message::Zone3SearchChanged("Wave Optics".to_string()));
        assert!(!app.rag_citations.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_personalized_prompt_builder() {
        let profile = storage::db::UserProfile {
            name: "Mapla".to_string(),
            career_track: "Class12BoardsTN600".to_string(),
            target_score_value: 590,
            target_score_max: 600,
            maindy_style: "thanglish".to_string(),
            coaching_style: "Friendly Thanglish Bro".to_string(),
            weak_topics: vec!["Electrochemistry Nernst Eq".to_string()],
            ..Default::default()
        };

        let memories = vec![storage::db::UserMemory {
            id: 1,
            memory_text: "Strongest in Calculus".to_string(),
            category: "Academic Strength".to_string(),
            created_at: "2026-11-21T00:00:00Z".to_string(),
        }];

        let capsules = vec![storage::db::CapsuleRecord {
            id: "cap-1".to_string(),
            target_date: "2026-11-21".to_string(),
            subject: "Physics".to_string(),
            unit: "Gauss Law & Flux".to_string(),
            estimated_minutes: 40,
            xp_reward: 100,
            status: "Pending".to_string(),
            stage: "Heavy Lift".to_string(),
        }];

        let prompt =
            ai::prompt_builder::build_personalized_system_prompt(&profile, &memories, &capsules);

        assert!(prompt.contains("Mapla"));
        assert!(prompt.contains("590/600"));
        assert!(prompt.contains("Strongest in Calculus"));
        assert!(prompt.contains("Gauss Law & Flux"));
        assert!(prompt.contains("THANGLISH"));
        assert!(prompt.contains("ZERO ROBOTIC META-COMMENTARY"));
        assert!(prompt.contains("DISPLAY:"));
        assert!(prompt.contains("SPEECH:"));
    }

    #[test]
    fn test_agent_permission_interlock_and_capsule_creation() {
        let (mut app, dir) = temp_app("agent_perm");
        app.profile.onboarding_completed = true;

        let action_raw = r#"I'm preparing to read your notes.
<os_action>
{
  "action": "ReadDocument",
  "params": {
    "file_path": "Formula Sheets/Physics_Formula_Sheet.txt"
  }
}
</os_action>"#;

        app.active_messages.push(storage::ChatMessageRecord {
            id: 999,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:00:00".to_string(),
        });

        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::AgentPermission);
        assert!(app.pending_agent_action.is_some());

        let _ = app.update(Message::AgentPermissionResponse(
            ai::agent::AgentPermissionDecision::Deny,
        ));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert!(app.pending_agent_action.is_none());
        assert!(app
            .active_messages
            .last()
            .unwrap()
            .content
            .contains("Permission was denied"));

        app.active_messages.push(storage::ChatMessageRecord {
            id: 1000,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:01:00".to_string(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::AgentPermission);

        let _ = app.update(Message::AgentPermissionResponse(
            ai::agent::AgentPermissionDecision::AllowSession,
        ));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert!(app.session_allowed_folders.contains("Formula Sheets"));

        app.active_messages.push(storage::ChatMessageRecord {
            id: 1001,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:02:00".to_string(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);

        let _ = app.update(Message::RevokeAllAgentPermissions);
        assert!(app.session_allowed_folders.is_empty());

        let initial_capsule_count = app.capsules.len();
        let initial_odyssey_count = app.core.odyssey.nodes.len();
        let capsule_action_raw = r#"Done!
<os_action>
{
  "action": "CreateAndShowCapsule",
  "params": {
    "subject": "Physics",
    "topic": "Dual Nature of Radiation",
    "duration_mins": 30,
    "priority": "High",
    "notes_summary": "Einstein photoelectric equation hf = W + KE."
  }
}
</os_action>"#;

        app.active_messages.push(storage::ChatMessageRecord {
            id: 1002,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: capsule_action_raw.to_string(),
            created_at: "2026-11-21 12:03:00".to_string(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert_eq!(app.capsules.len(), initial_capsule_count + 1);
        assert_eq!(app.core.odyssey.nodes.len(), initial_odyssey_count + 1);
        assert!(app.core.wm.is_open(WindowId::StudyReader));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_browser_and_youtube_study_navigation() {
        let (mut app, dir) = temp_app("browser");

        let _ = app.update(Message::OpenYouTubeStudy);
        assert_eq!(app.browser_url, "https://www.youtube.com");
        assert!(app.core.wm.is_open(WindowId::SearchBrowser));
        assert_eq!(app.browser_history.len(), 1);

        let _ = app.update(Message::BrowserNavigate(
            "https://ncert.nic.in/textbook.php".to_string(),
        ));
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history.len(), 2);
        assert_eq!(app.browser_history_idx, 1);

        let _ = app.update(Message::BrowserNavigate(
            "class 12 physics derivations".to_string(),
        ));
        assert_eq!(
            app.browser_url,
            "https://www.youtube.com/results?search_query=class+12+physics+derivations"
        );
        assert_eq!(app.browser_history.len(), 3);
        assert_eq!(app.browser_history_idx, 2);

        let _ = app.update(Message::BrowserBack);
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history_idx, 1);

        let _ = app.update(Message::BrowserBack);
        assert_eq!(app.browser_url, "https://www.youtube.com");
        assert_eq!(app.browser_history_idx, 0);

        let _ = app.update(Message::BrowserForward);
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history_idx, 1);

        assert!(app.browser_focus_shield);
        let _ = app.update(Message::ToggleBrowserFocusShield);
        assert!(!app.browser_focus_shield);
        let _ = app.update(Message::ToggleBrowserFocusShield);
        assert!(app.browser_focus_shield);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_network_and_wifi_manager_state() {
        let (mut app, dir) = temp_app("wifi");

        assert!(!app.wifi_menu_open);
        assert!(app.wifi_connecting_ssid.is_none());

        let _ = app.update(Message::ToggleWifiMenu);
        assert!(app.wifi_menu_open);

        let _ = app.update(Message::SelectWifiNetwork("Campus-5G".to_string()));
        assert_eq!(app.wifi_connecting_ssid.as_deref(), Some("Campus-5G"));

        let _ = app.update(Message::WifiPasswordInputChanged(
            "StudyFocus2026".to_string(),
        ));
        assert_eq!(app.wifi_password_input, "StudyFocus2026");

        let _ = app.update(Message::CancelWifiConnect);
        assert!(app.wifi_connecting_ssid.is_none());
        assert!(app.wifi_password_input.is_empty());

        let custom_status = mindset_sys::network::NetworkStatus {
            is_wifi_enabled: true,
            is_online: true,
            connected_ssid: Some("MindSet-HighSpeed-Mesh".to_string()),
            ip_address: Some("192.168.1.105".to_string()),
            signal_percent: 92,
            latency_ms: Some(14),
        };
        let _ = app.update(Message::NetworkStatusUpdated(custom_status));
        assert!(app.network_status.is_online);
        assert_eq!(
            app.network_status.connected_ssid.as_deref(),
            Some("MindSet-HighSpeed-Mesh")
        );
        assert_eq!(app.network_status.signal_percent, 92);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_popover_strict_mutual_exclusion_and_dismissal() {
        let (mut app, dir) = temp_app("popover_mutex");
        app.boot_splash.skip();
        app.profile.onboarding_completed = true;

        // 1. Open Control Center -> opens cleanly
        let _ = app.update(Message::ToggleOverlay(ActiveOverlay::ControlCenter));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ControlCenter);

        // 2. Open Dynamic Island Search -> MUST close Control Center & Maindy drawer
        let _ = app.update(Message::ToggleMaindyDrawer);
        assert!(app.maindy_drawer_open);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);

        let _ = app.update(Message::SetIslandMode(
            shell::dynamic_island::IslandMode::Search,
        ));
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Search);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert!(!app.maindy_drawer_open);

        // 3. Open Launcher -> MUST collapse Dynamic Island Search
        let _ = app.update(Message::ToggleOverlay(ActiveOverlay::Launcher));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::Launcher);
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);

        // 4. Open SystemMenu ("MS OS") -> closes Launcher cleanly
        let _ = app.update(Message::ToggleOverlay(ActiveOverlay::SystemMenu));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::SystemMenu);

        // 5. CloseOverlays (outside-click dismiss handler) clears everything
        let _ = app.update(Message::CloseOverlays);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);
        assert!(!app.maindy_drawer_open);

        // 6. LockScreen security interlock: Escape NEVER dismisses LockScreen
        let _ = app.update(Message::SystemLogout);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::LockScreen);
        let _ = app.update(Message::EventOccurred(Event::Keyboard(
            iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
                modified_key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
                physical_key: iced::keyboard::key::Physical::Code(
                    iced::keyboard::key::Code::Escape,
                ),
                location: iced::keyboard::Location::Standard,
                modifiers: iced::keyboard::Modifiers::default(),
                repeat: false,
                text: None,
            },
        )));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::LockScreen);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_wifi_ssid_ellipsis_truncation_prevents_dnd_collision() {
        let long_ssid = "MindSet_University_5GHz_UltraLong";
        let clipped = shell::control_center::truncate_ellipsis(long_ssid, 9);
        assert_eq!(clipped.chars().count(), 9);
        assert!(clipped.ends_with('…'));

        let short_ssid = "HomeWiFi";
        assert_eq!(
            shell::control_center::truncate_ellipsis(short_ssid, 9),
            "HomeWiFi"
        );

        // Multibyte UTF-8 safety (Tamil / Emoji)
        let tamil_ssid = "தமிழ்_வயர்லெஸ்_நெட்வொர்க்";
        let tamil_clipped = shell::control_center::truncate_ellipsis(tamil_ssid, 8);
        assert_eq!(tamil_clipped.chars().count(), 8);
        assert!(tamil_clipped.ends_with('…'));
    }

    #[test]
    fn test_three_wallpaper_wcag_aa_contrast_matrix() {
        use shell::wallpaper::WallpaperProfile;
        use theme::{adaptive_frost_tint, blend_frost_tint, composite_over, wcag_contrast_ratio};

        let profiles = [
            WallpaperProfile::BrightMinimalist,
            WallpaperProfile::DarkNeon,
            WallpaperProfile::HighDetailBusy,
        ];
        let modes = [
            ThemeMode::AlpineGlass,
            ThemeMode::DarkSpace,
            ThemeMode::VioletNebula,
            ThemeMode::SolarAmber,
        ];

        for mode in modes {
            let palette = ThemePalette::for_mode(mode);
            let tint = adaptive_frost_tint(palette);
            let frost_surface = blend_frost_tint(
                iced::Color { a: 0.28, ..tint },
                palette.glass_surface_elevated,
            );

            for wp in profiles {
                let backdrop = wp.sample_backdrop_color();
                let composited_bg = composite_over(frost_surface, backdrop);
                let primary_ratio = wcag_contrast_ratio(palette.text_primary, composited_bg);
                let secondary_ratio = wcag_contrast_ratio(palette.text_secondary, composited_bg);
                let muted_ratio = wcag_contrast_ratio(palette.text_muted, composited_bg);

                assert!(
                    primary_ratio >= 7.0,
                    "Primary text must pass WCAG AAA (>= 7.0) on {:?} / {:?}, got {:.2}",
                    mode,
                    wp,
                    primary_ratio
                );
                assert!(
                    secondary_ratio >= 4.5,
                    "Secondary text must pass WCAG AA (>= 4.5) on {:?} / {:?}, got {:.2}",
                    mode,
                    wp,
                    secondary_ratio
                );
                assert!(
                    muted_ratio >= 4.5,
                    "Muted text must pass WCAG AA (>= 4.5) on {:?} / {:?}, got {:.2}",
                    mode,
                    wp,
                    muted_ratio
                );
            }
        }

        // Dynamic Island search placeholder (#CBD5E1) against dark obsidian notch
        let notch_bg = iced::Color::from_rgb(0.05, 0.08, 0.14);
        let placeholder = iced::Color::from_rgb(0.80, 0.84, 0.88);
        assert!(
            wcag_contrast_ratio(placeholder, notch_bg) >= 4.5,
            "Notch Search/Ctrl+K placeholder must exceed WCAG AA"
        );

        // Study Reader bottom exam-tip dark amber on tinted amber container
        let amber_bg = iced::Color::from_rgb(0.996, 0.953, 0.780);
        let amber_header = iced::Color::from_rgb(0.471, 0.208, 0.059);
        let amber_body = iced::Color::from_rgb(0.271, 0.102, 0.012);
        assert!(wcag_contrast_ratio(amber_header, amber_bg) >= 7.0);
        assert!(wcag_contrast_ratio(amber_body, amber_bg) >= 10.0);
    }

    #[test]
    fn test_dual_resolution_scaling_and_60fps_frame_budget() {
        let (mut app, dir) = temp_app("scale_perf");
        app.boot_splash.skip();
        app.profile.onboarding_completed = true;

        // Open heavy windows migrated off desktop grid (Study Reader, 3D Odyssey, Scratchpad)
        let _ = app.update(Message::OpenWindow(WindowId::OdysseyMap));
        let _ = app.update(Message::OpenWindow(WindowId::Scratchpad));
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        assert!(app.core.wm.is_open(WindowId::OdysseyMap));
        assert!(app.core.wm.is_open(WindowId::Scratchpad));
        assert!(app.core.wm.is_open(WindowId::StudyReader));
        assert_eq!(app.dock_springs.len(), 14);

        // Side HUD defaults visible and toggles cleanly
        assert!(app.hud_visible);
        let _ = app.update(Message::ToggleSideHud);
        assert!(!app.hud_visible);
        let _ = app.update(Message::ToggleSideHud);
        assert!(app.hud_visible);

        // Stress 1280x720 and 1920x1080 resizes + measure 60 spring/view cycles (<= 16.6ms avg)
        for (w, h) in [(1280.0_f32, 720.0_f32), (1920.0_f32, 1080.0_f32)] {
            let _ = app.update(Message::EventOccurred(Event::Window(
                iced::window::Event::Resized(Size::new(w, h)),
            )));
            assert_eq!(app.wm_interaction.screen_width, w);
            assert_eq!(app.wm_interaction.screen_height, h);

            let _ = app.update(Message::DockHovered(Some(4)));
            let start = std::time::Instant::now();
            for frame_idx in 0..60u64 {
                let ts = std::time::Instant::now();
                let _ = app.update(Message::AnimationFrame(ts));
                {
                    let _tree = app.view();
                }
                if frame_idx == 29 {
                    let _ = app.update(Message::DockHovered(None));
                }
            }
            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            let avg_frame_ms = elapsed_ms / 60.0;
            assert!(
                avg_frame_ms <= 16.6,
                "{w}x{h} avg view/spring frame time exceeded 16.6ms budget: {avg_frame_ms:.2}ms"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_cli_launch_options_parsing() {
        let args: Vec<String> = vec![
            "--windowed",
            "--size",
            "1920x1080",
            "--wallpaper",
            "bright",
            "--theme",
            "alpine",
            "--smoke-frames",
            "5",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let opts = LaunchOptions::parse_args(&args);
        assert!(!opts.start_maximized);
        assert!(!opts.fullscreen);
        assert_eq!(opts.window_size, Size::new(1920.0, 1080.0));
        assert_eq!(opts.wallpaper_override.as_deref(), Some("bright"));
        assert_eq!(opts.theme_override, Some(ThemeMode::AlpineGlass));
        assert!(opts.skip_boot);
        assert_eq!(opts.smoke_frames, Some(5));
    }

    #[test]
    fn test_zero_chat_error_leakage_missing_file_and_offline_ollama() {
        let (mut app, dir) = temp_app("zero_chat_errors");
        app.core.toast_queue.toasts.clear();
        let initial_len = app.active_messages.len();

        // 1. Missing file read must route ONLY to OsNotificationBus, never into chat
        let _ = app.update(Message::RunQuickOrbCommand(
            "read NonExistent_Physics_Formula_Sheet.txt".to_string(),
        ));
        assert_eq!(
            app.active_messages.len(),
            initial_len + 1,
            "Only user prompt should be added; no error bubble should enter chat"
        );
        assert!(app
            .active_messages
            .iter()
            .all(|m| !m.content.contains("not found")
                && !m.content.contains("Notice:")
                && !m.content.contains("⚠️")));
        assert!(app.core.toast_queue.toasts.iter().any(|t| {
            t.severity == mindset_core::NotificationSeverity::Warning
                && t.action == Some(mindset_core::NotificationAction::OpenVaultFolder)
        }));

        // 2. Simulated Ollama system alert + error stream must never leave Notice/Error bubble in chat
        app.active_messages.push(storage::ChatMessageRecord {
            id: 0,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".into(),
            content: String::new(),
            created_at: String::new(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::SystemAlert {
            severity: "error".into(),
            title: "Maindy Engine Paused".into(),
            body: "Local Ollama unavailable.".into(),
        }));
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Error(
            "Connection refused (127.0.0.1:11434)".into(),
        )));
        assert!(!app.is_generating);
        assert!(app
            .active_messages
            .iter()
            .all(|m| !m.content.contains("Notice:") && !m.content.contains("Connection refused")));
        assert!(app.core.toast_queue.toasts.iter().any(|t| {
            t.severity == mindset_core::NotificationSeverity::Error
                && t.action == Some(mindset_core::NotificationAction::RetryOllama)
        }));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_capsule_creation_and_capsule_theater_presentation() {
        let (mut app, dir) = temp_app("capsule_theater");
        let _ = app.update(Message::RunQuickOrbCommand(
            "Create a 30 min capsule for Gauss's Law".to_string(),
        ));

        assert_eq!(
            app.maindy_screen,
            apps::ai_tutor::MaindyScreen::CapsuleTheater
        );
        assert!(app.core.wm.is_open(WindowId::LayaAiChat));

        let cid = app
            .active_capsule_id
            .clone()
            .expect("active_capsule_id should be populated after capsule creation");
        assert!(app
            .study_capsules
            .iter()
            .any(|c| c.id == cid && c.duration_mins == 30));
        assert!(app.capsules.iter().any(|c| c.id == cid));

        let _ = app.update(Message::SelectCapsulePhase(2));
        assert_eq!(app.active_capsule_phase, 2);
        let _ = app.update(Message::ToggleCapsuleHint);
        assert!(app.capsule_hint_open);

        let _ = app.update(Message::StartCapsuleFocusSprint(cid.clone(), 20));
        assert!(app.core.pomodoro.is_running);
        assert!(app.core.preferences.focus_dnd);

        let cap_json = ai::maindy_system_bridge::capsules_dir().join(format!("{cid}.json"));
        let _ = std::fs::remove_file(cap_json);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_default_system_access_and_destructive_confirmation() {
        let (mut app, dir) = temp_app("system_access");

        // Safe system navigation & controls execute immediately with zero permission gate
        let _ = app.update(Message::RunQuickOrbCommand(
            "open wifi settings".to_string(),
        ));
        assert!(app.core.wm.is_open(WindowId::Settings));
        assert_eq!(
            app.core.preferences.active_tab,
            mindset_core::PreferencesTab::AudioRadios
        );
        assert!(app.pending_destructive_action.is_none());

        let _ = app.update(Message::RunQuickOrbCommand("open files".to_string()));
        assert!(app.core.wm.is_open(WindowId::FileVault));

        let _ = app.update(Message::RunQuickOrbCommand(
            "turn on do not disturb".to_string(),
        ));
        assert!(app.core.preferences.focus_dnd);
        assert!(app.pending_destructive_action.is_none());

        // Destructive action queues inline confirmation instead of running blindly
        let _ = app.update(Message::RunQuickOrbCommand(
            "delete temp_note.txt".to_string(),
        ));
        assert!(matches!(
            app.pending_destructive_action,
            Some(ai::MaindyOsAction::DeleteVaultFile { .. })
        ));
        let _ = app.update(Message::CancelDestructiveOsAction);
        assert!(app.pending_destructive_action.is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_personalization_memory_deduplication() {
        let (mut app, dir) = temp_app("mem_dedup");

        let _ = app.update(Message::OnboardingComplete);
        let _ = app.update(Message::OnboardingComplete);

        let _ = app.update(Message::NewMemoryInputChanged(
            "Revise Ampere's Circuital Law nightly".to_string(),
        ));
        let _ = app.update(Message::AddUserMemory);
        let _ = app.update(Message::NewMemoryInputChanged(
            "Revise Ampere's Circuital Law nightly".to_string(),
        ));
        let _ = app.update(Message::AddUserMemory);

        assert_eq!(
            app.memories
                .iter()
                .filter(|m| m.category == "Core Mission")
                .count(),
            1
        );
        assert_eq!(
            app.memories
                .iter()
                .filter(|m| m.category == "Growth Area")
                .count(),
            1
        );
        assert_eq!(
            app.memories
                .iter()
                .filter(|m| m.category == "Academic Strength")
                .count(),
            1
        );
        assert_eq!(
            app.memories
                .iter()
                .filter(|m| m.memory_text == "Revise Ampere's Circuital Law nightly")
                .count(),
            1
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_dock_clearance_and_maindy_screens_render_dual_resolution() {
        assert_eq!(apps::ai_tutor::DOCK_CLEARANCE, 88.0);
        let (mut app, dir) = temp_app("maindy_views");
        app.boot_splash.skip();
        app.profile.onboarding_completed = true;

        let _ = app.update(Message::OpenWindow(WindowId::LayaAiChat));
        let _ = app.update(Message::RunQuickOrbCommand(
            "Create a 25 min capsule for Wave Optics".to_string(),
        ));
        let cid = app.active_capsule_id.clone();

        for (w, h) in [(1280.0_f32, 720.0_f32), (1920.0_f32, 1080.0_f32)] {
            let _ = app.update(Message::EventOccurred(Event::Window(
                iced::window::Event::Resized(Size::new(w, h)),
            )));

            let _ = app.update(Message::SetMaindyScreen(
                apps::ai_tutor::MaindyScreen::TutorStudio,
            ));
            {
                let _studio_tree = app.view();
            }

            let _ = app.update(Message::SetMaindyScreen(
                apps::ai_tutor::MaindyScreen::CapsuleTheater,
            ));
            {
                let _theater_tree = app.view();
            }
        }

        if let Some(id) = cid {
            let _ = std::fs::remove_file(
                ai::maindy_system_bridge::capsules_dir().join(format!("{id}.json")),
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
