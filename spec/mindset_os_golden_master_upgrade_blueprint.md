# MindSet OS — Golden Master Upgrade Blueprint (v1.0.0-GM -> v1.1.0 Student Exam OS)

**Target Architecture:** Existing Tauri 2 + Rust Kernel + Dual SQLite (`local_core.sqlite` & `sync_oplog.sqlite` with HLC) + Laya/Ollama Local AI Router + `@mindset/focus-guard` + `@mindset/connect-protocol`  
**Baseline Preservation:** 100% compatible with existing 113 verified tests (55 Rust + 58 TypeScript), <100 MB idle shell+kernel RAM ceiling, zero-telemetry default, and hardened PowerShell/terminal/Ed25519 security gates.

---

## 1. Architectural Alignment Strategy

Rather than rewriting your verified Golden Master backend, every new capability plugs directly into existing subsystems discovered in `MINDSETOS_PRODUCTION_READINESS_AUDIT.md`:

| New Vision Capability | Existing Backend Anchor | Upgrade Strategy |
| :--- | :--- | :--- |
| **Exam Scout Agent & Target-to-Capsule Engine** | `packages/laya-router` + `packages/identity-engine` + `local_core.sqlite` | Add `packages/exam-engine` + SQLite Schema v3 (`exam_goals`, `syllabus_units`, `daily_capsules`) with backward mark-gap math and buffer-day auto-healing. |
| **Proof-of-Study Verification Boss (100-Pt Gate)** | `packages/focus-guard` (`FocusSessionManager`, `XpEngine`) + `LayaEngine` | Replace manual task completion on daily capsules with a Rust-enforced `CapsuleState` machine requiring `total_score >= 70` (or `>= 85` in speedrun test-out). |
| **5 Panel-Approved Engagement Features + Exam-Sprint Mode** | `user_personalization` (Schema v2) + `DesktopShell` + `connect-protocol` | Add Rest Shields (1 per 6 verified capsules, max 2), sub-300ms Boss HP feedback, Zero-Chat Silent Library presence dots, milestone static Living Wallpaper swaps, post-capsule Earned Workspace unlocks, and 1-click `exam_sprint_mode`. |
| **Official Website Profile (`mindsetos.com/@handle`) & Mobile Sync** | `sync_oplog.sqlite` (HLC) + `ed25519-dalek` + `apps/mindset-mobile` (PWA) | Add opt-in `cloud_sync_queue` using your existing `Ed25519` device keypair to sign verified `capsule-commit` and `heartbeat` JSON payloads without leaking raw notes or breaking offline mode. |
| **Low-RAM AI Protection (4GB–8GB Student Laptops)** | `MemoryWatchdogService` + `LayaEngine::unload_active_model_from_ram` | Enforce `keep_alive: 0` immediately after Exam Scout blueprint generation or Stage-3 Feynman grading so `gemma2:2b` (~1.85 GB) never starves system RAM. |

---

## 2. Database Schema v3 Migration (`local_core.sqlite` & `sync_oplog.sqlite`)

Extend `apps/mindset-desktop/src-tauri/src/db/migrations.rs` inside an atomic transaction (with automatic `.backup.sqlite` snapshot and `PRAGMA integrity_check;`):

```sql
-- Migration v3: Exam Scout, Target-to-Capsule, Proof-of-Study Boss & Engagement State
CREATE TABLE IF NOT EXISTS exam_goals (
    id TEXT PRIMARY KEY,
    exam_code TEXT NOT NULL,              -- e.g., 'TN_12_BOARD', 'NEET', 'JEE', 'UPSC', 'TNPSC', 'CA'
    exam_title TEXT NOT NULL,             -- e.g., 'TN Class 12 Public Exam 2027'
    target_score REAL NOT NULL,           -- e.g., 590.0
    max_score REAL NOT NULL,              -- e.g., 600.0
    baseline_score REAL NOT NULL,         -- e.g., 430.0
    predicted_score REAL NOT NULL,        -- Updated ONLY by verified capsules & retention checks
    exam_date_iso TEXT NOT NULL,          -- e.g., '2027-03-01'
    weekday_budget_mins INTEGER NOT NULL DEFAULT 240,
    weekend_budget_mins INTEGER NOT NULL DEFAULT 420,
    safety_buffer_marks REAL NOT NULL DEFAULT 10.0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at_hlc INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS syllabus_units (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL REFERENCES exam_goals(id) ON DELETE CASCADE,
    subject TEXT NOT NULL,
    chapter_title TEXT NOT NULL,
    exam_weight_marks REAL NOT NULL,
    pyq_frequency_score REAL NOT NULL,    -- 0.0 to 1.0 from 10-year PYQ forensics
    roi_quadrant TEXT NOT NULL,           -- 'GOLDMINE' | 'HEAVYWEIGHT' | 'QUICK_WIN' | 'TRAP'
    phase_number INTEGER NOT NULL,        -- 1..=4
    mastery_pct REAL NOT NULL DEFAULT 0.0,
    freshness_pct REAL NOT NULL DEFAULT 100.0,
    last_verified_at_iso TEXT,
    next_revision_due_iso TEXT
);

CREATE TABLE IF NOT EXISTS daily_capsules (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL REFERENCES exam_goals(id) ON DELETE CASCADE,
    scheduled_date_iso TEXT NOT NULL,
    unit_id TEXT NOT NULL REFERENCES syllabus_units(id),
    boss_name TEXT NOT NULL,              -- e.g., 'Lord Electrochemistry'
    marks_value REAL NOT NULL,            -- e.g., 3.5
    planned_minutes INTEGER NOT NULL,
    active_study_seconds INTEGER NOT NULL DEFAULT 0,
    state TEXT NOT NULL,                  -- 'LOCKED'|'ACTIVE'|'STUDYING'|'READY_FOR_BOSS'|'REMEDIATION'|'COMPLETED'
    speedrun_mode INTEGER NOT NULL DEFAULT 0,
    highest_boss_score INTEGER NOT NULL DEFAULT 0,
    rank_awarded TEXT,                    -- 'GOLD' (70..89) | 'PLATINUM' (90..100)
    xp_awarded INTEGER NOT NULL DEFAULT 0,
    verified_at_iso TEXT,
    hlc_timestamp INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS boss_attempts (
    id TEXT PRIMARY KEY,
    capsule_id TEXT NOT NULL REFERENCES daily_capsules(id) ON DELETE CASCADE,
    attempt_number INTEGER NOT NULL,
    recall_points INTEGER NOT NULL,       -- 0..=30
    application_points INTEGER NOT NULL,  -- 0..=40
    feynman_points INTEGER NOT NULL,      -- 0..=30
    focus_loss_count INTEGER NOT NULL DEFAULT 0,
    total_score INTEGER NOT NULL,         -- 0..=100
    passed INTEGER NOT NULL,              -- 1 iff total_score >= threshold
    weak_subtopics_json TEXT NOT NULL,
    created_at_iso TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS student_engagement_state (
    profile_id TEXT PRIMARY KEY,
    rest_shields_held INTEGER NOT NULL DEFAULT 0 CHECK (rest_shields_held BETWEEN 0 AND 2),
    consecutive_verified_capsules INTEGER NOT NULL DEFAULT 0,
    exam_sprint_mode INTEGER NOT NULL DEFAULT 0,
    silent_library_enabled INTEGER NOT NULL DEFAULT 1,
    living_wallpaper_stage INTEGER NOT NULL DEFAULT 1,
    unlocked_cosmetics_json TEXT NOT NULL DEFAULT '["theme_obsidian_default"]',
    website_sync_enabled INTEGER NOT NULL DEFAULT 0,
    website_handle TEXT,
    share_predicted_score_public INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS outbound_sync_queue (
    event_id TEXT PRIMARY KEY,            -- UUIDv7 for server idempotency
    event_type TEXT NOT NULL,             -- 'CAPSULE_COMMIT' | 'PRESENCE_HEARTBEAT'
    payload_json TEXT NOT NULL,
    ed25519_signature_hex TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING', -- 'PENDING' | 'SENT' | 'ACKED' | 'FAILED'
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at_iso TEXT NOT NULL
);
```

---

## 3. New Monorepo Packages & Rust Kernel Modules

### 3.1 New TypeScript Workspace Package: `packages/exam-engine`
Create `packages/exam-engine` (registered in `pnpm-workspace.yaml` and `tsconfig.base.json`) containing pure, deterministic domain logic with unit tests:
1. **`ExamScoutPlanner.ts`:**
   * Bundled offline exam blueprint packs for **TN Class 12 Boards (600 marks)**, **NEET (720 marks)**, **JEE Main**, **UPSC Prelims/Mains**, **TNPSC Group I/II/IV**, and **CA Foundation/Inter**, plus custom syllabus import.
   * Classifies units into `GOLDMINE`, `HEAVYWEIGHT`, `QUICK_WIN`, and `TRAP` using exam mark weightage and PYQ frequency.
   * Splits remaining calendar days into 4 Macro Phases (`45%` Core Foundation, `25%` Gap Closure, `20%` Exam Simulation, `10%` Lock-In & Taper) with `10%` built-in buffer days.
2. **`AutoHealingScheduler.ts`:**
   * When a day is missed: consumes a **Rest Shield** (if held) to protect the streak, absorbs missed marks into buffer days first, then trims low-ROI `TRAP` units before raising daily `HEAVY_LIFT` load by at most `10%`.
3. **`BossVerificationEngine.ts`:**
   * Computes `total_score = recall_points (0..30) + application_points (0..40) + feynman_points (0..30)`.
   * Enforces hard pass threshold: `>= 70` standard, `>= 85` if `speedrun_mode == true`.
   * Awards **1 Rest Shield** every 6 consecutive verified capsules (capped at `2` held).
   * Computes **Living Wallpaper Stage** (`1..=6`) from `predicted_score / target_score` milestones.

### 3.2 Rust Kernel Extensions (`apps/mindset-desktop/src-tauri/src/`)
1. **`exam/mod.rs` & `exam/commands.rs`:**
   * Typed Tauri IPC commands (`exam_create_goal`, `exam_get_today_capsule`, `exam_record_study_heartbeat`, `exam_submit_boss_attempt`, `exam_toggle_sprint_mode`, `exam_apply_post_capsule_cosmetic`).
   * **Kernel-Enforced Completion Lock:** `exam_submit_boss_attempt` verifies `total_score >= threshold` inside Rust before updating `daily_capsules.state = 'COMPLETED'`, incrementing `exam_goals.predicted_score`, recording an HLC oplog entry, and enqueuing a signed `CAPSULE_COMMIT` event.
   * **Post-Capsule Customization Guard:** `exam_apply_post_capsule_cosmetic` rejects theme/cosmetic changes if a capsule is currently in `STUDYING` state (unless unlocking after a verified Boss win), preventing mid-study procrastination.
2. **`ai/boss_generator.rs` (Extending `LayaEngine`):**
   * Uses existing XML-delimited `<user_context>` prompt sanitization to generate freshly randomized 3-stage Boss questions from the active capsule's chapter notes/PDFs via local Ollama (`127.0.0.1:11434`).
   * Grades Stage-3 Feynman explanations on a 0–30 rubric with automatic deterministic keyword/concept fallback if Ollama is stopped or memory-constrained.
   * Immediately triggers `unload_active_model_from_ram` (`keep_alive: 0`) on devices with `< 8 GB` physical RAM.
3. **`connect/website_sync.rs` (Extending `connect` & `ed25519-dalek`):**
   * Opt-in background worker (`tokio` task) that drains `outbound_sync_queue` when `website_sync_enabled == 1` and internet is available.
   * Signs each `CAPSULE_COMMIT` JSON payload with the device's `Ed25519` private key (`X-MindSet-Signature`), never transmitting raw notes, PDFs, or journal entries.
   * Emits 60-second `PRESENCE_HEARTBEAT` pings (`ttl_seconds: 120`) for the official website live badge and **Zero-Chat Silent Library** active presence dots (exposing *active study state only*, never total daily hours).

---

## 4. UI Upgrades (`apps/mindset-desktop/src/`)

1. **`CommandCenterPage.tsx` -> Today's Capsule Hero:**
   * Displays Today's Capsule (`+3.5 Marks Value -> On Track for 590/600`), the 4 capsule blocks (`Heavy Lift`, `Active Practice`, `Memory Anchor`, `Verification Boss`), `Predicted Exam Score` gauge, `Rest Shields` badge (`0/2` to `2/2`), and a global **Exam-Sprint Mode** toggle in the TitleBar.
2. **`VerificationBossModal.tsx`:**
   * Crisp `<300ms` Boss HP bar animations (automatically disabled in `Exam-Sprint Mode`), 3-stage quiz flow, window-blur anti-cheat listener (regenerating the active question if focus is lost for `> 4s`), and immediate animated Predicted Score tick on Gold (`70–89`) or Platinum (`90–100`) clear.
3. **`SilentLibraryTopBar.tsx`:**
   * Minimal top-bar presence indicator showing paired squad members currently in Deep Focus (`"3 friends studying right now"`) with one-click post-block silent reactions (`Chai ☕` / `Fire 🔥`) and zero chat or hour leaderboards.
4. **`SettingsPage.tsx` -> Website Profile & QR Mobile Pairing Tab:**
   * Configure `mindsetos.com/@handle`, granular privacy toggles (keep exact Predicted Score private or shared via parent token), and QR code pairing for `apps/mindset-mobile` (commute flashcards & verified share cards, zero desk-hour phone nudges).

---

## 5. Verification Gates Before Shipping v1.1.0

* [ ] `pnpm typecheck` passes with **0 errors across all 11 workspace packages** (including new `@mindset/exam-engine`).
* [ ] `pnpm test:all` passes **100% of existing 113 tests + all new exam/boss/sync unit tests**.
* [ ] Rust unit test proves `exam_submit_boss_attempt` rejects scores `0`, `49`, and `69` (and `84` in speedrun mode) without marking a capsule completed.
* [ ] `MemoryWatchdogService` confirms idle shell + kernel RAM remains `< 100 MB RSS` and Ollama evicts model weights after Boss generation on `< 8 GB` RAM systems.
* [ ] `outbound_sync_queue` signs payloads with `ed25519-dalek` and verifies offline persistence when network is disconnected.