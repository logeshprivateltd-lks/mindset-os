//! Data display, Table, EmptyState, Avatar, and ProgressRing widgets.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::{ElementId, WidgetState};
use serde::{Deserialize, Serialize};

/// Data Table column definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub title: String,
    pub width_weight: f32,
}

/// Data Table row item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<String>,
    pub is_selected: bool,
}

/// Data Table widget supporting sorting and selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub id: ElementId,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub selected_row: Option<usize>,
    pub state: WidgetState,
}

impl DataTable {
    pub fn new(id: ElementId, columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self {
            id,
            columns,
            rows,
            selected_row: None,
            state: WidgetState::Normal,
        }
    }

    pub fn select_row(&mut self, idx: usize) {
        if idx < self.rows.len() {
            self.selected_row = Some(idx);
        }
    }
}

/// Empty state illustration / card widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyState {
    pub id: ElementId,
    pub icon_badge: &'static str,
    pub title: String,
    pub description: String,
    pub action_label: Option<String>,
}

impl EmptyState {
    pub fn new(
        id: ElementId,
        icon_badge: &'static str,
        title: impl Into<String>,
        desc: impl Into<String>,
    ) -> Self {
        Self {
            id,
            icon_badge,
            title: title.into(),
            description: desc.into(),
            action_label: None,
        }
    }
}

/// Avatar shape format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AvatarShape {
    #[default]
    Circle,
    Squircle,
}

/// Avatar presence status indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AvatarStatus {
    #[default]
    None,
    Online,
    Away,
    Busy,
    Offline,
}

impl AvatarStatus {
    /// Semantic RGBA color corresponding to user presence status.
    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::None => [0.0, 0.0, 0.0, 0.0],
            Self::Online => [0.133, 0.773, 0.369, 1.000], // #22C55E Emerald Green
            Self::Away => [0.961, 0.620, 0.043, 1.000],   // #F59E0B Solar Amber
            Self::Busy => [0.937, 0.267, 0.267, 1.000],   // #EF4444 Danger Red
            Self::Offline => [0.392, 0.455, 0.545, 1.000], // #64748B Slate Muted
        }
    }
}

/// Avatar profile icon widget with Squircle/Circle shapes, initials centering, and presence indicator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar {
    pub id: ElementId,
    pub initials: String,
    pub bg_color: [f32; 4],
    pub radius: f32,
    pub shape: AvatarShape,
    pub status: AvatarStatus,
}

impl Avatar {
    pub const BADGE_SIZE: f32 = 10.0;
    pub const KNOCKOUT_SIZE: f32 = 14.0;

    pub fn new(id: ElementId, initials: impl Into<String>, bg_color: [f32; 4]) -> Self {
        Self {
            id,
            initials: initials.into(),
            bg_color,
            radius: 20.0,
            shape: AvatarShape::Circle,
            status: AvatarStatus::None,
        }
    }

    pub fn with_shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_status(mut self, status: AvatarStatus) -> Self {
        self.status = status;
        self
    }

    /// Corner radius according to shape: Circle = radius, Squircle = radius * 2 * 0.28.
    pub fn corner_radius(&self) -> f32 {
        match self.shape {
            AvatarShape::Circle => self.radius,
            AvatarShape::Squircle => (self.radius * 2.0 * 0.28).min(14.0),
        }
    }

    /// Proportional font size for centered initials.
    pub fn font_size(&self) -> f32 {
        (self.radius * 0.85).clamp(10.0, 24.0)
    }

    /// Outer presence indicator badge rectangle located at the bottom-right corner.
    pub fn status_badge_rect(
        &self,
        avatar_rect: crate::layout::LayoutRect,
    ) -> crate::layout::LayoutRect {
        crate::layout::LayoutRect::new(
            avatar_rect.x + avatar_rect.w - Self::BADGE_SIZE,
            avatar_rect.y + avatar_rect.h - Self::BADGE_SIZE,
            Self::BADGE_SIZE,
            Self::BADGE_SIZE,
        )
    }

    /// Knockout background ring rectangle for visual separation against dark glass.
    pub fn status_knockout_rect(
        &self,
        avatar_rect: crate::layout::LayoutRect,
    ) -> crate::layout::LayoutRect {
        crate::layout::LayoutRect::new(
            avatar_rect.x + avatar_rect.w - Self::KNOCKOUT_SIZE + 2.0,
            avatar_rect.y + avatar_rect.h - Self::KNOCKOUT_SIZE + 2.0,
            Self::KNOCKOUT_SIZE,
            Self::KNOCKOUT_SIZE,
        )
    }
}

/// Divider line widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider {
    pub id: ElementId,
    pub is_vertical: bool,
    pub thickness: f32,
}

impl Divider {
    pub fn horizontal(id: ElementId) -> Self {
        Self {
            id,
            is_vertical: false,
            thickness: 1.0,
        }
    }

    pub fn vertical(id: ElementId) -> Self {
        Self {
            id,
            is_vertical: true,
            thickness: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutRect;

    #[test]
    fn test_avatar_shapes_and_status() {
        let r = LayoutRect::new(10.0, 10.0, 40.0, 40.0);
        let avatar = Avatar::new(ElementId::from_raw(1), "MS", [0.2, 0.4, 0.8, 1.0])
            .with_shape(AvatarShape::Squircle)
            .with_status(AvatarStatus::Online);

        assert_eq!(avatar.shape, AvatarShape::Squircle);
        assert_eq!(avatar.status, AvatarStatus::Online);
        assert!((avatar.corner_radius() - 40.0 * 0.28).abs() < 0.01);

        let badge_r = avatar.status_badge_rect(r);
        assert_eq!(badge_r.w, Avatar::BADGE_SIZE);
        assert_eq!(badge_r.x, 10.0 + 40.0 - 10.0);
        assert_eq!(badge_r.y, 10.0 + 40.0 - 10.0);

        let col = avatar.status.color();
        assert_eq!(col, [0.133, 0.773, 0.369, 1.0]);
    }
}
