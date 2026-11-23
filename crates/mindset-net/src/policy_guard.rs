//! MindSet OS — Policy Guard & Zero-Incognito Enforcement Subsystem
//! Enforces permanent prevention of private browsing, CLI argument sanitation,
//! internal URL scheme blocking, and SQLite audit logging in local_core.sqlite.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

/// Reason message returned when incognito or private browsing shortcuts are pressed.
pub const INCOGNITO_BLOCKED_MSG: &str =
    "Incognito / Private Mode is permanently disabled by MindSet Integrity Policy.";

/// Flag prefixes and flags stripped during browser launch sanitation.
const FORBIDDEN_CLI_FLAGS: &[&str] = &[
    "--incognito",
    "-incognito",
    "--inprivate",
    "-inprivate",
    "--guest",
    "-guest",
    "--bwsi",
    "--disable-extensions",
    "--temp-profile",
    "--profile-directory=guest",
];

/// Prohibited internal browser URLs that would allow bypassing policies or disabling extensions.
const RESTRICTED_INTERNAL_SCHEMES: &[&str] = &[
    "chrome://extensions",
    "edge://extensions",
    "brave://extensions",
    "opera://extensions",
    "vivaldi://extensions",
    "chrome://settings",
    "edge://settings",
    "chrome://flags",
    "edge://flags",
    "about:blank#blocked",
    "about:config",
    "about:addons",
];

/// A persistent audit log entry of browser navigation and security events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityLogEntry {
    pub id: i64,
    pub timestamp: i64,
    pub url: String,
    pub category: String,
    pub action: String,
}

/// Core policy guard and sanitizer.
#[derive(Debug, Clone, Default)]
pub struct PolicyGuard {
    pub audit_logging_enabled: bool,
}

impl PolicyGuard {
    pub fn new() -> Self {
        Self {
            audit_logging_enabled: true,
        }
    }

    /// Evaluates keyboard shortcuts and rejects incognito / private mode triggers.
    /// Blocks Ctrl + Shift + N (Chrome, Edge, Brave, Opera) and Ctrl + Shift + P (Firefox, Safari).
    #[inline]
    pub fn is_shortcut_blocked(
        ctrl: bool,
        shift: bool,
        alt: bool,
        key: &str,
    ) -> Option<&'static str> {
        if ctrl && shift && !alt {
            let k = key.trim();
            if k.eq_ignore_ascii_case("n") || k.eq_ignore_ascii_case("p") {
                return Some(INCOGNITO_BLOCKED_MSG);
            }
        }
        None
    }

    /// Sanitizes browser process startup arguments, stripping any incognito, inprivate,
    /// guest-mode, or extension-disabling command-line flags.
    pub fn sanitize_browser_args(args: &[String]) -> Vec<String> {
        args.iter()
            .filter(|arg| {
                let lower = arg.to_ascii_lowercase();
                !FORBIDDEN_CLI_FLAGS
                    .iter()
                    .any(|flag| lower == *flag || lower.starts_with(&format!("{flag}=")))
            })
            .cloned()
            .collect()
    }

    /// Validates and normalizes navigation URL schemes.
    /// Blocks internal browser configuration pages and dangerous execution protocols,
    /// while normalizing standard domain queries to `https://`.
    pub fn validate_url_scheme(url: &str) -> Result<String, String> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err("Navigation URL cannot be empty.".to_string());
        }

        let lower = trimmed.to_ascii_lowercase();

        // 1. Check prohibited browser internals
        for &restricted in RESTRICTED_INTERNAL_SCHEMES {
            if lower.starts_with(restricted) {
                return Err(format!(
                    "Access to internal browser configuration '{restricted}' is restricted by MindSet Integrity Policy."
                ));
            }
        }

        // 2. Reject malicious scripting and local file schemes
        if lower.starts_with("javascript:")
            || lower.starts_with("data:")
            || lower.starts_with("vbscript:")
        {
            return Err(
                "Execution schemes (javascript:, data:, vbscript:) are strictly blocked."
                    .to_string(),
            );
        }

        // 3. Normalize HTTP / HTTPS schemes
        if lower.starts_with("http://") {
            // Automatically upgrade plain HTTP to secure HTTPS
            Ok(format!("https://{}", &trimmed[7..]))
        } else if lower.starts_with("https://") {
            Ok(trimmed.to_string())
        } else if lower.contains("://") {
            Err("Only secure HTTP and HTTPS protocols are permitted in MindSet OS.".to_string())
        } else {
            // Bare domain (e.g. "wikipedia.org") -> prepend https://
            Ok(format!("https://{trimmed}"))
        }
    }

    /// Initializes the `browser_integrity_log` table in SQLite.
    pub fn init_db(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS browser_integrity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                url TEXT NOT NULL,
                category TEXT NOT NULL,
                action TEXT NOT NULL
            );",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_browser_integrity_ts ON browser_integrity_log(timestamp);",
            [],
        )?;
        Ok(())
    }

    /// Records a navigation event into the SQLite audit table.
    pub fn log_navigation(
        conn: &Connection,
        url: &str,
        category: &str,
        action: &str,
    ) -> SqlResult<i64> {
        let now_ts = Utc::now().timestamp();
        conn.execute(
            "INSERT INTO browser_integrity_log (timestamp, url, category, action) VALUES (?1, ?2, ?3, ?4);",
            params![now_ts, url, category, action],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Retrieves the most recent navigation logs.
    pub fn fetch_recent_logs(conn: &Connection, limit: usize) -> SqlResult<Vec<IntegrityLogEntry>> {
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, url, category, action 
             FROM browser_integrity_log 
             ORDER BY id DESC 
             LIMIT ?1;",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(IntegrityLogEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                url: row.get(2)?,
                category: row.get(3)?,
                action: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    /// Counts total blocked distraction attempts logged in the database.
    pub fn count_blocked_logs(conn: &Connection) -> SqlResult<usize> {
        let mut stmt =
            conn.prepare("SELECT COUNT(*) FROM browser_integrity_log WHERE action = 'BLOCKED';")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_shortcut_blocked_incognito_combinations() {
        // Ctrl + Shift + N (Chrome, Edge, Brave, Opera)
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, true, false, "n"),
            Some(INCOGNITO_BLOCKED_MSG)
        );
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, true, false, "N"),
            Some(INCOGNITO_BLOCKED_MSG)
        );

        // Ctrl + Shift + P (Firefox, Safari)
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, true, false, "p"),
            Some(INCOGNITO_BLOCKED_MSG)
        );
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, true, false, "P"),
            Some(INCOGNITO_BLOCKED_MSG)
        );

        // Allowed shortcuts (Ctrl + N without Shift = new window, Ctrl + T = new tab, Alt + N)
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, false, false, "n"),
            None
        );
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, false, false, "t"),
            None
        );
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(false, true, true, "n"),
            None
        );
        assert_eq!(
            PolicyGuard::is_shortcut_blocked(true, true, true, "n"),
            None
        );
    }

    #[test]
    fn test_sanitize_browser_args_strips_forbidden_flags() {
        let raw_args = vec![
            "chrome.exe".to_string(),
            "--incognito".to_string(),
            "--remote-debugging-port=9222".to_string(),
            "--inprivate".to_string(),
            "--disable-extensions".to_string(),
            "--guest".to_string(),
            "--user-data-dir=C:\\Data".to_string(),
            "--bwsi".to_string(),
            "--temp-profile".to_string(),
        ];

        let clean = PolicyGuard::sanitize_browser_args(&raw_args);
        assert_eq!(clean.len(), 3);
        assert_eq!(clean[0], "chrome.exe");
        assert_eq!(clean[1], "--remote-debugging-port=9222");
        assert_eq!(clean[2], "--user-data-dir=C:\\Data");
    }

    #[test]
    fn test_validate_url_scheme_blocking_and_normalization() {
        // Prohibited internal configurations
        assert!(PolicyGuard::validate_url_scheme("chrome://extensions").is_err());
        assert!(PolicyGuard::validate_url_scheme("edge://extensions/details").is_err());
        assert!(PolicyGuard::validate_url_scheme("chrome://settings").is_err());
        assert!(PolicyGuard::validate_url_scheme("about:blank#blocked").is_err());
        assert!(PolicyGuard::validate_url_scheme("javascript:alert(1)").is_err());

        // Normalized allowed URLs
        let norm1 = PolicyGuard::validate_url_scheme("wikipedia.org").unwrap();
        assert_eq!(norm1, "https://wikipedia.org");

        let norm2 = PolicyGuard::validate_url_scheme("http://khanacademy.org").unwrap();
        assert_eq!(norm2, "https://khanacademy.org");

        let norm3 = PolicyGuard::validate_url_scheme("https://arxiv.org/abs/2301.0001").unwrap();
        assert_eq!(norm3, "https://arxiv.org/abs/2301.0001");
    }

    #[test]
    fn test_sqlite_activity_audit_log_lifecycle() {
        let conn = Connection::open_in_memory().unwrap();
        PolicyGuard::init_db(&conn).unwrap();

        PolicyGuard::log_navigation(&conn, "https://wikipedia.org", "Education", "ALLOWED")
            .unwrap();
        PolicyGuard::log_navigation(&conn, "https://netflix.com", "Entertainment", "BLOCKED")
            .unwrap();
        PolicyGuard::log_navigation(&conn, "https://instagram.com", "Social", "BLOCKED").unwrap();

        let logs = PolicyGuard::fetch_recent_logs(&conn, 10).unwrap();
        assert_eq!(logs.len(), 3);
        assert_eq!(logs[0].action, "BLOCKED"); // ordered by id DESC
        assert_eq!(logs[0].url, "https://instagram.com");

        let blocked_count = PolicyGuard::count_blocked_logs(&conn).unwrap();
        assert_eq!(blocked_count, 2);
    }
}
