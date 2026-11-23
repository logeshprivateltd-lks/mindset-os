//! "Why This Matters" Goal Bridge
//! Connects daily syllabus tasks to high-stakes exam marks (e.g. NEET 2027 MBBS seat).
//! Never fabricates numbers — if mastery or syllabus weights are missing, marks delta returns None.

use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct GoalBridge {
    pub exam: String,                 // "NEET 2027"
    pub exam_date: NaiveDate,         // 2027-04-05
    pub target_marks: u32,            // 650
    pub subject_weight: f32,          // fraction of exam this subject is (Physics = 0.25)
    pub topic_weight: f32,            // fraction of subject this topic is (Wave Optics = 0.06)
    pub current_mastery: Option<f32>, // 0.40
    pub target_mastery: Option<f32>,  // 0.75
}

impl GoalBridge {
    pub fn new(
        exam: String,
        exam_date: NaiveDate,
        target_marks: u32,
        subject_weight: f32,
        topic_weight: f32,
        current_mastery: Option<f32>,
        target_mastery: Option<f32>,
    ) -> Self {
        Self {
            exam,
            exam_date,
            target_marks,
            subject_weight,
            topic_weight,
            current_mastery,
            target_mastery,
        }
    }

    pub fn why_line(&self, _topic: &str) -> String {
        if let (Some(curr), Some(tgt)) = (self.current_mastery, self.target_mastery) {
            let pct_exam = (self.topic_weight * self.subject_weight * 100.0).round();
            format!(
                "This topic = {:.0}% of {} Physics. You're at {:.0}% → target {:.0}%.",
                pct_exam.max(1.0),
                self.exam,
                (curr * 100.0).round(),
                (tgt * 100.0).round()
            )
        } else {
            format!("This topic matters for {}.", self.exam)
        }
    }

    pub fn marks_delta(&self) -> Option<f32> {
        if let (Some(curr), Some(tgt)) = (self.current_mastery, self.target_mastery) {
            if tgt > curr
                && self.subject_weight > 0.0
                && self.topic_weight > 0.0
                && self.target_marks > 0
            {
                let delta = (tgt - curr)
                    * self.topic_weight
                    * self.subject_weight
                    * (self.target_marks as f32);
                let rounded = (delta * 10.0).round() / 10.0;
                if rounded > 0.0 {
                    return Some(rounded);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_bridge_never_fabricates_marks() {
        let date = NaiveDate::from_ymd_opt(2027, 4, 5).unwrap();

        // 1. Missing mastery row -> marks_delta() == None
        let bridge_missing = GoalBridge::new(
            "NEET 2027".to_string(),
            date,
            720,
            0.25,
            0.06,
            None,
            Some(0.80),
        );
        assert_eq!(bridge_missing.marks_delta(), None);
        assert_eq!(
            bridge_missing.why_line("Wave Optics"),
            "This topic matters for NEET 2027."
        );

        // 2. Both present -> valid calculated delta
        let bridge_valid = GoalBridge::new(
            "NEET 2027".to_string(),
            date,
            720,
            0.25,
            0.06,
            Some(0.40),
            Some(0.75),
        );
        // (0.75 - 0.40) * 0.06 * 0.25 * 720 = 0.35 * 0.015 * 720 = 3.78 -> ~3.8 marks
        let delta = bridge_valid
            .marks_delta()
            .expect("Marks delta should be computed");
        assert!(delta > 3.0 && delta < 4.5);
    }
}
