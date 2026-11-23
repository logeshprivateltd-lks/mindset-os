//! Engaging & Funny OS Micro-Activities
//!
//! Provides the native engagement layer:
//! 1. Themed Boss Battles with live HP bar and Platinum/Gold victory rewards.
//! 2. "Roast My Excuse" inside MindfulGuard with a 15-second rapid formula check.
//! 3. 2-Minute Brain-Reset Arcade (Formula Speed-Match) strictly during Pomodoro breaks.
//! 4. Future-You Time-Capsule Postcards triggered on milestone streaks.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. Live Boss Battle State & HP Mechanics
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BossBattleSession {
    pub boss_name: String,
    pub boss_title: String,
    pub chapter: String,
    pub current_hp: f32,   // 100.0 -> 0.0
    pub max_hp: f32,       // 100.0
    pub current_stage: u8, // 1: Rapid Recall, 2: Applied, 3: Feynman, 4: Result
    pub stage1_selected: [Option<usize>; 2],
    pub stage2_selected: [Option<usize>; 2],
    pub stage3_explanation: String,
    pub is_victory: bool,
    pub is_defeated_by_boss: bool,
    pub focus_loss_count: u8,
}

impl BossBattleSession {
    pub fn new(boss_name: &str, boss_title: &str, chapter: &str) -> Self {
        Self {
            boss_name: boss_name.to_string(),
            boss_title: boss_title.to_string(),
            chapter: chapter.to_string(),
            current_hp: 100.0,
            max_hp: 100.0,
            current_stage: 1,
            stage1_selected: [None, None],
            stage2_selected: [None, None],
            stage3_explanation: String::new(),
            is_victory: false,
            is_defeated_by_boss: false,
            focus_loss_count: 0,
        }
    }

    /// Reduces Boss HP by damage points (clamped to 0.0).
    pub fn apply_damage(&mut self, damage: f32) {
        self.current_hp = (self.current_hp - damage).max(0.0);
        if self.current_hp <= 0.0 {
            self.is_victory = true;
        }
    }

    /// Progresses to the next stage in the verification boss battle.
    pub fn advance_stage(&mut self) {
        if self.current_stage < 4 {
            self.current_stage += 1;
        }
    }
}

// ============================================================================
// 2. Roast My Excuse & 15-Second Rapid Formula Bailout
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RapidFormulaCheck {
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub seconds_allotted: u8, // 15s
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExcuseRoast {
    pub excuse_given: String,
    pub witty_roast: String,
    pub movie_reference: String,
    pub formula_check: RapidFormulaCheck,
    pub seconds_remaining: u8,
    pub is_unlocked: bool,
    pub has_failed: bool,
}

pub struct ExcuseRoastEngine;

impl ExcuseRoastEngine {
    /// Generates a cinema-styled witty roast for an excuse with a 15-second bailout check.
    pub fn generate_roast(excuse_text: &str) -> ExcuseRoast {
        let (roast, movie_ref) = match excuse_text.to_lowercase() {
            e if e.contains("reel") || e.contains("phone") || e.contains("scroll") => (
                "Just 5 minutes of reels? That's how 590 turns into 490! Padikkama reel paatha mark varuma thambi?".to_string(),
                "Padayappa — 'Adhigam aasaipattavanum, adhigam reel paathavanum urupattadha sarithiramilla!'".to_string(),
            ),
            e if e.contains("tired") || e.contains("sleep") || e.contains("rest") => (
                "Sleep is essential, but skipping Today's Capsule because of a sudden yawn? Your future self is giving you side-eye right now.".to_string(),
                "Anniyan — '5 marks poiduchu Remo... Aiyayo poiduchu!'".to_string(),
            ),
            e if e.contains("tomorrow") || e.contains("later") => (
                "'I will study double tomorrow' is the single greatest lie in Indian educational history since 1947.".to_string(),
                "Baashha — 'Naan oru thadava skip panna, nooru thadava backlog aagum!'".to_string(),
            ),
            _ => (
                "Your excuse has been processed by Maindy AI and rejected with 99.9% statistical confidence. Get back to the derivation!".to_string(),
                "Vikram — 'Aarambikalangala?!'".to_string(),
            ),
        };

        let formula_check = RapidFormulaCheck {
            prompt: "RAPID 15s CHECK: Which equation represents Ohm's Law at constant temperature?"
                .to_string(),
            options: vec![
                "V = I * R".to_string(),
                "P = V * I²".to_string(),
                "E = m * c²".to_string(),
                "F = m * a".to_string(),
            ],
            correct_index: 0,
            seconds_allotted: 15,
        };

        ExcuseRoast {
            excuse_given: excuse_text.to_string(),
            witty_roast: roast,
            movie_reference: movie_ref,
            formula_check,
            seconds_remaining: 15,
            is_unlocked: false,
            has_failed: false,
        }
    }

    /// Evaluates the user's bailout attempt.
    pub fn evaluate_answer(roast: &mut ExcuseRoast, chosen_index: usize) -> bool {
        if chosen_index == roast.formula_check.correct_index && roast.seconds_remaining > 0 {
            roast.is_unlocked = true;
            roast.has_failed = false;
            true
        } else {
            roast.has_failed = true;
            roast.is_unlocked = false;
            false
        }
    }
}

// ============================================================================
// 3. 2-Minute Brain-Reset Arcade (Break-Only Unlock)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaCard {
    pub id: usize,
    pub text: String,
    pub pair_id: usize,
    pub is_matched: bool,
    pub is_selected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrainResetArcade {
    pub cards: Vec<FormulaCard>,
    pub selected_first: Option<usize>,
    pub matches_found: usize,
    pub total_pairs: usize,
    pub moves_count: u32,
    pub is_completed: bool,
    pub break_seconds_left: u32,
}

impl BrainResetArcade {
    /// Creates a fresh Formula Speed-Match game for Pomodoro breaks.
    pub fn new(break_duration_seconds: u32) -> Self {
        let cards = vec![
            FormulaCard {
                id: 0,
                text: "E = E° - (0.0591/n)logQ".to_string(),
                pair_id: 1,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 1,
                text: "Nernst Equation".to_string(),
                pair_id: 1,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 2,
                text: "Φ = Q_enclosed / ε₀".to_string(),
                pair_id: 2,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 3,
                text: "Gauss's Law".to_string(),
                pair_id: 2,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 4,
                text: "ε = -dΦ/dt".to_string(),
                pair_id: 3,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 5,
                text: "Faraday-Lenz Law".to_string(),
                pair_id: 3,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 6,
                text: "F = q(E + v × B)".to_string(),
                pair_id: 4,
                is_matched: false,
                is_selected: false,
            },
            FormulaCard {
                id: 7,
                text: "Lorentz Force".to_string(),
                pair_id: 4,
                is_matched: false,
                is_selected: false,
            },
        ];

        Self {
            cards,
            selected_first: None,
            matches_found: 0,
            total_pairs: 4,
            moves_count: 0,
            is_completed: false,
            break_seconds_left: break_duration_seconds,
        }
    }

    /// Handles clicking a formula card.
    pub fn select_card(&mut self, card_index: usize) -> bool {
        if card_index >= self.cards.len() || self.cards[card_index].is_matched {
            return false;
        }

        if let Some(first_idx) = self.selected_first {
            if first_idx == card_index {
                return false;
            }
            self.moves_count += 1;
            let is_pair = self.cards[first_idx].pair_id == self.cards[card_index].pair_id;

            if is_pair {
                self.cards[first_idx].is_matched = true;
                self.cards[card_index].is_matched = true;
                self.cards[first_idx].is_selected = false;
                self.cards[card_index].is_selected = false;
                self.matches_found += 1;
                self.selected_first = None;
                if self.matches_found == self.total_pairs {
                    self.is_completed = true;
                }
                true
            } else {
                self.cards[first_idx].is_selected = false;
                self.cards[card_index].is_selected = false;
                self.selected_first = None;
                false
            }
        } else {
            self.cards[card_index].is_selected = true;
            self.selected_first = Some(card_index);
            false
        }
    }

    /// Ticks the arcade timer; returns true if the arcade should auto-lock because break ended.
    pub fn tick(&mut self) -> bool {
        self.break_seconds_left = self.break_seconds_left.saturating_sub(1);
        self.break_seconds_left == 0
    }
}

// ============================================================================
// 4. Future-You Time-Capsule Postcards
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuturePostcard {
    pub headline: String,
    pub subheadline: String,
    pub date_label: String,
    pub future_score_badge: String,
    pub quote: String,
    pub streak_milestone: u32,
}

pub struct FuturePostcardGenerator;

impl FuturePostcardGenerator {
    /// Generates a humorous and inspiring mock newspaper postcard on milestone streaks.
    pub fn generate_for_streak(
        streak_days: u32,
        student_name: &str,
        target_score: u32,
    ) -> FuturePostcard {
        let (headline, subheadline, quote) = match streak_days {
            s if s >= 30 => (
                format!(
                    "BREAKING: {} Crushes Entire Exam Blueprint With Superhuman Consistency!",
                    student_name
                ),
                format!(
                    "Official Exam Council in Awe as Projected Score Nails {}/600 Target.",
                    target_score
                ),
                "They said it was impossible, but 30 verified capsules in a row proved them wrong."
                    .to_string(),
            ),
            s if s >= 14 => (
                format!(
                    "EXAM SCOUT ALERT: {} Defeated 14 Consecutive Bosses Without Dropping A Mark!",
                    student_name
                ),
                "Neighbors Inquire About Secret Formula; Sources Confirm 'MindSet OS Deep Focus'."
                    .to_string(),
                "Two full weeks of zero excuses. Your future rank is mathematically secured."
                    .to_string(),
            ),
            _ => (
                format!(
                    "LOCAL NEWS: {} Celebrates 5-Day Verified Study Streak!",
                    student_name
                ),
                "Electrochemistry & Optics Bosses Send Formal Letters of Surrender.".to_string(),
                "Consistency is compounding. Keep fighting the daily boss!".to_string(),
            ),
        };

        FuturePostcard {
            headline,
            subheadline,
            date_label: "May 2027 — Post-Exam Special Edition".to_string(),
            future_score_badge: format!("{}/600 Target Confirmed", target_score),
            quote,
            streak_milestone: streak_days,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_battle_hp_and_victory() {
        let mut session =
            BossBattleSession::new("Lord Nernst", "Keeper of Potentials", "Electrochem");
        assert_eq!(session.current_hp, 100.0);
        assert!(!session.is_victory);

        session.apply_damage(30.0);
        assert_eq!(session.current_hp, 70.0);
        assert!(!session.is_victory);

        session.apply_damage(70.0);
        assert_eq!(session.current_hp, 0.0);
        assert!(session.is_victory);
    }

    #[test]
    fn test_roast_my_excuse_evaluation() {
        let mut roast = ExcuseRoastEngine::generate_roast("I was just watching reels on my phone");
        assert!(roast.witty_roast.contains("reel"));
        assert!(!roast.is_unlocked);

        // Correct answer is index 0 (V = I * R)
        let success = ExcuseRoastEngine::evaluate_answer(&mut roast, 0);
        assert!(success);
        assert!(roast.is_unlocked);
    }

    #[test]
    fn test_brain_reset_arcade_matching() {
        let mut arcade = BrainResetArcade::new(120);
        assert_eq!(arcade.matches_found, 0);

        // Select Card 0 (Nernst formula, pair_id: 1) and Card 1 (Nernst label, pair_id: 1)
        arcade.select_card(0);
        let matched = arcade.select_card(1);
        assert!(matched);
        assert_eq!(arcade.matches_found, 1);
        assert!(!arcade.is_completed);
    }

    #[test]
    fn test_future_postcard_milestones() {
        let card5 = FuturePostcardGenerator::generate_for_streak(5, "Logesh", 590);
        assert!(card5.headline.contains("5-Day"));

        let card14 = FuturePostcardGenerator::generate_for_streak(14, "Logesh", 590);
        assert!(card14.headline.contains("14 Consecutive"));
    }
}
