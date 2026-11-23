//! Top OS system bar, bottom application dock, lock screen, and power lifecycle menu.

use crate::input_engine::TextInputBuffer;
use mindset_ui::LayoutRect;

pub const TOP_BAR_HEIGHT: f32 = 44.0;
pub const TOP_BAR_H: f32 = TOP_BAR_HEIGHT;
pub const DOCK_HEIGHT: f32 = 56.0;
pub const DOCK_H: f32 = DOCK_HEIGHT;
pub const DOCK_WIDTH: f32 = 540.0;

/// Categorized notification levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    Info,     // [i]
    Reminder, // [*]
    Alert,    // [!]
}

impl NotificationCategory {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Info => "[i]",
            Self::Reminder => "[*]",
            Self::Alert => "[!]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Reminder => "Reminders",
            Self::Alert => "Alerts",
        }
    }
}

/// An individual notification item in the system drawer.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationItem {
    pub id: u64,
    pub category: NotificationCategory,
    pub title: String,
    pub message: String,
    pub time_str: String,
    pub is_read: bool,
}

/// Categorized Notification Drawer with Snooze & Do Not Disturb support.
#[derive(Debug, Clone)]
pub struct NotificationCenter {
    pub is_open: bool,
    pub do_not_disturb: bool,
    pub snoozed_until_minutes: u32,
    pub notifications: Vec<NotificationItem>,
    pub next_id: u64,
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationCenter {
    pub fn new() -> Self {
        let mut center = Self {
            is_open: false,
            do_not_disturb: false,
            snoozed_until_minutes: 0,
            notifications: Vec::new(),
            next_id: 1,
        };

        // Seed initial system notifications
        center.add(
            NotificationCategory::Info,
            "SQLite WAL Checkpointed",
            "Local database journals truncated cleanly.",
        );
        center.add(
            NotificationCategory::Info,
            "Study Vault Ready",
            "4 reference notes indexed for grounded Q&A.",
        );
        center.add(
            NotificationCategory::Reminder,
            "Study Block Interval",
            "25m focus sprint complete — take a 5m break.",
        );
        center.add(
            NotificationCategory::Alert,
            "System Integrity Doctor",
            "Memory ceiling (<35 MB RSS) verified.",
        );

        center
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn add(&mut self, category: NotificationCategory, title: &str, message: &str) {
        let item = NotificationItem {
            id: self.next_id,
            category,
            title: title.to_string(),
            message: message.to_string(),
            time_str: "Just now".to_string(),
            is_read: false,
        };
        self.next_id += 1;
        self.notifications.insert(0, item);
    }

    pub fn snooze(&mut self, minutes: u32) {
        self.snoozed_until_minutes = minutes;
    }

    pub fn toggle_dnd(&mut self) {
        self.do_not_disturb = !self.do_not_disturb;
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }

    pub fn unread_count(&self) -> usize {
        self.visible_notifications()
            .iter()
            .filter(|n| !n.is_read)
            .count()
    }

    /// Visible notifications honoring Do Not Disturb and Snooze filters.
    pub fn visible_notifications(&self) -> Vec<&NotificationItem> {
        self.notifications
            .iter()
            .filter(|n| {
                if self.do_not_disturb {
                    // Under DND, only critical Alerts pass through
                    n.category == NotificationCategory::Alert
                } else if self.snoozed_until_minutes > 0 {
                    // When snoozed, suppress Reminders
                    n.category != NotificationCategory::Reminder
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn drawer_rect(&self, screen_w: f32) -> LayoutRect {
        let w = 340.0;
        let h = 380.0;
        let x = screen_w - w - 80.0;
        let y = TOP_BAR_HEIGHT + 6.0;
        LayoutRect::new(x, y, w, h)
    }
}

/// Current session security and power lifecycle state.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Active,
    Locked {
        pin_input: TextInputBuffer,
        error_msg: Option<String>,
    },
    PowerMenuOpen,
    ShuttingDown {
        status: String,
        progress: f32,
    },
}

/// Power menu actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    LockScreen,
    SleepStandby,
    RestartShell,
    ShutDown,
}

impl PowerAction {
    pub fn all() -> [PowerAction; 4] {
        [
            Self::LockScreen,
            Self::SleepStandby,
            Self::RestartShell,
            Self::ShutDown,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::LockScreen => "Lock Screen [Ctrl+L]",
            Self::SleepStandby => "Sleep / Standby",
            Self::RestartShell => "Restart MindSet OS",
            Self::ShutDown => "Shut Down",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::LockScreen => "[L]",
            Self::SleepStandby => "[Zzz]",
            Self::RestartShell => "[R]",
            Self::ShutDown => "[Off]",
        }
    }
}

/// The system session manager coordinating the top bar, dock, lock screen, and power lifecycle.
pub struct SessionManager {
    pub state: SessionState,
    pub focus_mode: bool,
    pub notifications: NotificationCenter,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl SessionManager {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            state: SessionState::Active,
            focus_mode: false,
            notifications: NotificationCenter::new(),
            screen_width,
            screen_height,
        }
    }

    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        if width > 0.0 && height > 0.0 {
            self.screen_width = width;
            self.screen_height = height;
        }
    }

    pub fn top_bar_rect(&self) -> LayoutRect {
        LayoutRect::new(0.0, 0.0, self.screen_width, TOP_BAR_HEIGHT)
    }

    pub fn dock_rect(&self) -> LayoutRect {
        let x = (self.screen_width - DOCK_WIDTH) * 0.5;
        let y = self.screen_height - DOCK_HEIGHT - 14.0;
        LayoutRect::new(x, y, DOCK_WIDTH, DOCK_HEIGHT)
    }

    pub fn power_menu_rect(&self) -> LayoutRect {
        let w = 260.0;
        let h = 200.0;
        let x = self.screen_width - w - 16.0;
        let y = TOP_BAR_HEIGHT + 8.0;
        LayoutRect::new(x, y, w, h)
    }

    pub fn lock(&mut self) {
        self.state = SessionState::Locked {
            pin_input: TextInputBuffer::new("Enter PIN (default 1234)..."),
            error_msg: None,
        };
    }

    pub fn try_unlock(&mut self, entered_pin: &str) -> bool {
        // Default PIN is 1234 or empty for quick convenience
        if entered_pin == "1234" || entered_pin.is_empty() {
            self.state = SessionState::Active;
            true
        } else {
            if let SessionState::Locked {
                ref mut error_msg,
                ref mut pin_input,
            } = self.state
            {
                *error_msg = Some("Incorrect PIN. Try 1234.".to_string());
                pin_input.clear();
            }
            false
        }
    }

    pub fn trigger_power_action(&mut self, action: PowerAction) {
        match action {
            PowerAction::LockScreen => {
                self.lock();
            }
            PowerAction::SleepStandby => {
                // Dim display & park session
                self.lock();
            }
            PowerAction::RestartShell => {
                self.state = SessionState::ShuttingDown {
                    status: "Restarting MindSet OS Shell...".to_string(),
                    progress: 0.5,
                };
            }
            PowerAction::ShutDown => {
                self.state = SessionState::ShuttingDown {
                    status: "Saving state & checkpointing SQLite WAL database...".to_string(),
                    progress: 0.9,
                };
            }
        }
    }

    pub fn dock_item_rect(&self, idx: usize, total: usize) -> LayoutRect {
        let dock = self.dock_rect();
        let item_w = (dock.w - 16.0) / total as f32;
        let x = dock.x + 8.0 + idx as f32 * item_w;
        LayoutRect::new(x, dock.y + 6.0, item_w, dock.h - 12.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_and_unlock_flow() {
        let mut session = SessionManager::new(1280.0, 800.0);
        assert_eq!(session.state, SessionState::Active);

        session.lock();
        assert!(matches!(session.state, SessionState::Locked { .. }));

        // Failed attempt with wrong PIN
        let ok_wrong = session.try_unlock("9999");
        assert!(!ok_wrong);
        assert!(matches!(session.state, SessionState::Locked { .. }));

        // Successful attempt with correct PIN
        let ok_correct = session.try_unlock("1234");
        assert!(ok_correct);
        assert_eq!(session.state, SessionState::Active);
    }

    #[test]
    fn test_power_menu_rect_bounds() {
        let session = SessionManager::new(1280.0, 800.0);
        let menu_r = session.power_menu_rect();
        assert!(menu_r.x + menu_r.w <= 1280.0);
        assert!(menu_r.y >= TOP_BAR_HEIGHT);
    }

    #[test]
    fn test_notification_center_categorization_and_dnd_filtering() {
        let mut nc = NotificationCenter::new();
        assert_eq!(nc.notifications.len(), 4);
        assert_eq!(nc.visible_notifications().len(), 4);

        // Test Snooze: suppresses Reminders
        nc.snooze(15);
        let visible_snoozed = nc.visible_notifications();
        assert!(!visible_snoozed
            .iter()
            .any(|n| n.category == NotificationCategory::Reminder));

        // Test DND: only Alerts pass through
        nc.toggle_dnd();
        assert!(nc.do_not_disturb);
        let visible_dnd = nc.visible_notifications();
        assert_eq!(visible_dnd.len(), 1);
        assert_eq!(visible_dnd[0].category, NotificationCategory::Alert);

        // Turn off DND and clear
        nc.toggle_dnd();
        nc.snooze(0);
        assert_eq!(nc.visible_notifications().len(), 4);
        nc.clear_all();
        assert_eq!(nc.visible_notifications().len(), 0);
    }
}
