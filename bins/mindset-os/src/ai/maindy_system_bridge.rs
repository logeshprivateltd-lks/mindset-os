//! Maindy System Bridge — Default OS Control & Capsule Persistence Engine
//!
//! Gives Maindy AI built-in, zero-friction OS execution capabilities tailored for
//! Debian 13 ("Trixie") Linux (`~/.local/share/mindset-os/`) while keeping cross-platform
//! fallbacks clean:
//! - Tier 1: Deterministic Rust Fast-Path Parser (<5ms latency, resilient even without 2B JSON cooperation)
//! - Tier 2: Structured `<os_action>{...}</os_action>` extraction supporting both `MaindyOsAction` & `AgentAction`
//! - Safe Default Execution: read/open/capsule/focus actions execute automatically; destructive actions
//!   require a single inline confirmation card.
//! - Capsule Persistence: writes rich JSON `StudyCapsule` files to `~/.local/share/mindset-os/capsules/<id>.json`.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::storage::db::CapsuleRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsSection {
    General,
    Wifi,
    Bluetooth,
    Display,
    FocusGuard,
    Audio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemControl {
    Wifi,
    Bluetooth,
    DoNotDisturb,
    AirplaneMode,
    DarkTheme,
    AudioMute,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaindyOsAction {
    OpenSettings {
        section: Option<SettingsSection>,
    },
    OpenFiles {
        path: Option<PathBuf>,
    },
    LaunchApp {
        app_id: String,
    },
    ToggleSystemControl {
        control: SystemControl,
        enabled: bool,
    },
    CreateAndPresentCapsule {
        topic: String,
        subject: Option<String>,
        duration_mins: u16,
    },
    StartFocusSprint {
        capsule_id: String,
        minutes: u16,
    },
    ReadVaultFile {
        file_name: String,
    },
    DeleteVaultFile {
        path: PathBuf,
    },
    PowerOffConfirm,
}

impl MaindyOsAction {
    /// Read/open/navigation/capsule actions run automatically by default.
    /// Only destructive OS actions require explicit inline confirmation.
    pub fn is_destructive(&self) -> bool {
        matches!(self, Self::DeleteVaultFile { .. } | Self::PowerOffConfirm)
    }

    pub fn human_label(&self) -> String {
        match self {
            Self::OpenSettings { .. } => "Open MindSet Settings".to_string(),
            Self::OpenFiles { .. } => "Open Study Vault Files".to_string(),
            Self::LaunchApp { app_id } => format!("Launch {app_id}"),
            Self::ToggleSystemControl { control, enabled } => {
                format!(
                    "{} {:?}",
                    if *enabled { "Enable" } else { "Disable" },
                    control
                )
            }
            Self::CreateAndPresentCapsule { topic, .. } => {
                format!("Create Capsule: {topic}")
            }
            Self::StartFocusSprint { minutes, .. } => format!("Start {minutes}m Focus Sprint"),
            Self::ReadVaultFile { file_name } => format!("Read Note: {file_name}"),
            Self::DeleteVaultFile { path } => format!("Delete File: {}", path.display()),
            Self::PowerOffConfirm => "Power Off MindSet OS".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsulePhase {
    pub index: usize,
    pub name: String,
    pub duration_mins: u16,
    pub summary: String,
    pub hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StudyCapsule {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub target_marks_impact: String,
    pub duration_mins: u16,
    pub phases: Vec<CapsulePhase>,
    pub core_formulas: Vec<String>,
    pub step_by_step_notes: Vec<String>,
    pub created_at: String,
}

impl StudyCapsule {
    pub fn to_db_record(&self) -> CapsuleRecord {
        CapsuleRecord {
            id: self.id.clone(),
            target_date: Local::now().format("%Y-%m-%d").to_string(),
            subject: self.subject.clone(),
            unit: self.title.clone(),
            estimated_minutes: self.duration_mins as u32,
            xp_reward: 60,
            status: "In Progress".to_string(),
            stage: self
                .phases
                .first()
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "1. Concept".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HybridRouteResult {
    pub action: Option<MaindyOsAction>,
    pub clean_reply: Option<String>,
}

/// Canonical Debian 13 XDG data directory (`~/.local/share/mindset-os/`) without hardcoded user paths.
pub fn linux_data_root() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        let trimmed = xdg.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed).join("mindset-os");
        }
    }
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    #[cfg(target_os = "linux")]
    {
        home.join(".local").join("share").join("mindset-os")
    }
    #[cfg(not(target_os = "linux"))]
    {
        // Keep clean XDG-parity structure under local data / home without any hardcoded drive usernames
        if let Some(local) = dirs::data_local_dir() {
            local.join("mindset-os")
        } else {
            home.join(".local").join("share").join("mindset-os")
        }
    }
}

pub fn capsules_dir() -> PathBuf {
    linux_data_root().join("capsules")
}

pub fn vault_dir() -> PathBuf {
    linux_data_root().join("vault")
}

pub struct MaindySystemBridge;

impl MaindySystemBridge {
    /// Ensures `~/.local/share/mindset-os/{capsules,vault}` exists cleanly.
    pub fn ensure_linux_directories() -> PathBuf {
        let caps = capsules_dir();
        let vault = vault_dir();
        let _ = fs::create_dir_all(&caps);
        let _ = fs::create_dir_all(&vault);
        let _ = fs::create_dir_all(vault.join("Physics"));
        let _ = fs::create_dir_all(vault.join("Chemistry"));
        let _ = fs::create_dir_all(vault.join("Formula Sheets"));
        let _ = fs::create_dir_all(vault.join("Notes"));
        caps
    }

    /// Resolves a file inside `~/.local/share/mindset-os/vault/` (without falling back to errors in chat).
    pub fn resolve_vault_file(file_name: &str) -> Option<PathBuf> {
        Self::ensure_linux_directories();
        let clean = file_name.trim().trim_matches(|c| c == '"' || c == '\'');
        if clean.is_empty() || clean.contains("..") {
            return None;
        }
        let vault = vault_dir();
        let candidates = [
            vault.join(clean),
            vault.join("Formula Sheets").join(clean),
            vault.join("Notes").join(clean),
            vault.join("Physics").join(clean),
            vault.join("Chemistry").join(clean),
        ];
        candidates.into_iter().find(|p| p.is_file())
    }

    /// Synthesizes a structured 4-phase `StudyCapsule` tailored to the topic & target score.
    pub fn build_capsule(
        topic: &str,
        subject_opt: Option<&str>,
        duration_mins: u16,
    ) -> StudyCapsule {
        let lower = topic.to_lowercase();
        let inferred_subject = subject_opt.map(|s| s.to_string()).unwrap_or_else(|| {
            if lower.contains("nernst")
                || lower.contains("electrochem")
                || lower.contains("equilibrium")
                || lower.contains("organic")
                || lower.contains("chem")
            {
                "Chemistry".to_string()
            } else if lower.contains("matrix")
                || lower.contains("calculus")
                || lower.contains("integral")
                || lower.contains("differential")
                || lower.contains("probability")
            {
                "Mathematics".to_string()
            } else if lower.contains("dna")
                || lower.contains("genetics")
                || lower.contains("cell")
                || lower.contains("biology")
            {
                "Biology".to_string()
            } else {
                "Physics".to_string()
            }
        });

        let dur = duration_mins.clamp(10, 120);
        let p1 = ((dur as f32) * 0.35).round().max(3.0) as u16;
        let p2 = ((dur as f32) * 0.25).round().max(3.0) as u16;
        let p3 = ((dur as f32) * 0.20).round().max(2.0) as u16;
        let p4 = dur.saturating_sub(p1 + p2 + p3).max(2);

        let (core_formulas, step_by_step_notes) =
            Self::knowledge_pack_for_topic(topic, &inferred_subject);
        let id = format!("cap-{}", &Uuid::new_v4().simple().to_string()[..10]);
        let clean_title = Self::title_case_topic(topic);

        StudyCapsule {
            id,
            title: clean_title.clone(),
            subject: inferred_subject,
            target_marks_impact: "+5 Board / High-Yield Concept".to_string(),
            duration_mins: dur,
            phases: vec![
                CapsulePhase {
                    index: 0,
                    name: "Heavy Lift".to_string(),
                    duration_mins: p1,
                    summary: format!(
                        "Understand physical intuition, boundary assumptions, and first-principles proof of {}.",
                        clean_title
                    ),
                    hint: "Start from fundamental conservation or symmetry law before substituting values.".to_string(),
                },
                CapsulePhase {
                    index: 1,
                    name: "Numericals".to_string(),
                    duration_mins: p2,
                    summary: format!(
                        "Solve 3 graded board & JEE numerical problems on {} checking SI units at every line.",
                        clean_title
                    ),
                    hint: "Convert centimeters/milliliters to strict SI base units before plugging into formulas.".to_string(),
                },
                CapsulePhase {
                    index: 2,
                    name: "Memory Anchor".to_string(),
                    duration_mins: p3,
                    summary: format!(
                        "Lock in constants, edge cases, and graphical slope behavior for {}.",
                        clean_title
                    ),
                    hint: "Recall the formula blindfolded, then state one common examiner trap.".to_string(),
                },
                CapsulePhase {
                    index: 3,
                    name: "Boss Battle".to_string(),
                    duration_mins: p4,
                    summary: format!(
                        "Verify {} mastery in timed 590/600 rigor Proof-of-Study gate.",
                        clean_title
                    ),
                    hint: "Explain the causal link aloud without looking at notes.".to_string(),
                },
            ],
            core_formulas,
            step_by_step_notes,
            created_at: Local::now().to_rfc3339(),
        }
    }

    /// Saves structured capsule JSON into `<capsules_root>/<id>.json`.
    pub fn save_capsule_to_dir(capsule: &StudyCapsule, dir: &Path) -> Result<PathBuf, String> {
        fs::create_dir_all(dir).map_err(|e| format!("Failed creating capsule dir: {e}"))?;
        let file_path = dir.join(format!("{}.json", capsule.id));
        let json = serde_json::to_string_pretty(capsule)
            .map_err(|e| format!("Capsule serialization failed: {e}"))?;
        fs::write(&file_path, json).map_err(|e| format!("Failed writing capsule JSON: {e}"))?;
        Ok(file_path)
    }

    pub fn save_capsule(capsule: &StudyCapsule) -> Result<PathBuf, String> {
        let dir = Self::ensure_linux_directories();
        Self::save_capsule_to_dir(capsule, &dir)
    }

    /// Loads saved `StudyCapsule` JSON files from `~/.local/share/mindset-os/capsules/`.
    pub fn load_capsules_from_dir(dir: &Path) -> Vec<StudyCapsule> {
        let mut out = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(raw) = fs::read_to_string(&p) {
                        if let Ok(cap) = serde_json::from_str::<StudyCapsule>(&raw) {
                            out.push(cap);
                        }
                    }
                }
            }
        }
        out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        out
    }

    pub fn load_saved_capsules() -> Vec<StudyCapsule> {
        let dir = Self::ensure_linux_directories();
        Self::load_capsules_from_dir(&dir)
    }

    /// Executes OS radio / D-Bus hooks on Debian 13 (non-blocking) for network/audio controls.
    pub fn apply_debian_dbus_control(control: SystemControl, enabled: bool) {
        #[cfg(target_os = "linux")]
        {
            match control {
                SystemControl::Wifi => {
                    let arg = if enabled { "on" } else { "off" };
                    let _ = std::process::Command::new("nmcli")
                        .args(["radio", "wifi", arg])
                        .spawn();
                }
                SystemControl::Bluetooth => {
                    let arg = if enabled { "power on" } else { "power off" };
                    let _ = std::process::Command::new("bluetoothctl")
                        .args(arg.split_whitespace())
                        .spawn();
                }
                SystemControl::AirplaneMode => {
                    let arg = if enabled { "off" } else { "on" };
                    let _ = std::process::Command::new("nmcli")
                        .args(["radio", "all", arg])
                        .spawn();
                }
                SystemControl::AudioMute => {
                    let arg = if enabled { "1" } else { "0" };
                    let _ = std::process::Command::new("wpctl")
                        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", arg])
                        .spawn();
                }
                SystemControl::DoNotDisturb | SystemControl::DarkTheme => {
                    let _ = std::process::Command::new("dbus-send")
                        .args([
                            "--session",
                            "--type=signal",
                            "/org/mindsetos/Shell",
                            "org.mindsetos.Shell.ControlToggled",
                        ])
                        .spawn();
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (control, enabled);
        }
    }

    /// Tier 1 Deterministic Fast-Path Parser (<5ms latency).
    /// Reliably resolves clear OS control and capsule requests before waiting on 2B models.
    pub fn route_fast_path(prompt: &str) -> Option<HybridRouteResult> {
        let trimmed = prompt.trim();
        let lower = trimmed.to_lowercase();
        if lower.is_empty() {
            return None;
        }

        // 1. Destructive actions always gated via single inline confirmation
        if lower.contains("delete ")
            && (lower.contains("file") || lower.contains(".txt") || lower.contains(".pdf"))
        {
            let fname = Self::extract_filename_token(trimmed).unwrap_or("selected_note.txt");
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::DeleteVaultFile {
                    path: vault_dir().join(fname),
                }),
                clean_reply: Some(format!(
                    "Please confirm below if you want to remove `{fname}` from your Study Vault."
                )),
            });
        }
        if lower == "power off" || lower == "shut down" || lower == "shutdown system" {
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::PowerOffConfirm),
                clean_reply: Some(
                    "Confirm below when you are ready to power off MindSet OS.".into(),
                ),
            });
        }

        // 2. Capsule creation command ("create a capsule for Gauss's Law", "make a 30 min capsule for Wave Optics")
        let wants_capsule = (lower.contains("create")
            || lower.contains("make")
            || lower.contains("generate")
            || lower.contains("build"))
            && lower.contains("capsule");
        if wants_capsule {
            let duration = Self::extract_duration_minutes(&lower).unwrap_or(25);
            let topic = Self::extract_capsule_topic(trimmed);
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::CreateAndPresentCapsule {
                    topic: topic.clone(),
                    subject: None,
                    duration_mins: duration,
                }),
                clean_reply: Some(format!(
                    "Synthesized a {duration}-minute Study Capsule for **{topic}** and launched Capsule Theater."
                )),
            });
        }

        // 3. Explicit reading of a missing or named vault file
        if (lower.starts_with("read ") || lower.starts_with("open "))
            && (lower.contains(".txt")
                || lower.contains(".pdf")
                || lower.contains(".md")
                || lower.contains(".capsule"))
        {
            if let Some(fname) = Self::extract_filename_token(trimmed) {
                return Some(HybridRouteResult {
                    action: Some(MaindyOsAction::ReadVaultFile {
                        file_name: fname.to_string(),
                    }),
                    clean_reply: None,
                });
            }
        }

        // 4. Settings navigation ("open settings", "open wi-fi settings", "open audio settings")
        if (lower.contains("open") || lower.contains("show") || lower.contains("go to"))
            && lower.contains("setting")
        {
            let section = if lower.contains("wi-fi") || lower.contains("wifi") {
                Some(SettingsSection::Wifi)
            } else if lower.contains("bluetooth") {
                Some(SettingsSection::Bluetooth)
            } else if lower.contains("display") || lower.contains("theme") {
                Some(SettingsSection::Display)
            } else if lower.contains("audio") || lower.contains("sound") || lower.contains("voice")
            {
                Some(SettingsSection::Audio)
            } else if lower.contains("focus") || lower.contains("shield") {
                Some(SettingsSection::FocusGuard)
            } else {
                Some(SettingsSection::General)
            };
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::OpenSettings { section }),
                clean_reply: Some(
                    "Opened MindSet OS Settings so you can adjust preferences right away.".into(),
                ),
            });
        }
        if lower.contains("open wifi") || lower.contains("open wi-fi") {
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::OpenSettings {
                    section: Some(SettingsSection::Wifi),
                }),
                clean_reply: Some("Opened Wi-Fi & Network controls.".into()),
            });
        }

        // 5. File Vault / Study Notes ("open files", "open my physics notes", "open file manager")
        if (lower.contains("open") || lower.contains("show") || lower.contains("list"))
            && (lower.contains("file")
                || lower.contains("vault")
                || lower.contains("folder")
                || lower.contains("physics notes")
                || lower.contains("my notes"))
        {
            let sub = if lower.contains("physics") {
                Some(vault_dir().join("Physics"))
            } else if lower.contains("chem") {
                Some(vault_dir().join("Chemistry"))
            } else if lower.contains("formula") {
                Some(vault_dir().join("Formula Sheets"))
            } else {
                Some(vault_dir())
            };
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::OpenFiles { path: sub }),
                clean_reply: Some("Opened your Linux Study Vault in the File Manager.".into()),
            });
        }

        // 6. System control toggles ("turn on do not disturb", "turn off wifi", etc.)
        if lower.contains("do not disturb") || lower.contains(" dnd") || lower.ends_with("dnd") {
            let enabled = !lower.contains("off") && !lower.contains("disable");
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::ToggleSystemControl {
                    control: SystemControl::DoNotDisturb,
                    enabled,
                }),
                clean_reply: Some(if enabled {
                    "Do Not Disturb enabled — distractions muted so you can stay locked in.".into()
                } else {
                    "Do Not Disturb disabled — notifications resumed.".into()
                }),
            });
        }
        if (lower.contains("turn on")
            || lower.contains("turn off")
            || lower.contains("enable")
            || lower.contains("disable"))
            && (lower.contains("wifi") || lower.contains("wi-fi") || lower.contains("bluetooth"))
        {
            let enabled = lower.contains("turn on") || lower.contains("enable");
            let control = if lower.contains("bluetooth") {
                SystemControl::Bluetooth
            } else {
                SystemControl::Wifi
            };
            return Some(HybridRouteResult {
                action: Some(MaindyOsAction::ToggleSystemControl { control, enabled }),
                clean_reply: Some(format!(
                    "Updated {:?} radio ({}).",
                    control,
                    if enabled { "Enabled" } else { "Disabled" }
                )),
            });
        }

        // 7. Native & `.ms` App Launcher ("open youtube", "open scratchpad", "open odyssey", "open google")
        if lower.contains("open ") || lower.contains("launch ") {
            let app_id = if lower.contains("youtube") {
                Some("youtube")
            } else if lower.contains("google") || lower.contains("browser") {
                Some("google")
            } else if lower.contains("odyssey") {
                Some("odyssey_map")
            } else if lower.contains("scratchpad") {
                Some("scratchpad")
            } else if lower.contains("reader") {
                Some("study_reader")
            } else if lower.contains("app center") || lower.contains("appstore") {
                Some("app_center")
            } else {
                None
            };
            if let Some(id) = app_id {
                return Some(HybridRouteResult {
                    action: Some(MaindyOsAction::LaunchApp {
                        app_id: id.to_string(),
                    }),
                    clean_reply: Some(format!("Launched **{id}** for your study session.")),
                });
            }
        }

        None
    }

    /// Tier 2 LLM Structured Action Extractor: cleanly strips `<os_action>...</os_action>`
    /// from chat output even if the model returned partial/hybrid JSON.
    pub fn extract_llm_os_action(raw: &str) -> (Option<MaindyOsAction>, String) {
        let start_tag = "<os_action>";
        let end_tag = "</os_action>";

        if let Some(start_idx) = raw.find(start_tag) {
            let after_start = &raw[start_idx + start_tag.len()..];
            if let Some(end_idx) = after_start.find(end_tag) {
                let json_str = after_start[..end_idx].trim();
                let before = &raw[..start_idx];
                let after = &after_start[end_idx + end_tag.len()..];
                let cleaned = format!("{before}{after}").trim().to_string();

                if let Ok(bridge_act) = serde_json::from_str::<MaindyOsAction>(json_str) {
                    return (Some(bridge_act), cleaned);
                }

                // Fallback conversion from AgentAction JSON schema
                let (_, _) = mindset_intelligence::agent_tools::extract_agent_action(raw);
                if let Ok(agent_act) =
                    serde_json::from_str::<mindset_intelligence::agent_tools::AgentAction>(json_str)
                {
                    let mapped = match agent_act {
                        mindset_intelligence::agent_tools::AgentAction::CreateAndShowCapsule {
                            subject,
                            topic,
                            duration_mins,
                            ..
                        } => MaindyOsAction::CreateAndPresentCapsule {
                            topic,
                            subject: Some(subject),
                            duration_mins: duration_mins.min(u16::MAX as u32) as u16,
                        },
                        mindset_intelligence::agent_tools::AgentAction::ListFolder { path } => {
                            MaindyOsAction::OpenFiles {
                                path: Some(PathBuf::from(path)),
                            }
                        }
                        mindset_intelligence::agent_tools::AgentAction::ReadDocument {
                            file_path,
                        } => MaindyOsAction::ReadVaultFile {
                            file_name: file_path,
                        },
                        mindset_intelligence::agent_tools::AgentAction::OpenApp { app_id } => {
                            MaindyOsAction::LaunchApp { app_id }
                        }
                        mindset_intelligence::agent_tools::AgentAction::ControlSystem {
                            action,
                        } => match action {
                            mindset_intelligence::agent_tools::SystemControlAction::StartPomodoro {
                                duration_mins,
                            } => MaindyOsAction::StartFocusSprint {
                                capsule_id: "active-capsule".to_string(),
                                minutes: duration_mins.min(u16::MAX as u32) as u16,
                            },
                            mindset_intelligence::agent_tools::SystemControlAction::OpenBrowserUrl(_) => {
                                MaindyOsAction::LaunchApp {
                                    app_id: "google".to_string(),
                                }
                            }
                            _ => MaindyOsAction::OpenSettings {
                                section: Some(SettingsSection::General),
                            },
                        },
                    };
                    return (Some(mapped), cleaned);
                }
                return (None, cleaned);
            }
        }

        (None, raw.trim().to_string())
    }

    fn extract_duration_minutes(lower: &str) -> Option<u16> {
        let words: Vec<&str> = lower.split_whitespace().collect();
        for i in 0..words.len() {
            if let Ok(n) = words[i].parse::<u16>() {
                if let Some(next) = words.get(i + 1) {
                    if next.starts_with("min") {
                        return Some(n.clamp(10, 120));
                    }
                }
            }
        }
        None
    }

    fn extract_capsule_topic(prompt: &str) -> String {
        let lower = prompt.to_lowercase();
        for sep in [" for ", " on ", " about "] {
            if let Some(idx) = lower.find(sep) {
                let candidate = prompt[idx + sep.len()..].trim();
                let cleaned = candidate.trim_end_matches(|c: char| ".!?".contains(c));
                if !cleaned.is_empty() {
                    return Self::title_case_topic(cleaned);
                }
            }
        }
        if lower.contains("gauss") {
            "Gauss's Law & Flux".to_string()
        } else if lower.contains("nernst") {
            "Nernst Equation & EMF".to_string()
        } else if lower.contains("wave optics") {
            "Wave Optics & Interference".to_string()
        } else {
            "High-Yield Syllabus Mastery".to_string()
        }
    }

    fn extract_filename_token(text: &str) -> Option<&str> {
        for tok in text.split_whitespace() {
            let clean = tok.trim_matches(|c: char| "\"'`,?!()[]".contains(c));
            if clean.ends_with(".txt")
                || clean.ends_with(".pdf")
                || clean.ends_with(".md")
                || clean.ends_with(".capsule")
                || clean.ends_with(".pptx")
            {
                return Some(clean);
            }
        }
        None
    }

    fn title_case_topic(raw: &str) -> String {
        let s = raw.trim();
        if s.is_empty() {
            return "High-Yield Concept".to_string();
        }
        s.chars().take(64).collect()
    }

    fn knowledge_pack_for_topic(topic: &str, subject: &str) -> (Vec<String>, Vec<String>) {
        let q = topic.to_lowercase();
        if q.contains("gauss") {
            (
                vec![
                    "Φ_E = ∮ E · dA = q_enclosed / ε₀".to_string(),
                    "Infinite Plane Sheet: E = σ / (2 ε₀)".to_string(),
                    "Spherical Shell (r > R): E = q / (4π ε₀ r²)".to_string(),
                ],
                vec![
                    "Step 1: Choose a closed Gaussian surface exploiting spherical, cylindrical, or planar symmetry."
                        .to_string(),
                    "Step 2: Only enclosed charge q_enclosed inside the surface contributes to net electric flux Φ_E."
                        .to_string(),
                    "Step 3: Evaluate E·dA over flat/curved faces; zero flux through surfaces perpendicular to E."
                        .to_string(),
                    "Exam Trap: Charges outside the closed Gaussian surface contribute ZERO net flux."
                        .to_string(),
                ],
            )
        } else if q.contains("nernst") {
            (
                vec![
                    "E_cell = E°_cell - (RT / nF) ln(Q)".to_string(),
                    "At 298 K: E_cell = E°_cell - (0.0591 / n) log₁₀([Products]/[Reactants])"
                        .to_string(),
                    "ΔG° = -n F E°_cell".to_string(),
                ],
                vec![
                    "Step 1: Write balanced half-cell reactions and find electrons transferred n."
                        .to_string(),
                    "Step 2: Calculate E°_cell = E°_cathode - E°_anode from standard reduction potentials."
                        .to_string(),
                    "Step 3: Construct reaction quotient Q excluding pure solids and liquids (activity = 1)."
                        .to_string(),
                    "Exam Trap: At equilibrium E_cell = 0 and log₁₀(K_c) = n E°_cell / 0.0591."
                        .to_string(),
                ],
            )
        } else if q.contains("wave optics") || q.contains("huygens") || q.contains("slit") {
            (
                vec![
                    "Fringe Width: β = λ D / d".to_string(),
                    "Snell's Law (Wavefront): sin(i) / sin(r) = v₁ / v₂ = n₂ / n₁".to_string(),
                    "Resultant Intensity: I = I₁ + I₂ + 2√(I₁ I₂) cos(φ)".to_string(),
                ],
                vec![
                    "Step 1: Every point on a primary wavefront acts as a source of secondary wavelets."
                        .to_string(),
                    "Step 2: Path difference Δx = d sin(θ) ≈ y d / D for small-angle screens."
                        .to_string(),
                    "Step 3: Bright fringes occur when Δx = n λ; dark fringes when Δx = (n + ½) λ."
                        .to_string(),
                    "Exam Trap: Submerging YDSE in refractive index μ decreases λ and fringe width by μ."
                        .to_string(),
                ],
            )
        } else {
            (
                vec![
                    format!("{} — Fundamental Conservation & Governing Law", topic),
                    "Check Strict SI Base Units & Boundary Limits".to_string(),
                ],
                vec![
                    format!(
                        "Step 1: Map the core physical/chemical definitions of {} ({}).",
                        topic, subject
                    ),
                    "Step 2: Derive the governing relation symbolically before plugging in constants."
                        .to_string(),
                    "Step 3: Verify special/limiting cases and memorize high-yield examiner deductions."
                        .to_string(),
                ],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_path_system_access_routing() {
        let r_set = MaindySystemBridge::route_fast_path("open settings").expect("settings route");
        assert!(matches!(
            r_set.action,
            Some(MaindyOsAction::OpenSettings { .. })
        ));
        assert!(!r_set.action.as_ref().unwrap().is_destructive());

        let r_wifi =
            MaindySystemBridge::route_fast_path("open Wi-Fi settings").expect("wifi route");
        assert_eq!(
            r_wifi.action,
            Some(MaindyOsAction::OpenSettings {
                section: Some(SettingsSection::Wifi)
            })
        );

        let r_files = MaindySystemBridge::route_fast_path("open files").expect("files route");
        assert!(matches!(
            r_files.action,
            Some(MaindyOsAction::OpenFiles { .. })
        ));

        let r_cap = MaindySystemBridge::route_fast_path("Create a 30 min capsule for Wave Optics")
            .expect("capsule route");
        assert!(matches!(
            r_cap.action,
            Some(MaindyOsAction::CreateAndPresentCapsule {
                duration_mins: 30,
                ..
            })
        ));

        let r_dnd = MaindySystemBridge::route_fast_path("Turn on Do Not Disturb").expect("dnd");
        assert_eq!(
            r_dnd.action,
            Some(MaindyOsAction::ToggleSystemControl {
                control: SystemControl::DoNotDisturb,
                enabled: true,
            })
        );
    }

    #[test]
    fn test_capsule_persistence_roundtrip() {
        let dir = std::env::temp_dir().join(format!("mindset_cap_test_{}", Uuid::new_v4()));
        let cap = MaindySystemBridge::build_capsule("Gauss's Law", Some("Physics"), 25);
        assert_eq!(cap.phases.len(), 4);
        assert!(!cap.core_formulas.is_empty());

        let saved_path =
            MaindySystemBridge::save_capsule_to_dir(&cap, &dir).expect("save capsule json");
        assert!(saved_path.exists());

        let loaded = MaindySystemBridge::load_capsules_from_dir(&dir);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, cap.id);
        assert_eq!(loaded[0].subject, "Physics");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_strip_os_action_tags_cleanly() {
        let raw = "Let's master this.\n<os_action>{\"action\":\"OpenApp\",\"params\":{\"app_id\":\"youtube\"}}</os_action>\nFocus now!";
        let (act, cleaned) = MaindySystemBridge::extract_llm_os_action(raw);
        assert!(matches!(act, Some(MaindyOsAction::LaunchApp { .. })));
        assert!(!cleaned.contains("<os_action>"));
        assert!(!cleaned.contains("</os_action>"));
    }
}
