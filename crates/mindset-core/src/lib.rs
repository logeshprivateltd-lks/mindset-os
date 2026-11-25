//! MindSet OS Core Domain Engine
//!
//! 100% Pure Rust domain logic, Pomodoro state machine, Study datasets,
//! Odyssey gamified journey, Window management, and System preferences.
//!
//! Free of any rendering or windowing library dependencies.

pub mod agent;
pub mod engine;
pub mod mindful;
pub mod odyssey;
pub mod pomodoro;
pub mod preferences;
pub mod study_reader;
pub mod tasks;
pub mod today;
pub mod wm;

pub use agent::{
    ChapterRoi, DynamicExamGenerator, ExamIntelligenceReport, ExamProfile, ExamScoutAgent,
    ExamSection, FeynmanGradeResult, FeynmanGrader, FeynmanPrompt, GeneratedBossExam,
    RecallQuestion, RoiQuadrant, SubjectBreakdown,
};
pub use engine::{
    BossBattleSession, BrainResetArcade, CapsulePlanner, CapsuleRank, CapsuleSegment, CapsuleState,
    DailyCapsule, ExcuseRoast, ExcuseRoastEngine, FormulaCard, FuturePostcard,
    FuturePostcardGenerator, PreparationPhase, ProofOfStudyGate, RapidFormulaCheck, SyncPayload,
    TargetCapsulePlan, VerificationScoreBreakdown, VerifiedCapsuleRecord,
};
pub use mindful::{MindfulGuardState, OnboardingWizardState, WizardStep};
pub use odyssey::{
    NodeStatus, NodeType, OdysseyJourneyState, OdysseyNode, StageAction, TrophyCard,
};
pub use pomodoro::{PomodoroEngine, POMODORO_PRESETS};
pub use preferences::{
    NotificationAction, NotificationSeverity, OsNotificationBus, PreferencesTab, SystemPreferences,
    ThemeMode, ToastMessage, ToastQueue,
};
pub use study_reader::{StudyReaderState, StudyTopic, STUDY_TOPICS};
pub use tasks::{SubjectQuota, TaskItem, TaskQuotaStore};
pub use today::{FutureSelfSnapshot, RingProgress, StreakSnapshot, TaskSnapshot, TodayState};
pub use wm::{
    ActiveOverlay, DesktopLayoutConfig, DesktopWidgetId, DesktopWidgetPlacement, WindowId,
    WindowInstance, WindowManager, WindowMode,
};

use serde::{Deserialize, Serialize};

/// Aggregated domain state for MindSet OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindSetCoreState {
    pub pomodoro: PomodoroEngine,
    pub today: TodayState,
    pub tasks: TaskQuotaStore,
    pub study_reader: StudyReaderState,
    pub odyssey: OdysseyJourneyState,
    pub mindful_guard: MindfulGuardState,
    pub onboarding: OnboardingWizardState,
    pub preferences: SystemPreferences,
    pub toast_queue: ToastQueue,
    pub wm: WindowManager,
    pub exam_plan: TargetCapsulePlan,
    pub boss_session: Option<BossBattleSession>,
    pub excuse_roast: Option<ExcuseRoast>,
    pub break_arcade: Option<BrainResetArcade>,
    pub active_exam: Option<GeneratedBossExam>,
}

impl Default for MindSetCoreState {
    fn default() -> Self {
        Self::new()
    }
}

impl MindSetCoreState {
    pub fn new() -> Self {
        let default_profile = ExamProfile::tn_class_12_board();
        let exam_plan = CapsulePlanner::generate_plan(&default_profile, 590, 430, 100, 90);

        Self {
            pomodoro: PomodoroEngine::new(25),
            today: TodayState::new(),
            tasks: TaskQuotaStore::new(),
            study_reader: StudyReaderState::new(),
            odyssey: OdysseyJourneyState::new(),
            mindful_guard: MindfulGuardState::new(),
            onboarding: OnboardingWizardState::new(),
            preferences: SystemPreferences::new(),
            toast_queue: ToastQueue::new(),
            wm: WindowManager::new(),
            exam_plan,
            boss_session: None,
            excuse_roast: None,
            break_arcade: None,
            active_exam: None,
        }
    }

    /// Advance simulation clock by dt seconds.
    pub fn tick(&mut self, dt_seconds: u32) {
        // Tick mindful guard unlock timer
        self.mindful_guard.tick(dt_seconds);

        // Tick toast expiration (dt in seconds as float)
        self.toast_queue.tick(dt_seconds as f32);

        // Tick pomodoro engine
        let completed = self.pomodoro.tick(dt_seconds as f32);

        // Tick Break Arcade timer if active
        if let Some(arcade) = &mut self.break_arcade {
            if arcade.tick() {
                self.break_arcade = None;
                if self.wm.active_overlay == ActiveOverlay::BreakArcade {
                    self.wm.active_overlay = ActiveOverlay::None;
                }
            }
        }

        // Tick Excuse Roast timer if active
        if let Some(roast) = &mut self.excuse_roast {
            if roast.seconds_remaining > 0 && !roast.is_unlocked && !roast.has_failed {
                roast.seconds_remaining = roast.seconds_remaining.saturating_sub(dt_seconds as u8);
                if roast.seconds_remaining == 0 {
                    roast.has_failed = true;
                }
            }
        }

        // If a focus interval completed, sync study minutes
        if completed {
            let session_mins = self.pomodoro.duration_mins;
            self.today.log_study_minutes(session_mins);
            self.tasks.log_study_minutes("Physics", session_mins);
            self.toast_queue.push(
                "Focus Complete!",
                format!(
                    "Recorded {}m of deep study. Take a 5m breather.",
                    session_mins
                ),
                "🎉",
            );
        }
    }

    /// Start focus session from the Today screen primary CTA.
    pub fn start_focus_session(&mut self) {
        self.pomodoro.start_focus_session();
        self.wm.open(WindowId::StudyReader);
        self.toast_queue.push(
            "Deep Focus Activated",
            "Gauss's Law session loaded. Mindful Guard is active.",
            "🎯",
        );
    }

    /// Starts the Proof-of-Study Verification Boss battle.
    pub fn start_verification_boss(&mut self) {
        let topic = if let Some(cap) = self
            .exam_plan
            .capsules
            .get(self.exam_plan.current_capsule_index)
        {
            cap.title.clone()
        } else {
            "Electrochemistry & Nernst Equation".to_string()
        };

        let exam = DynamicExamGenerator::generate_for_topic(&topic, 1);
        let session = BossBattleSession::new(&exam.boss_name, &exam.boss_title, &exam.chapter);

        self.active_exam = Some(exam);
        self.boss_session = Some(session);
        self.wm.active_overlay = ActiveOverlay::VerificationBoss;
    }

    /// Triggers Roast My Excuse modal inside MindfulGuard.
    pub fn start_excuse_roast(&mut self, excuse: &str) {
        let roast = ExcuseRoastEngine::generate_roast(excuse);
        self.excuse_roast = Some(roast);
        self.wm.active_overlay = ActiveOverlay::ExcuseRoast;
    }

    /// Unlocks the 2-Minute Brain-Reset Arcade during Pomodoro breaks.
    pub fn start_break_arcade(&mut self) {
        let break_seconds = (self.pomodoro.break_mins * 60).max(120);
        self.break_arcade = Some(BrainResetArcade::new(break_seconds));
        self.wm.active_overlay = ActiveOverlay::BreakArcade;
    }

    /// Opens the Exam Intelligence Report modal.
    pub fn open_exam_report(&mut self) {
        self.wm.active_overlay = ActiveOverlay::ExamReport;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_state_initialization() {
        let state = MindSetCoreState::new();
        assert!(!state.pomodoro.is_running);
        assert!(state.wm.is_open(WindowId::Today));
        assert_eq!(STUDY_TOPICS.len(), 3);
        assert_eq!(state.tasks.subjects.len(), 3);
    }

    #[test]
    fn test_core_focus_flow() {
        let mut state = MindSetCoreState::new();
        state.start_focus_session();
        assert!(state.pomodoro.is_running);
        assert!(state.wm.is_open(WindowId::StudyReader));
        assert_eq!(state.toast_queue.toasts.len(), 1);
    }
}
