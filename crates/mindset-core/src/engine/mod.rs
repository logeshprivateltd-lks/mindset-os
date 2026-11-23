//! MindSet OS Core Execution Engine
//!
//! Subsystems for Target-to-Capsule planning, strict Proof-of-Study verification,
//! native engagement activities, and local JSON synchronization payloads.

pub mod capsule_planner;
pub mod engagement;
pub mod sync_payload;
pub mod verification;

pub use capsule_planner::{
    CapsulePlanner, CapsuleSegment, DailyCapsule, PreparationPhase, TargetCapsulePlan,
};
pub use engagement::{
    BossBattleSession, BrainResetArcade, ExcuseRoast, ExcuseRoastEngine, FormulaCard,
    FuturePostcard, FuturePostcardGenerator, RapidFormulaCheck,
};
pub use sync_payload::{SyncPayload, VerifiedCapsuleRecord};
pub use verification::{CapsuleRank, CapsuleState, ProofOfStudyGate, VerificationScoreBreakdown};
