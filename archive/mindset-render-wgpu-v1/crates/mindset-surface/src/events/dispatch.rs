//! MindSet OS — WindowRequest Execution Pass (`dispatch.rs`)
//! Pattern P3: Single atomic pass carrying out queued window requests with WindowManager.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::events::request::WindowRequest;
use crate::wm::WindowManager;

/// Drain and apply all queued WindowRequests in a single atomic pass.
pub fn apply_window_requests(requests: &[WindowRequest], wm: &mut WindowManager) {
    for req in requests {
        match req {
            WindowRequest::DragStart { window_id, cursor } => {
                if let Some(win) = wm.find_window_mut(*window_id) {
                    win.is_dragging = true;
                    win.drag_offset = (cursor.0 - win.rect.x, cursor.1 - win.rect.y);
                }
                wm.active_window_id = Some(*window_id);
            }
            WindowRequest::DragDelta { window_id, dx, dy } => {
                if let Some(win) = wm.find_window_mut(*window_id) {
                    win.rect.x += dx;
                    win.rect.y += dy;
                }
            }
            WindowRequest::ResizeStart { .. } => {
                // Handled by compositor resize grip state
            }
            WindowRequest::Minimize { window_id } => {
                wm.minimize_window(*window_id);
            }
            WindowRequest::ToggleMaximize { window_id } => {
                wm.toggle_maximize(*window_id);
            }
            WindowRequest::Close { window_id } => {
                wm.close_window(*window_id);
            }
            WindowRequest::PromoteToFront { window_id } => {
                wm.promote_to_front(*window_id);
            }
            WindowRequest::ToggleWindow { window_id } => {
                wm.toggle_window(*window_id);
            }
            WindowRequest::SetState { window_id, state } => {
                if let Some(win) = wm.find_window_mut(*window_id) {
                    win.state = *state;
                    win.is_open = true;
                }
                wm.promote_to_front(*window_id);
            }
        }
    }
}
