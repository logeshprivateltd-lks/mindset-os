# MINDSET OS — SPECIFICATION 04: LAYA AI ROUTER, 5 AGENTS, RAG TUTOR & FLOATING ORB
**Company:** LKS PVT LTD  
**Module:** `packages/laya-router/`, `src-tauri/src/ai/`, and Study / RAG Systems  
**Author:** Lead AI Systems Architect  
**Status:** Approved & Implemented  

---

## 1. Executive Summary & RAM Strategy

MindSet OS provides an offline-first, private artificial intelligence suite specifically engineered for students and scholars operating on everyday hardware (8 GB to 16 GB unified RAM). 

### 1.1 Non-Negotiable Single-Model RAM Discipline
Conventional AI desktops run multiple language models simultaneously (or load heavy Python frameworks like PyTorch and ChromaDB), immediately consuming 8–14 GB of RAM and causing intense thermal throttling.

MindSet OS enforces **Single-Model RAM Discipline**:
1. **`OLLAMA_MAX_LOADED_MODELS=1`:** Exactly **ONE** local LLM is resident in RAM at any given moment.
2. **`OLLAMA_FLASH_ATTENTION=1`:** Enables hardware-accelerated memory-efficient flash attention across CPU/GPU cores.
3. **`OLLAMA_KV_CACHE_TYPE=q8_0`:** Compresses key-value attention cache by 50% without quality degradation, saving ~1.2 GB of RAM during long 4096-token contexts.
4. **`keep_alive: "15m"`:** Automatically unloads the model from RAM after 15 minutes of inactivity to restore baseline OS memory to strictly $< 120\text{ MB}$.
5. **Zero Python / Zero PyTorch / Zero ChromaDB:** The inference client, prompt sanitization, vector retrieval, and token streaming run 100% inside native Rust (`tokio` async HTTP/SSE client) and lightweight TypeScript.

---

## 2. Laya AI Router Architecture (`packages/laya-router/`)

The Laya AI Router acts as the intelligent broker between the student's requests, the active persona agent, the local SQLite database, and the local inference engine (`Ollama` or embedded `llama.cpp`).

```
                    ┌──────────────────────────────┐
                    │      Student User Query      │
                    └──────────────┬───────────────┘
                                   │
                                   ▼
                    ┌──────────────────────────────┐
                    │       Laya AI Router         │
                    │   (packages/laya-router)     │
                    └──────────────┬───────────────┘
                                   │
         ┌─────────────────────────┼─────────────────────────┐
         │                         │                         │
         ▼                         ▼                         ▼
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│ Dynamic Persona │       │ PersonalContext │       │ GBNF Structured │
│ Prompt Selector │       │  Builder (L1-L3)│       │  Tool Grammar   │
└────────┬────────┘       └────────┬────────┘       └────────┬────────┘
         │                         │                         │
         └─────────────────────────┼─────────────────────────┘
                                   │
                                   ▼
                    ┌──────────────────────────────┐
                    │   Rust AI Runtime Bridge     │
                    │   (src-tauri/src/ai/)        │
                    └──────────────┬───────────────┘
                                   │  HTTP/1.1 SSE (Streaming)
                                   ▼
                    ┌──────────────────────────────┐
                    │ Ollama / llama.cpp (1 Model) │
                    │   e.g. gemma2:9b-instruct    │
                    └──────────────────────────────┘
```

### 2.1 Recommended Model Matrix
MindSet OS benchmarks and recommends local quantized models calibrated for Indic bilingual fluency and reasoning:
1. **Primary All-Rounder (12 GB+ RAM):** `gemma2:9b-instruct-q4_K_M` (~5.4 GB RAM)
   - 256k tokenizer vocabulary with unmatched contemporary Tamil (`தமிழ்`) and Tanglish fluency.
2. **Multilingual Specialist:** `aya-expanse:8b-q4_K_M` (~4.8 GB RAM)
   - Cohere's dedicated multilingual architecture supporting all 8 Indian languages with remarkable translation fidelity.
3. **Fast Fallback (8 GB RAM):** `qwen2.5:7b-instruct-q4_K_M` (~4.7 GB RAM) or `sarvam-1` (~1.6 GB RAM)
   - Extremely rapid generation (<15ms per token on CPU) for instant command parsing and schedule updates.

### 2.2 Decoupled Reasoning & GBNF Structured Actions
A common failure in conversational AI is hallucinations during database writes. Laya solves this by strictly decoupling:
1. **Conversational Stream:** Free-flowing, empathetic pedagogical dialogue formatted in the student's chosen language and style (Tanglish, Pure Tamil, Hindi, English).
2. **Structured Actions (`format: "json"`):** When creating tasks, altering routines, or recording mood check-ins, Laya enforces strict JSON schema / GBNF grammar constraints. Even when chatting in colloquial Tanglish, the emitted action payload is guaranteed valid JSON for atomic SQLite insertion.

### 2.3 The 8-Point Multilingual Benchmark Suite
Local models vary widely in Indic tokenization. Laya includes an automated 8-point test runner accessible from `/status` and `/settings`:
1. **Pure Tamil Script (`தமிழ்`):** Correct orthography, vowel markers, and natural sentence construction.
2. **Conversational Tanglish:** Natural Romanized Tamil with English STEM terminology (*"Bro, innaiku evening 6 PM-ku Physics Electrostatics revise pannalam"*).
3. **High-Yield English:** Crisp, direct pedagogical instruction without filler.
4. **Contemporary Hindi (`हिन्दी`):** Standard Devanagari script and grammatical gender agreement.
5. **STEM Code-Switching:** Verification that scientific symbols ($F = ma$, $\vec{E} = \frac{\vec{F}}{q_0}$) and English technical terms are never mangled by translation.
6. **Profile Recall:** Accurate extraction of student identity and top goals from `<user_context_data>`.
7. **Timetable JSON Generation:** 100% syntactically valid JSON conforming to the `RoutineSchedule` schema.
8. **Balance AI Empathy & Breaks:** Recognizing study fatigue and generating caring, actionable rest suggestions.

---

## 3. Five Dynamic Persona Agents Over One Model (`/mind`)

Rather than loading five distinct models into memory, Laya switches persona prompts and tool authorization boundaries dynamically over the single loaded model:

### 3.1 `StudyAgent` (The Socratic Subject Tutor)
- **Role:** Conceptual deconstruction, step-by-step formula derivations, and textbook RAG explanations.
- **Pedagogical Directives:** Never provides a bare numerical answer without explaining the governing theorem; uses Socratic questions to prompt the student's deduction.
- **Tools:** `search_rag_textbook(query, top_k)`, `get_formula_derivation(topic)`, `create_flashcard(term, definition)`.

### 3.2 `PlannerAgent` (The Timetable & Strategy Architect)
- **Role:** Generates and recalibrates daily and weekly study blocks around fixed school/college hours (`08:30–16:00`), sleep routines (`22:30`), and exam target dates.
- **Pedagogical Directives:** Respects cognitive limits; never packs 8 consecutive hours of study without breaks.
- **Tools:** `update_routine_schedule(routine_json)`, `schedule_focus_slot(start, end, subject)`, `rebalance_daily_plan()`.

### 3.3 `HabitAgent` (The Routine & Consistency Guardian)
- **Role:** Tracks daily habits (morning focus, hydration, evening gratitude reflections) with positive reinforcement.
- **Pedagogical Directives:** **Zero streak-shaming**. If a student misses 2 days due to illness or exam pressure, the agent focuses on compassionate restart rather than broken numbers.
- **Tools:** `record_habit_completion(habit_id)`, `create_habit(title, category)`, `get_habit_stats()`.

### 3.4 `MemoryAgent` (The Epistemic Progress Curator)
- **Role:** Extracts mastered concepts, exam breakthroughs, and journal realizations into `sqlite-vec` Level 3 memory.
- **Pedagogical Directives:** Sovereign student consent. Prompts the student before storing permanent memories: *"That was a great breakthrough on integration by parts! Would you like me to remember this in your epistemic memory?"*
- **Tools:** `add_epistemic_memory(title, details, tags)`, `query_past_breakthroughs(topic)`.

### 3.5 `CoachAgent` (The Wellbeing & MindSet Balance Mentor)
- **Role:** Emotional grounding, burnout prevention, exam anxiety alleviation, and free-time hobby balance.
- **Pedagogical Directives:** Warm, highly empathetic, grounded in cognitive behavioral coaching and mindful restoration.
- **Tools:** `log_mood_energy(mood, energy)`, `suggest_rest_activity(hobbies)`, `start_ambient_soundscape(type)`.

---

## 4. Distraction-Free File Explorer (`/files`) & Zero-RAM RAG Tutor

### 4.1 Canonical Study Filesystem Structure
MindSet OS provides an integrated file manager pinned to study roots under `~/.mindset_data/vault/`:
- 📁 `Textbooks & PDFs`: Subject textbooks and syllabus materials.
- 📁 `Notes & Cheat-Sheets`: Markdown notes, derivations, and summaries.
- 📁 `Practice Problems`: Past papers, mock tests, and problem sets.
- 📁 `My Phone Vault`: Wirelessly synchronized study photos and handwritten homework.
- 📁 `Recycle Bin`: Soft-deleted files with one-click restore.

### 4.2 Zero-RAM PDF RAG Pipeline
1. **Document Ingestion:** When a student drags a textbook PDF or clicks *"Send to StudyAgent"*:
   - Text is extracted page-by-page.
   - Text is chunked into 500-token segments with 50-token overlap, tagging each chunk with `document_id`, `page_number`, and `chunk_index`.
2. **On-Demand Vector Embedding:**
   - Calls Ollama `/api/embeddings` (using `bge-m3` or `all-minilm` quantized) for batch embedding generation.
   - Embeddings are committed to SQLite `vec_memory` using `sqlite-vec`.
   - The embedding model is immediately purged from RAM (`keep_alive: 0`).
3. **Split-Screen Document Tutor UI:**
   - Left Pane: PDF viewer with page controls, zoom, and text highlight.
   - Right Pane: AI chat tutor.
   - **Clickable `[Page X]` Citations:** Every AI statement cites exact page numbers (e.g. `[Page 42]`). Clicking the citation jumps the PDF reader directly to that page and automatically synchronizes `last_read_page`.

---

## 5. Floating AI Orb (`Alt + Space`), Screen Doubt Solver (`Alt + S`) & Voice Bridge

### 5.1 Floating AI Orb (`Alt + Space`)
- A subtle `#111827` Liquid Glass floating orb positioned in the bottom-right corner of the desktop shell.
- Dims to 20% opacity during active Focus sessions to prevent visual distraction.
- Pressing `Alt + Space` expands the orb into a sleek quick-query pill with instant Agent switching (`Study`, `Planner`, `Habit`, `Coach`).

### 5.2 Privacy-First Screen Doubt Solver (`Alt + S`)
- Triggered **exclusively on user demand** via `Alt + S` or the *"Read My Screen"* action button.
- Captures the active window or user-selected rectangle.
- Extracts mathematical formulas, diagrams, and text via lightweight local OCR / vision and immediately presents a step-by-step breakdown via `StudyAgent`.

### 5.3 Offline Voice & TTS Bridge
- Checks `HardwareCapabilities.has_mic` before activating microphone controls.
- Integrates local speech-to-text (Whisper.cpp) and text-to-speech (Piper TTS).
- Falls back silently and gracefully to text input if no microphone is plugged in.

---

## 6. Verification & Acceptance Criteria
- [x] Single-model RAM discipline enforced (`OLLAMA_MAX_LOADED_MODELS=1`, `keep_alive: "15m"`).
- [x] Streaming SSE client in Rust emits `mindset://ai-stream-chunk` token-by-token.
- [x] Model manager commands (`list_local_models`, `set_active_model`, `unload_active_model_from_ram`).
- [x] GBNF structured tool call wrapper guarantees valid JSON actions.
- [x] 8-Point Multilingual Benchmark Suite evaluates model fluency across Tamil, Tanglish, Hindi, English, and JSON generation.
- [x] Zero Python / Zero PyTorch: Entire stack runs inside Tauri 2 (Rust) + TypeScript.
