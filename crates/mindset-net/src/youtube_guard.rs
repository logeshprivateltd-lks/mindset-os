//! MindSet OS — YouTube Study Screen-Time Timer & Anti-Shorts Guard
//! Enforces daily quota caps, active video countdown timers, permanent `/shorts/` blocking,
//! and SQLite WAL quota state persistence.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

/// Default daily YouTube study allowance: 45 minutes (2,700 seconds).
pub const DEFAULT_YOUTUBE_LIMIT_SECS: u32 = 45 * 60;

/// Standard screen-time limit presets in minutes.
pub const YOUTUBE_PRESETS_MINUTES: [u32; 4] = [15, 30, 45, 60];

/// Error message returned when YouTube Shorts are intercepted.
pub const YOUTUBE_SHORTS_BLOCKED_MSG: &str =
    "YouTube Shorts are permanently blocked in MindSet OS.";

/// Error message returned when the daily screen-time allocation has expired.
pub const YOUTUBE_DAILY_LIMIT_EXHAUSTED_MSG: &str = "Daily YouTube Screen-Time Limit Reached.";

/// Coordinator for YouTube screen-time countdown and study filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeGuard {
    pub daily_limit_secs: u32,
    pub used_today_secs: u32,
    pub timer_running: bool,
    pub shorts_blocked_count: u32,
}

impl Default for YoutubeGuard {
    fn default() -> Self {
        Self {
            daily_limit_secs: DEFAULT_YOUTUBE_LIMIT_SECS,
            used_today_secs: 0,
            timer_running: false,
            shorts_blocked_count: 0,
        }
    }
}

impl YoutubeGuard {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates whether a URL is a YouTube domain.
    pub fn is_youtube_url(url: &str) -> bool {
        let lower = url.to_ascii_lowercase();
        lower.contains("youtube.com") || lower.contains("youtu.be")
    }

    /// Checks if a navigation URL is allowed under YouTube screen-time and anti-shorts rules.
    /// Returns `Err` if Shorts are detected or if the daily quota is exhausted.
    pub fn check_youtube_url(&mut self, url: &str) -> Result<(), String> {
        let lower = url.trim().to_ascii_lowercase();

        if !Self::is_youtube_url(&lower) {
            return Ok(());
        }

        // 1. YouTube Shorts are permanently blocked in MindSet OS
        if lower.contains("youtube.com/shorts")
            || lower.contains("youtu.be/shorts")
            || lower.contains("/shorts/")
            || lower.ends_with("/shorts")
        {
            self.shorts_blocked_count = self.shorts_blocked_count.saturating_add(1);
            return Err(YOUTUBE_SHORTS_BLOCKED_MSG.to_string());
        }

        // 2. Check if the daily study screen-time quota is exhausted
        if self.used_today_secs >= self.daily_limit_secs {
            return Err(YOUTUBE_DAILY_LIMIT_EXHAUSTED_MSG.to_string());
        }

        Ok(())
    }

    /// Advances the timer by one second if `timer_running` is active.
    /// Returns `true` the instant the daily limit is exhausted.
    pub fn tick_second(&mut self) -> bool {
        if !self.timer_running {
            return false;
        }

        if self.used_today_secs < self.daily_limit_secs {
            self.used_today_secs = self.used_today_secs.saturating_add(1);
            if self.used_today_secs >= self.daily_limit_secs {
                self.timer_running = false;
                return true; // Limit reached on this tick
            }
        } else {
            self.timer_running = false;
        }

        false
    }

    /// Starts or resumes the active video playback timer.
    pub fn start_timer(&mut self) {
        if self.used_today_secs < self.daily_limit_secs {
            self.timer_running = true;
        }
    }

    /// Pauses the video screen-time timer.
    pub fn pause_timer(&mut self) {
        self.timer_running = false;
    }

    /// Returns remaining seconds for today.
    pub fn remaining_seconds(&self) -> u32 {
        self.daily_limit_secs.saturating_sub(self.used_today_secs)
    }

    /// Formats remaining study video time as `MM:SS` (or `HH:MM:SS` if > 60 minutes).
    pub fn remaining_formatted(&self) -> String {
        let rem = self.remaining_seconds();
        let hours = rem / 3600;
        let mins = (rem % 3600) / 60;
        let secs = rem % 60;

        if hours > 0 {
            format!("{hours:02}:{mins:02}:{secs:02}")
        } else {
            format!("{mins:02}:{secs:02}")
        }
    }

    /// Applies one of the standard preset durations in minutes (e.g. 15, 30, 45, 60).
    pub fn set_preset(&mut self, minutes: u32) {
        self.daily_limit_secs = minutes.saturating_mul(60);
    }

    /// Resets daily timer consumption (e.g. at midnight).
    pub fn reset_daily_timer(&mut self) {
        self.used_today_secs = 0;
        self.timer_running = false;
    }

    /// Initializes the `youtube_quota` table in SQLite.
    pub fn init_db(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS youtube_quota (
                date TEXT PRIMARY KEY,
                daily_limit_secs INTEGER NOT NULL,
                used_today_secs INTEGER NOT NULL,
                shorts_blocked INTEGER NOT NULL
            );",
            [],
        )?;
        Ok(())
    }

    /// Persists current quota and timer state for the given ISO-8601 date (e.g. "2026-11-15").
    pub fn save_to_db(&self, conn: &Connection, date_str: &str) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO youtube_quota (date, daily_limit_secs, used_today_secs, shorts_blocked)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(date) DO UPDATE SET
                daily_limit_secs = excluded.daily_limit_secs,
                used_today_secs = excluded.used_today_secs,
                shorts_blocked = excluded.shorts_blocked;",
            params![
                date_str,
                self.daily_limit_secs as i64,
                self.used_today_secs as i64,
                self.shorts_blocked_count as i64
            ],
        )?;
        Ok(())
    }

    /// Loads persisted quota state for the given date. Returns `true` if found, `false` otherwise.
    pub fn load_from_db(&mut self, conn: &Connection, date_str: &str) -> SqlResult<bool> {
        let mut stmt = conn.prepare(
            "SELECT daily_limit_secs, used_today_secs, shorts_blocked 
             FROM youtube_quota 
             WHERE date = ?1;",
        )?;

        let mut rows = stmt.query(params![date_str])?;
        if let Some(row) = rows.next()? {
            let limit: i64 = row.get(0)?;
            let used: i64 = row.get(1)?;
            let shorts: i64 = row.get(2)?;

            self.daily_limit_secs = limit.max(0) as u32;
            self.used_today_secs = used.max(0) as u32;
            self.shorts_blocked_count = shorts.max(0) as u32;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_guard_shorts_blocking() {
        let mut guard = YoutubeGuard::new();

        // Standard educational video is allowed when quota remains
        assert!(guard
            .check_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            .is_ok());
        assert!(guard
            .check_youtube_url("https://youtu.be/dQw4w9WgXcQ")
            .is_ok());

        // Shorts URLs are blocked immediately
        assert_eq!(
            guard.check_youtube_url("https://www.youtube.com/shorts/abcd1234"),
            Err(YOUTUBE_SHORTS_BLOCKED_MSG.to_string())
        );
        assert_eq!(
            guard.check_youtube_url("https://youtu.be/shorts/abcd1234"),
            Err(YOUTUBE_SHORTS_BLOCKED_MSG.to_string())
        );
        assert_eq!(guard.shorts_blocked_count, 2);
    }

    #[test]
    fn test_youtube_screen_time_quota_exhaustion() {
        let mut guard = YoutubeGuard::new();
        guard.daily_limit_secs = 5; // 5 seconds test quota
        guard.start_timer();

        assert_eq!(guard.remaining_formatted(), "00:05");

        // Tick 4 seconds
        for _ in 0..4 {
            assert!(!guard.tick_second());
        }
        assert_eq!(guard.used_today_secs, 4);
        assert!(guard.timer_running);
        assert!(guard
            .check_youtube_url("https://youtube.com/watch?v=123")
            .is_ok());

        // 5th second ticks -> limit exhausted
        let exhausted = guard.tick_second();
        assert!(exhausted);
        assert_eq!(guard.used_today_secs, 5);
        assert!(!guard.timer_running);

        // Subsequent navigation to YouTube is blocked
        assert_eq!(
            guard.check_youtube_url("https://youtube.com/watch?v=123"),
            Err(YOUTUBE_DAILY_LIMIT_EXHAUSTED_MSG.to_string())
        );
    }

    #[test]
    fn test_youtube_guard_sqlite_persistence() {
        let conn = Connection::open_in_memory().unwrap();
        YoutubeGuard::init_db(&conn).unwrap();

        let mut guard = YoutubeGuard::new();
        guard.daily_limit_secs = 1800; // 30m
        guard.used_today_secs = 600; // 10m used
        guard.shorts_blocked_count = 7;

        guard.save_to_db(&conn, "2026-11-15").unwrap();

        // Restore into fresh guard instance
        let mut fresh = YoutubeGuard::new();
        let loaded = fresh.load_from_db(&conn, "2026-11-15").unwrap();
        assert!(loaded);
        assert_eq!(fresh.daily_limit_secs, 1800);
        assert_eq!(fresh.used_today_secs, 600);
        assert_eq!(fresh.shorts_blocked_count, 7);
        assert_eq!(fresh.remaining_seconds(), 1200);
        assert_eq!(fresh.remaining_formatted(), "20:00");
    }
}
