//! MINDSET ENGINE — Premium Native Browser UI/UX Components
//! Combines Arc spatial layout, Safari minimalism, Windows 11 Fluent surfaces,
//! Apple Liquid Glass aesthetics, and MindSet OS's Solar Amber identity.
//! 100% Pure Rust + wgpu 24, Zero HTML, Zero CSS, Zero WebViews.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::WidgetState;
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};

// ── MindSet OS Solar Amber & Liquid Glass Color Tokens ───────────────────────
pub const BROWSER_BG_DEEP: [f32; 4] = [0.043, 0.051, 0.071, 0.98]; // #0B0D12 Deep charcoal background
pub const BROWSER_GLASS_RAISED: [f32; 4] = [0.082, 0.098, 0.133, 0.92]; // #151922 Raised glass surfaces
pub const BROWSER_SURFACE_SEC: [f32; 4] = [0.110, 0.133, 0.188, 0.85]; // #1C2230 Secondary surfaces
pub const BROWSER_SOLAR_AMBER: [f32; 4] = [0.898, 0.678, 0.271, 1.0]; // #E5AD45 Solar Amber accent
pub const BROWSER_AMBER_GLOW: [f32; 4] = [0.898, 0.678, 0.271, 0.25]; // Subtle amber glow
pub const BROWSER_AMBER_MUTED: [f32; 4] = [0.898, 0.678, 0.271, 0.14]; // Translucent amber fill
pub const BROWSER_TEXT_PRI: [f32; 4] = [0.961, 0.965, 0.980, 1.0]; // #F5F6FA Primary text
pub const BROWSER_TEXT_SEC: [f32; 4] = [0.573, 0.604, 0.667, 1.0]; // #929AAA Secondary text
pub const BROWSER_BORDER: [f32; 4] = [1.0, 1.0, 1.0, 0.10]; // rgba(255,255,255,0.10)
pub const BROWSER_BORDER_FOCUS: [f32; 4] = [0.898, 0.678, 0.271, 0.65]; // Solar Amber border focus
pub const BROWSER_EMERALD: [f32; 4] = [0.20, 0.78, 0.55, 1.0]; // Muted emerald success
pub const BROWSER_SOFT_RED: [f32; 4] = [0.94, 0.35, 0.35, 1.0]; // Soft red error

// ── Screen Mode & Workspace Types ───────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserScreen {
    NewTab,
    ActiveBrowsing,
    Bookmarks,
    ReadingList,
    History,
    Downloads,
    Settings,
    ShieldDashboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarSection {
    Navigation,
    Workspaces,
    System,
}

// ── 1. BrowserTitleBar ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTitleBar {
    pub rect: LayoutRect,
    pub title: String,
    pub version_label: String,
    pub workspace_label: String,
}

impl BrowserTitleBar {
    pub fn new(rect: LayoutRect, title: impl Into<String>) -> Self {
        Self {
            rect,
            title: title.into(),
            version_label: "v2.4 Pro".to_string(),
            workspace_label: "🏛️ Sovereign Academic Workspace".to_string(),
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        [0.082, 0.098, 0.133, 0.95]
    }

    pub fn border_color(&self) -> [f32; 4] {
        BROWSER_BORDER
    }

    pub fn min_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x + self.rect.w - 92.0,
            self.rect.y + 6.0,
            26.0,
            24.0,
        )
    }

    pub fn max_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x + self.rect.w - 62.0,
            self.rect.y + 6.0,
            26.0,
            24.0,
        )
    }

    pub fn close_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x + self.rect.w - 32.0,
            self.rect.y + 6.0,
            26.0,
            24.0,
        )
    }
}

// ── 2. BrowserTab ────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub icon: String,
    pub is_active: bool,
    pub is_loading: bool,
    pub state: WidgetState,
}

impl BrowserTab {
    pub fn new(
        id: usize,
        title: impl Into<String>,
        url: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            url: url.into(),
            icon: icon.into(),
            is_active: false,
            is_loading: false,
            state: WidgetState::Normal,
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        if self.is_active {
            BROWSER_GLASS_RAISED
        } else {
            match self.state {
                WidgetState::Hover => [0.12, 0.14, 0.18, 0.70],
                WidgetState::Pressed => [0.10, 0.12, 0.16, 0.85],
                WidgetState::Disabled => [0.06, 0.07, 0.09, 0.40],
                _ => [0.06, 0.07, 0.09, 0.55],
            }
        }
    }

    pub fn text_color(&self) -> [f32; 4] {
        if self.is_active {
            BROWSER_TEXT_PRI
        } else {
            BROWSER_TEXT_SEC
        }
    }

    pub fn indicator_color(&self) -> [f32; 4] {
        if self.is_active {
            BROWSER_SOLAR_AMBER
        } else {
            [0.0; 4]
        }
    }
}

// ── 3. BrowserTabBar ─────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTabBar {
    pub rect: LayoutRect,
    pub tabs: Vec<BrowserTab>,
    pub active_idx: usize,
}

impl BrowserTabBar {
    pub fn new(rect: LayoutRect) -> Self {
        Self {
            rect,
            tabs: Vec::new(),
            active_idx: 0,
        }
    }

    pub fn new_tab_btn_rect(&self) -> LayoutRect {
        let count = self.tabs.len() as f32;
        let tab_w = 175.0_f32.min((self.rect.w - 40.0) / count.max(1.0));
        let x = self.rect.x + count * (tab_w + 4.0) + 4.0;
        LayoutRect::new(x, self.rect.y + 3.0, 28.0, 28.0)
    }
}

// ── 4. NavigationToolbar ────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationToolbar {
    pub rect: LayoutRect,
    pub state: WidgetState,
}

impl NavigationToolbar {
    pub fn new(rect: LayoutRect) -> Self {
        Self {
            rect,
            state: WidgetState::Normal,
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        BROWSER_GLASS_RAISED
    }

    pub fn border_color(&self) -> [f32; 4] {
        BROWSER_BORDER
    }
}

// ── 5. AddressSearchField ───────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSearchField {
    pub rect: LayoutRect,
    pub text: String,
    pub placeholder: String,
    pub is_secure: bool,
    pub state: WidgetState,
}

impl AddressSearchField {
    pub fn new(rect: LayoutRect, placeholder: impl Into<String>) -> Self {
        Self {
            rect,
            text: String::new(),
            placeholder: placeholder.into(),
            is_secure: true,
            state: WidgetState::Normal,
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        match self.state {
            WidgetState::Focused => [0.10, 0.12, 0.17, 0.98],
            WidgetState::Hover => [0.09, 0.11, 0.15, 0.95],
            _ => [0.07, 0.08, 0.12, 0.90],
        }
    }

    pub fn border_color(&self) -> [f32; 4] {
        if self.state == WidgetState::Focused {
            BROWSER_BORDER_FOCUS
        } else {
            BROWSER_BORDER
        }
    }

    pub fn lock_color(&self) -> [f32; 4] {
        if self.is_secure {
            BROWSER_EMERALD
        } else {
            BROWSER_SOFT_RED
        }
    }
}

// ── 6. SidebarNavigationItem ────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarNavigationItem {
    pub id: usize,
    pub label: String,
    pub icon: String,
    pub badge: Option<String>,
    pub screen: BrowserScreen,
    pub section: SidebarSection,
    pub state: WidgetState,
}

// ── 7. BrowserSidebar ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSidebar {
    pub rect: LayoutRect,
    pub expanded: bool,
    pub compact_width: f32,
    pub expanded_width: f32,
    pub items: Vec<SidebarNavigationItem>,
    pub active_screen: BrowserScreen,
}

impl BrowserSidebar {
    pub fn new(rect: LayoutRect, expanded: bool) -> Self {
        let items = vec![
            SidebarNavigationItem {
                id: 0,
                label: "New Tab".into(),
                icon: "🏠".into(),
                badge: None,
                screen: BrowserScreen::NewTab,
                section: SidebarSection::Navigation,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 1,
                label: "Bookmarks".into(),
                icon: "★".into(),
                badge: Some("24".into()),
                screen: BrowserScreen::Bookmarks,
                section: SidebarSection::Navigation,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 2,
                label: "Reading List".into(),
                icon: "📖".into(),
                badge: Some("6".into()),
                screen: BrowserScreen::ReadingList,
                section: SidebarSection::Navigation,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 3,
                label: "History".into(),
                icon: "⏱️".into(),
                badge: None,
                screen: BrowserScreen::History,
                section: SidebarSection::Navigation,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 4,
                label: "Study Space".into(),
                icon: "🎓".into(),
                badge: Some("Active".into()),
                screen: BrowserScreen::ActiveBrowsing,
                section: SidebarSection::Workspaces,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 5,
                label: "Research Lab".into(),
                icon: "🔬".into(),
                badge: None,
                screen: BrowserScreen::ActiveBrowsing,
                section: SidebarSection::Workspaces,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 6,
                label: "Downloads".into(),
                icon: "📥".into(),
                badge: Some("3".into()),
                screen: BrowserScreen::Downloads,
                section: SidebarSection::System,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 7,
                label: "Shield Hub".into(),
                icon: "🛡️".into(),
                badge: Some("100%".into()),
                screen: BrowserScreen::ShieldDashboard,
                section: SidebarSection::System,
                state: WidgetState::Normal,
            },
            SidebarNavigationItem {
                id: 8,
                label: "Settings".into(),
                icon: "⚙️".into(),
                badge: None,
                screen: BrowserScreen::Settings,
                section: SidebarSection::System,
                state: WidgetState::Normal,
            },
        ];

        Self {
            rect,
            expanded,
            compact_width: 62.0,
            expanded_width: 200.0,
            items,
            active_screen: BrowserScreen::NewTab,
        }
    }

    pub fn current_width(&self) -> f32 {
        if self.expanded {
            self.expanded_width
        } else {
            self.compact_width
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        [0.055, 0.065, 0.088, 0.96]
    }
}

// ── 8. QuickAccessTile ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAccessTile {
    pub rect: LayoutRect,
    pub title: String,
    pub category: String,
    pub icon: String,
    pub url: String,
    pub state: WidgetState,
}

impl QuickAccessTile {
    pub fn new(
        rect: LayoutRect,
        title: impl Into<String>,
        category: impl Into<String>,
        icon: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            rect,
            title: title.into(),
            category: category.into(),
            icon: icon.into(),
            url: url.into(),
            state: WidgetState::Normal,
        }
    }

    pub fn surface_color(&self) -> [f32; 4] {
        match self.state {
            WidgetState::Hover => [0.12, 0.15, 0.22, 0.95],
            WidgetState::Pressed => [0.14, 0.17, 0.25, 1.0],
            _ => BROWSER_GLASS_RAISED,
        }
    }

    pub fn border_color(&self) -> [f32; 4] {
        if self.state == WidgetState::Hover {
            BROWSER_BORDER_FOCUS
        } else {
            BROWSER_BORDER
        }
    }
}

// ── 9. RecentWebsiteCard ────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentWebsiteCard {
    pub rect: LayoutRect,
    pub title: String,
    pub url: String,
    pub timestamp: String,
    pub icon: String,
    pub state: WidgetState,
}

// ── 10. ReadingListCard ─────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingListCard {
    pub rect: LayoutRect,
    pub title: String,
    pub domain: String,
    pub est_minutes: u32,
    pub progress: f32,
    pub is_cached: bool,
    pub state: WidgetState,
}

// ── 11. BookmarkCard ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkCard {
    pub rect: LayoutRect,
    pub title: String,
    pub url: String,
    pub folder: String,
    pub icon: String,
    pub date_added: String,
    pub state: WidgetState,
}

// ── 12. DownloadCard ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadCard {
    pub rect: LayoutRect,
    pub file_name: String,
    pub file_size_mb: f32,
    pub progress: f32,
    pub speed_mbps: f32,
    pub is_completed: bool,
    pub state: WidgetState,
}

// ── 13. PrivacyStatusCard ───────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyStatusCard {
    pub rect: LayoutRect,
    pub title: String,
    pub count: u64,
    pub status_desc: String,
    pub is_safe: bool,
    pub indicator_color: [f32; 4],
    pub state: WidgetState,
}

// ── 14. BrowserSettingsSection ──────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettingsSection {
    pub rect: LayoutRect,
    pub title: String,
    pub description: String,
    pub icon: String,
}

// ── 15. BrowserProfileButton ────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfileButton {
    pub rect: LayoutRect,
    pub initials: String,
    pub name: String,
    pub is_online: bool,
    pub state: WidgetState,
}

// ── 16. BrowserContextMenu ──────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserContextMenu {
    pub rect: LayoutRect,
    pub items: Vec<String>,
    pub is_open: bool,
}

// ── 17. BrowserTooltip ──────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTooltip {
    pub rect: LayoutRect,
    pub text: String,
    pub visible: bool,
}

// ── 18. BrowserDialog ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserDialog {
    pub rect: LayoutRect,
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub is_open: bool,
}

// ── 19. BrowserEmptyState ───────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserEmptyState {
    pub rect: LayoutRect,
    pub icon: String,
    pub headline: String,
    pub subtext: String,
    pub action_label: Option<String>,
}

impl BrowserEmptyState {
    pub fn new(
        rect: LayoutRect,
        icon: impl Into<String>,
        headline: impl Into<String>,
        subtext: impl Into<String>,
    ) -> Self {
        Self {
            rect,
            icon: icon.into(),
            headline: headline.into(),
            subtext: subtext.into(),
            action_label: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_tokens_and_contrast() {
        assert_eq!(BROWSER_BG_DEEP.len(), 4);
        assert_eq!(BROWSER_SOLAR_AMBER[0], 0.898);
        assert_eq!(BROWSER_EMERALD[1], 0.78);
    }

    #[test]
    fn test_browser_tab_states() {
        let mut tab = BrowserTab::new(1, "NPTEL", "https://nptel.ac.in", "🏛️");
        assert_eq!(tab.indicator_color(), [0.0; 4]);
        tab.is_active = true;
        assert_eq!(tab.indicator_color(), BROWSER_SOLAR_AMBER);
    }

    #[test]
    fn test_browser_sidebar_expand_collapse() {
        let mut sb = BrowserSidebar::new(LayoutRect::new(0.0, 0.0, 62.0, 600.0), false);
        assert_eq!(sb.current_width(), 62.0);
        sb.expanded = true;
        assert_eq!(sb.current_width(), 200.0);
        assert_eq!(sb.items.len(), 9);
    }
}
