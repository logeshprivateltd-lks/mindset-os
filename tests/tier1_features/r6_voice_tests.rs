//! Tier 1: Subsystem R6 Natural Voice & Thanglish Interaction Tests (Features 22 to 24)
//!
//! Covers:
//! - F22: Conversational Thanglish NLP engine & intent parsing (>=5 tests)
//! - F23: Natural voice synthesis streaming buffer generation (>=5 tests)
//! - F24: Dual-text script generation (display vs speech) (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_os::ai::persona::{
    parse_dual_text, thanglish_to_tamil_phonetic, AcademicIntent, MaindyStyle, SubjectEntity,
    ThanglishParser,
};
use mindset_os::ai::tts::{
    create_wav_header, strip_emojis_and_markdown, LanguagePreference, TtsEngine, TtsRoute,
    VoiceEngine,
};

// ===========================================================================
// FEATURE 22: Conversational Thanglish NLP Engine & Intent Parsing
// ===========================================================================

#[test]
fn test_f22_thanglish_colloquial_prompt_parsing() {
    let input = "hey brow , bro p6 la chapter 10 la 10.6 example solli kudu bro";
    let parsed = ThanglishParser::parse(input);

    assert_eq!(parsed.subject, Some(SubjectEntity::Physics));
    assert_eq!(parsed.chapter, Some(10));
    assert_eq!(parsed.example.as_deref(), Some("10.6"));
    assert_eq!(parsed.section.as_deref(), Some("10.6"));
    assert_eq!(parsed.intent, AcademicIntent::TeachConcept);
    assert_eq!(parsed.target_audience.as_deref(), Some("hey"));
    assert!(parsed.confidence >= 90);

    let prompt = ThanglishParser::format_academic_prompt(&parsed);
    assert!(prompt.contains("Subject: Physics"));
    assert!(prompt.contains("Chapter: 10"));
    assert!(prompt.contains("Section/Example: 10.6"));
    assert!(prompt.contains("TEACH_CONCEPT"));
}

#[test]
fn test_f22_subject_abbreviation_normalization() {
    assert_eq!(
        ThanglishParser::parse("p6 la chapter 1").subject,
        Some(SubjectEntity::Physics)
    );
    assert_eq!(
        ThanglishParser::parse("chem la chapter 2").subject,
        Some(SubjectEntity::Chemistry)
    );
    assert_eq!(
        ThanglishParser::parse("maths la chapter 3").subject,
        Some(SubjectEntity::Mathematics)
    );
    assert_eq!(
        ThanglishParser::parse("bio la chapter 4").subject,
        Some(SubjectEntity::Biology)
    );
    assert_eq!(
        ThanglishParser::parse("cs la chapter 5").subject,
        Some(SubjectEntity::ComputerScience)
    );
}

#[test]
fn test_f22_locative_postposition_handling() {
    let (tokens_la, _) = ThanglishParser::normalize_tokens("chapter10la");
    assert_eq!(tokens_la, vec!["chapter", "10", "la"]);

    let q_dative = ThanglishParser::parse("exam-ku p6 chapter 4 solli kudu");
    assert_eq!(q_dative.dative_context.as_deref(), Some("exam"));
    assert_eq!(q_dative.subject, Some(SubjectEntity::Physics));
    assert_eq!(q_dative.chapter, Some(4));

    // "example" must never trigger false-positive "exam" dative context
    let q_example = ThanglishParser::parse("example 6 formula sollu");
    assert_eq!(q_example.example.as_deref(), Some("6"));
    assert!(q_example.dative_context.is_none());
}

#[test]
fn test_f22_academic_intent_extraction() {
    assert_eq!(
        ThanglishParser::parse("bro 10.6 example solli kudu bro").intent,
        AcademicIntent::TeachConcept
    );
    assert_eq!(
        ThanglishParser::parse("innaiku test vey bro").intent,
        AcademicIntent::GeneratePractice
    );
    assert_eq!(
        ThanglishParser::parse("formula enna mapla").intent,
        AcademicIntent::RecallFormula
    );
    assert_eq!(
        ThanglishParser::parse("gauss law doubt irukku bro").intent,
        AcademicIntent::ClarifyDoubt
    );
    assert_eq!(
        ThanglishParser::parse("numerical epdi podanum bro").intent,
        AcademicIntent::SolveStepByStep
    );
}

#[test]
fn test_f22_chapter_and_example_number_extraction() {
    let parsed = ThanglishParser::parse("p6 la ch10 la 10.6 example solli kudu bro");
    assert_eq!(parsed.subject, Some(SubjectEntity::Physics));
    assert_eq!(parsed.chapter, Some(10));
    assert_eq!(parsed.example.as_deref(), Some("10.6"));
    assert_eq!(parsed.section.as_deref(), Some("10.6"));

    let parsed2 = ThanglishParser::parse("chem la 3rd chapter example 3.4 epdi podanum");
    assert_eq!(parsed2.subject, Some(SubjectEntity::Chemistry));
    assert_eq!(parsed2.chapter, Some(3));
    assert_eq!(parsed2.example.as_deref(), Some("3.4"));
    assert_eq!(parsed2.intent, AcademicIntent::SolveStepByStep);
}

// ===========================================================================
// FEATURE 23: Natural Voice Synthesis Streaming Buffer Generation
// ===========================================================================

#[test]
fn test_f23_strip_emojis_and_markdown() {
    let raw = "✨ **Vanakkam da Mapla!** Formula: `E = mc^2` 🚀 Let's study!";
    let cleaned = strip_emojis_and_markdown(raw);

    assert!(!cleaned.contains("✨"));
    assert!(!cleaned.contains("🚀"));
    assert!(!cleaned.contains("**"));
    assert!(!cleaned.contains('`'));
    assert!(cleaned.contains("Vanakkam da Mapla!"));
    assert!(cleaned.contains("E = mc^2"));
}

#[test]
fn test_f23_thanglish_to_tamil_phonetic_transliteration() {
    let input = "vanakkam mapla innaiku romba nalla padi";
    let tamil_phonetic = thanglish_to_tamil_phonetic(input);

    assert!(tamil_phonetic.contains("வணக்கம்"));
    assert!(tamil_phonetic.contains("மாப்ள"));
    assert!(tamil_phonetic.contains("இன்னைக்கு"));
    assert!(tamil_phonetic.contains("ரொம்ப"));
    assert!(tamil_phonetic.contains("நல்லா"));
    assert!(tamil_phonetic.contains("படி"));
}

#[test]
fn test_f23_no_console_window_flag_on_windows() {
    // Verify TtsEngine::stop() is safe outside Tokio runtime and prepare_speech_text sanitizes shell metacharacters
    TtsEngine::stop();
    assert!(!TtsEngine::is_speaking());

    let prepared = TtsEngine::prepare_speech_text(
        r#"Gauss Law: $E = \frac{q}{4\pi\epsilon_0 r^2}$ with "quotes" and \slashes\"#,
        "en",
    );
    assert!(!prepared.contains('"'));
    assert!(!prepared.contains('\\'));
    assert!(!prepared.contains('$'));
    assert!(prepared.contains("divided by"));
}

#[test]
fn test_f23_piper_and_edge_neural_voice_routing() {
    let ta = LanguagePreference::from_lang_code("ta-IN");
    let en = LanguagePreference::from_lang_code("en-IN");
    let auto = LanguagePreference::from_lang_code("auto");

    assert_eq!(ta.default_neural_voice(), "ta-IN-ValluvarNeural");
    assert_eq!(ta.secondary_neural_voice(), "ta-IN-PallaviNeural");
    assert_eq!(en.default_neural_voice(), "en-IN-NeerjaNeural");
    assert_eq!(auto.default_neural_voice(), "ta-IN-PallaviNeural");

    assert_eq!(ta.default_piper_model(), "ta_IN-valluvar-medium");
    assert_eq!(en.default_piper_model(), "en_US-lessac-medium");

    let route_ta = VoiceEngine::resolve_optimal_route(&ta);
    assert!(route_ta.estimated_ram_mb() < 60);
    assert!(!route_ta.description().is_empty());

    let cloud_route = TtsRoute::CloudNeural {
        voice: ta.default_neural_voice().to_string(),
    };
    assert_eq!(cloud_route.estimated_ram_mb(), 4);
    let piper_route = TtsRoute::PiperLocal {
        model_path: std::path::PathBuf::from(ta.default_piper_model()),
    };
    assert!(piper_route.estimated_ram_mb() < 60);
}

#[test]
fn test_f23_audio_streaming_buffer_size_calculation() {
    // 16,000 Hz, 16-bit mono, 2.5 seconds = 80,000 PCM bytes + 44-byte RIFF WAVE header
    let pcm = vec![0u8; 80_000];
    let wav = create_wav_header(pcm, 16_000, 1, 16);

    assert_eq!(wav.len(), 80_044);
    assert_eq!(&wav[0..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    assert_eq!(&wav[12..16], b"fmt ");
    assert_eq!(&wav[36..40], b"data");
    assert_eq!(
        u32::from_le_bytes([wav[24], wav[25], wav[26], wav[27]]),
        16_000
    );
    assert_eq!(
        u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]),
        80_000
    );
}

// ===========================================================================
// FEATURE 24: Dual-Text Script Generation (Display vs Speech)
// ===========================================================================

#[test]
fn test_f24_parse_tagged_dual_text_format() {
    let tagged_output = "DISPLAY: Electric flux formula is phi = oint E . dS = q / eps0\n\
                         SPEECH: Electric flux equals q divided by epsilon not.";

    let res = parse_dual_text(tagged_output, "thanglish");
    assert_eq!(
        res.display_text,
        "Electric flux formula is phi = oint E . dS = q / eps0"
    );
    assert_eq!(
        res.speech_text,
        "Electric flux equals q divided by epsilon not."
    );
    assert_eq!(res.primary_lang, "ta-IN");
}

#[test]
fn test_f24_parse_json_dual_text_format() {
    let json_output = r#"{
        "display_text": "Gauss Law: \\oint \\vec{E} \\cdot d\\vec{A} = \\frac{Q_{enc}}{\\varepsilon_0}",
        "speech_text": "Gauss Law states that total electric flux equals enclosed charge over epsilon not.",
        "primary_lang": "en-IN"
    }"#;

    let res = parse_dual_text(json_output, "en");
    assert!(res.display_text.contains("\\oint"));
    assert!(res.speech_text.contains("total electric flux"));
    assert_eq!(res.primary_lang, "en-IN");
}

#[test]
fn test_f24_markdown_json_block_stripping() {
    let markdown_json = "```json\n{\n  \"display_text\": \"Derivation on screen\",\n  \"speech_text\": \"Spoken aloud\"\n}\n```";
    let res = parse_dual_text(markdown_json, "thanglish");

    assert_eq!(res.display_text, "Derivation on screen");
    assert_eq!(res.speech_text, "Spoken aloud");
}

#[test]
fn test_f24_fallback_text_mirroring() {
    let plain_en = "Focus on the core concept step by step.";
    let res_en = parse_dual_text(plain_en, "english");
    assert_eq!(res_en.display_text, plain_en);
    assert_eq!(res_en.speech_text, plain_en);

    // In Thanglish mode, speech text is phonetically optimized for TTS
    let plain_th = "Vanakkam mapla, simple direct advice.";
    let res_th = parse_dual_text(plain_th, "thanglish");
    assert_eq!(res_th.display_text, plain_th);
    assert!(!res_th.speech_text.is_empty());
}

#[test]
fn test_f24_language_tag_assignment() {
    assert_eq!(MaindyStyle::from_str_code("thanglish").code(), "thanglish");
    assert_eq!(MaindyStyle::from_str_code("tamil").code(), "ta");
    assert_eq!(MaindyStyle::from_str_code("english").code(), "en");
    assert_eq!(MaindyStyle::from_str_code("malayalam").code(), "ml");
}
