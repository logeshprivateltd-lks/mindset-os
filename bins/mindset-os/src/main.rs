#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! MindSet OS — Native Desktop Visual Computing Shell (Iced 0.14.0)
//!
//! 100% Pure Rust + wgpu presentation layer, zero Electron/WebView overhead.
//! Target: < 60–80 MB Idle RSS. Single-model RAM discipline.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

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
use iced::widget::{column, container, row, stack};
use iced::{event, mouse, window, Alignment, Element, Event, Length, Subscription, Task};
use mindset_core::agent::ollama_grader::FeynmanGrader;
use mindset_core::engine::verification::{
    CapsuleRank, CapsuleState, ProofOfStudyGate, VerificationScoreBreakdown,
};
use mindset_core::{ActiveOverlay, MindSetCoreState, ThemeMode, WindowId, WindowMode};
use theme::{h_space, v_space, ThemePalette};

#[derive(Debug, Clone)]
pub enum Message {
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
    pub dock_springs: [mindset_motion::Spring1D; 10],
    pub hovered_dock_index: Option<usize>,
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
}

impl Default for MindSetApp {
    fn default() -> Self {
        let storage = storage::StorageManager::new();
        let scratchpad_path = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("mindset_os_scratchpad.txt");
        let initial_scratchpad = storage.load_or_migrate_scratchpad(&scratchpad_path);
        let system = system::SystemState::new_with_scratchpad(initial_scratchpad);

        let threads = storage.list_threads();
        let active_thread_id = threads
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "main-session".to_string());
        let active_messages = storage.load_recent_messages(&active_thread_id, 30);
        let wallpaper_handle = if std::path::Path::new("assets/wallpaper.jpg").exists() {
            Some(iced::widget::image::Handle::from_path(
                "assets/wallpaper.jpg",
            ))
        } else if std::path::Path::new("C:/MINDSETOS/assets/wallpaper.jpg").exists() {
            Some(iced::widget::image::Handle::from_path(
                "C:/MINDSETOS/assets/wallpaper.jpg",
            ))
        } else {
            None
        };

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
        let saved_layouts = storage.load_widget_layouts();
        let i18n = i18n::I18n::new(i18n::AppLanguage::from_code(&profile.language_code));
        let ai_activity_log = storage.list_ai_activities(50);
        for (w_id_str, layout) in saved_layouts {
            let id = match w_id_str.as_str() {
                "ScholarBriefing" => Some(mindset_core::DesktopWidgetId::ScholarBriefing),
                "OdysseyMap" | "OdysseyTrail" => Some(mindset_core::DesktopWidgetId::OdysseyTrail),
                "Scratchpad" => Some(mindset_core::DesktopWidgetId::Scratchpad),
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

        Self {
            core,
            system,
            storage,
            sysinfo: system::SysinfoCollector::new(),
            wm_interaction: wm::WindowInteractionState::new(),
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
        }
    }
}

impl MindSetApp {
    pub fn new() -> (Self, Task<Message>) {
        let app = Self::default();
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
                    title: topic.clone(),
                    subject: subject.clone(),
                    description: notes_summary.clone(),
                    status: mindset_core::odyssey::NodeStatus::InProgress,
                    marks: 0,
                    max_marks: 100,
                    xp: 50,
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
                let reply = match listing_res {
                    Ok(files) => {
                        if files.is_empty() {
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
                        }
                    }
                    Err(err) => format!("⚠️ Could not list folder `{}`: {}", path, err),
                };
                self.storage
                    .save_message(&self.active_thread_id, "assistant", &reply);
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);
            }
            mindset_intelligence::agent_tools::AgentAction::ReadDocument { file_path } => {
                let read_res = ai::agent::safe_read_document(&file_path);
                let reply = match read_res {
                    Ok((content, is_truncated)) => {
                        let trunc_note = if is_truncated {
                            "\n*(Content capped at 16 KB)*"
                        } else {
                            ""
                        };
                        format!(
                            "📄 **Document Read Complete:** `{}` ({} bytes read){}\n\n\
                            ```text\n{}\n```\n\n\
                            💡 *I've loaded this document into context and can now answer any questions about it!*",
                            file_path,
                            content.len(),
                            trunc_note,
                            content
                        )
                    }
                    Err(err) => format!("⚠️ Could not read document `{}`: {}", file_path, err),
                };
                self.storage
                    .save_message(&self.active_thread_id, "assistant", &reply);
                self.active_messages = self
                    .storage
                    .load_recent_messages(&self.active_thread_id, 30);
            }
            mindset_intelligence::agent_tools::AgentAction::OpenApp { app_id } => {
                let lower = app_id.to_lowercase();
                match lower.as_str() {
                    "youtube" | "youtube_study" | "yt" => {
                        let res = mindset_net::ChromiumEngineBridge::spawn_youtube_app();
                        let reply = match res {
                            Ok(msg) => format!("📺 Launched **Official YouTube App**: {msg}"),
                            Err(e) => format!("⚠️ Could not launch YouTube App: {e}"),
                        };
                        self.storage.save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self.storage.load_recent_messages(&self.active_thread_id, 30);
                    }
                    "freetube" => {
                        let res = mindset_net::ChromiumEngineBridge::spawn_freetube_app();
                        let reply = match res {
                            Ok(msg) => format!("🛡️ Launched **FreeTube (Ad-Free)**: {msg}"),
                            Err(e) => format!("⚠️ Could not launch FreeTube: {e}"),
                        };
                        self.storage.save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self.storage.load_recent_messages(&self.active_thread_id, 30);
                    }
                    "browser" | "mindset_browser" | "chromium" => {
                        let res = mindset_net::ChromiumEngineBridge::spawn_mindset_browser(None);
                        let reply = match res {
                            Ok(msg) => format!("🌐 Launched **MindSet Browser**: {msg}"),
                            Err(e) => format!("⚠️ Could not launch Browser: {e}"),
                        };
                        self.storage.save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self.storage.load_recent_messages(&self.active_thread_id, 30);
                    }
                    _ => {
                        let win_id = match lower.as_str() {
                            "file_vault" | "filevault" | "vault" | "files" => Some(WindowId::FileVault),
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
                            let reply = format!("⚠️ Unknown app ID `{}` requested.", app_id);
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", &reply);
                            self.active_messages = self
                                .storage
                                .load_recent_messages(&self.active_thread_id, 30);
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
                        let reply = format!("🌐 Maindy AI language preference set to **{}**.", lang);
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
                        let res = mindset_net::ChromiumEngineBridge::spawn_mindset_browser(Some(&url));
                        let reply = match res {
                            Ok(msg) => format!("🌐 Launched MindSet Browser to destination `{url}` ({msg})"),
                            Err(e) => format!("⚠️ Could not launch browser: {e}"),
                        };
                        self.storage
                            .save_message(&self.active_thread_id, "assistant", &reply);
                        self.active_messages = self
                            .storage
                            .load_recent_messages(&self.active_thread_id, 30);
                    }
                }
            }
        }
        Task::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.core.tick(1);
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
            Message::ToggleOverlay(overlay) => {
                self.core.wm.toggle_overlay(overlay);
            }
            Message::CloseOverlays => {
                self.core.wm.close_overlays();
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
            Message::EmbarkNode(id) => {
                self.core.odyssey.embark(&id);
                if let Some(idx) = self.core.odyssey.nodes.iter().position(|n| n.id == id) {
                    self.odyssey_map_state.selected_node = Some(idx);
                }
            }
            Message::CompleteStage(node_id) => {
                self.core.odyssey.complete_stage(&node_id);
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

                    // 2. Insert placeholder assistant message for live streaming
                    self.active_messages.push(storage::ChatMessageRecord {
                        id: 0,
                        thread_id: self.active_thread_id.clone(),
                        role: "user".into(),
                        content: prompt.clone(),
                        created_at: chrono::Local::now().to_rfc3339(),
                    });
                    self.active_messages.push(storage::ChatMessageRecord {
                        id: 0,
                        thread_id: self.active_thread_id.clone(),
                        role: "assistant".into(),
                        content: String::new(),
                        created_at: chrono::Local::now().to_rfc3339(),
                    });

                    self.is_generating = true;
                    let system_prompt = ai::prompt_builder::build_personalized_system_prompt(
                        &self.profile,
                        &self.memories,
                        &self.capsules,
                    );
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
                ai::StreamToken::Done => {
                    self.is_generating = false;
                    self.cancel_flag = None;
                    let mut pending_action_to_run = None;
                    if let Some(last) = self.active_messages.last_mut() {
                        let (action_opt, cleaned) =
                            mindset_intelligence::agent_tools::extract_agent_action(&last.content);
                        let parsed =
                            ai::persona::parse_dual_text(&cleaned, &self.profile.maindy_style);
                        last.content = parsed.display_text.clone();
                        self.storage.save_message(
                            &self.active_thread_id,
                            "assistant",
                            &last.content,
                        );

                        if self.auto_speak_enabled {
                            ai::TtsEngine::speak(parsed.speech_text, parsed.primary_lang);
                            self.island_mode = shell::dynamic_island::IslandMode::MaindySpeaking;
                            let (tw, th) = self.island_mode.target_dimensions(0);
                            self.island_width_spring.set_target(tw);
                            self.island_height_spring.set_target(th);
                        }

                        if let Some(action) = action_opt {
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
                                    ai::agent::is_folder_permitted(
                                        p,
                                        &self.session_allowed_folders,
                                    )
                                } else {
                                    false
                                };

                                if is_permitted {
                                    let (_, target) = mindset_intelligence::agent_tools::action_description(&action);
                                    let action_type = match &action {
                                        mindset_intelligence::agent_tools::AgentAction::ListFolder { .. } => "ListFolder",
                                        mindset_intelligence::agent_tools::AgentAction::ReadDocument { .. } => "ReadDocument",
                                        mindset_intelligence::agent_tools::AgentAction::ControlSystem { .. } => "ControlSystem",
                                        _ => "Other",
                                    };
                                    self.storage.log_ai_activity(action_type, &target, "Approved (Session)", "Authorized by session permission");
                                    self.ai_activity_log = self.storage.list_ai_activities(50);
                                    pending_action_to_run = Some(action);
                                } else {
                                    let (_, target) = mindset_intelligence::agent_tools::action_description(&action);
                                    let action_type = match &action {
                                        mindset_intelligence::agent_tools::AgentAction::ListFolder { .. } => "ListFolder",
                                        mindset_intelligence::agent_tools::AgentAction::ReadDocument { .. } => "ReadDocument",
                                        mindset_intelligence::agent_tools::AgentAction::ControlSystem { .. } => "ControlSystem",
                                        _ => "Other",
                                    };
                                    self.storage.log_ai_activity(action_type, &target, "Pending", "Awaiting user approval");
                                    self.ai_activity_log = self.storage.list_ai_activities(50);
                                    self.pending_agent_action = Some(action);
                                    self.core.wm.toggle_overlay(ActiveOverlay::AgentPermission);
                                }
                            } else {
                                let (_, target) = mindset_intelligence::agent_tools::action_description(&action);
                                let action_type = match &action {
                                    mindset_intelligence::agent_tools::AgentAction::CreateAndShowCapsule { .. } => "CreateAndShowCapsule",
                                    mindset_intelligence::agent_tools::AgentAction::OpenApp { .. } => "OpenApp",
                                    _ => "AutoAction",
                                };
                                self.storage.log_ai_activity(action_type, &target, "Auto", "Auto-executed safe action");
                                self.ai_activity_log = self.storage.list_ai_activities(50);
                                pending_action_to_run = Some(action);
                            }
                        }
                    }
                    if let Some(act) = pending_action_to_run {
                        return self.execute_agent_action(act);
                    }
                }
                ai::StreamToken::Error(err) => {
                    self.is_generating = false;
                    self.cancel_flag = None;
                    if let Some(last) = self.active_messages.last_mut() {
                        last.content = format!("Notice: {}. Ensure Ollama is running.", err);
                    }
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
            Message::ModelUnloaded(res) => {
                if res.is_ok() {
                    self.core.toast_queue.push(
                        "RAM Evicted",
                        "Ollama model evicted from memory (keep_alive: 0).",
                        "🧹",
                    );
                }
            }
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
                let finished = self.boot_splash.update(0.033);
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
                self.onboarding_data = onboarding::OnboardingData::new_with_profile(self.profile.clone());
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

                // Persist 3 tailored memories into SQLite user_memories
                self.storage.add_memory(
                    &format!("Strongest academic subject: {}", self.onboarding_data.strongest_subject),
                    "Academic Strength",
                );
                self.storage.add_memory(
                    &format!("Toughest priority area for reinforcement: {}", self.onboarding_data.toughest_subject),
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
                self.maindy_spring.set_target(0.92);
            }
            Message::SetIslandMode(mode) => {
                self.island_mode = mode;
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
                            self.storage
                                .save_message(&self.active_thread_id, "assistant", denial_msg);
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
                        bridge.launch_shielded_url(&target_url, &mut entertainment, &mut youtube, focus_shield)
                    },
                    Message::BrowserLaunched,
                );
            }
            Message::BrowserLaunched(res) => {
                match res {
                    Ok(msg) => {
                        self.browser_status_note = Some(msg.clone());
                        self.core.toast_queue.push("Browser Launched", msg, "🚀");
                    }
                    Err(err) => {
                        self.browser_status_note = Some(format!("Error: {err}"));
                        self.core.toast_queue.push("Browser Notice", err, "⚠️");
                    }
                }
            }
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
                self.browser_status_note = Some("Spawning real YouTube standalone app...".to_string());
                return Task::perform(
                    async { mindset_net::ChromiumEngineBridge::spawn_youtube_app() },
                    Message::BrowserLaunched,
                );
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
                    if self.browser_url.is_empty() || self.browser_url == "https://ncert.nic.in/textbook.php" {
                        None
                    } else {
                        Some(self.browser_url.clone())
                    }
                });
                self.browser_status_note = Some("Spawning real MindSet Browser (Chromium)...".to_string());
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
                            mindset_sys::network::NetworkManager::connect(&ssid, password.as_deref())
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
            Message::EventOccurred(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
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
                // Ctrl+K -> Toggle Dynamic Island Search
                if modifiers.control() {
                    if let Key::Character(c) = &key {
                        if c.as_str() == "k" || c.as_str() == "K" {
                            if self.island_mode == shell::dynamic_island::IslandMode::Search {
                                self.island_mode = shell::dynamic_island::IslandMode::Compact;
                                self.island_search_query.clear();
                            } else {
                                self.island_mode = shell::dynamic_island::IslandMode::Search;
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
                // Ctrl+Space -> Toggle Launcher
                if modifiers.control() {
                    if let Key::Named(Named::Space) = key {
                        self.core.wm.toggle_overlay(ActiveOverlay::Launcher);
                    }
                }
                // Alt+Tab -> Toggle Window Switcher
                if modifiers.alt() {
                    if let Key::Named(Named::Tab) = key {
                        self.core.wm.toggle_overlay(ActiveOverlay::WindowSwitcher);
                    }
                }
                // Super+D -> Toggle Desktop
                if modifiers.logo() {
                    if let Key::Character(c) = &key {
                        if c.as_str() == "d" || c.as_str() == "D" {
                            self.core.preferences.toggle_desktop();
                        } else if c.as_str() == "l" || c.as_str() == "L" {
                            self.core.wm.toggle_overlay(ActiveOverlay::LockScreen);
                        }
                    }
                }
                // Esc -> Exit Tunnel Vision FIRST, or Close Dynamic Island / Overlays
                if let Key::Named(Named::Escape) = key {
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
                    }
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
            )
        };

        // 3. Floating Application Windows with Exact Safe-Area Positioning & Z-Stack Layering
        let mut workspace_stack = stack![base_content];

        for &id in &self.core.wm.z_stack {
            if id != WindowId::Today {
                if let Some(win) = self.core.wm.get_window(id) {
                    if win.is_open && win.mode != WindowMode::Minimized {
                        let win_view = apps::render_window(
                            win,
                            &self.core,
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
                        );

                        let positioned_win: Element<'_, Message> =
                            if win.mode == WindowMode::Maximized {
                                win_view
                            } else {
                                let rel_y = (win.y - wm::TOP_BAR_SAFE_AREA).max(0.0);
                                let rel_x = win.x.max(0.0);
                                column![
                                    iced::widget::Space::new().height(rel_y),
                                    row![
                                        iced::widget::Space::new().width(rel_x),
                                        win_view,
                                    ],
                                ]
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
                        shell::dock::view(&self.core, &dock_scales, palette, &neu),
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

        // 9. Alpine Wallpaper Background Layer (Cached Handle)
        let mut root_stack = if let Some(handle) = &self.wallpaper_handle {
            let bg_img = iced::widget::image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(iced::ContentFit::Cover);
            stack![bg_img, workspace, island_layer]
        } else {
            stack![workspace, island_layer]
        }
        .width(Length::Fill)
        .height(Length::Fill);

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

        // 30 FPS (~33 ms) Liquidmorphic boot animation timer (only while boot splash is active)
        if self.boot_splash.is_active {
            subs.push(iced::time::every(Duration::from_millis(33)).map(|_| Message::BootSplashTick));
        }

        // 18 FPS (~55 ms) mild ambient motion for 3D Odyssey path (fireflies & breathing ring)
        // Completely paused when Eco Static mode is selected or during Tunnel Vision
        if self.profile.mild_motion_enabled && !self.tunnel_vision_active {
            subs.push(
                iced::time::every(Duration::from_millis(55)).map(|_| Message::MapAmbientTick),
            );
        }

        let springs_moving = !self.tunnel_vision_spring.is_at_rest()
            || !self.maindy_spring.is_at_rest()
            || !self.island_width_spring.is_at_rest()
            || !self.island_height_spring.is_at_rest()
            || self.dock_springs.iter().any(|s| !s.is_at_rest());

        if springs_moving {
            subs.push(window::frames().map(Message::AnimationFrame));
        }

        Subscription::batch(subs)
    }
}

pub fn main() -> iced::Result {
    let _ = env_logger::try_init();

    iced::application(MindSetApp::new, MindSetApp::update, MindSetApp::view)
        .title("MindSet OS — Uninterrupted Deep Focus")
        .window_size(iced::Size::new(1280.0, 800.0))
        .scale_factor(MindSetApp::scale_factor)
        .theme(MindSetApp::theme)
        .subscription(MindSetApp::subscription)
        .font(include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf").as_slice())
        .font(include_bytes!("../../../assets/fonts/NotoSansTamil-Regular.ttf").as_slice())
        .font(include_bytes!("../../../assets/fonts/NotoSansMalayalam-Regular.ttf").as_slice())
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    #[test]
    fn test_open_exam_report_modal() {
        let mut app = MindSetApp::default();
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);

        let _ = app.update(Message::OpenExamReport);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ExamReport);

        let _ = app.update(Message::CloseOverlays);
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
    }

    #[test]
    fn test_open_verification_boss_and_battle_lifecycle() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::OpenVerificationBoss);

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::VerificationBoss);
        assert!(app.core.boss_session.is_some());
        assert!(app.core.active_exam.is_some());

        let initial_hp = app.core.boss_session.as_ref().unwrap().current_hp;
        assert_eq!(initial_hp, 100.0);

        // Select Stage 1 question 0 option 0
        let _ = app.update(Message::SelectBossStage1(0, 0));
        assert_eq!(app.stage1_selections[0], Some(0));
        assert!(app.core.boss_session.as_ref().unwrap().current_hp < initial_hp);

        // Input Feynman explanation
        let explanation = "Nernst equation connects cell potential with reaction quotient because standard potentials shift when ion concentration changes.";
        let _ = app.update(Message::FeynmanInputChanged(explanation.to_string()));
        assert_eq!(app.feynman_input, explanation);
    }

    #[test]
    fn test_excuse_roast_and_bailout_flow() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::TriggerExcuseRoast(
            "I need 5 minutes of Instagram reels".to_string(),
        ));

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::ExcuseRoast);
        assert!(app.core.excuse_roast.is_some());

        // Select correct bailout formula (index 0: V = I * R)
        let _ = app.update(Message::AnswerExcuseFormula(0));
        assert_eq!(app.excuse_selected_answer, Some(0));
    }

    #[test]
    fn test_break_arcade_card_interaction() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::OpenBreakArcade);

        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::BreakArcade);
    }

    #[test]
    fn test_airplane_mode_interlock_in_app() {
        let mut app = MindSetApp::default();
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
    }

    #[test]
    fn test_scratchpad_persistence() {
        let mut app = MindSetApp::default();
        let test_note = "E = mc^2 • Photoelectric effect hf = W + KE";
        let _ = app.update(Message::ScratchpadChanged(test_note.to_string()));
        assert_eq!(app.system.scratchpad_text, test_note);

        // Test clear
        let _ = app.update(Message::ClearScratchpad);
        assert_eq!(app.system.scratchpad_text, "");

        // Test copy status
        let _ = app.update(Message::CopyScratchpad);
        assert_eq!(app.system.scratchpad_save_status, "Copied to clipboard!");
    }

    #[test]
    fn test_odyssey_embark_and_selection() {
        let mut app = MindSetApp::default();
        assert_eq!(app.core.odyssey.current_stage_index, 0);
        let _ = app.update(Message::EmbarkNode("phy-101".to_string()));
        assert_eq!(app.odyssey_map_state.selected_node, Some(0));
    }

    #[test]
    fn test_daily_capsule_generation() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::GenerateDailyCapsules);
        assert!(!app.core.toast_queue.toasts.is_empty());
    }

    #[test]
    fn test_preferences_tab_navigation() {
        let mut app = MindSetApp::default();
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
    }

    #[test]
    fn test_control_center_live_bindings() {
        let mut app = MindSetApp::default();

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
    }

    #[test]
    fn test_file_vault_actions() {
        let mut app = MindSetApp::default();

        let _ = app.update(Message::VaultSelectSidebar("Documents".to_string()));
        let _ = app.update(Message::VaultSearchChanged("Gauss".to_string()));
        let _ = app.update(Message::VaultEntryClicked("README_STUDY_VAULT.txt".to_string(), false));
        let _ = app.update(Message::VaultNavigateUp);
    }

    #[test]
    fn test_window_drag_lifecycle() {
        let mut app = MindSetApp::default();
        let id = WindowId::StudyReader;
        let _ = app.update(Message::OpenWindow(id));
        assert!(app.core.wm.is_open(id));

        let initial_win = app.core.wm.get_window(id).unwrap().clone();

        // 1. Position cursor over window titlebar
        app.cursor_position = Point::new(initial_win.x + 50.0, initial_win.y + 15.0);

        // 2. Start drag on header
        let _ = app.update(Message::StartWindowDrag(id));
        assert!(app.wm_interaction.dragging.is_some());
        let (drag_id, off_x, off_y) = app.wm_interaction.dragging.unwrap();
        assert_eq!(drag_id, id);
        assert!((off_x - 50.0).abs() < f32::EPSILON);
        assert!((off_y - 15.0).abs() < f32::EPSILON);

        // 3. Move cursor to (200.0, 80.0) -> expected y: 80.0 - 15.0 = 65.0 (within safe area [44.0, 108.0])
        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(200.0, 80.0),
            },
        )));

        let moved_win = app.core.wm.get_window(id).unwrap();
        assert_eq!(moved_win.x, 200.0 - 50.0);
        assert_eq!(moved_win.y, 80.0 - 15.0);

        // 4. Move cursor out of bounds to test clamping at bottom
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

        // 5. Release mouse button to end dragging
        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::ButtonReleased(mouse::Button::Left),
        )));
        assert!(app.wm_interaction.dragging.is_none());
    }

    #[test]
    fn test_focus_window_and_bring_to_front() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        let _ = app.update(Message::OpenWindow(WindowId::Tasks));
        let _ = app.update(Message::OpenWindow(WindowId::FileVault));

        assert_eq!(app.core.wm.active_window(), Some(WindowId::FileVault));

        // Click StudyReader to focus
        let _ = app.update(Message::FocusWindow(WindowId::StudyReader));
        assert_eq!(app.core.wm.active_window(), Some(WindowId::StudyReader));
        assert_eq!(app.core.wm.z_stack.last(), Some(&WindowId::StudyReader));
    }

    #[test]
    fn test_desktop_widget_drag_and_repositioning() {
        let mut app = MindSetApp::default();
        app.core.wm.widget_layout.reset();
        let widget_id = mindset_core::DesktopWidgetId::ScholarBriefing;

        let initial_widget = app
            .core
            .wm
            .get_widget_placement(widget_id)
            .unwrap()
            .clone();

        app.cursor_position = Point::new(initial_widget.x + 20.0, initial_widget.y + 10.0);

        // 1. Start widget drag
        let _ = app.update(Message::StartWidgetDrag(widget_id));
        assert!(app.wm_interaction.dragging_widget.is_some());
        assert!(app.core.wm.widget_layout.free_placement);

        // 2. Move cursor
        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(500.0, 150.0),
            },
        )));

        let moved_widget = app.core.wm.get_widget_placement(widget_id).unwrap();
        assert_eq!(moved_widget.x, 500.0 - 20.0);
        assert_eq!(moved_widget.y, 150.0 - 10.0);

        // 3. Release mouse button
        let _ = app.update(Message::EventOccurred(Event::Mouse(
            mouse::Event::ButtonReleased(mouse::Button::Left),
        )));
        assert!(app.wm_interaction.dragging_widget.is_none());

        // 4. Reset layout restores defaults
        let _ = app.update(Message::ResetWidgetLayout);
        let reset_widget = app.core.wm.get_widget_placement(widget_id).unwrap();
        assert_eq!(reset_widget.x, initial_widget.x);
        assert_eq!(reset_widget.y, initial_widget.y);
    }

    #[test]
    fn test_tunnel_vision_disengage_on_close_study_reader() {
        let mut app = MindSetApp::default();
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        assert!(app.tunnel_vision_active);
        assert_eq!(app.tunnel_vision_spring.target, 0.0);

        // Closing StudyReader must always disengage tunnel vision
        let _ = app.update(Message::CloseWindow(WindowId::StudyReader));
        assert!(!app.tunnel_vision_active);
        assert_eq!(app.tunnel_vision_spring.target, 1.0);
        assert_eq!(app.island_mode, shell::dynamic_island::IslandMode::Compact);
    }

    #[test]
    fn test_tunnel_vision_disengage_on_escape() {
        use iced::keyboard::key::Named;
        use iced::keyboard::{Event as KbEvent, Key, Modifiers};

        let mut app = MindSetApp::default();
        let _ = app.update(Message::OpenWindow(WindowId::StudyReader));
        assert!(app.tunnel_vision_active);

        // Pressing Escape must disengage tunnel vision and restore compact notch
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
    }

    #[test]
    fn test_boot_splash_lifecycle() {
        let mut splash = shell::boot_splash::BootSplashState::new();
        assert!(splash.is_active);
        assert!(!splash.is_finished);

        // Update partially
        let finished = splash.update(1.0);
        assert!(!finished);
        assert_eq!(splash.elapsed, 1.0);

        // Advance to full duration
        let finished2 = splash.update(2.0);
        assert!(finished2);
        assert!(splash.is_finished);
        assert!(!splash.is_active);

        // Test skip
        let mut splash2 = shell::boot_splash::BootSplashState::new();
        splash2.skip();
        assert!(splash2.is_finished);
        assert!(!splash2.is_active);
    }

    #[test]
    fn test_onboarding_three_step_flow_and_commit() {
        let mut data = onboarding::OnboardingData::default();
        assert_eq!(data.current_step, onboarding::OnboardingStep::Step1WhoAreYou);

        // Step 1: Who Are You?
        data.name_input = "Arjun".to_string();
        data.class_degree_input = "Class 12".to_string();
        data.selected_language = "thanglish".to_string();
        data.next();
        assert_eq!(data.current_step, onboarding::OnboardingStep::Step2YourMission);

        // Step 2: Your Mission
        data.apply_track("Class12BoardsTN600");
        assert_eq!(data.target_score_max, 600);
        assert_eq!(data.target_score_value, 590);
        data.exam_date_input = "2027-03-01".to_string();
        data.daily_study_hours = 4;
        data.next();
        assert_eq!(data.current_step, onboarding::OnboardingStep::Step3LearningStyle);

        // Step 3: Learning Style & Weak Spots
        data.strongest_subject = "Physics".to_string();
        data.toughest_subject = "Electrochemistry Nernst Eq".to_string();
        data.coaching_tone = "Friendly Thanglish Bro".to_string();
        data.next();
        assert!(data.profile.onboarding_completed);
        assert_eq!(data.profile.name, "Arjun");
        assert_eq!(data.profile.career_track, "Class12BoardsTN600");
        assert_eq!(data.profile.target_score_value, 590);
        assert_eq!(data.profile.coaching_style, "Friendly Thanglish Bro");
        assert_eq!(data.profile.maindy_style, "thanglish");
        assert!(data.profile.weak_topics.contains(&"Electrochemistry Nernst Eq".to_string()));
    }

    #[test]
    fn test_personalized_prompt_builder() {
        let mut profile = storage::db::UserProfile::default();
        profile.name = "Mapla".to_string();
        profile.career_track = "Class12BoardsTN600".to_string();
        profile.target_score_value = 590;
        profile.target_score_max = 600;
        profile.maindy_style = "thanglish".to_string();
        profile.coaching_style = "Friendly Thanglish Bro".to_string();
        profile.weak_topics = vec![
            "Electrochemistry Nernst Eq".to_string(),
        ];

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

        let prompt = ai::prompt_builder::build_personalized_system_prompt(
            &profile,
            &memories,
            &capsules,
        );

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
        let mut app = MindSetApp::default();
        app.profile.onboarding_completed = true;

        // 1. Unsolicited ReadDocument action triggers Permission Interlock
        let action_raw = r#"I'm preparing to read your notes.
<os_action>
{
  "action": "ReadDocument",
  "params": {
    "file_path": "Formula Sheets/Physics_Formula_Sheet.txt"
  }
}
</os_action>"#;

        // Push assistant message and trigger Done
        app.active_messages.push(storage::ChatMessageRecord {
            id: 999,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:00:00".to_string(),
        });

        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));

        // Must pop up permission modal
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::AgentPermission);
        assert!(app.pending_agent_action.is_some());

        // 2. Denying the action closes modal and cancels action
        let _ = app.update(Message::AgentPermissionResponse(ai::agent::AgentPermissionDecision::Deny));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert!(app.pending_agent_action.is_none());
        assert!(app.active_messages.last().unwrap().content.contains("Permission was denied"));

        // 3. User approves folder for session
        app.active_messages.push(storage::ChatMessageRecord {
            id: 1000,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:01:00".to_string(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::AgentPermission);

        let _ = app.update(Message::AgentPermissionResponse(ai::agent::AgentPermissionDecision::AllowSession));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        assert!(app.session_allowed_folders.contains("Formula Sheets"));

        // Subsequent ReadDocument in same folder does NOT prompt modal again
        app.active_messages.push(storage::ChatMessageRecord {
            id: 1001,
            thread_id: app.active_thread_id.clone(),
            role: "assistant".to_string(),
            content: action_raw.to_string(),
            created_at: "2026-11-21 12:02:00".to_string(),
        });
        let _ = app.update(Message::AiStreamToken(ai::StreamToken::Done));
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);

        // 4. Revoking permissions clears session authorizations
        let _ = app.update(Message::RevokeAllAgentPermissions);
        assert!(app.session_allowed_folders.is_empty());

        // 5. CreateAndShowCapsule executes immediately without permission modal
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

        // No modal shown
        assert_eq!(app.core.wm.active_overlay, ActiveOverlay::None);
        // Capsule added
        assert_eq!(app.capsules.len(), initial_capsule_count + 1);
        assert_eq!(app.core.odyssey.nodes.len(), initial_odyssey_count + 1);
        // Study Reader opened
        assert!(app.core.wm.is_open(WindowId::StudyReader));
    }

    #[test]
    fn test_browser_and_youtube_study_navigation() {
        let mut app = MindSetApp::default();

        // 1. Open YouTube Study sets URL and opens SearchBrowser
        let _ = app.update(Message::OpenYouTubeStudy);
        assert_eq!(app.browser_url, "https://www.youtube.com");
        assert!(app.core.wm.is_open(WindowId::SearchBrowser));
        assert_eq!(app.browser_history.len(), 1);

        // 2. BrowserNavigate with educational URL
        let _ = app.update(Message::BrowserNavigate("https://ncert.nic.in/textbook.php".to_string()));
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history.len(), 2);
        assert_eq!(app.browser_history_idx, 1);

        // 3. BrowserNavigate with plain search terms normalizes to YouTube search query
        let _ = app.update(Message::BrowserNavigate("class 12 physics derivations".to_string()));
        assert_eq!(
            app.browser_url,
            "https://www.youtube.com/results?search_query=class+12+physics+derivations"
        );
        assert_eq!(app.browser_history.len(), 3);
        assert_eq!(app.browser_history_idx, 2);

        // 4. BrowserBack steps backwards through history
        let _ = app.update(Message::BrowserBack);
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history_idx, 1);

        let _ = app.update(Message::BrowserBack);
        assert_eq!(app.browser_url, "https://www.youtube.com");
        assert_eq!(app.browser_history_idx, 0);

        // 5. BrowserForward steps forward through history
        let _ = app.update(Message::BrowserForward);
        assert_eq!(app.browser_url, "https://ncert.nic.in/textbook.php");
        assert_eq!(app.browser_history_idx, 1);

        // 6. Focus Shield toggle
        assert!(app.browser_focus_shield);
        let _ = app.update(Message::ToggleBrowserFocusShield);
        assert!(!app.browser_focus_shield);
        let _ = app.update(Message::ToggleBrowserFocusShield);
        assert!(app.browser_focus_shield);
    }

    #[test]
    fn test_network_and_wifi_manager_state() {
        let mut app = MindSetApp::default();

        // Initial Wi-Fi menu is closed
        assert!(!app.wifi_menu_open);
        assert!(app.wifi_connecting_ssid.is_none());

        // Toggle Wi-Fi menu opens it
        let _ = app.update(Message::ToggleWifiMenu);
        assert!(app.wifi_menu_open);

        // Selecting a Wi-Fi network prepares connection
        let _ = app.update(Message::SelectWifiNetwork("Campus-5G".to_string()));
        assert_eq!(app.wifi_connecting_ssid.as_deref(), Some("Campus-5G"));

        // Entering password
        let _ = app.update(Message::WifiPasswordInputChanged("StudyFocus2026".to_string()));
        assert_eq!(app.wifi_password_input, "StudyFocus2026");

        // Cancelling connection resets target
        let _ = app.update(Message::CancelWifiConnect);
        assert!(app.wifi_connecting_ssid.is_none());
        assert!(app.wifi_password_input.is_empty());

        // Network status update reflection
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
    }
}
