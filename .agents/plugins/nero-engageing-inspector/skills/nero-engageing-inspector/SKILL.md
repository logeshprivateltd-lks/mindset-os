---
name: nero-engageing-inspector
description: Audits UI views, widgets, and themes for student cognitive engagement, focus retention, dopamine micro-rewards, and ergonomic comfort during long study sessions. Use whenever auditing, designing, or refactoring UI for study tools, student OS views, Pomodoro timers, or learning apps.
---

# NERO-ENGAGEING-INSPECTOR: Cognitive UX & Student Engagement Auditor

`NERO-ENGAGEING-INSPECTOR` is an advanced neuro-cognitive UI inspector designed to evaluate whether digital learning interfaces protect student attention, reduce cognitive fatigue, supply dopamine-driven progression cues, and sustain hours of deep, uninterrupted academic flow.

It audits pure Rust + Iced 0.14.0 desktop views, WGSL shader pipelines, web applications (React/TSX/CSS), and mobile/Flutter UIs against a rigorous **100-point 6-pillar neuro-engagement rubric**.

---

## 1. When to Use This Skill

Activate this skill when:
- Auditing existing or new UI views for student focus, cognitive load, and motivation.
- Reviewing study capsules, Pomodoro timers, break arcade games, or exam report screens.
- Checking contrast ergonomics, dark mode palettes, or translucent frosted-glass shaders to prevent eye strain during long nocturnal study sessions.
- Refactoring UI layouts to comply with Hick's Law, Sweller's Cognitive Load Theory, and WCAG AA/AAA standards.

---

## 2. The 6-Pillar Weighted Cognitive Rubric

Every audit computes a weighted 0–100 score across 6 pillars:

| Pillar | Max Pts | Focus Area & Guardrails |
| :--- | :---: | :--- |
| **1. Cognitive Load & Hick's Law** | **20** | $\le 2$ primary CTAs per screen; avoid choice paralysis; container nesting $\le 6$; chunk long text ($< 90$ chars); generous padding & spacing tokens. |
| **2. Attention Anchoring & Visual Hierarchy** | **20** | Active study subject is the undisputed visual focal point; 4-tier font scale (caption, body, subhead, metric); peripheral chrome remains visually subordinate. |
| **3. Dopamine & Micro-Reward Feedback Loops** | **20** | Visible progression indicators (streaks, progress bars, milestone counters, XP tags); micro-transitions on hover/press; immediate gratifying feedback on task completion. |
| **4. Distraction Shielding & Flow Protection** | **15** | Suppression of non-academic notifications; calm soothing palettes; high-saturation danger/alarm reds reserved strictly for destructive or urgent actions. |
| **5. Eye-Strain & Long-Session Ergonomics** | **15** | Minimum 4.5:1 body text contrast (WCAG AA); eliminate harsh `#FFFFFF` on `#000000` glare (prefer softened obsidian `#0A0F1A` / off-white `#E6EDF3`); frosted glass opacity $\ge 74\%$ with scrim borders. |
| **6. Emotional Resonance & Student Agency** | **10** | Constructive, growth-mindset microcopy; unambiguous next-action guidance; clear escape hatches and non-punitive empty/error states. |

For full scientific citations and clinical criteria, consult [neuro_engagement_rubric.md](references/neuro_engagement_rubric.md).

---

## 3. Audit Execution Workflow

When auditing a UI file or directory:

### Step 1: Run the MCP Inspector Tools
Call the local `NERO-ENGAGEING-INSPECTOR` MCP server tools:
1. `scan_ui_engagement`:
   - Pass the file path (e.g. `C:/MINDSETOS/bins/mindset-os/src/views/break_arcade.rs`).
   - Extract metrics on container depth, button counts, font scales, progress cues, and microcopy.
2. `check_contrast_ergonomics`:
   - Pass the theme or UI file (e.g. `C:/MINDSETOS/bins/mindset-os/src/theme/mod.rs`).
   - Audit contrast ratios, pure-glare hazards, and frosted-glass opacity.
3. `generate_engagement_scorecard`:
   - Pass the aggregated scan and contrast results to produce the standardized 0–100 scorecard.

### Step 2: Output the Standardized Audit Report
Format the response using the strict output structure below.

---

## 4. Required Output Format

Always structure the audit report with these 4 sections:

### Section A: Executive Summary & Engagement Scorecard Table

```markdown
# 🧠 Neuro-Engagement Audit: [View or Component Name]

**Overall Score:** `[Score] / 100` — **[PASS / WARN / FAIL]**  
**Verdict:** `[Deep Flow Certified | Cognitive Friction Detected | High Dropout Hazard]`

| Pillar | Score | Status | Key Findings |
| :--- | :---: | :---: | :--- |
| **1. Cognitive Load & Hick's Law** | `X / 20` | `PASS/WARN/FAIL` | Brief summary of CTA counts and chunking |
| **2. Attention Anchoring & Visual Hierarchy** | `X / 20` | `PASS/WARN/FAIL` | Evaluation of focal anchor and typography |
| **3. Dopamine & Micro-Reward Loops** | `X / 20` | `PASS/WARN/FAIL` | Evaluation of streaks, XP, and completion cues |
| **4. Distraction Shielding & Flow Protection** | `X / 15` | `PASS/WARN/FAIL` | Evaluation of visual noise and alert restraint |
| **5. Eye-Strain & Ergonomics** | `X / 15` | `PASS/WARN/FAIL` | Contrast ratios, halation risk, glass alpha |
| **6. Emotional Resonance & Student Agency** | `X / 10` | `PASS/WARN/FAIL` | Microcopy tone and next-action cues |
```

### Section B: Granular Flaws & Findings
List every detected flaw with exact location coordinates:
- **Location:** `[File Path]:[Line Number]`
- **Widget / Element:** `[e.g., button, container, text]`
- **Severity:** `[CRITICAL | WARNING | INFO]`
- **Cognitive Impact:** Explain how this impacts working memory, attention, or eye fatigue.

### Section C: Ready-to-Apply Code Patches
Provide tailored, drop-in replacement code patches for the target framework (e.g. Rust + Iced 0.14.0 `container`, `column!`, `button`, `ThemePalette`, TSX, or Flutter):

```rust
// --- Before: [Issue Description] ---
// [Existing problematic code]

// +++ After: [Remediated Code with Cognitive Rationale] +++
// [Drop-in replacement code]
```

### Section D: Cognitive Architectural Justification
Provide a concise 2–3 sentence scientific justification referencing Cognitive Load Theory, Hick's Law, or retinal ergonomics explaining why the change protects student academic flow.
