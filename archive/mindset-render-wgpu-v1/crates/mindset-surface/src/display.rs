//! Display backend abstraction: Windowed winit backend and Direct Linux DRM/KMS scanout descriptor.

use std::sync::Arc;
use winit::window::Window;

/// Direct Linux DRM/KMS scanout mode descriptor for bare-metal Linux sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrmScanoutDescriptor {
    pub connector_id: u32,
    pub crtc_id: u32,
    pub mode_clock_khz: u32,
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub vrefresh: u16,
    pub is_atomic: bool,
}

impl DrmScanoutDescriptor {
    pub fn standard_1080p() -> Self {
        Self {
            connector_id: 1,
            crtc_id: 1,
            mode_clock_khz: 148500,
            hdisplay: 1920,
            vdisplay: 1080,
            vrefresh: 60,
            is_atomic: true,
        }
    }

    pub fn standard_1440p_120hz() -> Self {
        Self {
            connector_id: 1,
            crtc_id: 1,
            mode_clock_khz: 497000,
            hdisplay: 2560,
            vdisplay: 1440,
            vrefresh: 120,
            is_atomic: true,
        }
    }
}

/// Target display mode for the MindSet OS compositor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayMode {
    Windowed { width: u32, height: u32 },
    BorderlessFullscreen,
    DirectScanoutDrmKms(DrmScanoutDescriptor),
}

/// Common trait for all display outputs (desktop window or hardware scanout).
pub trait DisplayBackend {
    fn dimensions(&self) -> (u32, u32);
    fn scale_factor(&self) -> f64;
    fn is_fullscreen(&self) -> bool;
    fn request_redraw(&self);
}

/// A display adapter wrapping a native `winit::window::Window`.
pub struct WinitDisplayAdapter {
    pub window: Arc<Window>,
}

impl WinitDisplayAdapter {
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}

impl DisplayBackend for WinitDisplayAdapter {
    fn dimensions(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn is_fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_scanout_descriptors() {
        let desc = DrmScanoutDescriptor::standard_1440p_120hz();
        assert_eq!(desc.hdisplay, 2560);
        assert_eq!(desc.vdisplay, 1440);
        assert_eq!(desc.vrefresh, 120);
        assert!(desc.is_atomic);
    }

    #[test]
    fn test_display_mode_variants() {
        let mode = DisplayMode::Windowed {
            width: 1280,
            height: 800,
        };
        assert!(matches!(
            mode,
            DisplayMode::Windowed {
                width: 1280,
                height: 800
            }
        ));

        let borderless = DisplayMode::BorderlessFullscreen;
        assert_eq!(borderless, DisplayMode::BorderlessFullscreen);
    }
}
