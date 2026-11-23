//! Query Intent Classifier for MindSet Intelligence.
//! Categorizes queries into StudyExplanation, FormulaAndStepSolver, VaultSearch, and SystemCommand.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StudySubject {
    Physics,
    Chemistry,
    Mathematics,
    ComputerScience,
    TamilLiterature,
    General,
}

impl StudySubject {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Physics => "Physics",
            Self::Chemistry => "Chemistry",
            Self::Mathematics => "Mathematics",
            Self::ComputerScience => "Computer Science",
            Self::TamilLiterature => "Tamil Literature",
            Self::General => "General Studies",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemAction {
    EnableFocusMode,
    DisableFocusMode,
    SwitchDarkTheme,
    SwitchLightTheme,
    AddTask(String),
    LockSession,
    UnloadAiModel,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryIntent {
    StudyExplanation {
        subject: StudySubject,
        topic: String,
    },
    FormulaAndStepSolver {
        subject: StudySubject,
        problem: String,
    },
    VaultSearch {
        search_terms: String,
    },
    SystemCommand {
        action: SystemAction,
    },
}

pub struct IntentRouter;

impl IntentRouter {
    /// Classifies an incoming student query or command string.
    pub fn classify(query: &str) -> QueryIntent {
        let q_trimmed = query.trim();
        let q_lower = q_trimmed.to_lowercase();

        // 1. System Command detection
        if q_lower == "focus" || q_lower == "enable focus" || q_lower == "focus mode on" {
            return QueryIntent::SystemCommand {
                action: SystemAction::EnableFocusMode,
            };
        }
        if q_lower == "unfocus" || q_lower == "disable focus" || q_lower == "focus mode off" {
            return QueryIntent::SystemCommand {
                action: SystemAction::DisableFocusMode,
            };
        }
        if q_lower == "dark" || q_lower == "dark theme" || q_lower == "dark mode" {
            return QueryIntent::SystemCommand {
                action: SystemAction::SwitchDarkTheme,
            };
        }
        if q_lower == "light" || q_lower == "light theme" || q_lower == "light mode" {
            return QueryIntent::SystemCommand {
                action: SystemAction::SwitchLightTheme,
            };
        }
        if q_lower == "lock" || q_lower == "lock session" || q_lower == "lock screen" {
            return QueryIntent::SystemCommand {
                action: SystemAction::LockSession,
            };
        }
        if q_lower == "unload model" || q_lower == "free vram" || q_lower == "unload ai" {
            return QueryIntent::SystemCommand {
                action: SystemAction::UnloadAiModel,
            };
        }
        if q_lower.starts_with("add task ") {
            let rest = q_trimmed[9..].trim();
            return QueryIntent::SystemCommand {
                action: SystemAction::AddTask(rest.to_string()),
            };
        }
        if q_lower.starts_with("todo ") {
            let rest = q_trimmed[5..].trim();
            return QueryIntent::SystemCommand {
                action: SystemAction::AddTask(rest.to_string()),
            };
        }

        // 2. Vault / Note Search queries
        if q_lower.starts_with("search ")
            || q_lower.starts_with("find note ")
            || q_lower.starts_with("vault ")
        {
            let terms = q_trimmed
                .trim_start_matches("search ")
                .trim_start_matches("find note ")
                .trim_start_matches("vault ")
                .trim()
                .to_string();
            return QueryIntent::VaultSearch {
                search_terms: terms,
            };
        }

        // 3. Formula and Step-by-Step Solver queries
        let has_math_symbols = q_trimmed.contains('=')
            || q_trimmed.contains('+')
            || q_trimmed.contains('-')
            || q_trimmed.contains('*')
            || q_trimmed.contains('/')
            || q_trimmed.contains('^')
            || q_trimmed.contains("derive")
            || q_trimmed.contains("derivation")
            || q_trimmed.contains("calculate")
            || q_trimmed.contains("solve")
            || q_trimmed.contains("formula")
            || q_trimmed.contains("equation")
            || q_trimmed.contains("integral")
            || q_trimmed.contains("integrate")
            || q_trimmed.contains("derivative")
            || q_trimmed.contains("differentiate")
            || q_trimmed.contains("matrix")
            || q_trimmed.contains("eigen")
            || q_trimmed.contains("∫")
            || q_trimmed.contains("∮")
            || q_trimmed.contains("Δ")
            || q_trimmed.contains("√")
            || q_trimmed.contains("λ")
            || q_trimmed.contains("π")
            || q_trimmed.contains("θ")
            || q_trimmed.contains("ε");

        let is_chemistry = q_lower.contains("reaction")
            || q_lower.contains("electrode")
            || q_lower.contains("nernst")
            || q_lower.contains("equilibrium")
            || q_lower.contains("mole")
            || q_lower.contains("gibbs")
            || q_lower.contains("acid")
            || q_lower.contains("base")
            || q_lower.contains("molar")
            || q_lower.contains("redox")
            || q_lower.contains("ph ")
            || q_lower.contains("ph=");

        let is_physics = q_lower.contains("gauss")
            || q_lower.contains("flux")
            || q_lower.contains("electric")
            || q_lower.contains("magnetic")
            || q_lower.contains("optics")
            || q_lower.contains("velocity")
            || q_lower.contains("acceleration")
            || q_lower.contains("diffraction")
            || q_lower.contains("interference")
            || q_lower.contains("capacitance")
            || q_lower.contains("inductance")
            || q_lower.contains("resistance")
            || q_lower.contains("ohm")
            || q_lower.contains("wave");

        let is_tamil = q_trimmed
            .chars()
            .any(|c| (0x0B80..=0x0BFF).contains(&(c as u32)))
            || q_lower.contains("thirukkural")
            || q_lower.contains("kural")
            || q_lower.contains("sangam")
            || q_lower.contains("bharathi");

        let is_cs = q_lower.contains("rust")
            || q_lower.contains("python")
            || q_lower.contains("sqlite")
            || q_lower.contains("algorithm")
            || q_lower.contains("database")
            || q_lower.contains("binary tree")
            || q_lower.contains("graph")
            || q_lower.contains("async");

        if has_math_symbols {
            let subject = if is_chemistry {
                StudySubject::Chemistry
            } else if is_physics {
                StudySubject::Physics
            } else if is_cs {
                StudySubject::ComputerScience
            } else {
                StudySubject::Mathematics
            };
            return QueryIntent::FormulaAndStepSolver {
                subject,
                problem: q_trimmed.to_string(),
            };
        }

        // 4. Study Concept Explanation
        let subject = if is_tamil {
            StudySubject::TamilLiterature
        } else if is_chemistry {
            StudySubject::Chemistry
        } else if is_physics {
            StudySubject::Physics
        } else if is_cs {
            StudySubject::ComputerScience
        } else if q_lower.contains("calculus")
            || q_lower.contains("geometry")
            || q_lower.contains("algebra")
        {
            StudySubject::Mathematics
        } else {
            StudySubject::General
        };

        QueryIntent::StudyExplanation {
            subject,
            topic: q_trimmed.to_string(),
        }
    }

    /// Extracts formulas, equations, or mathematical expressions from user input or study text.
    /// Provides resilient extraction using delimiter matching, equation symbols, and fallback heuristics
    /// without crashing on malformed Unicode or non-matching inputs.
    pub fn extract_formulas(text: &str) -> Vec<String> {
        let mut formulas = Vec::new();

        // 1. Check for LaTeX-style math blocks: $$...$$, $...$
        let mut in_dollars = false;
        let mut dollar_start = 0;
        let chars: Vec<(usize, char)> = text.char_indices().collect();

        let mut i = 0;
        while i < chars.len() {
            let (idx, c) = chars[i];
            if c == '$' {
                if in_dollars {
                    let formula = text[dollar_start..idx].trim();
                    if !formula.is_empty()
                        && formula.chars().any(|ch| {
                            ch == '='
                                || ch == '+'
                                || ch == '-'
                                || ch == '/'
                                || ch == '^'
                                || ch == '\\'
                                || ch == '*'
                                || ch.is_alphanumeric()
                        })
                    {
                        formulas.push(formula.to_string());
                    }
                    in_dollars = false;
                } else {
                    in_dollars = true;
                    dollar_start = idx + 1;
                }
            }
            i += 1;
        }

        // 2. Line-by-line fallback extraction for standalone equations (e.g. "E = mc^2", "β = λD / d")
        for line in text.lines() {
            let trimmed = line.trim();
            // Skip markdown headings or empty lines
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            // Clean line of list bullets or markdown bolding
            let cleaned = trimmed.trim_start_matches(&['-', '*', '+', '•'][..]).trim();
            let unbolded = cleaned.replace("**", "").replace('`', "");
            let candidate = unbolded.trim();

            if candidate.contains('=') {
                let parts: Vec<&str> = candidate.split('=').collect();
                if parts.len() >= 2 {
                    let lhs = parts[0].trim();
                    let rhs = parts[1].trim();
                    // LHS should be a plausible variable/expression (e.g. <= 40 chars) and RHS non-empty
                    if !lhs.is_empty() && lhs.len() <= 40 && !rhs.is_empty() {
                        let expr = candidate.to_string();
                        if !formulas.contains(&expr) {
                            formulas.push(expr);
                        }
                    }
                }
            } else if candidate.contains("∮")
                || candidate.contains("∫")
                || candidate.contains("∑")
                || candidate.contains("lim_")
            {
                let expr = candidate.to_string();
                if !formulas.contains(&expr) {
                    formulas.push(expr);
                }
            }
        }

        formulas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_command_classification() {
        assert_eq!(
            IntentRouter::classify("enable focus"),
            QueryIntent::SystemCommand {
                action: SystemAction::EnableFocusMode
            }
        );
        assert_eq!(
            IntentRouter::classify("dark mode"),
            QueryIntent::SystemCommand {
                action: SystemAction::SwitchDarkTheme
            }
        );
        assert_eq!(
            IntentRouter::classify("add task Complete Chapter 4 Physics"),
            QueryIntent::SystemCommand {
                action: SystemAction::AddTask("Complete Chapter 4 Physics".to_string())
            }
        );
        assert_eq!(
            IntentRouter::classify("unload model"),
            QueryIntent::SystemCommand {
                action: SystemAction::UnloadAiModel
            }
        );
    }

    #[test]
    fn test_vault_search_classification() {
        assert_eq!(
            IntentRouter::classify("search wave optics"),
            QueryIntent::VaultSearch {
                search_terms: "wave optics".to_string()
            }
        );
        assert_eq!(
            IntentRouter::classify("vault electrochemistry"),
            QueryIntent::VaultSearch {
                search_terms: "electrochemistry".to_string()
            }
        );
    }

    #[test]
    fn test_formula_solver_classification() {
        let q = "derive Gauss's Law: ∮ E·dA = Q_enc / ε₀";
        match IntentRouter::classify(q) {
            QueryIntent::FormulaAndStepSolver { subject, .. } => {
                assert_eq!(subject, StudySubject::Physics);
            }
            _ => panic!("Expected FormulaAndStepSolver"),
        }

        let q2 = "calculate Nernst equation: E = E° - (RT/nF) ln Q";
        match IntentRouter::classify(q2) {
            QueryIntent::FormulaAndStepSolver { subject, .. } => {
                assert_eq!(subject, StudySubject::Chemistry);
            }
            _ => panic!("Expected FormulaAndStepSolver"),
        }
    }

    #[test]
    fn test_tamil_classification() {
        let q = "திருக்குறள் அதிகாரம் 1 விளக்கம்";
        match IntentRouter::classify(q) {
            QueryIntent::StudyExplanation { subject, .. } => {
                assert_eq!(subject, StudySubject::TamilLiterature);
            }
            _ => panic!("Expected StudyExplanation with TamilLiterature"),
        }
    }

    #[test]
    fn test_formula_extraction() {
        let sample = "Here is the key relationship: $E = mc^2$ and also:\n- β = λD / d\n- ∮ E · dA = Q_enc / ε₀\nNotice how it behaves.";
        let extracted = IntentRouter::extract_formulas(sample);
        assert!(extracted.contains(&"E = mc^2".to_string()));
        assert!(extracted.iter().any(|f| f.contains("β = λD / d")));
        assert!(extracted
            .iter()
            .any(|f| f.contains("∮ E · dA = Q_enc / ε₀")));
    }
}
