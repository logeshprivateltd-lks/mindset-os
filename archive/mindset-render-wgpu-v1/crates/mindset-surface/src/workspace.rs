//! Virtual workspaces and Alt+Tab window switching manager.

/// Number of available virtual workspaces in MindSet OS.
pub const NUM_WORKSPACES: u8 = 4;

pub const WORKSPACE_LABELS: [&str; 4] = ["[WS 1]", "[WS 2]", "[WS 3]", "[WS 4]"];

/// Virtual workspace coordinator.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceManager {
    pub active_workspace: u8,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            active_workspace: 0,
        }
    }

    pub fn switch_workspace(&mut self, target: u8) -> bool {
        if target < NUM_WORKSPACES {
            self.active_workspace = target;
            true
        } else {
            false
        }
    }

    pub fn next_workspace(&mut self) {
        self.active_workspace = (self.active_workspace + 1) % NUM_WORKSPACES;
    }

    pub fn prev_workspace(&mut self) {
        self.active_workspace = if self.active_workspace == 0 {
            NUM_WORKSPACES - 1
        } else {
            self.active_workspace - 1
        };
    }

    pub fn active_label(&self) -> &'static str {
        WORKSPACE_LABELS[self.active_workspace as usize]
    }
}

/// Alt+Tab window cycling state.
#[derive(Debug, Clone, Default)]
pub struct AltTabSwitcher {
    pub is_visible: bool,
    pub selected_index: usize,
}

impl AltTabSwitcher {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            selected_index: 0,
        }
    }

    pub fn open(&mut self) {
        self.is_visible = true;
        self.selected_index = 0;
    }

    pub fn close(&mut self) {
        self.is_visible = false;
    }

    pub fn cycle_next(&mut self, open_count: usize) {
        if open_count > 0 {
            self.selected_index = (self.selected_index + 1) % open_count;
        }
    }

    pub fn cycle_prev(&mut self, open_count: usize) {
        if open_count > 0 {
            self.selected_index = if self.selected_index == 0 {
                open_count - 1
            } else {
                self.selected_index - 1
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_switching_and_bounds() {
        let mut wm = WorkspaceManager::new();
        assert_eq!(wm.active_workspace, 0);
        assert_eq!(wm.active_label(), "[WS 1]");

        assert!(wm.switch_workspace(2));
        assert_eq!(wm.active_workspace, 2);
        assert_eq!(wm.active_label(), "[WS 3]");

        // Out of bounds
        assert!(!wm.switch_workspace(4));
        assert_eq!(wm.active_workspace, 2);

        // Next wrapping
        wm.next_workspace(); // 3
        assert_eq!(wm.active_workspace, 3);
        wm.next_workspace(); // 0
        assert_eq!(wm.active_workspace, 0);

        // Prev wrapping
        wm.prev_workspace(); // 3
        assert_eq!(wm.active_workspace, 3);
    }

    #[test]
    fn test_alt_tab_cycling() {
        let mut switcher = AltTabSwitcher::new();
        switcher.open();
        assert_eq!(switcher.selected_index, 0);

        // 3 open windows: 0 -> 1 -> 2 -> 0
        switcher.cycle_next(3);
        assert_eq!(switcher.selected_index, 1);
        switcher.cycle_next(3);
        assert_eq!(switcher.selected_index, 2);
        switcher.cycle_next(3);
        assert_eq!(switcher.selected_index, 0);

        // Cycle prev: 0 -> 2 -> 1
        switcher.cycle_prev(3);
        assert_eq!(switcher.selected_index, 2);
        switcher.cycle_prev(3);
        assert_eq!(switcher.selected_index, 1);
    }
}
