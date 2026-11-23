//! MindSet OS — Overlay Subsystem Manager (`overlay.rs`)
//! Enforces Pattern P2 strict 3-stage modal dispatch: Item -> Panel -> Backdrop.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::layout::snapshot::{LayoutSnapshot, UiId};
use mindset_ui::ActiveOverlay;

/// Action outcome from overlay hit-test dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayDispatchResult {
    /// An interior item was clicked (e.g. tile, button, slider).
    Item(UiId),
    /// Click was inside the modal panel body (event trapped, no bleed-through).
    PanelTrapped,
    /// Click was strictly outside the modal panel (dismiss overlay).
    BackdropDismiss,
    /// No overlay was active or unhandled.
    Ignored,
}

/// Overlay Manager handling modal bounds, item registration, and containment.
pub struct OverlayManager;

impl OverlayManager {
    /// Dispatch a pointer event against an active overlay using Pattern P2 ordering:
    /// 1. Interior items tested FIRST.
    /// 2. Panel containment tested SECOND (traps clicks within modal bounds).
    /// 3. Backdrop dismissal tested THIRD (outside clicks close overlay).
    pub fn dispatch(
        overlay: ActiveOverlay,
        cursor: (f32, f32),
        snapshot: &LayoutSnapshot,
    ) -> OverlayDispatchResult {
        match overlay {
            ActiveOverlay::None => OverlayDispatchResult::Ignored,
            ActiveOverlay::ControlCenter => {
                let panel = snapshot.rect(UiId::ControlCenterPanel);
                // 1. Interior items
                let items = [
                    UiId::ControlCenterCloseBtn,
                    UiId::ControlCenterSettingsBtn,
                    UiId::ControlCenterWifiTile,
                    UiId::ControlCenterBluetoothTile,
                    UiId::ControlCenterDndTile,
                    UiId::ControlCenterAirplaneTile,
                    UiId::ControlCenterNightLightTile,
                    UiId::ControlCenterBrightnessTrack,
                    UiId::ControlCenterVolumeTrack,
                    UiId::ControlCenterUiScaleTrack,
                ];
                for id in items {
                    if snapshot.rect(id).contains(cursor.0, cursor.1) {
                        return OverlayDispatchResult::Item(id);
                    }
                }
                // 2. Panel containment
                if panel.contains(cursor.0, cursor.1) {
                    return OverlayDispatchResult::PanelTrapped;
                }
                // 3. Backdrop dismissal
                OverlayDispatchResult::BackdropDismiss
            }
            ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. } => {
                let panel = snapshot.rect(UiId::LauncherPanel);
                // 1. Interior items
                if snapshot
                    .rect(UiId::LauncherSearchInput)
                    .contains(cursor.0, cursor.1)
                {
                    return OverlayDispatchResult::Item(UiId::LauncherSearchInput);
                }
                if snapshot
                    .rect(UiId::LauncherAiHub)
                    .contains(cursor.0, cursor.1)
                {
                    return OverlayDispatchResult::Item(UiId::LauncherAiHub);
                }
                if snapshot
                    .rect(UiId::LauncherPowerBtn)
                    .contains(cursor.0, cursor.1)
                {
                    return OverlayDispatchResult::Item(UiId::LauncherPowerBtn);
                }
                for i in 0..4 {
                    if snapshot
                        .rect(UiId::LauncherFooterBtn(i))
                        .contains(cursor.0, cursor.1)
                    {
                        return OverlayDispatchResult::Item(UiId::LauncherFooterBtn(i));
                    }
                }
                // 2. Panel containment
                if panel.contains(cursor.0, cursor.1) {
                    return OverlayDispatchResult::PanelTrapped;
                }
                // 3. Backdrop dismissal
                OverlayDispatchResult::BackdropDismiss
            }
            _ => OverlayDispatchResult::Ignored,
        }
    }
}
