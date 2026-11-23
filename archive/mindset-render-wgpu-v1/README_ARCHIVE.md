# MindSet OS — Legacy wgpu / SDF Renderer Archive (v1)

## Overview
This archive preserves the complete custom wgpu / SDF rendering stack, compositor surface engine, Taffy layout tree, and shaders created for MindSet OS prior to the Iced 0.14.0 migration.

**Preservation Date:** November 2026  
**Copyright:** (c) 2026 LKS PVT LTD. All rights reserved.

---

## What Was Archived

### 1. Crates
- `crates/mindset-render`:
  - `sdf.rs`: GPU Signed Distance Field (SDF) shape batching (`SdfPipeline`, `SdfInstance`, buttons, squircle badges, progress rings).
  - `text.rs`: GPU font rasterizer (`TextRenderer`), cosmic-text / fontdue shaping, glyph atlas, HiDPI `TextBounds` scissoring.
  - `background.rs`: Dynamic canvas wallpaper shader pipeline (`BackgroundPipeline`, time uniforms, focus depth, night warmth).
  - `context.rs`: Raw wgpu instance, adapter, device, queue, and surface configuration.
  - `celebration.rs`: Confetti and celebration burst particle simulations.
  - `shaders/`:
    - `sdf_panel.wgsl`: High-performance SDF panel, rounded rectangles, progress rings, and specular rim shader.
    - `liquid_glass.wgsl`: Refraction, specular gloss, and chromatic dispersion glass shader.
    - `frosted_glass.wgsl`: Blurred backdrop frosted-glass acrylic shader.
    - `background.wgsl`: Procedural gradient and atmospheric background shader.
    - `text.wgsl`: Subpixel font rendering and scissor-rect clipped text shader.

- `crates/mindset-material`:
  - `blur.rs`: Dual-Kawase multi-pass downsampling and upsampling blur pipeline (`DualKawasePipeline`).
  - `target.rs`: Off-screen texture render targets and ping-pong buffers (`MaterialTargetManager`).
  - `shaders/kawase_blur.wgsl`: Optimized Kawase blur pass shaders.

- `crates/mindset-ui`:
  - `layout.rs`: Taffy-based flex/grid layout solver.
  - `theme.rs`: WCAG AAA frosted-glass design tokens, palette definitions, and contrast formulas.
  - `widgets/`: Full SDF widget suite (`button.rs`, `card.rs`, `controls.rs`, `browser.rs`, `goal_tracker.rs`, `navigation.rs`, `progress_ring.rs`, etc.).
  - `odyssey_map.rs`: Interactive journey map graph models and state.

- `crates/mindset-surface`:
  - `wm.rs`: Window manager (`WindowManager`), floating window records, z-order promotion, edge snap, and tile constraints.
  - `session.rs`: Screen session management, lock/sleep lifecycle, notification center drawer, power menu.
  - `input_engine.rs`: Text input cursor navigation, multibyte UTF-8 backspacing, selection logic.
  - `layout/`: Modular layout manager, `LayoutSnapshot` 2D geometry capture, `WindowChromeManager`, and `OverlayManager`.
  - `events/`: Declarative `WindowRequest` queue and hit-testing pass.

### 2. Application Shell (`bins/mindset-os`)
- `src/frame.rs`: 2,300+ line unified draw loop painting the multi-window desktop, Today view, top capsule bar, widgets, modals, and dock.
- `src/event_handler.rs`: 4,000+ line winit 0.30 event loop translating raw mouse/keyboard events into compositor actions.
- `src/today.rs`: Custom SDF single-column Today neuro-mission screen.
- `src/window_renderers/`: Per-window draw implementations for Browser, StudyReader, Tasks, Maindy AI, Settings, SystemMonitor, and FileVault.
- `src/shaders/`: Window-specific WGSL shaders.

---

## How to Integrate Custom WGSL Shaders into Iced (`iced::widget::shader`)

Iced 0.14.0 natively supports custom wgpu shaders through the `iced::widget::shader` API. This allows embedding our custom WGSL shaders directly inside the Iced widget tree:

### 1. Implement `shader::Program`
```rust
use iced::widget::shader::{self, wgpu, Viewport};
use iced::Rectangle;

pub struct LiquidGlassShader {
    // Custom uniform buffers, bind groups, pipelines
}

impl<Message> shader::Program<Message> for LiquidGlassShader {
    type State = ();
    type Primitive = ();

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced::mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        // Prepare primitive draw data
    }
}
```

### 2. Re-use `archive/mindset-render-wgpu-v1/crates/mindset-render/src/shaders/liquid_glass.wgsl`:
- Load the WGSL source:
  ```rust
  let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("liquid_glass_shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("../../../archive/mindset-render-wgpu-v1/crates/mindset-render/src/shaders/liquid_glass.wgsl").into()),
  });
  ```
- Bind uniforms (resolution, mouse, time, tint) and render in `Primitive::render()`.

### 3. Restoring the Full Custom Renderer:
If returning to the standalone custom renderer after the competition:
1. Re-add `crates/mindset-render`, `crates/mindset-material`, `crates/mindset-ui`, `crates/mindset-surface` to `[workspace.members]` in root `Cargo.toml`.
2. Move files from `archive/mindset-render-wgpu-v1/crates/` back into `crates/`.
3. Re-link `bins/mindset-os` to use `winit = "0.30"` and `mindset-render`.
