//! Storage Subsystem for MindSet OS
//!
//! Provides SQLite persistence for chat threads, messages,
//! student profile, user memories, and daily study capsules.

pub mod db;

pub use db::{
    AiActivityRecord, AiProviderConfigRecord, CapsuleRecord, ChatMessageRecord, ChatThread,
    StorageManager, UserMemory, UserProfile, WidgetLayoutRecord,
};
