//! MindSet OS — Active Overlay & Modal Exclusivity Architecture (`overlay.rs`)
//! Rule 3: Compile-Time Modal Exclusivity.
//! Ensures mutually exclusive overlays: dual-layered or double-draw overlays are unrepresentable.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

/// Mutually exclusive system overlay enum.
/// Only one overlay may be active at any given moment.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ActiveOverlay {
    #[default]
    None,
    Launcher,
    CommandPalette {
        query: String,
        selected_idx: usize,
    },
    ControlCenter,
    FlashSwitcher,
    Notifications,
    ContextMenu,
    OnboardingWizard,
    OdysseyMap,
    TrophyDeck,
}

impl ActiveOverlay {
    /// Returns true if no overlay is currently active.
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns true if any overlay is active.
    #[inline]
    pub fn is_open(&self) -> bool {
        !self.is_none()
    }

    /// Human-readable title or identifier for telemetry / debugging.
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Launcher => "Launcher",
            Self::CommandPalette { .. } => "CommandPalette",
            Self::ControlCenter => "ControlCenter",
            Self::FlashSwitcher => "FlashSwitcher",
            Self::Notifications => "Notifications",
            Self::ContextMenu => "ContextMenu",
            Self::OnboardingWizard => "OnboardingWizard",
            Self::OdysseyMap => "OdysseyMap",
            Self::TrophyDeck => "TrophyDeck",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_overlay_default_and_variants() {
        let overlay = ActiveOverlay::default();
        assert_eq!(overlay, ActiveOverlay::None);
        assert!(overlay.is_none());
        assert!(!overlay.is_open());

        let launcher = ActiveOverlay::Launcher;
        assert!(launcher.is_open());
        assert_eq!(launcher.name(), "Launcher");

        let cmd = ActiveOverlay::CommandPalette {
            query: "physics".to_string(),
            selected_idx: 2,
        };
        assert!(cmd.is_open());
        assert_eq!(cmd.name(), "CommandPalette");

        let cc = ActiveOverlay::ControlCenter;
        assert_ne!(launcher, cc);
    }
}
