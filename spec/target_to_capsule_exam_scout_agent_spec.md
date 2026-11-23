# MindSet OS — Target-to-Capsule, Exam Scout Agent & Proof-of-Study Specification

**Module Family:** `MINDSET INTELLIGENCE` + Core Execution Engine  
**Target Stack:** 100% Pure Rust · `iced 0.14.0` · `tokio` · Local Ollama Bridge (`Maindy AI`) · Local Persistence (`serde` / JSON or SQLite)  
**Design Principle:** *No fake checkboxes. No blind studying. Every exam goal turns into a verified daily capsule.*

---

## 1. System Overview

This specification defines four interconnected engines inside **MindSet OS**:

1. **The Exam Scout AI Agent:** Researches the student's chosen exam (Board Exams, NEET, JEE, UPSC, TNPSC, CA, etc.), builds a structured **Exam Intelligence Report**, and maps every syllabus unit by real exam mark weightage and past-paper frequency.
2. **The Target-to-Capsule Breakdown Engine:** Converts a target score and exam date (e.g., **590 / 600 in March**) into daily, mark-valued **Today's Capsules** with automatic missed-day recalibration.
3. **The Proof-of-Study Verification Gate & Point Scoring System:** Enforces a strict rule—**a capsule cannot be marked finished until the student proves they actually studied** through a dynamically generated multi-stage test that awards points based on answer accuracy, reasoning, and integrity.
4. **Engaging & Funny OS Micro-Activities:** Built-in, non-distracting personality layers (Boss Battles, Excuse Roasts, 2-Minute Break Arcade, Future-You Postcards, and Silent Squad Reactions) that make students genuinely look forward to opening MindSet OS every day.

---

## 2. The Exam Scout AI Agent (Research & Blueprint Pipeline)

When a student selects an exam and target score, the **Exam Scout Agent** runs a structured 4-stage pipeline and stores the output locally so the OS works 100% offline afterward.

### Stage 1: Official Exam Structure & Pattern Retrieval
The agent collects (via web research connector when online, or curated local exam packs + Ollama synthesis when offline):
* **Exam Metadata:** Exam name, total marks, exam date, duration, section breakdown (e.g., 1-mark MCQs, 2-mark, 3-mark, 5-mark descriptive, or $+4 / -1$ negative marking).
* **Syllabus Verification:** Official current-year units, chapters, sub-topics, and explicitly deleted portions so zero time is wasted on outdated content.
* **Choice & Compulsory Rules:** Internal choice structure versus mandatory high-risk sections.

### Stage 2: 10-Year PYQ Forensics & ROI Classification
The agent scores every chapter by **Exam Weightage** and **Effort/Difficulty**, sorting the syllabus into four strategic quadrants:

| Quadrant | Definition | Scheduling Priority |
| :--- | :--- | :--- |
| **Goldmines** | High mark weight + predictable / moderate difficulty | **Priority 1** — Mastered early to build rapid score confidence. |
| **Heavyweights** | High mark weight + conceptually deep / tough | **Priority 2** — Split into smaller focused capsules with extra revisions. |
| **Quick Wins** | Lower mark weight + fast to complete | **Priority 3** — Paired with Heavyweights or scheduled on low-energy days. |
| **Traps** | Very low mark weight + huge time cost | **Priority 4** — Scheduled only after the target mark threshold is secured. |

### Stage 3: The Exam Intelligence Report (UI Deliverable)
Before Day 1 starts, MindSet OS presents a crisp, visual **Exam Intelligence Report** showing:
* Exam pattern & marking rules at a glance.
* Top 10 highest-ROI chapters (*"These 10 chapters account for 42% of your 590 target"*).
* Subject-wise weightage bars and expected mark distribution.
* A 15-minute optional **Baseline Diagnostic Scan** (or self-rated chapter confidence) to calculate the student's starting score (e.g., `430 / 600`).

---

## 3. Target-to-Capsule Breakdown Engine

### 3.1 Score Gap & Safety Buffer Math
Given:
* $T$ = Target Score (e.g., $590$)
* $M_{\max}$ = Maximum Possible Score (e.g., $600$)
* $B$ = Current Baseline Readiness Score (e.g., $430$)
* $D_{\text{rem}}$ = Remaining Study Days before the exam date

The engine computes the required mark gain including a **Safety Buffer** $\beta$ (typically $2\%\text{–}3\%$ capped at $M_{\max}$):

$$\Delta M = \min(T + \beta, M_{\max}) - B$$

### 3.2 Four Macro Preparation Phases
The remaining calendar $D_{\text{rem}}$ is divided into four balanced phases:
1. **Phase 1 — Core Foundation (45% of days):** Cover all *Goldmines* and *Heavyweights* with concept mastery.
2. **Phase 2 — Gap Closure (25% of days):** Complete remaining units, numerical drills, and weak-area remediation.
3. **Phase 3 — Exam Simulation (20% of days):** Full timed mock tests, writing speed, and negative-mark elimination.
4. **Phase 4 — Lock-In & Taper (10% of days):** Spaced revision of error logs, formula sheets, and calm exam readiness.

### 3.3 Anatomy of "Today's Capsule"
Each day generates exactly **one unified Daily Capsule** tailored to the student's weekday/weekend hour budget, displaying a clear promise:

> **"Complete Today's Capsule (+3.5 Marks Value) $\rightarrow$ Stay 100% On Track for 590 / 600 in March."**

A capsule contains **4 mandatory segments**:
1. **Heavy Lift (45% time):** Core concept, derivation, or textbook pages to study inside the Study Reader.
2. **Active Practice (25% time):** Past-year questions, numericals, or structured writing drills.
3. **Memory Anchor (15% time):** Spaced-repetition refresher of chapters completed 3, 7, or 21 days ago (triggered automatically as chapter freshness decays).
4. **Verification Boss Gate (15% time):** The mandatory Proof-of-Study test that unlocks capsule completion.

### 3.4 Zero-Guilt Auto-Healing Recalibration
If a student misses a day (illness, family event, school overload), the engine **never** dumps an overwhelming overdue pile:
* Reserved **Buffer Days** (10% of calendar built into Phases 1–3) absorb missed capsules first.
* If buffer days are exhausted, remaining low-ROI *Trap* filler is trimmed before increasing daily Heavy Lift loads by more than $10\%$.
* The **Projected Exam Score** updates honestly and transparently.

---

## 4. Proof-of-Study Verification Gate & Point Scoring System (CRITICAL)

### 4.1 Strict Completion Rule (State Machine Lock)
**There is NO manual "Mark Capsule Complete" checkbox anywhere in MindSet OS.**

A capsule must pass through a strict compile-time state machine:

```
Locked -> Active -> Studying -> ReadyForBoss -> VerificationInProgress -> Completed (Gold / Platinum)
                                                      |
                                                      v (Score < 70)
                                              NeedsRemediation -> Re-Challenge -> Completed
```

A capsule transitions to `Completed` **if and only if**:
1. Minimum genuine study engagement telemetry is met (anti-AFK check), **AND**
2. The student scores **$\ge 70 / 100$ Verification Points** in the Proof-of-Study Boss Test.

### 4.2 Passive Study Authenticity Telemetry (Pre-Gate Check)
Before the Verification Boss unlocks (or as an integrity modifier):
* Tracks **Active Study Seconds** inside MindSet OS (keyboard, scroll, highlight, note, or tutor interaction heartbeat; pauses automatically if idle for $> 90\text{ seconds}$).
* If the student tries to click *"Finish Capsule"* after only 2 minutes of opening a 45-minute topic, the OS flags a **Speedrun Alert**: *"Finished in 2 minutes? Bold move! Pass the Hardcore Proof-of-Study Boss with 85%+ to prove you already mastered this."* (Allows genuinely smart students who already know a chapter to test out, while stopping fakers!)

### 4.3 Dynamic 3-Stage Proof-of-Study Test (100 Points Total)
Generated dynamically by `Maindy AI` (local Ollama bridge) from the exact sub-topics of Today's Capsule—freshly randomized on every attempt so answers cannot be memorized:

| Stage | Format | Max Points | What It Proves |
| :--- | :--- | :---: | :--- |
| **Stage 1: Rapid Recall** | 4–5 randomized MCQs / Formula Fill-ins (with per-question timer) | **30 pts** | Core facts, formulas, units, and definitions are memorized. |
| **Stage 2: Applied Problem / Step Check** | 2–3 numericals, derivation step-ordering, or PYQ mini-problems | **40 pts** | Student can actually *apply* the concept to exam-style questions. |
| **Stage 3: Feynman Explain-It-Back** | 1 short open response (2–4 sentences in the student's own words) | **30 pts** | Deep conceptual understanding—graded by local AI against 3–4 rubric criteria (key principles, cause-effect logic, zero gibberish/copy-paste). |

### 4.4 Point Grading Formula & Outcomes

Let $P_{\text{recall}} \in [0, 30]$, $P_{\text{apply}} \in [0, 40]$, and $P_{\text{feynman}} \in [0, 30]$.
Total Verification Score:

$$P_{\text{total}} = P_{\text{recall}} + P_{\text{apply}} + P_{\text{feynman}}$$

*(Speedrun Test-Out Mode requires $P_{\text{total}} \ge 85$; Standard Study Mode requires $P_{\text{total}} \ge 70$.)*

| Verification Score | Verdict | Capsule State | Exam Score Credit & OS Rewards |
| :--- | :--- | :--- | :--- |
| **90 – 100 pts** | **PLATINUM CLEAR (S-Rank)** | **FINISHED** | **100%** of capsule mark value added to Predicted Exam Score + **+150 XP** + Rare Cosmetic Loot Drop + Next revision in 7 days. |
| **70 – 89 pts** | **GOLD CLEAR (A-Rank)** | **FINISHED** | **85%–95%** proportional mark value added + **+100 XP** + Missed questions auto-added to Error Flashcards + Next revision in 3 days. |
| **50 – 69 pts** | **ALMOST THERE (Partial)** | **NOT FINISHED** | **0** capsule finish yet. OS unlocks a **3-Minute Targeted Micro-Fix** explaining only the missed concepts, then opens a **Targeted Re-Challenge** strictly on weak items. |
| **0 – 49 pts** | **BOSS DEFEATED YOU (Fail)** | **NOT FINISHED** | **0** capsule finish. Triggers a playful supportive roast + mandatory 5-minute guided walkthrough with Maindy AI before a freshly randomized Boss rematch unlocks. |

### 4.5 Bonus Skill Points (XP Economy — Separate from Exam Marks)
To keep students motivated without polluting their honest **Predicted Exam Score**:
* **Exam Marks (Sacred & Honest):** Only increase from verified capsules, spaced revision retention, and mock tests.
* **MindSet XP (Fun & Rewarding):** Awarded from $P_{\text{total}}$ plus bonuses:
  * *First-Try Clean Sweep:* `+25 XP`
  * *Zero-Hint Victory:* `+20 XP`
  * *Consecutive Verified Capsule Streak:* $\text{XP} \times (1 + 0.1 \times \min(\text{streak\_days}, 5))$
* **Mobile & Website Sync Payload:** Every finished capsule emits a signed, lightweight JSON achievement packet (Capsule ID, Rank, Score, Streak, Updated Predicted Score) ready to sync with the student's companion phone and official MindSet OS profile.

### 4.6 Anti-Cheat & Integrity Safeguards
* **No Answer Memorization:** Retries regenerate numbers, rephrase stems, and shuffle options.
* **Focus-Loss Detection:** If the MindSet OS window loses OS focus during an active Verification Boss test for more than 4 seconds, the current question is silently swapped for a fresh equivalent on return.
* **Fast-Click Guard:** Submitting a multi-step application question in under 2 seconds triggers an instant 1-line *"Quick check: which formula did you use?"* confirmation.
* **Gibberish / Copy-Paste Guard:** Stage 3 Feynman grading rejects overly short filler, repeated words, or verbatim textbook paste, prompting the student to explain it simply like teaching a friend.

---

## 5. Funny & Engaging OS Activities (Native Engagement Layer)

All fun activities are **earned through real effort** and strictly time-boxed at the OS level so they recharge dopamine without causing distraction spirals:

1. **Verification Boss Battles:** Each Daily Capsule ends with a themed Boss card (e.g., *"Lord Nernst — 5 Marks HP"*). Every correct answer lowers the Boss HP bar with crisp GPU animations; defeating the Boss bursts confetti particles and unlocks XP + OS cosmetics (new obsidian/nebula themes, custom glass tints, chime packs, or study companion badges).
2. **"Roast My Excuse" (MindfulGuard Bailout):** Trying to snooze Today's Capsule triggers a witty, light-hearted Tamil/English cinema-style roast from Maindy AI (*"Just 5 minutes of reels? That's how 590 turns into 490!"*). To actually unlock snooze, the user must clear a 15-second Rapid Formula Check.
3. **2-Minute Brain-Reset Arcade (Break-Only Unlock):** Unlocks *exclusively* during Pomodoro breaks and auto-blurs back to study mode at `00:00`:
   * *Formula Speed-Match*
   * *1-Minute Chess Tactical Puzzle*
   * *Periodic Table / Rapid-Fire Trivia Blitz*
4. **Future-You Time-Capsule Postcards:** On hitting 5-day, 14-day, or Phase completion streaks, the OS generates a funny mock newspaper headline from the student's future exam result day (e.g., *"BREAKING: Student Who Defeated Electrochemistry Boss Without Crying Spotted Celebrating 592/600!"*), shareable to the official MindSet OS website.
5. **Squad Ghost Mode (Zero-Chat Co-Study Bar):** Minimal status indicators in the top bar showing paired friends' active focus timers with one-click silent boosts (*Send Chai ☕*, *Fire Boost 🔥*) unlocked only when you finish your own block.

---

## 6. Pure Rust Module Blueprint (For `C:\MINDSETOS`)

Follow the existing `iced 0.14.0` Elm architecture (`State`, `Message`, `update`, `view`, `subscription`) and compile-time `ActiveOverlay` mutual exclusion:

```text
src/
├── agent/
│   ├── mod.rs
│   ├── exam_scout.rs          // Exam Blueprint researcher, PYQ ROI ranker, Report generator
│   └── ollama_grader.rs       // Dynamic question generator & Feynman Stage-3 rubric grader
├── engine/
│   ├── mod.rs
│   ├── capsule_planner.rs     // Target-to-Capsule math, 4-phase splitter, auto-healing recalibrator
│   ├── verification.rs        // Strict CapsuleState machine, 100-pt scoring, anti-cheat guards
│   ├── engagement.rs          // Boss Battles, Excuse Roasts, XP/Cosmetics, Future Postcards, 2-min Break Arcade
│   └── sync_payload.rs        // Verified achievement JSON snapshot for Mobile & Official Website sync
└── ui/
    ├── exam_report_view.rs    // Exam Intelligence Report & Goal Onboarding screen
    ├── today_capsule_view.rs  // Upgraded Today Screen with Capsule Mark Value, Progress & Boss Trigger
    └── verification_modal.rs  // ActiveOverlay::VerificationBoss & ActiveOverlay::ExcuseRoast modals
```

### Key Rust Data Contracts (Core Structs)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapsuleState {
    Locked,
    Active,
    Studying { active_seconds: u64, idle_warnings: u8 },
    ReadyForBoss { speedrun_mode: bool },
    VerificationInProgress { attempt: u8 },
    NeedsRemediation { last_score: u8, weak_topics: Vec<String>, cooldown_until_epoch: u64 },
    Completed {
        final_score: u8,          // Strictly >= 70 (or >= 85 in speedrun_mode)
        rank: CapsuleRank,        // Gold or Platinum
        marks_locked: f32,        // Added to Predicted Exam Score
        xp_earned: u32,
        verified_at_epoch: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapsuleRank {
    Gold,     // 70..=89
    Platinum, // 90..=100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationScoreBreakdown {
    pub recall_points: u8,        // 0..=30
    pub application_points: u8,   // 0..=40
    pub feynman_points: u8,       // 0..=30
    pub focus_loss_penalties: u8,
    pub total_points: u8,         // 0..=100
    pub passed: bool,             // True ONLY if threshold met
    pub feedback: String,
    pub weak_subtopics: Vec<String>,
}
```

### Mandatory Acceptance Gates
1. **Zero Fake Completion:** Unit tests MUST prove that `CapsuleState` cannot transition to `Completed` when `total_points < 70` (or `< 85` in speedrun mode).
2. **Offline Fallback:** If Ollama is unavailable or offline, `ollama_grader.rs` MUST gracefully fall back to deterministic local rubric/keyword + question-bank verification so a student is never blocked from completing their capsule.
3. **Overlay Mutual Exclusion:** All new modals (`ExamReport`, `VerificationBoss`, `ExcuseRoast`, `BreakArcade`) MUST be variants of `ActiveOverlay`.
4. **Performance & Build Health:**
   * `cargo test --workspace` — 100% pass
   * `cargo clippy -- -D warnings` — 0 warnings, 0 errors
   * `cargo fmt --check` — 100% compliant
   * Release binary (`target\release\mindset-os.exe`) strictly `< 15.0 MB`
   * Working set RAM `< 200 MB`
   * `archive/mindset-render-wgpu-v1/` remains 100% untouched.