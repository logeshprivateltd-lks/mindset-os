//! # mindset-surface
//!
//! Multi-window compositor, desktop wallpaper shortcut grid, text input engine,
//! session lifecycle, and direct display abstraction for MindSet OS.

pub mod clipboard;
pub mod desktop;
pub mod display;
pub mod events;
pub mod input_engine;
pub mod layout;
pub mod mindful_mode;
pub mod session;
pub mod wm;
pub mod workspace;

pub use clipboard::SystemClipboard;
pub use desktop::{
    BackgroundStyle, ContextMenuAction, DesktopContextMenu, DesktopGrid, DesktopIcon,
    GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_GAP,
};
pub use display::{DisplayBackend, DisplayMode, DrmScanoutDescriptor, WinitDisplayAdapter};
pub use events::{apply_window_requests, hit_test, ResizeEdge, WindowRequest};
pub use input_engine::{CommandLauncher, TextInputBuffer};
pub use layout::{
    normalized_slider_value, slider_thumb_center_x, ControlCenterLayout, LayoutManager,
    LayoutSnapshot, LayoutViolation, OverlayDispatchResult, OverlayManager, UiId,
    WindowChromeHitTargets, WindowChromeManager,
};
pub use mindful_mode::{MindfulHomeConfig, MindfulModeEngine, OsFocusMode};
pub use session::{
    NotificationCategory, NotificationCenter, NotificationItem, PowerAction, SessionManager,
    SessionState, DOCK_HEIGHT, DOCK_WIDTH, TOP_BAR_HEIGHT,
};
pub use wm::{
    SurfaceWindow, WindowId, WindowManager, WindowState, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH,
    TITLEBAR_HEIGHT,
};
pub use workspace::{AltTabSwitcher, WorkspaceManager, NUM_WORKSPACES, WORKSPACE_LABELS};
