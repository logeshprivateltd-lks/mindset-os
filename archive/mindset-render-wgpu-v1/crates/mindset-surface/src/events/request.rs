//! MindSet OS — WindowRequest Model (`request.rs`)
//! Pattern P3 (forma-winit): Typed declarative window requests queued by input events.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::wm::{WindowId, WindowState};

/// Window edge targeted during resize dragging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    BottomRight,
    Right,
    Bottom,
}

/// Declarative window mutation request queued by event handlers.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowRequest {
    DragStart {
        window_id: WindowId,
        cursor: (f32, f32),
    },
    DragDelta {
        window_id: WindowId,
        dx: f32,
        dy: f32,
    },
    ResizeStart {
        window_id: WindowId,
        edge: ResizeEdge,
    },
    Minimize {
        window_id: WindowId,
    },
    ToggleMaximize {
        window_id: WindowId,
    },
    Close {
        window_id: WindowId,
    },
    PromoteToFront {
        window_id: WindowId,
    },
    ToggleWindow {
        window_id: WindowId,
    },
    SetState {
        window_id: WindowId,
        state: WindowState,
    },
}
