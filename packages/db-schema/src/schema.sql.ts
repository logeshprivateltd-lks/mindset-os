/**
 * Canonical SQL Migrations and Low-RAM Pragmas for MindSet OS
 * local_core.sqlite & sync_oplog.sqlite
 */

export const SQLITE_LOW_RAM_PRAGMAS = `
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = MEMORY;
PRAGMA mmap_size = 33554432;
PRAGMA cache_size = -8000;
PRAGMA foreign_keys = ON;
PRAGMA busy_timeout = 5000;
`.trim();

export const LOCAL_CORE_MIGRATION_V1 = `
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

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

CREATE TABLE IF NOT EXISTS tasks_and_goals (
    id TEXT PRIMARY KEY,
    parent_goal_id TEXT,
    title TEXT NOT NULL,
    subject_tag TEXT,
    priority INTEGER NOT NULL DEFAULT 2,
    status TEXT NOT NULL DEFAULT 'pending',
    target_score_weight REAL DEFAULT 1.0,
    scheduled_start TEXT,
    scheduled_end TEXT,
    completed_at TEXT,
    xp_reward INTEGER NOT NULL DEFAULT 10,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS habits (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'wellness',
    frequency_json TEXT NOT NULL DEFAULT '{"type":"daily"}',
    current_streak INTEGER NOT NULL DEFAULT 0,
    best_streak INTEGER NOT NULL DEFAULT 0,
    last_completed_date TEXT,
    updated_at TEXT NOT NULL
);

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

CREATE TABLE IF NOT EXISTS mood_checkins (
    id TEXT PRIMARY KEY,
    mood TEXT NOT NULL,
    energy_tier TEXT NOT NULL,
    context_note TEXT,
    active_activity TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS focus_sessions (
    id TEXT PRIMARY KEY,
    profile_mode TEXT NOT NULL,
    planned_minutes INTEGER NOT NULL,
    actual_minutes INTEGER NOT NULL DEFAULT 0,
    task_id TEXT,
    interruption_count INTEGER NOT NULL DEFAULT 0,
    completed INTEGER NOT NULL DEFAULT 0,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    FOREIGN KEY(task_id) REFERENCES tasks_and_goals(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS timeline_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    title TEXT NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rag_documents (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    title TEXT NOT NULL,
    subject_tag TEXT,
    total_pages INTEGER NOT NULL DEFAULT 1,
    last_read_page INTEGER NOT NULL DEFAULT 1,
    indexed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rag_chunks (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    page_number INTEGER NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY(document_id) REFERENCES rag_documents(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS vec_memory_fallback (
    id TEXT PRIMARY KEY,
    embedding_json TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    created_at TEXT NOT NULL
);
`.trim();

export const SYNC_OPLOG_MIGRATION_V1 = `
CREATE TABLE IF NOT EXISTS oplog_entries (
    op_id TEXT PRIMARY KEY,
    hlc_timestamp TEXT NOT NULL,
    device_id TEXT NOT NULL,
    table_name TEXT NOT NULL,
    row_id TEXT NOT NULL,
    operation TEXT CHECK(operation IN ('INSERT', 'UPDATE', 'DELETE')) NOT NULL,
    payload_json TEXT NOT NULL,
    synced_devices_json TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_oplog_hlc ON oplog_entries(hlc_timestamp);
CREATE INDEX IF NOT EXISTS idx_oplog_table_row ON oplog_entries(table_name, row_id);

CREATE TABLE IF NOT EXISTS trusted_devices (
    device_id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    device_type TEXT CHECK(device_type IN ('desktop', 'mobile')) NOT NULL,
    public_key_ed25519 TEXT NOT NULL,
    permissions_json TEXT NOT NULL DEFAULT '{"can_sync": true, "can_push": true}',
    last_seen_at TEXT NOT NULL,
    is_revoked INTEGER NOT NULL DEFAULT 0
);
`.trim();

