//! MindSet OS Multi-Modal Biometric Auth & Smart Lock Engine
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod credentials;
pub mod face_unlock;
pub mod fingerprint;
pub mod session;

pub use credentials::{CredentialsError, CredentialsManager};
pub use face_unlock::{BiometricMatchResult, FaceAuthError, FaceUnlockEngine};
pub use fingerprint::{FingerprintEngine, FingerprintError, FingerprintResult};
pub use session::{AuthSession, AuthStatus, SessionManager};

use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Credentials error: {0}")]
    Credentials(#[from] CredentialsError),
    #[error("Face biometric error: {0}")]
    Face(#[from] FaceAuthError),
    #[error("Fingerprint error: {0}")]
    Fingerprint(#[from] FingerprintError),
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Session locked or unauthorized")]
    Unauthorized,
}

pub struct AuthEngine {
    pub session: Arc<Mutex<SessionManager>>,
}

impl AuthEngine {
    pub fn new() -> Self {
        Self {
            session: Arc::new(Mutex::new(SessionManager::new())),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.session.lock().unwrap().is_locked()
    }

    pub fn is_privacy_blurred(&self) -> bool {
        self.session.lock().unwrap().is_privacy_blurred()
    }

    pub fn is_uac_elevated(&self) -> bool {
        self.session.lock().unwrap().is_uac_elevated()
    }

    pub fn lock(&self) {
        self.session.lock().unwrap().lock();
    }

    pub fn toggle_privacy_blur(&self) -> bool {
        self.session.lock().unwrap().toggle_privacy_blur()
    }

    pub fn set_privacy_blur(&self, blurred: bool) {
        self.session.lock().unwrap().set_privacy_blur(blurred);
    }

    pub fn get_status(&self) -> AuthStatus {
        self.session.lock().unwrap().get_status()
    }

    /// Unlocks using 6-digit PIN verified via bcrypt
    pub fn unlock_with_pin(
        &self,
        conn: &Connection,
        user_id: &str,
        pin: &str,
    ) -> Result<AuthSession, AuthError> {
        CredentialsManager::ensure_default_credentials(conn, user_id)?;
        CredentialsManager::verify_pin(conn, user_id, pin)?;

        let session = SessionManager::create_session(user_id, "pin");
        self.session.lock().unwrap().unlock(session.clone());

        info!("User '{}' unlocked via PIN", user_id);
        Ok(session)
    }

    /// Unlocks using password verified via bcrypt (cost 12)
    pub fn unlock_with_password(
        &self,
        conn: &Connection,
        user_id: &str,
        password: &str,
    ) -> Result<AuthSession, AuthError> {
        CredentialsManager::ensure_default_credentials(conn, user_id)?;
        CredentialsManager::verify_password(conn, user_id, password)?;

        let session = SessionManager::create_session(user_id, "password");
        self.session.lock().unwrap().unlock(session.clone());

        info!("User '{}' unlocked via password", user_id);
        Ok(session)
    }

    /// Unlocks using transient facial verification with <300ms ephemeral lifecycle
    pub fn unlock_with_face(
        &self,
        conn: &Connection,
        user_id: &str,
        embedding: &[f32],
    ) -> Result<BiometricMatchResult, AuthError> {
        CredentialsManager::ensure_default_credentials(conn, user_id)?;
        let match_result = FaceUnlockEngine::verify_face(conn, user_id, embedding)?;

        let session = SessionManager::create_session(user_id, "face_biometric");
        self.session.lock().unwrap().unlock(session);

        info!("User '{}' unlocked via facial biometrics", user_id);
        Ok(match_result)
    }

    /// Unlocks using hardware fingerprint touch
    pub fn unlock_with_fingerprint(
        &self,
        conn: &Connection,
        user_id: &str,
    ) -> Result<FingerprintResult, AuthError> {
        CredentialsManager::ensure_default_credentials(conn, user_id)?;
        let result = FingerprintEngine::verify_fingerprint(conn, user_id)?;

        if result.success {
            let session = SessionManager::create_session(user_id, "fingerprint");
            self.session.lock().unwrap().unlock(session);
            info!("User '{}' unlocked via fingerprint", user_id);
        }

        Ok(result)
    }

    /// Evaluates credentials to grant temporary 3-minute UAC elevation privilege
    pub fn request_uac_elevation(
        &self,
        conn: &Connection,
        user_id: &str,
        pin_or_password: &str,
        is_pin: bool,
    ) -> Result<String, AuthError> {
        CredentialsManager::ensure_default_credentials(conn, user_id)?;

        if is_pin {
            CredentialsManager::verify_pin(conn, user_id, pin_or_password)?;
        } else {
            CredentialsManager::verify_password(conn, user_id, pin_or_password)?;
        }

        let token = self.session.lock().unwrap().elevate_uac();
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("In-memory SQLite");
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS auth_credentials (
                user_id TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL,
                pin_hash TEXT,
                face_embedding_blob BLOB,
                fingerprint_enrolled INTEGER NOT NULL DEFAULT 0,
                auto_lock_minutes INTEGER NOT NULL DEFAULT 5,
                failed_attempts INTEGER NOT NULL DEFAULT 0,
                locked_until TEXT
            );
            "#,
        )
        .expect("Create test schema");
        conn
    }

    #[test]
    fn test_credentials_verification_and_lock_lifecycle() {
        let conn = setup_test_conn();
        let engine = AuthEngine::new();

        assert!(engine.is_locked());
        assert!(!engine.is_privacy_blurred());
        assert!(!engine.is_uac_elevated());

        // Test wrong PIN
        let bad_pin = engine.unlock_with_pin(&conn, "default_student", "999999");
        assert!(bad_pin.is_err());
        assert!(engine.is_locked());

        // Test correct default PIN ("123456")
        let unlock_res = engine.unlock_with_pin(&conn, "default_student", "123456");
        assert!(unlock_res.is_ok());
        assert!(!engine.is_locked());

        // Test manual lock
        engine.lock();
        assert!(engine.is_locked());

        // Test password unlock ("MindSet2026!")
        let pwd_res = engine.unlock_with_password(&conn, "default_student", "MindSet2026!");
        assert!(pwd_res.is_ok());
        assert!(!engine.is_locked());
    }

    #[test]
    fn test_brute_force_lockout_after_5_attempts() {
        let conn = setup_test_conn();
        let engine = AuthEngine::new();

        CredentialsManager::ensure_default_credentials(&conn, "default_student").unwrap();

        // 4 failed attempts should fail with InvalidCredentials
        for _ in 0..4 {
            let res = engine.unlock_with_pin(&conn, "default_student", "000000");
            assert!(res.is_err());
        }

        // 5th failed attempt triggers lockout
        let fifth_attempt = engine.unlock_with_pin(&conn, "default_student", "000000");
        assert!(fifth_attempt.is_err());

        // 6th attempt should be blocked by AccountLocked
        let locked_attempt = engine.unlock_with_pin(&conn, "default_student", "123456");
        match locked_attempt {
            Err(AuthError::Credentials(CredentialsError::AccountLocked(secs))) => {
                assert!(secs > 0);
            }
            other => panic!("Expected AccountLocked error, got: {:?}", other),
        }
    }

    #[test]
    fn test_privacy_blur_and_uac_elevation() {
        let conn = setup_test_conn();
        let engine = AuthEngine::new();

        // Privacy Blur toggle
        assert!(!engine.is_privacy_blurred());
        let blurred = engine.toggle_privacy_blur();
        assert!(blurred);
        assert!(engine.is_privacy_blurred());
        engine.set_privacy_blur(false);
        assert!(!engine.is_privacy_blurred());

        // UAC elevation with correct PIN
        let token = engine
            .request_uac_elevation(&conn, "default_student", "123456", true)
            .expect("UAC elevation should succeed");
        assert!(token.starts_with("uac-"));
        assert!(engine.is_uac_elevated());

        // UAC elevation with wrong PIN should fail
        let failed_uac = engine.request_uac_elevation(&conn, "default_student", "wrong_pin", true);
        assert!(failed_uac.is_err());
    }

    #[test]
    fn test_face_biometrics_and_fingerprint_fallback() {
        let conn = setup_test_conn();
        let engine = AuthEngine::new();

        // Face enrollment and verification
        let sample_embedding = vec![0.1f32; 128];
        let enroll_res = FaceUnlockEngine::enroll_face(&conn, "default_student", &sample_embedding);
        assert!(enroll_res.is_ok());

        let verify_res = engine.unlock_with_face(&conn, "default_student", &sample_embedding);
        assert!(verify_res.is_ok());
        let match_info = verify_res.unwrap();
        assert!(match_info.success);
        assert!(match_info.match_score >= 0.82);

        // Fingerprint graceful fallback test (when hardware absent)
        let fp_res = engine
            .unlock_with_fingerprint(&conn, "default_student")
            .unwrap();
        // Should return a response without panicking
        assert!(!fp_res.message.is_empty());
    }
}
