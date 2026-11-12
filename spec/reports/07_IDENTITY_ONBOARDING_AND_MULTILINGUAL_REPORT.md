# MINDSET OS — ENGINEERING REPORT 07
## Identity & Personalisation Engine, Progressive Onboarding, Multilingual Architecture, and 3-Tier Memory

**Company:** LKS PVT LTD  
**Module:** `packages/identity-engine/` & `apps/mindset-desktop/src/pages/onboarding/`  
**Specification:** [`specs/03_IDENTITY_AND_MULTILINGUAL.md`](file:///c:/MINDSETOS/specs/03_IDENTITY_AND_MULTILINGUAL.md)  
**Status:** Complete & Verified  

---

### 1. Executive Summary

Prompt 07 delivers the foundational human layer of MindSet OS: **Identity as a Living Epistemic Model**. Rather than reducing the student to an email address and arbitrary settings, MindSet OS anchors every interaction in the student's authentic ambitions, daily timetable, cognitive learning style, and mother tongue.

Key systems engineered in this phase:
1. **`specs/03_IDENTITY_AND_MULTILINGUAL.md`:** Comprehensive master architecture detailing progressive disclosure, dual-layer localization, 3-tier memory, prompt-injection defense boundaries, and consent-driven profile evolution.
2. **`packages/identity-engine/`:**
   - [`PersonalContextBuilder.ts`](file:///c:/MINDSETOS/packages/identity-engine/src/PersonalContextBuilder.ts): Deterministic 8-step assembly pipeline with XML encapsulation (`<user_context_data>`), adversarial token redaction, Tanglish/Hinglish speech directives, and strict sub-800 token budgeting.
   - [`LivingProfileEngine.ts`](file:///c:/MINDSETOS/packages/identity-engine/src/LivingProfileEngine.ts): Heuristic observer for goal milestone completions, routine timetable drift, and habit streaks with an absolute **zero-silent-mutation guarantee** requiring explicit student consent.
   - [`MemoryInspectorService.ts`](file:///c:/MINDSETOS/packages/identity-engine/src/MemoryInspectorService.ts): Sovereign student data control suite offering inspection, search filtering, JSON/Markdown export, and granular deletion across all 3 memory levels.
3. **`apps/mindset-desktop/src/pages/onboarding/OnboardingWizard.tsx`:** 6-screen progressive onboarding wizard with instant theme preview, dual-layer bilingual calibration, top 3 goals builder with presets, 24-hour visual routine timeline, and live AI greeting synthesis.
4. **Desktop Shell Integration:**
   - [`MemoryInspectorModal.tsx`](file:///c:/MINDSETOS/apps/mindset-desktop/src/components/MemoryInspectorModal.tsx): Global shortcut (`Ctrl + M`) and Start Menu access.
   - [`LivingProfileSuggestionCard.tsx`](file:///c:/MINDSETOS/apps/mindset-desktop/src/components/LivingProfileSuggestionCard.tsx): Floating non-intrusive card prompting users when routines or goals evolve.

---

### 2. Architecture Diagram

```mermaid
flowchart TD
    subgraph UI_Layer ["Desktop Shell & Onboarding Layer"]
        OB["OnboardingWizard (6 Screens)"]
        DS["DesktopShell & StartMenu"]
        MIM["MemoryInspectorModal (Ctrl + M)"]
        LSC["LivingProfileSuggestionCard"]
    end

    subgraph Identity_Core ["packages/identity-engine"]
        PCB["PersonalContextBuilder (8-Step Pipeline)"]
        LPE["LivingProfileEngine (Evolution Heuristics)"]
        MIS["MemoryInspectorService (CRUD & Export)"]
    end

    subgraph Memory_Hierarchy ["3-Tier Memory Architecture"]
        L1["Level 1: Profile Memory (local_core.sqlite: user_profile)"]
        L2["Level 2: Active Working Context (Kernel Event Bus & State)"]
        L3["Level 3: Epistemic Long-Term Memory (sqlite-vec: vec_memory)"]
    end

    OB -->|Commits Onboarding Profile| L1
    L1 --> PCB
    L2 --> PCB
    L3 --> PCB
    PCB -->|Bounded & Sanitized Prompt < 800 tokens| AI["Local LLM (Phi-3 / Ollama)"]

    DS -->|Observes Task Completions & Focus| LPE
    LPE -->|Emits Suggestion Card| LSC
    LSC -->|User Confirms (Zero Silent Mutation)| L1

    MIM -->|Inspects & Exports| MIS
    MIS --> L1
    MIS --> L2
    MIS --> L3
```

---

### 3. Detailed Component Breakdown

#### 3.1 Progressive 6-Screen Onboarding Flow (`/onboarding`)
- **Screen 1 — Welcome & Neuroaesthetic Atmosphere:**
  - Displays philosophy: *"Your technology should help you achieve your goals, not compete for your attention."*
  - Interactive Theme Preview: Real-time live switching between all 5 themes (`aurora-focus`, `solar-bloom`, `forest-zen`, `cosmic-arcade`, `paper-studio`).
- **Screen 2 — Dual-Layer Multilingual Calibration:**
  - Layer A: OS UI Language (`en`, `ta`, `hi`, `te`, `ml`, `kn`, `bn`, `mr`).
  - Layer B: AI Speech Style (Conversational Tanglish, Pure Tamil, Hindi, Pure English).
  - Synchronous Live Preview Card comparing OS UI button labels with live AI response bubbles.
- **Screen 3 — Identity & Ambition:**
  - Preferred display name, role (`student`, `researcher`, `professional`).
  - Top 3 Goals Builder with single-click preset chips (*JEE Advanced 2027*, *NEET UG*, *UPSC Civil Services*, *Class 12 Boards (95%+)*, *Master Rust*).
  - Focus subject tags (*Physics*, *Mathematics*, *Chemistry*, *Computer Science*).
- **Screen 4 — Daily Routine & 24-Hour Visual Timeline:**
  - Wake-up (`06:00`), sleep time (`22:30`), school/work hours (`08:30 – 16:00`), and dedicated morning/evening study focus windows.
  - Interactive 24-hour visual progress bar color-coded by Sleep (indigo), Study Focus (accent), Commitments (slate), and Relaxation (emerald).
- **Screen 5 — Learning Style & AI Mentor Persona:**
  - 5 Cognitive Learning Styles (Step-by-Step, Visual Concept Maps, Practical Examples, Condensed Notes, Socratic Dialogue).
  - 4 Communication Tones (Friendly & Direct, Encouraging, Analytical, High-Yield Concise).
  - Planning style (Flexible vs Structured) and hobby rejuvenation tags.
- **Screen 6 — Meet Your AI & First Command Center Launch:**
  - Live AI Mentor greeting generated in real-time by `PersonalContextBuilder.generateGreeting(...)`.
  - Privacy and sovereignty guarantee checklist (100% local storage, offline inference, zero telemetry).

#### 3.2 `PersonalContextBuilder` 8-Step Pipeline & Prompt-Injection Guardrail
1. **Level 1 Profile Ingestion:** Safe parsing of JSON attributes with robust fallbacks.
2. **Level 2 Active Context Ingestion:** Ingests volatile timeline state (available free minutes before next commitment, active focus profile, top priority pending tasks).
3. **Level 3 Epistemic Memory Selection:** Top 3 relevant milestone and conceptual records.
4. **Prompt-Injection Sanitization:**
   - Detects and replaces adversarial tokens (`<|im_start|>`, `<|im_end|>`, `[INST]`, `<<SYS>>`, `<system>`, `ignore all previous instructions`) with `[REDACTED_PROMPT_CONTROL]`.
   - Escapes XML characters (`&`, `<`, `>`, `"`, `'`).
   - Encapsulates all student data inside `<user_context_data source="local_core_sqlite" permission="read_only">`.
   - Injects explicit system-level pre-emption rules commanding the LLM that user data cannot override system roles or security restrictions.
5. **Dual-Layer Multilingual Directives:** Natural conversational code-switching rules for Tanglish/Hinglish (conversational Tamil in Latin script, technical concepts in English, empathetic pedagogical particles).
6. **Cloud Consent Gating:** Checks permissions; enforces strict local-only execution if cloud sync is disabled.
7. **Token Budgeting:** Token estimation (~4 chars/token) enforcing a strict budget $< 800$ tokens for sub-second offline inference on local hardware.
8. **Structured Prompt Compilation:** Outputs `AssembledPromptResult` with metadata, timestamps, and warning telemetry.

#### 3.3 Living User Profile Engine (Zero-Silent-Mutation Guarantee)
- **Heuristic Milestone Detection:** Detects when completed tasks achieve a primary goal and generates a celebration suggestion.
- **Routine Drift Detection:** Detects when study sessions consistently occur outside scheduled hours (e.g. repeated 05:00 AM sessions before 06:00 AM wake-up) and proposes schedule adjustments.
- **Habit Streak Recognition:** Celebrates 7-day, 21-day, and 30-day consistency milestones.
- **Zero-Silent-Mutation Rule:** If user rejects or dismisses a suggestion, `user_profile` is left completely untouched. SQLite mutations occur solely when the student clicks `Accept & Update Profile`.

#### 3.4 Sovereign Memory Inspector & Export
- Accessible anytime via `Ctrl + M` or the Start Menu.
- Full CRUD operations on Level 3 epistemic memories (search by title/text, tag filtering, add, update, delete, purge).
- Instant export to **JSON** (for structured data portability) and **Markdown** (for human reading).

---

### 4. Verification & Test Results

```
PASS: 8/8 Identity Engine Unit Tests (packages/identity-engine/test/identityEngine.test.ts)
  ✔ Test 1: Prompt-Injection Sanitization & Boundary Protection
  ✔ Test 2: Dual-Layer Multilingual & Tanglish Context Assembly (747 tokens < 800)
  ✔ Test 3: Personalized Live Greeting Generation
  ✔ Test 4: Goal Milestone Evolution Detection
  ✔ Test 5: Zero-Silent-Mutation Guarantee
  ✔ Test 6: Routine Drift Detection
  ✔ Test 7: Habit Streak Milestone Detection
  ✔ Test 8: Memory Inspector CRUD and Export Suite

PASS: All 6 Workspace Monorepo Projects typecheck cleanly (tsc --noEmit)
PASS: Production Desktop Bundle built in 2.76s (vite build)
PASS: 15/15 Rust Kernel Unit Tests pass (cargo test --bin mindset-desktop)
```

---

### 5. Architectural Compliance Checklist

| Requirement | Status | Implementation Details |
| :--- | :---: | :--- |
| Zero Python / Zero ChromaDB | ✅ Compliant | Pure Rust backend + TypeScript/React frontend; zero Python runtimes. |
| OS Shell RAM Overhead < 120 MB | ✅ Compliant | Bundled frontend footprint is 449 kB JS; Kernel memory watchdog enforces < 120 MB RSS. |
| 6-Screen Progressive Onboarding | ✅ Compliant | [`OnboardingWizard.tsx`](file:///c:/MINDSETOS/apps/mindset-desktop/src/pages/onboarding/OnboardingWizard.tsx) with 220ms transitions and instant SQLite persistence. |
| Dual-Layer Multilingual Support | ✅ Compliant | OS UI (`i18n-locales`) independent from AI speech style (Tanglish, Pure Tamil, Hindi, English). |
| 3-Tier Memory Architecture | ✅ Compliant | Level 1 Profile, Level 2 Volatile Context, Level 3 Epistemic Vectors with full CRUD inspector. |
| Prompt-Injection Guardrails | ✅ Compliant | `<user_context_data>` XML sandbox, control token redaction, and system pre-emption rules. |
| Zero Silent Mutations | ✅ Compliant | `LivingProfileEngine` requires explicit user confirmation before any `user_profile` update. |
