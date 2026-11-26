//! SQLite Persistence Layer for MindSet OS (mindset_os.db)
//!
//! Enforces WAL mode, 2MB cache cap (PRAGMA cache_size = -2000),
//! paginated message retrieval (last 30 messages), and student memory.

use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub onboarding_completed: bool,
    pub name: String,
    pub greeting_style: String,
    pub career_track: String,
    pub stream_or_focus: String,
    pub target_score_value: u32,
    pub target_score_max: u32,
    pub target_label: String,
    pub exam_date: String,
    pub baseline_level: String,
    pub current_mock_score: u32,
    pub subjects: Vec<String>,
    pub weak_topics: Vec<String>,
    pub school_or_work_hours: String,
    pub wake_time: String,
    pub bed_time: String,
    pub weekday_study_hours: u32,
    pub weekend_study_hours: u32,
    pub peak_energy_window: String,
    pub obstacles: Vec<String>,
    pub coaching_style: String,
    pub language_pref: String,
    pub ram_profile: String,
    pub wallpaper_path: String,
    pub map_theme: String,
    pub theme_mode: String,
    pub mild_motion_enabled: bool,
    pub language_code: String,
    pub maindy_style: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            onboarding_completed: false,
            name: "Scholar".to_string(),
            greeting_style: "Focused & Motivated".to_string(),
            career_track: "Class12BoardsTN600".to_string(),
            stream_or_focus: "Science (Physics, Chem, Math, Bio)".to_string(),
            target_score_value: 590,
            target_score_max: 600,
            target_label: "TN Class 12 Boards • 590/600 Target".to_string(),
            exam_date: "2027-03-01".to_string(),
            baseline_level: "Intermediate (70%)".to_string(),
            current_mock_score: 430,
            subjects: vec![
                "Physics".into(),
                "Chemistry".into(),
                "Mathematics".into(),
                "Biology".into(),
            ],
            weak_topics: vec![
                "Electrochemistry Nernst Equation".into(),
                "Gauss Law & Flux".into(),
                "Matrix Determinants".into(),
            ],
            school_or_work_hours: "08:30 - 15:30".to_string(),
            wake_time: "05:30".to_string(),
            bed_time: "22:30".to_string(),
            weekday_study_hours: 4,
            weekend_study_hours: 7,
            peak_energy_window: "Morning (06:00 - 08:30)".to_string(),
            obstacles: vec!["Digital Distractions".into(), "Post-School Fatigue".into()],
            coaching_style: "Calm Mentor".to_string(),
            language_pref: "English (US)".to_string(),
            ram_profile: "LowRam4GB".to_string(),
            wallpaper_path: "assets/wallpaper.jpg".to_string(),
            map_theme: "AlpineTrail".to_string(),
            theme_mode: "AlpineMatteLight".to_string(),
            mild_motion_enabled: true,
            language_code: "en".to_string(),
            maindy_style: "thanglish".to_string(),
        }
    }
}

impl UserProfile {
    pub fn days_remaining(&self) -> i64 {
        if let Ok(target) = NaiveDate::parse_from_str(&self.exam_date, "%Y-%m-%d") {
            let now = Local::now().date_naive();
            (target - now).num_days().max(0)
        } else {
            90
        }
    }

    pub fn target_exam_display(&self) -> String {
        let max = if self.target_score_max > 0 {
            self.target_score_max
        } else {
            600
        };
        let clamped = self.target_score_value.min(max);
        match self.career_track.as_str() {
            "Class12BoardsTN600" => {
                format!("TN Class 12 Boards • Target: {}/600", clamped.min(600))
            }
            "Class12BoardsCBSE500" => {
                format!("CBSE Class 12 Boards • Target: {}/500", clamped.min(500))
            }
            "NEET720" => format!("NEET-UG • Target: {}/720", clamped.min(720)),
            "JEE300" => format!("JEE Main/Adv • Target: {}/300", clamped.min(300)),
            "UPSC_TNPSC" => format!("UPSC / TNPSC • Target: {}/200", clamped.min(200)),
            "CA" => format!("CA Foundation • Target: {}/400", clamped.min(400)),
            "TechCareer" => format!("Software Engineering • Target: {}/100", clamped.min(100)),
            _ => format!("Academic Goal • Target: {}/{}", clamped, max),
        }
    }

    /// Formats stored coaching style keys into clean human-readable labels.
    pub fn coaching_tone_display(&self) -> String {
        match self.coaching_style.trim() {
            "BalancedMentor" | "" => "Calm Mentor".to_string(),
            "FriendlyThanglishBro" => "Friendly Thanglish Bro".to_string(),
            "StrictExamCoach" => "Strict Exam Coach".to_string(),
            other => {
                let mut out = String::with_capacity(other.len() + 4);
                for (i, ch) in other.chars().enumerate() {
                    if i > 0
                        && ch.is_uppercase()
                        && other
                            .chars()
                            .nth(i - 1)
                            .is_some_and(|prev| prev.is_lowercase())
                    {
                        out.push(' ');
                    }
                    out.push(ch);
                }
                out
            }
        }
    }

    pub fn track_name(&self) -> &str {
        match self.career_track.as_str() {
            "Class12BoardsTN600" => "TN Class 12 Boards",
            "Class12BoardsCBSE500" => "CBSE Class 12 Boards",
            "NEET720" => "NEET-UG",
            "JEE300" => "JEE Main & Advanced",
            "UPSC_TNPSC" => "UPSC / TNPSC",
            "CA" => "CA Foundation",
            "TechCareer" => "Software Systems",
            _ => {
                if self.career_track.is_empty() {
                    "Competitive Exam"
                } else {
                    &self.career_track
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatThread {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub rolling_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageRecord {
    pub id: i64,
    pub thread_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMemory {
    pub id: i64,
    pub memory_text: String,
    pub category: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCheckinRecord {
    pub date: String,
    pub energy_level: String, // "High-Energy Sprint", "Normal", "Busy Day", "Exhausted"
    pub available_hours_today: f32,
    pub situation_note: String,
    pub recommended_capsule_minutes: u32,
    pub recommended_break_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleRecord {
    pub id: String,
    pub target_date: String,
    pub subject: String,
    pub unit: String,
    pub estimated_minutes: u32,
    pub xp_reward: u32,
    pub status: String, // "Pending", "In Progress", "Completed", "Missed"
    pub stage: String,  // "Heavy Lift", "Active Practice", "Memory Anchor", "Proof-of-Study"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetLayoutRecord {
    pub widget_id: String,
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub is_pinned: bool,
    pub z_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiActivityRecord {
    pub id: i64,
    pub action_type: String,
    pub target_resource: String,
    pub status: String,
    pub details: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfigRecord {
    pub active_mode: String,
    pub cloud_provider: String,
    pub cloud_api_key: String,
    pub cloud_model: String,
    pub cloud_base_url: String,
    pub ollama_host: String,
    pub ollama_model: String,
    pub tts_engine: String,
    pub tts_api_key: String,
    pub tts_auto_speak: bool,
    pub tts_speech_rate: f32,
}

impl Default for AiProviderConfigRecord {
    fn default() -> Self {
        Self {
            active_mode: "auto".to_string(),
            cloud_provider: "gemini".to_string(),
            cloud_api_key: String::new(),
            cloud_model: "gemini-2.5-flash".to_string(),
            cloud_base_url: String::new(),
            ollama_host: "http://127.0.0.1:11434".to_string(),
            ollama_model: "qwen2.5:1.5b".to_string(),
            tts_engine: "os_native".to_string(),
            tts_api_key: String::new(),
            tts_auto_speak: false,
            tts_speech_rate: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageManager {
    db_path: PathBuf,
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageManager {
    pub fn new() -> Self {
        let base_dir = crate::ai::maindy_system_bridge::linux_data_root();
        let _ = fs::create_dir_all(&base_dir);
        let db_path = base_dir.join("mindset_os.db");

        Self::with_path(db_path)
    }

    pub fn with_path(db_path: PathBuf) -> Self {
        if let Some(parent) = db_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = fs::create_dir_all(parent);
            }
        }
        let manager = Self { db_path };
        let _ = manager.init_schema();
        manager
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn open_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        // Enforce WAL mode and strict 2MB RAM cache ceiling
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -2000;
             PRAGMA temp_store = MEMORY;",
        )?;
        Ok(conn)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.open_connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS user_profile (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                name TEXT NOT NULL,
                target_exam TEXT NOT NULL,
                target_score INTEGER NOT NULL,
                exam_date TEXT NOT NULL,
                subjects_json TEXT NOT NULL,
                weak_topics_json TEXT NOT NULL,
                daily_study_hours INTEGER NOT NULL,
                preferred_study_window TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chat_threads (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                rolling_summary TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS chat_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                thread_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (thread_id) REFERENCES chat_threads(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS user_memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                memory_text TEXT NOT NULL,
                category TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS daily_capsules (
                id TEXT PRIMARY KEY,
                target_date TEXT NOT NULL,
                subject TEXT NOT NULL,
                unit TEXT NOT NULL,
                estimated_minutes INTEGER NOT NULL,
                xp_reward INTEGER NOT NULL,
                status TEXT NOT NULL,
                stage TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS daily_checkins (
                date TEXT PRIMARY KEY,
                energy_level TEXT NOT NULL,
                available_hours_today REAL NOT NULL,
                situation_note TEXT NOT NULL,
                recommended_capsule_minutes INTEGER NOT NULL DEFAULT 30,
                recommended_break_minutes INTEGER NOT NULL DEFAULT 5
            );

            CREATE TABLE IF NOT EXISTS capsule_plans (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                plan_json TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS installed_ms_apps (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                kind TEXT NOT NULL,
                manifest_json TEXT NOT NULL,
                install_dir TEXT NOT NULL,
                installed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS student_mistake_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                subject TEXT NOT NULL,
                chapter TEXT NOT NULL,
                question_prompt TEXT NOT NULL,
                student_wrong_answer TEXT NOT NULL,
                correct_answer TEXT NOT NULL,
                misconception_category TEXT NOT NULL,
                review_count INTEGER NOT NULL DEFAULT 0,
                is_resolved INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS leitner_cards (
                id TEXT PRIMARY KEY,
                front_prompt TEXT NOT NULL,
                back_solution TEXT NOT NULL,
                subject TEXT NOT NULL,
                current_box INTEGER NOT NULL DEFAULT 1,
                last_reviewed_at TEXT NOT NULL,
                next_review_due TEXT NOT NULL,
                consecutive_successes INTEGER NOT NULL DEFAULT 0,
                total_reviews INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS scratchpad_notes (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                content TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS widget_layouts (
                widget_id TEXT PRIMARY KEY,
                pos_x REAL NOT NULL,
                pos_y REAL NOT NULL,
                width REAL NOT NULL,
                height REAL NOT NULL,
                is_pinned INTEGER NOT NULL DEFAULT 0,
                z_index INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS ai_provider_config (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                active_mode TEXT NOT NULL DEFAULT 'auto',
                cloud_provider TEXT NOT NULL DEFAULT 'gemini',
                cloud_api_key TEXT NOT NULL DEFAULT '',
                cloud_model TEXT NOT NULL DEFAULT 'gemini-2.5-flash',
                cloud_base_url TEXT NOT NULL DEFAULT '',
                ollama_host TEXT NOT NULL DEFAULT 'http://127.0.0.1:11434',
                ollama_model TEXT NOT NULL DEFAULT 'qwen2.5:1.5b',
                tts_engine TEXT NOT NULL DEFAULT 'os_native',
                tts_api_key TEXT NOT NULL DEFAULT '',
                tts_auto_speak INTEGER NOT NULL DEFAULT 0,
                tts_speech_rate REAL NOT NULL DEFAULT 1.0
            );

            CREATE TABLE IF NOT EXISTS ai_activity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action_type TEXT NOT NULL,
                target_resource TEXT NOT NULL,
                status TEXT NOT NULL,
                details TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS auth_credentials (
                user_id TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL,
                pin_hash TEXT,
                face_embedding_blob BLOB,
                fingerprint_enrolled INTEGER NOT NULL DEFAULT 0,
                auto_lock_minutes INTEGER NOT NULL DEFAULT 5,
                failed_attempts INTEGER NOT NULL DEFAULT 0,
                locked_until TEXT
            );",
        )?;

        // Safe migrations for extended student situation fields
        let migrations = [
            "ALTER TABLE user_profile ADD COLUMN onboarding_completed INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE user_profile ADD COLUMN greeting_style TEXT NOT NULL DEFAULT 'Focused'",
            "ALTER TABLE user_profile ADD COLUMN career_track TEXT NOT NULL DEFAULT 'Class12BoardsTN600'",
            "ALTER TABLE user_profile ADD COLUMN stream_or_focus TEXT NOT NULL DEFAULT 'Science'",
            "ALTER TABLE user_profile ADD COLUMN target_score_max INTEGER NOT NULL DEFAULT 600",
            "ALTER TABLE user_profile ADD COLUMN target_label TEXT NOT NULL DEFAULT 'TN Class 12 Boards • 590/600 Target'",
            "ALTER TABLE user_profile ADD COLUMN baseline_level TEXT NOT NULL DEFAULT 'Intermediate'",
            "ALTER TABLE user_profile ADD COLUMN current_mock_score INTEGER NOT NULL DEFAULT 430",
            "ALTER TABLE user_profile ADD COLUMN school_or_work_hours TEXT NOT NULL DEFAULT '08:30 - 15:30'",
            "ALTER TABLE user_profile ADD COLUMN wake_time TEXT NOT NULL DEFAULT '05:30'",
            "ALTER TABLE user_profile ADD COLUMN bed_time TEXT NOT NULL DEFAULT '22:30'",
            "ALTER TABLE user_profile ADD COLUMN weekday_study_hours INTEGER NOT NULL DEFAULT 4",
            "ALTER TABLE user_profile ADD COLUMN weekend_study_hours INTEGER NOT NULL DEFAULT 7",
            "ALTER TABLE user_profile ADD COLUMN peak_energy_window TEXT NOT NULL DEFAULT 'Morning'",
            "ALTER TABLE user_profile ADD COLUMN obstacles_json TEXT NOT NULL DEFAULT '[]'",
            "ALTER TABLE user_profile ADD COLUMN coaching_style TEXT NOT NULL DEFAULT 'BalancedMentor'",
            "ALTER TABLE user_profile ADD COLUMN language_pref TEXT NOT NULL DEFAULT 'English (US)'",
            "ALTER TABLE user_profile ADD COLUMN ram_profile TEXT NOT NULL DEFAULT 'LowRam4GB'",
            "ALTER TABLE user_profile ADD COLUMN wallpaper_path TEXT NOT NULL DEFAULT 'assets/wallpaper.jpg'",
            "ALTER TABLE user_profile ADD COLUMN map_theme TEXT NOT NULL DEFAULT 'AlpineTrail'",
            "ALTER TABLE user_profile ADD COLUMN theme_mode TEXT NOT NULL DEFAULT 'AlpineMatteLight'",
            "ALTER TABLE user_profile ADD COLUMN mild_motion_enabled INTEGER NOT NULL DEFAULT 1",
            "ALTER TABLE user_profile ADD COLUMN language_code TEXT NOT NULL DEFAULT 'en'",
            "ALTER TABLE user_profile ADD COLUMN maindy_style TEXT NOT NULL DEFAULT 'thanglish'",
            "ALTER TABLE scratchpad_notes ADD COLUMN is_pinned INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE daily_checkins ADD COLUMN recommended_capsule_minutes INTEGER NOT NULL DEFAULT 30",
            "ALTER TABLE daily_checkins ADD COLUMN recommended_break_minutes INTEGER NOT NULL DEFAULT 5",
            "ALTER TABLE user_memories ADD COLUMN fact_key TEXT NOT NULL DEFAULT ''",
        ];

        for mig in migrations {
            let _ = conn.execute(mig, []);
        }

        // Deduplicate singleton / identical student personalization memories on startup
        self.deduplicate_memories_in_conn(&conn);

        // Seed default profile if table is brand new
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM user_profile WHERE id = 1", [], |r| {
                r.get(0)
            })?;

        if count == 0 {
            let def = UserProfile::default();
            let subjects_json = serde_json::to_string(&def.subjects).unwrap_or_default();
            let weak_json = serde_json::to_string(&def.weak_topics).unwrap_or_default();
            let obs_json = serde_json::to_string(&def.obstacles).unwrap_or_default();
            let _ = conn.execute(
                "INSERT INTO user_profile (
                    id, name, target_exam, target_score, exam_date, subjects_json, weak_topics_json,
                    daily_study_hours, preferred_study_window, onboarding_completed, greeting_style,
                    career_track, stream_or_focus, target_score_max, target_label, baseline_level,
                    current_mock_score, school_or_work_hours, wake_time, bed_time, weekday_study_hours,
                    weekend_study_hours, peak_energy_window, obstacles_json, coaching_style, language_pref, ram_profile
                ) VALUES (
                    1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26
                )",
                params![
                    def.name,
                    def.target_exam_display(),
                    def.target_score_value,
                    def.exam_date,
                    subjects_json,
                    weak_json,
                    def.weekday_study_hours,
                    def.peak_energy_window,
                    if def.onboarding_completed { 1 } else { 0 },
                    def.greeting_style,
                    def.career_track,
                    def.stream_or_focus,
                    def.target_score_max,
                    def.target_label,
                    def.baseline_level,
                    def.current_mock_score,
                    def.school_or_work_hours,
                    def.wake_time,
                    def.bed_time,
                    def.weekday_study_hours,
                    def.weekend_study_hours,
                    def.peak_energy_window,
                    obs_json,
                    def.coaching_style,
                    def.language_pref,
                    def.ram_profile,
                ],
            );
        }

        // Self-healing: clamp legacy/corrupted target_score in user_profile
        let _ = conn.execute(
            "UPDATE user_profile SET target_score = CASE WHEN target_score_max > 0 THEN MIN(target_score, target_score_max) ELSE 590 END, target_score_max = CASE WHEN target_score_max > 0 THEN target_score_max ELSE 600 END WHERE target_score > target_score_max OR target_score_max = 0",
            [],
        );

        // Seed default chat thread if none exists
        let thread_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM chat_threads", [], |r| r.get(0))?;
        if thread_count == 0 {
            let now = Local::now().to_rfc3339();
            conn.execute(
                "INSERT INTO chat_threads (id, title, created_at, updated_at, rolling_summary)
                 VALUES ('main-session', 'Maindy AI Study Mentor', ?1, ?1, '')",
                params![now],
            )?;
            conn.execute(
                "INSERT INTO chat_messages (thread_id, role, content, created_at)
                 VALUES ('main-session', 'assistant', 'Greetings! I am Maindy AI, your goal-guided personal tutor. I am here to help you conquer high-yield syllabus concepts with step-by-step clarity.', ?1)",
                params![now],
            )?;
        }

        // Seed default memories if empty
        let mem_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM user_memories", [], |r| r.get(0))?;
        if mem_count == 0 {
            let now = Local::now().to_rfc3339();
            let _ = conn.execute(
                "INSERT INTO user_memories (memory_text, category, created_at) VALUES (?1, ?2, ?3)",
                params![
                    "Struggles with Nernst equation concentration quotients when non-standard states are applied.",
                    "Growth Area",
                    now
                ],
            );
            let _ = conn.execute(
                "INSERT INTO user_memories (memory_text, category, created_at) VALUES (?1, ?2, ?3)",
                params![
                    "Prefers morning study sprints (06:00 - 08:30) before school fatigue sets in.",
                    "Routine",
                    now
                ],
            );
        }

        // Seed default credentials for "scholar"
        let _ = mindset_sys::auth::CredentialsManager::ensure_default_credentials(&conn, "scholar");

        Ok(())
    }

    pub fn load_profile(&self) -> UserProfile {
        if let Ok(conn) = self.open_connection() {
            let result = conn.query_row(
                "SELECT name, target_exam, target_score, exam_date, subjects_json, weak_topics_json,
                        daily_study_hours, preferred_study_window, onboarding_completed, greeting_style,
                        career_track, stream_or_focus, target_score_max, target_label, baseline_level,
                        current_mock_score, school_or_work_hours, wake_time, bed_time, weekday_study_hours,
                        weekend_study_hours, peak_energy_window, obstacles_json, coaching_style, language_pref, ram_profile,
                        wallpaper_path, map_theme, theme_mode, mild_motion_enabled, language_code, maindy_style
                 FROM user_profile WHERE id = 1",
                [],
                |row| {
                    let name: String = row.get(0)?;
                    let _target_exam_legacy: String = row.get(1)?;
                    let target_score_raw: u32 = row.get(2)?;
                    let exam_date: String = row.get(3)?;
                    let subjects_raw: String = row.get(4)?;
                    let weak_raw: String = row.get(5)?;
                    let _daily_hours_legacy: u32 = row.get(6)?;
                    let _preferred_window_legacy: String = row.get(7)?;
                    let onboarding_completed_int: i32 = row.get(8).unwrap_or(0);
                    let greeting_style: String = row.get(9).unwrap_or_else(|_| "Focused".into());
                    let career_track: String = row.get(10).unwrap_or_else(|_| "Class12BoardsTN600".into());
                    let stream_or_focus: String = row.get(11).unwrap_or_else(|_| "Science".into());
                    let target_score_max: u32 = row.get(12).unwrap_or(600);
                    let target_score_value = target_score_raw;
                    let target_label: String = row.get(13).unwrap_or_else(|_| "TN Class 12 Boards • 590/600 Target".into());
                    let baseline_level: String = row.get(14).unwrap_or_else(|_| "Intermediate".into());
                    let current_mock_score: u32 = row.get(15).unwrap_or(430);
                    let school_or_work_hours: String = row.get(16).unwrap_or_else(|_| "08:30 - 15:30".into());
                    let wake_time: String = row.get(17).unwrap_or_else(|_| "05:30".into());
                    let bed_time: String = row.get(18).unwrap_or_else(|_| "22:30".into());
                    let weekday_study_hours: u32 = row.get(19).unwrap_or(4);
                    let weekend_study_hours: u32 = row.get(20).unwrap_or(7);
                    let peak_energy_window: String = row.get(21).unwrap_or_else(|_| "Morning".into());
                    let obs_raw: String = row.get(22).unwrap_or_else(|_| "[]".into());
                    let coaching_style: String = row.get(23).unwrap_or_else(|_| "BalancedMentor".into());
                    let language_pref: String = row.get(24).unwrap_or_else(|_| "English (US)".into());
                    let ram_profile: String = row.get(25).unwrap_or_else(|_| "LowRam4GB".into());
                    let wallpaper_path: String = row.get(26).unwrap_or_else(|_| "assets/wallpaper.jpg".into());
                    let map_theme: String = row.get(27).unwrap_or_else(|_| "AlpineTrail".into());
                    let theme_mode: String = row.get(28).unwrap_or_else(|_| "AlpineMatteLight".into());
                    let mild_motion_enabled: bool = row.get::<_, i32>(29).unwrap_or(1) != 0;
                    let language_code: String = row.get(30).unwrap_or_else(|_| "en".into());
                    let maindy_style: String = row.get(31).unwrap_or_else(|_| "thanglish".into());

                    let subjects = serde_json::from_str(&subjects_raw).unwrap_or_default();
                    let weak_topics = serde_json::from_str(&weak_raw).unwrap_or_default();
                    let obstacles = serde_json::from_str(&obs_raw).unwrap_or_default();

                    Ok(UserProfile {
                        onboarding_completed: onboarding_completed_int != 0,
                        name,
                        greeting_style,
                        career_track,
                        stream_or_focus,
                        target_score_value,
                        target_score_max,
                        target_label,
                        exam_date,
                        baseline_level,
                        current_mock_score,
                        subjects,
                        weak_topics,
                        school_or_work_hours,
                        wake_time,
                        bed_time,
                        weekday_study_hours,
                        weekend_study_hours,
                        peak_energy_window,
                        obstacles,
                        coaching_style,
                        language_pref,
                        ram_profile,
                        wallpaper_path,
                        map_theme,
                        theme_mode,
                        mild_motion_enabled,
                        language_code,
                        maindy_style,
                    })
                },
            );
            if let Ok(mut p) = result {
                let max = if p.target_score_max > 0 {
                    p.target_score_max
                } else {
                    600
                };
                p.target_score_max = max;
                if p.target_score_value > max || p.target_score_value == 0 {
                    // Auto-heal corrupted database record: clamp to target_score_max or 590 if TN600
                    p.target_score_value = if max == 600 { 590 } else { max };
                    let _ = self.save_profile(&p);
                }
                return p;
            }
        }
        UserProfile::default()
    }

    pub fn load_scratchpad(&self) -> Option<String> {
        if let Ok(conn) = self.open_connection() {
            let res: Result<String> = conn.query_row(
                "SELECT content FROM scratchpad_notes WHERE id = 1",
                [],
                |r| r.get(0),
            );
            if let Ok(text) = res {
                return Some(text);
            }
        }
        None
    }

    pub fn save_scratchpad(&self, content: &str) -> Result<()> {
        let conn = self.open_connection()?;
        let now = chrono::Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO scratchpad_notes (id, content, updated_at) VALUES (1, ?1, ?2)
             ON CONFLICT(id) DO UPDATE SET content = excluded.content, updated_at = excluded.updated_at",
            params![content, now],
        )?;
        Ok(())
    }

    pub fn load_or_migrate_scratchpad(&self, legacy_file: &std::path::Path) -> String {
        if let Some(existing) = self.load_scratchpad() {
            return existing;
        }

        let initial_text = if legacy_file.exists() {
            fs::read_to_string(legacy_file).unwrap_or_else(|_| default_scratchpad_text())
        } else {
            default_scratchpad_text()
        };

        let _ = self.save_scratchpad(&initial_text);
        initial_text
    }

    pub fn save_profile(&self, p: &UserProfile) -> Result<()> {
        let conn = self.open_connection()?;
        let sub_json = serde_json::to_string(&p.subjects).unwrap_or_default();
        let weak_json = serde_json::to_string(&p.weak_topics).unwrap_or_default();
        let obs_json = serde_json::to_string(&p.obstacles).unwrap_or_default();
        conn.execute(
            "UPDATE user_profile SET
                name = ?1, target_exam = ?2, target_score = ?3, exam_date = ?4,
                subjects_json = ?5, weak_topics_json = ?6, daily_study_hours = ?7,
                preferred_study_window = ?8, onboarding_completed = ?9, greeting_style = ?10,
                career_track = ?11, stream_or_focus = ?12, target_score_max = ?13,
                target_label = ?14, baseline_level = ?15, current_mock_score = ?16,
                school_or_work_hours = ?17, wake_time = ?18, bed_time = ?19,
                weekday_study_hours = ?20, weekend_study_hours = ?21, peak_energy_window = ?22,
                obstacles_json = ?23, coaching_style = ?24, language_pref = ?25, ram_profile = ?26,
                wallpaper_path = ?27, map_theme = ?28, theme_mode = ?29, mild_motion_enabled = ?30,
                language_code = ?31, maindy_style = ?32
             WHERE id = 1",
            params![
                p.name,
                p.target_exam_display(),
                p.target_score_value,
                p.exam_date,
                sub_json,
                weak_json,
                p.weekday_study_hours,
                p.peak_energy_window,
                if p.onboarding_completed { 1 } else { 0 },
                p.greeting_style,
                p.career_track,
                p.stream_or_focus,
                p.target_score_max,
                p.target_label,
                p.baseline_level,
                p.current_mock_score,
                p.school_or_work_hours,
                p.wake_time,
                p.bed_time,
                p.weekday_study_hours,
                p.weekend_study_hours,
                p.peak_energy_window,
                obs_json,
                p.coaching_style,
                p.language_pref,
                p.ram_profile,
                p.wallpaper_path,
                p.map_theme,
                p.theme_mode,
                if p.mild_motion_enabled { 1 } else { 0 },
                p.language_code,
                p.maindy_style,
            ],
        )?;
        let _ = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        Ok(())
    }

    pub fn save_capsule_plan(&self, plan: &mindset_core::TargetCapsulePlan) -> Result<()> {
        let conn = self.open_connection()?;
        let json = serde_json::to_string(plan).unwrap_or_default();
        let now = Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO capsule_plans (id, plan_json, updated_at)
             VALUES (1, ?1, ?2)
             ON CONFLICT(id) DO UPDATE SET plan_json = excluded.plan_json, updated_at = excluded.updated_at",
            params![json, now],
        )?;
        let _ = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        Ok(())
    }

    pub fn load_capsule_plan(&self) -> Option<mindset_core::TargetCapsulePlan> {
        let conn = self.open_connection().ok()?;
        let json: String = conn
            .query_row(
                "SELECT plan_json FROM capsule_plans WHERE id = 1",
                [],
                |r| r.get(0),
            )
            .ok()?;
        serde_json::from_str(&json).ok()
    }

    pub fn seed_localized_greeting(
        &self,
        thread_id: &str,
        student_name: &str,
        lang_style: &str,
        tone: &str,
    ) {
        let name = if student_name.trim().is_empty() {
            "Scholar"
        } else {
            student_name.trim()
        };
        let greeting = match lang_style {
            "thanglish" => match tone {
                "Strict Exam Coach" => format!(
                    "Vanakkam {name}! Naan Maindy AI. Time waste pannaama high-yield capsules mudippom — first goal ready, start pannalama?"
                ),
                _ => format!(
                    "Hey {name} bro! Naan unga Maindy AI. Innaiku semma-ya padichitu target-a crack pannalam! Edhavadhu doubt na kelu."
                ),
            },
            "ta" => format!(
                "வணக்கம் {name}! நான் உங்கள் Maindy AI. இன்றைய உயர்-மதிப்பெண் பாடங்களைத் தெளிவாகப் படித்து முடிக்கலாம்!"
            ),
            "ml" => format!(
                "നമസ്കാരം {name}! ഞാൻ നിങ്ങളുടെ Maindy AI ആണ്. ഇന്നത്തെ പഠന ലക്ഷ്യങ്ങൾ നമുക്ക് ഒരുമിച്ച് പൂർത്തിയാക്കാം!"
            ),
            _ => match tone {
                "Strict Exam Coach" => format!(
                    "Welcome {name}. I am Maindy AI. Your daily capsules are scheduled — let's lock in high-yield focus and hit your target score."
                ),
                _ => format!(
                    "Welcome {name}! I am Maindy AI, your personal study mentor. Let's tackle today's capsules step-by-step with zero stress."
                ),
            },
        };
        if let Ok(conn) = self.open_connection() {
            let now = Local::now().to_rfc3339();
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM chat_messages WHERE thread_id = ?1",
                    params![thread_id],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            if count <= 1 {
                let _ = conn.execute(
                    "DELETE FROM chat_messages WHERE thread_id = ?1",
                    params![thread_id],
                );
                let _ = conn.execute(
                    "INSERT INTO chat_messages (thread_id, role, content, created_at) VALUES (?1, 'assistant', ?2, ?3)",
                    params![thread_id, greeting, now],
                );
            }
        }
    }

    pub fn save_installed_ms_app(
        &self,
        id: &str,
        name: &str,
        version: &str,
        kind: &str,
        manifest_json: &str,
        install_dir: &str,
    ) -> Result<()> {
        let conn = self.open_connection()?;
        let now = Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO installed_ms_apps (id, name, version, kind, manifest_json, install_dir, installed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
               name = excluded.name,
               version = excluded.version,
               kind = excluded.kind,
               manifest_json = excluded.manifest_json,
               install_dir = excluded.install_dir,
               installed_at = excluded.installed_at",
            params![id, name, version, kind, manifest_json, install_dir, now],
        )?;
        Ok(())
    }

    pub fn list_installed_ms_apps(&self) -> Vec<(String, String, String, String, String, String)> {
        let Ok(conn) = self.open_connection() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare(
            "SELECT id, name, version, kind, manifest_json, install_dir FROM installed_ms_apps ORDER BY installed_at DESC",
        ) else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
            ))
        }) else {
            return Vec::new();
        };
        rows.flatten().collect()
    }

    pub fn remove_installed_ms_app(&self, id: &str) -> Result<()> {
        let conn = self.open_connection()?;
        conn.execute("DELETE FROM installed_ms_apps WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn load_widget_layouts(&self) -> HashMap<String, WidgetLayoutRecord> {
        let mut map = HashMap::new();
        if let Ok(conn) = self.open_connection() {
            if let Ok(mut stmt) = conn.prepare("SELECT widget_id, pos_x, pos_y, width, height, is_pinned, z_index FROM widget_layouts") {
                if let Ok(rows) = stmt.query_map([], |row| {
                    Ok(WidgetLayoutRecord {
                        widget_id: row.get(0)?,
                        pos_x: row.get::<_, f64>(1)? as f32,
                        pos_y: row.get::<_, f64>(2)? as f32,
                        width: row.get::<_, f64>(3)? as f32,
                        height: row.get::<_, f64>(4)? as f32,
                        is_pinned: row.get::<_, i32>(5)? != 0,
                        z_index: row.get(6)?,
                    })
                }) {
                    for r in rows.flatten() {
                        map.insert(r.widget_id.clone(), r);
                    }
                }
            }
        }
        map
    }

    pub fn save_widget_layout(&self, layout: &WidgetLayoutRecord) -> Result<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT INTO widget_layouts (widget_id, pos_x, pos_y, width, height, is_pinned, z_index)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(widget_id) DO UPDATE SET
                pos_x = excluded.pos_x,
                pos_y = excluded.pos_y,
                width = excluded.width,
                height = excluded.height,
                is_pinned = excluded.is_pinned,
                z_index = excluded.z_index",
            params![
                layout.widget_id,
                layout.pos_x as f64,
                layout.pos_y as f64,
                layout.width as f64,
                layout.height as f64,
                if layout.is_pinned { 1 } else { 0 },
                layout.z_index,
            ],
        )?;
        Ok(())
    }

    pub fn load_ai_config(&self) -> AiProviderConfigRecord {
        if let Ok(conn) = self.open_connection() {
            let res = conn.query_row(
                "SELECT active_mode, cloud_provider, cloud_api_key, cloud_model, cloud_base_url,
                        ollama_host, ollama_model, tts_engine, tts_api_key, tts_auto_speak, tts_speech_rate
                 FROM ai_provider_config WHERE id = 1",
                [],
                |row| {
                    Ok(AiProviderConfigRecord {
                        active_mode: row.get(0).unwrap_or_else(|_| "auto".into()),
                        cloud_provider: row.get(1).unwrap_or_else(|_| "gemini".into()),
                        cloud_api_key: row.get(2).unwrap_or_default(),
                        cloud_model: row.get(3).unwrap_or_else(|_| "gemini-2.5-flash".into()),
                        cloud_base_url: row.get(4).unwrap_or_default(),
                        ollama_host: row.get(5).unwrap_or_else(|_| "http://127.0.0.1:11434".into()),
                        ollama_model: row.get(6).unwrap_or_else(|_| "qwen2.5:1.5b".into()),
                        tts_engine: row.get(7).unwrap_or_else(|_| "os_native".into()),
                        tts_api_key: row.get(8).unwrap_or_default(),
                        tts_auto_speak: row.get::<_, i32>(9).unwrap_or(0) != 0,
                        tts_speech_rate: row.get::<_, f64>(10).unwrap_or(1.0) as f32,
                    })
                },
            );
            if let Ok(cfg) = res {
                return cfg;
            }
        }
        AiProviderConfigRecord::default()
    }

    pub fn save_ai_config(&self, cfg: &AiProviderConfigRecord) -> Result<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT INTO ai_provider_config (
                id, active_mode, cloud_provider, cloud_api_key, cloud_model, cloud_base_url,
                ollama_host, ollama_model, tts_engine, tts_api_key, tts_auto_speak, tts_speech_rate
            ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(id) DO UPDATE SET
                active_mode = excluded.active_mode,
                cloud_provider = excluded.cloud_provider,
                cloud_api_key = excluded.cloud_api_key,
                cloud_model = excluded.cloud_model,
                cloud_base_url = excluded.cloud_base_url,
                ollama_host = excluded.ollama_host,
                ollama_model = excluded.ollama_model,
                tts_engine = excluded.tts_engine,
                tts_api_key = excluded.tts_api_key,
                tts_auto_speak = excluded.tts_auto_speak,
                tts_speech_rate = excluded.tts_speech_rate",
            params![
                cfg.active_mode,
                cfg.cloud_provider,
                cfg.cloud_api_key,
                cfg.cloud_model,
                cfg.cloud_base_url,
                cfg.ollama_host,
                cfg.ollama_model,
                cfg.tts_engine,
                cfg.tts_api_key,
                if cfg.tts_auto_speak { 1 } else { 0 },
                cfg.tts_speech_rate as f64,
            ],
        )?;
        Ok(())
    }

    pub fn save_daily_checkin(&self, checkin: &DailyCheckinRecord) {
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute(
                "INSERT OR REPLACE INTO daily_checkins
                 (date, energy_level, available_hours_today, situation_note, recommended_capsule_minutes, recommended_break_minutes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    checkin.date,
                    checkin.energy_level,
                    checkin.available_hours_today,
                    checkin.situation_note,
                    checkin.recommended_capsule_minutes,
                    checkin.recommended_break_minutes,
                ],
            );
        }
    }

    pub fn get_daily_checkin(&self, date: &str) -> Option<DailyCheckinRecord> {
        if let Ok(conn) = self.open_connection() {
            let result = conn.query_row(
                "SELECT date, energy_level, available_hours_today, situation_note,
                        recommended_capsule_minutes, recommended_break_minutes
                 FROM daily_checkins WHERE date = ?1",
                params![date],
                |row| {
                    Ok(DailyCheckinRecord {
                        date: row.get(0)?,
                        energy_level: row.get(1)?,
                        available_hours_today: row.get(2)?,
                        situation_note: row.get(3)?,
                        recommended_capsule_minutes: row.get(4).unwrap_or(30),
                        recommended_break_minutes: row.get(5).unwrap_or(5),
                    })
                },
            );
            return result.ok();
        }
        None
    }

    pub fn list_threads(&self) -> Vec<ChatThread> {
        let mut list = Vec::new();
        if let Ok(conn) = self.open_connection() {
            if let Ok(mut stmt) = conn.prepare("SELECT id, title, created_at, updated_at, rolling_summary FROM chat_threads ORDER BY updated_at DESC") {
                let rows = stmt.query_map([], |row| {
                    Ok(ChatThread {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        created_at: row.get(2)?,
                        updated_at: row.get(3)?,
                        rolling_summary: row.get(4)?,
                    })
                });
                if let Ok(mapped) = rows {
                    for t in mapped.flatten() {
                        list.push(t);
                    }
                }
            }
        }
        list
    }

    pub fn create_thread(&self, title: &str) -> String {
        let id = format!("thread-{}", uuid::Uuid::new_v4());
        let now = Local::now().to_rfc3339();
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute(
                "INSERT INTO chat_threads (id, title, created_at, updated_at, rolling_summary)
                 VALUES (?1, ?2, ?3, ?3, '')",
                params![id, title, now],
            );
        }
        id
    }

    /// Paginated message loading: only loads the last 30 messages to keep RAM minimal (<60 MB RSS)
    pub fn load_recent_messages(&self, thread_id: &str, limit: usize) -> Vec<ChatMessageRecord> {
        let mut msgs = Vec::new();
        if let Ok(conn) = self.open_connection() {
            let sql = format!(
                "SELECT id, thread_id, role, content, created_at FROM chat_messages
                 WHERE thread_id = ?1 ORDER BY id DESC LIMIT {}",
                limit.max(1)
            );
            if let Ok(mut stmt) = conn.prepare(&sql) {
                let rows = stmt.query_map(params![thread_id], |row| {
                    Ok(ChatMessageRecord {
                        id: row.get(0)?,
                        thread_id: row.get(1)?,
                        role: row.get(2)?,
                        content: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                });
                if let Ok(mapped) = rows {
                    for m in mapped.flatten() {
                        msgs.push(m);
                    }
                }
            }
        }
        msgs.reverse(); // Display in chronological order
        msgs
    }

    pub fn save_message(&self, thread_id: &str, role: &str, content: &str) {
        let now = Local::now().to_rfc3339();
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute(
                "INSERT INTO chat_messages (thread_id, role, content, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![thread_id, role, content, now],
            );
            let _ = conn.execute(
                "UPDATE chat_threads SET updated_at = ?1 WHERE id = ?2",
                params![now, thread_id],
            );
        }
    }

    pub fn list_memories(&self) -> Vec<UserMemory> {
        let mut list = Vec::new();
        let mut seen_singletons = std::collections::HashSet::new();
        let mut seen_pairs = std::collections::HashSet::new();
        if let Ok(conn) = self.open_connection() {
            if let Ok(mut stmt) = conn.prepare(
                "SELECT id, memory_text, category, created_at FROM user_memories ORDER BY id DESC",
            ) {
                let rows = stmt.query_map([], |row| {
                    Ok(UserMemory {
                        id: row.get(0)?,
                        memory_text: row.get(1)?,
                        category: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                });
                if let Ok(mapped) = rows {
                    for m in mapped.flatten() {
                        let cat_key = m.category.trim().to_lowercase();
                        let is_singleton = matches!(
                            cat_key.as_str(),
                            "core mission" | "growth area" | "academic strength"
                        );
                        if is_singleton && !seen_singletons.insert(cat_key.clone()) {
                            continue;
                        }
                        let pair_key = (cat_key, m.memory_text.trim().to_lowercase());
                        if !seen_pairs.insert(pair_key) {
                            continue;
                        }
                        list.push(m);
                    }
                }
            }
        }
        list
    }

    fn deduplicate_memories_in_conn(&self, conn: &Connection) {
        let _ = conn.execute(
            "DELETE FROM user_memories WHERE id NOT IN (
                SELECT MAX(id) FROM user_memories GROUP BY LOWER(TRIM(category)), LOWER(TRIM(memory_text))
            )",
            [],
        );
        let _ = conn.execute(
            "DELETE FROM user_memories WHERE LOWER(TRIM(category)) IN ('core mission', 'growth area', 'academic strength')
             AND id NOT IN (
                SELECT MAX(id) FROM user_memories
                WHERE LOWER(TRIM(category)) IN ('core mission', 'growth area', 'academic strength')
                GROUP BY LOWER(TRIM(category))
             )",
            [],
        );
    }

    pub fn add_memory(&self, memory_text: &str, category: &str) {
        let trimmed_text = memory_text.trim();
        let trimmed_cat = category.trim();
        if trimmed_text.is_empty() || trimmed_cat.is_empty() {
            return;
        }
        let now = Local::now().to_rfc3339();
        let cat_lower = trimmed_cat.to_lowercase();
        let is_singleton = matches!(
            cat_lower.as_str(),
            "core mission" | "growth area" | "academic strength"
        );
        if let Ok(conn) = self.open_connection() {
            if is_singleton {
                let _ = conn.execute(
                    "DELETE FROM user_memories WHERE LOWER(TRIM(category)) = ?1",
                    params![cat_lower],
                );
            } else {
                let _ = conn.execute(
                    "DELETE FROM user_memories WHERE LOWER(TRIM(category)) = ?1 AND LOWER(TRIM(memory_text)) = ?2",
                    params![cat_lower, trimmed_text.to_lowercase()],
                );
            }
            let _ = conn.execute(
                "INSERT INTO user_memories (memory_text, category, created_at, fact_key)
                 VALUES (?1, ?2, ?3, ?4)",
                params![trimmed_text, trimmed_cat, now, cat_lower],
            );
        }
    }

    pub fn delete_memory(&self, id: i64) {
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute("DELETE FROM user_memories WHERE id = ?1", params![id]);
        }
    }

    pub fn log_ai_activity(
        &self,
        action_type: &str,
        target_resource: &str,
        status: &str,
        details: &str,
    ) {
        let now = Local::now().to_rfc3339();
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute(
                "INSERT INTO ai_activity_log (action_type, target_resource, status, details, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![action_type, target_resource, status, details, now],
            );
        }
    }

    pub fn list_ai_activities(&self, limit: usize) -> Vec<AiActivityRecord> {
        let mut list = Vec::new();
        if let Ok(conn) = self.open_connection() {
            if let Ok(mut stmt) = conn.prepare(
                "SELECT id, action_type, target_resource, status, details, timestamp
                 FROM ai_activity_log ORDER BY id DESC LIMIT ?1",
            ) {
                let rows = stmt.query_map(params![limit as i64], |row| {
                    Ok(AiActivityRecord {
                        id: row.get(0)?,
                        action_type: row.get(1)?,
                        target_resource: row.get(2)?,
                        status: row.get(3)?,
                        details: row.get(4)?,
                        timestamp: row.get(5)?,
                    })
                });
                if let Ok(mapped) = rows {
                    for r in mapped.flatten() {
                        list.push(r);
                    }
                }
            }
        }
        list
    }

    pub fn clear_ai_activities(&self) {
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute("DELETE FROM ai_activity_log", []);
        }
    }

    pub fn list_capsules(&self, date: &str) -> Vec<CapsuleRecord> {
        let mut list = Vec::new();
        if let Ok(conn) = self.open_connection() {
            if let Ok(mut stmt) = conn.prepare(
                "SELECT id, target_date, subject, unit, estimated_minutes, xp_reward, status, stage
                 FROM daily_capsules WHERE target_date = ?1",
            ) {
                let rows = stmt.query_map(params![date], |row| {
                    Ok(CapsuleRecord {
                        id: row.get(0)?,
                        target_date: row.get(1)?,
                        subject: row.get(2)?,
                        unit: row.get(3)?,
                        estimated_minutes: row.get(4)?,
                        xp_reward: row.get(5)?,
                        status: row.get(6)?,
                        stage: row.get(7)?,
                    })
                });
                if let Ok(mapped) = rows {
                    for c in mapped.flatten() {
                        list.push(c);
                    }
                }
            }
        }
        list
    }

    pub fn save_capsule(&self, c: &CapsuleRecord) {
        if let Ok(conn) = self.open_connection() {
            let _ = conn.execute(
                "INSERT OR REPLACE INTO daily_capsules (id, target_date, subject, unit, estimated_minutes, xp_reward, status, stage)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    c.id,
                    c.target_date,
                    c.subject,
                    c.unit,
                    c.estimated_minutes,
                    c.xp_reward,
                    c.status,
                    c.stage
                ],
            );
        }
    }

    /// Compact Student Context Builder (~250–280 tokens max) injected into Maindy AI's system prompt
    pub fn build_student_context_prompt(&self) -> String {
        let profile = self.load_profile();
        let weak_str = profile.weak_topics.join(", ");
        let memories = self.list_memories();
        let mem_str = if memories.is_empty() {
            "None logged yet.".to_string()
        } else {
            memories
                .iter()
                .take(3)
                .map(|m| format!("• [{}] {}", m.category, m.memory_text))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let today = Local::now().format("%Y-%m-%d").to_string();
        let checkin_str = if let Some(ch) = self.get_daily_checkin(&today) {
            format!(
                "Energy: {} • Available: {:.1}h • Note: {}",
                ch.energy_level, ch.available_hours_today, ch.situation_note
            )
        } else {
            "Standard Energy • On Plan".to_string()
        };

        format!(
            "You are Maindy AI, the empathetic personal study mentor inside MindSet OS.\n\
             Student: {} | Goal: {} (Target: {}/{}) | Days to Exam: {}\n\
             Routine: Wake {}, Bed {} | School/Work: {} | Peak Focus: {}\n\
             Today's Status: {}\n\
             Weak Areas to Prioritize: {}\n\
             Known Context & Memories:\n{}\n\
             Coaching Style: {} (Tone: Rigorous, compassionate, zero-guilt, step-by-step causal reasoning).",
            profile.name,
            profile.career_track,
            profile.target_score_value,
            profile.target_score_max,
            profile.days_remaining(),
            profile.wake_time,
            profile.bed_time,
            profile.school_or_work_hours,
            profile.peak_energy_window,
            checkin_str,
            if weak_str.is_empty() { "None" } else { &weak_str },
            mem_str,
            profile.coaching_style,
        )
    }
}

pub fn default_scratchpad_text() -> String {
    "// MindSet OS Quick Scratchpad (Auto-saved to SQLite)\n\
     • Gauss's Law: Φ = ∮ E·dA = q_enc / ε₀\n\
     • Nernst Eq: E = E° - (RT/nF) ln Q\n\
     • Target: 590/600 | Syllabus Sprint Active"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_score_auto_heal_and_display() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", uuid::Uuid::new_v4()));
        let _ = fs::create_dir_all(&temp_dir);
        let storage = StorageManager {
            db_path: temp_dir.join("test.db"),
        };
        assert!(storage.init_schema().is_ok());

        // Corrupt score directly
        let conn = storage.open_connection().unwrap();
        conn.execute(
            "UPDATE user_profile SET target_score = 10005, target_score_max = 600 WHERE id = 1",
            [],
        )
        .unwrap();

        // Load profile and verify auto-healing
        let healed = storage.load_profile();
        assert_eq!(healed.target_score_value, 590);
        assert_eq!(healed.target_score_max, 600);
        assert_eq!(
            healed.target_exam_display(),
            "TN Class 12 Boards • Target: 590/600"
        );

        // Verify it was persisted back to SQLite
        let raw_score: u32 = conn
            .query_row(
                "SELECT target_score FROM user_profile WHERE id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(raw_score, 590);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_scratchpad_persistence() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", uuid::Uuid::new_v4()));
        let _ = fs::create_dir_all(&temp_dir);
        let storage = StorageManager {
            db_path: temp_dir.join("test.db"),
        };
        assert!(storage.init_schema().is_ok());

        let legacy_file = temp_dir.join("legacy_scratch.txt");
        let initial_text = "E = mc^2 • First Law of Thermodynamics";
        fs::write(&legacy_file, initial_text).unwrap();

        // Test migration
        let loaded = storage.load_or_migrate_scratchpad(&legacy_file);
        assert_eq!(loaded, initial_text);

        // Update in DB
        let updated = "Schrödinger Eq: Hψ = Eψ";
        assert!(storage.save_scratchpad(updated).is_ok());
        assert_eq!(storage.load_scratchpad(), Some(updated.to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_widget_layouts_crud() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", uuid::Uuid::new_v4()));
        let _ = fs::create_dir_all(&temp_dir);
        let storage = StorageManager {
            db_path: temp_dir.join("test.db"),
        };
        assert!(storage.init_schema().is_ok());

        let layout = WidgetLayoutRecord {
            widget_id: "greeting_card".to_string(),
            pos_x: 120.0,
            pos_y: 85.0,
            width: 460.0,
            height: 180.0,
            is_pinned: true,
            z_index: 2,
        };
        assert!(storage.save_widget_layout(&layout).is_ok());

        let map = storage.load_widget_layouts();
        let loaded = map
            .get("greeting_card")
            .expect("greeting_card layout exists");
        assert_eq!(loaded.pos_x, 120.0);
        assert_eq!(loaded.pos_y, 85.0);
        assert!(loaded.is_pinned);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ai_provider_config_crud() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", uuid::Uuid::new_v4()));
        let _ = fs::create_dir_all(&temp_dir);
        let storage = StorageManager {
            db_path: temp_dir.join("test.db"),
        };
        assert!(storage.init_schema().is_ok());

        let mut cfg = storage.load_ai_config();
        assert_eq!(cfg.active_mode, "auto");
        assert_eq!(cfg.cloud_provider, "gemini");

        cfg.active_mode = "cloud".to_string();
        cfg.cloud_api_key = "AIzaSyTest123".to_string();
        cfg.tts_engine = "sarvam".to_string();
        assert!(storage.save_ai_config(&cfg).is_ok());

        let reloaded = storage.load_ai_config();
        assert_eq!(reloaded.active_mode, "cloud");
        assert_eq!(reloaded.cloud_api_key, "AIzaSyTest123");
        assert_eq!(reloaded.tts_engine, "sarvam");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_daily_checkins_migration_and_personalization_coexistence() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", uuid::Uuid::new_v4()));
        let _ = fs::create_dir_all(&temp_dir);
        let db_path = temp_dir.join("legacy_coexist.db");

        // Create legacy 4-column daily_checkins table before StorageManager::with_path runs
        {
            let conn = Connection::open(&db_path).unwrap();
            conn.execute_batch(
                "CREATE TABLE daily_checkins (
                    date TEXT PRIMARY KEY,
                    energy_level TEXT NOT NULL,
                    available_hours_today REAL NOT NULL,
                    situation_note TEXT NOT NULL
                );
                INSERT INTO daily_checkins (date, energy_level, available_hours_today, situation_note)
                VALUES ('2026-11-24', 'Normal', 3.0, 'Legacy row');",
            )
            .unwrap();
        }

        let storage = StorageManager::with_path(db_path.clone());
        assert_eq!(storage.db_path(), db_path.as_path());

        let legacy = storage.get_daily_checkin("2026-11-24").unwrap();
        assert_eq!(legacy.recommended_capsule_minutes, 30);
        assert_eq!(legacy.recommended_break_minutes, 5);

        let pe = mindset_intelligence::PersonalizationEngine::new(db_path.clone()).unwrap();
        let fetched = pe.get_fatigue_checkin("2026-11-24").unwrap().unwrap();
        assert_eq!(fetched.recommended_capsule_minutes, 30);
        assert_eq!(fetched.recommended_break_minutes, 5);

        let new_rec = DailyCheckinRecord {
            date: "2026-11-25".to_string(),
            energy_level: "Fatigued".to_string(),
            available_hours_today: 2.0,
            situation_note: "Tired".to_string(),
            recommended_capsule_minutes: 18,
            recommended_break_minutes: 8,
        };
        storage.save_daily_checkin(&new_rec);
        let loaded = storage.get_daily_checkin("2026-11-25").unwrap();
        assert_eq!(loaded.recommended_capsule_minutes, 18);
        assert_eq!(loaded.recommended_break_minutes, 8);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
