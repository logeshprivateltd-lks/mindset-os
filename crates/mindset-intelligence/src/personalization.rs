//! Day-by-Day Personalization Engine for MindSet OS (`mindset_os.db`).
//!
//! Provides:
//! - Daily fatigue & energy level check-ins with cognitive pacing scaling
//! - Student mistake / misconception tracking with automated review injection
//! - 5-Box Leitner Spaced Repetition scheduler following the Ebbinghaus retention curve
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{Local, NaiveDate, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Student subjective energy / fatigue level for cognitive pacing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FatigueLevel {
    High,
    Normal,
    Fatigued,
}

impl FatigueLevel {
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "high" | "energetic" | "fresh" => Self::High,
            "fatigued" | "tired" | "low" | "exhausted" => Self::Fatigued,
            _ => Self::Normal,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Normal => "Normal",
            Self::Fatigued => "Fatigued",
        }
    }
}

/// Record of daily fatigue check-in and available study capacity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyFatigueCheckin {
    pub date: String, // YYYY-MM-DD
    pub energy_level: FatigueLevel,
    pub available_hours_today: f32,
    pub situation_note: String,
    pub recommended_capsule_minutes: u32,
    pub recommended_break_minutes: u32,
}

impl DailyFatigueCheckin {
    /// Computes scientifically optimized capsule duration and defrag breaks:
    /// - High: 35-45m intensive deep-dive block, 5m break
    /// - Normal: 25-30m standard ultradian focus block, 5m break
    /// - Fatigued: 15-20m micro-focus block, 8m break to prevent cognitive burnout
    pub fn compute_pacing(energy: FatigueLevel, available_hours: f32) -> (u32, u32) {
        match energy {
            FatigueLevel::High => {
                let cap = if available_hours >= 4.0 { 40 } else { 35 };
                (cap, 5)
            }
            FatigueLevel::Normal => {
                let cap = if available_hours >= 3.0 { 30 } else { 25 };
                (cap, 5)
            }
            FatigueLevel::Fatigued => (18, 8),
        }
    }
}

/// An error or misconception logged during study sessions, Boss Battles, or quizzes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StudentMistakeRecord {
    pub id: i64,
    pub timestamp: String,
    pub subject: String,
    pub chapter: String,
    pub question_prompt: String,
    pub student_wrong_answer: String,
    pub correct_answer: String,
    pub misconception_category: String, // e.g. "Sign Convention", "Unit Conversion", "Formula Error", "Algebra"
    pub review_count: u32,
    pub is_resolved: bool,
}

/// 5-Box Leitner Spaced Repetition intervals according to the Ebbinghaus forgetting curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LeitnerBox {
    Box1 = 1, // 1 day interval
    Box2 = 2, // 3 days interval
    Box3 = 3, // 7 days interval
    Box4 = 4, // 14 days interval
    Box5 = 5, // 30 days interval (Mastered)
}

impl LeitnerBox {
    pub fn from_u8(val: u8) -> Self {
        match val {
            2 => Self::Box2,
            3 => Self::Box3,
            4 => Self::Box4,
            5 => Self::Box5,
            _ => Self::Box1,
        }
    }

    pub fn interval_days(self) -> i64 {
        match self {
            Self::Box1 => 1,
            Self::Box2 => 3,
            Self::Box3 => 7,
            Self::Box4 => 14,
            Self::Box5 => 30,
        }
    }

    pub fn advance(self) -> Self {
        match self {
            Self::Box1 => Self::Box2,
            Self::Box2 => Self::Box3,
            Self::Box3 => Self::Box4,
            Self::Box4 | Self::Box5 => Self::Box5,
        }
    }

    pub fn demote_on_failure() -> Self {
        Self::Box1
    }
}

/// A spaced repetition flashcard tracked across Leitner 5-box intervals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeitnerCard {
    pub id: String,
    pub front_prompt: String,
    pub back_solution: String,
    pub subject: String,
    pub current_box: u8,
    pub last_reviewed_at: String,
    pub next_review_due: String,
    pub consecutive_successes: u32,
    pub total_reviews: u32,
}

/// Centralized Personalization Engine managing SQLite persistence.
pub struct PersonalizationEngine {
    db_path: PathBuf,
}

impl Default for PersonalizationEngine {
    fn default() -> Self {
        Self::default_engine().unwrap_or_else(|_| Self {
            db_path: PathBuf::from("mindset_os.db"),
        })
    }
}

impl PersonalizationEngine {
    /// Opens or connects to the database at the specified path and initializes tables.
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        let engine = Self { db_path };
        engine.init_schema()?;
        Ok(engine)
    }

    /// Default engine connecting to standard MindSet OS SQLite database path.
    pub fn default_engine() -> Result<Self, String> {
        let base_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MindSetOS");
        let _ = fs::create_dir_all(&base_dir);
        let db_path = base_dir.join("mindset_os.db");
        Self::new(db_path)
    }

    /// Returns the database path.
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Opens a connection with WAL mode and 2MB cache cap.
    fn open_connection(&self) -> Result<Connection, String> {
        let conn = Connection::open(&self.db_path).map_err(|e| e.to_string())?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -2000;
             PRAGMA temp_store = MEMORY;",
        )
        .map_err(|e| e.to_string())?;
        Ok(conn)
    }

    /// Initializes schema for daily checkins, mistake logs, and spaced repetition cards.
    pub fn init_schema(&self) -> Result<(), String> {
        let conn = self.open_connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS daily_checkins (
                date TEXT PRIMARY KEY,
                energy_level TEXT NOT NULL,
                available_hours_today REAL NOT NULL,
                situation_note TEXT NOT NULL,
                recommended_capsule_minutes INTEGER NOT NULL DEFAULT 30,
                recommended_break_minutes INTEGER NOT NULL DEFAULT 5
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
            );",
        )
        .map_err(|e| format!("Failed to init personalization schema: {}", e))?;
        Ok(())
    }

    // ─── 1. Fatigue Check-in Methods ──────────────────────────────────────────

    pub fn record_fatigue_checkin(&self, checkin: &DailyFatigueCheckin) -> Result<(), String> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO daily_checkins
             (date, energy_level, available_hours_today, situation_note, recommended_capsule_minutes, recommended_break_minutes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                checkin.date,
                checkin.energy_level.as_str(),
                checkin.available_hours_today,
                checkin.situation_note,
                checkin.recommended_capsule_minutes,
                checkin.recommended_break_minutes,
            ],
        )
        .map_err(|e| format!("Failed to save fatigue checkin: {}", e))?;
        Ok(())
    }

    pub fn get_fatigue_checkin(&self, date: &str) -> Result<Option<DailyFatigueCheckin>, String> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(
                "SELECT date, energy_level, available_hours_today, situation_note,
                        recommended_capsule_minutes, recommended_break_minutes
                 FROM daily_checkins WHERE date = ?1",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query(params![date]).map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let date: String = row.get(0).map_err(|e| e.to_string())?;
            let energy_str: String = row.get(1).map_err(|e| e.to_string())?;
            let available_hours_today: f32 = row.get(2).map_err(|e| e.to_string())?;
            let situation_note: String = row.get(3).map_err(|e| e.to_string())?;
            let recommended_capsule_minutes: u32 = row.get(4).unwrap_or(30);
            let recommended_break_minutes: u32 = row.get(5).unwrap_or(5);

            Ok(Some(DailyFatigueCheckin {
                date,
                energy_level: FatigueLevel::from_str_loose(&energy_str),
                available_hours_today,
                situation_note,
                recommended_capsule_minutes,
                recommended_break_minutes,
            }))
        } else {
            Ok(None)
        }
    }

    // ─── 2. Student Mistake & Misconception Tracker ───────────────────────────

    pub fn record_mistake(
        &self,
        subject: &str,
        chapter: &str,
        question_prompt: &str,
        wrong_answer: &str,
        correct_answer: &str,
        misconception_category: &str,
    ) -> Result<i64, String> {
        let conn = self.open_connection()?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO student_mistake_log
             (timestamp, subject, chapter, question_prompt, student_wrong_answer, correct_answer, misconception_category, review_count, is_resolved)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 0)",
            params![
                now,
                subject,
                chapter,
                question_prompt,
                wrong_answer,
                correct_answer,
                misconception_category,
            ],
        )
        .map_err(|e| format!("Failed to record student mistake: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(id)
    }

    pub fn get_unresolved_mistakes(
        &self,
        subject: Option<&str>,
    ) -> Result<Vec<StudentMistakeRecord>, String> {
        let conn = self.open_connection()?;
        let sql = match subject {
            Some(_) => {
                "SELECT id, timestamp, subject, chapter, question_prompt, student_wrong_answer, correct_answer, misconception_category, review_count, is_resolved
                 FROM student_mistake_log WHERE is_resolved = 0 AND subject = ?1 ORDER BY id DESC"
            }
            None => {
                "SELECT id, timestamp, subject, chapter, question_prompt, student_wrong_answer, correct_answer, misconception_category, review_count, is_resolved
                 FROM student_mistake_log WHERE is_resolved = 0 ORDER BY id DESC"
            }
        };

        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let rows = if let Some(subj) = subject {
            stmt.query_map(params![subj], Self::map_mistake_row)
        } else {
            stmt.query_map([], Self::map_mistake_row)
        }
        .map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for item in rows.flatten() {
            list.push(item);
        }
        Ok(list)
    }

    pub fn resolve_mistake(&self, mistake_id: i64) -> Result<(), String> {
        let conn = self.open_connection()?;
        conn.execute(
            "UPDATE student_mistake_log SET is_resolved = 1, review_count = review_count + 1 WHERE id = ?1",
            params![mistake_id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn map_mistake_row(row: &rusqlite::Row) -> rusqlite::Result<StudentMistakeRecord> {
        Ok(StudentMistakeRecord {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            subject: row.get(2)?,
            chapter: row.get(3)?,
            question_prompt: row.get(4)?,
            student_wrong_answer: row.get(5)?,
            correct_answer: row.get(6)?,
            misconception_category: row.get(7)?,
            review_count: row.get(8)?,
            is_resolved: row.get::<_, i64>(9)? != 0,
        })
    }

    // ─── 3. Leitner 5-Box Spaced Repetition Scheduler ────────────────────────

    pub fn add_leitner_card(
        &self,
        id: &str,
        front_prompt: &str,
        back_solution: &str,
        subject: &str,
    ) -> Result<LeitnerCard, String> {
        let conn = self.open_connection()?;
        let today = Local::now().format("%Y-%m-%d").to_string();

        let card = LeitnerCard {
            id: id.to_string(),
            front_prompt: front_prompt.to_string(),
            back_solution: back_solution.to_string(),
            subject: subject.to_string(),
            current_box: 1,
            last_reviewed_at: today.clone(),
            next_review_due: today,
            consecutive_successes: 0,
            total_reviews: 0,
        };

        conn.execute(
            "INSERT OR REPLACE INTO leitner_cards
             (id, front_prompt, back_solution, subject, current_box, last_reviewed_at, next_review_due, consecutive_successes, total_reviews)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                card.id,
                card.front_prompt,
                card.back_solution,
                card.subject,
                card.current_box,
                card.last_reviewed_at,
                card.next_review_due,
                card.consecutive_successes,
                card.total_reviews,
            ],
        )
        .map_err(|e| format!("Failed to add leitner card: {}", e))?;

        Ok(card)
    }

    pub fn record_card_review(
        &self,
        card_id: &str,
        success: bool,
        current_date_str: &str,
    ) -> Result<LeitnerCard, String> {
        let conn = self.open_connection()?;

        let mut stmt = conn
            .prepare(
                "SELECT id, front_prompt, back_solution, subject, current_box, last_reviewed_at, next_review_due, consecutive_successes, total_reviews
                 FROM leitner_cards WHERE id = ?1",
            )
            .map_err(|e| e.to_string())?;

        let mut card = stmt
            .query_row(params![card_id], |row| {
                Ok(LeitnerCard {
                    id: row.get(0)?,
                    front_prompt: row.get(1)?,
                    back_solution: row.get(2)?,
                    subject: row.get(3)?,
                    current_box: row.get(4)?,
                    last_reviewed_at: row.get(5)?,
                    next_review_due: row.get(6)?,
                    consecutive_successes: row.get(7)?,
                    total_reviews: row.get(8)?,
                })
            })
            .map_err(|e| format!("Card {} not found: {}", card_id, e))?;

        let base_date = NaiveDate::parse_from_str(current_date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());

        let box_enum = LeitnerBox::from_u8(card.current_box);
        let (new_box, next_days) = if success {
            let next_b = box_enum.advance();
            (next_b as u8, next_b.interval_days())
        } else {
            let reset_b = LeitnerBox::demote_on_failure();
            (reset_b as u8, reset_b.interval_days())
        };

        let next_date = base_date + chrono::Duration::days(next_days);

        card.current_box = new_box;
        card.last_reviewed_at = current_date_str.to_string();
        card.next_review_due = next_date.format("%Y-%m-%d").to_string();
        card.total_reviews += 1;
        if success {
            card.consecutive_successes += 1;
        } else {
            card.consecutive_successes = 0;
        }

        conn.execute(
            "UPDATE leitner_cards
             SET current_box = ?1, last_reviewed_at = ?2, next_review_due = ?3, consecutive_successes = ?4, total_reviews = ?5
             WHERE id = ?6",
            params![
                card.current_box,
                card.last_reviewed_at,
                card.next_review_due,
                card.consecutive_successes,
                card.total_reviews,
                card.id,
            ],
        )
        .map_err(|e| format!("Failed to update leitner card: {}", e))?;

        Ok(card)
    }

    pub fn get_due_leitner_cards(
        &self,
        current_date_str: &str,
    ) -> Result<Vec<LeitnerCard>, String> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, front_prompt, back_solution, subject, current_box, last_reviewed_at, next_review_due, consecutive_successes, total_reviews
                 FROM leitner_cards WHERE next_review_due <= ?1 ORDER BY current_box ASC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![current_date_str], |row| {
                Ok(LeitnerCard {
                    id: row.get(0)?,
                    front_prompt: row.get(1)?,
                    back_solution: row.get(2)?,
                    subject: row.get(3)?,
                    current_box: row.get(4)?,
                    last_reviewed_at: row.get(5)?,
                    next_review_due: row.get(6)?,
                    consecutive_successes: row.get(7)?,
                    total_reviews: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for c in rows.flatten() {
            list.push(c);
        }
        Ok(list)
    }

    /// Synthesizes due Leitner reviews and unresolved misconceptions into morning Memory Anchor study items.
    pub fn generate_morning_memory_anchor(
        &self,
        current_date: &str,
    ) -> Result<Vec<String>, String> {
        let mut anchors = Vec::new();

        // 1. Unresolved misconceptions
        let mistakes = self.get_unresolved_mistakes(None)?;
        for m in mistakes.into_iter().take(3) {
            anchors.push(format!(
                "[Misconception Review: {}] {} -> Key fix: {}",
                m.chapter, m.question_prompt, m.correct_answer
            ));
        }

        // 2. Due Leitner cards
        let due_cards = self.get_due_leitner_cards(current_date)?;
        for c in due_cards.into_iter().take(4) {
            anchors.push(format!(
                "[Leitner Box {} Review • {}] {} => {}",
                c.current_box, c.subject, c.front_prompt, c.back_solution
            ));
        }

        if anchors.is_empty() {
            anchors.push(
                "All misconceptions resolved! Standard active recall session queued.".to_string(),
            );
        }

        Ok(anchors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fatigue_pacing_computation() {
        let (cap_high, brk_high) = DailyFatigueCheckin::compute_pacing(FatigueLevel::High, 4.5);
        assert_eq!(cap_high, 40);
        assert_eq!(brk_high, 5);

        let (cap_norm, brk_norm) = DailyFatigueCheckin::compute_pacing(FatigueLevel::Normal, 3.0);
        assert_eq!(cap_norm, 30);
        assert_eq!(brk_norm, 5);

        let (cap_fatigued, brk_fatigued) =
            DailyFatigueCheckin::compute_pacing(FatigueLevel::Fatigued, 2.0);
        assert_eq!(cap_fatigued, 18);
        assert_eq!(brk_fatigued, 8);
    }

    #[test]
    fn test_personalization_engine_crud() {
        let temp_db = std::env::temp_dir().join(format!(
            "test_mindset_os_{}.db",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let _ = fs::remove_file(&temp_db);

        let engine =
            PersonalizationEngine::new(temp_db.clone()).expect("Failed to initialize engine");

        // 1. Checkin test
        let checkin = DailyFatigueCheckin {
            date: "2026-11-24".to_string(),
            energy_level: FatigueLevel::Fatigued,
            available_hours_today: 2.5,
            situation_note: "Post-exam recovery day".to_string(),
            recommended_capsule_minutes: 18,
            recommended_break_minutes: 8,
        };
        engine
            .record_fatigue_checkin(&checkin)
            .expect("Failed to save checkin");
        let fetched = engine
            .get_fatigue_checkin("2026-11-24")
            .expect("Query failed");
        assert!(fetched.is_some());
        let f = fetched.unwrap();
        assert_eq!(f.energy_level, FatigueLevel::Fatigued);
        assert_eq!(f.recommended_capsule_minutes, 18);

        // 2. Mistake log test
        let m_id = engine
            .record_mistake(
                "Physics",
                "Wave Optics",
                "Young's double slit fringe width relation to slit separation d",
                "beta is directly proportional to d",
                "beta is inversely proportional to d (beta = lambda*D / d)",
                "Inverse Proportionality Misconception",
            )
            .expect("Record mistake failed");
        assert!(m_id > 0);

        let unresolved = engine
            .get_unresolved_mistakes(Some("Physics"))
            .expect("Fetch mistakes failed");
        assert_eq!(unresolved.len(), 1);
        assert_eq!(
            unresolved[0].misconception_category,
            "Inverse Proportionality Misconception"
        );

        engine.resolve_mistake(m_id).expect("Resolve failed");
        let unresolved_after = engine
            .get_unresolved_mistakes(Some("Physics"))
            .expect("Fetch mistakes failed");
        assert!(unresolved_after.is_empty());

        // 3. Leitner 5-box test
        let card = engine
            .add_leitner_card(
                "card-wave-1",
                "Young's Double Slit fringe width formula",
                "beta = lambda * D / d",
                "Physics",
            )
            .expect("Add card failed");
        assert_eq!(card.current_box, 1);

        // Review success -> advances to Box 2 (+3 days)
        let promoted = engine
            .record_card_review("card-wave-1", true, "2026-11-24")
            .expect("Review failed");
        assert_eq!(promoted.current_box, 2);
        assert_eq!(promoted.next_review_due, "2026-11-27");
        assert_eq!(promoted.consecutive_successes, 1);

        // Review failure -> demotes back to Box 1 (+1 day)
        let demoted = engine
            .record_card_review("card-wave-1", false, "2026-11-27")
            .expect("Review failure failed");
        assert_eq!(demoted.current_box, 1);
        assert_eq!(demoted.next_review_due, "2026-11-28");
        assert_eq!(demoted.consecutive_successes, 0);

        // Test Morning Memory Anchor generation
        let anchors = engine
            .generate_morning_memory_anchor("2026-11-28")
            .expect("Anchor gen failed");
        assert!(!anchors.is_empty());
        assert!(anchors[0].contains("Leitner Box 1 Review"));

        let _ = fs::remove_file(&temp_db);
    }
}
