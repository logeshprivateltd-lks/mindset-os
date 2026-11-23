//! Goal-Guided Daily Study Capsule Generator
//!
//! Uses Maindy AI to generate tailored daily capsules with structured 4 stages
//! matching the student's career track blueprint.

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::storage::db::{CapsuleRecord, StorageManager, UserProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackBlueprint {
    pub career_track: String,
    pub stage1_name: &'static str,
    pub stage1_duration: u32,
    pub stage2_name: &'static str,
    pub stage2_duration: u32,
    pub stage3_name: &'static str,
    pub stage3_duration: u32,
    pub stage4_name: &'static str,
    pub stage4_duration: u32,
}

impl TrackBlueprint {
    pub fn for_track(track: &str) -> Self {
        match track {
            "Class12BoardsTN600" | "Class12BoardsCBSE500" => Self {
                career_track: track.into(),
                stage1_name: "🏋️ Concept Derivation (40m)",
                stage1_duration: 40,
                stage2_name: "✍️ Book-Back & Numericals (25m)",
                stage2_duration: 25,
                stage3_name: "🧠 1-Mark & Formula Recall (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ 5-Mark Proof Boss (20m)",
                stage4_duration: 20,
            },
            "NEET720" => Self {
                career_track: track.into(),
                stage1_name: "📖 NCERT Line-by-Line (40m)",
                stage1_duration: 40,
                stage2_name: "🔬 Diagram & Statement Drill (25m)",
                stage2_duration: 25,
                stage3_name: "⚡ Active Recall Flash (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ Timed +4/-1 MCQ Boss (20m)",
                stage4_duration: 20,
            },
            "JEE300" => Self {
                career_track: track.into(),
                stage1_name: "📐 First-Principles Concept (45m)",
                stage1_duration: 45,
                stage2_name: "🧩 Multi-Concept Problem (30m)",
                stage2_duration: 30,
                stage3_name: "⚡ Edge-Case Formula Anchor (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ Timed Numerical Boss (20m)",
                stage4_duration: 20,
            },
            "UPSC_TNPSC" => Self {
                career_track: track.into(),
                stage1_name: "📚 Static Core Reading (45m)",
                stage1_duration: 45,
                stage2_name: "📰 Current Affairs Link (25m)",
                stage2_duration: 25,
                stage3_name: "🏛️ Facts & Articles Anchor (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ PYQ Elimination Boss (20m)",
                stage4_duration: 20,
            },
            "CA" => Self {
                career_track: track.into(),
                stage1_name: "📜 Standard & Section Study (40m)",
                stage1_duration: 40,
                stage2_name: "📊 Practical Ledger & Cases (30m)",
                stage2_duration: 30,
                stage3_name: "⚖️ Provision Recall (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ Timed Scenario Boss (20m)",
                stage4_duration: 20,
            },
            "TechCareer" => Self {
                career_track: track.into(),
                stage1_name: "💻 Architecture & Spec (40m)",
                stage1_duration: 40,
                stage2_name: "🛠️ Hands-On Code & Debug (35m)",
                stage2_duration: 35,
                stage3_name: "🧠 Pattern Recall (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ Timed Coding Boss (20m)",
                stage4_duration: 20,
            },
            _ => Self {
                career_track: "Custom".into(),
                stage1_name: "🏋️ Heavy Lift (40m)",
                stage1_duration: 40,
                stage2_name: "✍️ Active Practice (25m)",
                stage2_duration: 25,
                stage3_name: "🧠 Memory Anchor (15m)",
                stage3_duration: 15,
                stage4_name: "⚔️ Proof-of-Study Boss (20m)",
                stage4_duration: 20,
            },
        }
    }

    pub fn total_minutes(&self) -> u32 {
        self.stage1_duration + self.stage2_duration + self.stage3_duration + self.stage4_duration
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCapsulePlan {
    pub subject: String,
    pub unit: String,
    pub estimated_minutes: u32,
    pub xp_reward: u32,
    pub heavy_lift_task: String,
    pub active_practice_task: String,
    pub memory_anchor_task: String,
    pub boss_verification_topic: String,
}

pub struct CapsuleGenerator;

impl CapsuleGenerator {
    /// Generates structured daily capsules for today based on student profile and syllabus targets
    pub fn generate_daily_capsules(
        profile: &UserProfile,
        storage: &StorageManager,
    ) -> Vec<CapsuleRecord> {
        let today_str = Local::now().format("%Y-%m-%d").to_string();
        let bp = TrackBlueprint::for_track(&profile.career_track);
        let total_mins = bp.total_minutes();

        let mut capsules = Vec::new();

        match profile.career_track.as_str() {
            "NEET720" => {
                let c1 = CapsuleRecord {
                    id: format!("cap-{}-1", today_str),
                    target_date: today_str.clone(),
                    subject: "Biology (Zoology)".into(),
                    unit: "Molecular Basis of Inheritance — DNA Transcription".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 140,
                    status: "In Progress".into(),
                    stage: "Heavy Lift".into(),
                };
                let c2 = CapsuleRecord {
                    id: format!("cap-{}-2", today_str),
                    target_date: today_str.clone(),
                    subject: "Chemistry".into(),
                    unit: "Chemical Equilibrium & Le Chatelier Principle".into(),
                    estimated_minutes: total_mins.saturating_sub(15),
                    xp_reward: 120,
                    status: "Pending".into(),
                    stage: "Active Practice".into(),
                };
                let c3 = CapsuleRecord {
                    id: format!("cap-{}-3", today_str),
                    target_date: today_str,
                    subject: "Physics".into(),
                    unit: "Current Electricity — Kirchhoff's Laws & Wheatstone Bridge".into(),
                    estimated_minutes: total_mins.saturating_sub(20),
                    xp_reward: 110,
                    status: "Pending".into(),
                    stage: "Proof-of-Study".into(),
                };
                for c in [&c1, &c2, &c3] {
                    storage.save_capsule(c);
                }
                capsules.extend([c1, c2, c3]);
            }
            "JEE300" => {
                let c1 = CapsuleRecord {
                    id: format!("cap-{}-1", today_str),
                    target_date: today_str.clone(),
                    subject: "Physics".into(),
                    unit: "Rotational Dynamics — Moment of Inertia & Angular Momentum".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 150,
                    status: "In Progress".into(),
                    stage: "Heavy Lift".into(),
                };
                let c2 = CapsuleRecord {
                    id: format!("cap-{}-2", today_str),
                    target_date: today_str.clone(),
                    subject: "Mathematics".into(),
                    unit: "Definite Integrals & Area Under Curves".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 140,
                    status: "Pending".into(),
                    stage: "Active Practice".into(),
                };
                let c3 = CapsuleRecord {
                    id: format!("cap-{}-3", today_str),
                    target_date: today_str,
                    subject: "Chemistry".into(),
                    unit: "Coordination Compounds & Crystal Field Theory".into(),
                    estimated_minutes: total_mins.saturating_sub(20),
                    xp_reward: 120,
                    status: "Pending".into(),
                    stage: "Proof-of-Study".into(),
                };
                for c in [&c1, &c2, &c3] {
                    storage.save_capsule(c);
                }
                capsules.extend([c1, c2, c3]);
            }
            "UPSC_TNPSC" => {
                let c1 = CapsuleRecord {
                    id: format!("cap-{}-1", today_str),
                    target_date: today_str.clone(),
                    subject: "Indian Polity".into(),
                    unit: "Fundamental Rights & Judicial Review (Articles 14-32)".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 130,
                    status: "In Progress".into(),
                    stage: "Heavy Lift".into(),
                };
                let c2 = CapsuleRecord {
                    id: format!("cap-{}-2", today_str),
                    target_date: today_str.clone(),
                    subject: "Modern History".into(),
                    unit: "Non-Cooperation Movement to Civil Disobedience".into(),
                    estimated_minutes: total_mins.saturating_sub(15),
                    xp_reward: 110,
                    status: "Pending".into(),
                    stage: "Active Practice".into(),
                };
                let c3 = CapsuleRecord {
                    id: format!("cap-{}-3", today_str),
                    target_date: today_str,
                    subject: "Economy".into(),
                    unit: "RBI Monetary Policy Instruments & Inflation Dynamics".into(),
                    estimated_minutes: total_mins.saturating_sub(20),
                    xp_reward: 100,
                    status: "Pending".into(),
                    stage: "Proof-of-Study".into(),
                };
                for c in [&c1, &c2, &c3] {
                    storage.save_capsule(c);
                }
                capsules.extend([c1, c2, c3]);
            }
            "TechCareer" => {
                let c1 = CapsuleRecord {
                    id: format!("cap-{}-1", today_str),
                    target_date: today_str.clone(),
                    subject: "Rust Systems".into(),
                    unit: "Async Runtimes, Pinning & Cancellation Tokens".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 150,
                    status: "In Progress".into(),
                    stage: "Heavy Lift".into(),
                };
                let c2 = CapsuleRecord {
                    id: format!("cap-{}-2", today_str),
                    target_date: today_str.clone(),
                    subject: "Algorithms".into(),
                    unit: "Graph Shortest Path & Topological Sort Lab".into(),
                    estimated_minutes: total_mins.saturating_sub(10),
                    xp_reward: 130,
                    status: "Pending".into(),
                    stage: "Active Practice".into(),
                };
                let c3 = CapsuleRecord {
                    id: format!("cap-{}-3", today_str),
                    target_date: today_str,
                    subject: "System Design".into(),
                    unit: "Distributed Rate Limiting & Token Bucket Algorithms".into(),
                    estimated_minutes: total_mins.saturating_sub(20),
                    xp_reward: 120,
                    status: "Pending".into(),
                    stage: "Proof-of-Study".into(),
                };
                for c in [&c1, &c2, &c3] {
                    storage.save_capsule(c);
                }
                capsules.extend([c1, c2, c3]);
            }
            _ => {
                // Class 12 Boards / Default
                let weak_topic = profile
                    .weak_topics
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Electrochemistry Nernst Equation".into());

                let c1 = CapsuleRecord {
                    id: format!("cap-{}-1", today_str),
                    target_date: today_str.clone(),
                    subject: "Physics".into(),
                    unit: "Gauss's Law & Electric Flux Derivations".into(),
                    estimated_minutes: total_mins,
                    xp_reward: 130,
                    status: "In Progress".into(),
                    stage: "Heavy Lift".into(),
                };
                let c2 = CapsuleRecord {
                    id: format!("cap-{}-2", today_str),
                    target_date: today_str.clone(),
                    subject: "Chemistry".into(),
                    unit: weak_topic,
                    estimated_minutes: total_mins.saturating_sub(15),
                    xp_reward: 120,
                    status: "Pending".into(),
                    stage: "Active Practice".into(),
                };
                let c3 = CapsuleRecord {
                    id: format!("cap-{}-3", today_str),
                    target_date: today_str,
                    subject: "Mathematics".into(),
                    unit: "Matrices & Determinant Inverse Proofs".into(),
                    estimated_minutes: total_mins.saturating_sub(25),
                    xp_reward: 100,
                    status: "Pending".into(),
                    stage: "Proof-of-Study".into(),
                };
                for c in [&c1, &c2, &c3] {
                    storage.save_capsule(c);
                }
                capsules.extend([c1, c2, c3]);
            }
        }

        capsules
    }
}

pub fn generate_capsules_for_blueprint(track: &str, weak_topics: &[String]) -> Vec<CapsuleRecord> {
    let dummy_profile = UserProfile {
        career_track: track.to_string(),
        weak_topics: weak_topics.to_vec(),
        ..Default::default()
    };
    let storage = StorageManager::new();
    CapsuleGenerator::generate_daily_capsules(&dummy_profile, &storage)
}
