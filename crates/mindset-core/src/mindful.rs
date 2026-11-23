//! Mindful Guard distraction shield and Onboarding Wizard models for MindSet OS.

use serde::{Deserialize, Serialize};

/// State of the distraction-blocking Mindful Guard modal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindfulGuardState {
    pub is_active: bool,
    pub target_app_or_site: String,
    pub temporary_unlock_remaining_secs: u32,
    pub blocked_count: u32,
    pub reason: String,
}

impl Default for MindfulGuardState {
    fn default() -> Self {
        Self::new()
    }
}

impl MindfulGuardState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            target_app_or_site: "Distracting Media / Social Feed".into(),
            temporary_unlock_remaining_secs: 0,
            blocked_count: 0,
            reason: "Current focus session is active. Guard your attention.".into(),
        }
    }

    /// Triggers an interception for a target app or website.
    pub fn intercept(&mut self, target: &str) {
        if self.temporary_unlock_remaining_secs == 0 {
            self.target_app_or_site = target.to_string();
            self.is_active = true;
            self.blocked_count += 1;
        }
    }

    /// User chose "Stay Focused": closes modal and honors session.
    pub fn stay_focused(&mut self) {
        self.is_active = false;
    }

    /// User chose "Unlock 5m": grants 300s window.
    pub fn unlock_5m(&mut self) {
        self.is_active = false;
        self.temporary_unlock_remaining_secs = 300;
    }

    /// Ticks the 5m unlock countdown timer.
    pub fn tick(&mut self, seconds: u32) {
        if self.temporary_unlock_remaining_secs > 0 {
            self.temporary_unlock_remaining_secs =
                self.temporary_unlock_remaining_secs.saturating_sub(seconds);
        }
    }

    pub fn is_unlocked(&self) -> bool {
        self.temporary_unlock_remaining_secs > 0
    }
}

/// 5-step interactive onboarding setup wizard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WizardStep {
    Language = 0,
    Identity = 1,
    SubjectGoals = 2,
    FocusMode = 3,
    Ready = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingWizardState {
    pub is_open: bool,
    pub current_step: WizardStep,
    pub selected_language: String,
    pub student_name: String,
    pub target_exam: String,
    pub daily_goal_hours: u32,
    pub strictness_mode: String,
    pub completed: bool,
}

impl Default for OnboardingWizardState {
    fn default() -> Self {
        Self::new()
    }
}

impl OnboardingWizardState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            current_step: WizardStep::Language,
            selected_language: "English (US)".into(),
            student_name: "Aspirant".into(),
            target_exam: "JEE Advanced / NEET".into(),
            daily_goal_hours: 4,
            strictness_mode: "Strict Focus".into(),
            completed: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn next_step(&mut self) {
        match self.current_step {
            WizardStep::Language => self.current_step = WizardStep::Identity,
            WizardStep::Identity => self.current_step = WizardStep::SubjectGoals,
            WizardStep::SubjectGoals => self.current_step = WizardStep::FocusMode,
            WizardStep::FocusMode => self.current_step = WizardStep::Ready,
            WizardStep::Ready => {
                self.completed = true;
                self.is_open = false;
            }
        }
    }

    pub fn prev_step(&mut self) {
        match self.current_step {
            WizardStep::Language => {}
            WizardStep::Identity => self.current_step = WizardStep::Language,
            WizardStep::SubjectGoals => self.current_step = WizardStep::Identity,
            WizardStep::FocusMode => self.current_step = WizardStep::SubjectGoals,
            WizardStep::Ready => self.current_step = WizardStep::FocusMode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mindful_guard_lifecycle() {
        let mut guard = MindfulGuardState::new();
        assert!(!guard.is_active);
        assert!(!guard.is_unlocked());

        guard.intercept("youtube.com");
        assert!(guard.is_active);
        assert_eq!(guard.blocked_count, 1);

        // Unlock 5 minutes
        guard.unlock_5m();
        assert!(!guard.is_active);
        assert!(guard.is_unlocked());
        assert_eq!(guard.temporary_unlock_remaining_secs, 300);

        // Intercept while unlocked should do nothing
        guard.intercept("youtube.com");
        assert!(!guard.is_active);

        // Tick down
        guard.tick(200);
        assert_eq!(guard.temporary_unlock_remaining_secs, 100);
        guard.tick(150);
        assert_eq!(guard.temporary_unlock_remaining_secs, 0);
        assert!(!guard.is_unlocked());
    }

    #[test]
    fn test_wizard_steps() {
        let mut wizard = OnboardingWizardState::new();
        assert_eq!(wizard.current_step, WizardStep::Language);
        wizard.next_step();
        assert_eq!(wizard.current_step, WizardStep::Identity);
        wizard.next_step();
        assert_eq!(wizard.current_step, WizardStep::SubjectGoals);
        wizard.prev_step();
        assert_eq!(wizard.current_step, WizardStep::Identity);
    }
}
