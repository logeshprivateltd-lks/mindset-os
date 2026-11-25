//! MindSet OS Odyssey Journey & Gamified Learning Map
//!
//! Provides pure-domain models for syllabus journey nodes, stage progression,
//! marks calculation, and trophy achievements.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Completed,
    InProgress,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Theory,
    NumericalDrill,
    ReviewChest,
    UnitBoss,
}

impl NodeType {
    pub fn display_badge(&self) -> &'static str {
        match self {
            Self::Theory => "Theory",
            Self::NumericalDrill => "Numerical Drill",
            Self::ReviewChest => "Review Chest",
            Self::UnitBoss => "Unit Boss",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Theory => "📖",
            Self::NumericalDrill => "📐",
            Self::ReviewChest => "🎁",
            Self::UnitBoss => "⚔️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyNode {
    pub id: String,
    pub unit_id: String,
    pub unit_title: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub marks: u32,
    pub max_marks: u32,
    pub xp: u32,
    pub stars_earned: u8,
    pub is_crown_mastered: bool,
    #[serde(default = "default_estimated_minutes")]
    pub estimated_minutes: u32,
}

fn default_estimated_minutes() -> u32 {
    35
}

impl OdysseyNode {
    pub fn compute_stars(marks: u32, max_marks: u32) -> u8 {
        if max_marks == 0 {
            return 0;
        }
        let pct = (marks as f32 / max_marks as f32) * 100.0;
        if pct >= 95.0 {
            3
        } else if pct >= 80.0 {
            2
        } else if pct >= 60.0 {
            1
        } else {
            0
        }
    }

    pub fn compute_crown(marks: u32, max_marks: u32) -> bool {
        if max_marks == 0 {
            return false;
        }
        let pct = (marks as f32 / max_marks as f32) * 100.0;
        pct >= 98.0
    }

    pub fn update_evaluation(&mut self, marks: u32) {
        self.marks = marks.min(self.max_marks);
        self.stars_earned = Self::compute_stars(self.marks, self.max_marks);
        self.is_crown_mastered = Self::compute_crown(self.marks, self.max_marks);
    }

    pub fn star_rating_display(&self) -> String {
        match self.stars_earned {
            3 => "★★★".to_string(),
            2 => "★★☆".to_string(),
            1 => "★☆☆".to_string(),
            _ => "☆☆☆".to_string(),
        }
    }

    pub fn mastery_percentage(&self) -> u32 {
        if self.max_marks == 0 {
            0
        } else {
            ((self.marks as f32 / self.max_marks as f32) * 100.0).round() as u32
        }
    }

    pub fn estimated_time_display(&self) -> String {
        format!("{} mins", self.estimated_minutes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrophyCard {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageAction {
    Embark,
    CompleteStage,
    RestAndReflect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyJourneyState {
    pub nodes: Vec<OdysseyNode>,
    pub trophies: Vec<TrophyCard>,
    pub current_stage_index: usize,
    pub total_xp: u32,
    pub level: u32,
}

impl Default for OdysseyJourneyState {
    fn default() -> Self {
        Self::new()
    }
}

impl OdysseyJourneyState {
    pub fn new() -> Self {
        let nodes = vec![
            OdysseyNode {
                id: "phy-101".into(),
                unit_id: "unit-1-electromagnetism".into(),
                unit_title: "Unit 1: Electromagnetism & Potentials".into(),
                title: "Electrostatics & Gauss's Law".into(),
                subject: "Physics".into(),
                description:
                    "Electric flux, closed Gaussian surfaces, and Coulomb field derivation.".into(),
                node_type: NodeType::Theory,
                status: NodeStatus::InProgress,
                marks: 85,
                max_marks: 100,
                xp: 250,
                stars_earned: 2,
                is_crown_mastered: false,
                estimated_minutes: 35,
            },
            OdysseyNode {
                id: "chem-102".into(),
                unit_id: "unit-1-electromagnetism".into(),
                unit_title: "Unit 1: Electromagnetism & Potentials".into(),
                title: "Electrochemistry & Nernst Equation".into(),
                subject: "Chemistry".into(),
                description: "Cell potential, standard hydrogen electrode, and Gibbs free energy."
                    .into(),
                node_type: NodeType::NumericalDrill,
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 220,
                stars_earned: 0,
                is_crown_mastered: false,
                estimated_minutes: 40,
            },
            OdysseyNode {
                id: "phy-103".into(),
                unit_id: "unit-2-optics-waves".into(),
                unit_title: "Unit 2: Wave Optics & Radiation".into(),
                title: "Wave Optics & Huygens Principle".into(),
                subject: "Physics".into(),
                description:
                    "Wavefront propagation, secondary wavelets, reflection & refraction proof."
                        .into(),
                node_type: NodeType::ReviewChest,
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 280,
                stars_earned: 0,
                is_crown_mastered: false,
                estimated_minutes: 45,
            },
            OdysseyNode {
                id: "math-104".into(),
                unit_id: "unit-2-optics-waves".into(),
                unit_title: "Unit 2: Wave Optics & Radiation".into(),
                title: "Differential Equations in Oscillations".into(),
                subject: "Mathematics".into(),
                description: "Second order homogeneous ODEs and harmonic resonance models.".into(),
                node_type: NodeType::UnitBoss,
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 300,
                stars_earned: 0,
                is_crown_mastered: false,
                estimated_minutes: 50,
            },
        ];

        let trophies = vec![
            TrophyCard {
                id: "first-embark".into(),
                title: "The First Step".into(),
                description: "Embark on your very first MindSet study journey.".into(),
                icon: "🚀".into(),
                unlocked: true,
            },
            TrophyCard {
                id: "gauss-master".into(),
                title: "Flux Pioneer".into(),
                description: "Mastered Gauss's Law with >= 80% marks.".into(),
                icon: "⚡".into(),
                unlocked: true,
            },
            TrophyCard {
                id: "streak-warrior".into(),
                title: "Focus Titan".into(),
                description: "Maintained a 14-day study streak.".into(),
                icon: "🔥".into(),
                unlocked: false,
            },
            TrophyCard {
                id: "polymath".into(),
                title: "Syllabus Conqueror".into(),
                description: "Complete all stages across Physics, Chemistry, and Math.".into(),
                icon: "👑".into(),
                unlocked: false,
            },
        ];

        let mut state = Self {
            nodes,
            trophies,
            current_stage_index: 0,
            total_xp: 250,
            level: 1,
        };
        state.recalculate();
        state
    }

    /// Recalculates total XP, level, and marks.
    pub fn recalculate(&mut self) {
        let earned_xp: u32 = self
            .nodes
            .iter()
            .filter(|n| n.status == NodeStatus::Completed)
            .map(|n| n.xp)
            .sum();

        self.total_xp = earned_xp + 250; // base onboarding XP
        self.level = (self.total_xp / 200).max(1);

        // Check polymath trophy
        let all_completed = self.nodes.iter().all(|n| n.status == NodeStatus::Completed);
        if all_completed {
            if let Some(trophy) = self.trophies.iter_mut().find(|t| t.id == "polymath") {
                trophy.unlocked = true;
            }
        }
    }

    /// Complete current stage with evaluation score and unlock next stage.
    pub fn complete_stage_with_score(&mut self, node_id: &str, marks: u32) -> bool {
        let mut completed_idx = None;
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.id == node_id && node.status == NodeStatus::InProgress {
                node.status = NodeStatus::Completed;
                node.update_evaluation(marks);
                completed_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = completed_idx {
            // Unlock next node if exists
            if idx + 1 < self.nodes.len() {
                self.nodes[idx + 1].status = NodeStatus::InProgress;
                self.current_stage_index = idx + 1;
            }
            self.recalculate();
            true
        } else {
            false
        }
    }

    /// Complete current stage and unlock next stage.
    pub fn complete_stage(&mut self, node_id: &str) -> bool {
        let max_marks = self
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|n| n.max_marks)
            .unwrap_or(100);
        self.complete_stage_with_score(node_id, max_marks)
    }

    /// Embark on a specific unlocked node.
    pub fn embark(&mut self, node_id: &str) -> bool {
        if let Some(idx) = self.nodes.iter().position(|n| n.id == node_id) {
            if self.nodes[idx].status != NodeStatus::Locked {
                self.current_stage_index = idx;
                return true;
            }
        }
        false
    }

    /// Calculate aggregate marks ratio (percentage 0..=100)
    pub fn calculate_marks_percentage(&self) -> u32 {
        let total_marks: u32 = self.nodes.iter().map(|n| n.marks).sum();
        let max_marks: u32 = self.nodes.iter().map(|n| n.max_marks).sum();
        if max_marks == 0 {
            0
        } else {
            ((total_marks as f32 / max_marks as f32) * 100.0).round() as u32
        }
    }

    /// Calculate syllabus completion progress ratio (0.0..=1.0)
    pub fn overall_progress_ratio(&self) -> f32 {
        let total = self.nodes.len();
        if total == 0 {
            0.0
        } else {
            let completed = self
                .nodes
                .iter()
                .filter(|n| n.status == NodeStatus::Completed)
                .count();
            completed as f32 / total as f32
        }
    }

    /// Calculate syllabus completion percentage (0..=100)
    pub fn overall_progress_pct(&self) -> u32 {
        (self.overall_progress_ratio() * 100.0).round() as u32
    }

    /// Number of completed nodes
    pub fn completed_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.status == NodeStatus::Completed)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odyssey_initialization() {
        let state = OdysseyJourneyState::new();
        assert_eq!(state.nodes.len(), 4);
        assert_eq!(state.nodes[0].status, NodeStatus::InProgress);
        assert_eq!(state.nodes[1].status, NodeStatus::Locked);
        assert!(state.total_xp >= 250);
        assert_eq!(state.level, 1);
    }

    #[test]
    fn test_stage_progression() {
        let mut state = OdysseyJourneyState::new();
        assert_eq!(state.current_stage_index, 0);

        let success = state.complete_stage("phy-101");
        assert!(success);
        assert_eq!(state.nodes[0].status, NodeStatus::Completed);
        assert_eq!(state.nodes[1].status, NodeStatus::InProgress);
        assert_eq!(state.current_stage_index, 1);
        assert!(state.total_xp > 250);

        // Cannot complete already locked or completed node
        assert!(!state.complete_stage("phy-101"));
        assert!(!state.complete_stage("math-104"));
    }

    #[test]
    fn test_trophy_polymath_unlock() {
        let mut state = OdysseyJourneyState::new();
        for i in 0..state.nodes.len() {
            let id = state.nodes[i].id.clone();
            state.nodes[i].status = NodeStatus::InProgress;
            state.complete_stage(&id);
        }
        let polymath = state.trophies.iter().find(|t| t.id == "polymath").unwrap();
        assert!(polymath.unlocked);
        assert_eq!(state.overall_progress_pct(), 100);
        assert_eq!(state.completed_count(), 4);
    }

    #[test]
    fn test_node_mastery_and_estimates() {
        let state = OdysseyJourneyState::new();
        let node0 = &state.nodes[0];
        assert_eq!(node0.mastery_percentage(), 85);
        assert_eq!(node0.estimated_time_display(), "35 mins");
        assert_eq!(state.overall_progress_pct(), 0);
    }

    #[test]
    fn test_multi_chapter_units_and_node_types() {
        let state = OdysseyJourneyState::new();
        assert_eq!(state.nodes.len(), 4);
        assert_eq!(state.nodes[0].unit_id, "unit-1-electromagnetism");
        assert_eq!(state.nodes[0].node_type, NodeType::Theory);
        assert_eq!(state.nodes[1].unit_id, "unit-1-electromagnetism");
        assert_eq!(state.nodes[1].node_type, NodeType::NumericalDrill);
        assert_eq!(state.nodes[2].unit_id, "unit-2-optics-waves");
        assert_eq!(state.nodes[2].node_type, NodeType::ReviewChest);
        assert_eq!(state.nodes[3].unit_id, "unit-2-optics-waves");
        assert_eq!(state.nodes[3].node_type, NodeType::UnitBoss);

        assert_eq!(state.nodes[0].node_type.display_badge(), "Theory");
        assert_eq!(state.nodes[0].node_type.icon(), "📖");
        assert_eq!(state.nodes[1].node_type.display_badge(), "Numerical Drill");
        assert_eq!(state.nodes[2].node_type.display_badge(), "Review Chest");
        assert_eq!(state.nodes[3].node_type.display_badge(), "Unit Boss");
    }

    #[test]
    fn test_3_star_and_state_board_crown_mastery() {
        assert_eq!(OdysseyNode::compute_stars(55, 100), 0);
        assert_eq!(OdysseyNode::compute_stars(60, 100), 1);
        assert_eq!(OdysseyNode::compute_stars(79, 100), 1);
        assert_eq!(OdysseyNode::compute_stars(80, 100), 2);
        assert_eq!(OdysseyNode::compute_stars(94, 100), 2);
        assert_eq!(OdysseyNode::compute_stars(95, 100), 3);
        assert_eq!(OdysseyNode::compute_stars(100, 100), 3);

        assert!(!OdysseyNode::compute_crown(94, 100));
        assert!(!OdysseyNode::compute_crown(97, 100));
        assert!(OdysseyNode::compute_crown(98, 100));
        assert!(OdysseyNode::compute_crown(100, 100));

        let mut node = OdysseyNode {
            id: "test-node".into(),
            unit_id: "u1".into(),
            unit_title: "Unit 1".into(),
            title: "Test Topic".into(),
            subject: "Physics".into(),
            description: "Desc".into(),
            node_type: NodeType::Theory,
            status: NodeStatus::InProgress,
            marks: 0,
            max_marks: 100,
            xp: 100,
            stars_earned: 0,
            is_crown_mastered: false,
            estimated_minutes: 30,
        };

        node.update_evaluation(99);
        assert_eq!(node.stars_earned, 3);
        assert!(node.is_crown_mastered);
        assert_eq!(node.star_rating_display(), "★★★");

        node.update_evaluation(82);
        assert_eq!(node.stars_earned, 2);
        assert!(!node.is_crown_mastered);
        assert_eq!(node.star_rating_display(), "★★☆");

        node.update_evaluation(65);
        assert_eq!(node.stars_earned, 1);
        assert!(!node.is_crown_mastered);
        assert_eq!(node.star_rating_display(), "★☆☆");

        node.update_evaluation(40);
        assert_eq!(node.stars_earned, 0);
        assert!(!node.is_crown_mastered);
        assert_eq!(node.star_rating_display(), "☆☆☆");
    }

    #[test]
    fn test_complete_stage_with_score_evaluation() {
        let mut state = OdysseyJourneyState::new();
        assert_eq!(state.nodes[0].status, NodeStatus::InProgress);

        let success = state.complete_stage_with_score("phy-101", 99);
        assert!(success);
        assert_eq!(state.nodes[0].status, NodeStatus::Completed);
        assert_eq!(state.nodes[0].marks, 99);
        assert_eq!(state.nodes[0].stars_earned, 3);
        assert!(state.nodes[0].is_crown_mastered);
        assert_eq!(state.nodes[1].status, NodeStatus::InProgress);

        // Next stage completed with 82% (2 stars, no crown)
        let success2 = state.complete_stage_with_score("chem-102", 82);
        assert!(success2);
        assert_eq!(state.nodes[1].status, NodeStatus::Completed);
        assert_eq!(state.nodes[1].marks, 82);
        assert_eq!(state.nodes[1].stars_earned, 2);
        assert!(!state.nodes[1].is_crown_mastered);
        assert_eq!(state.nodes[2].status, NodeStatus::InProgress);
    }
}
