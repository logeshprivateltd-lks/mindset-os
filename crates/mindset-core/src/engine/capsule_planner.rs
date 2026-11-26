//! Target-to-Capsule Breakdown Engine & Auto-Healing Recalibrator
//!
//! Converts high-stakes exam targets (e.g. 590/600 in March) into daily mark-valued capsules.
//! Splits prep days across the 4 Macro Preparation Phases, reserves 10% buffer days,
//! and recalibrates automatically with zero guilt when days are missed.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::agent::exam_scout::ExamProfile;
use crate::engine::verification::CapsuleState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreparationPhase {
    /// Phase 1: Core Foundation (45% of study days) — Goldmines & Heavyweights
    CoreFoundation,
    /// Phase 2: Gap Closure (25% of study days) — Remaining units, numerical drills
    GapClosure,
    /// Phase 3: Exam Simulation (20% of study days) — Full timed mocks & speed drills
    ExamSimulation,
    /// Phase 4: Lock-In & Taper (10% of study days) — High-yield revision & error log
    LockInTaper,
}

impl PreparationPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CoreFoundation => "Phase 1: Core Foundation (45%)",
            Self::GapClosure => "Phase 2: Gap Closure (25%)",
            Self::ExamSimulation => "Phase 3: Exam Simulation (20%)",
            Self::LockInTaper => "Phase 4: Lock-In & Taper (10%)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapsuleSegment {
    pub name: String,
    pub description: String,
    pub estimated_minutes: u32,
    pub percentage: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyCapsule {
    pub capsule_id: String,
    pub day_index: u32,
    #[serde(default)]
    pub scheduled_date: String,
    #[serde(default)]
    pub is_weekend: bool,
    #[serde(default)]
    pub time_slot_label: String,
    #[serde(default)]
    pub budget_mode: String,
    pub phase: PreparationPhase,
    pub subject: String,
    pub title: String,
    pub chapter: String,
    pub why_matters: String,
    pub mark_value: f32,
    pub total_minutes: u32,
    pub segments: Vec<CapsuleSegment>,
    pub state: CapsuleState,
    pub is_buffer_day: bool,
}

impl DailyCapsule {
    /// Creates a standard 4-segment Daily Capsule.
    #[allow(clippy::too_many_arguments)]
    pub fn new_study_capsule(
        capsule_id: &str,
        day_index: u32,
        phase: PreparationPhase,
        subject: &str,
        chapter: &str,
        title: &str,
        mark_value: f32,
        total_minutes: u32,
    ) -> Self {
        let heavy_lift_mins = (total_minutes as f32 * 0.45).round() as u32;
        let active_practice_mins = (total_minutes as f32 * 0.25).round() as u32;
        let memory_anchor_mins = (total_minutes as f32 * 0.15).round() as u32;
        let verification_gate_mins = total_minutes
            .saturating_sub(heavy_lift_mins + active_practice_mins + memory_anchor_mins);

        let segments = vec![
            CapsuleSegment {
                name: "Heavy Lift (45%)".to_string(),
                description: format!("Core concept & derivation: {}", title),
                estimated_minutes: heavy_lift_mins,
                percentage: 45,
            },
            CapsuleSegment {
                name: "Active Practice (25%)".to_string(),
                description: "Past-year questions & step-by-step problem set".to_string(),
                estimated_minutes: active_practice_mins,
                percentage: 25,
            },
            CapsuleSegment {
                name: "Memory Anchor (15%)".to_string(),
                description: "Spaced-repetition review of earlier concepts".to_string(),
                estimated_minutes: memory_anchor_mins,
                percentage: 15,
            },
            CapsuleSegment {
                name: "Proof-of-Study Boss (15%)".to_string(),
                description: "3-Stage verification test (Recall, Applied, Feynman)".to_string(),
                estimated_minutes: verification_gate_mins,
                percentage: 15,
            },
        ];

        Self {
            capsule_id: capsule_id.to_string(),
            day_index,
            scheduled_date: String::new(),
            is_weekend: false,
            time_slot_label: "Morning Sprint".to_string(),
            budget_mode: "Standard".to_string(),
            phase,
            subject: subject.to_string(),
            title: title.to_string(),
            chapter: chapter.to_string(),
            why_matters: format!(
                "Worth +{:.1} marks directly mapped to your target score.",
                mark_value
            ),
            mark_value,
            total_minutes,
            segments,
            state: CapsuleState::Active,
            is_buffer_day: false,
        }
    }

    /// Creates a designated zero-guilt Buffer Day capsule.
    pub fn new_buffer_day(day_index: u32, phase: PreparationPhase) -> Self {
        Self {
            capsule_id: format!("buffer_day_{}", day_index),
            day_index,
            scheduled_date: String::new(),
            is_weekend: false,
            time_slot_label: "Flexible Catch-Up".to_string(),
            budget_mode: "Buffer".to_string(),
            phase,
            subject: "Buffer & Consolidation".to_string(),
            title: "Zero-Guilt Catch-Up & Spaced Revision Buffer".to_string(),
            chapter: "System Buffer".to_string(),
            why_matters: "Reserved safety day to absorb unexpected life delays without stress."
                .to_string(),
            mark_value: 0.0,
            total_minutes: 60,
            segments: vec![CapsuleSegment {
                name: "Buffer Rest / Review".to_string(),
                description: "Catch up on missed capsules or do calm spaced flashcard review"
                    .to_string(),
                estimated_minutes: 60,
                percentage: 100,
            }],
            state: CapsuleState::Active,
            is_buffer_day: true,
        }
    }
}

/// The state of the Target-to-Capsule Engine across the student's study timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCapsulePlan {
    pub exam_id: String,
    pub exam_name: String,
    pub target_score: u32,
    pub max_possible_marks: u32,
    pub baseline_readiness: u32,
    pub predicted_exam_score: f32,
    pub mindset_xp: u32,
    pub total_study_days: u32,
    pub remaining_study_days: u32,
    pub capsules: Vec<DailyCapsule>,
    pub current_capsule_index: usize,
    pub buffer_days_remaining: u32,
    pub missed_days_healed: u32,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub last_active_date: String,
}

pub struct CapsulePlanner;

impl CapsulePlanner {
    /// Generates a comprehensive Target-to-Capsule breakdown plan.
    pub fn generate_plan(
        profile: &ExamProfile,
        target_score: u32,
        baseline_score: u32,
        remaining_days: u32,
        daily_minutes_budget: u32,
    ) -> TargetCapsulePlan {
        let effective_target = target_score.min(profile.total_marks);
        let effective_baseline = baseline_score.min(effective_target);

        // Safety buffer: 2.5% of total marks
        let safety_buffer = (profile.total_marks as f32 * 0.025).round();
        let target_with_buffer =
            (effective_target as f32 + safety_buffer).min(profile.total_marks as f32);
        let mark_gap = target_with_buffer - effective_baseline as f32;

        let total_days = remaining_days.max(10);

        // 4 Macro phase day allocations
        let phase1_days = ((total_days as f32) * 0.45).round() as u32;
        let phase2_days = ((total_days as f32) * 0.25).round() as u32;
        let phase3_days = ((total_days as f32) * 0.20).round() as u32;
        let _phase4_days = total_days.saturating_sub(phase1_days + phase2_days + phase3_days);

        // Filter and sort chapters: Goldmines first, Heavyweights second, QuickWins third, Traps last
        let mut sorted_chapters = profile.chapters.clone();
        sorted_chapters.sort_by(|a, b| {
            a.quadrant
                .priority()
                .cmp(&b.quadrant.priority())
                .then_with(|| {
                    b.weightage_marks
                        .partial_cmp(&a.weightage_marks)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        // Reserve 10% of days as dedicated buffer days
        let total_buffer_days = (total_days as f32 * 0.10).max(1.0).round() as u32;
        let mut buffer_days_left = total_buffer_days;

        let mut capsules = Vec::new();
        let mut chapter_idx = 0;
        let total_active_chapters = sorted_chapters.len().max(1);

        // Mark value per day proportional to gap
        let active_study_days = total_days.saturating_sub(total_buffer_days).max(1);
        let avg_mark_per_capsule = mark_gap / active_study_days as f32;

        for day in 1..=total_days {
            let phase = if day <= phase1_days {
                PreparationPhase::CoreFoundation
            } else if day <= phase1_days + phase2_days {
                PreparationPhase::GapClosure
            } else if day <= phase1_days + phase2_days + phase3_days {
                PreparationPhase::ExamSimulation
            } else {
                PreparationPhase::LockInTaper
            };

            // Every 7th or 10th day is an automatic buffer day if buffers remain
            let is_buffer_slot = (day % 8 == 0 || day % 15 == 0) && buffer_days_left > 0;

            if is_buffer_slot {
                capsules.push(DailyCapsule::new_buffer_day(day, phase));
                buffer_days_left = buffer_days_left.saturating_sub(1);
            } else {
                let chapter = &sorted_chapters[chapter_idx % total_active_chapters];
                chapter_idx += 1;

                let mark_val = (avg_mark_per_capsule * 10.0).round() / 10.0;
                let cap = DailyCapsule::new_study_capsule(
                    &format!("cap_day_{}", day),
                    day,
                    phase,
                    &chapter.subject,
                    &chapter.chapter_name,
                    &chapter
                        .high_yield_subtopics
                        .first()
                        .cloned()
                        .unwrap_or_else(|| chapter.chapter_name.clone()),
                    mark_val.max(0.5),
                    daily_minutes_budget,
                );
                capsules.push(cap);
            }
        }

        let start_date_naive = chrono::Local::now().date_naive();
        let start_date = start_date_naive.format("%Y-%m-%d").to_string();
        for (idx, cap) in capsules.iter_mut().enumerate() {
            use chrono::Datelike;
            let day_date = start_date_naive + chrono::Duration::days(idx as i64);
            cap.scheduled_date = day_date.format("%Y-%m-%d").to_string();
            let wd = day_date.weekday();
            cap.is_weekend = matches!(wd, chrono::Weekday::Sat | chrono::Weekday::Sun);
            if cap.is_weekend && !cap.is_buffer_day {
                cap.budget_mode = "Weekend Sprint".to_string();
            }
        }

        TargetCapsulePlan {
            exam_id: profile.id.clone(),
            exam_name: profile.name.clone(),
            target_score: effective_target,
            max_possible_marks: profile.total_marks,
            baseline_readiness: effective_baseline,
            predicted_exam_score: effective_baseline as f32,
            mindset_xp: 0,
            total_study_days: total_days,
            remaining_study_days: total_days,
            capsules,
            current_capsule_index: 0,
            buffer_days_remaining: total_buffer_days,
            missed_days_healed: 0,
            start_date: start_date.clone(),
            last_active_date: start_date,
        }
    }

    /// Zero-Guilt Auto-Healing Recalibration when a student misses one or more days.
    pub fn auto_heal_missed_day(plan: &mut TargetCapsulePlan, missed_days_count: u32) {
        if missed_days_count == 0 {
            return;
        }

        let mut days_to_absorb = missed_days_count;

        // Step 1: Absorb into reserved buffer days first
        if plan.buffer_days_remaining > 0 {
            let absorbed = days_to_absorb.min(plan.buffer_days_remaining);
            plan.buffer_days_remaining -= absorbed;
            days_to_absorb -= absorbed;
            plan.missed_days_healed += absorbed;
        }

        // Step 2: If buffer days exhausted, trim low-ROI Trap capsules
        if days_to_absorb > 0 {
            let mut trimmed_count = 0;
            // Iterate from the end and remove buffer/trap filler
            plan.capsules.retain(|cap| {
                if days_to_absorb > 0
                    && (cap.is_buffer_day
                        || cap.title.contains("Trap")
                        || cap.title.contains("Historical"))
                {
                    days_to_absorb -= 1;
                    trimmed_count += 1;
                    false
                } else {
                    true
                }
            });
            plan.missed_days_healed += trimmed_count;
        }

        // Step 3: Spread any remaining small delta across future Heavy Lift capsules by max 10%
        if days_to_absorb > 0 {
            for cap in plan.capsules.iter_mut().skip(plan.current_capsule_index) {
                if !cap.is_buffer_day {
                    cap.mark_value = (cap.mark_value * 1.05).min(20.0);
                }
            }
        }

        // Adjust remaining study days
        plan.remaining_study_days = plan.remaining_study_days.saturating_sub(missed_days_count);
    }

    /// Dynamically adapts today's active capsule duration & 4-segment breakdown
    /// from the student's daily energy check-in and available hours.
    pub fn recalibrate_for_checkin(
        plan: &mut TargetCapsulePlan,
        energy_level: &str,
        available_hours_today: f32,
    ) {
        let base_mins = (available_hours_today.max(0.4) * 60.0).round() as u32;
        let scaled_mins = match energy_level {
            "High-Energy Sprint" => ((base_mins as f32) * 1.15).round() as u32,
            "Busy Day" => ((base_mins as f32) * 0.70).round() as u32,
            "Exhausted" => 25,
            _ => base_mins,
        }
        .clamp(25, 360);

        if let Some(cap) = plan.capsules.get_mut(plan.current_capsule_index) {
            cap.total_minutes = scaled_mins;
            cap.budget_mode = energy_level.to_string();
            if cap.segments.len() == 4 {
                let s1 = (scaled_mins as f32 * 0.45).round() as u32;
                let s2 = (scaled_mins as f32 * 0.25).round() as u32;
                let s3 = (scaled_mins as f32 * 0.15).round() as u32;
                let s4 = scaled_mins.saturating_sub(s1 + s2 + s3);
                cap.segments[0].estimated_minutes = s1;
                cap.segments[1].estimated_minutes = s2;
                cap.segments[2].estimated_minutes = s3;
                cap.segments[3].estimated_minutes = s4;
            }
        }
    }

    /// Reconciles elapsed missed days between `last_active_date` and `today_date` (`YYYY-MM-DD`).
    /// Returns the number of missed days that were healed cleanly.
    pub fn reconcile_elapsed_days(plan: &mut TargetCapsulePlan, today_date: &str) -> u32 {
        if plan.last_active_date.is_empty() || plan.last_active_date == today_date {
            plan.last_active_date = today_date.to_string();
            return 0;
        }
        let Ok(prev) = chrono::NaiveDate::parse_from_str(&plan.last_active_date, "%Y-%m-%d") else {
            plan.last_active_date = today_date.to_string();
            return 0;
        };
        let Ok(curr) = chrono::NaiveDate::parse_from_str(today_date, "%Y-%m-%d") else {
            return 0;
        };
        let delta = (curr - prev).num_days();
        plan.last_active_date = today_date.to_string();
        if delta > 1 {
            let missed = (delta - 1) as u32;
            Self::auto_heal_missed_day(plan, missed);
            missed
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_590_target_plan_generation() {
        let p = ExamProfile::tn_class_12_board();
        // Target 590/600 with 100 days remaining and 90m daily budget
        let plan = CapsulePlanner::generate_plan(&p, 590, 430, 100, 90);

        assert_eq!(plan.target_score, 590);
        assert_eq!(plan.max_possible_marks, 600);
        assert_eq!(plan.baseline_readiness, 430);
        assert_eq!(plan.predicted_exam_score, 430.0);
        assert_eq!(plan.total_study_days, 100);
        assert!(!plan.capsules.is_empty());

        // Confirm 4 phases exist
        let has_phase1 = plan
            .capsules
            .iter()
            .any(|c| c.phase == PreparationPhase::CoreFoundation);
        let has_phase2 = plan
            .capsules
            .iter()
            .any(|c| c.phase == PreparationPhase::GapClosure);
        let has_phase3 = plan
            .capsules
            .iter()
            .any(|c| c.phase == PreparationPhase::ExamSimulation);
        let has_phase4 = plan
            .capsules
            .iter()
            .any(|c| c.phase == PreparationPhase::LockInTaper);
        assert!(has_phase1 && has_phase2 && has_phase3 && has_phase4);

        // Confirm buffer days were reserved (approx 10%)
        assert!(plan.buffer_days_remaining >= 8);
    }

    #[test]
    fn test_zero_guilt_auto_healing_absorbs_buffer() {
        let p = ExamProfile::tn_class_12_board();
        let mut plan = CapsulePlanner::generate_plan(&p, 590, 430, 60, 90);
        let initial_buffers = plan.buffer_days_remaining;
        assert!(initial_buffers >= 4);

        // Student missed 2 days
        CapsulePlanner::auto_heal_missed_day(&mut plan, 2);

        assert_eq!(plan.missed_days_healed, 2);
        assert_eq!(plan.buffer_days_remaining, initial_buffers - 2);
    }

    #[test]
    fn test_capsule_four_segments_sum_to_total() {
        let cap = DailyCapsule::new_study_capsule(
            "test_1",
            1,
            PreparationPhase::CoreFoundation,
            "Physics",
            "Electrostatics",
            "Gauss Law",
            3.5,
            100,
        );

        assert_eq!(cap.segments.len(), 4);
        let sum_mins: u32 = cap.segments.iter().map(|s| s.estimated_minutes).sum();
        assert_eq!(sum_mins, 100);
    }
}
