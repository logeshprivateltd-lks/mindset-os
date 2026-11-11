# MINDSET OS — SPECIFICATION 04: LAYA AI ROUTER, 5 AGENTS, RAG TUTOR & FLOATING ORB
**Company:** LKS PVT LTD  
**RAM Strategy:** Single-Model Ollama / `llama.cpp` Orchestration (`OLLAMA_MAX_LOADED_MODELS=1`, `OLLAMA_FLASH_ATTENTION=1`, `OLLAMA_KV_CACHE_TYPE=q8_0`). Zero Python/PyTorch overhead.

## 1. Laya AI Router & Tamil-First Model Standard (`packages/laya-router/`)
1. **Supported Local Models (12 GB System Target):**
   - Primary All-Rounder: `gemma2:9b-instruct-q4_K_M` (~5.5 GB RAM) — 256k tokenizer for clean Tamil (`தமிழ்`) & Tanglish.
   - Multilingual Specialist: `aya-expanse:8b-q4_K_M` (~4.8 GB RAM).
   - Fast Fallback: `qwen2.5:7b-instruct-q4_K_M` (~4.7 GB RAM) or `sarvam-1` (~1.6 GB RAM).
2. **Decoupled Reasoning & Action (GBNF JSON Grammar):**
   - Tool calls (updating SQLite tasks, habits, or schedules) use strict JSON schema enforcement (`format: "json"`), while user-facing coaching responses stream in natural Tamil, Tanglish, English, or Hindi via `PersonalContextBuilder`.
3. **8-Point Multilingual Verification Suite:**
   - Tests candidate models on: (1) Pure Tamil script, (2) Conversational Tanglish, (3) English, (4) Hindi, (5) Mixed code-switching with English STEM terms, (6) Long-term preference recall, (7) Schedule generation JSON accuracy, and (8) Context-aware break recommendations.

## 2. Five Specialised Agents Over One Loaded Model (`/mind`)
Instead of loading 5 models into RAM, Laya switches system prompts and tool permissions dynamically:
1. **`StudyAgent`:** Subject tutoring, step-by-step concept breakdowns, formula derivations, and RAG textbook Q&A with exact page citations.
2. **`PlannerAgent`:** Generates and adjusts daily/weekly schedules around school hours (`08:00–17:00`), sleep (`22:00`), and exam targets.
3. **`HabitAgent`:** Tracks daily routines (including `06:00` and `22:00` visualization/gratitude sessions) without streak-shaming.
4. **`MemoryAgent`:** Extracts user-approved milestones and learning progress into `sqlite-vec` Level 3 memory.
5. **`CoachAgent`:** Emotional support, exam motivation, distraction recovery, and **MindSet Balance AI** free-time coaching.

## 3. Distraction-Free File Explorer (`/files`) & Zero-RAM RAG Tutor
1. **File Explorer (`/files`):**
   - Pre-configured study roots: `Subjects`, `Textbooks & PDFs`, `Notes`, `Projects`, `Downloads`, `My Phone Vault`, and `Recycle Bin`.
   - Full file operations: Create, rename, copy, move, soft-delete to Recycle Bin, restore, `.zip`/`.tar` compression/extraction, and storage partition breakdown.
   - Right-click context action: **"Send to StudyAgent / Index for AI"**.
2. **Zero-RAM PDF RAG Pipeline:**
   - Parses PDF text page-by-page using `pdfjs-dist` (frontend) or `pdf-extract` (Rust).
   - Chunks text (500 tokens, 50-token overlap) storing `document_id` and exact `page_number` in `rag_chunks`.
   - Generates 384-d embeddings on-demand via Ollama `/api/embeddings` (`all-minilm` or `bge-m3` quantized), writes to `vec_memory` (`sqlite-vec`), and immediately unloads the embedding model.
   - **Document Tutor UI:** Split-screen PDF reader (auto-suggests *Paper Studio* `#F6F1E8` theme) + AI chat where every answer cites clickable `[Page X]` badges that jump the PDF viewer to that exact page and sync `last_read_page` to `sync_oplog.sqlite`.

## 4. Floating AI Orb (`Alt + Space`), Screen Reader (`Alt + S`) & Voice Bridge
1. **Floating AI Orb (`Alt + Space`):**
   - Draggable `#111827` Liquid Glass orb in the bottom-right corner; dims to 20% opacity during active Focus sessions.
   - Expands into a compact chat bar with instant agent selector and voice/screen buttons.
2. **On-Demand Screen Doubt Solver (`Alt + S`):**
   - Strictly privacy-first: captures the active window or a user-dragged region ONLY when triggered via `Alt + S` or the **"Read My Screen"** button.
   - Extracts formulas/text via lightweight local ONNX RapidOCR / Tesseract (or optional `moondream` vision model) and routes to `StudyAgent`.
3. **Offline Voice & TTS Bridge:**
   - Integrates Faster Whisper / Whisper.cpp (STT) and Piper TTS. Automatically checks `HardwareCapabilities.has_mic` and falls back to text input if no microphone or wireless phone mic is connected.