//! MindSet OS — Odyssey Journey Map & High-Yield Learning Game Engine (`odyssey_map.rs`)
//! Implements the neuroscience-backed game progression:
//! 1. Sinuous Level Expedition Map (Level 1..50..365+ leading to MBBS Doctor NEET 2027).
//! 2. 3-Stage High-Yield Daily Missions: Concept Flash (20m) -> PYQ Drill (25m) -> Mastery Check (15m).
//! 3. Collectible Mastery Cards (Trophy Vault) with formula summaries and high-yield pro-tips.
//! 4. Resilience Campfire Protocol: Zero-guilt cognitive de-escalation for missed days.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

/// Status of an Odyssey level node on the expedition map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Completed,
    ActiveToday,
    Campfire,
    Locked,
}

impl NodeStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::ActiveToday)
    }

    pub fn is_campfire(&self) -> bool {
        matches!(self, Self::Campfire)
    }

    pub fn is_locked(&self) -> bool {
        matches!(self, Self::Locked)
    }
}

/// Execution status of an atomic mission stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageStatus {
    Pending,
    InProgress,
    Completed,
}

/// A 15-25 minute atomic study sprint within a daily level mission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionStage {
    pub name: String,
    pub duration_mins: u32,
    pub status: StageStatus,
    pub description: String,
}

impl MissionStage {
    pub fn new(
        name: impl Into<String>,
        duration_mins: u32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            duration_mins,
            status: StageStatus::Pending,
            description: description.into(),
        }
    }
}

/// 3-Stage High-Yield Mission for today's active level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreeStageMission {
    pub stage1_concept: MissionStage, // 20m Formula Flash
    pub stage2_drill: MissionStage,   // 25m PYQ Practice
    pub stage3_mastery: MissionStage, // 15m Quiz Check
}

impl ThreeStageMission {
    pub fn new(chapter: &str, marks: u32) -> Self {
        Self {
            stage1_concept: MissionStage::new(
                "1. Concept & Formula Flash",
                20,
                format!("Review core formulas and high-yield theory for {}", chapter),
            ),
            stage2_drill: MissionStage::new(
                "2. High-Yield PYQ Drill",
                25,
                format!(
                    "Solve 10 Previous Year Questions with Maindy AI hints for {}",
                    chapter
                ),
            ),
            stage3_mastery: MissionStage::new(
                "3. Mastery Check",
                15,
                format!("Complete 5-question validation to lock in +{} Marks", marks),
            ),
        }
    }

    pub fn is_fully_completed(&self) -> bool {
        self.stage1_concept.status == StageStatus::Completed
            && self.stage2_drill.status == StageStatus::Completed
            && self.stage3_mastery.status == StageStatus::Completed
    }

    pub fn completed_count(&self) -> usize {
        let mut count = 0;
        if self.stage1_concept.status == StageStatus::Completed {
            count += 1;
        }
        if self.stage2_drill.status == StageStatus::Completed {
            count += 1;
        }
        if self.stage3_mastery.status == StageStatus::Completed {
            count += 1;
        }
        count
    }

    pub fn progress_ratio(&self) -> f32 {
        self.completed_count() as f32 / 3.0
    }

    pub fn active_stage_idx(&self) -> usize {
        if self.stage1_concept.status != StageStatus::Completed {
            0
        } else if self.stage2_drill.status != StageStatus::Completed {
            1
        } else if self.stage3_mastery.status != StageStatus::Completed {
            2
        } else {
            3
        }
    }

    pub fn advance_next_stage(&mut self) -> bool {
        if self.stage1_concept.status != StageStatus::Completed {
            self.stage1_concept.status = StageStatus::Completed;
            self.stage2_drill.status = StageStatus::InProgress;
            false
        } else if self.stage2_drill.status != StageStatus::Completed {
            self.stage2_drill.status = StageStatus::Completed;
            self.stage3_mastery.status = StageStatus::InProgress;
            false
        } else if self.stage3_mastery.status != StageStatus::Completed {
            self.stage3_mastery.status = StageStatus::Completed;
            true // Whole mission completed!
        } else {
            true
        }
    }
}

/// Collectible trophy card minted when a level is mastered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MasteryCard {
    pub id: u32,
    pub level: u32,
    pub title: String,
    pub subject: String,
    pub marks_yield: u32,
    pub formula_summary: String,
    pub pro_tip: String,
    pub unlocked_at: String,
}

/// A node on the sinuous Odyssey Expedition Map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdysseyNode {
    pub level: u32,
    pub subject: String,
    pub chapter_title: String,
    pub marks_yield: u32,
    pub status: NodeStatus,
    pub mission: ThreeStageMission,
    pub norm_x: f32, // Normalized 0.0..1.0 coordinate for winding curve
    pub norm_y: f32, // Normalized 0.0..1.0 coordinate for winding curve
}

/// State for the Resilience Campfire (Zero-guilt failure recovery protocol).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampfireRecovery {
    pub active: bool,
    pub points_needed: u32,
    pub micro_task_title: String,
    pub duration_mins: u32,
    pub completed: bool,
    pub supportive_message: String,
}

impl Default for CampfireRecovery {
    fn default() -> Self {
        Self {
            active: false,
            points_needed: 4,
            micro_task_title: "15m Key Formula Review".to_string(),
            duration_mins: 15,
            completed: false,
            supportive_message: "Every doctor has pitstops. Let's keep it light today: just 15 minutes to resume momentum.".to_string(),
        }
    }
}

/// Complete state of the Odyssey Map and Game Engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdysseyMapState {
    pub current_level: u32,
    pub total_levels: u32,
    pub target_exam: String,
    pub target_role: String,
    pub days_remaining: u32,
    pub target_score: u32,
    pub projected_score: u32,
    pub max_score: u32,
    pub campfire: CampfireRecovery,
    pub nodes: Vec<OdysseyNode>,
    pub trophy_deck: Vec<MasteryCard>,
    pub selected_node_idx: Option<usize>,
    pub streak_days: u32,
}

impl Default for OdysseyMapState {
    fn default() -> Self {
        Self::neet_2027_doctor_preset()
    }
}

impl OdysseyMapState {
    /// Creates the standard NEET 2027 MBBS Doctor preset.
    pub fn neet_2027_doctor_preset() -> Self {
        let mut nodes = Vec::new();

        let syllabus = [
            (
                1,
                "Physics",
                "Units & Measurements & Dimensional Analysis",
                8,
                0.15,
                0.88,
            ),
            (
                2,
                "Chemistry",
                "Some Basic Concepts of Chemistry (Mole Concept)",
                8,
                0.28,
                0.84,
            ),
            (
                3,
                "Biology",
                "Living World & Biological Classification",
                12,
                0.42,
                0.86,
            ),
            (
                4,
                "Physics",
                "Motion in a Straight Line & Vectors",
                8,
                0.58,
                0.82,
            ),
            (5, "Biology", "Plant Kingdom & Morphology", 16, 0.72, 0.85),
            (
                6,
                "Chemistry",
                "Atomic Structure & Quantum Numbers",
                8,
                0.85,
                0.80,
            ),
            (7, "Physics", "Laws of Motion & Friction", 12, 0.80, 0.68),
            (
                8,
                "Biology",
                "Cell: The Unit of Life & Cell Cycle",
                16,
                0.65,
                0.65,
            ),
            (
                9,
                "Chemistry",
                "Chemical Bonding & Molecular Structure",
                16,
                0.50,
                0.62,
            ),
            (10, "Physics", "Work, Energy & Power", 8, 0.35, 0.60),
            // Milestone Level 50 (Active)
            (
                50,
                "Physics",
                "Modern Physics: Photoelectric Effect & Photons",
                16,
                0.25,
                0.45,
            ),
            (
                51,
                "Biology",
                "Molecular Basis of Inheritance (DNA & Replication)",
                20,
                0.38,
                0.40,
            ),
            (
                52,
                "Chemistry",
                "Thermodynamics & Enthalpy of Reactions",
                12,
                0.55,
                0.42,
            ),
            (
                53,
                "Physics",
                "Ray Optics & Optical Instruments",
                16,
                0.72,
                0.38,
            ),
            (
                54,
                "Biology",
                "Genetics & Principles of Inheritance (Mendelism)",
                24,
                0.82,
                0.32,
            ),
            (
                55,
                "Chemistry",
                "Electrochemistry & Nernst Equation",
                12,
                0.70,
                0.22,
            ),
            (
                56,
                "Physics",
                "Current Electricity & Kirchhoff's Rules",
                16,
                0.52,
                0.20,
            ),
            (
                57,
                "Biology",
                "Human Physiology: Neural Control & Coordination",
                16,
                0.35,
                0.18,
            ),
            (
                58,
                "Chemistry",
                "Coordination Compounds & Crystal Field Theory",
                12,
                0.22,
                0.12,
            ),
            (
                100,
                "Goal Summit",
                "MBBS Doctor (Govt Medical College Cut-off)",
                150,
                0.50,
                0.04,
            ),
        ];

        for (lvl, subj, title, marks, nx, ny) in syllabus {
            let status = if lvl < 50 {
                NodeStatus::Completed
            } else if lvl == 50 {
                NodeStatus::ActiveToday
            } else {
                NodeStatus::Locked
            };

            let mut mission = ThreeStageMission::new(title, marks);
            if status == NodeStatus::Completed {
                mission.stage1_concept.status = StageStatus::Completed;
                mission.stage2_drill.status = StageStatus::Completed;
                mission.stage3_mastery.status = StageStatus::Completed;
            }

            nodes.push(OdysseyNode {
                level: lvl,
                subject: subj.to_string(),
                chapter_title: title.to_string(),
                marks_yield: marks,
                status,
                mission,
                norm_x: nx,
                norm_y: ny,
            });
        }

        // Seed unlocked trophy cards from completed chapters
        let trophy_deck = vec![
            MasteryCard {
                id: 1,
                level: 1,
                title: "Dimensional Analysis & Significant Figures".to_string(),
                subject: "Physics".to_string(),
                marks_yield: 8,
                formula_summary: "[M L^2 T^-2] for Energy • Error % = (ΔA/A + ΔB/B) * 100"
                    .to_string(),
                pro_tip: "Dimensions of RC, L/R, and sqrt(LC) are all Time [T]!".to_string(),
                unlocked_at: "Level 1 Cleared".to_string(),
            },
            MasteryCard {
                id: 2,
                level: 2,
                title: "Mole Concept & Stoichiometry".to_string(),
                subject: "Chemistry".to_string(),
                marks_yield: 8,
                formula_summary:
                    "Molarity = n/V(L) • Normality = M * n-factor • Ideal Gas PV = nRT".to_string(),
                pro_tip:
                    "Look for Limiting Reagent by dividing moles by stoichiometric coefficient!"
                        .to_string(),
                unlocked_at: "Level 2 Cleared".to_string(),
            },
            MasteryCard {
                id: 3,
                level: 3,
                title: "Biological Classification & Five Kingdoms".to_string(),
                subject: "Biology".to_string(),
                marks_yield: 12,
                formula_summary:
                    "Monera, Protista, Fungi, Plantae, Animalia • Carl Woese 3 Domains".to_string(),
                pro_tip: "Mycoplasma lack cell wall and are resistant to penicillin!".to_string(),
                unlocked_at: "Level 3 Cleared".to_string(),
            },
        ];

        Self {
            current_level: 50,
            total_levels: 100,
            target_exam: "NEET 2027".to_string(),
            target_role: "MBBS Doctor".to_string(),
            days_remaining: 384,
            target_score: 680,
            projected_score: 585,
            max_score: 720,
            campfire: CampfireRecovery::default(),
            nodes,
            trophy_deck,
            selected_node_idx: Some(10), // index of level 50
            streak_days: 14,
        }
    }

    /// Returns the currently active node for today.
    pub fn active_node(&self) -> Option<&OdysseyNode> {
        self.nodes
            .iter()
            .find(|n| n.status == NodeStatus::ActiveToday || n.status == NodeStatus::Campfire)
    }

    /// Returns a mutable reference to the active node.
    pub fn active_node_mut(&mut self) -> Option<&mut OdysseyNode> {
        self.nodes
            .iter_mut()
            .find(|n| n.status == NodeStatus::ActiveToday || n.status == NodeStatus::Campfire)
    }

    /// Advances the current mission stage (Stage 1 -> Stage 2 -> Stage 3).
    /// If Stage 3 completes, automatically advances the level and mints a Trophy Card.
    pub fn advance_mission(&mut self) -> bool {
        let (marks_to_add, level, title, subject) = {
            let active = match self.active_node_mut() {
                Some(n) => n,
                None => return false,
            };

            let mission_done = active.mission.advance_next_stage();
            if !mission_done {
                return false;
            }

            active.status = NodeStatus::Completed;
            (
                active.marks_yield,
                active.level,
                active.chapter_title.clone(),
                active.subject.clone(),
            )
        };

        // Mission complete! Lock in marks and mint trophy
        self.projected_score = (self.projected_score + marks_to_add).min(self.max_score);
        self.streak_days += 1;

        // Mint collectible card
        let card_id = (self.trophy_deck.len() + 1) as u32;
        self.trophy_deck.push(MasteryCard {
            id: card_id,
            level,
            title: title.clone(),
            subject: subject.clone(),
            marks_yield: marks_to_add,
            formula_summary: format!("Core Formulas & Laws for {}", title),
            pro_tip: "High-yield NEET trap avoided: mastered via MindSet OS!".to_string(),
            unlocked_at: format!("Level {} Cleared", level),
        });

        // Unlock next node
        let next_idx = self.nodes.iter().position(|n| n.level > level);
        if let Some(idx) = next_idx {
            self.nodes[idx].status = NodeStatus::ActiveToday;
            self.current_level = self.nodes[idx].level;
            self.selected_node_idx = Some(idx);
        }

        true
    }

    /// Triggers the zero-guilt Resilience Campfire mode (called when student missed yesterday's task).
    pub fn trigger_campfire(&mut self, points_needed: u32, micro_task: &str) {
        self.campfire.active = true;
        self.campfire.points_needed = points_needed;
        self.campfire.micro_task_title = micro_task.to_string();
        self.campfire.completed = false;

        if let Some(active) = self.active_node_mut() {
            active.status = NodeStatus::Campfire;
        }
    }

    /// Resolves the campfire mode once the 15-minute micro task is completed.
    pub fn resolve_campfire(&mut self) {
        self.campfire.completed = true;
        self.campfire.active = false;
        self.projected_score =
            (self.projected_score + self.campfire.points_needed).min(self.max_score);

        if let Some(active) = self.active_node_mut() {
            active.status = NodeStatus::ActiveToday;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odyssey_map_preset_initialization() {
        let map = OdysseyMapState::neet_2027_doctor_preset();
        assert_eq!(map.current_level, 50);
        assert_eq!(map.target_exam, "NEET 2027");
        assert_eq!(map.projected_score, 585);
        assert_eq!(map.streak_days, 14);
        assert!(map.nodes.len() >= 10);
        assert!(!map.trophy_deck.is_empty());

        let active = map.active_node().expect("Active node must exist");
        assert_eq!(active.level, 50);
        assert_eq!(active.status, NodeStatus::ActiveToday);
        assert_eq!(active.mission.active_stage_idx(), 0);
    }

    #[test]
    fn test_three_stage_mission_lifecycle_and_trophy_minting() {
        let mut map = OdysseyMapState::neet_2027_doctor_preset();
        let initial_score = map.projected_score;
        let initial_cards = map.trophy_deck.len();

        // Stage 1: Concept Flash
        let done1 = map.advance_mission();
        assert!(!done1);
        assert_eq!(map.active_node().unwrap().mission.active_stage_idx(), 1);

        // Stage 2: PYQ Drill
        let done2 = map.advance_mission();
        assert!(!done2);
        assert_eq!(map.active_node().unwrap().mission.active_stage_idx(), 2);

        // Stage 3: Mastery Check (Level Complete!)
        let done3 = map.advance_mission();
        assert!(done3);
        assert!(map.projected_score > initial_score);
        assert_eq!(map.trophy_deck.len(), initial_cards + 1);
        assert_eq!(map.current_level, 51);
    }

    #[test]
    fn test_resilience_campfire_de_escalation() {
        let mut map = OdysseyMapState::neet_2027_doctor_preset();
        map.trigger_campfire(4, "15m Key Formula Review");
        assert!(map.campfire.active);
        assert_eq!(map.active_node().unwrap().status, NodeStatus::Campfire);

        let pre_score = map.projected_score;
        map.resolve_campfire();
        assert!(!map.campfire.active);
        assert!(map.campfire.completed);
        assert_eq!(map.projected_score, pre_score + 4);
        assert_eq!(map.active_node().unwrap().status, NodeStatus::ActiveToday);
    }
}
