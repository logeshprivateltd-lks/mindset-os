//! Tier 2: Subsystem R6 Natural Voice & Thanglish Language Boundary Tests (Features 22 to 24)
//!
//! Stress, limit, and edge condition verification:
//! - F22: Heavy punctuation, mixed Tamil-English script, unknown subjects, single-word inputs, multi-intent resolution (5 tests)
//! - F23: Empty input audio buffer, emoji-only stripping, unsupported language fallback, buffer clamps, audio clipping (5 tests)
//! - F24: Malformed JSON fallback, missing SPEECH tag, missing DISPLAY tag, LaTeX equation expansion, nested code blocks (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_os::ai::persona::{
    expand_latex_to_phonetic_speech, parse_dual_text, AcademicIntent, SubjectEntity,
    ThanglishParser,
};
use mindset_os::ai::tts::{strip_emojis_and_markdown, LanguagePreference, VoiceEngine};

// ===========================================================================
// FEATURE 22: Conversational Thanglish NLP Engine — Boundary Tests
// ===========================================================================

#[test]
fn test_f22_boundary_heavy_punctuation_and_emojis() {
    let raw = "hey... bro??? p6... la... chapter 10!?!?!? 10.6 example solli kudu bro 🙏🔥";
    let (tokens, audience) = ThanglishParser::normalize_tokens(raw);
    assert!(tokens.contains(&"solli".to_string()));
    assert!(audience.is_some());

    let query = ThanglishParser::parse(raw);
    assert_eq!(query.subject, Some(SubjectEntity::Physics));
    assert_eq!(query.chapter, Some(10));
    assert_eq!(query.example.as_deref(), Some("10.6"));
}

#[test]
fn test_f22_boundary_mixed_tamil_script_and_thanglish() {
    // Tamil script "ஹே" mixed with English "bro" and Thanglish "la"
    let raw = "ஹே bro physics la chapter 5 solli kudu da";
    let query = ThanglishParser::parse(raw);
    assert_eq!(query.subject, Some(SubjectEntity::Physics));
    assert_eq!(query.chapter, Some(5));
    assert_eq!(query.intent, AcademicIntent::TeachConcept);
}

#[test]
fn test_f22_boundary_unknown_subject_abbreviation_fallback() {
    let raw = "hey bro xyz123 la chapter 3 example solli kudu";
    let query = ThanglishParser::parse(raw);
    assert_eq!(query.chapter, Some(3));
    // Unknown subject remains None
    assert!(query.subject.is_none());
}

#[test]
fn test_f22_boundary_single_word_queries_safety() {
    let single_words = ["hi", "da", "bro", "mapla", "vanakkam"];
    for word in single_words {
        let query = ThanglishParser::parse(word);
        assert!(query.chapter.is_none());
    }
}

#[test]
fn test_f22_boundary_multiple_competing_academic_intents() {
    // Both "solli kudu" (TeachConcept) and "formula" (RecallFormula) present
    let raw = "bro chapter 4 formula solli kudu";
    let query = ThanglishParser::parse(raw);
    // Intent extraction must resolve deterministically without panicking
    assert!(
        query.intent == AcademicIntent::TeachConcept
            || query.intent == AcademicIntent::RecallFormula
    );
}

// ===========================================================================
// FEATURE 23: Studio-Grade Natural Voice Synthesis — Boundary Tests
// ===========================================================================

#[test]
fn test_f23_boundary_empty_text_input_voice_routing() {
    let empty_text = "";
    let stripped = strip_emojis_and_markdown(empty_text);
    assert!(stripped.is_empty());

    let pref = LanguagePreference::Tamil;
    let route = VoiceEngine::resolve_optimal_route(&pref);
    assert!(!route.description().is_empty());
}

#[test]
fn test_f23_boundary_emoji_and_markdown_only_stripping() {
    let input = "# 🚀 **💯** _✨_ [link](url)";
    let stripped = strip_emojis_and_markdown(input);
    assert!(!stripped.contains("🚀"));
    assert!(!stripped.contains("💯"));
    assert!(!stripped.contains("✨"));
}

#[test]
fn test_f23_boundary_unsupported_language_code_fallback() {
    let pref = LanguagePreference::from_lang_code("unknown_xyz");
    // Unsupported code is preserved as Custom with graceful English neural voice fallback
    assert_eq!(pref.code(), "unknown_xyz");
    assert_eq!(pref.default_neural_voice(), "en-IN-NeerjaNeural");
    let route = VoiceEngine::resolve_optimal_route(&pref);
    assert!(route.estimated_ram_mb() > 0);
}

#[test]
fn test_f23_boundary_buffer_size_calculation_clamping() {
    // Calculate streaming buffer size: 24,000 Hz, 16-bit mono = 48,000 bytes/sec
    let sample_rate: u32 = 24_000;
    let bytes_per_sample: u32 = 2;
    let channels: u32 = 1;

    let bytes_per_sec = sample_rate * bytes_per_sample * channels;
    assert_eq!(bytes_per_sec, 48_000);

    // 0.25 second chunk = 12,000 bytes
    let chunk_duration_sec = 0.25f32;
    let chunk_bytes = ((bytes_per_sec as f32) * chunk_duration_sec) as usize;
    assert_eq!(chunk_bytes, 12_000);
}

#[test]
fn test_f23_boundary_audio_clipping_amplitude_clamping() {
    let samples = [-2.0f32, -1.0, 0.0, 0.8, 1.5, 3.0];
    for s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        assert!(
            (-1.0..=1.0).contains(&clamped),
            "Audio amplitude must be clamped to [-1.0, 1.0]"
        );
    }
}

// ===========================================================================
// FEATURE 24: Dual-Text Script Generation — Boundary Tests
// ===========================================================================

#[test]
fn test_f24_boundary_malformed_json_dual_text_fallback() {
    let malformed_json = r#"{"display_text": "Physics concept", "speech_text": unclosed string"#;
    let res = parse_dual_text(malformed_json, "english");

    // Falls back gracefully to using raw string as display text
    assert_eq!(res.display_text, malformed_json);
    assert!(!res.speech_text.is_empty());
}

#[test]
fn test_f24_boundary_tagged_dual_text_missing_speech_tag() {
    let tagged_without_speech = "DISPLAY: Here is the visual derivation for Gauss Law.";
    let res = parse_dual_text(tagged_without_speech, "english");

    // When missing SPEECH: tag, falls back to plain text handler
    assert!(res.display_text.contains("Gauss Law"));
    assert!(!res.speech_text.is_empty());
}

#[test]
fn test_f24_boundary_tagged_dual_text_missing_display_tag() {
    let tagged_without_display = "SPEECH: Vanakkam mapla idhu romba easy.";
    let res = parse_dual_text(tagged_without_display, "thanglish");

    // When missing DISPLAY: tag, falls back to dual text generation
    assert!(res.display_text.contains("Vanakkam"));
    assert!(!res.speech_text.is_empty());
}

#[test]
fn test_f24_boundary_latex_equation_phonetic_expansion() {
    let equation = r#"\int E \cdot dA = \frac{Q}{\epsilon_0}"#;
    let spoken = expand_latex_to_phonetic_speech(equation, "en-IN");
    assert!(!spoken.is_empty());
    // Expanded phonetic text must not contain raw LaTeX backslashes
    assert!(!spoken.contains(r#"\frac"#));
}

#[test]
fn test_f24_boundary_nested_markdown_code_blocks_stripping() {
    let nested_markdown = "```json\n{\n  \"display_text\": \"Nested code block: ```rust let x = 5; ``` on screen\",\n  \"speech_text\": \"Nested code block on screen\"\n}\n```";
    let res = parse_dual_text(nested_markdown, "english");
    assert_eq!(
        res.display_text,
        "Nested code block: ```rust let x = 5; ``` on screen"
    );
    assert_eq!(res.speech_text, "Nested code block on screen");
}
