//! MindSet OS — Event Hit-Testing Against LayoutSnapshot (`hit_test.rs`)
//! Pattern P4: Consumes logical coordinates directly; zero `* ui_scale` calculations.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::layout::snapshot::{LayoutSnapshot, UiId};

/// Perform hit-testing against a LayoutSnapshot using logical coordinates.
/// Deepest-node resolution is enforced (later siblings win, matching paint order).
#[inline]
pub fn hit_test(cursor_logical: (f32, f32), snapshot: &LayoutSnapshot) -> Option<UiId> {
    snapshot.hit_test(cursor_logical)
}
