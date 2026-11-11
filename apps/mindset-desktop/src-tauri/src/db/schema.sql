-- MindSet OS Master SQLite DDL Schema (v1)
-- Low-RAM Configuration: WAL mode, 32MB mmap, 8MB cache
-- Copyright (c) 2026 LKS PVT LTD. All rights reserved.

-- 1. Schema Version Tracking
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

-- 2. Living Student Profile
CREATE TABLE IF NOT EXISTS user_profile (
    user_id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'student',
    interface_lang TEXT NOT NULL DEFAULT 'en',
    ai_lang TEXT NOT NULL DEFAULT 'ta',
    conversation_style TEXT NOT NULL DEFAULT 'tanglish',
    goals_json TEXT NOT NULL DEFAULT '[]',
    focus_areas_json TEXT NOT NULL DEFAULT '[]',
    routine_json TEXT NOT NULL DEFAULT '{}',
    preferences_json TEXT NOT NULL DEFAULT '{}',
    interests_json TEXT NOT NULL DEFAULT '[]',
    permissions_json TEXT NOT NULL DEFAULT '{}',
    updated_at TEXT NOT NULL
);

-- 3. Cryptographic Auth Credentials
CREATE TABLE IF NOT EXISTS auth_credentials (
    user_id TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    pin_hash TEXT,
    face_embedding_blob BLOB,
    fingerprint_enrolled INTEGER NOT NULL DEFAULT 0,
    auto_lock_minutes INTEGER NOT NULL DEFAULT 5,
    failed_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TEXT,
    FOREIGN KEY(user_id) REFERENCES user_profile(user_id) ON DELETE CASCADE
);

-- 4. Tasks, Objectives & Priority Goals
CREATE TABLE IF NOT EXISTS tasks_and_goals (
    id TEXT PRIMARY KEY,
    parent_goal_id TEXT,
    title TEXT NOT NULL,
    subject_tag TEXT,
    priority INTEGER NOT NULL DEFAULT 2, -- 1=Urgent/Crucial, 2=Medium, 3=Low
    status TEXT NOT NULL DEFAULT 'pending', -- pending, in_progress, completed, archived
    target_score_weight REAL DEFAULT 1.0,
    scheduled_start TEXT,
    scheduled_end TEXT,
    completed_at TEXT,
    xp_reward INTEGER NOT NULL DEFAULT 10,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks_and_goals(status);
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks_and_goals(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_scheduled ON tasks_and_goals(scheduled_start, scheduled_end);

-- 5. Habit Tracking & Streaks
CREATE TABLE IF NOT EXISTS habits (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'wellness', -- wellness, study, mindset, routine
    frequency_json TEXT NOT NULL DEFAULT '{"type":"daily"}',
    current_streak INTEGER NOT NULL DEFAULT 0,
    best_streak INTEGER NOT NULL DEFAULT 0,
    last_completed_date TEXT,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_habits_category ON habits(category);

-- 6. Encrypted Reflective Journaling
CREATE TABLE IF NOT EXISTS journal_entries (
    id TEXT PRIMARY KEY,
    title TEXT,
    content_md TEXT NOT NULL,
    gratitude_items_json TEXT NOT NULL DEFAULT '[]',
    mood_tag TEXT,
    energy_level TEXT,
    is_encrypted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_journal_created ON journal_entries(created_at);

-- 7. Real-time Mood & Energy Check-ins
CREATE TABLE IF NOT EXISTS mood_checkins (
    id TEXT PRIMARY KEY,
    mood TEXT NOT NULL,
    energy_tier TEXT NOT NULL, -- Low, Medium, High, Hyper
    context_note TEXT,
    active_activity TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mood_created ON mood_checkins(created_at);

-- 8. Deep Work & Focus Sessions
CREATE TABLE IF NOT EXISTS focus_sessions (
    id TEXT PRIMARY KEY,
    profile_mode TEXT NOT NULL, -- ExamPrep, Coding, Reading, DeepWork
    planned_minutes INTEGER NOT NULL,
    actual_minutes INTEGER NOT NULL DEFAULT 0,
    task_id TEXT,
    interruption_count INTEGER NOT NULL DEFAULT 0,
    completed INTEGER NOT NULL DEFAULT 0,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    FOREIGN KEY(task_id) REFERENCES tasks_and_goals(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_focus_started ON focus_sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_focus_task ON focus_sessions(task_id);

-- 9. Unified Chronological Timeline Events
CREATE TABLE IF NOT EXISTS timeline_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL, -- focus_session, task_completion, mood_logged, habit_tick, system_boot, lock_screen
    title TEXT NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_timeline_created ON timeline_events(created_at);
CREATE INDEX IF NOT EXISTS idx_timeline_type ON timeline_events(event_type);

-- 10. Ingested RAG Knowledge Documents
CREATE TABLE IF NOT EXISTS rag_documents (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    title TEXT NOT NULL,
    subject_tag TEXT,
    total_pages INTEGER NOT NULL DEFAULT 1,
    last_read_page INTEGER NOT NULL DEFAULT 1,
    indexed_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_rag_docs_subject ON rag_documents(subject_tag);

-- 11. Chunked Text Vectors
CREATE TABLE IF NOT EXISTS rag_chunks (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    page_number INTEGER NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY(document_id) REFERENCES rag_documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_rag_chunks_doc ON rag_chunks(document_id);

-- 12. Vector Embeddings (float[384] compatible with all-MiniLM-L6-v2)
CREATE TABLE IF NOT EXISTS vec_memory_fallback (
    id TEXT PRIMARY KEY,
    embedding_json TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_vec_source ON vec_memory_fallback(source_type, source_id);
