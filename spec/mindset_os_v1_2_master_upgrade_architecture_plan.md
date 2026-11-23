# MindSet OS (`msOS`) — Master Upgrade & Implementation Specification (v1.2)

**Workspace Root:** `C:\MINDSETOS`  
**Confirmed Runtime Stack:** Pure Rust + `iced 0.14.0` (`wgpu`, `tokio`, `image`, `canvas`, `svg`, `advanced`) + `mindset-motion` (`Spring1D`) + SQLite (`rusqlite`, WAL mode, $\le 2\text{ MB}$ cache)  
**Target Hardware Envelope:** Windows 11 Desktop & Low-Spec Student Laptops ($4\text{--}8\text{ GB}$ RAM) · Target Idle GUI RSS $< 95\text{ MB}$ · Idle CPU $< 0.5\%$ (Eco) / $< 1.5\%$ (Mild Dynamic)  
**Brand Identity:** **MindSet OS** (Monogram Logo: **MS**) & **Maindy AI** (keep `"Maindy"` / `"Maindy AI"` consistent everywhere)

---

## 1. Executive Synthesis (Audits + Live Screen Demo + New Vision)

Cross-referencing the live UI walkthrough with `MINDSET_AUDIT_REPORT.md`, `MINDSET_OS_AUDIT_REPORT.md`, `mindset_os_design_architecture_specification.md`, and `target_to_capsule_exam_scout_agent_spec.md` reveals that `C:\MINDSETOS` is a pure-Rust Iced `0.14.0` desktop environment (not Electron/Tauri) with strong domain logic in `crates/mindset-core`, but critical presentation-layer, input-handling, networking, and accessibility gaps in `bins/mindset-os`.

This blueprint resolves **all four layers** in a single cohesive architecture:
1. **Critical Bug & Interaction Repairs:** File Vault click/selection & cursor bugs, draggable/repositionable windows (`wm/mod.rs` + `main.rs`), Preferences sidebar tab routing, Control Center live sliders & brightness scrim, `Target: 10005` DB auto-heal, async debounced SQLite scratchpad, and fixing `ai/engine.rs` raw TCP chunked framing.
2. **Spatial OS Chrome & Widget Upgrade:** 3-Zone Top Bar (Left group with vector **MS** logo, Center **Dynamic Island**, Right system tray), Right-Anchored Notification Side Widget, User-Draggable & Pinnable Desktop Widgets (persisted in SQLite), upgraded **3D Syllabus Odyssey Trail**, and upgraded **Quick Study Scratchpad**.
3. **System-Wide Multi-Language Accessibility (`i18n`):** Native English (`en`), Tamil (`ta` — தமிழ்), Malayalam (`ml` — മലയാളം), and extensible Indic locale architecture with bundled Noto Sans Tamil/Malayalam font shaping in Iced `0.14.0` (`cosmic-text`).
4. **Maindy AI v2 (Thanglish/Tamil/English Text + Dual-Text TTS + Hybrid API Slot / Free Local AI):** Conversational Thanglish (*"Vanakkam da mapla!"*), Tamil, Malayalam, and English support; dual-text pronunciation pipeline so TTS never mangles Romanized Tamil; Bring-Your-Own-Key (BYOK) API Holder Slot (Gemini, OpenAI, Groq, OpenRouter, Custom OpenAI-compatible); and an automatic zero-key Free Local AI fallback chain (Ollama $\rightarrow$ Optional Local GGUF Sidecar $\rightarrow$ Offline Multilingual Study Helper).

---

## 2. Target Workspace Module Map (`bins/mindset-os/src/`)

```text
bins/mindset-os/
├── assets/
│   ├── fonts/
│   │   ├── NotoSansTamil-Regular.ttf       # Bundled for guaranteed Tamil shaping
│   │   ├── NotoSansTamil-Bold.ttf
│   │   ├── NotoSansMalayalam-Regular.ttf   # Bundled for guaranteed Malayalam shaping
│   │   └── NotoSansMalayalam-Bold.ttf
│   └── icons/
│       └── ms_logo.svg                     # Crisp minimalist geometric "MS" vector monogram
└── src/
    ├── main.rs                             # App coordinator, mouse drag/resize, brightness scrim, zero-idle springs
    ├── i18n/
    │   ├── mod.rs                          # NEW: Language enum, LocaleRegistry, reactive t(key) lookup
    │   ├── en.json                         # NEW: English dictionary
    │   ├── ta.json                         # NEW: Tamil (தமிழ்) dictionary
    │   └── ml.json                         # NEW: Malayalam (മലയാളം) dictionary
    ├── ai/
    │   ├── mod.rs                          # NEW: Unified AI router (Cloud BYOK -> Ollama -> Offline Helper)
    │   ├── provider.rs                     # NEW: AiProvider trait + Gemini/OpenAI/Groq/OpenRouter/Custom clients
    │   ├── engine.rs                       # REFACTOR: reqwest streaming Ollama client (fixes raw TCP chunking bug)
    │   ├── persona.rs                      # NEW: Maindy AI system prompts (Thanglish/Tamil/English/Malayalam + Dual-Text JSON)
    │   ├── offline_helper.rs               # NEW: Zero-failure offline multilingual study & formula assistant
    │   └── tts.rs                          # NEW: Dual-Text TTS router (Windows OneCore/SAPI + Sarvam/Cloud + rodio playback)
    ├── shell/
    │   ├── top_bar.rs                      # REFACTOR: 3-Zone layout (Left + MS Logo | Center Notch | Right Tray)
    │   ├── dynamic_island.rs               # NEW: Morphing obsidian notch (Compact/Search/Control/Focus/MaindySpeaking)
    │   ├── notifications.rs                # NEW: Right-edge anchored glassmorphic Notification Side Widget
    │   ├── desktop.rs                      # REFACTOR: Draggable/pinnable widgets + live weather + dynamic events
    │   ├── odyssey_map.rs                  # NEW: 3D Perspective Syllabus Odyssey Trail (Dual canvas::Cache @ 18 FPS)
    │   ├── control_center.rs               # FIX: Live brightness/volume/toggles + quick language pill
    │   └── dock.rs                         # UPGRADE: Spring1D hover wave + zero-idle stop condition
    ├── apps/
    │   ├── mod.rs                          # FIX: Interactive title-bar drag handle + z-order focus + opaque backing
    │   ├── file_explorer.rs                # FIX: Clickable folders/files, pointer cursor, double-click nav, live preview
    │   ├── preferences.rs                  # FIX: Working sidebar tabs + new Language & Region + Maindy AI & Keys tabs
    │   └── ai_tutor.rs                     # UPGRADE: Language/Thanglish mode selector, Speak/Auto-Speak button, AI badge
    ├── wm/
    │   └── mod.rs                          # UPGRADE: Full pointer drag/resize delta engine + viewport clamping
    ├── storage/
    │   └── db.rs                           # UPGRADE: Schema migrations (scratchpad, widget_layouts, ai_config, i18n, auto-heal)
    └── system/
        ├── state.rs                        # FIX: Async debounced scratchpad state + audio/TTS state
        └── audio.rs                        # NEW: rodio playback manager (TTS audio bytes + Lo-Fi ambient)
```

---

## 3. SQLite Schema Upgrades (`storage/db.rs`)

All migrations MUST run idempotently inside a transaction on startup in `StorageManager::open_connection()` while preserving `PRAGMA journal_mode = WAL;` and `PRAGMA cache_size = -2000;`.

```sql
-- 1. Profile auto-heal & OS customization columns (idempotent ALTER TABLE checks)
-- Ensure target_score_value <= target_score_max on load (auto-heals corrupted 10005 -> 590)
ALTER TABLE user_profile ADD COLUMN language_code TEXT NOT NULL DEFAULT 'en';
ALTER TABLE user_profile ADD COLUMN maindy_style TEXT NOT NULL DEFAULT 'thanglish'; -- 'thanglish'|'ta'|'en'|'ml'|'auto'
ALTER TABLE user_profile ADD COLUMN wallpaper_path TEXT NOT NULL DEFAULT 'assets/wallpaper.jpg';
ALTER TABLE user_profile ADD COLUMN map_theme TEXT NOT NULL DEFAULT 'AlpineTrail';
ALTER TABLE user_profile ADD COLUMN mild_motion_enabled INTEGER NOT NULL DEFAULT 1;

-- 2. Async debounced scratchpad persistence (replaces synchronous scratchpad.txt writes)
CREATE TABLE IF NOT EXISTS scratchpad_notes (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    content TEXT NOT NULL DEFAULT '',
    is_pinned INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);

-- 3. Free-placement desktop widget coordinates (persisted across reloads)
CREATE TABLE IF NOT EXISTS widget_layouts (
    widget_id TEXT PRIMARY KEY, -- 'greeting_card' | 'odyssey_map' | 'scratchpad' | 'weather_card'
    pos_x REAL NOT NULL,
    pos_y REAL NOT NULL,
    width REAL NOT NULL,
    height REAL NOT NULL,
    is_pinned INTEGER NOT NULL DEFAULT 0,
    z_index INTEGER NOT NULL DEFAULT 1
);

-- 4. Maindy AI Provider Holder Slot & TTS Configuration
CREATE TABLE IF NOT EXISTS ai_provider_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    active_mode TEXT NOT NULL DEFAULT 'auto',       -- 'auto' | 'cloud' | 'ollama' | 'offline'
    cloud_provider TEXT NOT NULL DEFAULT 'gemini',  -- 'gemini' | 'openai' | 'groq' | 'openrouter' | 'custom'
    cloud_api_key TEXT NOT NULL DEFAULT '',         -- Obfuscated/DPAPI-protected on Windows where available
    cloud_model TEXT NOT NULL DEFAULT 'gemini-2.5-flash',
    cloud_base_url TEXT NOT NULL DEFAULT '',
    ollama_host TEXT NOT NULL DEFAULT 'http://127.0.0.1:11434',
    ollama_model TEXT NOT NULL DEFAULT 'qwen2.5:1.5b',
    tts_engine TEXT NOT NULL DEFAULT 'os_native',   -- 'os_native' | 'sarvam' | 'google_tts' | 'elevenlabs'
    tts_api_key TEXT NOT NULL DEFAULT '',
    tts_auto_speak INTEGER NOT NULL DEFAULT 0,
    tts_speech_rate REAL NOT NULL DEFAULT 1.0
);
```

---

## 4. Detailed Engineering Specifications

### 4.1 Critical UI & Interaction Bug Fixes

#### A. Student File Vault Selection & Cursor (`apps/file_explorer.rs`)
* **Root Cause:** File/folder rows were rendered as inert containers or lacked proper `mouse_area` / `button` hit-testing, preventing selection state updates and leaving the default arrow cursor.
* **Fix:**
  * Wrap every sidebar folder (`Documents`, `Study Capsules`, `Downloads`, `Formula Sheets`) and file list item (`README_STUDY_VAULT.txt`, notes, capsules) in an interactive `button` or `mouse_area` with `.interaction(iced::mouse::Interaction::Pointer)`.
  * Maintain `selected_folder: PathBuf`, `selected_file: Option<PathBuf>`, `last_click_instant: Option<(PathBuf, Instant)>` for double-click detection ($\le 350\text{ ms}$).
  * Single-click selects the item, applies an active accent highlight border/fill, and immediately loads its content (capped to first $64\text{ KB}$ UTF-8 preview) into the `File Preview` pane.
  * Double-clicking a folder navigates into that directory and updates the breadcrumb bar.
  * Audit `main.rs:view()` layer order to guarantee no transparent modal backdrop swallows pointer events when `active_overlay == None`.

#### B. Draggable & Repositionable App Windows (`wm/mod.rs`, `apps/mod.rs`, `main.rs`)
* **Root Cause:** `wm/mod.rs` already defines `WindowInteractionState`, `clamp_window`, and `check_edge_snap`, but `main.rs` lacks `Message` variants and ignores `mouse::Event::CursorMoved` and `mouse::Event::ButtonReleased`.
* **Fix:**
  * Add `Message` variants:
    ```rust
    WindowFocus(AppId),
    WindowDragStart { id: AppId },
    WindowResizeStart { id: AppId, edge: ResizeEdge },
    PointerMoved(iced::Point),
    PointerReleased,
    ```
  * Wrap each window's title bar in `apps/mod.rs` with `mouse_area(title_bar).on_press(Message::WindowDragStart { id }).interaction(Interaction::Grab)`.
  * Wrap the entire window outer frame with `mouse_area(frame).on_press(Message::WindowFocus(id))` to promote its `z_index` to top without collapsing sibling panels.
  * In `main.rs::update`:
    * On `PointerMoved(point)`: store `self.cursor_pos = point`. If `wm_interaction.dragging` is `Some(drag_state)`, compute delta $(\Delta x, \Delta y) = \text{point} - \text{drag\_state.anchor}$, update the window's `(x, y)` coordinates, and run `wm_interaction.clamp_window(...)` so windows cannot clip beneath the top bar (`min_y = 54.0`) or off-screen.
    * On `PointerReleased`: clear `dragging` / `resizing` and commit any edge-snap layout.
  * **Eliminate Window Bleed-Through (`theme/mod.rs`):** Raise window chrome surface opacity to `0.96` (`AlpineGlass`) and `0.94` (`DarkSpace`) with a crisp `1 px` specular rim so desktop cards never bleed through open app windows.

#### C. Mindset Preferences Sidebar & Control Center Live State (`apps/preferences.rs`, `shell/control_center.rs`, `main.rs`)
* **Sidebar Tab Routing:** Store `pub active_pref_tab: PreferencesTab` (`Appearance`, `FocusAndShields`, `AudioAndRadios`, `LanguageAndRegion`, `MaindyAiAndKeys`, `HardwareAndPower`). Bind every sidebar button to `Message::SelectPreferencesTab(tab)` with `Interaction::Pointer` and an active pill style.
* **Control Center Live Bindings:**
  * **Brightness Scrim (`main.rs:view()`):** Render a non-blocking top-most scrim container with `Color::from_rgba(0.0, 0.0, 0.0, (1.0 - self.core.preferences.brightness).clamp(0.0, 0.72))` so dragging the Brightness slider smoothly dims/brightens the entire OS screen in real time. (Use a custom pass-through overlay or `canvas`/container that does not capture mouse events—important in Iced `0.14`: ensure the scrim sits visually above windows without blocking pointer hit-testing by implementing a zero-event `Widget` wrapper `PassThroughScrim`!).
  * **Master Volume:** Wire slider directly to `audio_manager.set_volume(val)` and persist to preferences.
  * **Window Overlap Fix:** Ensure Control Center renders in the dedicated `ActiveOverlay::ControlCenter` top layer (not duplicated inside `shell/desktop.rs`) so clicking controls never collapses behind background windows.

#### D. Audit Data Integrity Fixes
* **Build Fix:** Ensure `use iced::Color;` is present in `bins/mindset-os/src/views/modals.rs`.
* **Profile Auto-Heal:** Clamp `target_score_value` to `0..=target_score_max` in `storage/db.rs::load_profile()`, `onboarding/mod.rs`, and `main.rs`; auto-`UPDATE` corrupted values (`10005` $\rightarrow$ `590`) on startup. Render human-readable `profile.target_label`.
* **Live Weather & Events:** Bind `shell/desktop.rs` weather widget to `system.weather` (removing `"Bratislava"`) and filter `Upcoming Events` against `chrono::Local::now()`.
* **`Launch ->` Wrapping Fix:** Give capsule titles `Length::Fill` and lock the launch button text with `.wrapping(text::Wrapping::None)`.
* **Async Debounced Scratchpad:** Remove synchronous `std::fs::write` on keystrokes; debounce $500\text{ ms}$ via `scratchpad_revision: u64` and `Task::perform` into SQLite `scratchpad_notes`.

---

### 4.2 OS Shell Architecture: 3-Zone Top Bar, MS Logo & Anchored Notifications

#### A. 3-Zone Top Taskbar + "MS" Monogram Logo (`shell/top_bar.rs`, `shell/dynamic_island.rs`)
Restructure the top bar into **three visually separated floating groups** using a fixed-center layout (`stack!` or weighted `row!`):
1. **Left Floating Zone (`Alignment::Start`):**
   * Crisp vector **MS** logo (`bins/mindset-os/assets/icons/ms_logo.svg` rendered via `iced::widget::svg` at `22x22 px`, dynamically tinted by theme) at the far top-left corner.
   * Clean app menu items (`File`, `Edit`, `View`, `Help` — localized via `i18n`).
2. **Dead-Center Dynamic Island (`Alignment::Center`):**
   * Isolated obsidian pill (`#0B0E14`, rounded `999 px` in compact mode, `22 px` when expanded) driven by `Spring1D` width/height springs.
   * States: `Compact` (Active Focus / Streak / Clock pulse), `Search` (`Ctrl+K` live filter), `ActiveFocus` (Pomodoro timer), and **`MaindySpeaking`** (compact animated audio waveform bars + Stop button while Maindy AI TTS is speaking!).
3. **Right Floating Zone (`Alignment::End`):**
   * Quick Language Pill (`EN` / `தமிழ்` / `മല` — cycles or opens quick dropdown).
   * Maindy AI Status Dot (Green = Cloud API, Cyan = Local Ollama, Amber = Offline).
   * System tray icons (Wi-Fi, Volume, Battery), Control Center toggle, Notification Bell icon (with unread count badge), and live Clock.

#### B. Anchored Notification Side Widget (`shell/notifications.rs`)
* **Fix:** Remove center-screen modal positioning for `ActiveOverlay::Notifications`.
* Render Notifications as a **right-anchored floating glassmorphic side drawer** (`width: 340 px`, positioned at `top: 56 px`, `right: 16 px`) using a right-aligned overlay container (`container(panel).align_x(Horizontal::Right).align_y(Vertical::Top).padding(Padding { top: 56.0, right: 16.0, bottom: 16.0, left: 0.0 })`).
* Clicking outside the drawer closes it cleanly; includes `"Clear All"`, `"Mark Read"`, and per-notification dismiss buttons.

---

### 4.3 Free-Placement Desktop Widgets & 3D Syllabus Odyssey Trail

#### A. Draggable & Pinnable Desktop Widgets (`shell/desktop.rs`, `storage/db.rs`)
* Convert the desktop workspace into a positioned canvas layer (`stack!` of absolute-offset containers driven by `HashMap<String, WidgetLayout>`).
* Each widget (`Good Evening, Scholar!`, `3D Syllabus Odyssey Trail`, `Quick Study Scratchpad`) has:
  * A subtle top drag-bar handle (`⠿`) with `Interaction::Grab`.
  * A **Pin / Lock Toggle** (`📌`) preventing accidental drags once positioned.
  * Optional $16\text{ px}$ snap-to-grid alignment on release.
  * Automatic debounced save to SQLite `widget_layouts` + a `"Reset Layout"` option in Preferences.

#### B. Redesigned 3D Syllabus Odyssey Trail (`shell/odyssey_map.rs`)
* Implemented with `iced::widget::canvas` using **dual `canvas::Cache`** (`static_road_cache` + `dynamic_ambient_cache` throttled to ~18 FPS only when visible and `mild_motion_enabled == true`):
  * **Perspective 3D Winding Path:** Parametrized depth scale $z(t) = 1.0 - 0.52 \cdot t$, gradient ribbon road connecting completed segments in glowing cyan/emerald and upcoming segments in frosted slate.
  * **3D Extruded Node Pucks:** Ground shadow + extruded base + lit top face + specular rim arc:
    * *Completed:* Emerald fill + checkmark.
    * *Current Active:* Pulsing radial glow ring + elevated lift.
    * *Boss Milestone:* Gold crown/trophy puck.
    * *Locked/Upcoming:* Frosted matte puck.
  * **Hover & Click Interactivity:** Hovering lifts the puck smoothly and displays a floating glass tooltip (Chapter Name, Mastery `%`, Estimated Study Time, Mark Value); clicking launches or focuses that chapter's Study Capsule.
  * **Header Progress Bar:** Compact overall syllabus mastery bar (`X% Syllabus Cleared`).

#### C. Upgraded Quick Study Scratchpad (`shell/desktop.rs`)
* Fully editable `text_editor` widget with monospace formula styling (handles equations like `E = mc^2`, `hf = W + KE`, `Phi = q_enc / eps_0`).
* Action bar: **Auto-Save indicator** (`"Saving..."` / `"Saved to SQLite"`), **Copy to Clipboard** button, **Clear / New Note** button, **Ask Maindy about this note** button (sends scratchpad text directly into Maindy AI chat!), and **Pin** toggle.

---

### 4.4 System-Wide Multi-Language Accessibility (`i18n`)

#### A. Pure Rust Extensible Localization Registry (`src/i18n/mod.rs`)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppLanguage {
    English,   // "en"
    Tamil,     // "ta" — தமிழ்
    Malayalam, // "ml" — മലയാളം
    // Extensible: drop hi.json / te.json / kn.json and add enum variant
}

pub struct I18n {
    current: AppLanguage,
    dictionaries: HashMap<AppLanguage, HashMap<String, String>>,
}

impl I18n {
    pub fn t(&self, key: &str) -> String {
        self.dictionaries
            .get(&self.current)
            .and_then(|d| d.get(key))
            .or_else(|| self.dictionaries.get(&AppLanguage::English).and_then(|d| d.get(key)))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}
```
* Embed `en.json`, `ta.json`, and `ml.json` at compile time with `include_str!(...)` (zero runtime file-not-found errors, $< 40\text{ KB}$ binary impact) while allowing user-supplied external JSON language packs in `%LOCALAPPDATA%\MindSetOS\locales\` to override or add new languages (Hindi, Telugu, Kannada) without recompiling!

#### B. Indic Complex Script Shaping & Font Loading (CRITICAL for Iced `0.14.0`)
* Iced `0.14.0` uses `cosmic-text` + `rustybuzz` for complex Indic ligature shaping (Tamil uyirmei vowels, Malayalam chillu letters).
* Register bundled fonts during app startup in `main.rs`:
  ```rust
  iced::application(MindSetApp::new, MindSetApp::update, MindSetApp::view)
      .font(include_bytes!("../assets/fonts/NotoSansTamil-Regular.ttf").as_slice())
      .font(include_bytes!("../assets/fonts/NotoSansTamil-Bold.ttf").as_slice())
      .font(include_bytes!("../assets/fonts/NotoSansMalayalam-Regular.ttf").as_slice())
      .font(include_bytes!("../assets/fonts/NotoSansMalayalam-Bold.ttf").as_slice())
  ```
* If font files are not yet downloaded locally, fall back gracefully to Windows system font `"Nirmala UI"` (pre-installed on all Windows 10/11 machines with complete Tamil & Malayalam coverage) and document exact font download commands in setup instructions.
* **UI Coverage:** Translate top bar menus, window titles, Preferences tabs, Control Center labels, File Vault headings, Notification drawer, Odyssey Map header, and dynamic time-of-day greetings:
  * English: `"Good Evening, Scholar!"`
  * Tamil: `"மாலை வணக்கம், மாணவரே!"`
  * Malayalam: `"ശുഭസായാഹ്നം, വിദ്യാർത്ഥി!"`

---

### 4.5 Maindy AI v2: Bilingual Thanglish/Tamil/English Text + Dual-Text TTS

#### A. Maindy AI Conversation Modes (`src/ai/persona.rs`)
Users can switch Maindy AI's chat style independently of the OS UI language inside the Maindy AI header or Preferences:
1. **`Thanglish` (Default Casual Study Buddy):** Speaks natural Tamil-English code-mixed text in Latin script—e.g. *"Vanakkam da mapla! Innaiku Photoelectric Effect ($hf = W + KE$) easy-aa mudichuralaama?"*
2. **`Tamil (தமிழ்)`:** Clear, encouraging native Tamil with standard English scientific terms kept intact so exam terminology stays accurate.
3. **`English`:** Clear, crisp Indian-English academic mentor tone.
4. **`Malayalam (മലയാളം)`:** Friendly Malayalam/Manglish study mentor tone.
5. **`Auto-Match`:** Detects user input script/style and mirrors it automatically.

#### B. The Dual-Text Pronunciation Pipeline (Why Thanglish TTS Sounds Natural)
Standard TTS engines fail when handed raw Thanglish (`"Vanakkam da mapla"`) because an English voice pronounces it with a heavy Western accent, while a Tamil voice expects Tamil Unicode script (`"வணக்கம் டா மாப்ள"`).

**Solution:** Maindy AI's system prompt instructs the LLM (Cloud or Ollama) to return a clean structured JSON block (with graceful plain-text fallback if a tiny local model skips JSON):
```json
{
  "display_text": "Vanakkam da mapla! Photoelectric effect-la threshold frequency romba mukkiyam. Formula paaru: hf = W + KE.",
  "speech_text": "வணக்கம் டா மாப்ள! Photoelectric effect-ல threshold frequency ரொம்ப முக்கியம். Formula பாரு: h f equals W plus K E.",
  "primary_lang": "ta-IN"
}
```
* **Chat UI renders `display_text`** (readable Thanglish + crisp math formulas).
* **TTS Engine speaks `speech_text`**, using a lightweight script-segmenter in `src/ai/tts.rs` that splits Unicode Tamil runs (`\u{0B80}..\u{0BFF}`), Malayalam runs (`\u{0D00}..\u{0D7F}`), and Latin English runs (`A-Za-z0-9`) so mixed sentences route cleanly!
* **Fallback Transliterator:** If a small local model returns raw Thanglish without JSON, `tts.rs` runs a fast phonetic Thanglish-to-Tamil lookup/mapping table for common conversational words (`vanakkam` $\rightarrow$ `வணக்கம்`, `mapla` $\rightarrow$ `மாப்ள`, `enna` $\rightarrow$ `என்ன`, `super` $\rightarrow$ `சூப்பர்`, `paaru` $\rightarrow$ `பாரு`, `nalla` $\rightarrow$ `நல்லா`) before speaking.

#### C. Multi-Tier Pure Rust TTS Engine (`src/ai/tts.rs` + `src/system/audio.rs`)
1. **Tier 1 — Free Built-In Native OS TTS (Zero API Key, Zero Setup, Offline):**
   * Uses `tts` crate / Windows OneCore Speech Synthesizer (`ta-IN` Microsoft Valluvar / `en-IN` Microsoft Heera/Neerja / `ml-IN`), executing asynchronously on a dedicated `tokio::task::spawn_blocking` worker so the Iced UI thread never stalls.
   * Includes Play, Pause/Stop, Speech Rate slider (`0.8x`–`1.4x`), per-message **Speak (`🔊`)** button, and **Auto-Speak** toggle.
2. **Tier 2 — Optional Neural Cloud TTS API Slot (Studio-Grade Code-Switching):**
   * Supports optional API keys in `Mindset Preferences -> Maindy AI & Keys`:
     * **Sarvam AI (`bulbul:v3` — Recommended for Tamil/Malayalam/Thanglish):** Single endpoint handles seamless Tamil-English code-switching natively with Indian voices; returns base64/WAV audio streamed directly to `rodio::Sink`.
     * **Google Cloud TTS (`ta-IN-Wavenet` / `en-IN-Wavenet`)** or **ElevenLabs**.
3. **Dynamic Notch Live Feedback:** While `tts_is_speaking == true`, the centered Dynamic Island displays a subtle animated audio bar wave + `"Maindy Speaking..."` + one-click Stop button.

---

### 4.6 Maindy AI Hybrid Backend: API Key Holder Slot + Free Local AI Fallback

#### A. Unified Provider Trait (`src/ai/provider.rs`)
Replace the brittle raw-TCP client in `src/ai/engine.rs` (which broke on HTTP chunked encoding per `MINDSET_OS_AUDIT_REPORT.md`) with `reqwest` (`features = ["json", "stream", "rustls-tls"]`) behind a clean async trait:

```rust
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> String;
    async fn health_check(&self) -> Result<ProviderStatus, String>;
    async fn stream_chat(
        &self,
        messages: Vec<ChatMessage>,
        system_prompt: String,
        tx: tokio::sync::mpsc::UnboundedSender<AiStreamChunk>,
        cancel: Arc<AtomicBool>,
    ) -> Result<(), String>;
}
```

#### B. Cloud API Holder Slot (`apps/preferences.rs -> Maindy AI & Keys`)
Users who have an API key can choose any provider in Preferences:
* **Google Gemini API:** `https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent` (default `gemini-2.5-flash`).
* **OpenAI API:** `https://api.openai.com/v1/chat/completions` (`gpt-4o-mini`).
* **Groq API:** `https://api.groq.com/openai/v1/chat/completions` (`llama-3.3-70b-versatile` or `qwen-qwq-32b`).
* **OpenRouter API:** `https://openrouter.ai/api/v1/chat/completions`.
* **Custom OpenAI-Compatible Endpoint:** User enters Base URL + Model Name + Optional API Key (works with LM Studio, vLLM, DeepSeek, Sarvam, etc.).
* **Security & UX Controls:** Masked password field (`secure(true)`) with Eye show/hide toggle, **"Test Connection"** button showing live badge (`Connected • 240ms`, `Invalid API Key`, or `Offline -> Using Free Local AI`), and **"Clear Key"** button.

#### C. Automatic Free Local AI Fallback Chain (Zero API Key Needed)
When no API key is configured (or `active_mode == "ollama"` / Cloud request fails due to no internet):
1. **Local Ollama Bridge (`http://127.0.0.1:11434`):**
   * Fixed `reqwest` NDJSON streaming parser (eliminates chunked-hex parsing drops!).
   * Auto-discovers installed models via `GET /api/tags`, prioritizing multilingual models (`qwen2.5:1.5b`, `qwen2.5:3b`, `gemma2:2b`, `llama3.2:3b`).
   * Enforces low-RAM safety: `num_ctx: 2048`, `keep_alive: "2m"`, and automatic `keep_alive: 0` eviction when RAM pressure is detected (`< 8 GB` hosts).
2. **Optional Local OpenAI-Compatible Server Auto-Detect:** Checks `http://127.0.0.1:1234` (LM Studio) or `http://127.0.0.1:8080` (`llama-server` GGUF sidecar) if Ollama is not running.
3. **Zero-Failure Offline Multilingual Study Helper (`src/ai/offline_helper.rs`):**
   * If neither a Cloud API key nor any local LLM server is active, Maindy AI NEVER crashes or shows a dead error box.
   * Responds instantaneously in the user's chosen mode (Thanglish/Tamil/Malayalam/English), pulls live formulas from the user's SQLite `scratchpad_notes` and `daily_capsules`, answers common physics/chemistry/math formula lookups locally, and displays a friendly 1-click guide to either install Ollama (`ollama pull qwen2.5:1.5b`) or paste a free Gemini/Groq API key in Preferences.

---

## 5. Strict 8-Phase Execution Plan (For Antigravity)

Run `cargo check --workspace` after **every single phase** before advancing.

| Phase | Scope & Exact Files | Acceptance Gate |
| :---: | :--- | :--- |
| **Phase 1** | **Build Health, DB Migrations & Auto-Heal:** Fix `Color` import in `views/modals.rs`, add SQLite migrations (`language_code`, `maindy_style`, `scratchpad_notes`, `widget_layouts`, `ai_provider_config`), auto-heal `target_score_value > target_score_max`, clamp onboarding inputs, migrate scratchpad to $500\text{ ms}$ async debounced SQLite writes. | `cargo check --workspace` compiles with 0 errors; corrupted `10005` score auto-heals to `590`. |
| **Phase 2** | **Core Input & Window Management Fixes:** Wire `WindowFocus`, `WindowDragStart`, `PointerMoved`, `PointerReleased` in `main.rs`, `wm/mod.rs`, `apps/mod.rs` with viewport clamping; fix `file_explorer.rs` selection, pointer cursors, double-click folder nav & preview; fix `preferences.rs` sidebar tab switching; add pass-through Brightness scrim & live Control Center bindings; raise window opacity to stop bleed-through. | All windows drag smoothly; File Vault selects & previews files; Preferences tabs switch cleanly; Brightness slider visually dims screen. |
| **Phase 3** | **Shell Layout, MS Logo & Anchored Notifications:** Create `assets/icons/ms_logo.svg`; rebuild `shell/top_bar.rs` into 3-Zone layout (Left + MS Logo, Center Dynamic Island, Right Tray + Lang Pill); move Notifications out of center modal into right-anchored side widget (`shell/notifications.rs`); remove duplicate desktop overlays; delete dead files in `views/`. | Top bar clearly separates Left, Center Notch, and Right groups; MS logo appears top-left; Notifications anchor top-right. |
| **Phase 4** | **Draggable Desktop Widgets, 3D Odyssey Map & Scratchpad:** Implement persisted `(x, y)` free-placement + pin toggle for desktop widgets in `shell/desktop.rs`; build dual-cache `shell/odyssey_map.rs` (3D perspective trail, extruded pucks, hover tooltips, pulsing active node, click-to-launch); upgrade Quick Study Scratchpad with Copy/Clear/Ask Maindy buttons; fix weather, events, and `Launch ->` wrapping. | Widgets drag & retain position across restarts; Odyssey Map renders smoothly at $\le 18\text{ FPS}$ ambient with zero idle drain when paused. |
| **Phase 5** | **System-Wide `i18n` & Indic Font Support:** Create `src/i18n/{mod.rs, en.json, ta.json, ml.json}`; register Tamil/Malayalam fonts (bundled Noto Sans or Nirmala UI fallback); wire `t(...)` across top bar, windows, widgets, Control Center, and Preferences; add `Language & Region` tab + Control Center quick toggle. | Switching between English, Tamil (தமிழ்), and Malayalam (മലയാളം) updates the entire OS live with zero tofu boxes and persists across restart. |
| **Phase 6** | **Hybrid Maindy AI Engine (API Slot + Free Local AI):** Add `reqwest` streaming; implement `ai/provider.rs` (Gemini, OpenAI, Groq, OpenRouter, Custom), refactor `ai/engine.rs` Ollama client (fixing chunked framing), build `ai/offline_helper.rs`, build `ai/persona.rs` (Thanglish/Tamil/English/Malayalam + Dual-Text JSON), and add `Maindy AI & Keys` tab in `apps/preferences.rs` with Test Connection & active mode badge. | Works smoothly with BYOK Cloud key, works with local Ollama with zero dropped chunks, and falls back cleanly to Offline Helper with no key. |
| **Phase 7** | **Maindy AI Bilingual TTS & Dynamic Notch Waveform:** Implement `src/ai/tts.rs` + `src/system/audio.rs` (`rodio` + Windows native OS TTS + optional Sarvam AI / Cloud TTS slot); wire dual-text `speech_text` + Thanglish fallback transliterator; add Speak/Auto-Speak controls in `ai_tutor.rs` and `MaindySpeaking` waveform state in `dynamic_island.rs`. | Maindy displays Thanglish (`"Vanakkam da mapla!"`), speaks natural Tamil/English audio without freezing UI, and animates in Dynamic Notch. |
| **Phase 8** | **Zero-Idle Verification, Tests & Release Build:** Verify `window::frames()` stops when `Spring1D::is_at_rest()`; run `cargo test --workspace`, `cargo clippy --workspace`, and `cargo build --release`; confirm binary size and low-RAM RSS compliance. | 100% passing unit tests, 0 compile errors, release binary builds cleanly under memory budget. |

---

## 6. Hard Guardrails (Do Not Violate)
1. **Pure Rust + Iced `0.14.0` Only:** Do NOT introduce Node.js, Electron, WebView, or Tauri into `bins/mindset-os`.
2. **Never Touch `archive/`:** Leave `archive/mindset-render-wgpu-v1/` completely untouched.
3. **Zero Idle 60 FPS Loops:** Never run unconditional `window::frames()` when all `Spring1D` animations are at rest.
4. **Never Block the GUI Thread:** All HTTP streaming, SQLite writes, file dialogs, and TTS speech synthesis MUST execute asynchronously on Tokio tasks or `spawn_blocking`.
5. **Preserve `"Maindy AI"` Identity:** Do not rename Maindy AI to Mindy or anything else.