//! Input Sanitization, Prompt Safety & Structured Pedagogical Study Templates.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub const MAX_QUERY_CHARS: usize = 2048;

pub const DEFAULT_STUDY_SYSTEM_PROMPT: &str = "\
You are Maindy AI, an intelligent, empathetic, and rigorous academic study tutor on MindSet OS developed by LKS PVT LTD.
Your objective is to help Class 12 and university students master difficult concepts with clarity and precision.
Guidelines:
1. Provide structured explanations with bold headings, clear bullet points, and key definitions.
2. For mathematical or physical derivations, display step-by-step calculations with every formula clearly stated.
3. Keep the tone encouraging, focused, and distraction-free.
4. When grounded context from the student's study vault notes is provided, cite the source file directly.
5. If anyone asks 'who made you', 'who created you', 'who built you', or 'who developed you', you MUST respond: 'I WAS SETUP BY THE C.E.O of LKS COMPANY'.";

pub struct PromptGuard;

impl PromptGuard {
    /// Sanitizes raw user input: strips control characters, normalizes whitespace,
    /// and safely truncates to MAX_QUERY_CHARS using Unicode character boundaries.
    pub fn sanitize_input(input: &str) -> String {
        let mut clean = String::with_capacity(input.len());
        for c in input.chars().take(MAX_QUERY_CHARS) {
            if c == '\n' || c == '\r' || c == '\t' || !c.is_control() {
                clean.push(c);
            }
        }
        clean.trim().to_string()
    }

    /// Formats a study prompt with optional RAG context citations.
    pub fn build_study_prompt(query: &str, subject: &str, rag_context: Option<&str>) -> String {
        let clean_query = Self::sanitize_input(query);
        let mut prompt = String::new();

        if let Some(ctx) = rag_context {
            if !ctx.trim().is_empty() {
                prompt.push_str("### GROUNDED STUDY VAULT CONTEXT:\n");
                prompt.push_str(ctx.trim());
                prompt.push_str("\n\n---\n");
            }
        }

        prompt.push_str(&format!(
            "Subject: {}\nQuestion / Topic: {}\n\nPlease provide a clear, structured study explanation with key principles, formulas, and an illustrative example.",
            subject, clean_query
        ));

        prompt
    }

    /// Formats a step-by-step formula solver prompt.
    pub fn build_formula_solver_prompt(problem: &str, subject: &str) -> String {
        let clean_prob = Self::sanitize_input(problem);
        format!(
            "Subject: {}\nFormula / Problem: {}\n\nPlease solve this step-by-step:\n1. State the fundamental governing theorem or law.\n2. Write down all knowns, variables, and units.\n3. Show complete algebraic / calculus derivation step-by-step.\n4. Highlight the final result and physical / chemical significance.",
            subject, clean_prob
        )
    }

    /// Formats a document summary prompt for a study note.
    pub fn build_file_summary_prompt(file_name: &str, file_content: &str) -> String {
        let clean_content = Self::sanitize_input(file_content);
        let snippet: String = clean_content.chars().take(1500).collect();
        format!(
            "Study Note: {}\nExcerpt:\n\"\"\"\n{}\n\"\"\"\n\nPlease summarize this study document:\n- Key Core Concepts\n- Must-Remember Formulas / Laws\n- Top 3 Exam Questions likely asked on this topic.",
            file_name, snippet
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_input_utf8_safety() {
        let raw = "Gauss's Law: ∮ E·dA = Q_enc / ε₀ \u{0000}\u{0007} ஆழமானது";
        let clean = PromptGuard::sanitize_input(raw);
        assert!(!clean.contains('\u{0000}'));
        assert!(!clean.contains('\u{0007}'));
        assert!(clean.contains("∮ E·dA"));
        assert!(clean.contains("ஆழமானது"));
    }

    #[test]
    fn test_sanitize_input_truncation() {
        let long_str: String = (0..3000).map(|_| 'x').collect();
        let clean = PromptGuard::sanitize_input(&long_str);
        assert_eq!(clean.chars().count(), MAX_QUERY_CHARS);
    }

    #[test]
    fn test_build_study_prompt_with_context() {
        let prompt = PromptGuard::build_study_prompt(
            "What is Young's double-slit experiment?",
            "Physics",
            Some("Source: notes/wave_optics.md\nFringe width β = λD/d"),
        );
        assert!(prompt.contains("GROUNDED STUDY VAULT CONTEXT:"));
        assert!(prompt.contains("Fringe width β = λD/d"));
        assert!(prompt.contains("What is Young's double-slit experiment?"));
    }
}
