//! MINDSET ENGINE — Proprietary Native UI Framework
//! 100% Pure Rust + wgpu 24, Zero HTML, Zero CSS, Zero WebViews.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod accessibility;
pub mod core;
pub mod input;
pub mod layout;
pub mod odyssey_map;
pub mod overlay;
pub mod scroll;
pub mod state;
pub mod theme;
pub mod tree;
pub mod widgets;

// Core re-exports
pub use accessibility::{
    A11yRole, AccessibilityNode, AccessibilityTree, FocusRing, ModalFocusTrap,
};
pub use core::{Component, ComponentNode, DirtyFlags, ElementId, WidgetState};
pub use input::{
    ClipboardEngine, EditableTextBuffer, EventPhase, InputState, ModifiersState, PointerState,
};
pub use layout::{
    AlignItems, BoundedTextRules, FlexContainer, FlexDirection, GridContainer, JustifyContent,
    LayoutCache, LayoutRect, SizeConstraint, CARD_RADIUS, HEADER_HEIGHT, INNER_GAP, OUTER_PAD,
    SIDEBAR_WIDTH,
};
pub use odyssey_map::{
    CampfireRecovery, MasteryCard, MissionStage, NodeStatus, OdysseyMapState, OdysseyNode,
    StageStatus, ThreeStageMission,
};
pub use overlay::ActiveOverlay;
pub use scroll::{ScrollContainer, VirtualList};
pub use state::StateHandle;
pub use theme::{
    elevation::{
        CornerRadii, DualKawaseParams, ElevationLevel, ElevationTokens, LiquidGlassParams,
        SpecularHighlight,
    },
    primitives,
    typography::{TypographyScale, TypographyScaleLevel, TypographyToken},
    wcag::{WcagCompliance, WcagLevel},
    AccentColor, GlassTokens, Rgba, SemanticTokens, Theme, ThemeMode, ThemePalette,
};
pub use tree::{UiBuilder, UiTree};
pub use widgets::*;
