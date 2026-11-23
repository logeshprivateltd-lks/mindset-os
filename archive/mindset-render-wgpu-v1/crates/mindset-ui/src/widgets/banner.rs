//! Dismissible Banner and batched NotificationCenter.

use crate::input::InputState;
use crate::layout::LayoutRect;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    Info,
    Reminder,
    Alert,
}

pub struct Banner {
    pub rect: LayoutRect,
    pub message: String,
    pub kind: BannerKind,
    pub dismissed: bool,
    /// Slide animation progress: 0.0 = hidden, 1.0 = fully visible (drive with Spring1D).
    pub slide_t: f32,
}

impl Banner {
    pub fn new(rect: LayoutRect, message: &str, kind: BannerKind) -> Self {
        Self {
            rect,
            message: message.to_owned(),
            kind,
            dismissed: false,
            slide_t: 0.0,
        }
    }

    /// Dismiss this banner if the pointer was released inside it.
    pub fn dismiss_if_clicked(&mut self, input: &InputState) {
        if input.clicked(self.rect) {
            self.dismissed = true;
        }
    }

    /// Accent color for the banner's left-side indicator strip.
    pub fn accent_color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        match self.kind {
            BannerKind::Info => p.accent_primary,
            BannerKind::Reminder => p.warning,
            BannerKind::Alert => p.error,
        }
    }

    /// Icon symbol for this banner.
    pub fn icon(&self) -> &'static str {
        match self.kind {
            BannerKind::Info => "[i]",
            BannerKind::Reminder => "[*]",
            BannerKind::Alert => "[!]",
        }
    }
}

// ─── NotificationCenter ───────────────────────────────────────────────────────

pub struct NotificationCenter {
    pub is_open: bool,
    pub badge_count: usize,
    pub do_not_disturb: bool,
    pub items: Vec<Banner>,
    pub bell_rect: LayoutRect,
    pub panel_rect: LayoutRect,
}

impl NotificationCenter {
    pub fn new(bell_rect: LayoutRect) -> Self {
        // Panel drops below the bell button (320px wide, up to 400px tall)
        let panel_rect = LayoutRect::new(
            bell_rect.x + bell_rect.w - 320.0,
            bell_rect.y + bell_rect.h + 4.0,
            320.0,
            400.0,
        );
        Self {
            is_open: false,
            badge_count: 0,
            do_not_disturb: false,
            items: Vec::new(),
            bell_rect,
            panel_rect,
        }
    }

    /// Toggle the panel open/closed when the bell is clicked.
    pub fn toggle(&mut self, input: &InputState) {
        if input.clicked(self.bell_rect) {
            self.is_open = !self.is_open;
        }
    }

    /// Push a new notification (ignored when Do Not Disturb is active).
    pub fn add_notification(&mut self, message: &str, kind: BannerKind) {
        if self.do_not_disturb {
            return;
        }
        // Reuse a small rect; layout in the panel is handled by the shell renderer.
        let r = LayoutRect::new(0.0, 0.0, 300.0, 56.0);
        self.items.push(Banner::new(r, message, kind));
        self.badge_count = self.unread_count();
    }

    /// Dismiss all notifications and close the panel.
    pub fn dismiss_all(&mut self) {
        for b in &mut self.items {
            b.dismissed = true;
        }
        self.badge_count = 0;
        self.is_open = false;
    }

    /// Count of non-dismissed notifications.
    pub fn unread_count(&self) -> usize {
        self.items.iter().filter(|b| !b.dismissed).count()
    }
}
