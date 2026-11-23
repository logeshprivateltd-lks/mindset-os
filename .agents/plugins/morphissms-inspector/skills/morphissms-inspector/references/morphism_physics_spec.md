# Morphism Visual Physics & Material Specification

This specification defines the physical optics, lighting geometry, color math, and implementation standards for modern UI morphisms: **Glassmorphism**, **Liquidmorphism**, **Claymorphism**, and **Neomorphism**. It targets **pure Rust + Iced 0.14.0**, **WGSL / `wgpu` shaders**, and cross-platform UI engines (CSS/Tailwind/Flutter).

---

## 1. Global Lighting Geometry & Vector Optics

All physical UI materials depend on a coherent, singular virtual illumination environment. When multiple widgets in the same viewport have shadows or specular highlights cast from conflicting angles, the visual system experiences cognitive dissonance and perceives the UI as synthetic, flat, or glitchy.

### 1.1 Virtual Light Vector
A standard UI directional light source is situated at upper-left elevation:
$$\vec{L} = \begin{pmatrix} -\frac{\sqrt{2}}{2} \\ -\frac{\sqrt{2}}{2} \\ 1 \end{pmatrix} \approx \begin{pmatrix} -0.707 \\ -0.707 \\ 1.0 \end{pmatrix}$$

- **Azimuth angle ($\phi$):** $135^\circ$ (top-left coming down towards bottom-right) or $90^\circ$ (top-down vertical ambient).
- **Elevation angle ($\theta$):** $45^\circ$ to $60^\circ$ above the display plane.

### 1.2 Shadow Projection Math
The projected shadow vector $\vec{S}_{offset} = (S_x, S_y)$ on the surface behind an elevated element at elevation $h$:
$$\vec{S}_{offset} = h \cdot \tan(90^\circ - \theta) \cdot \begin{pmatrix} \cos(\phi + 180^\circ) \\ \sin(\phi + 180^\circ) \end{pmatrix}$$

For top-left light ($\phi = 135^\circ$), the cast shadow offset points towards bottom-right:
$$S_x > 0, \quad S_y > 0 \quad (\text{e.g., } \text{Vector::new}(4.0, 8.0))$$

Opposing specular highlight bevels point opposite to the shadow:
$$H_x < 0, \quad H_y < 0 \quad (\text{e.g., } \text{Vector::new}(-3.0, -3.0))$$

---

## 2. Morphism Fidelity Laws

### 2.1 Glassmorphism Fidelity

Glassmorphism emulates semi-transparent frosted glass (borosilicate or silica glass with micro-etched sandblasted rear surfaces).

```
   Directional Light (Top-Left)
              \
               \
   +------------\---------------------+  <- 1px Specular Border (Alpha 0.25..0.35)
   |             v                    |
   |   Frosted Glass Substrate        |  <- Low Fill Alpha (0.08..0.22)
   |   [Multi-pass Kawase Blur]       |  <- Real Blur (16px..28px equivalent)
   |                                  |
   +----------------------------------+  <- Subtle bottom rim (Alpha 0.08..0.12)
            \        /
       Soft Ambient Shadow (blur >= 18px, alpha 0.12..0.22)
```

1. **Translucency Boundary ($0.08 \le \alpha \le 0.25$):**
   - **Flaw: Murky Plastic ($\alpha > 0.35$):** Elements with alpha $\ge 0.50$ look like painted acrylic or plastic slabs, blocking underlying wallpapers and destroying the glass illusion.
   - **Flaw: Ghost Transparency ($\alpha < 0.05$):** The element lacks structural presence; text becomes difficult to read over busy backgrounds.
   - **Rule:** Container fill alpha must remain strictly between `0.08` and `0.25`.

2. **Backdrop Blur Requirement ($12\text{px} \le \sigma_{blur} \le 28\text{px}$):**
   - **Flaw: Flat Acetate Sheet:** Applying alpha transparency *without* backdrop blur is not glassmorphism—it is a tinted cellophane overlay that causes severe moiré and legibility breakdown over high-frequency wallpapers.
   - **Rule:** Must execute dual-pass Kawase blur or separable Gaussian blur in `wgpu` or platform compositor.

3. **Specular Edge Rim Lighting:**
   - **Rule:** A crisp 1.0px border (`rgba(255, 255, 255, 0.18)` to `rgba(255, 255, 255, 0.38)`). In advanced shaders or dual containers, the top and left edges must exhibit higher luminance than the bottom and right edges to simulate directional grazing incidence reflection.

4. **Ambient Separation Drop Shadow:**
   - **Rule:** Soft diffused drop shadow with `blur_radius >= 16.0px` and `offset.y >= 4.0px`. This elevates the glass panel above the wallpaper plane so foreground text remains legible even over high-luminance wallpaper regions.

---

### 2.2 Liquidmorphism Fidelity

Liquidmorphism models viscous, surface-tension-dominated fluid droplets, mercury, and dynamic organic liquid glass.

1. **Organic Geometry:**
   - **Rule:** Sharp $90^\circ$ or boxy corners ($\text{radius} < 18\text{px}$) are forbidden. Elements must use large radii ($\text{radius} \ge 24\text{px}$), continuous squircles, or pill shapes ($\text{radius} \ge 999.0\text{px}$).
   
2. **Refraction & Specular Sheen:**
   - **Rule:** Elements must feature continuous gradient sheens or WGSL Fresnel distortion shaders:
     $$F(\theta) = F_0 + (1 - F_0)(1 - \vec{N} \cdot \vec{V})^5$$
   - Light entering the edge of the liquid droop bends and creates an organic inner caustic or chromatic aberration rim.

3. **Fluid State Interpolation:**
   - **Rule:** State transitions (hover, active, press) must use spring physics or cubic ease-out curves (`ease_out_cubic` or critically damped springs). Instant or linear jumps destroy fluid physical behavior.

---

### 2.3 Claymorphism Fidelity

Claymorphism represents tactile, inflated 3D clay/plasticine sculptures with soft matte surfaces and substantial tactile thickness.

```
                  Light Source (Top-Left)
                         \
      +-------------------\------------------+
     /   Bright Inner Highlight Bevel         \  <- Inner Highlight (Top-Left)
    |    (offset: -2px, -2px; alpha 0.4..0.6)  |
    |                                          |  <- Surface Fill: Soft matte pastel
    |           Puffy 3D Clay Body             |     or subtle gradient
    |                                          |
    |    Dark Inner Depth Shade                |  <- Inner Rim (Bottom-Right)
     \   (offset: 3px, 4px; tinted hue)       /
      +--------------------------------------+
            \                       /
       Soft Tinted Drop Shadow (offset: 0, 8..14; blur: 20..32px;
                                TINTED with surface hue, NEVER #000)
```

1. **Inflated Form & Sculptural Radii:**
   - **Rule:** Generous border radii ($20.0\text{px} \le \text{radius} \le 36.0\text{px}$ or circle).

2. **Mandatory Dual-Layer Depth (Outer + Inner Shading):**
   - **Flaw: Flat Cookie-Cutter (Single Shadow):** A rounded rectangle with just an outer drop shadow looks like paper or cardboard, not clay.
   - **Rule:** True clay requires:
     1. **Outer Diffused Shadow:** Soft, large blur radius ($\ge 18\text{px}$), giving elevation.
     2. **Inner Top-Left Specular Rim:** Crisp bright inner highlight simulating light hitting the convex dome.
     3. **Inner Bottom-Right Tinted Shadow:** Sculpted darker inset tone simulating ambient occlusion under the puff.

3. **Harmonious Shadow Tinting:**
   - **Flaw: Muddy Pure-Black Shadow:** Setting shadow color to `#000000` or `rgba(0,0,0,0.5)` generates a gray, muddy, artificial edge that ruins pastel or colored clay.
   - **Rule:** Shadow color *must* be color-tinted using the base surface hue (e.g., lavender surface `#8F7BFF` must cast a deep indigo/periwinkle shadow `rgba(0.357, 0.322, 0.941, 0.45)`, never pure black).

---

### 2.4 Neomorphism Fidelity

Neomorphism (Soft UI) creates extruded plastic structures where the widget appears to be carved or stamped directly out of the background canvas.

```
      Light Source (-45 deg, Top-Left)
             \
      Light Shadow / Bevel (offset: -5px, -5px; bright #FFFFFF, alpha 0.7..0.9)
             \  +--------------------------------+
              \ |                                |
                |       Continuity Surface       |  <- Delta luminance <= 5% from canvas
                |       (Delta E < 3.0)          |
                |                                | \
                +--------------------------------+  \
                      Dark Shadow (offset: 5px, 5px; tinted dark, alpha 0.3..0.6)
```

1. **Surface Continuity:**
   - **Rule:** Base surface color must closely match the parent canvas tone. Contrast between button/card surface and canvas must be subtle ($\Delta L^* \le 8\%$).

2. **Opposing Dual Shadows:**
   - **Rule:** Must pair:
     1. **Light Bevel:** Offset $(-d_x, -d_y)$ with light color (`rgba(255, 255, 255, 0.8)` in light theme or subtle luminance shift in dark theme).
     2. **Dark Cavity Shadow:** Offset $(+d_x, +d_y)$ with darker shade (`rgba(163, 177, 198, 0.6)` in light theme).
   - **Pressed / Inset State:** When pressed, outer dual shadows collapse to zero, and inner dual shadows appear in inverted polarity:
     $$\vec{S}_{\text{inset\_dark}} = (+d_x, +d_y), \quad \vec{S}_{\text{inset\_light}} = (-d_x, -d_y)$$

3. **Accessibility Guardrail (WCAG AA/AAA):**
   - **Flaw: Neomorphic Invisibility Trap:** Because neomorphic surfaces rely on minimal contrast, text and icons placed inside neomorphic buttons frequently fail WCAG minimum contrast (3:1 for large, 4.5:1 for regular text, 7:1 for AAA).
   - **Rule:** All labels, glyphs, and interactive indicator icons placed upon neomorphic containers *must* achieve $\ge 7:1$ contrast against the surface.

---

## 3. Pure Rust + Iced 0.14.0 Architecture

In Iced 0.14.0, `iced::widget::container::Style` contains:
```rust
pub struct Style {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border: Border,
    pub shadow: Shadow,
}
```

Because `container::Style` exposes a single `Shadow` struct, physical dual-shadow architectures (such as Neomorphism and Claymorphism) are constructed via **Nested Container Stacking** or **Gradient Canvas Shaders**.

### 3.1 Dual-Shadow Stacking Pattern in Iced 0.14.0

```rust
use iced::widget::container;
use iced::{Background, Border, Color, Element, Shadow, Theme, Vector};

/// Production-grade Neomorphic Raised Card in Iced 0.14.0
pub fn neu_raised_card<'a, M: 'a>(
    content: impl Into<Element<'a, M>>,
    bg_surface: Color,
    light_shadow: Color,
    dark_shadow: Color,
    radius: f32,
) -> Element<'a, M> {
    // Outer Container: Casts the negative top-left light highlight
    container(
        // Inner Container: Casts the positive bottom-right dark shadow and carries the surface
        container(content)
            .padding(16)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(bg_surface)),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                    width: 0.5,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: dark_shadow,
                    offset: Vector::new(6.0, 6.0),
                    blur_radius: 14.0,
                },
                ..Default::default()
            })
    )
    .style(move |_theme: &Theme| container::Style {
        background: None,
        shadow: Shadow {
            color: light_shadow,
            offset: Vector::new(-5.0, -5.0),
            blur_radius: 12.0,
        },
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
```

### 3.2 High-Fidelity Claymorphic Card in Iced 0.14.0

```rust
/// Puffy Claymorphic Card with Harmonious Tinted Shadow and Tactile Specular Rim
pub fn clay_card_v2<'a, M: 'a>(
    content: impl Into<Element<'a, M>>,
    base_color: Color,
    radius: f32,
) -> Element<'a, M> {
    // Derive harmonious tinted shadow from base hue (multiply luminance by 0.6)
    let shadow_color = Color::from_rgba(
        base_color.r * 0.45,
        base_color.g * 0.40,
        base_color.b * 0.85,
        0.48,
    );
    
    // Top-left specular rim highlight
    let rim_highlight = Color::from_rgba(1.0, 1.0, 1.0, 0.55);

    container(
        container(content)
            .padding(16)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(base_color)),
                border: Border {
                    color: rim_highlight,
                    width: 2.0,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: shadow_color,
                    offset: Vector::new(0.0, 10.0),
                    blur_radius: 24.0,
                },
                ..Default::default()
            })
    )
    .style(move |_theme: &Theme| container::Style {
        background: None,
        shadow: Shadow {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
            offset: Vector::new(-2.0, -2.0),
            blur_radius: 6.0,
        },
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
```

---

## 4. WGSL / `wgpu` Shader Performance Guardrails

1. **Blur Radius Fill-Rate Limit:**
   - Shaders attempting single-pass Gaussian blur with $\sigma > 16.0$ cause GPU stalls on integrated GPUs (Intel Xe, Apple M-series base).
   - Use **Dual-Pass Kawase Blur** with 2 downsamples and 2 upsamples.

2. **Frame-Rate Caching for Static Glass:**
   - Frosted glass backdrops for static panels (e.g. sidebars, control centers) must be cached in an offscreen `wgpu::TextureView`. Only re-render when the underlying scene geometry changes or during window resize.

3. **Global Light Uniform:**
   - All shaders must bind a single `LightParams` uniform buffer:
     ```wgsl
     struct LightParams {
         direction: vec3<f32>,
         intensity: f32,
         ambient_tint: vec4<f32>,
     };
     ```
   - Never hardcode conflicting light angles across separate shader pipelines.
