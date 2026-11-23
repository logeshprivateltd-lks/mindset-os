---
name: morphissms-inspector
description: Audits Glassmorphism, Liquidmorphism, Claymorphism, and Neomorphism UI implementations across Rust (Iced 0.14.0, wgpu WGSL shaders), CSS/Tailwind, and Flutter. Checks physical light angles, dual-layer shadows, specular edge rims, translucency, and blur depth.
---

# Morphisms Inspector (`MORPHISSMS_INSPECTOR`)

Expert UI lighting, material physics, and shader inspection skill. Analyzes user interfaces implemented in **pure Rust + Iced 0.14.0**, **WGSL / `wgpu` shaders**, **CSS/Tailwind**, or **Flutter** to verify whether **Glassmorphism**, **Liquidmorphism**, **Claymorphism**, and **Neomorphism** adhere to rigorous real-world optics and material physics.

---

## 1. Morphism Physics & Material Rules

Whenever evaluating or refactoring morphism implementations, enforce these strict material laws:

### 1.1 Glassmorphism Fidelity
- **Translucency Boundary ($0.08 \le \alpha \le 0.25$):** Fill alpha must stay strictly between `0.08` and `0.25`. Flag panels with $\alpha > 0.35$ as `WEAK` (opaque plastic/murky slabs) and $\alpha < 0.05$ as `WEAK` (invisible ghost panels).
- **Backdrop Blur Requirement:** Must apply real blur ($12\text{px}$–$28\text{px}$ equivalent or multi-pass Kawase/Gaussian blur in `wgpu`). Flag glass panels that only use alpha transparency without blur as `MISSING` blur.
- **Specular Edge Rim:** Must include a crisp 1.0px light border (`rgba(255, 255, 255, 0.18–0.38)`), ideally brighter on top/left edges to simulate directional grazing light.
- **Ambient Separation:** Needs a soft low-opacity drop shadow (`blur_radius >= 16.0`, `offset.y >= 4.0`) so text stays legible over bright or detailed wallpapers.

### 1.2 Liquidmorphism Fidelity
- **Organic Geometry:** High border radius ($\ge 22.0\text{px}$), smooth squircle, or pill geometry. Flag sharp boxy corners.
- **Refraction & Sheen:** Gradient sheen, edge chromatic/specular highlights, or WGSL Fresnel/distortion logic simulating liquid glass rather than flat translucent rectangles.
- **Fluid Motion:** Spring-based or eased interpolation on hover/focus/active states. Flag abrupt linear or instant state jumps.

### 1.3 Claymorphism Fidelity
- **Inflated Form:** Generous rounded corners ($20.0\text{px} \le \text{radius} \le 36.0\text{px}$) that feel soft and sculptural.
- **Mandatory Dual-Layer Depth:** Must combine a soft diffused outer drop shadow (`blur_radius >= 18.0`) WITH dual inner shading (top-left bright inner highlight + bottom-right darker tinted inner rim). Flag single-shadow clay cards as `WEAK` or `FLAT`.
- **Harmonious Shadow Tinting:** Shadows must be tinted from the surface hue. Flag muddy pure-black (`#000000` / `rgba(0,0,0,...)`) shadows.

### 1.4 Neomorphism Fidelity
- **Surface Continuity:** Base card color must closely match the parent canvas tone ($\Delta L^* \le 8\%$).
- **Opposing Dual Shadows:** Must pair a light top-left shadow offset (e.g. `offset: Vector::new(-4.0, -4.0)` with bright highlight) with a darker bottom-right shadow offset (e.g. `offset: Vector::new(5.0, 5.0)` with dark tinted shadow), and invert them cleanly for pressed/inset states.
- **Accessibility Guardrail:** Flag neomorphic buttons/inputs whose icon or text contrast drops below readable levels ($\ge 7:1$ for WCAG AAA).

### 1.5 Rendering & `wgpu` Performance Guardrails
- **Fill-rate hazard:** Flag blur radius overdraw ($\sigma > 64\text{px}$).
- **Shader caching:** Flag uncached blur passes running every frame on static containers.
- **Global Light Angle Consistency:** Flag sibling widgets illuminated from conflicting angles (e.g., widget A lit from top-left, widget B lit from bottom-right).

---

## 2. MCP Tools Overview

The local stdio MCP server `MORPHISSMS_INSPECTOR` exposes three core tools:

1. `scan_morphism_tokens`:
   - Scans target `.rs`, `.wgsl`, `.css`, `.tsx`, or `.dart` files.
   - Extracts all `Shadow`, `Border`, `blur_radius`, `offset`, `Color::from_rgba`, gradients, and shader uniforms.
   - Returns structured JSON of detected vs missing physical tokens.

2. `audit_lighting_consistency`:
   - Analyzes shadow offsets and highlight vectors across multiple files/widgets.
   - Computes angular azimuth ($\phi$) of cast shadows.
   - Detects conflicting light angles and inconsistent specular bevels.

3. `evaluate_morphism_quality`:
   - Evaluates each detected widget against the 5 material physics laws.
   - Flags `PASS`, `WEAK`, or `MISSING` across shadows, blur, specular rim, geometry, and lighting angle.
   - Returns visual critiques and code upgrade suggestions.

---

## 3. Execution & Reporting Workflow

Whenever `/morphissms-inspector` is triggered:

### Step 1: Scan Tokens & Audit Lighting
Call `scan_morphism_tokens` and `audit_lighting_consistency` across the specified UI files, theme modules, or shader paths.

### Step 2: Produce the Material Audit Matrix
Present a clean markdown table summarizing every analyzed component:

| Widget / Function | Target Morphism | Shadow Quality | Blur / Depth | Specular Detail | Light Angle | Grade |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `glass_card` | Glassmorphism | `PASS` | `WEAK` | `PASS` | $90^\circ$ (Top) | **WEAK** |
| `clay_card` | Claymorphism | `WEAK` | `FLAT` | `PASS` | $90^\circ$ (Top) | **FLAT** |
| `neu_card` | Neomorphism | `PASS` | `PASS` | `PASS` | $135^\circ$ (Top-Left) | **PASS** |

### Step 3: Visual Physics Critique
Explain visually *why* weak widgets look flat, murky, or artificial:
- E.g.: "The clay card uses only a single outer drop shadow with flat background color, omitting the top-left highlight dome and bottom-right dark inner rim. As a result, it resembles flat cardboard rather than an inflated 3D tactile clay button."
- E.g.: "The glass surface has 74% opacity without backdrop blur, which reads as tinted smoky acrylic rather than authentic silica glass."

### Step 4: Drop-in Replacement Code Patches
Provide ready-to-copy code blocks in **Rust Iced 0.14.0** or **WGSL** that upgrade each flagged component to full material fidelity (e.g. using nested dual-container shadow stacking for Iced).
