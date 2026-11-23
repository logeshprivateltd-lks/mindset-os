//! Native System Clipboard Integration via arboard with headless fallback.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use arboard::Clipboard;
use std::sync::Mutex;

pub struct SystemClipboard {
    inner: Option<Mutex<Clipboard>>,
    fallback: Mutex<String>,
}

impl Default for SystemClipboard {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemClipboard {
    pub fn new() -> Self {
        let inner = match Clipboard::new() {
            Ok(cb) => Some(Mutex::new(cb)),
            Err(e) => {
                log::warn!(
                    "Failed to initialize OS clipboard (using headless fallback): {}",
                    e
                );
                None
            }
        };
        Self {
            inner,
            fallback: Mutex::new(String::new()),
        }
    }

    /// Retrieves text from the host operating system clipboard.
    pub fn get_text(&self) -> String {
        if let Some(ref cb_mutex) = self.inner {
            if let Ok(mut cb) = cb_mutex.lock() {
                if let Ok(text) = cb.get_text() {
                    return text;
                }
            }
        }
        self.fallback
            .lock()
            .map_or_else(|_| String::new(), |s| s.clone())
    }

    /// Sets text on the host operating system clipboard.
    pub fn set_text(&self, text: &str) {
        if let Some(ref cb_mutex) = self.inner {
            if let Ok(mut cb) = cb_mutex.lock() {
                let _ = cb.set_text(text.to_string());
            }
        }
        if let Ok(mut s) = self.fallback.lock() {
            *s = text.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clipboard_read_write() {
        let clipboard = SystemClipboard::new();
        clipboard.set_text("MindSet OS Native Clipboard 2026");
        let read = clipboard.get_text();
        assert_eq!(read, "MindSet OS Native Clipboard 2026");
    }
}
