# MINDSET OS — SPECIFICATION 02: NEUROAESTHETIC UI DESIGN SYSTEM & DESKTOP SHELL
**Company:** LKS PVT LTD  
**Document ID:** `specs/02_NEUROAESTHETIC_UI.md`  
**Version:** `1.1.0` (Production Master Specification)  
**Design Philosophy:** Neuroaesthetic, Emotionally Engaging, Cognitive Caloric Conservation, Ethical Dopamine, Zero-Bloat GPU Rendering  
**Core Principle:** "Make progress feel rewarding. Make focus feel natural. Make relaxation feel welcome."

---

## 1. Five Context-Aware Theme Token Sets (`packages/ui-core/src/styles/themes.css`)

All components consume semantic CSS variables rather than hardcoded hex values so theme switching occurs instantaneously in $< 16\text{ ms}$ without layout shifts.

### 1.1 Default Identity: `aurora-focus` (Study, Deep Focus & Default OS Shell)
- `--bg-base`: `#07090D` (Obsidian Deep)
- `--surface-primary`: `#111827` (Layer 1 Panel)
- `--surface-secondary`: `#182235` (Layer 2 Inset)
- `--surface-elevated`: `#202B40` (Layer 3 Dropdown / Popover)
- `--accent-primary`: `#8B9CFF` (Electric Indigo — Primary focus & CTA)
- `--accent-secondary`: `#65E6C4` (Teal Vitality — Real-time active states)
- `--accent-achievement`: `#FFD078` (Solar Gold — Streaks & milestones)
- `--accent-creative`: `#FF8EAA` (Rose Quartz — Ideas, notes, hypotheses)
- `--accent-danger`: `#FF6B6B` (Crimson — Security alerts & cancellations)
- `--text-dominant`: `#F9FAFB` (High contrast primary text)
- `--text-muted`: `#9CA3AF` (Secondary labels & metadata)
- `--border-subtle`: `#1F2937`
- `--border-glow`: `rgba(139, 156, 255, 0.28)`
- `--glass-bg`: `rgba(17, 24, 39, 0.72)`
- `--glass-border`: `rgba(139, 156, 255, 0.18)`

### 1.2 Theme A: `solar-bloom` (Daytime Study, Reading & Morning General Use)
- `--bg-base`: `#FAF7EE` (Warm Solar Ivory)
- `--surface-primary`: `#FFFFFF` (Pure Card)
- `--surface-secondary`: `#F3EEDD` (Secondary Container)
- `--surface-elevated`: `#E8DFCA` (Elevated Shelf)
- `--accent-primary`: `#E06D14` (Deep Solar Tangerine)
- `--accent-secondary`: `#3B9B78` (Sage Calm)
- `--accent-achievement`: `#D48806` (Golden Harvest)
- `--accent-creative`: `#E05273` (Warm Coral)
- `--accent-danger`: `#D9383A` (Vermillion)
- `--text-dominant`: `#1C1917` (Deep Warm Charcoal)
- `--text-muted`: `#6B6358` (Treated Umber)
- `--border-subtle`: `#E7DEC8`
- `--border-glow`: `rgba(224, 109, 20, 0.22)`
- `--glass-bg`: `rgba(255, 255, 255, 0.78)`
- `--glass-border`: `rgba(224, 109, 20, 0.16)`

### 1.3 Theme B: `forest-zen` (Cortisol-Reducing Calm, Reflection & Balance AI)
- `--bg-base`: `#09120E` (Deep Canopy)
- `--surface-primary`: `#11201A` (Forest Card)
- `--surface-secondary`: `#172C24` (Evergreen Well)
- `--surface-elevated`: `#1F3A30` (Moss Inset)
- `--accent-primary`: `#52C794` (Emerald Moss)
- `--accent-secondary`: `#82D9B3` (Mint Mist)
- `--accent-achievement`: `#E5BE65` (Bamboo Gold)
- `--accent-creative`: `#69B4D9` (Glacial Stream)
- `--accent-danger`: `#E66868` (Red Bark)
- `--text-dominant`: `#F2F7F4` (Clean Birch)
- `--text-muted`: `#8BA398` (Lichen Grey)
- `--border-subtle`: `#1F382E`
- `--border-glow`: `rgba(82, 199, 148, 0.25)`
- `--glass-bg`: `rgba(17, 32, 26, 0.75)`
- `--glass-border`: `rgba(82, 199, 148, 0.18)`

### 1.4 Theme C: `cosmic-arcade` (Gamified Learning, Milestones & Quizzes)
- `--bg-base`: `#080514` (Cyber Abyss)
- `--surface-primary`: `#120E29` (Arcade Slate)
- `--surface-secondary`: `#1C163D` (Cabinet Panel)
- `--surface-elevated`: `#271F52` (CRT Bezel)
- `--accent-primary`: `#D847FF` (Neon Magenta)
- `--accent-secondary`: `#00F0FF` (Laser Cyan)
- `--accent-achievement`: `#FFE600` (High Score Gold)
- `--accent-creative`: `#FF3366` (Synthwave Sunset)
- `--accent-danger`: `#FF2A6D` (Overdrive Crimson)
- `--text-dominant`: `#FDFBFF` (Starlight White)
- `--text-muted`: `#A198C2` (Phosphor Violet)
- `--border-subtle`: `#2D235E`
- `--border-glow`: `rgba(216, 71, 255, 0.35)`
- `--glass-bg`: `rgba(18, 14, 41, 0.78)`
- `--glass-border`: `rgba(216, 71, 255, 0.25)`

### 1.5 Theme D: `paper-studio` (Tactile Editorial, E-Ink & Deep Reading)
- `--bg-base`: `#F5F2EB` (Linen Parchment)
- `--surface-primary`: `#FAF8F5` (Editorial Sheet)
- `--surface-secondary`: `#EAE5DB` (Drafting Well)
- `--surface-elevated`: `#DDD7CB` (Binder Spine)
- `--accent-primary`: `#2B303A` (Graphite Ink)
- `--accent-secondary`: `#5C6479` (Slate Stamp)
- `--accent-achievement`: `#9C6644` (Leather Bookbinding Bronze)
- `--accent-creative`: `#A85A48` (Wax Seal Terracotta)
- `--accent-danger`: `#B83A3A` (Red Editing Pencil)
- `--text-dominant`: `#191B1F` (Carbon Black)
- `--text-muted`: `#636773` (Pencil Gray)
- `--border-subtle`: `#D5CEBF`
- `--border-glow`: `rgba(43, 48, 58, 0.18)`
- `--glass-bg`: `rgba(250, 248, 245, 0.85)`
- `--glass-border`: `rgba(43, 48, 58, 0.15)`

---

## 2. Controlled Liquid Glass & Low-RAM GPU Rules

### 2.1 Permitted Glass Elements
Liquid Glass (`backdrop-filter: blur(12px - 24px)`) is computationally intensive on integrated GPUs (Intel UHD / AMD Radeon 600M). Therefore, it is strictly permitted **only** on:
1. **TitleBar & Window Navigation Header** (`height: 38px`)
2. **Floating Bottom Dock & Navigation Bar**
3. **Modal Dialogs & Backdrops** (`/status`, UAC Confirmation, Settings)
4. **Command Palette (`Ctrl + K`)**
5. **Smart Lock Screen & Privacy Blur Overlay**
6. **Tier 1 Priority Card & Active Timer Countdown Pill**

### 2.2 Prohibited Glass Elements
Under no circumstances may Liquid Glass be applied to:
- Standard task lists, habit rows, or syllabus tree nodes.
- Full viewport background panes when modals are not open.
- In-memory data tables or diagnostic log consoles.
- High-frequency live streaming video canvases.
- Layered opaque surfaces (`#111827`, `#182235`, `#202B40`) MUST be used for standard cards.

---

## 3. The 3-Tier Visual Attention Hierarchy & Ethical Dopamine

### 3.1 3-Tier Hierarchy
- **Priority 1 (Dominant Focus — ~50% Attention):**
  - Current active task, next scheduled commitment, active Focus session countdown pill.
  - Border luminescence: `1px solid var(--accent-primary)` with `box-shadow: 0 0 20px var(--border-glow)`.
  - Single dominant primary action button per screen.
  - Subtle Liquid Glass backdrop blur (`backdrop-blur-md`).
- **Priority 2 (Core Context — ~35% Attention):**
  - Daily progress ring, study statistics, upcoming syllabus milestones.
  - Opaque layered surface (`--surface-primary` or `--surface-secondary`).
  - Zero GPU blur waste.
- **Priority 3 (Ambient Cadence — ~15% Attention):**
  - Historical reports, sync log, diagnostic readouts, build info.
  - Compact row height, muted text (`--text-muted`).

### 3.2 Anti-Addiction & Ethical Dopamine Rules
- **Non-Punitive Habit Streaks:** Missing a study day due to rest, illness, or scheduled rest freezes streaks rather than resetting them to zero.
- **Luminescent Aura Feedback:** Task completion triggers a $200\text{ms}$ subtle amber halo (`#FFD078`) radiating from the checkbox, accompanied by an optional "+50 XP" badge.
- **Zero Slot-Machine Mechanics:** No flashing lights, no spinning wheels, no red badge pressure.

### 3.3 Animation Timing Bounds
- Button feedback: `120–180 ms` (`cubic-bezier(0.16, 1, 0.3, 1)`)
- Micro-transitions (toggles/tabs): `150–220 ms`
- Card & window transitions: `180–250 ms`
- Major route transitions: `200–300 ms`
- Complete honor of `prefers-reduced-motion: reduce`.

---

## 4. Accessibility & UI Scaling Standards (WCAG 2.2 AA)

1. **Contrast:** Minimum $4.5:1$ contrast for normal text, $3:1$ for large text and UI controls.
2. **Tabular Numerals:** Enforce `font-variant-numeric: tabular-nums` (class `tabular-nums`) on all clocks, timers, metrics, and progress percentages to prevent jitter.
3. **Colour-Independent Status:** Status badges combine color with geometric icons (Checkmark for Healthy/Done, Warning Triangle for Degraded/Alert, Octagon for Error/Locked) plus full `aria-label` metadata.
4. **Interface Density Modes:**
   - `compact`: $28\text{px}$ row height, $8\text{px}$ padding, $12\text{px}$ text.
   - `comfortable` (Default): $36\text{px}$ row height, $12\text{px}$ padding, $14\text{px}$ text.
   - `spacious`: $48\text{px}$ row height, $16\text{px}$ padding, $16\text{px}$ text.
5. **Scalable Font Modes:** `sm` ($13\text{px}$), `md` ($14\text{px}$), `lg` ($16\text{px}$), `xl` ($18\text{px}$).

---

## 5. Activity-Aware Theme Suggestions
MindSet OS monitors student activity context via kernel events:
- Opening a PDF document or eBook in the Vault $\rightarrow$ suggests switching to **Paper Studio**.
- Evening study session or extended relaxation break $\rightarrow$ suggests switching to **Forest Zen**.
- Starting an interactive Quiz or Milestone challenge $\rightarrow$ suggests switching to **Cosmic Arcade**.

> **Crucial Rule:** The system NEVER forces an automatic theme switch without user consent. It presents an unobtrusive, non-blocking toast suggestion with an "Accept" or "Dismiss" action.

---

## 6. Desktop OS Shell, Window Manager & Activity Workspaces

### 6.1 Desktop Canvas
- Renders curated calm wallpapers (deep gradients, nature scenes, or custom user wallpapers).
- Desktop shortcut icons: `My Textbooks`, `Daily Journal`, `Board Exam 590 Target`, `MindSet Terminal`, `Recycle Bin`.
- Right-click Desktop Context Menu: Change Wallpaper, New Study Folder, Open Terminal, Switch Activity (`Super + Tab`), Display Settings.

### 6.2 MindSet Taskbar & Start Menu
- **Taskbar Dock:** Bottom Liquid Glass dock (`var(--glass-bg)`) with:
  - **MindSet Start Button (`Super`):** Opens Start Menu with pinned apps, activity switcher, user profile badge, and power controls (`Lock Screen Super+L`, `Sleep`, `Restart`, `Shutdown`).
  - **Pinned & Active App Icons:** Displays active window dots, window preview tooltips, and minimized window restore.
  - **Interactive System Tray:** Live tabular clock & calendar flyout, Active Focus Session countdown pill, Wi-Fi/Network status icon, Audio volume slider popup, Battery/Thermal profile indicator, Do Not Disturb toggle, and Notification Center drawer toggle.

### 6.3 Multi-Window Manager & Snap Layouts
- High-performance draggable, resizable windows running at 60 FPS using CSS `translate3d`.
- **Snap Layouts (`Super + Arrow Keys` or drag-to-edge / hover snap picker):**
  - **2-Grid Split (50% / 50%):** Textbook PDF Reader on the left + Notes/Journal on the right.
  - **3-Grid Study Layout:** 50% Left (Main Reader) + 25% Top-Right (AI Tutor) + 25% Bottom-Right (Tasks Checklist).
  - **4-Grid Quadrant Layout:** 4 equal tiles (25% each).

### 6.4 KDE Plasma-Style Activity Workspaces (`Super + Tab`)
Isolates windows, desktop shortcuts, pinned taskbar tools, and suggested themes across 4 dedicated environments:
1. **Deep Study Activity:** Pins Command Center, Study Reader, Focus Timer, and StudyAgent; suggests `aurora-focus` or `paper-studio`.
2. **Coding & Projects Activity:** Pins MindSet Terminal, File Explorer, and Tasks; suggests `aurora-focus`.
3. **Planning & Reflection Activity:** Pins Daily Journal, Habit Tracker, Mood Check-ins, and Timeline; suggests `forest-zen`.
4. **Controlled Relaxation Activity:** Pins MindSet Balance AI and Hobby Discovery; suggests `forest-zen`.

