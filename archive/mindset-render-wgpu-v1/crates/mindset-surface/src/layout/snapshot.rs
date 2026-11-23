//! MindSet OS — Layout Snapshot & Invariant Assertions (`snapshot.rs`)
//! Single immutable capture of resolved 2D geometry per frame, keyed by `UiId`.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::wm::WindowId;
use mindset_ui::LayoutRect;
use std::collections::HashMap;

/// Semantic identifier for any layout node or interactive hit-target in MindSet OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiId {
    // ── Top Bar Island ──
    TopBarLeftCapsule,
    TopBarFlashPill,
    TopBarMapPill,
    TopBarMenuFile,
    TopBarMenuEdit,
    TopBarMenuView,
    TopBarMenuHelp,
    TopBarRightCapsule,
    TopBarClockPill,
    TopBarWifiPill,
    TopBarNotifPill,
    TopBarCcBtn,

    // ── Left-Column Desktop Widgets ──
    PrimaryGoalCard,
    MissionCard,
    MissionStartBtn,
    MissionLockBtn,
    SubjectQuotaCard,
    PomodoroCard,
    PomodoroDurationBtn,
    PomodoroCycleBtn,
    SearchPill,

    // ── Today Screen (Distraction-Free Root) ──
    TodayColumn,
    TodayMinimizeBtn,
    TodayOneThingCard,
    TodayStartFocusBtn,
    TodaySetupGoalsBtn,
    TodayRestartBtn,
    TodaySkipBtn,
    TodayRing(usize),
    TodayFutureSelfCard,

    // ── Window Chrome & Content ──
    WindowBody(WindowId),
    WindowContent(WindowId),
    WindowChromeClose(WindowId),
    WindowChromeMax(WindowId),
    WindowChromeMin(WindowId),
    WindowTitlebarDrag(WindowId),
    WindowResizeGrip(WindowId),

    // ── Control Center Drawer ──
    ControlCenterPanel,
    ControlCenterCloseBtn,
    ControlCenterSettingsBtn,
    ControlCenterWifiTile,
    ControlCenterBluetoothTile,
    ControlCenterDndTile,
    ControlCenterAirplaneTile,
    ControlCenterNightLightTile,
    ControlCenterBrightnessTrack,
    ControlCenterVolumeTrack,
    ControlCenterUiScaleTrack,

    // ── Launcher & Command Palette ──
    LauncherPanel,
    LauncherSearchInput,
    LauncherAiHub,
    LauncherAppIcon(WindowId),
    LauncherRecentFile(usize),
    LauncherFooterBtn(usize),
    LauncherPowerBtn,

    // ── Power Menu ──
    PowerMenuPanel,
    PowerMenuItem(usize),

    // ── General Fallback ──
    Custom(&'static str),
    Node(u64),
}

/// Diagnostic report for layout geometry violations (overlaps, clipping, out-of-bounds).
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutViolation {
    Overlap {
        first: UiId,
        second: UiId,
        intersection: LayoutRect,
    },
    Clipping {
        parent: UiId,
        child: UiId,
        child_rect: LayoutRect,
        parent_rect: LayoutRect,
    },
    OutOfBounds {
        id: UiId,
        rect: LayoutRect,
        viewport: LayoutRect,
    },
}

/// Immutable snapshot of resolved layout geometry for a frame.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutSnapshot {
    pub nodes: HashMap<UiId, LayoutRect>,
    pub z_order: Vec<UiId>,
    pub viewport_w: f32,
    pub viewport_h: f32,
    pub ui_scale: f32,
}

impl LayoutSnapshot {
    /// Create a new empty layout snapshot with given viewport dimensions and UI scale.
    pub fn new(viewport_w: f32, viewport_h: f32, ui_scale: f32) -> Self {
        Self {
            nodes: HashMap::new(),
            z_order: Vec::new(),
            viewport_w,
            viewport_h,
            ui_scale,
        }
    }

    /// Register a resolved node rect and append to z-order stack.
    pub fn insert(&mut self, id: UiId, rect: LayoutRect) {
        self.nodes.insert(id, rect);
        if !self.z_order.contains(&id) {
            self.z_order.push(id);
        }
    }

    /// Retrieve the resolved rect for an identifier.
    #[inline]
    pub fn get(&self, id: UiId) -> Option<LayoutRect> {
        self.nodes.get(&id).copied()
    }

    /// Retrieve the resolved rect or zero rect if not found.
    #[inline]
    pub fn rect(&self, id: UiId) -> LayoutRect {
        self.nodes.get(&id).copied().unwrap_or(LayoutRect::zero())
    }

    /// Perform reverse z-order hit testing (topmost / latest sibling wins).
    pub fn hit_test(&self, cursor: (f32, f32)) -> Option<UiId> {
        for &id in self.z_order.iter().rev() {
            if let Some(rect) = self.nodes.get(&id) {
                if rect.contains(cursor.0, cursor.1) {
                    return Some(id);
                }
            }
        }
        None
    }

    /// Assert that no two widgets in the provided slice overlap in 2D space.
    pub fn assert_no_overlaps(&self, ids: &[UiId]) -> Result<(), Vec<LayoutViolation>> {
        let mut violations = Vec::new();
        for i in 0..ids.len() {
            let id_a = ids[i];
            let Some(rect_a) = self.nodes.get(&id_a) else {
                continue;
            };
            for &id_b in ids.iter().skip(i + 1) {
                let Some(rect_b) = self.nodes.get(&id_b) else {
                    continue;
                };
                let x1 = rect_a.x.max(rect_b.x);
                let y1 = rect_a.y.max(rect_b.y);
                let x2 = (rect_a.x + rect_a.w).min(rect_b.x + rect_b.w);
                let y2 = (rect_a.y + rect_a.h).min(rect_b.y + rect_b.h);
                if x2 > x1 && y2 > y1 {
                    let inter = LayoutRect::new(x1, y1, x2 - x1, y2 - y1);
                    if inter.w > 0.01 && inter.h > 0.01 {
                        violations.push(LayoutViolation::Overlap {
                            first: id_a,
                            second: id_b,
                            intersection: inter,
                        });
                    }
                }
            }
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check if child node is completely contained within parent node bounds.
    pub fn assert_contained(&self, parent: UiId, child: UiId) -> bool {
        let Some(p) = self.nodes.get(&parent) else {
            return false;
        };
        let Some(c) = self.nodes.get(&child) else {
            return false;
        };
        c.x >= p.x - 0.01
            && c.y >= p.y - 0.01
            && c.x + c.w <= p.x + p.w + 0.01
            && c.y + c.h <= p.y + p.h + 0.01
    }

    /// Assert that child does not clip outside parent.
    pub fn assert_no_clipping(&self, parent: UiId, child: UiId) -> Result<(), LayoutViolation> {
        if self.assert_contained(parent, child) {
            Ok(())
        } else {
            let p = self.rect(parent);
            let c = self.rect(child);
            Err(LayoutViolation::Clipping {
                parent,
                child,
                child_rect: c,
                parent_rect: p,
            })
        }
    }

    // ─── Geometric Derivation Primitives (Enforcing 0 LayoutRect::new outside layout) ───

    /// Scale a logical pixel value by the snapshot's UI scale.
    #[inline]
    pub fn scaled(&self, px: f32) -> f32 {
        px * self.ui_scale
    }

    /// Scale a logical pixel value and clamp to [min, max].
    #[inline]
    pub fn scaled_clamped(&self, px: f32, min: f32, max: f32) -> f32 {
        (px * self.ui_scale).clamp(min, max)
    }

    /// Scale a logical pixel value with a minimum bound.
    #[inline]
    pub fn scaled_max(&self, px: f32, min: f32) -> f32 {
        (px * self.ui_scale).max(min)
    }

    /// Scale a logical pixel value with a maximum bound.
    #[inline]
    pub fn scaled_min(&self, px: f32, max: f32) -> f32 {
        (px * self.ui_scale).min(max)
    }

    /// Resolve an explicit 2D layout rectangle from logical coordinates.
    #[inline]
    pub fn resolve_rect(&self, x: f32, y: f32, w: f32, h: f32) -> LayoutRect {
        LayoutRect::new(x, y, w, h)
    }

    /// Clamp window drag Y coordinate relative to top bar clearance.
    #[inline]
    pub fn clamp_drag_y(&self, y: f32, offset_y: f32, viewport_h: f32) -> f32 {
        let min_y = 52.0 * self.ui_scale;
        let max_y = (viewport_h - 60.0).max(min_y);
        (y - offset_y).clamp(min_y, max_y)
    }

    /// Full viewport rectangle covering the entire display canvas.
    #[inline]
    pub fn viewport_rect(&self) -> LayoutRect {
        LayoutRect::new(0.0, 0.0, self.viewport_w, self.viewport_h)
    }

    /// Centered modal rectangle with given width and height at a vertical position ratio.
    #[inline]
    pub fn centered_rect(&self, w: f32, h: f32, y_ratio: f32) -> LayoutRect {
        let actual_w = w.min(self.viewport_w - 20.0);
        let actual_h = h.min(self.viewport_h - 40.0);
        let x = (self.viewport_w - actual_w) * 0.5;
        let y = (self.viewport_h - actual_h) * y_ratio;
        LayoutRect::new(x, y, actual_w, actual_h)
    }

    /// Derived sub-rectangle relative to a base rectangle.
    #[inline]
    pub fn sub_rect(&self, base: LayoutRect, rel_x: f32, rel_y: f32, w: f32, h: f32) -> LayoutRect {
        LayoutRect::new(base.x + rel_x, base.y + rel_y, w, h)
    }

    /// Fill rectangle for progress bars.
    #[inline]
    pub fn progress_fill(&self, bar: LayoutRect, progress: f32) -> LayoutRect {
        LayoutRect::new(bar.x, bar.y, bar.w * progress.clamp(0.0, 1.0), bar.h)
    }

    /// High-contrast toast notification pill rect.
    #[inline]
    pub fn toast_rect(&self, text_w: f32, text_h: f32) -> LayoutRect {
        let s = self.ui_scale;
        let toast_w = (text_w + 36.0 * s).clamp(180.0 * s, self.viewport_w - 40.0 * s);
        let toast_h = (text_h + 16.0 * s).clamp(32.0 * s, 44.0 * s);
        let toast_y = (self.viewport_h - 118.0 * s).max(48.0);
        LayoutRect::new((self.viewport_w - toast_w) * 0.5, toast_y, toast_w, toast_h)
    }

    /// Grid cell position for keypads and icon matrices.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn grid_cell(
        &self,
        base_x: f32,
        base_y: f32,
        col: usize,
        row: usize,
        cell_w: f32,
        cell_h: f32,
        gap_x: f32,
        gap_y: f32,
    ) -> LayoutRect {
        let x = base_x + (col as f32) * (cell_w + gap_x);
        let y = base_y + (row as f32) * (cell_h + gap_y);
        LayoutRect::new(x, y, cell_w, cell_h)
    }

    /// Row item rectangle with uniform height.
    #[inline]
    pub fn list_row(
        &self,
        base_x: f32,
        base_y: f32,
        width: f32,
        height: f32,
        idx: usize,
        gap: f32,
    ) -> LayoutRect {
        let y = base_y + (idx as f32) * (height + gap);
        LayoutRect::new(base_x, y, width, height)
    }

    /// Card child item rectangle placed at relative offsets.
    #[inline]
    pub fn card_item(
        &self,
        card: LayoutRect,
        x_offset: f32,
        y_offset: f32,
        w: f32,
        h: f32,
    ) -> LayoutRect {
        LayoutRect::new(card.x + x_offset, card.y + y_offset, w, h)
    }
}
