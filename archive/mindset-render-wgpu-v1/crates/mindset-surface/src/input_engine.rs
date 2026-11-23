//! Keyboard text-editing buffer, smooth mouse-wheel scrolling, and global hotkeys.

use crate::clipboard::SystemClipboard;
use mindset_render::safe_utf8_prefix;
use std::ops::Range;

/// An interactive single-line or multi-line text input buffer with UTF-8 character safety and selection.
#[derive(Debug, Clone, PartialEq)]
pub struct TextInputBuffer {
    pub text: String,
    /// Cursor position measured in Unicode scalar count (NOT byte index).
    pub cursor_char: usize,
    pub selection: Option<Range<usize>>,
    pub is_focused: bool,
    pub blink_timer: f32,
    pub placeholder: String,
}

impl TextInputBuffer {
    pub fn new(placeholder: &str) -> Self {
        Self {
            text: String::new(),
            cursor_char: 0,
            selection: None,
            is_focused: false,
            blink_timer: 0.0,
            placeholder: placeholder.to_string(),
        }
    }

    pub fn with_initial_text(text: &str, placeholder: &str) -> Self {
        let char_count = text.chars().count();
        Self {
            text: text.to_string(),
            cursor_char: char_count,
            selection: None,
            is_focused: false,
            blink_timer: 0.0,
            placeholder: placeholder.to_string(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_focused {
            self.blink_timer = (self.blink_timer + dt) % 1.0;
        } else {
            self.blink_timer = 0.0;
        }
    }

    /// True during the active half-second of the blink cycle.
    pub fn is_cursor_visible(&self) -> bool {
        self.is_focused && self.blink_timer < 0.53
    }

    pub fn total_chars(&self) -> usize {
        self.text.chars().count()
    }

    pub fn select_all(&mut self) {
        let total = self.total_chars();
        if total > 0 {
            self.selection = Some(0..total);
            self.cursor_char = total;
            self.blink_timer = 0.0;
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
                self.cursor_char = start;
                self.blink_timer = 0.0;
                return true;
            }
        }
        false
    }

    pub fn copy(&self, clipboard: &SystemClipboard) {
        if let Some(selected) = self.selected_text() {
            clipboard.set_text(&selected);
        } else if !self.text.is_empty() {
            clipboard.set_text(&self.text);
        }
    }

    pub fn cut(&mut self, clipboard: &SystemClipboard) {
        if let Some(selected) = self.selected_text() {
            clipboard.set_text(&selected);
            self.delete_selection();
        } else if !self.text.is_empty() {
            clipboard.set_text(&self.text);
            self.clear();
        }
    }

    pub fn paste(&mut self, clipboard: &SystemClipboard) {
        let text_to_paste = clipboard.get_text();
        if !text_to_paste.is_empty() {
            self.delete_selection();
            self.insert_str(&text_to_paste);
        }
    }

    /// Insert a character at the current cursor position with guaranteed UTF-8 safety.
    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();
        let prefix = safe_utf8_prefix(&self.text, self.cursor_char);
        let prefix_len = prefix.len();

        let mut new_text = String::with_capacity(self.text.len() + c.len_utf8());
        new_text.push_str(&self.text[..prefix_len]);
        new_text.push(c);
        new_text.push_str(&self.text[prefix_len..]);

        self.text = new_text;
        self.cursor_char += 1;
        self.blink_timer = 0.0;
    }

    /// Insert a string at the cursor.
    pub fn insert_str(&mut self, s: &str) {
        for c in s.chars() {
            self.insert_char(c);
        }
    }

    /// Delete character immediately to the left of the cursor (Backspace).
    pub fn backspace(&mut self) -> bool {
        if self.delete_selection() {
            return true;
        }
        if self.cursor_char == 0 {
            return false;
        }

        let delete_idx = self.cursor_char - 1;
        let prefix = safe_utf8_prefix(&self.text, delete_idx);
        let prefix_bytes = prefix.len();

        let after = safe_utf8_prefix(&self.text, self.cursor_char);
        let after_bytes = after.len();

        let mut new_text = String::with_capacity(self.text.len());
        new_text.push_str(&self.text[..prefix_bytes]);
        new_text.push_str(&self.text[after_bytes..]);

        self.text = new_text;
        self.cursor_char -= 1;
        self.blink_timer = 0.0;
        true
    }

    /// Delete character immediately to the right of the cursor (Delete).
    pub fn delete(&mut self) -> bool {
        if self.delete_selection() {
            return true;
        }
        let total = self.total_chars();
        if self.cursor_char >= total {
            return false;
        }

        let prefix = safe_utf8_prefix(&self.text, self.cursor_char);
        let prefix_bytes = prefix.len();

        let after = safe_utf8_prefix(&self.text, self.cursor_char + 1);
        let after_bytes = after.len();

        let mut new_text = String::with_capacity(self.text.len());
        new_text.push_str(&self.text[..prefix_bytes]);
        new_text.push_str(&self.text[after_bytes..]);

        self.text = new_text;
        self.blink_timer = 0.0;
        true
    }

    pub fn move_left(&mut self) {
        self.clear_selection();
        if self.cursor_char > 0 {
            self.cursor_char -= 1;
            self.blink_timer = 0.0;
        }
    }

    pub fn move_left_selection(&mut self, select: bool) {
        if select {
            let old_pos = self.cursor_char;
            if self.cursor_char > 0 {
                self.cursor_char -= 1;
                match self.selection.take() {
                    Some(range) => {
                        let new_start = range.start.min(self.cursor_char);
                        let new_end = range.end.max(old_pos);
                        self.selection = Some(new_start..new_end);
                    }
                    None => {
                        self.selection = Some(self.cursor_char..old_pos);
                    }
                }
            }
        } else {
            self.move_left();
        }
    }

    pub fn move_right(&mut self) {
        self.clear_selection();
        let total = self.total_chars();
        if self.cursor_char < total {
            self.cursor_char += 1;
            self.blink_timer = 0.0;
        }
    }

    pub fn move_right_selection(&mut self, select: bool) {
        if select {
            let old_pos = self.cursor_char;
            let total = self.total_chars();
            if self.cursor_char < total {
                self.cursor_char += 1;
                match self.selection.take() {
                    Some(range) => {
                        let new_start = range.start.min(old_pos);
                        let new_end = range.end.max(self.cursor_char);
                        self.selection = Some(new_start..new_end);
                    }
                    None => {
                        self.selection = Some(old_pos..self.cursor_char);
                    }
                }
            }
        } else {
            self.move_right();
        }
    }

    pub fn move_home(&mut self) {
        self.clear_selection();
        self.cursor_char = 0;
        self.blink_timer = 0.0;
    }

    pub fn move_home_selection(&mut self, select: bool) {
        if select {
            let old_pos = self.cursor_char;
            self.cursor_char = 0;
            self.selection = Some(0..old_pos);
        } else {
            self.move_home();
        }
    }

    pub fn move_end(&mut self) {
        self.clear_selection();
        self.cursor_char = self.total_chars();
        self.blink_timer = 0.0;
    }

    pub fn move_end_selection(&mut self, select: bool) {
        if select {
            let old_pos = self.cursor_char;
            let total = self.total_chars();
            self.cursor_char = total;
            self.selection = Some(old_pos..total);
        } else {
            self.move_end();
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_char = 0;
        self.selection = None;
        self.blink_timer = 0.0;
    }

    /// Substring before cursor.
    pub fn text_before_cursor(&self) -> &str {
        safe_utf8_prefix(&self.text, self.cursor_char)
    }

    /// Substring after cursor.
    pub fn text_after_cursor(&self) -> &str {
        let prefix = safe_utf8_prefix(&self.text, self.cursor_char);
        &self.text[prefix.len()..]
    }
}

/// Command Launcher (`Ctrl + K`) state and search query filter.
#[derive(Debug, Clone)]
pub struct CommandLauncher {
    pub is_open: bool,
    pub input: TextInputBuffer,
    pub selected_index: usize,
}

impl Default for CommandLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandLauncher {
    pub fn new() -> Self {
        Self {
            is_open: false,
            input: TextInputBuffer::new("Type an app name, command, or shortcut..."),
            selected_index: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.input.is_focused = true;
            self.input.clear();
            self.selected_index = 0;
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.input.is_focused = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_insert_and_cursor_navigation() {
        let mut buf = TextInputBuffer::new("Type here...");
        buf.insert_str("Hello");
        assert_eq!(buf.text, "Hello");
        assert_eq!(buf.cursor_char, 5);

        buf.move_left();
        buf.move_left();
        assert_eq!(buf.cursor_char, 3);

        buf.insert_char('!');
        assert_eq!(buf.text, "Hel!lo");
        assert_eq!(buf.cursor_char, 4);

        buf.move_home();
        assert_eq!(buf.cursor_char, 0);
        buf.insert_char('>');
        assert_eq!(buf.text, ">Hel!lo");

        buf.move_end();
        assert_eq!(buf.cursor_char, 7);
    }

    #[test]
    fn test_utf8_multibyte_backspace_safety() {
        let mut buf = TextInputBuffer::new("Formula");
        // Formula with multi-byte math symbols: ∮ (3 bytes), ε₀ (2+3 bytes)
        buf.insert_str("Flux: ∮ E·dA = Q/ε₀");
        assert!(std::str::from_utf8(buf.text.as_bytes()).is_ok());

        // Backspace ε₀
        buf.backspace();
        assert!(std::str::from_utf8(buf.text.as_bytes()).is_ok());
        buf.backspace();
        assert!(std::str::from_utf8(buf.text.as_bytes()).is_ok());

        // Backspace through Tamil text
        let mut tamil_buf = TextInputBuffer::new("Tamil");
        tamil_buf.insert_str("ஆழ்மனதின் சக்தி");
        assert!(std::str::from_utf8(tamil_buf.text.as_bytes()).is_ok());

        while tamil_buf.total_chars() > 0 {
            let ok = tamil_buf.backspace();
            assert!(ok);
            assert!(std::str::from_utf8(tamil_buf.text.as_bytes()).is_ok());
        }
        assert_eq!(tamil_buf.text, "");
        assert_eq!(tamil_buf.cursor_char, 0);
    }

    #[test]
    fn test_delete_and_clamping() {
        let mut buf = TextInputBuffer::new("");
        buf.insert_str("ABCD");
        buf.move_home();
        assert_eq!(buf.cursor_char, 0);

        buf.delete();
        assert_eq!(buf.text, "BCD");
        assert_eq!(buf.cursor_char, 0);

        buf.move_end();
        assert!(!buf.delete()); // Delete past end is a no-op
        assert_eq!(buf.text, "BCD");
    }
}
