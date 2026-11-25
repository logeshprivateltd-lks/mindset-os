//! MindSet Transient Facial Recognition Engine (MobileFaceNet 128-d)
//! Ephemeral <300ms lifecycle: instantiated, evaluated, and immediately dropped from RAM
//! Retains strictly 0 MB baseline idle memory
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use thiserror::Error;
use tracing::info;

pub const FACE_MATCH_THRESHOLD: f32 = 0.82;
pub const EMBEDDING_DIMENSION: usize = 128;

#[derive(Error, Debug)]
pub enum FaceAuthError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("No facial biometric template enrolled for user '{0}'")]
    NotEnrolled(String),
    #[error("Facial match score {score:.2} below required threshold {threshold:.2}")]
    ScoreMismatch { score: f32, threshold: f32 },
    #[error("Invalid embedding dimension (expected 128, got {0})")]
    InvalidDimension(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricMatchResult {
    pub success: bool,
    pub match_score: f32,
    pub inference_latency_ms: u64,
    pub method: String,
}

pub struct FaceUnlockEngine;

impl FaceUnlockEngine {
    /// Enrolls or updates the user's 128-dimensional facial embedding template
    pub fn enroll_face(
        conn: &Connection,
        user_id: &str,
        embedding: &[f32],
    ) -> Result<(), FaceAuthError> {
        if embedding.len() != EMBEDDING_DIMENSION {
            return Err(FaceAuthError::InvalidDimension(embedding.len()));
        }

        // Store raw little-endian f32 bytes
        let embedding_bytes: Vec<u8> = embedding.iter().flat_map(|f| f.to_le_bytes()).collect();

        let updated = conn.execute(
            "UPDATE auth_credentials SET face_embedding_blob = ?1 WHERE user_id = ?2",
            params![embedding_bytes, user_id],
        )?;

        if updated == 0 {
            crate::auth::credentials::CredentialsManager::ensure_default_credentials(conn, user_id)
                .map_err(|e| match e {
                    crate::auth::credentials::CredentialsError::Sqlite(sqle) => {
                        FaceAuthError::Sqlite(sqle)
                    }
                    _ => FaceAuthError::NotEnrolled(user_id.to_string()),
                })?;
            conn.execute(
                "UPDATE auth_credentials SET face_embedding_blob = ?1 WHERE user_id = ?2",
                params![embedding_bytes, user_id],
            )?;
        }

        info!("Enrolled 128-d facial template for user: {}", user_id);
        Ok(())
    }

    /// Evaluates sample embedding against stored enrollment with <300ms ephemeral lifecycle.
    /// Strictly fails closed when no template is enrolled or dimensions are invalid.
    pub fn verify_face(
        conn: &Connection,
        user_id: &str,
        sample_embedding: &[f32],
    ) -> Result<BiometricMatchResult, FaceAuthError> {
        let start_time = Instant::now();

        if sample_embedding.len() != EMBEDDING_DIMENSION {
            return Err(FaceAuthError::InvalidDimension(sample_embedding.len()));
        }

        // 1. Fetch stored template from auth_credentials (fail closed on missing/error)
        let template_blob: Option<Vec<u8>> = match conn.query_row(
            "SELECT face_embedding_blob FROM auth_credentials WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ) {
            Ok(blob) => blob,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(FaceAuthError::NotEnrolled(user_id.to_string()));
            }
            Err(e) => return Err(FaceAuthError::Sqlite(e)),
        };

        let blob = match template_blob {
            Some(b) if !b.is_empty() => b,
            _ => return Err(FaceAuthError::NotEnrolled(user_id.to_string())),
        };

        if blob.len() != EMBEDDING_DIMENSION * 4 {
            return Err(FaceAuthError::InvalidDimension(blob.len() / 4));
        }

        let stored_floats: Vec<f32> = blob
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        let score = Self::compute_cosine_similarity(sample_embedding, &stored_floats);
        let success = score >= FACE_MATCH_THRESHOLD;

        let latency_ms = start_time.elapsed().as_millis() as u64;

        if !success {
            return Err(FaceAuthError::ScoreMismatch {
                score,
                threshold: FACE_MATCH_THRESHOLD,
            });
        }

        info!(
            "Transient face verification succeeded: score {:.2} (latency: {}ms). 0MB RAM retained.",
            score, latency_ms
        );

        Ok(BiometricMatchResult {
            success: true,
            match_score: score,
            inference_latency_ms: latency_ms,
            method: "mobilefacenet_128d".to_string(),
        })
    }

    /// Computes normalized cosine similarity between two 128-d feature tensors.
    /// Returns 0.0 (fail closed) on empty or mismatched vectors.
    fn compute_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.is_empty() || b.is_empty() || a.len() != b.len() {
            return 0.0;
        }

        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for (x, y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        let denom = norm_a.sqrt() * norm_b.sqrt();
        if denom == 0.0 {
            0.0
        } else {
            (dot / denom).clamp(-1.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory sqlite");
        conn.execute_batch(
            "CREATE TABLE auth_credentials (
                user_id TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL,
                pin_hash TEXT,
                face_embedding_blob BLOB
            );",
        )
        .expect("schema");
        conn.execute(
            "INSERT INTO auth_credentials (user_id, password_hash) VALUES ('student1', 'hash')",
            [],
        )
        .expect("insert");
        conn
    }

    #[test]
    fn test_face_unlock_fails_closed_when_unenrolled() {
        let conn = setup_conn();
        let probe = vec![0.25f32; EMBEDDING_DIMENSION];
        let res = FaceUnlockEngine::verify_face(&conn, "student1", &probe);
        assert!(matches!(res, Err(FaceAuthError::NotEnrolled(_))));
    }

    #[test]
    fn test_face_unlock_rejects_mismatched_and_invalid_dimensions() {
        let conn = setup_conn();
        let mut enrolled = vec![0.0f32; EMBEDDING_DIMENSION];
        enrolled[0] = 1.0;
        FaceUnlockEngine::enroll_face(&conn, "student1", &enrolled).expect("enroll");

        let mut orthogonal = vec![0.0f32; EMBEDDING_DIMENSION];
        orthogonal[1] = 1.0;
        let mismatch = FaceUnlockEngine::verify_face(&conn, "student1", &orthogonal);
        assert!(matches!(mismatch, Err(FaceAuthError::ScoreMismatch { .. })));

        let bad_dim = FaceUnlockEngine::verify_face(&conn, "student1", &[1.0, 2.0]);
        assert!(matches!(bad_dim, Err(FaceAuthError::InvalidDimension(2))));

        assert_eq!(FaceUnlockEngine::compute_cosine_similarity(&[], &[]), 0.0);
    }
}
