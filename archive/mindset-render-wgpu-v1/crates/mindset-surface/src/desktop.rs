//! Desktop wallpaper canvas, snap-to-grid icon shortcuts, and right-click context menu.

use crate::wm::WindowId;
use mindset_ui::LayoutRect;

pub const GRID_CELL_WIDTH: f32 = 96.0;
pub const GRID_CELL_HEIGHT: f32 = 92.0;
pub const GRID_MARGIN_LEFT: f32 = 24.0;
pub const GRID_MARGIN_TOP: f32 = 48.0;
pub const GRID_GAP: f32 = 24.0;

/// A desktop wallpaper icon shortcut.
#[derive(Debug, Clone, PartialEq)]
pub struct DesktopIcon {
    pub id: WindowId,
    pub label: String,
    pub symbol: String,
    pub col: usize,
    pub row: usize,
    pub is_selected: bool,
    pub is_pinned: bool,
    pub is_dragging: bool,
    pub drag_offset: (f32, f32),
    pub temp_pos: Option<(f32, f32)>,
    pub rendered_rect: LayoutRect,
}

impl DesktopIcon {
    pub fn new(id: WindowId, col: usize, row: usize) -> Self {
        let mut icon = Self {
            id,
            label: id.desktop_label().to_string(),
            symbol: id.icon_symbol().to_string(),
            col,
            row,
            is_selected: false,
            is_pinned: true,
            is_dragging: false,
            drag_offset: (0.0, 0.0),
            temp_pos: None,
            rendered_rect: LayoutRect::zero(),
        };
        icon.rendered_rect = icon.rect();
        icon
    }

    pub fn rect(&self) -> LayoutRect {
        if let Some((tx, ty)) = self.temp_pos {
            LayoutRect::new(tx, ty, GRID_CELL_WIDTH, GRID_CELL_HEIGHT)
        } else {
            let x = GRID_MARGIN_LEFT + self.col as f32 * (GRID_CELL_WIDTH + GRID_GAP);
            let y = GRID_MARGIN_TOP + self.row as f32 * (GRID_CELL_HEIGHT + GRID_GAP);
            LayoutRect::new(x, y, GRID_CELL_WIDTH, GRID_CELL_HEIGHT)
        }
    }
}

/// Wallpaper background render style preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundStyle {
    CleanSlate,
    SoftGradient,
    MinimalGrid,
    DeepNight,
}

impl BackgroundStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::CleanSlate => "Clean Slate",
            Self::SoftGradient => "Soft Gradient",
            Self::MinimalGrid => "Minimal Grid",
            Self::DeepNight => "Deep Night",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::CleanSlate => Self::SoftGradient,
            Self::SoftGradient => Self::MinimalGrid,
            Self::MinimalGrid => Self::DeepNight,
            Self::DeepNight => Self::CleanSlate,
        }
    }
}

/// The desktop icon grid manager.
pub struct DesktopGrid {
    pub icons: Vec<DesktopIcon>,
    pub background_style: BackgroundStyle,
}

impl Default for DesktopGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopGrid {
    pub fn new() -> Self {
        let mut grid = Self {
            icons: Vec::new(),
            background_style: BackgroundStyle::CleanSlate,
        };

        // Seed default 8 desktop icons in 2 columns
        let default_icons = [
            (WindowId::Tasks, 0, 0),
            (WindowId::StudyReader, 0, 1),
            (WindowId::LayaAiChat, 0, 2),
            (WindowId::SearchBrowser, 0, 3),
            (WindowId::FileVault, 1, 0),
            (WindowId::ProgressStats, 1, 1),
            (WindowId::SystemMonitor, 1, 2),
            (WindowId::Settings, 1, 3),
        ];

        for (id, col, row) in default_icons {
            grid.icons.push(DesktopIcon::new(id, col, row));
        }

        grid
    }

    /// Synchronizes the actual rendered screen rect for each icon (accounting for widgets offset)
    pub fn sync_rendered_layout(&mut self, base_offset_x: f32) {
        self.sync_rendered_layout_scaled(base_offset_x, 1.0);
    }

    /// Synchronizes the actual rendered screen rect for each icon with UI scaling factor `s`
    /// and dynamic left widget clearance offset `base_offset_x`.
    pub fn sync_rendered_layout_scaled(&mut self, base_offset_x: f32, s: f32) {
        let cell_w = 104.0 * s;
        let cell_h = 108.0 * s;
        let col_gap = 20.0 * s;
        let row_gap = 18.0 * s;
        let margin_top = 52.0 * s;

        for ic in &mut self.icons {
            if let Some(pos) = ic.temp_pos {
                ic.rendered_rect = LayoutRect::new(pos.0, pos.1, cell_w, cell_h);
            } else {
                let x = base_offset_x + ic.col as f32 * (cell_w + col_gap);
                let y = margin_top + ic.row as f32 * (cell_h + row_gap);
                ic.rendered_rect = LayoutRect::new(x, y, cell_w, cell_h);
            }
        }
    }

    /// Snap coordinate (x, y) to closest grid slot (col, row).
    pub fn snap_to_slot(x: f32, y: f32) -> (usize, usize) {
        let rel_x = (x - GRID_MARGIN_LEFT).max(0.0);
        let rel_y = (y - GRID_MARGIN_TOP).max(0.0);

        let col = (rel_x / (GRID_CELL_WIDTH + GRID_GAP)).round() as usize;
        let row = (rel_y / (GRID_CELL_HEIGHT + GRID_GAP)).round() as usize;
        (col.min(8), row.min(8))
    }

    /// Checks if a slot (col, row) is currently occupied by an icon (excluding `exclude_id`).
    pub fn is_slot_occupied(&self, col: usize, row: usize, exclude_id: Option<WindowId>) -> bool {
        self.icons.iter().any(|ic| {
            if Some(ic.id) == exclude_id {
                false
            } else {
                ic.is_pinned && ic.col == col && ic.row == row
            }
        })
    }

    /// Finds the next collision-free slot.
    pub fn next_free_slot(&self) -> (usize, usize) {
        for row in 0..8 {
            for col in 0..8 {
                if !self.is_slot_occupied(col, row, None) {
                    return (col, row);
                }
            }
        }
        (0, 0)
    }

    /// Move an icon to (col, row), automatically shifting collisions.
    pub fn place_icon(&mut self, id: WindowId, target_col: usize, target_row: usize) {
        if self.is_slot_occupied(target_col, target_row, Some(id)) {
            // Find next free slot for the icon
            let (free_col, free_row) = self.next_free_slot();
            if let Some(ic) = self.icons.iter_mut().find(|i| i.id == id) {
                ic.col = free_col;
                ic.row = free_row;
                ic.temp_pos = None;
            }
        } else if let Some(ic) = self.icons.iter_mut().find(|i| i.id == id) {
            ic.col = target_col;
            ic.row = target_row;
            ic.temp_pos = None;
        }
    }

    /// Auto-arranges all pinned icons in neat vertical columns.
    pub fn auto_arrange(&mut self) {
        let max_rows = 5;
        let mut idx = 0;
        for ic in &mut self.icons {
            if ic.is_pinned {
                ic.col = idx / max_rows;
                ic.row = idx % max_rows;
                ic.temp_pos = None;
                idx += 1;
            }
        }
    }

    pub fn select_icon_at(&mut self, px: f32, py: f32) -> Option<WindowId> {
        let mut clicked_id = None;
        for ic in &mut self.icons {
            let hit_rect = if ic.rendered_rect.w > 0.0 {
                ic.rendered_rect
            } else {
                ic.rect()
            };
            if ic.is_pinned && hit_rect.contains(px, py) {
                ic.is_selected = true;
                clicked_id = Some(ic.id);
            } else {
                ic.is_selected = false;
            }
        }
        clicked_id
    }
}

/// Actions available in the desktop wallpaper right-click context menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuAction {
    OpenStudySession,
    AskMaindyAi,
    AutoArrangeIcons,
    SwitchTheme,
    NextBackgroundStyle,
    OpenSettings,
}

impl ContextMenuAction {
    pub fn all() -> [ContextMenuAction; 6] {
        [
            Self::OpenStudySession,
            Self::AskMaindyAi,
            Self::AutoArrangeIcons,
            Self::SwitchTheme,
            Self::NextBackgroundStyle,
            Self::OpenSettings,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::OpenStudySession => "Open Study Session",
            Self::AskMaindyAi => "Ask Maindy AI",
            Self::AutoArrangeIcons => "Auto-Arrange Desktop Icons",
            Self::SwitchTheme => "Switch Theme (Light / Dark)",
            Self::NextBackgroundStyle => "Next Background Style",
            Self::OpenSettings => "Display & Personalization Settings",
        }
    }
}

pub const CONTEXT_MENU_WIDTH: f32 = 240.0;
pub const CONTEXT_MENU_ITEM_HEIGHT: f32 = 30.0;

/// Desktop right-click context menu popup.
#[derive(Debug, Clone)]
pub struct DesktopContextMenu {
    pub is_open: bool,
    pub x: f32,
    pub y: f32,
}

impl Default for DesktopContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopContextMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn open(&mut self, cursor_x: f32, cursor_y: f32, screen_w: f32, screen_h: f32) {
        self.is_open = true;
        let menu_h = ContextMenuAction::all().len() as f32 * CONTEXT_MENU_ITEM_HEIGHT + 16.0;
        self.x = cursor_x.clamp(8.0, (screen_w - CONTEXT_MENU_WIDTH - 8.0).max(8.0));
        self.y = cursor_y.clamp(40.0, (screen_h - menu_h - 8.0).max(40.0));
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn rect(&self) -> LayoutRect {
        let menu_h = ContextMenuAction::all().len() as f32 * CONTEXT_MENU_ITEM_HEIGHT + 16.0;
        LayoutRect::new(self.x, self.y, CONTEXT_MENU_WIDTH, menu_h)
    }

    pub fn item_rect(&self, idx: usize) -> LayoutRect {
        LayoutRect::new(
            self.x + 8.0,
            self.y + 8.0 + idx as f32 * CONTEXT_MENU_ITEM_HEIGHT,
            CONTEXT_MENU_WIDTH - 16.0,
            CONTEXT_MENU_ITEM_HEIGHT,
        )
    }

    pub fn item_at(&self, px: f32, py: f32) -> Option<ContextMenuAction> {
        if !self.is_open || !self.rect().contains(px, py) {
            return None;
        }

        for (idx, action) in ContextMenuAction::all().iter().enumerate() {
            if self.item_rect(idx).contains(px, py) {
                return Some(*action);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_snap_to_slot_calculation() {
        // Point right in slot (0, 0): margin_left=24, margin_top=48
        let (c1, r1) = DesktopGrid::snap_to_slot(24.0, 48.0);
        assert_eq!((c1, r1), (0, 0));

        // Point near slot (1, 2)
        let target_x = GRID_MARGIN_LEFT + 1.0 * (GRID_CELL_WIDTH + GRID_GAP) + 10.0;
        let target_y = GRID_MARGIN_TOP + 2.0 * (GRID_CELL_HEIGHT + GRID_GAP) + 10.0;
        let (c2, r2) = DesktopGrid::snap_to_slot(target_x, target_y);
        assert_eq!((c2, r2), (1, 2));
    }

    #[test]
    fn test_collision_free_slot_allocation() {
        let mut grid = DesktopGrid::new();
        // Slot (0,0) is occupied by Tasks
        assert!(grid.is_slot_occupied(0, 0, None));

        // Place a new item onto (0, 0) should automatically redirect to next free slot
        let free = grid.next_free_slot();
        grid.place_icon(WindowId::Settings, 0, 0);

        let Some(settings) = grid.icons.iter().find(|i| i.id == WindowId::Settings) else {
            panic!("Settings icon not found");
        };
        assert_eq!((settings.col, settings.row), free);
    }

    #[test]
    fn test_context_menu_screen_clamping() {
        let mut menu = DesktopContextMenu::new();
        // Click very close to right-bottom edge of screen (1280x800)
        menu.open(1270.0, 790.0, 1280.0, 800.0);

        assert!(menu.x + CONTEXT_MENU_WIDTH <= 1280.0);
        assert!(menu.y + menu.rect().h <= 800.0);
    }
}
