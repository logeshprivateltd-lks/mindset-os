//! Reusable Form & Input Widgets with 8 interaction states and full clipboard support.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::{ElementId, WidgetState};
use crate::input::EditableTextBuffer;
use serde::{Deserialize, Serialize};

/// Checkbox widget with 8 interaction states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkbox {
    pub id: ElementId,
    pub label: String,
    pub is_checked: bool,
    pub state: WidgetState,
}

impl Checkbox {
    pub fn new(id: ElementId, label: impl Into<String>, is_checked: bool) -> Self {
        Self {
            id,
            label: label.into(),
            is_checked,
            state: WidgetState::Normal,
        }
    }

    pub fn toggle(&mut self) -> bool {
        if self.state.is_interactive() {
            self.is_checked = !self.is_checked;
        }
        self.is_checked
    }
}

/// Radio Button widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioButton {
    pub id: ElementId,
    pub label: String,
    pub is_selected: bool,
    pub state: WidgetState,
}

impl RadioButton {
    pub fn new(id: ElementId, label: impl Into<String>, is_selected: bool) -> Self {
        Self {
            id,
            label: label.into(),
            is_selected,
            state: WidgetState::Normal,
        }
    }
}

/// Segmented Control (Pill-tab switcher).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentedControl {
    pub id: ElementId,
    pub options: Vec<String>,
    pub selected_idx: usize,
    pub state: WidgetState,
}

impl SegmentedControl {
    pub fn new(id: ElementId, options: Vec<String>, selected_idx: usize) -> Self {
        Self {
            id,
            options,
            selected_idx,
            state: WidgetState::Normal,
        }
    }

    pub fn select(&mut self, idx: usize) -> bool {
        if idx < self.options.len() && self.state.is_interactive() {
            self.selected_idx = idx;
            true
        } else {
            false
        }
    }

    pub fn selected_text(&self) -> &str {
        &self.options[self.selected_idx]
    }
}

/// Dropdown Select popup control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownSelect {
    pub id: ElementId,
    pub label: String,
    pub options: Vec<String>,
    pub selected_idx: usize,
    pub is_open: bool,
    pub state: WidgetState,
}

impl DropdownSelect {
    pub fn new(
        id: ElementId,
        label: impl Into<String>,
        options: Vec<String>,
        selected_idx: usize,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            options,
            selected_idx,
            is_open: false,
            state: WidgetState::Normal,
        }
    }

    pub fn toggle_open(&mut self) {
        if self.state.is_interactive() {
            self.is_open = !self.is_open;
        }
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.options.len() {
            self.selected_idx = idx;
            self.is_open = false;
        }
    }

    pub fn selected_text(&self) -> &str {
        if self.options.is_empty() {
            ""
        } else {
            &self.options[self.selected_idx]
        }
    }
}

/// Color Swatch Picker for personalizing accent and theme highlights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSwatchPicker {
    pub id: ElementId,
    pub swatches: Vec<[f32; 4]>,
    pub selected_idx: usize,
    pub state: WidgetState,
}

impl ColorSwatchPicker {
    pub fn new(id: ElementId, swatches: Vec<[f32; 4]>, selected_idx: usize) -> Self {
        Self {
            id,
            swatches,
            selected_idx,
            state: WidgetState::Normal,
        }
    }

    pub fn selected_color(&self) -> [f32; 4] {
        self.swatches
            .get(self.selected_idx)
            .copied()
            .unwrap_or([0.184, 0.502, 0.929, 1.0])
    }
}

/// Search Field widget with prefix icon, clear button, and filter capability.
#[derive(Debug, Clone)]
pub struct SearchField {
    pub id: ElementId,
    pub buffer: EditableTextBuffer,
    pub state: WidgetState,
}

impl SearchField {
    pub fn new(id: ElementId, placeholder: impl Into<String>) -> Self {
        Self {
            id,
            buffer: EditableTextBuffer::new(placeholder),
            state: WidgetState::Normal,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.text.clear();
        self.buffer.cursor = 0;
        self.buffer.clear_selection();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmented_control_selection() {
        let mut sc = SegmentedControl::new(
            ElementId::from_raw(1),
            vec!["Day".to_string(), "Week".to_string(), "Month".to_string()],
            0,
        );
        assert_eq!(sc.selected_text(), "Day");
        assert!(sc.select(2));
        assert_eq!(sc.selected_text(), "Month");
        assert!(!sc.select(10)); // Out of bounds
    }

    #[test]
    fn test_dropdown_toggle_and_select() {
        let mut dd = DropdownSelect::new(
            ElementId::from_raw(2),
            "Language",
            vec![
                "English".to_string(),
                "Tamil".to_string(),
                "Hindi".to_string(),
            ],
            0,
        );
        assert!(!dd.is_open);
        dd.toggle_open();
        assert!(dd.is_open);
        dd.select(1);
        assert_eq!(dd.selected_text(), "Tamil");
        assert!(!dd.is_open);
    }
}
