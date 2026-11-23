//! Strict Proof-of-Study Verification Gate & Point Scoring System
//!
//! Enforces the Core Product Rule: NO manual "Mark Complete" checkboxes.
//! A Daily Capsule transitions to `Completed` if and only if the student scores
//! >= 70/100 points (or >= 85/100 in Speedrun Test-Out mode) on the 3-stage Boss Battle.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapsuleRank {
    /// 70..=89 points (A-Rank) — 90% proportional marks + 100 XP
    Gold,
    /// 90..=100 points (S-Rank) — 100% full marks + 150 XP
    Platinum,
}

impl CapsuleRank {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Gold => "🥇 GOLD CLEAR (A-Rank)",
            Self::Platinum => "👑 PLATINUM CLEAR (S-Rank)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CapsuleState {
    Locked,
    #[default]
    Active,
    Studying {
        active_seconds: u64,
        idle_warnings: u8,
    },
    ReadyForBoss {
        speedrun_mode: bool,
    },
    VerificationInProgress {
        attempt: u8,
        speedrun_mode: bool,
        current_stage: u8,
        stage1_score: u8,
        stage2_score: u8,
        stage3_score: u8,
        focus_loss_count: u8,
    },
    NeedsRemediation {
        last_score: u8,
        weak_topics: Vec<String>,
        cooldown_until_epoch: u64,
        attempt: u8,
    },
    Completed {
        final_score: u8, // Strictly >= 70 (or >= 85 in speedrun_mode)
        rank: CapsuleRank,
        marks_locked: f32, // Added to Predicted Exam Score
        xp_earned: u32,
        verified_at_epoch: u64,
    },
}

impl CapsuleState {
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    pub fn completed_rank(&self) -> Option<CapsuleRank> {
        if let Self::Completed { rank, .. } = self {
            Some(*rank)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationScoreBreakdown {
    pub recall_points: u8,      // 0..=30
    pub application_points: u8, // 0..=40
    pub feynman_points: u8,     // 0..=30
    pub focus_loss_penalties: u8,
    pub total_points: u8, // 0..=100
    pub passed: bool,     // True strictly if threshold met
    pub feedback: String,
    pub weak_subtopics: Vec<String>,
}

impl VerificationScoreBreakdown {
    /// Computes total score with focus-loss penalties clamped between 0 and 100.
    pub fn compute(
        recall_pts: u8,
        applied_pts: u8,
        feynman_pts: u8,
        focus_loss_count: u8,
        speedrun_mode: bool,
        weak_topics: Vec<String>,
    ) -> Self {
        let raw_total = recall_pts.min(30) + applied_pts.min(40) + feynman_pts.min(30);
        let penalty = focus_loss_count.saturating_mul(2).min(10);
        let final_total = raw_total.saturating_sub(penalty);

        let threshold = if speedrun_mode { 85 } else { 70 };
        let passed = final_total >= threshold;

        let feedback = if final_total >= 90 {
            "PLATINUM VICTORY! Flawless conceptual mastery and precision problem solving."
                .to_string()
        } else if final_total >= 70 {
            "GOLD VICTORY! Solid proof of study. Weak points added to review queue.".to_string()
        } else if final_total >= 50 {
            "ALMOST THERE. Review the targeted micro-refresher before re-challenging the Boss."
                .to_string()
        } else {
            "BOSS DEFEATED YOU. Take 3 minutes to review the core derivation with Maindy AI."
                .to_string()
        };

        Self {
            recall_points: recall_pts.min(30),
            application_points: applied_pts.min(40),
            feynman_points: feynman_pts.min(30),
            focus_loss_penalties: penalty,
            total_points: final_total,
            passed,
            feedback,
            weak_subtopics: weak_topics,
        }
    }
}

pub struct ProofOfStudyGate;

impl ProofOfStudyGate {
    /// Minimum required study seconds to avoid speedrun hardcore mode (10 minutes = 600s).
    pub const MIN_NORMAL_STUDY_SECONDS: u64 = 600;

    /// Checks if a score passes the verification gate.
    pub fn is_passing(score: u8, speedrun_mode: bool) -> bool {
        if speedrun_mode {
            score >= 85
        } else {
            score >= 70
        }
    }

    /// Evaluates whether the user qualifies for normal boss mode or speedrun test-out mode.
    pub fn ready_for_boss(active_seconds: u64) -> CapsuleState {
        let is_speedrun = active_seconds < Self::MIN_NORMAL_STUDY_SECONDS;
        CapsuleState::ReadyForBoss {
            speedrun_mode: is_speedrun,
        }
    }

    /// Submits verification test results and updates `CapsuleState`.
    ///
    /// GUARANTEE: Returns `Err` and stays incomplete if `total_points < 70` (or `< 85` in speedrun).
    /// Can NEVER transition to `Completed` with failing scores.
    pub fn submit_verification(
        current_state: &mut CapsuleState,
        breakdown: VerificationScoreBreakdown,
        capsule_mark_value: f32,
        current_epoch: u64,
        attempt: u8,
        speedrun_mode: bool,
    ) -> Result<CapsuleRank, String> {
        let passed = Self::is_passing(breakdown.total_points, speedrun_mode);

        if !passed {
            let threshold = if speedrun_mode { 85 } else { 70 };
            *current_state = CapsuleState::NeedsRemediation {
                last_score: breakdown.total_points,
                weak_topics: breakdown.weak_subtopics.clone(),
                cooldown_until_epoch: current_epoch + 120, // 2-min micro-study cooldown
                attempt,
            };
            return Err(format!(
                "Verification failed with {}/100 points (Required: >={}). Capsule remains INCOMPLETE.",
                breakdown.total_points, threshold
            ));
        }

        // Passed! Determine rank and rewards
        let rank = if breakdown.total_points >= 90 {
            CapsuleRank::Platinum
        } else {
            CapsuleRank::Gold
        };

        let marks_locked = match rank {
            CapsuleRank::Platinum => capsule_mark_value,
            CapsuleRank::Gold => (capsule_mark_value * 0.90 * 10.0).round() / 10.0,
        };

        let xp_earned = match rank {
            CapsuleRank::Platinum => 150,
            CapsuleRank::Gold => 100,
        };

        *current_state = CapsuleState::Completed {
            final_score: breakdown.total_points,
            rank,
            marks_locked,
            xp_earned,
            verified_at_epoch: current_epoch,
        };

        Ok(rank)
    }

    /// Records focus loss during an active Boss Battle.
    pub fn record_focus_loss(current_state: &mut CapsuleState) {
        if let CapsuleState::VerificationInProgress {
            focus_loss_count, ..
        } = current_state
        {
            *focus_loss_count = focus_loss_count.saturating_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_lock_rejects_failing_scores() {
        let mut state = CapsuleState::Active;

        // Score 0
        let b0 = VerificationScoreBreakdown::compute(0, 0, 0, 0, false, vec!["All".to_string()]);
        let res0 = ProofOfStudyGate::submit_verification(&mut state, b0, 3.5, 1000, 1, false);
        assert!(res0.is_err());
        assert!(!state.is_completed());

        // Score 49
        let b49 =
            VerificationScoreBreakdown::compute(20, 20, 9, 0, false, vec!["Feynman".to_string()]);
        let res49 = ProofOfStudyGate::submit_verification(&mut state, b49, 3.5, 1000, 1, false);
        assert!(res49.is_err());
        assert!(!state.is_completed());

        // Score 69 (strictly 1 below 70 passing threshold)
        let b69 =
            VerificationScoreBreakdown::compute(25, 30, 14, 0, false, vec!["Formula".to_string()]);
        let res69 = ProofOfStudyGate::submit_verification(&mut state, b69, 3.5, 1000, 1, false);
        assert!(res69.is_err());
        assert!(!state.is_completed());

        // Confirm state transitioned to NeedsRemediation
        if let CapsuleState::NeedsRemediation { last_score, .. } = state {
            assert_eq!(last_score, 69);
        } else {
            panic!("Expected NeedsRemediation state");
        }
    }

    #[test]
    fn test_gold_clear_at_70_and_89() {
        let mut state = CapsuleState::Active;

        // Score 70
        let b70 = VerificationScoreBreakdown::compute(20, 30, 20, 0, false, vec![]);
        let res70 = ProofOfStudyGate::submit_verification(&mut state, b70, 3.5, 1000, 1, false);
        assert!(res70.is_ok());
        assert_eq!(res70.unwrap(), CapsuleRank::Gold);
        assert!(state.is_completed());

        if let CapsuleState::Completed {
            final_score,
            rank,
            marks_locked,
            xp_earned,
            ..
        } = state
        {
            assert_eq!(final_score, 70);
            assert_eq!(rank, CapsuleRank::Gold);
            assert!(marks_locked >= 3.0);
            assert_eq!(xp_earned, 100);
        } else {
            panic!("Expected Completed state");
        }
    }

    #[test]
    fn test_platinum_clear_at_90_and_100() {
        let mut state = CapsuleState::Active;

        // Score 95
        let b95 = VerificationScoreBreakdown::compute(30, 38, 27, 0, false, vec![]);
        let res95 = ProofOfStudyGate::submit_verification(&mut state, b95, 4.0, 2000, 1, false);
        assert!(res95.is_ok());
        assert_eq!(res95.unwrap(), CapsuleRank::Platinum);
        assert!(state.is_completed());

        if let CapsuleState::Completed {
            final_score,
            rank,
            marks_locked,
            xp_earned,
            ..
        } = state
        {
            assert_eq!(final_score, 95);
            assert_eq!(rank, CapsuleRank::Platinum);
            assert_eq!(marks_locked, 4.0);
            assert_eq!(xp_earned, 150);
        } else {
            panic!("Expected Completed state");
        }
    }

    #[test]
    fn test_speedrun_mode_enforces_85_threshold() {
        let mut state = CapsuleState::ReadyForBoss {
            speedrun_mode: true,
        };

        // Score 75 passes standard, but MUST fail speedrun mode
        let b75 = VerificationScoreBreakdown::compute(25, 30, 20, 0, true, vec![]);
        let res75 = ProofOfStudyGate::submit_verification(&mut state, b75, 5.0, 1000, 1, true);
        assert!(res75.is_err());
        assert!(!state.is_completed());

        // Score 85 passes speedrun mode
        let b85 = VerificationScoreBreakdown::compute(30, 35, 20, 0, true, vec![]);
        let res85 = ProofOfStudyGate::submit_verification(&mut state, b85, 5.0, 1000, 1, true);
        assert!(res85.is_ok());
        assert!(state.is_completed());
    }

    #[test]
    fn test_focus_loss_penalty_applied() {
        // 3 focus losses = 6 points penalty
        let b = VerificationScoreBreakdown::compute(30, 40, 30, 3, false, vec![]);
        assert_eq!(b.focus_loss_penalties, 6);
        assert_eq!(b.total_points, 94);
    }
}
