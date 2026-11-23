//! Mouse hit-testing, pointer capture, clipboard engine, and keyboard focus routing.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::ElementId;
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Event routing phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}

/// The interaction state of a pointer over a widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerState {
    Idle,
    Hover,
    Pressed,
    Released,
}

/// Result of a hit-test for a specific widget.
#[derive(Debug, Clone, Copy)]
pub struct HitResult {
    pub widget_id: u32,
    pub state: PointerState,
}

/// System-level shared clipboard engine.
#[derive(Debug, Clone, Default)]
pub struct ClipboardEngine {
    text: String,
    last_copy_toast: Option<String>,
}

impl ClipboardEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copy(&mut self, text: &str) {
        self.text = text.to_string();
        self.last_copy_toast = Some(format!(
            "Copied {} chars to clipboard",
            text.chars().count()
        ));
    }

    pub fn paste(&self) -> &str {
        &self.text
    }

    pub fn take_toast(&mut self) -> Option<String> {
        self.last_copy_toast.take()
    }
}

/// Keyboard modifiers tracker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModifiersState {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub logo: bool,
}

/// Global input state updated each frame from winit events.
#[derive(Debug, Clone)]
pub struct InputState {
    pub cursor: (f32, f32),
    pub left_down: bool,
    pub right_down: bool,
    pub just_released: bool,
    pub right_released: bool,
    pub focused_widget: Option<u32>,
    pub focus_chain: Vec<u32>,
    pub focused_element: Option<ElementId>,
    pub captured_element: Option<ElementId>,
    pub keyboard_char: Option<char>,
    pub modifiers: ModifiersState,
    pub clipboard: ClipboardEngine,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            cursor: (0.0, 0.0),
            left_down: false,
            right_down: false,
            just_released: false,
            right_released: false,
            focused_widget: None,
            focus_chain: Vec::new(),
            focused_element: None,
            captured_element: None,
            keyboard_char: None,
            modifiers: ModifiersState::default(),
            clipboard: ClipboardEngine::new(),
        }
    }

    pub fn register_focusable(&mut self, widget_id: u32) {
        if !self.focus_chain.contains(&widget_id) {
            self.focus_chain.push(widget_id);
        }
    }

    pub fn clear_focus_chain(&mut self) {
        self.focus_chain.clear();
    }

    pub fn set_focus(&mut self, widget_id: u32) {
        self.focused_widget = Some(widget_id);
    }

    pub fn clear_focus(&mut self) {
        self.focused_widget = None;
    }

    /// Advances focus to the next widget in the focus chain (Tab).
    pub fn focus_next(&mut self) {
        if self.focus_chain.is_empty() {
            return;
        }
        let next_idx = match self.focused_widget {
            Some(curr) => {
                if let Some(pos) = self.focus_chain.iter().position(|&id| id == curr) {
                    (pos + 1) % self.focus_chain.len()
                } else {
                    0
                }
            }
            None => 0,
        };
        self.focused_widget = Some(self.focus_chain[next_idx]);
    }

    /// Moves focus to the previous widget in the focus chain (Shift + Tab).
    pub fn focus_prev(&mut self) {
        if self.focus_chain.is_empty() {
            return;
        }
        let prev_idx = match self.focused_widget {
            Some(curr) => {
                if let Some(pos) = self.focus_chain.iter().position(|&id| id == curr) {
                    if pos == 0 {
                        self.focus_chain.len() - 1
                    } else {
                        pos - 1
                    }
                } else {
                    0
                }
            }
            None => self.focus_chain.len() - 1,
        };
        self.focused_widget = Some(self.focus_chain[prev_idx]);
    }

    pub fn update_cursor(&mut self, x: f32, y: f32) {
        self.cursor = (x, y);
    }

    pub fn press(&mut self) {
        self.left_down = true;
        self.just_released = false;
    }

    pub fn release(&mut self) {
        self.left_down = false;
        self.just_released = true;
        self.captured_element = None;
    }

    pub fn press_right(&mut self) {
        self.right_down = true;
        self.right_released = false;
    }

    pub fn release_right(&mut self) {
        self.right_down = false;
        self.right_released = true;
    }

    pub fn capture_pointer(&mut self, id: ElementId) {
        self.captured_element = Some(id);
    }

    pub fn release_pointer_capture(&mut self) {
        self.captured_element = None;
    }

    /// End-of-frame reset — call after processing all click and char events.
    pub fn end_frame(&mut self) {
        self.just_released = false;
        self.right_released = false;
        self.keyboard_char = None;
    }

    /// Performs a hit-test against `rect` for the given `widget_id`.
    pub fn hit_test(&self, rect: LayoutRect, id: u32) -> HitResult {
        let (cx, cy) = self.cursor;
        if rect.contains(cx, cy) {
            let state = if self.just_released {
                PointerState::Released
            } else if self.left_down {
                PointerState::Pressed
            } else {
                PointerState::Hover
            };
            HitResult {
                widget_id: id,
                state,
            }
        } else {
            HitResult {
                widget_id: id,
                state: PointerState::Idle,
            }
        }
    }

    /// Returns true when the cursor is inside `rect` and the left button was just released (a click).
    pub fn clicked(&self, rect: LayoutRect) -> bool {
        self.just_released && rect.contains(self.cursor.0, self.cursor.1)
    }

    /// Returns true when the cursor is currently hovering over `rect`.
    pub fn hovered(&self, rect: LayoutRect) -> bool {
        rect.contains(self.cursor.0, self.cursor.1)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Rich text buffer with selection range, cursor navigation, and full clipboard operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditableTextBuffer {
    pub text: String,
    pub cursor: usize,
    pub selection: Option<Range<usize>>,
    pub placeholder: String,
    pub is_focused: bool,
}

impl EditableTextBuffer {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            selection: None,
            placeholder: placeholder.into(),
            is_focused: false,
        }
    }

    pub fn with_text(text: impl Into<String>, placeholder: impl Into<String>) -> Self {
        let t = text.into();
        let len = t.chars().count();
        Self {
            text: t,
            cursor: len,
            selection: None,
            placeholder: placeholder.into(),
            is_focused: false,
        }
    }

    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn select_all(&mut self) {
        let count = self.char_count();
        if count > 0 {
            self.selection = Some(0..count);
            self.cursor = count;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn selected_text(&self) -> Option<String> {
        let range = self.selection.as_ref()?;
        let chars: Vec<char> = self.text.chars().collect();
        let start = range.start.min(chars.len());
        let end = range.end.min(chars.len());
        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    pub fn delete_selection(&mut self) -> bool {
        if let Some(range) = self.selection.take() {
            let mut chars: Vec<char> = self.text.chars().collect();
            let start = range.start.min(chars.len());
            let end = range.end.min(chars.len());
            if start < end {
                chars.drain(start..end);
                self.text = chars.into_iter().collect();
                self.cursor = start;
                return true;
            }
        }
        false
    }

    pub fn copy(&self, clipboard: &mut ClipboardEngine) {
        if let Some(selected) = self.selected_text() {
            clipboard.copy(&selected);
        } else if !self.text.is_empty() {
            clipboard.copy(&self.text);
        }
    }

    pub fn cut(&mut self, clipboard: &mut ClipboardEngine) {
        if let Some(selected) = self.selected_text() {
            clipboard.copy(&selected);
            self.delete_selection();
        }
    }

    pub fn paste(&mut self, clipboard: &ClipboardEngine) {
        let text_to_paste = clipboard.paste();
        if !text_to_paste.is_empty() {
            self.delete_selection();
            self.insert_str(text_to_paste);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        let pos = self.cursor.min(chars.len());
        chars.insert(pos, c);
        self.text = chars.into_iter().collect();
        self.cursor = pos + 1;
    }

    pub fn insert_str(&mut self, s: &str) {
        self.delete_selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        let pos = self.cursor.min(chars.len());
        let to_insert: Vec<char> = s.chars().collect();
        let added = to_insert.len();
        chars.splice(pos..pos, to_insert);
        self.text = chars.into_iter().collect();
        self.cursor = pos + added;
    }

    pub fn backspace(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.cursor > 0 {
            let mut chars: Vec<char> = self.text.chars().collect();
            let pos = (self.cursor - 1).min(chars.len() - 1);
            chars.remove(pos);
            self.text = chars.into_iter().collect();
            self.cursor = pos;
        }
    }

    pub fn delete(&mut self) {
        if self.delete_selection() {
            return;
        }
        let mut chars: Vec<char> = self.text.chars().collect();
        if self.cursor < chars.len() {
            chars.remove(self.cursor);
            self.text = chars.into_iter().collect();
        }
    }

    pub fn move_left(&mut self, shift: bool) {
        if self.cursor > 0 {
            let prev = self.cursor;
            self.cursor -= 1;
            if shift {
                let start = self.selection.as_ref().map_or(prev, |r| r.start);
                self.selection = Some(self.cursor.min(start)..self.cursor.max(start));
            } else {
                self.selection = None;
            }
        }
    }

    pub fn move_right(&mut self, shift: bool) {
        let max = self.char_count();
        if self.cursor < max {
            let prev = self.cursor;
            self.cursor += 1;
            if shift {
                let start = self.selection.as_ref().map_or(prev, |r| r.start);
                self.selection = Some(self.cursor.min(start)..self.cursor.max(start));
            } else {
                self.selection = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_copy_cut_paste_select_all() {
        let mut clipboard = ClipboardEngine::new();
        let mut buf = EditableTextBuffer::with_text("Hello World", "placeholder");

        // Select All (Ctrl+A)
        buf.select_all();
        assert_eq!(buf.selected_text(), Some("Hello World".to_string()));

        // Copy (Ctrl+C)
        buf.copy(&mut clipboard);
        assert_eq!(clipboard.paste(), "Hello World");

        // Cut (Ctrl+X)
        buf.cut(&mut clipboard);
        assert_eq!(buf.text, "");
        assert_eq!(buf.cursor, 0);

        // Paste (Ctrl+V)
        buf.paste(&clipboard);
        assert_eq!(buf.text, "Hello World");
        assert_eq!(buf.cursor, 11);
    }

    #[test]
    fn test_pointer_capture_lifecycle() {
        let mut input = InputState::new();
        let el = ElementId::from_raw(99);
        assert!(input.captured_element.is_none());

        input.capture_pointer(el);
        assert_eq!(input.captured_element, Some(el));

        input.release();
        assert!(input.captured_element.is_none());
    }
}
