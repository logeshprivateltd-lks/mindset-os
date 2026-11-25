//! MindSet OS — Native Intelligence Subsystem (`mindset-intelligence`)
//! Non-blocking AI interaction layer, intent routing, local RAG vault, and streaming bridge.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod agent_tools;
pub mod goal_bridge;
pub mod personalization;
pub mod prompt_guard;
pub mod rag_vault;
pub mod router;
pub mod search_engine;
pub mod stream_bridge;

pub use agent_tools::{
    action_description, agent_tools_system_prompt, extract_agent_action, requires_permission,
    AgentAction, SystemControlAction,
};
pub use personalization::{
    DailyFatigueCheckin, FatigueLevel, LeitnerBox, LeitnerCard, PersonalizationEngine,
    StudentMistakeRecord,
};
pub use prompt_guard::PromptGuard;
pub use rag_vault::{
    PdfCitationMatch, PdfDocumentChunk, RagError, RagMatch, RagVault, VaultFileInfo,
};
pub use router::{IntentRouter, QueryIntent, StudySubject, SystemAction};
pub use search_engine::{
    AcademicEncyclopedia, AcademicSearchEngine, AdBlockShield, ArXivConnector, DuckDuckGoConnector,
    GoogleSearchConnector, SearchFilter, SearchProviderType, SearchResult, WebSearchProvider,
};
pub use stream_bridge::{AiStreamBridge, AiStreamEvent, StreamHandle};
