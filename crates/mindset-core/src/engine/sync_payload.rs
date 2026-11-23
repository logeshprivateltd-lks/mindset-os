//! Local JSON Achievement & Verification Sync Snapshot
//!
//! Generates a lightweight, tamper-evident JSON synchronization payload containing
//! verified capsules, rank, streak, and Predicted Exam Score ready to sync with
//! companion mobile apps and the official MindSet OS website.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::engine::verification::CapsuleRank;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifiedCapsuleRecord {
    pub capsule_id: String,
    pub title: String,
    pub subject: String,
    pub final_score: u8,
    pub rank: CapsuleRank,
    pub marks_locked: f32,
    pub xp_earned: u32,
    pub verified_at_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncPayload {
    pub schema_version: &'static str,
    pub student_id: String,
    pub student_name: String,
    pub exam_id: String,
    pub exam_name: String,
    pub target_score: u32,
    pub max_marks: u32,
    pub predicted_exam_score: f32,
    pub streak_days: u32,
    pub total_mindset_xp: u32,
    pub verified_capsules: Vec<VerifiedCapsuleRecord>,
    pub generated_at_epoch: u64,
    pub payload_checksum: String,
}

impl SyncPayload {
    pub const SCHEMA_V1: &'static str = "mindset.sync.v1";

    /// Generates a signed sync snapshot from current core engine state.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        student_id: &str,
        student_name: &str,
        exam_id: &str,
        exam_name: &str,
        target_score: u32,
        max_marks: u32,
        predicted_exam_score: f32,
        streak_days: u32,
        total_mindset_xp: u32,
        verified_capsules: Vec<VerifiedCapsuleRecord>,
        generated_at_epoch: u64,
    ) -> Self {
        // Compute deterministic checksum
        let mut raw_data = format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            student_id,
            exam_id,
            target_score,
            max_marks,
            predicted_exam_score,
            streak_days,
            total_mindset_xp,
            generated_at_epoch
        );
        for c in &verified_capsules {
            raw_data.push_str(&format!(
                ":{}:{}:{}",
                c.capsule_id, c.final_score, c.marks_locked
            ));
        }

        let checksum = Self::compute_simple_hash(&raw_data);

        Self {
            schema_version: Self::SCHEMA_V1,
            student_id: student_id.to_string(),
            student_name: student_name.to_string(),
            exam_id: exam_id.to_string(),
            exam_name: exam_name.to_string(),
            target_score,
            max_marks,
            predicted_exam_score,
            streak_days,
            total_mindset_xp,
            verified_capsules,
            generated_at_epoch,
            payload_checksum: checksum,
        }
    }

    /// Verifies the payload checksum.
    pub fn is_valid_checksum(&self) -> bool {
        let mut raw_data = format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            self.student_id,
            self.exam_id,
            self.target_score,
            self.max_marks,
            self.predicted_exam_score,
            self.streak_days,
            self.total_mindset_xp,
            self.generated_at_epoch
        );
        for c in &self.verified_capsules {
            raw_data.push_str(&format!(
                ":{}:{}:{}",
                c.capsule_id, c.final_score, c.marks_locked
            ));
        }

        let computed = Self::compute_simple_hash(&raw_data);
        computed == self.payload_checksum
    }

    /// Serializes to formatted JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    fn compute_simple_hash(input: &str) -> String {
        // Robust 64-bit FNV-1a hash formatted as 16-char hex
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in input.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_payload_generation_and_validation() {
        let records = vec![VerifiedCapsuleRecord {
            capsule_id: "cap_day_1".to_string(),
            title: "Nernst Equation Derivation".to_string(),
            subject: "Chemistry".to_string(),
            final_score: 95,
            rank: CapsuleRank::Platinum,
            marks_locked: 3.5,
            xp_earned: 150,
            verified_at_epoch: 1732000000,
        }];

        let payload = SyncPayload::build(
            "student_01",
            "Logesh Karthik",
            "tn_hsc_12",
            "TN Class 12 Board Exam",
            590,
            600,
            433.5,
            14,
            150,
            records,
            1732000100,
        );

        assert!(payload.is_valid_checksum());
        let json = payload.to_json().expect("Serialization failed");
        assert!(json.contains("tn_hsc_12"));
        assert!(json.contains("Platinum"));
        assert!(json.contains("433.5"));
    }

    #[test]
    fn test_sync_payload_detects_tampering() {
        let mut payload = SyncPayload::build(
            "student_01",
            "Logesh",
            "neet_ug",
            "NEET UG",
            680,
            720,
            500.0,
            5,
            100,
            vec![],
            1000,
        );

        assert!(payload.is_valid_checksum());

        // Tamper with predicted score
        payload.predicted_exam_score = 720.0;
        assert!(!payload.is_valid_checksum());
    }
}
