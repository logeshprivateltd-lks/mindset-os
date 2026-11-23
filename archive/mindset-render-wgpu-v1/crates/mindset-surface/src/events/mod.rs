//! MindSet OS — Events Subsystem (`events/mod.rs`)
//! Pattern P3 & P4: Hit testing against LayoutSnapshot and WindowRequest queue execution.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod dispatch;
pub mod hit_test;
pub mod request;

pub use dispatch::apply_window_requests;
pub use hit_test::hit_test;
pub use request::{ResizeEdge, WindowRequest};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::snapshot::{LayoutSnapshot, UiId};
    use crate::wm::{WindowId, WindowManager};
    use mindset_ui::LayoutRect;

    #[test]
    fn test_events_hit_test_deepest_resolution() {
        let mut snapshot = LayoutSnapshot::new(1920.0, 1080.0, 1.0);
        let parent = LayoutRect::new(100.0, 100.0, 200.0, 200.0);
        let child = LayoutRect::new(120.0, 120.0, 50.0, 50.0);

        snapshot.insert(UiId::MissionCard, parent);
        snapshot.insert(UiId::MissionStartBtn, child);

        // Click inside child -> child must win (later sibling in z_order)
        let hit = hit_test((130.0, 130.0), &snapshot);
        assert_eq!(hit, Some(UiId::MissionStartBtn));

        // Click inside parent but outside child -> parent wins
        let hit_parent = hit_test((250.0, 250.0), &snapshot);
        assert_eq!(hit_parent, Some(UiId::MissionCard));

        // Click outside both -> None
        let hit_none = hit_test((50.0, 50.0), &snapshot);
        assert_eq!(hit_none, None);
    }

    #[test]
    fn test_apply_window_requests_queue_execution() {
        let mut wm = WindowManager::new(1280.0, 800.0);
        let requests = vec![
            WindowRequest::ToggleWindow {
                window_id: WindowId::Tasks,
            },
            WindowRequest::ToggleWindow {
                window_id: WindowId::StudyReader,
            },
            WindowRequest::Close {
                window_id: WindowId::StudyReader,
            },
        ];

        apply_window_requests(&requests, &mut wm);

        assert_eq!(wm.active_window_id, Some(WindowId::Tasks));
        let study = wm.find_window(WindowId::StudyReader);
        let Some(study_win) = study else {
            panic!("StudyReader window must exist");
        };
        assert!(!study_win.is_open);
    }
}
