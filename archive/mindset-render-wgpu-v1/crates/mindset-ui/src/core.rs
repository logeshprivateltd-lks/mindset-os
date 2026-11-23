//! Core retained component model, ElementId, Component lifecycle, and dirty flags.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Stable, deterministic identifier for every element in the retained UI tree.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ElementId(pub u64);

impl ElementId {
    /// Generates a deterministic ElementId by hashing a name and parent/index salt.
    pub fn from_name_and_index(name: &str, index: usize) -> Self {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        index.hash(&mut hasher);
        Self(hasher.finish())
    }

    /// Creates an ElementId from a raw u64.
    pub const fn from_raw(val: u64) -> Self {
        Self(val)
    }

    /// Helper for root element.
    pub const ROOT: Self = Self(1);
}

impl fmt::Debug for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElementId(0x{:016X})", self.0)
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08X}", self.0 as u32)
    }
}

impl Hash for ElementId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// The 8 universal interaction states for all MindSet UI widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WidgetState {
    #[default]
    Normal,
    Hover,
    Pressed,
    Focused,
    Disabled,
    Selected,
    Loading,
    Error,
}

impl WidgetState {
    pub fn is_interactive(&self) -> bool {
        !matches!(self, WidgetState::Disabled | WidgetState::Loading)
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self, WidgetState::Pressed)
    }

    pub fn is_hovered(&self) -> bool {
        matches!(self, WidgetState::Hover | WidgetState::Pressed)
    }

    pub fn is_focused(&self) -> bool {
        matches!(self, WidgetState::Focused)
    }
}

/// Granular dirty invalidation flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirtyFlags {
    pub layout: bool,
    pub paint: bool,
    pub subtree: bool,
}

impl DirtyFlags {
    pub const CLEAN: Self = Self {
        layout: false,
        paint: false,
        subtree: false,
    };
    pub const DIRTY_PAINT: Self = Self {
        layout: false,
        paint: true,
        subtree: false,
    };
    pub const DIRTY_LAYOUT: Self = Self {
        layout: true,
        paint: true,
        subtree: false,
    };
    pub const DIRTY_SUBTREE: Self = Self {
        layout: true,
        paint: true,
        subtree: true,
    };

    pub fn is_clean(&self) -> bool {
        !self.layout && !self.paint && !self.subtree
    }

    pub fn mark_layout(&mut self) {
        self.layout = true;
        self.paint = true;
    }

    pub fn mark_paint(&mut self) {
        self.paint = true;
    }

    pub fn mark_subtree(&mut self) {
        self.layout = true;
        self.paint = true;
        self.subtree = true;
    }

    pub fn clear(&mut self) {
        self.layout = false;
        self.paint = false;
        self.subtree = false;
    }
}

/// Component lifecycle hooks and abstract trait.
pub trait Component: Send + Sync {
    /// Returns the element's id.
    fn id(&self) -> ElementId;

    /// Called when the component is first mounted in the tree.
    fn on_mount(&mut self) {}

    /// Called on each frame update.
    fn on_update(&mut self, _dt: f32) {}

    /// Measures the intrinsic size of the component given maximum constraints.
    fn measure(&self, max_w: f32, max_h: f32) -> (f32, f32);

    /// Lay out self and compute rect bounds.
    fn layout(&mut self, rect: LayoutRect);

    /// Checks if a point hit-tests inside this component.
    fn hit_test(&self, px: f32, py: f32, rect: LayoutRect) -> bool {
        rect.contains(px, py)
    }
}

/// Retained node inside `UiTree`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub id: ElementId,
    pub name: String,
    pub parent: Option<ElementId>,
    pub children: Vec<ElementId>,
    pub rect: LayoutRect,
    pub dirty: DirtyFlags,
    pub state: WidgetState,
    pub is_visible: bool,
    pub z_index: u32,
    pub accessibility_label: String,
    pub tooltip: Option<String>,
}

impl ComponentNode {
    pub fn new(id: ElementId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            parent: None,
            children: Vec::new(),
            rect: LayoutRect::zero(),
            dirty: DirtyFlags::DIRTY_LAYOUT,
            state: WidgetState::Normal,
            is_visible: true,
            z_index: 0,
            accessibility_label: String::new(),
            tooltip: None,
        }
    }

    pub fn with_rect(mut self, rect: LayoutRect) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_parent(mut self, parent: ElementId) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn add_child(&mut self, child_id: ElementId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
            self.dirty.mark_subtree();
        }
    }

    pub fn remove_child(&mut self, child_id: ElementId) {
        if let Some(pos) = self.children.iter().position(|&c| c == child_id) {
            self.children.remove(pos);
            self.dirty.mark_subtree();
        }
    }
}
