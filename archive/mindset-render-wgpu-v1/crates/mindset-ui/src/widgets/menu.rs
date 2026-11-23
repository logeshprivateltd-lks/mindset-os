//! Elevated Liquid Glass Context Menus, Menu Items, Separators, and Dropdown cards.
//! Pure Rust hardware-accelerated interactive popup controls for MindSet OS.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::ElementId;
use crate::input::InputState;
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};

/// Individual item within a context menu or dropdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: usize,
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub is_separator: bool,
    pub is_disabled: bool,
    pub has_submenu: bool,
}

impl MenuItem {
    pub fn action(
        id: usize,
        label: impl Into<String>,
        icon: Option<&str>,
        shortcut: Option<&str>,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            icon: icon.map(|s| s.to_string()),
            shortcut: shortcut.map(|s| s.to_string()),
            is_separator: false,
            is_disabled: false,
            has_submenu: false,
        }
    }

    pub fn with_submenu(mut self) -> Self {
        self.has_submenu = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self
    }

    pub fn separator() -> Self {
        Self {
            id: 0,
            label: String::new(),
            icon: None,
            shortcut: None,
            is_separator: true,
            is_disabled: true,
            has_submenu: false,
        }
    }
}

/// Elevated Liquid Glass Context Menu card with hover highlights, shortcut indicators, and separator lines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenu {
    pub id: ElementId,
    pub position: (f32, f32),
    pub items: Vec<MenuItem>,
    pub hovered_idx: Option<usize>,
    pub selected_idx: Option<usize>,
    pub is_open: bool,
    pub width: f32,
}

impl ContextMenu {
    pub const ITEM_HEIGHT: f32 = 34.0;
    pub const SEPARATOR_HEIGHT: f32 = 8.0;
    pub const PADDING_Y: f32 = 6.0;
    pub const PADDING_X: f32 = 6.0;
    pub const CORNER_RADIUS: f32 = 12.0;
    pub const MIN_WIDTH: f32 = 210.0;

    /// Elevated liquid glass dark surface: #151922 rgba(21, 25, 34, 0.96)
    pub const SURFACE_FILL: [f32; 4] = [0.082, 0.098, 0.133, 0.960];
    /// Specular top-lit rim highlight: rgba(255, 255, 255, 0.18)
    pub const SPECULAR_RIM: [f32; 4] = [1.000, 1.000, 1.000, 0.180];
    /// Subtle item hover wash: rgba(255, 255, 255, 0.08)
    pub const ITEM_HOVER_FILL: [f32; 4] = [1.000, 1.000, 1.000, 0.080];
    /// Vibrant Solar Amber item selection tint: rgba(245, 158, 11, 0.15)
    pub const ITEM_SELECTED_FILL: [f32; 4] = [0.961, 0.620, 0.043, 0.150];
    /// Horizontal divider separator color: rgba(255, 255, 255, 0.10)
    pub const SEPARATOR_COLOR: [f32; 4] = [1.000, 1.000, 1.000, 0.100];

    pub fn new(id: ElementId, items: Vec<MenuItem>) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            items,
            hovered_idx: None,
            selected_idx: None,
            is_open: false,
            width: Self::MIN_WIDTH,
        }
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        self.position = (x, y);
        self.is_open = true;
        self.hovered_idx = None;
        self.selected_idx = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.hovered_idx = None;
        self.selected_idx = None;
    }

    /// Computes the total required height of the elevated card based on items and separators.
    pub fn content_height(&self) -> f32 {
        let mut total = Self::PADDING_Y * 2.0;
        for item in &self.items {
            if item.is_separator {
                total += Self::SEPARATOR_HEIGHT;
            } else {
                total += Self::ITEM_HEIGHT;
            }
        }
        total
    }

    /// Outer bounding rectangle for the elevated liquid glass card.
    pub fn card_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.position.0,
            self.position.1,
            self.width,
            self.content_height(),
        )
    }

    /// Clamped card rectangle ensuring the menu never bleeds outside the viewport.
    pub fn clamped_card_rect(&self, viewport_w: f32, viewport_h: f32) -> LayoutRect {
        let h = self.content_height();
        let x = if self.position.0 + self.width > viewport_w {
            (self.position.0 - self.width).max(0.0)
        } else {
            self.position.0
        };
        let y = if self.position.1 + h > viewport_h {
            (self.position.1 - h).max(0.0)
        } else {
            self.position.1
        };
        LayoutRect::new(x, y, self.width, h)
    }

    /// Computes layout rectangle for item at `idx`.
    pub fn item_rect(&self, idx: usize) -> LayoutRect {
        let mut y = self.position.1 + Self::PADDING_Y;
        for (i, item) in self.items.iter().enumerate() {
            let h = if item.is_separator {
                Self::SEPARATOR_HEIGHT
            } else {
                Self::ITEM_HEIGHT
            };
            if i == idx {
                return LayoutRect::new(
                    self.position.0 + Self::PADDING_X,
                    y,
                    self.width - Self::PADDING_X * 2.0,
                    h,
                );
            }
            y += h;
        }
        LayoutRect::zero()
    }

    /// Advances selection to the next interactive menu item (Down arrow key).
    pub fn navigate_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let start = self.selected_idx.map(|i| i + 1).unwrap_or(0);
        for i in 0..self.items.len() {
            let idx = (start + i) % self.items.len();
            if !self.items[idx].is_separator && !self.items[idx].is_disabled {
                self.selected_idx = Some(idx);
                self.hovered_idx = Some(idx);
                return;
            }
        }
    }

    /// Moves selection to the previous interactive menu item (Up arrow key).
    pub fn navigate_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let len = self.items.len();
        let start = self
            .selected_idx
            .map(|i| if i == 0 { len - 1 } else { i - 1 })
            .unwrap_or(len - 1);
        for i in 0..len {
            let idx = (start + len - (i % len)) % len;
            if !self.items[idx].is_separator && !self.items[idx].is_disabled {
                self.selected_idx = Some(idx);
                self.hovered_idx = Some(idx);
                return;
            }
        }
    }

    /// Handles pointer events. Returns `Some(action_id)` if an item was clicked.
    pub fn handle_input(&mut self, input: &InputState) -> Option<usize> {
        if !self.is_open {
            return None;
        }

        let card = self.card_rect();
        let is_over_card = card.contains(input.cursor.0, input.cursor.1);

        // Click outside closes the menu
        if (input.just_released || input.right_released) && !is_over_card {
            self.close();
            return None;
        }

        self.hovered_idx = None;
        for (i, item) in self.items.iter().enumerate() {
            if item.is_separator || item.is_disabled {
                continue;
            }
            let r = self.item_rect(i);
            if r.contains(input.cursor.0, input.cursor.1) {
                self.hovered_idx = Some(i);
                if input.just_released {
                    let action_id = item.id;
                    self.close();
                    return Some(action_id);
                }
            }
        }

        None
    }
}

/// Floating Dropdown menu anchored to an input or button trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownMenu {
    pub menu: ContextMenu,
    pub anchor_rect: LayoutRect,
}

impl DropdownMenu {
    pub fn new(id: ElementId, anchor_rect: LayoutRect, items: Vec<MenuItem>) -> Self {
        let mut menu = ContextMenu::new(id, items);
        menu.width = anchor_rect.w.max(ContextMenu::MIN_WIDTH);
        menu.position = (anchor_rect.x, anchor_rect.y + anchor_rect.h + 4.0);
        Self { menu, anchor_rect }
    }

    pub fn toggle(&mut self) {
        if self.menu.is_open {
            self.menu.close();
        } else {
            self.menu.open_at(
                self.anchor_rect.x,
                self.anchor_rect.y + self.anchor_rect.h + 4.0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_layout_and_hit_testing() {
        let items = vec![
            MenuItem::action(10, "Open File", Some("📂"), Some("Ctrl+O")),
            MenuItem::action(11, "Save As...", Some("💾"), Some("Ctrl+S")),
            MenuItem::separator(),
            MenuItem::action(12, "Delete", Some("🗑️"), Some("Del")).disabled(),
            MenuItem::action(13, "Properties", Some("⚙️"), Some("Alt+Enter")),
        ];

        let mut menu = ContextMenu::new(ElementId::from_raw(1), items);
        menu.open_at(100.0, 100.0);
        assert!(menu.is_open);

        let card_r = menu.card_rect();
        assert_eq!(card_r.x, 100.0);
        assert_eq!(card_r.y, 100.0);
        assert!(card_r.h >= ContextMenu::ITEM_HEIGHT * 4.0 + ContextMenu::SEPARATOR_HEIGHT);

        let item0_r = menu.item_rect(0);
        assert_eq!(item0_r.h, ContextMenu::ITEM_HEIGHT);

        // Test hover and click on item 1 ("Save As...")
        let item1_r = menu.item_rect(1);
        let input = InputState {
            cursor: (item1_r.x + 10.0, item1_r.y + 10.0),
            just_released: true,
            ..Default::default()
        };

        let result = menu.handle_input(&input);
        assert_eq!(result, Some(11));
        assert!(!menu.is_open); // Auto-closed after activation
    }

    #[test]
    fn test_keyboard_navigation_skips_separators_and_disabled() {
        let items = vec![
            MenuItem::action(1, "First", None, None),
            MenuItem::separator(),
            MenuItem::action(2, "Disabled", None, None).disabled(),
            MenuItem::action(3, "Second", None, None),
        ];

        let mut menu = ContextMenu::new(ElementId::from_raw(2), items);
        menu.open_at(50.0, 50.0);

        menu.navigate_next();
        assert_eq!(menu.selected_idx, Some(0));

        // Next should skip separator (idx 1) and disabled item (idx 2), landing on idx 3
        menu.navigate_next();
        assert_eq!(menu.selected_idx, Some(3));

        // Wrapping around should land back on idx 0
        menu.navigate_next();
        assert_eq!(menu.selected_idx, Some(0));
    }
}
