//! MindSet OS Cryptographic Credentials Engine
//! Bcrypt (cost 12) password/PIN hashing, verification, and brute-force lockout guard
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection};
use thiserror::Error;
use tracing::{info, warn};

pub const BCRYPT_COST: u32 = 12;
pub const MAX_FAILED_ATTEMPTS: i32 = 5;
pub const LOCKOUT_DURATION_SECS: i64 = 30;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Bcrypt hashing/verification error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("Invalid password or PIN")]
    InvalidCredentials,
    #[error("User account '{0}' not found in auth_credentials")]
    UserNotFound(String),
    #[error("Account locked due to 5 consecutive failed attempts. Retry after {0} seconds")]
    AccountLocked(i64),
}

pub struct CredentialsManager;

impl CredentialsManager {
    /// Ensures default credentials exist in auth_credentials for initial setup
    pub fn ensure_default_credentials(
        conn: &Connection,
        user_id: &str,
    ) -> Result<(), CredentialsError> {
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM auth_credentials WHERE user_id = ?1)",
                params![user_id],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if !exists {
            // Default demo password: "MindSet2026!"
            // Default demo 6-digit PIN: "123456"
            let password_hash = bcrypt::hash("MindSet2026!", BCRYPT_COST)?;
            let pin_hash = bcrypt::hash("123456", BCRYPT_COST)?;

            conn.execute(
                r#"
                INSERT INTO auth_credentials (
                    user_id, password_hash, pin_hash, face_embedding_blob, 
                    fingerprint_enrolled, auto_lock_minutes, failed_attempts, locked_until
                ) VALUES (?1, ?2, ?3, NULL, 0, 5, 0, NULL)
                "#,
                params![user_id, password_hash, pin_hash],
            )?;

            info!("Seeded default credentials for user: {}", user_id);
        }

        Ok(())
    }

    /// Verifies password against stored bcrypt hash with rate limiting
    pub fn verify_password(
        conn: &Connection,
        user_id: &str,
        password: &str,
    ) -> Result<bool, CredentialsError> {
        Self::check_lockout(conn, user_id)?;

        let hash: String = match conn.query_row(
            "SELECT password_hash FROM auth_credentials WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ) {
            Ok(h) => h,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(CredentialsError::UserNotFound(user_id.to_string()))
            }
            Err(e) => return Err(CredentialsError::Sqlite(e)),
        };

        match bcrypt::verify(password, &hash) {
            Ok(true) => {
                Self::reset_failed_attempts(conn, user_id)?;
                info!("Password verification successful for user: {}", user_id);
                Ok(true)
            }
            Ok(false) => {
                Self::record_failed_attempt(conn, user_id)?;
                Err(CredentialsError::InvalidCredentials)
            }
            Err(e) => Err(CredentialsError::Bcrypt(e)),
        }
    }

    /// Verifies 6-digit PIN against stored bcrypt hash with rate limiting
    pub fn verify_pin(
        conn: &Connection,
        user_id: &str,
        pin: &str,
    ) -> Result<bool, CredentialsError> {
        Self::check_lockout(conn, user_id)?;

        let pin_hash: Option<String> = match conn.query_row(
            "SELECT pin_hash FROM auth_credentials WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ) {
            Ok(h) => h,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(CredentialsError::UserNotFound(user_id.to_string()))
            }
            Err(e) => return Err(CredentialsError::Sqlite(e)),
        };

        let stored_hash = pin_hash.ok_or(CredentialsError::InvalidCredentials)?;

        match bcrypt::verify(pin, &stored_hash) {
            Ok(true) => {
                Self::reset_failed_attempts(conn, user_id)?;
                info!("PIN verification successful for user: {}", user_id);
                Ok(true)
            }
            Ok(false) => {
                Self::record_failed_attempt(conn, user_id)?;
                Err(CredentialsError::InvalidCredentials)
            }
            Err(e) => Err(CredentialsError::Bcrypt(e)),
        }
    }

    /// Updates user password with bcrypt hash
    #[allow(dead_code)]
    pub fn set_password(
        conn: &Connection,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), CredentialsError> {
        let hash = bcrypt::hash(new_password, BCRYPT_COST)?;
        conn.execute(
            "UPDATE auth_credentials SET password_hash = ?1, failed_attempts = 0, locked_until = NULL WHERE user_id = ?2",
            params![hash, user_id],
        )?;
        info!("Updated password hash for user: {}", user_id);
        Ok(())
    }

    /// Updates user 6-digit PIN with bcrypt hash
    #[allow(dead_code)]
    pub fn set_pin(
        conn: &Connection,
        user_id: &str,
        new_pin: &str,
    ) -> Result<(), CredentialsError> {
        let hash = bcrypt::hash(new_pin, BCRYPT_COST)?;
        conn.execute(
            "UPDATE auth_credentials SET pin_hash = ?1 WHERE user_id = ?2",
            params![hash, user_id],
        )?;
        info!("Updated 6-digit PIN hash for user: {}", user_id);
        Ok(())
    }

    /// Checks if the account is currently locked out due to excessive failed attempts
    pub fn check_lockout(conn: &Connection, user_id: &str) -> Result<(), CredentialsError> {
        let locked_until_opt: Option<String> = conn
            .query_row(
                "SELECT locked_until FROM auth_credentials WHERE user_id = ?1",
                params![user_id],
                |row| row.get(0),
            )
            .unwrap_or(None);

        if let Some(locked_str) = locked_until_opt {
            if let Ok(locked_until) = DateTime::parse_from_rfc3339(&locked_str) {
                let now = Utc::now();
                if now < locked_until {
                    let remaining_secs = (locked_until.timestamp() - now.timestamp()).max(1);
                    warn!(
                        "Brute-force protection: user '{}' locked out for {} more seconds",
                        user_id, remaining_secs
                    );
                    return Err(CredentialsError::AccountLocked(remaining_secs));
                } else {
                    // Lockout expired, clear it
                    let _ = conn.execute(
                        "UPDATE auth_credentials SET locked_until = NULL, failed_attempts = 0 WHERE user_id = ?1",
                        params![user_id],
                    );
                }
            }
        }

        Ok(())
    }

    fn record_failed_attempt(conn: &Connection, user_id: &str) -> Result<(), CredentialsError> {
        let attempts: i32 = conn
            .query_row(
                "SELECT COALESCE(failed_attempts, 0) FROM auth_credentials WHERE user_id = ?1",
                params![user_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let new_attempts = attempts + 1;

        if new_attempts >= MAX_FAILED_ATTEMPTS {
            let lockout_until =
                (Utc::now() + Duration::seconds(LOCKOUT_DURATION_SECS)).to_rfc3339();
            conn.execute(
                "UPDATE auth_credentials SET failed_attempts = ?1, locked_until = ?2 WHERE user_id = ?3",
                params![new_attempts, lockout_until, user_id],
            )?;
            warn!(
                "Account '{}' locked out for {}s after {} failed attempts",
                user_id, LOCKOUT_DURATION_SECS, new_attempts
            );
        } else {
            conn.execute(
                "UPDATE auth_credentials SET failed_attempts = ?1 WHERE user_id = ?2",
                params![new_attempts, user_id],
            )?;
        }

        Ok(())
    }

    fn reset_failed_attempts(conn: &Connection, user_id: &str) -> Result<(), CredentialsError> {
        conn.execute(
            "UPDATE auth_credentials SET failed_attempts = 0, locked_until = NULL WHERE user_id = ?1",
            params![user_id],
        )?;
        Ok(())
    }
}
