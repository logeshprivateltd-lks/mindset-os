use std::collections::HashMap;
use crate::analyzers::WorkspaceContext;
use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ModuleAuditResult, PrioritizedAction,
};

/// Module 6: Maindy AI, Agentic OS Control, Permissions & Voice TTS Audit
pub fn audit(ctx: &WorkspaceContext) -> ModuleAuditResult {
    let mut entries = Vec::new();
    let mut actions = Vec::new();
    let mut metrics = HashMap::new();
    let mut score: u32 = 100;

    let provider_rs = ctx.read_file("bins/mindset-os/src/ai/provider.rs").unwrap_or_default();
    let agent_rs = ctx.read_file("bins/mindset-os/src/ai/agent.rs").unwrap_or_default();
    let agent_tools_rs = ctx.read_file("crates/mindset-intelligence/src/agent_tools.rs").unwrap_or_default();
    let prompt_builder_rs = ctx.read_file("bins/mindset-os/src/ai/prompt_builder.rs").unwrap_or_default();
    let tts_rs = ctx.read_file("bins/mindset-os/src/ai/tts.rs").unwrap_or_default();
    let perm_modal_rs = ctx.read_file("bins/mindset-os/src/views/permission_modal.rs").unwrap_or_default();

    // 1. Hybrid Provider & Streaming Integrity
    let has_byok = provider_rs.contains("gemini") || provider_rs.contains("groq") || provider_rs.contains("openai");
    let has_ollama = provider_rs.contains("127.0.0.1:11434");
    let has_offline_fallback = provider_rs.contains("OfflineStudyHelper") || ctx.has_file("bins/mindset-os/src/ai/offline_helper.rs");
    let has_chunked_streaming = provider_rs.contains("bytes_stream") || provider_rs.contains("StreamExt");
    let has_fragile_raw_tcp = provider_rs.contains("TcpStream::connect") && provider_rs.contains("read_line");

    let prov_line = ctx
        .find_first_occurrence("bins/mindset-os/src/ai/provider.rs", "127.0.0.1:11434")
        .map(|(l, _)| l)
        .unwrap_or(36);

    if has_byok && has_ollama && has_offline_fallback && has_chunked_streaming && !has_fragile_raw_tcp {
        entries.push(CapabilityMatrixEntry {
            feature: "Hybrid BYOK / Ollama / Zero-Failure Offline Engine".into(),
            category: "AI".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/ai/provider.rs:{}", prov_line),
            notes: "Hybrid BYOK API key slots + local Ollama (127.0.0.1:11434) + resilient offline study helper with chunked HTTP streaming".into(),
        });
    } else {
        score = score.saturating_sub(20);
        let note = if has_fragile_raw_tcp {
            "Detected fragile raw TCP line reading that breaks HTTP chunked streaming"
        } else {
            "Provider engine missing BYOK slots, Ollama fallback, or offline helper"
        };

        entries.push(CapabilityMatrixEntry {
            feature: "Hybrid BYOK / Ollama / Zero-Failure Offline Engine".into(),
            category: "AI".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/ai/provider.rs:{}", prov_line),
            notes: note.into(),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Harden AI provider streaming and offline fallback".into(),
            location: format!("bins/mindset-os/src/ai/provider.rs:{}", prov_line),
            finding: "AI provider stream must use robust HTTP chunked parsing rather than raw TCP lines.".into(),
            remediation: "Use reqwest::Response::bytes_stream with SSE framing and ensure OfflineStudyHelper activates on disconnect.".into(),
        });
    }

    // 2. Profile Context & User Memory Injection
    let has_profile_context = prompt_builder_rs.contains("profile.target_exam_display")
        || prompt_builder_rs.contains("profile.name")
        || prompt_builder_rs.contains("weak_topics");
    let has_memory_injection = prompt_builder_rs.contains("user_memories") || prompt_builder_rs.contains("memories");

    let prompt_line = ctx
        .find_first_occurrence("bins/mindset-os/src/ai/prompt_builder.rs", "pub fn build_personalized_system_prompt")
        .map(|(l, _)| l)
        .unwrap_or(11);

    if has_profile_context && has_memory_injection {
        entries.push(CapabilityMatrixEntry {
            feature: "Student Persona Context & Long-Term Memory Injection".into(),
            category: "AI".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/ai/prompt_builder.rs:{}", prompt_line),
            notes: "Automatically injects student exam profile, weak topics, and SQLite user_memories into Maindy AI system prompt".into(),
        });
    } else {
        score = score.saturating_sub(15);
        entries.push(CapabilityMatrixEntry {
            feature: "Student Persona Context & Long-Term Memory Injection".into(),
            category: "AI".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/ai/prompt_builder.rs:{}", prompt_line),
            notes: "Maindy AI prompt builder does not dynamically inject student profile or SQLite memories".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::Medium,
            title: "Inject student memories and exam target into system prompt".into(),
            location: format!("bins/mindset-os/src/ai/prompt_builder.rs:{}", prompt_line),
            finding: "System prompt lacks personalized context from SQLite user_profile and user_memories.".into(),
            remediation: "Ensure build_personalized_system_prompt formats target exam, days left, and top memories.".into(),
        });
    }

    // 3. Agentic OS Capabilities & Strict Permission Gate Audit
    let has_tool_actions = agent_tools_rs.contains("CreateAndShowCapsule")
        && agent_tools_rs.contains("ListFolder")
        && agent_tools_rs.contains("ReadDocument")
        && agent_tools_rs.contains("OpenApp")
        && agent_tools_rs.contains("ControlSystem");

    let has_permission_check = agent_tools_rs.contains("pub fn requires_permission")
        && agent_rs.contains("AgentPermissionDecision");
    let has_permission_modal = perm_modal_rs.contains("MAINdy AI PERMISSION REQUEST")
        || perm_modal_rs.contains("SECURITY INTERLOCK");

    // Strictly audit for unguarded filesystem access
    // Check if agent.rs safe_read_document has path authorization checks
    let has_safe_read_cap = agent_rs.contains("MAX_DOCUMENT_READ_BYTES") || agent_rs.contains("16 * 1024");
    let has_vault_path_resolution = agent_rs.contains("resolve_vault_path");

    let perm_line = ctx
        .find_first_occurrence("crates/mindset-intelligence/src/agent_tools.rs", "pub fn requires_permission")
        .map(|(l, _)| l)
        .unwrap_or(46);

    if has_tool_actions && has_permission_check && has_permission_modal && has_safe_read_cap && has_vault_path_resolution {
        entries.push(CapabilityMatrixEntry {
            feature: "Strict Permission-Gated OS Tool Actions".into(),
            category: "AI".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("crates/mindset-intelligence/src/agent_tools.rs:{}", perm_line),
            notes: "Zero unguarded filesystem access: ListFolder, ReadDocument, and ControlSystem strictly gated behind Allow Once / Allow Session confirmation".into(),
        });
    } else {
        score = score.saturating_sub(30);
        entries.push(CapabilityMatrixEntry {
            feature: "Strict Permission-Gated OS Tool Actions".into(),
            category: "AI".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("crates/mindset-intelligence/src/agent_tools.rs:{}", perm_line),
            notes: "Incomplete permission gate or missing authorization checks for file access and OS settings".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::Critical,
            title: "Enforce strict permission modal gate before any file or OS mutation".into(),
            location: format!("crates/mindset-intelligence/src/agent_tools.rs:{}", perm_line),
            finding: "AI actions must never inspect folders, read documents, or change system settings silently.".into(),
            remediation: "Ensure requires_permission returns true for ListFolder/ReadDocument/ControlSystem, and dispatch permission sheet modal.".into(),
        });
    }

    // 4. Voice TTS & Thanglish Phonetics Pipeline
    let has_clean_speech = tts_rs.contains("strip_emojis_and_markdown");
    let has_thanglish = tts_rs.contains("thanglish_to_tamil_phonetic")
        || ctx.read_file("bins/mindset-os/src/ai/persona.rs").map(|s| s.contains("thanglish_to_tamil_phonetic")).unwrap_or(false);
    let has_create_no_window = tts_rs.contains("CREATE_NO_WINDOW");
    let has_linux_tts_fallback = tts_rs.contains("piper") || tts_rs.contains("espeak-ng") || tts_rs.contains("spd-say");

    let tts_line = ctx
        .find_first_occurrence("bins/mindset-os/src/ai/tts.rs", "CREATE_NO_WINDOW")
        .map(|(l, _)| l)
        .unwrap_or(16);

    if has_clean_speech && has_thanglish && has_create_no_window && has_linux_tts_fallback {
        entries.push(CapabilityMatrixEntry {
            feature: "Bilingual Voice TTS & Thanglish Phonetics".into(),
            category: "AI".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("bins/mindset-os/src/ai/tts.rs:{}", tts_line),
            notes: "Dual-text pipeline (DISPLAY vs SPEECH), emoji stripping, Windows CREATE_NO_WINDOW black window suppression, and Linux piper/espeak-ng fallback".into(),
        });
    } else {
        score = score.saturating_sub(15);
        let mut missing_tts = Vec::new();
        if !has_clean_speech { missing_tts.push("strip_emojis_and_markdown"); }
        if !has_thanglish { missing_tts.push("thanglish phonetics"); }
        if !has_create_no_window { missing_tts.push("CREATE_NO_WINDOW"); }
        if !has_linux_tts_fallback { missing_tts.push("Linux piper/espeak-ng fallback"); }

        entries.push(CapabilityMatrixEntry {
            feature: "Bilingual Voice TTS & Thanglish Phonetics".into(),
            category: "AI".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("bins/mindset-os/src/ai/tts.rs:{}", tts_line),
            notes: format!("TTS pipeline incomplete: missing {}", missing_tts.join(", ")),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Complete bilingual speech pipeline and Windows console suppression".into(),
            location: format!("bins/mindset-os/src/ai/tts.rs:{}", tts_line),
            finding: format!("Voice TTS engine is missing: {}", missing_tts.join(", ")),
            remediation: "Ensure emojis are stripped before synthesis, CREATE_NO_WINDOW flag is passed on Windows, and piper/espeak-ng is invoked on Linux.".into(),
        });
    }

    metrics.insert("ai_agent_tts_score".into(), score.to_string());
    let summary = format!(
        "Audited BYOK/Ollama hybrid streaming, memory prompt injection, permission-gated OS actions, and bilingual TTS. AI score: {}/100.",
        score
    );

    ModuleAuditResult {
        module_id: "ai_agent_tts".into(),
        module_name: "Module 6: Maindy AI, Agentic OS Control, Permissions & Voice TTS Audit".into(),
        score,
        summary,
        entries,
        actions,
        metrics,
    }
}
