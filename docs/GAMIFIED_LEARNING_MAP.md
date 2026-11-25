
# MindSet OS — Gamified Progression Map Architecture & Specification

**Document Version:** 1.0.0  
**Status:** Approved for Implementation (Milestone 2 / Requirement R2)  
**Target Platform:** MindSet OS Desktop Shell (`crates/mindset-core`, `bins/mindset-os`)  
**Design Benchmark:** Duolingo "Snake" Learning Path & Candy Crush Saga Progression Mechanics  
**Academic Benchmark:** Tamil Nadu State Board Class 11-12 HSC (590/600 Standard) & Competitive STEM (JEE/NEET)  

---

## 1. Executive Summary & Design Philosophy

MindSet OS transforms high-stakes secondary and higher-secondary exam preparation from an intimidating, solitary ordeal into a visually captivating, habit-forming academic journey. By synthesizing the world-class behavioral retention mechanics of **Duolingo** (single-track serpentine progression, radial micro-rings, legendary mastery, streak shields, milestone chests) and **Candy Crush Saga** (winding 3D road through thematic biomes, 3-star milestone evaluation, episode gateways, tactile visual "juice", and celebratory audio fanfares), MindSet OS creates an educational progression map that is both intrinsically motivating and academically rigorous.

### The Core Anti-Trivialization Principle
Gamification in high-school education frequently fails because it trivializes serious learning—awarding superficial badges for trivial multiple-choice clicks. MindSet OS adheres to a strict design mandate: **Gamification That Respects Academic Rigor**.
1. **Meaningful Effort:** Progress along the 3D Odyssey road cannot be bought or bypassed; it requires verified 35-to-45-minute focused study capsules.
2. **State Board 590/600 Precision:** In addition to standard 3-star competence, nodes feature a premier **State Board Crown Mastery ($\ge 98\%$)** tier, mirroring the exact marking standards required for top state medical (MBBS) and engineering (Anna University) ranks.
3. **Anti-Burnout Safety Nets:** Daily streak flame counters encourage consistent daily cadence, but are protected by **Streak Shield Vaults** that prevent soul-crushing streak resets when students face school pre-board exams or illness.
4. **Zero-Lag Native Rendering:** Built in Pure Rust with `iced 0.14.0`, `taffy`, and `wgpu`, using a dual-cache canvas pipeline that delivers fluid 60 FPS motion during interaction while consuming strict 0% CPU and negligible RAM at idle.

---

## 2. Comparative Deconstruction: Duolingo vs. Candy Crush Saga

| Architectural Dimension | Duolingo ("The Snake" Learning Path) | Candy Crush Saga (Episode Road) | MindSet OS Synthesized Architecture |
| :--- | :--- | :--- | :--- |
| **Path Topology** | Single-track serpentine S-curve; strictly linear to eliminate choice paralysis. | Non-linear winding road curving through thematic biomes with optical depth. | 3D perspective winding ribbon with mathematical sinusoidal sway and optical depth scaling. |
| **Pacing & Chunking** | Segmented into numbered Units with guidebooks and thematic banners. | Segmented into 15-level "Episodes" separated by physical roadblock gateways. | Units map directly to official syllabus chapters grouped into 5 thematic scientific biomes with chapter divider banners. |
| **Node States & Micro-Progress** | Stepping stones with radial progress donut rings (filling in 3-6 lessons). | Numbered pins displaying ground contact shadows, top faces, and star markers. | 3D extruded cylindrical pucks with ground shadows, specular rims, subject glyphs, and radial completion rings. |
| **Mastery Stratification** | Standard Completed $\rightarrow$ "Legendary" (gold/purple high-stakes trial). | 1-Star (Passing), 2-Stars (Proficient), 3-Stars (Mastery). | 4-Tier Evaluation: 1-Star (60%), 2-Star (80%), 3-Star (90%), plus **State Board Crown ($\ge 98\%$)**. |
| **Retention & Habit Loops** | Daily streak flame counter, consumable Streak Freezes, milestone chest drops. | Daily booster spin, lives refill countdown, timed infinite lives bonuses. | Daily streak flame with tri-color evolution (Blue $\rightarrow$ Amber $\rightarrow$ Cosmic), Streak Shield Vault, and XP multipliers. |
| **Tactile "Juice" & Physics** | Mascot (Duo) vector animations, confetti explosions, triumphant chimes. | Explosive "Sugar Crush", candy detonate cascades, screenshake, voice fanfares. | Procedural confetti bursts (36 particles), damped harmonic spring puck physics (`mindset-motion`), and traveling energy beacons. |
| **Gatekeeper Mechanisms** | Unit Review checkpoints before unlocking subsequent sections. | Toll Gates requiring 3 friend unlocks, quest completion, or gold bars. | **Gatekeeper Boss Battles**: Comprehensive timed exam papers that must be passed ($\ge 60\%$) to unlock the next biome. |
| **Sound Design** | Ascending marimba chimes, clean harmonic confirmation beeps. | Ascending orchestral chords, heavy physical thuds for stars, triumphant voiceovers. | Pure procedural harmonic major triads (C5-E5-G5-C6) generated via `rodio` with zero external WAV assets. |

---

## 3. Pedagogical Mechanics Translation for MindSet OS

### 3.1 Overcoming Choice Paralysis via "The Snake"
Traditional learning management systems overwhelm students with sprawling folder hierarchies and open-ended syllabus trees. Following Duolingo's seminal transition from the "Skill Tree" to "The Snake", MindSet OS places the student on a single, curated, chronological path. The system determines the optimal next step based on spaced repetition schedules, while still permitting review of previous nodes.

### 3.2 Cognitive Ultradian Rhythms & Session Chunking
Each stepping stone on the MindSet OS map represents a self-contained **Study Capsule** paced to human ultradian focus cycles (35 to 45 minutes):
- **Stage 1: Heavy Lift Concept & Derivation (15-20 min):** First-principles derivation and mathematical proof.
- **Stage 2: Book-Back & Numerical Drill (10-15 min):** Quantitative step-by-step problem sets.
- **Stage 3: 1-Mark & Formula Anchor (5 min):** Spaced active recall of crucial definitions and SI units.
- **Stage 4: Proof-of-Study Boss (5-10 min):** Strict rubric-graded quiz validating comprehension.

### 3.3 Node Taxonomy
Within each unit, nodes are visually and functionally differentiated by their `NodeType`:
1. `Theory` / `TheoryDerivation` (Icon: `📖` or `⚡` for Physics, `🧪` for Chemistry, `📐` for Math): Conceptual proofs and theoretical foundations.
2. `NumericalDrill` (Icon: `📐` / `🔢`): Step-by-step quantitative exercises with partial credit evaluation.
3. `ReviewChest` (Icon: `🎁`): Spaced-repetition revision milestone awarding bonus XP, streak shields, and memory anchors.
4. `UnitBoss` / `UnitBossBattle` (Icon: `⚔️`): High-stakes comprehensive chapter test enforcing State Board grading rules.

---

## 4. Syllabus Biome Architecture & Thematic Road Mapping

The progression path traverses five meticulously designed **Syllabus Biomes**, each reflecting the thematic atmosphere of the underlying academic domain:

```
[Start: Day 1]
      │
      ▼
+-------------------------------------------------------------------------+
| BIOME 1: ELECTROMAGNETISM & FIELD DYNAMICS                              |
| Subject: Physics | Color: Electric Cyan (#00E5FF) | Particles: Sparks   |
| Chapters: Electrostatics, Current Electricity, Magnetism, EMI & AC      |
+-------------------------------------------------------------------------+
      │
      ▼  [Gatekeeper 1: Maxwell Boss Battle]
+-------------------------------------------------------------------------+
| BIOME 2: QUANTUM REALM & WAVE OPTICS                                    |
| Subject: Physics | Color: Violet Nebula (#7B1FA2) | Particles: Photons  |
| Chapters: Ray Optics, Wave Optics, Dual Nature, Atomic Physics          |
+-------------------------------------------------------------------------+
      │
      ▼  [Gatekeeper 2: Huygens-Einstein Boss Battle]
+-------------------------------------------------------------------------+
| BIOME 3: CHEMICAL ENERGETICS & REACTION KINETICS                        |
| Subject: Chemistry | Color: Emerald Aurora (#00E676) | Particles: Droplets |
| Chapters: Solid State, Solutions, Electrochemistry, Chemical Kinetics   |
+-------------------------------------------------------------------------+
      │
      ▼  [Gatekeeper 3: Nernst-Arrhenius Boss Battle]
+-------------------------------------------------------------------------+
| BIOME 4: CARBON FORGE & ORGANIC SYNTHESIS                               |
| Subject: Chemistry | Color: Solar Amber (#FFAB00) | Particles: Carbons  |
| Chapters: Haloalkanes, Carbonyls, Carboxylic Acids, Biomolecules        |
+-------------------------------------------------------------------------+
      │
      ▼  [Gatekeeper 4: Organic Mechanism Boss Battle]
+-------------------------------------------------------------------------+
| BIOME 5: CALCULUS MATRIX & ABSTRACT TOPOLOGY                            |
| Subject: Mathematics | Color: Cyber Gold (#FFD600) | Particles: Glyphs  |
| Chapters: Matrices, Complex Numbers, Differential & Integral Calculus   |
+-------------------------------------------------------------------------+
      │
      ▼
[Syllabus Mastery: State Board Centum]
```

### 4.1 Chapter Divider Banners
At each transition between chapters and biomes, the 3D canvas draws a floating frosted-glass **Chapter Divider Banner**:
- **Visual Structure:** A rounded horizontal glass pill ($w = 260\text{ px}, h = 38\text{ px}$) rendered at road position $t$, containing:
  - Subject Eyebrow in uppercase 10px bold text (`"PHYSICS • UNIT 2"`).
  - Chapter Title in 13px semi-bold white text (`"Current Electricity & Kirchoff Laws"`).
  - Ambient glowing underline colored to match the active biome theme.

### 4.2 Syllabus Gateways (Roadblocks)
Directly mirroring Candy Crush episode gates, transitions between biomes feature a physical **Syllabus Gateway**:
- **Condition to Unlock:**
  1. All preceding core nodes in the biome must be in `Completed` status.
  2. Aggregate mark percentage across the biome must be $\ge 75\%$.
  3. The student must pass the biome's `UnitBoss` with at least 1 Star.
- **Visual Representation:** When locked, an imposing frosted barricade with a luminous golden lock glyph (`🔒 GATE LOCKED`) spans the ribbon road. When unlocked, a lock-shattering animation plays, the barricade dissolves into radial particle sparks, and the camera glides forward into the new biome landscape.

---

## 5. Multi-Tier Scoring & State Board Crown Mastery ($\ge 98\%$)

### 5.1 The 3-Star Mathematical Formula
At the conclusion of each study node or quiz, the student's performance is normalized into a percentage score $P$:

$$P = \text{round}\left( \frac{\text{Marks Awarded}}{\text{Maximum Marks}} \times 100 \right)$$

Stars are awarded according to explicit pedagogical thresholds:

| Star Rating | Score Range ($P$) | Nomenclature | Visual Badge | Academic Meaning | XP Multiplier |
| :---: | :---: | :---: | :---: | :--- | :---: |
| **0 Stars** | $P < 60\%$ | Incomplete / Review Needed | Hollow Gray Star Outline | Did not meet minimum passing standard. Node remains in progress; subsequent node stays locked. | $0.20\times$ (Participation) |
| **1 Star** | $60\% \le P < 80\%$ | Passing Bronze | 1 Solid Bronze Star (`#CD7F32`) | Satisfactory conceptual recall. Basic definitions correct; minor algebraic errors. Unlocks next node. | $1.00\times$ (Base XP) |
| **2 Stars** | $80\% \le P < 95\%$ | Proficient Silver | 2 Radiant Silver Stars (`#C0C0C0`) | Strong competence. Clear derivation, correct intermediate steps, solid book-back accuracy. | $1.25\times$ (Proficiency) |
| **3 Stars** | $95\% \le P \le 100\%$ | Distinction Gold | 3 Brilliant Gold Stars (`#FFD700`) | High-achiever mastery. Clean theoretical derivation, flawless numerical calculations. | $1.50\times$ (Mastery) |

---

### 5.2 State Board Crown Mastery ($\ge 98\%$) — The 590/600 Standard
In Tamil Nadu State Board (HSC) Class 12 examinations (total 600 marks), students aiming for top state medical (MBBS) and engineering (Anna University) ranks must achieve **590 to 598 marks out of 600** ($98.33\%$ to $99.67\%$). 

At this elite level, marks are never lost to conceptual ignorance; they are lost to **minor procedural infractions**. To prepare students for this standard, MindSet OS introduces **Crown Mastery**:

#### Strict Procedural Rubric Deductions
When evaluating answer scripts for Crown Mastery, the grading engine enforces official State Board evaluation penalties:
1. **Missing Mandatory SI Units:** Any numerical final value lacking correct SI units (e.g. writing `12.5` instead of `12.5 V` or `12.5 N/m`) incurs an automatic $-0.5$ mark deduction.
2. **Missing Ray Diagram Arrowheads:** In optics derivations (e.g. refraction through a prism, compound microscope), ray lines without directional arrows incur an automatic $-1.0$ mark deduction.
3. **Missing Circuit Loop Directions:** In Kirchoff loop equations, circuit diagrams lacking loop travel sense or current arrows incur a $-0.5$ mark deduction.
4. **Skipped Formula Substitution Steps:** Moving directly from algebraic formula to final answer without showing the intermediate numerical substitution step incurs a $-0.5$ mark deduction.
5. **Incomplete Law Statements:** Quoting physical laws without critical boundary conditions (e.g. stating Ohm's law without *"at constant temperature"*) incurs a $-1.0$ mark deduction.

#### Crown Award & Visual Styling
- **Qualification:** The student must score $P \ge 98.0\%$ with **zero procedural deductions**.
- **3D Canvas Rendering:**
  - A three-dimensional burnished golden tiara (`👑`) floats directly above the 3 gold stars.
  - The top surface of the milestone puck is bounded by a specular golden halo (`#FFE082` core with an animated `#FFFFFF` specular rim stroke).
  - Orbiting golden particle motes revolve around the crowned puck continuously.
- **Prestige Economics:**
  - Awards $2.00\times$ base XP.
  - Unlocks the "Centum Club" verified badge in the student's academic profile.
  - Adds $+1$ to the student's State Board 590 Index.

---

## 6. Streak System, Shields, and Anti-Burnout Mechanics

### 6.1 Daily Habit Loop & Flame Evolution
Daily consistency is the single highest predictor of academic success. Completing at least one 35-minute study capsule in a calendar day increments the student's streak counter:

$$\text{Streak}_{d} = \text{Streak}_{d-1} + 1$$

To visually signal mastery duration, the flame counter in the top notch island and the map header evolves through three distinct physical phases:

```
+-------------------------------------------------------------------------+
| PHASE 1: SPARK FLAME (Days 1 to 6)                                      |
| Color: Sky Blue (#42A5F5) | Glow: Soft 1.0 Hz breathing pulse           |
| Represents: Initial study momentum ignition                             |
+-------------------------------------------------------------------------+
                                    │
                                    ▼
+-------------------------------------------------------------------------+
| PHASE 2: SOLAR BLAZE (Days 7 to 29)                                     |
| Color: Solar Amber (#FF9800) | Glow: High-intensity flame with embers   |
| Represents: Established cognitive study habit                           |
+-------------------------------------------------------------------------+
                                    │
                                    ▼
+-------------------------------------------------------------------------+
| PHASE 3: COSMIC SUPERNOVA (Days 30+)                                    |
| Color: Violet Nebula (#E040FB) | Glow: Radiant plasma with cosmic sparks |
| Represents: Unstoppable academic discipline                             |
+-------------------------------------------------------------------------+
```

### 6.2 The Streak Shield Freeze Vault
A major flaw in naive gamification systems is the harsh reset of a long streak to zero upon a single missed day, triggering intense student demoralization and churn. MindSet OS implements the **Streak Shield Vault**:

1. **Earning Shields:** Students earn **1 Streak Shield** for every 7 days of unbroken streak completion.
2. **Storage Cap:** A student can hold a maximum of **2 Streak Shields** simultaneously (preventing hoarding).
3. **Automatic Shield Consumption:** If a calendar day elapses with zero study sessions logged:
   - If `streak_shields > 0`: One shield is automatically consumed (`streak_shields -= 1`). The streak counter is 100% preserved. An encouraging notification is displayed: *"Your Streak Shield protected your 28-day flame! Take a breath and resume today."*
   - If `streak_shields == 0`: The streak enters a **24-Hour Grace Period**. The student is offered a "Streak Repair Challenge" (a quick 15-minute recall quiz). If completed within the grace period, the streak is restored. Only if the grace period expires without activity does the streak counter reset to zero.

### 6.3 Streak XP Multipliers
To reward unbroken study consistency, base XP awards are multiplied by a streak-based scaling factor:

$$M_{\text{streak}} = \min\left(1.0 + 0.05 \times (\text{Streak} - 1), 1.50\right)$$

- Day 1: $1.00\times$
- Day 5: $1.20\times$
- Day 10: $1.45\times$
- Day 11+: $1.50\times$ (Maximum cap)

### 6.4 Milestone Chest Rewards
Milestone chests placed along the path reward specific streak and chapter milestones:
- **7-Day Bronze Chest:** $+250\text{ XP}$ + 1 Streak Shield.
- **14-Day Silver Chest:** $+500\text{ XP}$ + "Focus Titan" Trophy Card unlock.
- **30-Day Gold Chest:** $+1,000\text{ XP}$ + "Cosmic Nebula" UI Theme unlock.
- **100-Day Centum Chest:** $+5,000\text{ XP}$ + "State Board Centurion" Golden Trophy.

---

## 7. Audio-Visual "Juice", Micro-Rewards & Damped Spring Physics

### 7.1 Procedural Celebratory Particle Burst (Confetti)
Upon achieving a node completion or star milestone, a pure-mathematical particle burst erupts from the center of the 3D puck:
- **Particle Count:** $N = 36$ particles.
- **Velocity Model:**
  $$v_{x, i} = v_r \cos\left(\frac{2\pi i}{N} + \delta_i\right)$$
  $$v_{y, i} = v_r \sin\left(\frac{2\pi i}{N} + \delta_i\right) - v_{\text{up}}$$
  where $v_r \in [2.5, 6.5]\text{ px/frame}$, $v_{\text{up}} \in [4.0, 9.0]\text{ px/frame}$.
- **Decay & Gravity:** Gravity $g = 0.22\text{ px/frame}^2$, horizontal drag coefficient $C_d = 0.96$.
- **Lifespan:** 60 frames (~1.0 second at 60 FPS).
- **Color Cycling:** Procedural shuffle of Gold (`#FFD700`), Emerald (`#00E676`), Cyan (`#00E5FF`), and Magenta (`#E040FB`).

### 7.2 Procedural Harmonic Audio Synthesis (`rodio`)
To maintain strict low-RAM execution and avoid bulky WAV/MP3 asset bloat, celebratory audio cues are generated in pure Rust via procedural oscillators streamed to `rodio`:
- **Star 1 Chime:** Pure sine wave at C5 ($523.25\text{ Hz}$), duration 120 ms, soft exponential decay.
- **Star 2 Chime:** Ascending interval: C5 ($523.25\text{ Hz}$) $\rightarrow$ E5 ($659.25\text{ Hz}$), duration 140 ms each.
- **Star 3 Chime:** Major Triad arpeggio: C5 ($523.25\text{ Hz}$) $\rightarrow$ E5 ($659.25\text{ Hz}$) $\rightarrow$ G5 ($783.99\text{ Hz}$), duration 160 ms each.
- **Crown Mastery Fanfare:** Four-note triumphant fanfare: C5 $\rightarrow$ E5 $\rightarrow$ G5 $\rightarrow$ C6 ($1046.50\text{ Hz}$), accompanied by a warm 2nd harmonic overtone ($2093\text{ Hz}$) decaying over 650 ms.
- **Tactile Node Click:** Short 80 Hz mechanical click (25 ms) providing tactile confirmation on node press.

### 7.3 Damped Harmonic Spring Interaction (`mindset-motion`)
Interactions with 3D pucks on the canvas are governed by physical spring dynamics:

$$m \frac{d^2 x}{dt^2} + c \frac{dx}{dt} + k (x - x_{\text{target}}) = 0$$

- **Hover Lift:** When the cursor hovers over a puck, it lifts upward by $6.0\text{ px} \times z$ with slight spring overshoot ($\zeta = 0.72$, natural frequency $\omega_n = 18\text{ rad/s}$).
- **Press Depression:** On mouse button down, the puck depresses downward by $4.0\text{ px}$, simulating physical mechanical resistance.
- **Release Bounce:** On click release, the puck rebounds to its equilibrium elevation with smooth, pleasing damping.

### 7.4 Traveling Energy Beacon
Along the completed portion of the 3D ribbon road, a glowing photon beacon travels continuously:
- **Phase Equation:** $t_{\text{beacon}} = (0.02 \times t_{\text{ambient}}) \pmod{t_{\text{completed\_max}}}$
- **Rendering:** Renders an intense $4.5\text{ px} \times z$ photon core surrounded by an expanding $8.5\text{ px} \times z$ radial glow ring, illustrating the continuous flow of knowledge along the completed syllabus path.

---

## 8. Rust Domain Data Models & Canvas Engine Integration

### 8.1 Domain Models (`crates/mindset-core/src/odyssey.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Completed,
    InProgress,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Theory,
    NumericalDrill,
    ReviewChest,
    UnitBoss,
}

impl NodeType {
    pub fn display_badge(&self) -> &'static str {
        match self {
            Self::Theory => "Theory",
            Self::NumericalDrill => "Numerical Drill",
            Self::ReviewChest => "Review Chest",
            Self::UnitBoss => "Unit Boss",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Theory => "📖",
            Self::NumericalDrill => "📐",
            Self::ReviewChest => "🎁",
            Self::UnitBoss => "⚔️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyNode {
    pub id: String,
    pub unit_id: String,
    pub unit_title: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub marks: u32,
    pub max_marks: u32,
    pub xp: u32,
    pub stars_earned: u8,        // 0..=3
    pub is_crown_mastered: bool, // true when marks/max_marks >= 0.98
    pub estimated_minutes: u32,
}
```

### 8.2 Canvas Rendering Signatures (`bins/mindset-os/src/shell/odyssey_map.rs`)
The 3D canvas renderer executes two distinct geometry passes:

1. **Static Geometry Pass (`static_road_cache`):**
   - Projects 3D ribbon road points using `project(t, w, h, tilt_x, tilt_y, sway)`.
   - Renders 3D extruded milestone pucks in painter's order (decreasing $t$):
     * Ground contact shadow ellipse with elevation blur.
     * Cylindrical side thickness extrusion.
     * Top face with status fill and 1px specular rim stroke.
     * Subject icon glyph centered on the puck face.
     * **Miniature Star Badges:** 3 gold star glyphs (`★`) rendered in an arc above completed pucks.
     * **Golden Crown Tiara:** Floating `👑` glyph and burnished gold rim rendered for crowned nodes.
     * **Chapter Divider Banners:** Frosted glass banner pills rendered across unit boundaries.

2. **Dynamic Geometry Pass (`dynamic_ambient_cache`):**
   - Animated gradient progress trail connecting completed milestones.
   - Active node breathing pulse ring ($1.0\text{ Hz}$).
   - Traveling energy beacon dot and halo ring.
   - Procedural celebratory particle bursts.
   - Floating frosted-glass inspection tooltips on mouse hover.

---

## 9. Accessibility, WCAG AAA Compliance & Low-RAM Guardrails

### 9.1 WCAG AAA Contrast Discipline
- All text rendered on frosted glass banners and canvas pucks maintains a minimum contrast ratio of **$\ge 7.0:1$** against both the background wallpaper and glass containers:
  * Primary text: `#FFFFFF` on frosted dark space (`#131722` at 76% opacity) yields a contrast ratio of **$11.8:1$**.
  * Star Glyphs: Radiant Gold (`#FFD700`) stroked with dark outline (`#1A1400`) guarantees high visibility against all wallpaper biomes.
- **Screen Reader Navigation (`accesskit`):** Every milestone puck exposes an AccessKit node with accessible role `Button`, name `"{title} - {unit_title}"`, value `"{stars_earned} of 3 Stars, Status: {status}"`.

### 9.2 Strict Low-RAM Profile ($\le 2\text{ MB}$ VRAM)
- MindSet OS runs on hardware with strict resource budgets (4 GB system RAM target).
- The progression map canvas uses zero pre-baked bitmap textures or PNG sprite atlases. All ribbons, pucks, stars, crowns, and banners are rendered procedurally via vector math in `iced::widget::canvas::Geometry`.
- Total GPU VRAM footprint for the progression map is **$\le 256\text{ KB}$** (vector buffers only).
- The dual-cache architecture ensures that when the user is reading or idle, CPU utilization is strictly **0.0%**.

---

## 10. Verification Matrix & Edge Cases

| # | Invariant / Feature | Verification Method | Pass Criteria | Invalidation Condition |
|---|---|---|---|---|
| 1 | 3-Star Score Thresholds | Unit test in `crates/mindset-core/src/odyssey.rs` | 59% $\rightarrow$ 0★, 60% $\rightarrow$ 1★, 80% $\rightarrow$ 2★, 95% $\rightarrow$ 3★ | Any star boundary mismatch |
| 2 | Crown Mastery Rigor ($\ge 98\%$) | Evaluator test with scoring | $98\%$ $\rightarrow$ Crowned (`is_crown_mastered: true`); $< 98\%$ $\rightarrow$ No crown | Crown awarded below 98% |
| 3 | Division by Zero Protection | Evaluate node with `max_marks == 0` | Returns `0%` mastery, 0 stars, zero panics | Rust panic on division by zero |
| 4 | Streak Shield Vault | Advance calendar clock by 48h with 1 shield | Shield count decrements by 1, streak counter preserved | Streak counter resets while shield > 0 |
| 5 | Dual-Cache Rendering | Inspect frame render in debug mode | Canvas static cache does not re-render during ambient particle ticks | Re-rasterizing static ribbon every tick |
| 6 | Cognitive Ergonomics | Execute `NERO-ENGAGEING-INSPECTOR` | Total Score = 100/100 across all 6 pillars | Score < 90/100 or readability deduction |
| 7 | Pure Rust Compilation | `cargo test -p mindset-core --lib odyssey` | 100% tests pass with zero warnings | Compiler warnings or test failures |
