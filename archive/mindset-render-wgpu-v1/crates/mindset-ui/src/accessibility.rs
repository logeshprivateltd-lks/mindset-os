//! Accessibility (A11y) semantic node tree, roles, and focus navigation.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::ElementId;
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};

/// Semantic accessibility roles according to W3C ARIA standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum A11yRole {
    Button,
    IconButton,
    Slider,
    ToggleSwitch,
    TextField,
    MultilineEditor,
    SearchField,
    Checkbox,
    RadioButton,
    Window,
    Dialog,
    Tab,
    TabList,
    TabPanel,
    List,
    ListItem,
    Header,
    Label,
    ProgressBar,
    StatusBadge,
    MenuBar,
    MenuItem,
    NavigationRail,
    VirtualList,
}

impl A11yRole {
    pub fn is_focusable(&self) -> bool {
        matches!(
            self,
            A11yRole::Button
                | A11yRole::IconButton
                | A11yRole::Slider
                | A11yRole::ToggleSwitch
                | A11yRole::TextField
                | A11yRole::MultilineEditor
                | A11yRole::SearchField
                | A11yRole::Checkbox
                | A11yRole::RadioButton
                | A11yRole::Tab
                | A11yRole::ListItem
                | A11yRole::MenuItem
        )
    }
}

/// Accessibility metadata bound to a UI element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityNode {
    pub id: ElementId,
    pub role: A11yRole,
    pub label: String,
    pub value_description: Option<String>,
    pub hint: Option<String>,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub is_expanded: Option<bool>,
    pub focus_order: usize,
    pub bounds: LayoutRect,
}

impl AccessibilityNode {
    pub fn new(id: ElementId, role: A11yRole, label: impl Into<String>) -> Self {
        Self {
            id,
            role,
            label: label.into(),
            value_description: None,
            hint: None,
            is_disabled: false,
            is_selected: false,
            is_expanded: None,
            focus_order: 0,
            bounds: LayoutRect::zero(),
        }
    }
}

/// Complete retained Accessibility Tree.
#[derive(Debug, Clone, Default)]
pub struct AccessibilityTree {
    pub nodes: std::collections::HashMap<ElementId, AccessibilityNode>,
    pub focus_list: Vec<ElementId>,
}

impl AccessibilityTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, node: AccessibilityNode) {
        let is_focus = node.role.is_focusable() && !node.is_disabled;
        let id = node.id;
        self.nodes.insert(id, node);
        if is_focus && !self.focus_list.contains(&id) {
            self.focus_list.push(id);
        }
    }

    pub fn get(&self, id: ElementId) -> Option<&AccessibilityNode> {
        self.nodes.get(&id)
    }

    pub fn next_focus(&self, current: Option<ElementId>) -> Option<ElementId> {
        if self.focus_list.is_empty() {
            return None;
        }
        match current {
            None => self.focus_list.first().copied(),
            Some(curr) => {
                if let Some(pos) = self.focus_list.iter().position(|&id| id == curr) {
                    let next_pos = (pos + 1) % self.focus_list.len();
                    Some(self.focus_list[next_pos])
                } else {
                    self.focus_list.first().copied()
                }
            }
        }
    }

    pub fn prev_focus(&self, current: Option<ElementId>) -> Option<ElementId> {
        if self.focus_list.is_empty() {
            return None;
        }
        match current {
            None => self.focus_list.last().copied(),
            Some(curr) => {
                if let Some(pos) = self.focus_list.iter().position(|&id| id == curr) {
                    let prev_pos = if pos == 0 {
                        self.focus_list.len() - 1
                    } else {
                        pos - 1
                    };
                    Some(self.focus_list[prev_pos])
                } else {
                    self.focus_list.last().copied()
                }
            }
        }
    }
}

/// AccessKit semantic role mapping according to the AccessKit specification / Windows UI Automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessKitRole {
    Window,
    Button,
    Switch,
    Slider,
    TextInput,
    Header,
    List,
    ListItem,
    CheckBox,
    RadioButton,
    Tab,
    TabList,
    TabPanel,
    Status,
    Alert,
    GenericContainer,
}

impl AccessKitRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Window => "Window",
            Self::Button => "Button",
            Self::Switch => "Switch",
            Self::Slider => "Slider",
            Self::TextInput => "TextInput",
            Self::Header => "Header",
            Self::List => "List",
            Self::ListItem => "ListItem",
            Self::CheckBox => "CheckBox",
            Self::RadioButton => "RadioButton",
            Self::Tab => "Tab",
            Self::TabList => "TabList",
            Self::TabPanel => "TabPanel",
            Self::Status => "Status",
            Self::Alert => "Alert",
            Self::GenericContainer => "GenericContainer",
        }
    }
}

impl A11yRole {
    pub fn to_accesskit_role(&self) -> AccessKitRole {
        match self {
            A11yRole::Button | A11yRole::IconButton => AccessKitRole::Button,
            A11yRole::Slider => AccessKitRole::Slider,
            A11yRole::ToggleSwitch => AccessKitRole::Switch,
            A11yRole::TextField | A11yRole::MultilineEditor | A11yRole::SearchField => {
                AccessKitRole::TextInput
            }
            A11yRole::Checkbox => AccessKitRole::CheckBox,
            A11yRole::RadioButton => AccessKitRole::RadioButton,
            A11yRole::Window => AccessKitRole::Window,
            A11yRole::Dialog => AccessKitRole::Window,
            A11yRole::Tab => AccessKitRole::Tab,
            A11yRole::TabList => AccessKitRole::TabList,
            A11yRole::TabPanel => AccessKitRole::TabPanel,
            A11yRole::List | A11yRole::VirtualList => AccessKitRole::List,
            A11yRole::ListItem => AccessKitRole::ListItem,
            A11yRole::Header => AccessKitRole::Header,
            A11yRole::Label => AccessKitRole::GenericContainer,
            A11yRole::ProgressBar => AccessKitRole::Status,
            A11yRole::StatusBadge => AccessKitRole::Status,
            A11yRole::MenuBar => AccessKitRole::GenericContainer,
            A11yRole::MenuItem => AccessKitRole::Button,
            A11yRole::NavigationRail => AccessKitRole::List,
        }
    }
}

/// Parallel AccessKit Node representation for Windows UI Automation and Narrator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessKitNodeDescriptor {
    pub id: u64,
    pub role: AccessKitRole,
    pub name: String,
    pub value: Option<String>,
    pub numeric_value: Option<f64>,
    pub min_numeric_value: Option<f64>,
    pub max_numeric_value: Option<f64>,
    pub numeric_step: Option<f64>,
    pub is_toggled: Option<bool>,
    pub is_disabled: bool,
    pub is_focusable: bool,
    pub is_focused: bool,
    pub is_expanded: Option<bool>,
    pub heading_level: Option<usize>,
    pub bounds: LayoutRect,
    pub children: Vec<u64>,
}

impl AccessKitNodeDescriptor {
    pub fn new(id: u64, role: AccessKitRole, name: impl Into<String>) -> Self {
        let is_focusable = matches!(
            role,
            AccessKitRole::Button
                | AccessKitRole::Switch
                | AccessKitRole::Slider
                | AccessKitRole::TextInput
                | AccessKitRole::CheckBox
                | AccessKitRole::RadioButton
                | AccessKitRole::Tab
                | AccessKitRole::ListItem
        );
        Self {
            id,
            role,
            name: name.into(),
            value: None,
            numeric_value: None,
            min_numeric_value: None,
            max_numeric_value: None,
            numeric_step: None,
            is_toggled: None,
            is_disabled: false,
            is_focusable,
            is_focused: false,
            is_expanded: None,
            heading_level: if role == AccessKitRole::Header {
                Some(2)
            } else {
                None
            },
            bounds: LayoutRect::zero(),
            children: Vec::new(),
        }
    }

    pub fn button(id: u64, label: impl Into<String>) -> Self {
        Self::new(id, AccessKitRole::Button, label)
    }

    pub fn switch(id: u64, label: impl Into<String>, is_on: bool) -> Self {
        let mut node = Self::new(id, AccessKitRole::Switch, label);
        node.is_toggled = Some(is_on);
        node
    }

    pub fn slider(
        id: u64,
        label: impl Into<String>,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    ) -> Self {
        let mut node = Self::new(id, AccessKitRole::Slider, label);
        node.numeric_value = Some(value);
        node.min_numeric_value = Some(min);
        node.max_numeric_value = Some(max);
        node.numeric_step = Some(step);
        let pct = if (max - min).abs() > 1e-5 {
            ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        node.value = Some(format!("{:.0}%", pct));
        node
    }

    pub fn text_input(id: u64, label: impl Into<String>, current_value: impl Into<String>) -> Self {
        let mut node = Self::new(id, AccessKitRole::TextInput, label);
        node.value = Some(current_value.into());
        node
    }

    pub fn window(id: u64, title: impl Into<String>) -> Self {
        Self::new(id, AccessKitRole::Window, title)
    }

    pub fn header(id: u64, title: impl Into<String>, level: usize) -> Self {
        let mut node = Self::new(id, AccessKitRole::Header, title);
        node.heading_level = Some(level);
        node
    }
}

impl AccessibilityNode {
    pub fn to_accesskit_descriptor(&self, is_focused: bool) -> AccessKitNodeDescriptor {
        let role = self.role.to_accesskit_role();
        let mut desc = AccessKitNodeDescriptor::new(self.id.0, role, self.label.clone());
        desc.is_disabled = self.is_disabled;
        desc.is_focused = is_focused;
        desc.bounds = self.bounds;
        desc.value = self.value_description.clone();
        desc.is_expanded = self.is_expanded;
        if role == AccessKitRole::Switch {
            desc.is_toggled = Some(self.is_selected);
        }
        desc
    }
}

/// Batch tree update delivered across the platform accessibility bridge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessKitTreeUpdate {
    pub root: u64,
    pub nodes: Vec<AccessKitNodeDescriptor>,
    pub focus: Option<u64>,
}

/// Screen reader live announcement priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivePoliteness {
    /// Spoken at the next user pause (toasts, background events).
    Polite,
    /// Immediately interrupts current speech (alerts, errors, timer expirations).
    Assertive,
}

/// High-visibility focus ring specification for keyboard navigation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FocusRing {
    pub bounds: LayoutRect,
    pub corner_radius: f32,
    pub stroke_width: f32,
    pub offset: f32,
    pub anim_t: f32,
}

impl FocusRing {
    pub const DEFAULT_STROKE_WIDTH: f32 = 2.0;
    pub const DEFAULT_OFFSET: f32 = 2.0;

    /// Solar Amber dark mode focus ring glow: rgba(245, 158, 11, 0.90)
    pub const COLOR_DARK: [f32; 4] = [0.961, 0.620, 0.043, 0.900];
    /// Solar Amber light mode focus ring glow: rgba(217, 119, 6, 1.00)
    pub const COLOR_LIGHT: [f32; 4] = [0.851, 0.467, 0.024, 1.000];

    pub fn new(bounds: LayoutRect, corner_radius: f32) -> Self {
        Self {
            bounds,
            corner_radius,
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            offset: Self::DEFAULT_OFFSET,
            anim_t: 1.0,
        }
    }

    /// Outer rectangle encompassing the stroke and offset.
    pub fn outer_rect(&self) -> LayoutRect {
        let expand = self.offset + self.stroke_width * 0.5;
        LayoutRect::new(
            self.bounds.x - expand,
            self.bounds.y - expand,
            self.bounds.w + expand * 2.0,
            self.bounds.h + expand * 2.0,
        )
    }

    /// Corner radius for the outer focus outline.
    pub fn outer_radius(&self) -> f32 {
        self.corner_radius + self.offset
    }

    /// High-visibility theme-dependent focus ring color with alpha animated by `anim_t`.
    pub fn color(&self, is_dark_mode: bool) -> [f32; 4] {
        let mut base = if is_dark_mode {
            Self::COLOR_DARK
        } else {
            Self::COLOR_LIGHT
        };
        base[3] *= self.anim_t.clamp(0.0, 1.0);
        base
    }
}

/// Modal focus trapping state machine to confine sequential Tab navigation within active dialogs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModalFocusTrap {
    pub active_modal_id: Option<ElementId>,
    pub focus_chain: Vec<ElementId>,
    pub current_idx: usize,
}

impl ModalFocusTrap {
    pub fn new(modal_id: ElementId, elements: Vec<ElementId>) -> Self {
        Self {
            active_modal_id: Some(modal_id),
            focus_chain: elements,
            current_idx: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active_modal_id.is_some() && !self.focus_chain.is_empty()
    }

    pub fn current(&self) -> Option<ElementId> {
        self.focus_chain.get(self.current_idx).copied()
    }

    pub fn next_focus(&mut self) -> Option<ElementId> {
        if self.focus_chain.is_empty() {
            return None;
        }
        self.current_idx = (self.current_idx + 1) % self.focus_chain.len();
        Some(self.focus_chain[self.current_idx])
    }

    pub fn prev_focus(&mut self) -> Option<ElementId> {
        if self.focus_chain.is_empty() {
            return None;
        }
        self.current_idx = if self.current_idx == 0 {
            self.focus_chain.len() - 1
        } else {
            self.current_idx - 1
        };
        Some(self.focus_chain[self.current_idx])
    }

    pub fn release(&mut self) {
        self.active_modal_id = None;
        self.focus_chain.clear();
        self.current_idx = 0;
    }
}

/// Live region screen reader announcement contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveAnnouncement {
    pub text: String,
    pub politeness: LivePoliteness,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accesskit_role_mappings() {
        assert_eq!(A11yRole::Button.to_accesskit_role(), AccessKitRole::Button);
        assert_eq!(
            A11yRole::ToggleSwitch.to_accesskit_role(),
            AccessKitRole::Switch
        );
        assert_eq!(A11yRole::Slider.to_accesskit_role(), AccessKitRole::Slider);
        assert_eq!(
            A11yRole::TextField.to_accesskit_role(),
            AccessKitRole::TextInput
        );
        assert_eq!(A11yRole::Header.to_accesskit_role(), AccessKitRole::Header);
        assert_eq!(A11yRole::Window.to_accesskit_role(), AccessKitRole::Window);
    }

    #[test]
    fn test_accesskit_node_builders() {
        let btn = AccessKitNodeDescriptor::button(101, "Launch Terminal");
        assert_eq!(btn.role, AccessKitRole::Button);
        assert!(btn.is_focusable);
        assert!(!btn.is_disabled);

        let sw = AccessKitNodeDescriptor::switch(102, "Dark Mode", true);
        assert_eq!(sw.role, AccessKitRole::Switch);
        assert_eq!(sw.is_toggled, Some(true));

        let sl = AccessKitNodeDescriptor::slider(103, "Volume", 0.75, 0.0, 1.0, 0.05);
        assert_eq!(sl.role, AccessKitRole::Slider);
        assert_eq!(sl.value, Some("75%".to_string()));

        let inp = AccessKitNodeDescriptor::text_input(104, "Search", "Cosmic Text");
        assert_eq!(inp.role, AccessKitRole::TextInput);
        assert_eq!(inp.value, Some("Cosmic Text".to_string()));

        let hdr = AccessKitNodeDescriptor::header(105, "Settings Overview", 1);
        assert_eq!(hdr.role, AccessKitRole::Header);
        assert_eq!(hdr.heading_level, Some(1));
    }

    #[test]
    fn test_accessibility_node_conversion_and_focus() {
        let mut tree = AccessibilityTree::new();
        let n1 = AccessibilityNode::new(ElementId::from_raw(1), A11yRole::Button, "Submit");
        let n2 = AccessibilityNode::new(ElementId::from_raw(2), A11yRole::ToggleSwitch, "WiFi");
        tree.insert(n1.clone());
        tree.insert(n2.clone());

        assert_eq!(tree.focus_list.len(), 2);
        let first = tree.next_focus(None);
        assert_eq!(first, Some(ElementId::from_raw(1)));
        let second = tree.next_focus(first);
        assert_eq!(second, Some(ElementId::from_raw(2)));
        let wrap = tree.next_focus(second);
        assert_eq!(wrap, Some(ElementId::from_raw(1)));

        let desc = n2.to_accesskit_descriptor(true);
        assert_eq!(desc.role, AccessKitRole::Switch);
        assert!(desc.is_focused);
    }

    #[test]
    fn test_focus_ring_outer_geometry_and_modal_trap() {
        let bounds = LayoutRect::new(20.0, 30.0, 120.0, 40.0);
        let ring = FocusRing::new(bounds, 8.0);
        assert_eq!(ring.outer_radius(), 10.0);

        let outer = ring.outer_rect();
        assert_eq!(outer.x, 17.0); // 20.0 - (2.0 + 1.0)
        assert_eq!(outer.y, 27.0);
        assert_eq!(outer.w, 126.0); // 120 + 6
        assert_eq!(outer.h, 46.0);

        let col_dark = ring.color(true);
        assert_eq!(col_dark[3], 0.90);

        // Test modal trap cycling
        let id1 = ElementId::from_raw(10);
        let id2 = ElementId::from_raw(11);
        let mut trap = ModalFocusTrap::new(ElementId::from_raw(99), vec![id1, id2]);
        assert!(trap.is_active());
        assert_eq!(trap.current(), Some(id1));
        assert_eq!(trap.next_focus(), Some(id2));
        assert_eq!(trap.next_focus(), Some(id1)); // cycle
        assert_eq!(trap.prev_focus(), Some(id2)); // reverse
    }
}
