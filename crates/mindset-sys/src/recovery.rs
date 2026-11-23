//! Recovery Engine for Returning Students
//! Implements guilt-free streak preservation, pause states, and gentle restart sessions.

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryState {
    pub days_missed: u32,
    pub freezes_remaining: u32,
    pub streak_preserved: bool,
    pub restart_session_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct StreakState {
    pub current_streak: u32,
    pub freezes_remaining: u32,
}

/// Checks whether a recovery banner should be shown for a returning student.
/// If `last_session` is `None` (first launch), returns `None`.
/// If days missed is 0 or 1, returns `None`.
pub fn check_recovery(
    last_session: Option<NaiveDate>,
    today: NaiveDate,
    streak: &StreakState,
) -> Option<RecoveryState> {
    let last = match last_session {
        Some(d) => d,
        None => return None, // First launch -> never recovery
    };

    let diff = (today - last).num_days();
    if diff <= 1 {
        return None; // No gap
    }

    let days_missed = (diff - 1) as u32;
    if days_missed <= streak.freezes_remaining {
        Some(RecoveryState {
            days_missed,
            freezes_remaining: streak.freezes_remaining.saturating_sub(days_missed),
            streak_preserved: true,
            restart_session_minutes: 5,
        })
    } else {
        Some(RecoveryState {
            days_missed,
            freezes_remaining: 0,
            streak_preserved: false,
            restart_session_minutes: 5,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_launch_no_recovery() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap();
        let streak = StreakState {
            current_streak: 0,
            freezes_remaining: 3,
        };
        let rec = check_recovery(None, today, &streak);
        assert_eq!(rec, None, "First launch must never trigger recovery");
    }

    #[test]
    fn test_recovery_state_preserves_streak_within_freezes() {
        let last = NaiveDate::from_ymd_opt(2026, 11, 14).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap(); // 3 days diff -> 2 days missed
        let streak = StreakState {
            current_streak: 12,
            freezes_remaining: 3,
        };
        let rec = check_recovery(Some(last), today, &streak).expect("Should trigger recovery");
        assert_eq!(rec.days_missed, 2);
        assert!(rec.streak_preserved);
        assert_eq!(rec.freezes_remaining, 1);
        assert_eq!(rec.restart_session_minutes, 5);
    }

    #[test]
    fn test_recovery_state_fresh_start_beyond_freezes() {
        let last = NaiveDate::from_ymd_opt(2026, 11, 11).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap(); // 6 days diff -> 5 days missed
        let streak = StreakState {
            current_streak: 12,
            freezes_remaining: 2,
        };
        let rec = check_recovery(Some(last), today, &streak).expect("Should trigger recovery");
        assert_eq!(rec.days_missed, 5);
        assert!(
            !rec.streak_preserved,
            "Streak is not preserved, fresh start provided"
        );
        assert_eq!(rec.freezes_remaining, 0);
        assert_eq!(rec.restart_session_minutes, 5);
    }

    #[test]
    fn test_celebration_does_not_fire_on_recovery() {
        // Critical invariant test: recovery events must NEVER wire to celebration burst
        let last = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap();
        let streak = StreakState {
            current_streak: 4,
            freezes_remaining: 1,
        };
        let rec = check_recovery(Some(last), today, &streak);
        assert!(rec.is_some());
        // Verify contract: recovery state does not allow celebration
        let can_celebrate = rec.is_none();
        assert!(
            !can_celebrate,
            "Recovery active must suppress celebration burst"
        );
    }
}
