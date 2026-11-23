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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyNode {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub status: NodeStatus,
    pub marks: u32,
    pub max_marks: u32,
    pub xp: u32,
    #[serde(default = "default_estimated_minutes")]
    pub estimated_minutes: u32,
}

fn default_estimated_minutes() -> u32 {
    35
}

impl OdysseyNode {
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
                title: "Electrostatics & Gauss's Law".into(),
                subject: "Physics".into(),
                description:
                    "Electric flux, closed Gaussian surfaces, and Coulomb field derivation.".into(),
                status: NodeStatus::InProgress,
                marks: 85,
                max_marks: 100,
                xp: 250,
                estimated_minutes: 35,
            },
            OdysseyNode {
                id: "chem-102".into(),
                title: "Electrochemistry & Nernst Equation".into(),
                subject: "Chemistry".into(),
                description: "Cell potential, standard hydrogen electrode, and Gibbs free energy."
                    .into(),
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 220,
                estimated_minutes: 40,
            },
            OdysseyNode {
                id: "phy-103".into(),
                title: "Wave Optics & Huygens Principle".into(),
                subject: "Physics".into(),
                description:
                    "Wavefront propagation, secondary wavelets, reflection & refraction proof."
                        .into(),
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 280,
                estimated_minutes: 45,
            },
            OdysseyNode {
                id: "math-104".into(),
                title: "Differential Equations in Oscillations".into(),
                subject: "Mathematics".into(),
                description: "Second order homogeneous ODEs and harmonic resonance models.".into(),
                status: NodeStatus::Locked,
                marks: 0,
                max_marks: 100,
                xp: 300,
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

    /// Complete current stage and unlock next stage.
    pub fn complete_stage(&mut self, node_id: &str) -> bool {
        let mut completed_idx = None;
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.id == node_id && node.status == NodeStatus::InProgress {
                node.status = NodeStatus::Completed;
                node.marks = node.max_marks;
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
            let completed = self.nodes.iter().filter(|n| n.status == NodeStatus::Completed).count();
            completed as f32 / total as f32
        }
    }

    /// Calculate syllabus completion percentage (0..=100)
    pub fn overall_progress_pct(&self) -> u32 {
        (self.overall_progress_ratio() * 100.0).round() as u32
    }

    /// Number of completed nodes
    pub fn completed_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.status == NodeStatus::Completed).count()
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
}
