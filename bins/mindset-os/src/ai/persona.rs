//! Maindy AI Bilingual Persona & Dual-Text Pronunciation Engine
//!
//! Provides conversation styles (Thanglish, Tamil, English, Malayalam, Auto-Match),
//! the Dual-Text schema (display_text for UI, speech_text for native TTS),
//! and the Conversational Thanglish NLP Engine with 3-step semantic resolution.

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

// ---------------------------------------------------------------------------
// Conversational Thanglish NLP Domain Models (Feature 22)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectEntity {
    Physics,
    Chemistry,
    Mathematics,
    Biology,
    ComputerScience,
    General(String),
}

impl SubjectEntity {
    pub fn code(&self) -> &'static str {
        match self {
            SubjectEntity::Physics => "PHY",
            SubjectEntity::Chemistry => "CHEM",
            SubjectEntity::Mathematics => "MATH",
            SubjectEntity::Biology => "BIO",
            SubjectEntity::ComputerScience => "CS",
            SubjectEntity::General(_) => "GEN",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            SubjectEntity::Physics => "Physics",
            SubjectEntity::Chemistry => "Chemistry",
            SubjectEntity::Mathematics => "Mathematics",
            SubjectEntity::Biology => "Biology",
            SubjectEntity::ComputerScience => "Computer Science",
            SubjectEntity::General(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcademicIntent {
    /// Explain the theoretical concept or derivation (e.g. "solli kudu", "explain pannu")
    TeachConcept,
    /// Provide step-by-step problem solving walkthrough (e.g. "epdi podanum", "solve pannu")
    SolveStepByStep,
    /// Retrieve formula / equation memory anchor (e.g. "formula marandhuten", "formula enna")
    RecallFormula,
    /// Clarify confusing points or student doubts (e.g. "doubt irukku", "puriyala")
    ClarifyDoubt,
    /// Generate practice MCQs or exam drills (e.g. "mcq kudu", "test vey")
    GeneratePractice,
    /// Highlight high-yield questions for exam targeting (e.g. "important questions", "590 target")
    HighlightImportant,
    /// General conversational query or greeting
    GeneralInquiry,
}

impl AcademicIntent {
    pub fn code(&self) -> &'static str {
        match self {
            AcademicIntent::TeachConcept => "TEACH_CONCEPT",
            AcademicIntent::SolveStepByStep => "SOLVE_STEP_BY_STEP",
            AcademicIntent::RecallFormula => "RECALL_FORMULA",
            AcademicIntent::ClarifyDoubt => "CLARIFY_DOUBT",
            AcademicIntent::GeneratePractice => "GENERATE_PRACTICE",
            AcademicIntent::HighlightImportant => "HIGHLIGHT_IMPORTANT",
            AcademicIntent::GeneralInquiry => "GENERAL_INQUIRY",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedQuery {
    pub raw_query: String,
    pub normalized_tokens: Vec<String>,
    pub subject: Option<SubjectEntity>,
    pub chapter: Option<u32>,
    pub section: Option<String>,
    pub example: Option<String>,
    pub intent: AcademicIntent,
    pub target_audience: Option<String>,
    pub dative_context: Option<String>,
    pub confidence: u8,
}

// ---------------------------------------------------------------------------
// 3-Step Conversational Thanglish NLP Engine (Feature 22)
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct ExtractedEntities {
    pub subject: Option<SubjectEntity>,
    pub chapter: Option<u32>,
    pub section: Option<String>,
    pub example: Option<String>,
    pub dative: Option<String>,
}

pub struct ThanglishParser;

impl ThanglishParser {
    /// Performs full 3-step semantic resolution on conversational Thanglish input:
    /// 1. Colloquial token normalization (slang stripping & vocative capture)
    /// 2. Tamil grammar & postposition entity extraction (-la locative, -ku dative)
    /// 3. Action predicate intent classification
    pub fn parse(input: &str) -> ParsedQuery {
        let (normalized, audience) = Self::normalize_tokens(input);
        let raw_lower = input.to_lowercase();
        let entities = Self::extract_entities(&normalized, &raw_lower);
        let intent = Self::classify_intent(&normalized, &raw_lower);

        // Confidence scoring: base 50 + entity presence boosts
        let mut confidence: u8 = 50;
        if entities.subject.is_some() {
            confidence = confidence.saturating_add(15);
        }
        if entities.chapter.is_some() {
            confidence = confidence.saturating_add(15);
        }
        if entities.example.is_some() || entities.section.is_some() {
            confidence = confidence.saturating_add(10);
        }
        if intent != AcademicIntent::GeneralInquiry {
            confidence = confidence.saturating_add(10);
        }
        if confidence > 100 {
            confidence = 100;
        }

        ParsedQuery {
            raw_query: input.trim().to_string(),
            normalized_tokens: normalized,
            subject: entities.subject,
            chapter: entities.chapter,
            section: entities.section,
            example: entities.example,
            intent,
            target_audience: audience,
            dative_context: entities.dative,
            confidence,
        }
    }

    /// Step 1: Colloquial Token Normalization
    /// Strips conversational noise and extracts student vocative tags
    pub fn normalize_tokens(input: &str) -> (Vec<String>, Option<String>) {
        let raw = input.trim();
        let mut tokens: Vec<String> = Vec::new();
        let mut audience: Option<String> = None;

        // Slang & vocative tokens commonly used by Tamil students
        let vocatives = [
            "hey", "hi", "hello", "brow", "bro", "broo", "brother", "machan", "macha", "mapla",
            "maapla", "da", "pa", "dosthu", "bhai", "nanba", "nanbi", "thambi", "thala", "buddy",
            "dude", "dei", "ji", "thalaiva",
        ];

        // Replace punctuation with spaces, while preserving decimals (e.g. 10.6) and hyphens
        let mut cleaned = String::with_capacity(raw.len());
        let chars: Vec<char> = raw.chars().collect();
        let len = chars.len();

        for i in 0..len {
            let c = chars[i];
            if c == '.' {
                let prev_digit = i > 0 && chars[i - 1].is_ascii_digit();
                let next_digit = i + 1 < len && chars[i + 1].is_ascii_digit();
                if prev_digit && next_digit {
                    cleaned.push('.');
                } else {
                    cleaned.push(' ');
                }
            } else if c == '-' {
                cleaned.push('-');
            } else if c.is_alphanumeric() {
                cleaned.push(c.to_ascii_lowercase());
            } else {
                cleaned.push(' ');
            }
        }

        for raw_tok in cleaned.split_whitespace() {
            // Check for hyphenated postpositions (e.g. "p6-la" -> ["p6", "la"], "exam-ku" -> ["exam", "ku"])
            if raw_tok.contains('-') {
                for sub in raw_tok.split('-') {
                    let sub_trim = sub.trim();
                    if !sub_trim.is_empty() {
                        Self::push_or_process_token(
                            sub_trim,
                            &vocatives,
                            &mut tokens,
                            &mut audience,
                        );
                    }
                }
            } else {
                Self::push_or_process_token(raw_tok, &vocatives, &mut tokens, &mut audience);
            }
        }

        (tokens, audience)
    }

    fn push_stem_or_alnum(stem: &str, tokens: &mut Vec<String>) {
        // Split concatenated chapter/unit tokens like "ch10", "chapter10", "unit3"
        for prefix in ["chapter", "unit", "padam", "ch"] {
            if let Some(rest) = stem.strip_prefix(prefix) {
                if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
                    tokens.push(prefix.to_string());
                    tokens.push(rest.to_string());
                    return;
                }
            }
        }
        tokens.push(stem.to_string());
    }

    fn push_or_process_token(
        tok: &str,
        vocatives: &[&str],
        tokens: &mut Vec<String>,
        audience: &mut Option<String>,
    ) {
        if vocatives.contains(&tok) {
            if audience.is_none() {
                *audience = Some(tok.to_string());
            }
            // Skip conversational filler vocatives from normalized semantic stream
            return;
        }

        // English academic words & Tamil words ending in -la/-le/-ku/-kku that must NEVER be split
        const ALLOWLIST: &[&str] = &[
            "example",
            "formula",
            "molecule",
            "angle",
            "triangle",
            "circle",
            "principle",
            "variable",
            "rule",
            "module",
            "table",
            "double",
            "single",
            "simple",
            "article",
            "particle",
            "scale",
            "cycle",
            "irukku",
            "sollikudu",
            "kattukudu",
            "nalla",
            "nallaa",
            "puriyala",
            "puriyale",
            "illa",
            "illae",
            "pilla",
            "ella",
            "killa",
            "solla",
            "sollu",
            "kudu",
        ];

        if ALLOWLIST.contains(&tok) {
            tokens.push(tok.to_string());
            return;
        }

        // Handle concatenated postposition endings like "p6la" -> ["p6", "la"], "ch10la" -> ["ch", "10", "la"]
        if tok.len() > 3 && (tok.ends_with("la") || tok.ends_with("le")) {
            let stem = &tok[..tok.len() - 2];
            let post = &tok[tok.len() - 2..];
            Self::push_stem_or_alnum(stem, tokens);
            tokens.push(post.to_string());
            return;
        }

        if tok.len() > 3 && (tok.ends_with("ku") || tok.ends_with("kku")) {
            let cut = if tok.ends_with("kku") { 3 } else { 2 };
            let stem = &tok[..tok.len() - cut];
            let post = &tok[tok.len() - cut..];
            Self::push_stem_or_alnum(stem, tokens);
            tokens.push(post.to_string());
            return;
        }

        Self::push_stem_or_alnum(tok, tokens);
    }

    /// Step 2: Tamil Grammar & Postposition Entity Extraction
    /// Resolves `-la` / `-le` locative and `-ku` / `-kku` dative markers
    pub fn extract_entities(tokens: &[String], _raw_lower: &str) -> ExtractedEntities {
        let mut subject: Option<SubjectEntity> = None;
        let mut chapter: Option<u32> = None;
        let mut section: Option<String> = None;
        let mut example: Option<String> = None;
        let mut dative: Option<String> = None;

        let num_tokens = tokens.len();

        for i in 0..num_tokens {
            let tok = &tokens[i];

            // 1. Subject extraction (shorthands like p6, phy, physics, c4, chem, maths, bio, cs)
            if subject.is_none() {
                if tok == "physics" || tok == "phy" {
                    subject = Some(SubjectEntity::Physics);
                } else if tok == "chemistry" || tok == "chem" {
                    subject = Some(SubjectEntity::Chemistry);
                } else if tok == "maths" || tok == "math" || tok == "mathematics" {
                    subject = Some(SubjectEntity::Mathematics);
                } else if tok == "biology" || tok == "bio" || tok == "botany" || tok == "zoology" {
                    subject = Some(SubjectEntity::Biology);
                } else if tok == "cs" || tok == "computer" {
                    subject = Some(SubjectEntity::ComputerScience);
                } else if tok.starts_with('p')
                    && !tok[1..].is_empty()
                    && tok[1..].chars().all(|c| c.is_ascii_digit())
                {
                    // Shorthands like p6, p1, p2, p12
                    subject = Some(SubjectEntity::Physics);
                } else if tok.starts_with('c')
                    && !tok[1..].is_empty()
                    && tok[1..].chars().all(|c| c.is_ascii_digit())
                {
                    // Shorthands like c1, c2, c4
                    subject = Some(SubjectEntity::Chemistry);
                } else if tok.starts_with('m')
                    && !tok[1..].is_empty()
                    && tok[1..].chars().all(|c| c.is_ascii_digit())
                {
                    // Shorthands like m1, m2
                    subject = Some(SubjectEntity::Mathematics);
                }
            }

            // 2. Chapter extraction ("chapter 10", "ch 10", "unit 3", "10th chapter")
            if chapter.is_none() {
                if (tok == "chapter" || tok == "ch" || tok == "unit" || tok == "padam")
                    && i + 1 < num_tokens
                {
                    if let Ok(num) = tokens[i + 1].parse::<u32>() {
                        chapter = Some(num);
                    }
                } else if tok.ends_with("th")
                    || tok.ends_with("st")
                    || tok.ends_with("nd")
                    || tok.ends_with("rd")
                {
                    let prefix = &tok[..tok.len() - 2];
                    if let Ok(num) = prefix.parse::<u32>() {
                        if i + 1 < num_tokens
                            && (tokens[i + 1] == "chapter"
                                || tokens[i + 1] == "ch"
                                || tokens[i + 1] == "unit")
                        {
                            chapter = Some(num);
                        }
                    }
                }
            }

            // 3. Section / Example extraction (e.g. "10.6 example", "example 10.6", "sec 10.6")
            if tok.contains('.') && tok.chars().all(|c| c.is_ascii_digit() || c == '.') {
                let dec = tok.clone();
                let is_example = (i + 1 < num_tokens
                    && (tokens[i + 1] == "example"
                        || tokens[i + 1] == "eg"
                        || tokens[i + 1] == "ex"))
                    || (i > 0
                        && (tokens[i - 1] == "example"
                            || tokens[i - 1] == "eg"
                            || tokens[i - 1] == "ex"));

                if is_example {
                    example = Some(dec.clone());
                    section = Some(dec);
                } else {
                    section = Some(dec.clone());
                    if example.is_none() {
                        example = Some(dec);
                    }
                }
            } else if (tok == "example" || tok == "eg" || tok == "ex") && i + 1 < num_tokens {
                let next = &tokens[i + 1];
                if next.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    example = Some(next.clone());
                    section = Some(next.clone());
                }
            }

            // 4. Dative marker `-ku` / `-kku` extraction (e.g. "exam-ku", "test-ku", "revision-ku")
            if dative.is_none() && (tok == "ku" || tok == "kku" || tok == "ukku") && i > 0 {
                let prev = &tokens[i - 1];
                dative = Some(prev.clone());
            }
        }

        // Fallback scan on normalized token boundaries (avoids substring false positives like "example" -> "exam")
        if dative.is_none() {
            if tokens.iter().any(|t| t == "exam" || t == "exams") {
                dative = Some("exam".to_string());
            } else if tokens.iter().any(|t| t == "test" || t == "tests") {
                dative = Some("test".to_string());
            } else if tokens.iter().any(|t| t == "revision") {
                dative = Some("revision".to_string());
            } else if tokens.iter().any(|t| t == "590") {
                dative = Some("590_marks".to_string());
            }
        }

        ExtractedEntities {
            subject,
            chapter,
            section,
            example,
            dative,
        }
    }

    /// Step 3: Action Predicate Intent Classification
    pub fn classify_intent(tokens: &[String], raw_lower: &str) -> AcademicIntent {
        let joined = tokens.join(" ");

        // Priority 1: Formula Recall
        if raw_lower.contains("formula marandhuten")
            || raw_lower.contains("formula enna")
            || raw_lower.contains("formula sollu")
            || raw_lower.contains("formula venum")
            || raw_lower.contains("equation enna")
            || raw_lower.contains("equation sollu")
            || joined.contains("formula marandhuten")
            || joined.contains("formula enna")
        {
            return AcademicIntent::RecallFormula;
        }

        // Priority 2: Step-by-Step Problem Solving
        if raw_lower.contains("epdi podanum")
            || raw_lower.contains("epdi solve pannanum")
            || raw_lower.contains("solve pannu")
            || raw_lower.contains("step by step")
            || raw_lower.contains("epdi varum")
            || raw_lower.contains("working kaattu")
            || raw_lower.contains("steps kaattu")
            || joined.contains("epdi podanum")
            || joined.contains("solve pannu")
        {
            return AcademicIntent::SolveStepByStep;
        }

        // Priority 3: Concept Teaching & Derivations
        if raw_lower.contains("solli kudu")
            || raw_lower.contains("sollikudu")
            || raw_lower.contains("solli thaa")
            || raw_lower.contains("solli tharen")
            || raw_lower.contains("explain pannu")
            || raw_lower.contains("kattukudu")
            || raw_lower.contains("teach pannu")
            || raw_lower.contains("puriyavei")
            || raw_lower.contains("puriyavey")
            || joined.contains("solli kudu")
            || joined.contains("sollikudu")
            || joined.contains("solli thaa")
            || joined.contains("explain pannu")
        {
            return AcademicIntent::TeachConcept;
        }

        // Priority 4: Doubts & Clarifications
        if raw_lower.contains("doubt irukku")
            || raw_lower.contains("puriyala")
            || raw_lower.contains("puriyale")
            || raw_lower.contains("clarify pannu")
            || raw_lower.contains("confusion")
            || joined.contains("doubt")
            || joined.contains("puriyala")
        {
            return AcademicIntent::ClarifyDoubt;
        }

        // Priority 5: Practice & MCQs
        if raw_lower.contains("mcq kudu")
            || raw_lower.contains("question kelu")
            || raw_lower.contains("test vey")
            || raw_lower.contains("quiz pannu")
            || raw_lower.contains("practice kudu")
            || joined.contains("mcq")
            || joined.contains("quiz")
        {
            return AcademicIntent::GeneratePractice;
        }

        // Priority 6: Important Exam Topics / High Weightage
        if raw_lower.contains("important topic")
            || raw_lower.contains("important questions")
            || raw_lower.contains("high weightage")
            || raw_lower.contains("mukkiyam")
            || raw_lower.contains("exam la varuma")
            || joined.contains("important")
            || joined.contains("weightage")
        {
            return AcademicIntent::HighlightImportant;
        }

        AcademicIntent::GeneralInquiry
    }

    /// Formats the parsed Thanglish query into a structured academic instruction for Maindy AI
    pub fn format_academic_prompt(query: &ParsedQuery) -> String {
        let mut context_parts: Vec<String> = Vec::new();

        if let Some(ref sub) = query.subject {
            context_parts.push(format!("Subject: {}", sub.display_name()));
        }
        if let Some(ch) = query.chapter {
            context_parts.push(format!("Chapter: {ch}"));
        }
        if let Some(ref sec) = query.section {
            context_parts.push(format!("Section/Example: {sec}"));
        }
        if let Some(ref dat) = query.dative_context {
            context_parts.push(format!("Target Context: {dat}"));
        }
        context_parts.push(format!("Academic Intent: {}", query.intent.code()));

        format!(
            "[Maindy AI Grounding Context: {}]\nStudent Query: \"{}\"",
            context_parts.join(" | "),
            query.raw_query
        )
    }
}

// ---------------------------------------------------------------------------
// Dual-Text Result & Phonetic Speech Generation (Feature 24)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DualTextResult {
    pub display_text: String,
    pub speech_text: String,
    pub primary_lang: String,
}

impl DualTextResult {
    pub fn new(
        display: impl Into<String>,
        speech: impl Into<String>,
        lang: impl Into<String>,
    ) -> Self {
        Self {
            display_text: display.into(),
            speech_text: speech.into(),
            primary_lang: lang.into(),
        }
    }
}

/// Expands mathematical LaTeX expressions and markdown into natural, phonetically tuned spoken speech.
/// Preserves scientific accuracy while ensuring the voice engine pronounces formulas naturally.
pub fn expand_latex_to_phonetic_speech(raw: &str, lang: &str) -> String {
    let mut text = raw.to_string();

    // 1. Expand common multi-token LaTeX constructs
    text = text
        .replace("\\oint", " closed surface integral ")
        .replace("\\int", " integral ")
        .replace("\\sum", " summation ")
        .replace("\\vec{E}", " E vector ")
        .replace("\\vec{A}", " A vector ")
        .replace("\\vec{B}", " B vector ")
        .replace("\\vec{v}", " v vector ")
        .replace("\\vec{F}", " F vector ")
        .replace("\\cdot", " dot ")
        .replace("\\times", " into ")
        .replace("\\varepsilon_0", " epsilon naught ")
        .replace("\\epsilon_0", " epsilon naught ")
        .replace("\\mu_0", " mu naught ")
        .replace("\\lambda", " lambda ")
        .replace("\\sigma", " sigma ")
        .replace("\\omega", " omega ")
        .replace("\\theta", " theta ")
        .replace("\\alpha", " alpha ")
        .replace("\\beta", " beta ")
        .replace("\\gamma", " gamma ")
        .replace("\\Delta", " delta ")
        .replace("\\pi", " pie ")
        .replace("\\Phi_E", " flux phi ")
        .replace("\\Phi", " flux phi ")
        .replace("\\approx", " approximately ")
        .replace("\\pm", " plus or minus ")
        .replace("\\le", " less than or equal to ")
        .replace("\\ge", " greater than or equal to ")
        .replace("\\rightarrow", " gives ")
        .replace("q_{\\text{enc}}", " q enclosed ")
        .replace("q_{\\text{enclosed}}", " q enclosed ");

    // 2. Expand fractions \frac{num}{den} -> "num divided by den"
    while let Some(pos) = text.find("\\frac") {
        let after = &text[pos + 5..];
        if let Some(open_first) = after.find('{') {
            if let Some(close_first) = after.find('}') {
                let num = &after[open_first + 1..close_first];
                let rem = &after[close_first + 1..];
                if let Some(open_second) = rem.find('{') {
                    if let Some(close_second) = rem.find('}') {
                        let den = &rem[open_second + 1..close_second];
                        let total_len = 5 + close_first + 1 + close_second + 1;
                        let replacement = format!(" {} divided by {} ", num.trim(), den.trim());
                        text.replace_range(pos..pos + total_len, &replacement);
                        continue;
                    }
                }
            }
        }
        // Fallback break if unparseable fraction
        break;
    }

    // 3. Expand square roots \sqrt{val} -> "square root of val"
    while let Some(pos) = text.find("\\sqrt") {
        let after = &text[pos + 5..];
        if let Some(open) = after.find('{') {
            if let Some(close) = after.find('}') {
                let inside = &after[open + 1..close];
                let total_len = 5 + close + 1;
                let replacement = format!(" square root of {} ", inside.trim());
                text.replace_range(pos..pos + total_len, &replacement);
                continue;
            }
        }
        break;
    }

    // 4. Expand powers and subscripts
    text = text
        .replace("^2", " squared ")
        .replace("^3", " cubed ")
        .replace("_0", " naught ")
        .replace("_1", " 1 ")
        .replace("_2", " 2 ")
        .replace(" = ", " equals ")
        .replace(" + ", " plus ")
        .replace(" - ", " minus ")
        .replace(" * ", " times ")
        .replace(" / ", " divided by ")
        .replace('%', " percent ");

    // 5. Strip markdown and remaining LaTeX markup characters
    text = text
        .replace("$$", " ")
        .replace("**", " ")
        .replace(['$', '\\', '{', '}', '*', '`', '#', '~'], " ");

    // 6. If language is Tamil/Thanglish, transliterate common colloquial words
    if lang.starts_with("ta") || lang.contains("thanglish") {
        thanglish_to_tamil_phonetic(&text)
    } else {
        // Collapse multiple whitespace
        text.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}

/// Generates a DualTextResult with Latin/LaTeX formatted display text and phonetically optimized speech text
pub fn generate_dual_text(display_markdown_latex: &str, lang: &str) -> DualTextResult {
    let clean_display = display_markdown_latex.trim().to_string();
    let spoken = expand_latex_to_phonetic_speech(&clean_display, lang);
    let primary_lang = if lang.starts_with("ta") || lang.contains("thanglish") {
        "ta-IN".to_string()
    } else if lang.starts_with("ml") {
        "ml-IN".to_string()
    } else {
        "en-IN".to_string()
    };

    DualTextResult {
        display_text: clean_display,
        speech_text: spoken,
        primary_lang,
    }
}

/// Builds the system prompt enforcing persona, exam focus, and Dual-Text output format.
pub fn build_system_prompt(
    style: &str,
    student_name: &str,
    target_track: &str,
    weak_topics: &[String],
) -> String {
    let mode = MaindyStyle::from_str_code(style);
    let name = if student_name.trim().is_empty() {
        "Scholar"
    } else {
        student_name.trim()
    };
    let track = if target_track.trim().is_empty() {
        "TN Class 12 Boards / JEE"
    } else {
        target_track.trim()
    };
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
        let clean = word
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();
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
            let speech_raw = after_disp[speech_idx + "SPEECH:".len()..]
                .trim()
                .to_string();
            let style = MaindyStyle::from_str_code(fallback_style);
            let lang = match style {
                MaindyStyle::Thanglish | MaindyStyle::Tamil => "ta-IN",
                MaindyStyle::Malayalam => "ml-IN",
                _ => "en-IN",
            };
            let speech_opt = expand_latex_to_phonetic_speech(&speech_raw, lang);
            return DualTextResult {
                display_text: display,
                speech_text: speech_opt,
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
            let speech_raw = val
                .get("speech_text")
                .and_then(|v| v.as_str())
                .unwrap_or(display);
            let lang = val
                .get("primary_lang")
                .and_then(|v| v.as_str())
                .unwrap_or("ta-IN")
                .to_string();
            let speech_opt = expand_latex_to_phonetic_speech(speech_raw, &lang);

            return DualTextResult {
                display_text: display.to_string(),
                speech_text: speech_opt,
                primary_lang: lang,
            };
        }
    }

    // 3. Fallback for plain text response
    let style = MaindyStyle::from_str_code(fallback_style);
    let lang = match style {
        MaindyStyle::Thanglish | MaindyStyle::Tamil => "ta-IN",
        MaindyStyle::Malayalam => "ml-IN",
        _ => "en-IN",
    };

    generate_dual_text(trimmed, lang)
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
    fn test_thanglish_parser_query_p6_ch10_example_10_6() {
        // Query from Requirement R6:
        // "hey brow , bro p6 la chapter 10 la 10.6 example solli kudu bro"
        let input = "hey brow , bro p6 la chapter 10 la 10.6 example solli kudu bro";
        let parsed = ThanglishParser::parse(input);

        assert_eq!(parsed.subject, Some(SubjectEntity::Physics));
        assert_eq!(parsed.chapter, Some(10));
        assert_eq!(parsed.example, Some("10.6".to_string()));
        assert_eq!(parsed.section, Some("10.6".to_string()));
        assert_eq!(parsed.intent, AcademicIntent::TeachConcept);
        assert!(parsed.confidence >= 85);
        assert!(parsed.target_audience.is_some());
    }

    #[test]
    fn test_thanglish_parser_intent_classification() {
        let q1 = ThanglishParser::parse("machan chemistry la chapter 4 epdi podanum solve pannu");
        assert_eq!(q1.subject, Some(SubjectEntity::Chemistry));
        assert_eq!(q1.chapter, Some(4));
        assert_eq!(q1.intent, AcademicIntent::SolveStepByStep);

        let q2 = ThanglishParser::parse("maths unit 3 formula marandhuten bro");
        assert_eq!(q2.subject, Some(SubjectEntity::Mathematics));
        assert_eq!(q2.chapter, Some(3));
        assert_eq!(q2.intent, AcademicIntent::RecallFormula);

        let q3 = ThanglishParser::parse("physics la doubt irukku puriyala");
        assert_eq!(q3.subject, Some(SubjectEntity::Physics));
        assert_eq!(q3.intent, AcademicIntent::ClarifyDoubt);

        let q4 = ThanglishParser::parse("exam-ku mcq kudu test vey");
        assert_eq!(q4.dative_context, Some("exam".to_string()));
        assert_eq!(q4.intent, AcademicIntent::GeneratePractice);
    }

    #[test]
    fn test_thanglish_academic_prompt_formatting() {
        let input = "hey bro p6 la chapter 10 la 10.6 example solli kudu bro";
        let parsed = ThanglishParser::parse(input);
        let prompt = ThanglishParser::format_academic_prompt(&parsed);

        assert!(prompt.contains("Subject: Physics"));
        assert!(prompt.contains("Chapter: 10"));
        assert!(prompt.contains("Section/Example: 10.6"));
        assert!(prompt.contains("Academic Intent: TEACH_CONCEPT"));
    }

    #[test]
    fn test_dual_text_json_parsing() {
        let raw = r#"{
            "display_text": "Vanakkam da mapla! Photoelectric formula: hf = W + KE",
            "speech_text": "வணக்கம் டா மாப்ள! Photoelectric formula: hf = W + KE",
            "primary_lang": "ta-IN"
        }"#;

        let res = parse_dual_text(raw, "thanglish");
        assert_eq!(
            res.display_text,
            "Vanakkam da mapla! Photoelectric formula: hf = W + KE"
        );
        assert!(res.speech_text.contains("வணக்கம்"));
        assert_eq!(res.primary_lang, "ta-IN");
    }

    #[test]
    fn test_dual_text_tagged_parsing() {
        let raw = "DISPLAY: Vanakkam da mapla! Photoelectric formula: hf = W + KE\n\nSPEECH: வணக்கம் டா மாப்ள! போட்டோ எலக்ட்ரிக் ஃபார்முலா";
        let res = parse_dual_text(raw, "thanglish");
        assert_eq!(
            res.display_text,
            "Vanakkam da mapla! Photoelectric formula: hf = W + KE"
        );
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

    #[test]
    fn test_latex_expansion_to_phonetic_speech() {
        let latex = r#"Gauss's Law is $$\oint \vec{E} \cdot d\vec{A} = \frac{q}{\varepsilon_0}$$ with $E = \frac{\lambda}{2\pi\varepsilon_0 r}$"#;
        let speech = expand_latex_to_phonetic_speech(latex, "en-IN");

        assert!(speech.contains("closed surface integral"));
        assert!(speech.contains("E vector"));
        assert!(speech.contains("dot"));
        assert!(speech.contains("divided by"));
        assert!(speech.contains("epsilon naught"));
        assert!(!speech.contains("$$"));
        assert!(!speech.contains('\\'));
    }

    #[test]
    fn test_thanglish_allowlist_and_variable_boundary_guards() {
        // 1. "example", "formula", "circle", "irukku", "sollikudu" must not be split
        let (tokens, _) =
            ThanglishParser::normalize_tokens("example formula circle irukku sollikudu");
        assert_eq!(
            tokens,
            vec!["example", "formula", "circle", "irukku", "sollikudu"]
        );

        // 2. "example" must NOT trigger dative_context = Some("exam")
        let q_example = ThanglishParser::parse("p6la ch10 10.6 example sollikudu");
        assert_eq!(q_example.subject, Some(SubjectEntity::Physics));
        assert_eq!(q_example.chapter, Some(10));
        assert_eq!(q_example.example.as_deref(), Some("10.6"));
        assert_eq!(q_example.intent, AcademicIntent::TeachConcept);
        assert_eq!(q_example.dative_context, None);

        // 3. Single-letter variables p, c, m must NOT be misclassified as subjects
        let q_vars = ThanglishParser::parse("find p and c and m values");
        assert_eq!(q_vars.subject, None);
    }
}
