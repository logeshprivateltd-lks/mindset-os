//! Maindy AI Bilingual Persona & Dual-Text Pronunciation Engine
//!
//! Provides conversation styles (Thanglish, Tamil, English, Malayalam, Auto-Match)
//! and the Dual-Text schema (display_text for UI, speech_text for native TTS).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaindyStyle {
    Thanglish,
    Tamil,
    English,
    Malayalam,
    Auto,
}

impl MaindyStyle {
    pub fn from_str_code(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "ta" | "tamil" => MaindyStyle::Tamil,
            "en" | "english" => MaindyStyle::English,
            "ml" | "malayalam" => MaindyStyle::Malayalam,
            "auto" => MaindyStyle::Auto,
            _ => MaindyStyle::Thanglish,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            MaindyStyle::Thanglish => "thanglish",
            MaindyStyle::Tamil => "ta",
            MaindyStyle::English => "en",
            MaindyStyle::Malayalam => "ml",
            MaindyStyle::Auto => "auto",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MaindyStyle::Thanglish => "Thanglish (Casual Buddy)",
            MaindyStyle::Tamil => "தமிழ் (Tamil Mentor)",
            MaindyStyle::English => "English (Academic Mentor)",
            MaindyStyle::Malayalam => "മലയാളം (Malayalam Mentor)",
            MaindyStyle::Auto => "Auto-Match User Style",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DualTextResult {
    pub display_text: String,
    pub speech_text: String,
    pub primary_lang: String,
}

/// Builds the system prompt enforcing persona, exam focus, and Dual-Text output format.
pub fn build_system_prompt(
    style: &str,
    student_name: &str,
    target_track: &str,
    weak_topics: &[String],
) -> String {
    let mode = MaindyStyle::from_str_code(style);
    let name = if student_name.trim().is_empty() { "Scholar" } else { student_name.trim() };
    let track = if target_track.trim().is_empty() { "TN Class 12 Boards / JEE" } else { target_track.trim() };
    let weak_str = if weak_topics.is_empty() {
        "Core Fundamentals".to_string()
    } else {
        weak_topics.join(", ")
    };

    let style_guidance = match mode {
        MaindyStyle::Thanglish => {
            "You speak in natural, friendly, encouraging THANGLISH (Tamil written in Latin/English letters, like 'Vanakkam da mapla! Photoelectric effect romba easy, formula paaru: hf = W + KE'). Keep formulas and standard scientific terminology in English so exam readiness is preserved."
        }
        MaindyStyle::Tamil => {
            "You speak in clear, encouraging native Tamil (தமிழ்). Keep scientific formulas (e.g. hf = W + KE) and technical exam terms in standard English so the student stays exam-ready."
        }
        MaindyStyle::English => {
            "You speak as an encouraging, crisp, top-tier academic mentor in clear Indian English. Direct, insightful, and motivating."
        }
        MaindyStyle::Malayalam => {
            "You speak in friendly, encouraging Malayalam / Manglish (മലയാളം), keeping scientific formulas and technical terms in English."
        }
        MaindyStyle::Auto => {
            "Detect the student's language and tone (Thanglish, Tamil, English, or Malayalam) and mirror it naturally and encourage them."
        }
    };

    format!(
r#"You are Maindy AI, the goal-guided personal tutor and cognitive study companion for MindSet OS.
Student Name: {name}
Target Track: {track}
Priority Focus / Weak Topics: {weak_str}

Style Guidance:
{style_guidance}

CRITICAL DUAL-TEXT OUTPUT RULE:
Whenever possible, reply in a clean JSON object with this exact structure:
{{
  "display_text": "<Your full helpful explanation with math formulas for the screen UI>",
  "speech_text": "<The phonetic spoken text optimized for natural speech audio>",
  "primary_lang": "ta-IN" (or "en-IN", "ml-IN")
}}
If you cannot output JSON, simply output your answer directly."#
    )
}

/// Common phonetic transliteration table from conversational Thanglish to Tamil Unicode
pub fn thanglish_to_tamil_phonetic(input: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for word in input.split_whitespace() {
        let clean = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        let replacement = match clean.as_str() {
            "vanakkam" => "வணக்கம்",
            "mapla" | "maapla" => "மாப்ள",
            "thambi" => "தம்பி",
            "akka" => "அக்கா",
            "nalla" | "nallaa" => "நல்லா",
            "romba" => "ரொம்ப",
            "paaru" => "பாரு",
            "paathuko" => "பாத்துக்கோ",
            "enna" => "என்ன",
            "super" | "semma" => "சூப்பர்",
            "sari" => "சரி",
            "solra" | "solren" => "சொல்றேன்",
            "innaiku" => "இன்னைக்கு",
            "mudichuralaama" => "முடிச்சுடலாமா",
            "mudiyum" => "முடியும்",
            "padi" => "படி",
            "padikkalaam" => "படிக்கலாம்",
            "theriyuma" => "தெரியுமா",
            "purinjidha" => "புரிஞ்சுதா",
            "kandippa" => "கண்டிப்பா",
            "easy" => "ஈஸி",
            _ => word,
        };
        words.push(replacement.to_string());
    }
    words.join(" ")
}

/// Parses the LLM's raw response, extracting display_text and speech_text cleanly.
pub fn parse_dual_text(raw: &str, fallback_style: &str) -> DualTextResult {
    let trimmed = raw.trim();

    // 0. Try tagged format: DISPLAY: ... SPEECH: ...
    if let Some(disp_idx) = trimmed.find("DISPLAY:") {
        let after_disp = &trimmed[disp_idx + "DISPLAY:".len()..];
        if let Some(speech_idx) = after_disp.find("SPEECH:") {
            let display = after_disp[..speech_idx].trim().to_string();
            let speech = after_disp[speech_idx + "SPEECH:".len()..].trim().to_string();
            let style = MaindyStyle::from_str_code(fallback_style);
            let lang = match style {
                MaindyStyle::Thanglish | MaindyStyle::Tamil => "ta-IN",
                MaindyStyle::Malayalam => "ml-IN",
                _ => "en-IN",
            };
            return DualTextResult {
                display_text: display,
                speech_text: speech,
                primary_lang: lang.to_string(),
            };
        }
    }

    // 1. Try stripping markdown JSON code blocks ```json ... ```
    let candidate = if trimmed.starts_with("```json") {
        trimmed
            .strip_prefix("```json")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
    } else if trimmed.starts_with("```") {
        trimmed
            .strip_prefix("```")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
    } else {
        trimmed
    };

    // 2. Try JSON deserialization
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(candidate) {
        if let Some(display) = val.get("display_text").and_then(|v| v.as_str()) {
            let speech = val
                .get("speech_text")
                .and_then(|v| v.as_str())
                .unwrap_or(display)
                .to_string();
            let lang = val
                .get("primary_lang")
                .and_then(|v| v.as_str())
                .unwrap_or("ta-IN")
                .to_string();

            return DualTextResult {
                display_text: display.to_string(),
                speech_text: speech,
                primary_lang: lang,
            };
        }
    }

    // 3. Fallback for plain text response
    let style = MaindyStyle::from_str_code(fallback_style);
    let (speech_text, primary_lang) = match style {
        MaindyStyle::Thanglish => (thanglish_to_tamil_phonetic(trimmed), "ta-IN".to_string()),
        MaindyStyle::Tamil => (trimmed.to_string(), "ta-IN".to_string()),
        MaindyStyle::Malayalam => (trimmed.to_string(), "ml-IN".to_string()),
        _ => (trimmed.to_string(), "en-IN".to_string()),
    };

    DualTextResult {
        display_text: trimmed.to_string(),
        speech_text,
        primary_lang,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thanglish_transliteration() {
        let input = "Vanakkam mapla, innaiku super-aa padikkalaam!";
        let out = thanglish_to_tamil_phonetic(input);
        assert!(out.contains("வணக்கம்"));
        assert!(out.contains("மாப்ள"));
    }

    #[test]
    fn test_dual_text_json_parsing() {
        let raw = r#"{
            "display_text": "Vanakkam da mapla! Photoelectric formula: hf = W + KE",
            "speech_text": "வணக்கம் டா மாப்ள! Photoelectric formula: h f equals W plus K E",
            "primary_lang": "ta-IN"
        }"#;

        let res = parse_dual_text(raw, "thanglish");
        assert_eq!(res.display_text, "Vanakkam da mapla! Photoelectric formula: hf = W + KE");
        assert!(res.speech_text.contains("வணக்கம்"));
        assert_eq!(res.primary_lang, "ta-IN");
    }

    #[test]
    fn test_dual_text_tagged_parsing() {
        let raw = "DISPLAY: Vanakkam da mapla! Photoelectric formula: hf = W + KE\n\nSPEECH: வணக்கம் டா மாப்ள! போட்டோ எலக்ட்ரிக் ஃபார்முலா";
        let res = parse_dual_text(raw, "thanglish");
        assert_eq!(res.display_text, "Vanakkam da mapla! Photoelectric formula: hf = W + KE");
        assert!(res.speech_text.contains("வணக்கம்"));
        assert_eq!(res.primary_lang, "ta-IN");
    }

    #[test]
    fn test_dual_text_plain_text_fallback() {
        let raw = "Vanakkam mapla, exam ready-aa?";
        let res = parse_dual_text(raw, "thanglish");
        assert_eq!(res.display_text, "Vanakkam mapla, exam ready-aa?");
        assert!(res.speech_text.contains("வணக்கம்"));
    }
}
