# MindSet OS — Next-Gen Spatial UI Architecture & Design Specification

**Target Platform:** Windows 11 Desktop (4–8 GB RAM, Low-Spec Optimized)  
**Tech Stack:** Pure Rust + Iced `0.14.0` (`wgpu`, `tokio`, `image`, `canvas`, `advanced`) + `mindset-motion` (`Spring1D`) + SQLite (WAL)  
**Target Memory Budget:** $< 95\text{ MB}$ GUI RSS (excluding on-demand Ollama models)  
**Idle CPU Target:** $< 1.5\%$ with Mild Dynamic Map active, $< 0.5\%$ in Static/Eco Mode  
**Brand Identity:** **MindSet OS** with **Maindy AI** (keep `"Maindy"` unchanged everywhere)

---

## 1. Visual Philosophy: The Spatial Hybrid System

Mixing four distinct visual languages (**Neumorphism**, **Claymorphism**, **Dynamic Island**, and a **3D Odyssey Map**) works cleanly when every style has a strict spatial role and never competes on the same layer:

```text
+----------------------------------------------------------------------------------+
|                       [ (•) Dynamic Island Notch (Obsidian) ]                    |
+---------------------+------------------------------------+-----------------------+
|  NEUMORPHIC RAIL    |    CUSTOMIZABLE WALLPAPER STAGE    |   NEUMORPHIC RAIL     |
|  (Matte Surface)    |   + Mild Parallax (±10 px spring)  |   (Matte Surface)     |
|                     |                                    |                       |
|  [ Target & Weather]|        [ 🏆 Boss Milestone ]       |  [ ⚡ Daily Boss Bar ]|
|  [ Upcoming Events ]|              /                     |  [ #1 Priority Card ] |
|  [ Study Momentum  ]|      [ 📐 Math Node ]              |  [ Capsule Queue    ] |
|  [ Study Radar     ]|               \                    |  [ Deep Focus Stats ] |
|  [ Scratchpad      ]|        [ 🧪 Chem Node ]            |                       |
|                     |               /                    |      ( 🧠 )           |
|                     |    (( ⚡ Active Physics Node ))    |   CLAYMORPHIC         |
|                     |       3D Odyssey Road (Canvas)     |   MAINDY AI ORB       |
+---------------------+------------------------------------+-----------------------+
|                   [ Neumorphic Dock (macOS Hover Wave + Dots) ]                  |
+----------------------------------------------------------------------------------+
```

### The Golden Contrast Rule
* **Why Neumorphism lives on matte rails & panels:** Flat Neumorphic dual shadows vanish over complex photographs. Side widget rails, cards, and the dock use a **soft matte surface** (with subtle framing against the center canvas) so top-left light bevels and bottom-right sculpted shadows read cleanly with WCAG AAA text contrast.
* **Why the Center Stage stays open:** The customizable wallpaper occupies the full viewport, framed by the matte widgets so the **3D Odyssey Map** floats gracefully over the scenic center space.
* **Why Maindy AI uses Claymorphism:** By reserving puffy gradients, round `28 px` geometry, and colored shadows exclusively for **Maindy AI**, students immediately recognize their AI study companion as a friendly, living entity separate from OS chrome.
* **Why the Dynamic Island is Obsidian:** High-contrast `#0B0E14` anchors the top edge, housing Search, Quick Controls, and live Pomodoro status without cluttering the desktop.

---

## 2. Design System & Token Specification

Centralize all tokens inside `bins/mindset-os/src/theme/` (removing all inline `Color::from_rgba` calls from `shell/desktop.rs`).

### 2.1 Neumorphism Tokens (`theme/neumorphism.rs`)

| Token | Light Matte (`AlpineMatteLight`) | Dark Matte (`ObsidianMatteDark`) | Usage |
| :--- | :--- | :--- | :--- |
| `surface_base` | `#E8EEF4` (`0.91, 0.93, 0.96`) | `#1B2029` (`0.11, 0.13, 0.16`) | Card & dock fills |
| `surface_elevated` | `#F0F4F9` | `#222834` | Hover cards, popovers |
| `surface_inset` | `#DFE6EF` | `#151921` | Scratchpad input, active toggles |
| `shadow_light` | `rgba(255, 255, 255, 0.90)` | `rgba(52, 62, 82, 0.45)` | Top-left raised highlight |
| `shadow_dark` | `rgba(163, 177, 198, 0.65)` | `rgba(8, 10, 14, 0.80)` | Bottom-right depth shadow |
| `text_primary` | `#16202A` | `#EDF2F7` | Headings, active values |
| `text_muted` | `#5A6B7C` | `#94A3B8` | Subtitles, metadata |
| `accent_primary` | `#148FC7` (Alpine Teal-Blue) | `#28B4E8` | Primary buttons, focus rings |
| `status_streak` | `#F28524` (Warm Orange) | `#FB923C` | **Strictly** Streak badges |
| `status_done` | `#29B873` (Emerald) | `#34D399` | **Strictly** Completed / Verified |
| `status_boss` | `#EAB308` (Amber Gold) | `#FACC15` | Boss battles & Milestones |

#### Dual-Shadow Neumorphic Container Technique (Zero Custom Shaders)
Because an Iced `container::Style` supports one `Shadow`, build reusable helpers using a lightweight 2-layer container wrapper:

```rust
// bins/mindset-os/src/theme/neumorphism.rs
use iced::widget::container;
use iced::{Background, Border, Color, Element, Length, Shadow, Theme, Vector};

pub fn neu_card<'a, M: 'a>(
    content: impl Into<Element<'a, M>>,
    palette: &NeuPalette,
    radius: f32,
    opacity: f32,
) -> Element<'a, M> {
    // Inner layer: matte surface + bottom-right dark shadow
    let inner = container(content)
        .padding(14)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(Color { a: 0.92 * opacity, ..palette.surface_base })),
            border: Border {
                color: Color { a: 0.55 * opacity, ..palette.shadow_light },
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow {
                color: Color { a: palette.shadow_dark.a * opacity, ..palette.shadow_dark },
                offset: Vector::new(6.0, 6.0),
                blur_radius: 14.0,
            },
            ..Default::default()
        });

    // Outer layer: top-left light highlight shadow
    container(inner)
        .style(move |_theme: &Theme| container::Style {
            background: None,
            border: Border { radius: radius.into(), ..Default::default() },
            shadow: Shadow {
                color: Color { a: palette.shadow_light.a * opacity, ..palette.shadow_light },
                offset: Vector::new(-4.0, -4.0),
                blur_radius: 10.0,
            },
            ..Default::default()
        })
        .into()
}
```
* **Inset Variant (`neu_inset`):** Used for the Scratchpad editor and depressed toggle states. Uses `palette.surface_inset` with a subtle `1.5 px` darker border and zero outer elevation shadow.

---

### 2.2 Claymorphism Tokens for Maindy AI (`theme/claymorphism.rs`)

| Token | Value | Purpose |
| :--- | :--- | :--- |
| `clay_grad_top` | `#8F7BFF` (Soft Lavender) | Upper sunlit curve |
| `clay_grad_bottom` | `#5B52F0` (Deep Periwinkle) | Lower rounded base |
| `clay_rim_light` | `rgba(255, 255, 255, 0.58)` | `2.0 px` specular rim simulating inner clay highlight |
| `clay_outer_shadow` | `rgba(91, 82, 240, 0.38)` | Colored soft drop shadow (`offset: (0, 12)`, `blur: 24`) |
| `clay_radius` | `28.0 px` (Card) / `999.0 px` (Floating Orb) | Puffy, bubbly silhouette |

* **Living Clay Physics:** Attach a `Spring1D` scale spring (`SpringPreset::SnappyMicro`) to the floating **Maindy AI** orb:
  * Resting scale: $1.0$
  * Hover scale: $1.08$
  * Pressed squish: $0.92$ (snaps back with subtle overshoot)
  * Idle gentle breath when a study hint is available: subtle badge glow (no 60 FPS spin).

---

### 2.3 Dynamic Island Notch (`shell/dynamic_island.rs`)

Anchored top-center (`Alignment::Center`, `top margin: 10 px`), rendered as a deep obsidian pill (`#0B0E14`, `1 px` rim `rgba(255, 255, 255, 0.14)`, soft drop shadow).

#### State Machine & Morph Targets

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslandMode {
    Compact,        // Width: 260 px, Height: 36 px -> Time • Streak • Focus Pulse
    Search,         // Width: 500 px, Height: 52 px (+ results drawer up to 260 px)
    ControlCenter,  // Width: 420 px, Height: 240 px -> Lo-Fi, Theme, Wallpaper & Map Studio
    ActiveFocus,    // Width: 320 px, Height: 40 px -> Live Pomodoro Timer + Pause/Exit
}
```

#### Morphing Springs
Drive `island_width: Spring1D` and `island_height: Spring1D` from `crates/mindset-motion`:
* **`Ctrl+K` or Click Search Icon:** Morphs to `IslandMode::Search` and auto-focuses the search input (`text_input::focus`). Searches capsules, apps, and saved notes live. Unmounted when not in `Search` mode.
* **Click Control Icon (Right side of Notch):** Morphs to `IslandMode::ControlCenter`, revealing:
  1. **Ambient / Lo-Fi Audio Toggle** (`rodio` stream or procedural brown noise) + volume slider.
  2. **Theme Toggle:** `AlpineMatteLight` vs `ObsidianMatteDark`.
  3. **Motion Toggle:** `Mild Dynamic (18 FPS)` vs `Eco Static (0% idle)`.
  4. **Wallpaper Studio:** Built-in presets + **"Choose Image..."** button (async `rfd::AsyncFileDialog` for local `.jpg`/`.png`).
  5. **Odyssey Map Theme Picker:** `AlpineTrail`, `GoldenValley`, `MidnightStarpath`, `MinimalMatte`.
* **Click Outside / Press `Esc`:** Collapses smoothly back to `Compact` (or `ActiveFocus` if a Pomodoro is running). Stops `window::frames()` the instant both springs report `is_at_rest()`.

---

### 2.4 3D Odyssey Map Widget over Customizable Wallpaper (`shell/odyssey_map.rs`)

Built cleanly with Iced `stack![]` + `iced::widget::canvas`—zero external 3D game engine bloat.

#### Layer Stack Composition
1. **Layer 0 — Customizable Wallpaper with Mild Parallax:**
   * Single cached `image::Handle` stored in `MindSetApp` (capped to max `1920x1080` when loading user images so VRAM never exceeds $\approx 8\text{ MB}$).
   * Rendered slightly oversized ($1.04\times$) inside a clipped container; cursor offset $(dx, dy) \in [-1, 1]$ drives two damped `Spring1D` offsets bounded to $\pm 10\text{ px}$ for subtle 3D depth.
2. **Layer 1 — Perspective 3D Odyssey Road (`canvas::Program`):**
   * Represents the student's daily/chapter study path (Duolingo-style winding trail) climbing from the foreground (bottom center) toward the scenic horizon (upper center).
   * **Path Parametrization:** For milestone index $i \in \{0, 1, \dots, N-1\}$, define normalized progress $t_i = \frac{i}{\max(N-1, 1)} \in [0, 1]$ (adjusted by vertical scroll offset $s$).
   * **Winding Horizontal Curve (Normalized):**
     $$u(t) = 0.5 + A \cdot \sin(\omega \cdot t + \phi)$$
     where $A = 0.22$, $\omega = 2.6\pi$, and $\phi = 0.35$.
   * **3D Perspective Depth Scaling:** Let $z(t) = 1.0 - 0.52 \cdot t$ represent depth scale ($z = 1.0$ near bottom foreground, $z = 0.48$ near distant horizon). Screen coordinates inside canvas bounds $(W, H)$ are:
     $$x(t) = W \cdot \left(0.5 + (u(t) - 0.5) \cdot z(t)\right)$$
     $$y(t) = H \cdot \left(0.84 - 0.62 \cdot t^{0.85}\right)$$
     $$r(t) = r_{\text{base}} \cdot z(t)$$
     where $r_{\text{base}} = 28\text{ px}$ is the foreground node radius.
   * **Perspective Road Ribbon:** Sample 60 intermediate points along $t \in [0, 1]$, computing left/right ribbon edges offset by $\pm 16\text{ px} \cdot z(t)$ perpendicular to the curve tangent, filled with a smooth translucent path gradient + dashed center milestone ticks.
3. **Layer 2 — 3D Extruded Milestone Pucks & Overlayed Icons:**
   * Each node is rendered in Painter's Order (back-to-front, decreasing $t$) as a 3D puck:
     1. **Ground Contact Shadow:** Soft dark ellipse at $(x, y + 9 \cdot z)$.
     2. **3D Extrusion Base (Thickness):** Darker shade ellipse at $(x, y + 6 \cdot z)$ representing physical height.
     3. **Top Lit Face:** Bright ellipse at $(x, y - \text{lift})$ where $\text{lift} = 5\text{ px}$ when hovered.
        * **Completed:** Emerald green (`status_done`) + checkmark badge.
        * **Current Active (#1 Priority):** Alpine Teal (`accent_primary`) + animated outer breathing ring.
        * **Boss Milestone:** Gold (`status_boss`) + trophy/crown glyph.
        * **Upcoming / Locked:** Soft frosted slate matte.
     4. **Specular Arc:** Crisp `1.5 px` highlight arc along the top rim for tactile 3D sheen.
     5. **Overlayed Subject Icon:** Crisp centered glyph (`⚡` Physics, `🧪` Chemistry, `📐` Math, `🏆` Boss Battle) scaled by $z(t)$ + clean pill label directly underneath the hovered or active node.
4. **Layer 3 — Interactivity & Floating Launch Popover:**
   * `canvas::Program::update` hit-tests cursor distance $\sqrt{(m_x - x_i)^2 + (m_y - y_i)^2} \le r_i \cdot 1.25$ against projected nodes.
   * Hovering lifts the node smoothly and shows its topic + duration (`"Gauss's Law • 100m"`).
   * Clicking a node opens a compact **Neumorphic Node Popover** anchored above the node with: Topic Title, Subject, Mark Impact (`+4 marks potential`), and a non-wrapping `Launch ->` button (which triggers **Tunnel Vision Mode**).
   * Mouse wheel over the map smoothly scrolls forward/backward along the syllabus trail.

#### Dual-Cache "Mild Dynamic" Engine (Low CPU Guarantee)
Split rendering inside `OdysseyMapCanvas` into **two separate `iced::widget::canvas::Cache` instances**:
1. **`static_road_cache`:** Draws the perspective ribbon road, distant decorations, and completed/locked 3D pucks. Cleared **ONLY** on window resize, map theme change, scroll, hover index change, or capsule completion.
2. **`dynamic_ambient_cache`:** Draws ONLY the active node's gentle breathing pulse ring + 10 slow-drifting translucent ambient fireflies/clouds.
   * Driven by a throttled timer `time::every(Duration::from_millis(55))` (**~18 FPS**) when `mild_motion_enabled == true` and window is focused.
   * Completely paused during Tunnel Vision or when `Eco Static` is selected.

#### Map Theme Presets (`MapTheme` enum)
* **`AlpineTrail`:** Crisp glacier-blue path, snow-white dashed center line, mist drift particles.
* **`GoldenValley`:** Warm sandstone trail, amber glow rings, golden dust motes.
* **`MidnightStarpath`:** Deep indigo-cyan constellation path, starlight twinkle nodes.
* **`MinimalMatte`:** Clean monochrome architectural road for distraction-free minimalism.

---

## 3. Target Workspace File Structure

```text
bins/mindset-os/src/
├── main.rs                    # App coordinator, zero-idle Spring1D frame scheduler, DB boot
├── theme/
│   ├── mod.rs                 # ThemeMode (Light/Dark), unified tokens
│   ├── neumorphism.rs         # NEW: 2-layer dual-shadow neu_card & neu_inset helpers
│   └── claymorphism.rs        # NEW: Puffy gradient + colored shadow Maindy AI helpers
├── shell/
│   ├── mod.rs                 # Spatial layout stack + Tunnel Vision unmounting
│   ├── dynamic_island.rs      # NEW: Morphing Obsidian Notch (Compact/Search/Control/Focus)
│   ├── odyssey_map.rs         # NEW: 3D Perspective Canvas Map + Dual-Cache + Node Popover
│   ├── maindy_orb.rs          # NEW: Floating Claymorphic Maindy AI orb & quick-ask drawer
│   ├── desktop.rs             # Refactored Neumorphic side rails, Boss Bar, bug fixes
│   ├── dock.rs                # Upgraded bottom-anchored Neumorphic Dock + Spring1D hover wave
│   └── top_bar.rs             # Thin shell wrapper hosting Dynamic Island & minimal window chrome
├── system/
│   ├── state.rs               # Debounced scratchpad revision state, weather, map/wallpaper state
│   └── audio.rs               # NEW: Pure-Rust rodio Lo-Fi streamer + <1 MB brown-noise generator
├── storage/
│   └── db.rs                  # Profile score clamp/auto-heal, scratchpad_notes, customization cols
└── views/
    └── (Delete dead wrappers: desktop.rs, dock.rs, today.rs, top_bar.rs, windows.rs)
```

---

## 4. Confirmed Audit Bugs & Persistence Fixes (Mandatory)

All verified flaws from `MINDSET_AUDIT_REPORT.md` must be resolved first:

1. **Profile Target Score Auto-Heal (`storage/db.rs`, `onboarding/mod.rs`, `main.rs`):**
   * In `load_profile()`: if `target_score_value > target_score_max` (e.g., corrupted `10005` on disk against max `600`), clamp to `target_score_max` (or sensible `590` if `> max`) and immediately execute an `UPDATE user_profile` to heal `%LOCALAPPDATA%\MindSetOS\mindset_os.db`.
   * Clamp onboarding input (`onboarding/mod.rs:281`) and `OnboardingSetTargetScore` (`main.rs:608`) to `0..=target_score_max`.
   * Render human-readable `profile.target_label` / `target_exam_display()` (e.g., `Class 12 Boards • Target: 590/600`) instead of raw enum `Class12BoardsTN600`.
2. **Real Debounced Async SQLite Scratchpad (`storage/db.rs`, `system/state.rs`, `main.rs`):**
   * Remove synchronous `std::fs::write` from `set_scratchpad` (`system/state.rs:128–132`).
   * Create SQLite table:
     ```sql
     CREATE TABLE IF NOT EXISTS scratchpad_notes (
         id INTEGER PRIMARY KEY CHECK (id = 1),
         content TEXT NOT NULL DEFAULT '',
         updated_at TEXT NOT NULL
     );
     ```
   * Migrate legacy `scratchpad.txt` into `scratchpad_notes` once at startup if present.
   * On edit: increment `scratchpad_revision: u64`, set status `"Saving..."`, and spawn a `500 ms` debounced `Task::perform` that writes to SQLite off the GUI thread only if the revision still matches, then sets status `"Auto-saved to SQLite"`.
3. **Live Weather Binding (`shell/desktop.rs`):**
   * Stop discarding `_system`. Bind the weather card directly to `system.weather` (defaulting to Madurai, TN / `31°C` instead of hardcoded `"Bratislava"`).
4. **Dynamic Upcoming Events (`shell/desktop.rs`):**
   * Filter `core.tasks` against `chrono::Local::now()`, hiding already-passed events (no more static `"Today 2:00 PM"` when clock is `4:43 PM`). Show a clean `"All scheduled blocks cleared"` state if none remain.
5. **Fix `Launch ->` Button Wrapping (`shell/desktop.rs`):**
   * Constrain capsule title column with `Length::Fill`, set explicit button width/padding and `text("Launch ->").wrapping(text::Wrapping::None)` (or `Launch →` with `Wrapping::None` and fixed min-width `88.0`) so long titles never split the arrow onto a second line.
6. **Left Panel Split + Daily Boss Bar + #1 Priority CTA (`shell/desktop.rs`):**
   * Split `unified_card` into 3 breathing Neumorphic micro-cards (`Upcoming Events`, `Study Momentum`, `Study Radar`) with `12 px` vertical spacing.
   * Add **Daily Boss Bar** (`X / Y Capsules Cleared` + progress bar) at top of `Daily Capsule Queue`, highlight the #1 incomplete capsule with a `"NEXT UP"` pill, and provide a quick **"Start Next Capsule"** button.
7. **Tunnel Vision Mode (`main.rs`, `shell/mod.rs`):**
   * Launching any capsule (from Queue or Odyssey Map) triggers `tunnel_vision_active = true`, driving a `Spring1D` opacity spring from $1.0 \to 0.0$.
   * When opacity $\le 0.01$, completely unmount side rails, dock, and Maindy orb so Iced skips laying them out; Dynamic Island morphs to `IslandMode::ActiveFocus` with live Pomodoro timer and Exit button.
8. **Customization SQLite Columns (`storage/db.rs`):**
   * Safely add columns via idempotent `ALTER TABLE user_profile ADD COLUMN ...` migration (ignoring duplicate column error):
     * `wallpaper_path TEXT DEFAULT 'assets/wallpaper.jpg'`
     * `map_theme TEXT DEFAULT 'AlpineTrail'`
     * `theme_mode TEXT DEFAULT 'AlpineMatteLight'`
     * `mild_motion_enabled INTEGER DEFAULT 1`
9. **Dead Code & Asset Cleanup:**
   * Remove unused forwarding wrappers `bins/mindset-os/src/views/{desktop,dock,today,top_bar,windows}.rs`.
   * Ensure `main.rs` resolves `assets/wallpaper.jpg` reliably relative to executable/workspace root, then delete duplicate `bins/mindset-os/assets/wallpaper.jpg` and unreferenced legacy files in `assets/icons/` and `assets/wallpapers/`.

---

## 5. Animation & Low-RAM Performance Discipline

### Zero-Idle 60 FPS Subscription Scheduler (`main.rs`)
Keep existing `1 Hz` clock tick and `0.25 Hz` sysinfo tick intact. Add two strict conditional subscriptions:

```rust
let mut subs = vec![
    time::every(Duration::from_secs(1)).map(|_| Message::Tick),
    time::every(Duration::from_secs(4)).map(|_| Message::SysinfoTick),
    event::listen().map(Message::SystemEvent),
];

// 1. 60 FPS ONLY while any Spring1D is actively moving (Dock, Island, Maindy, Tunnel Vision, Parallax)
if self.has_active_springs() {
    subs.push(window::frames().map(Message::AnimationFrame));
}

// 2. Mild Dynamic Odyssey Map ambient pulse (~18 FPS) ONLY when enabled, visible, and not in Tunnel Vision
if self.mild_motion_enabled && !self.tunnel_vision_active && self.window_focused {
    subs.push(time::every(Duration::from_millis(55)).map(|_| Message::MapAmbientTick));
}
```

### Resource Budget Verification Table

| Subsystem | Expected RAM / VRAM | Guardrail |
| :--- | :--- | :--- |
| Wallpaper Texture | $4\text{--}8\text{ MB}$ VRAM | Loaded once into `image::Handle`, downscaled if user selects $> 1920\text{ px}$ image |
| 3D Odyssey Map (`canvas`) | $< 2\text{ MB}$ RAM | Dual `canvas::Cache`, 18 FPS ambient cap, zero 3D mesh engine |
| Neumorphic + Claymorphic Styling | $0\text{ MB}$ extra | Pure Iced containers, gradients & shadows (no multi-pass blur shader) |
| Dynamic Island & Dock Springs | $< 0.1\text{ MB}$ RAM | `mindset-motion::Spring1D` array; stops subscribing when `is_at_rest()` |
| Lo-Fi Ambient Audio (`rodio`) | $1\text{--}4\text{ MB}$ RAM | Streamed `.ogg` via `BufReader`, or procedural brown-noise fallback ($< 1\text{ MB}$) |
| SQLite Database | $\le 2\text{ MB}$ cache | Existing WAL mode + `PRAGMA cache_size = -2000`, async debounced writes |

---

## 6. Safe 8-Phase Implementation Roadmap

Execute strictly in this order, running `cargo check --workspace` after every single phase:

1. **Phase 1 — Data Integrity, DB Migrations & Async SQLite Scratchpad:** Auto-heal `target_score_value`, clamp onboarding, add customization columns, migrate scratchpad to debounced SQLite.
2. **Phase 2 — Theme Engines (`neumorphism.rs` & `claymorphism.rs`):** Build Light/Dark matte tokens, 2-layer `neu_card`/`neu_inset`, and Maindy AI clay helpers.
3. **Phase 3 — Neumorphic Side Rails & Bug Fixes (`desktop.rs`):** Wire live weather, filter upcoming events, fix `Launch` wrapping, split left panel into 3 micro-cards, add Daily Boss Bar & #1 Priority CTA.
4. **Phase 4 — Claymorphic Maindy AI (`maindy_orb.rs` & `ai_tutor.rs`):** Build floating puffy Maindy orb with `Spring1D` squish physics and claymorphic quick-tutor drawer (preserving `"Maindy AI"` branding).
5. **Phase 5 — Dynamic Island Notch & Lo-Fi Audio (`dynamic_island.rs`, `audio.rs`):** Implement morphing `IslandMode` notch with `Ctrl+K` Liquid Search, Control Center (Theme, Motion, Wallpaper picker, Map Theme picker), and `rodio` Lo-Fi/brown-noise engine.
6. **Phase 6 — 3D Odyssey Map Over Customizable Wallpaper (`odyssey_map.rs`):** Implement cached parallax wallpaper, perspective road, 3D extruded pucks with overlayed icons, dual-cache 18 FPS ambient drift, and interactive node launch popover.
7. **Phase 7 — Dock Hover Wave & Tunnel Vision (`dock.rs`, `shell/mod.rs`, `main.rs`):** Wire `Spring1D` bottom-anchored dock wave (`1.35x` / `1.15x` / `1.0x`), active app dots, Tunnel Vision fade + unmounting, and zero-idle subscription guard.
8. **Phase 8 — Cleanup & Release Verification:** Remove dead view wrappers and verified orphan assets; run `cargo check --workspace`, `cargo test --workspace`, and `cargo build --release`.