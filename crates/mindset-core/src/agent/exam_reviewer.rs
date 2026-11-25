//! MindSet OS — State Board 590/600 AI Exam Reviewer
//!
//! Evaluates student exam submissions (text and PDF documents) against official
//! state board scoring rubrics with high-achiever (590/600 standard) rigor.
//!
//! Key Rigor Deductions:
//! - Rule `DED-SI-UNIT`: -0.5 mark deduction for missing or incorrect SI units on final answers
//! - Rule `DED-DIAGRAM-LABEL`: -1.0 mark deduction for missing ray arrows, circuit loop directions, polarity, or labels
//! - Rule `DED-DERIVATION-STEP`: -0.5 mark deduction for skipped intermediate substitution or calculus steps
//! - Rule `DED-SIGNIFICANT-FIGS`: -0.5 mark deduction for precision truncation or improper rounding
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// 1. Error Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExamError {
    IoError(String),
    ParseError(String),
    RubricError(String),
    InvalidScore(String),
}

impl fmt::Display for ExamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExamError::IoError(msg) => write!(f, "Exam Reviewer I/O Error: {}", msg),
            ExamError::ParseError(msg) => write!(f, "Exam Reviewer Parse Error: {}", msg),
            ExamError::RubricError(msg) => write!(f, "Exam Reviewer Rubric Error: {}", msg),
            ExamError::InvalidScore(msg) => write!(f, "Exam Reviewer Invalid Score: {}", msg),
        }
    }
}

impl std::error::Error for ExamError {}

// ---------------------------------------------------------------------------
// 2. Rubric Data Structures
// ---------------------------------------------------------------------------

fn default_crown_threshold() -> f32 {
    98.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamRubric {
    pub board: String,
    pub examination: String,
    pub subject: String,
    pub subject_code: String,
    pub benchmark_standard: String,
    pub theory_max_marks: f32,
    pub passing_marks: f32,
    #[serde(default = "default_crown_threshold")]
    pub state_board_crown_threshold_percentage: f32,
    #[serde(default)]
    pub strict_rubric_deduction_rules: Vec<DeductionRule>,
    pub questions: Vec<RubricQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeductionRule {
    pub rule_id: String,
    pub name: String,
    pub deduction_marks: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricQuestion {
    pub question_number: u32,
    pub topic: String,
    pub max_marks: f32,
    #[serde(default)]
    pub section: Option<String>,
    pub criteria: Vec<RubricCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricCriterion {
    pub step: String,
    pub marks: f32,
    #[serde(default)]
    pub mandatory_keywords: Vec<String>,
}

// ---------------------------------------------------------------------------
// 3. Evaluation Sheet & Feedback Structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationSheet {
    pub student_name: Option<String>,
    pub roll_number: Option<String>,
    pub subject: String,
    pub examination: String,
    pub target_standard: String,
    pub total_score: f32,
    pub max_marks: f32,
    pub percentage: f32,
    pub achieved_590_crown: bool,
    pub itemized_scores: Vec<QuestionEvaluation>,
    pub section_breakdown: Vec<SectionScore>,
    pub total_deductions: f32,
    pub red_pen_annotations: Vec<String>,
    pub gap_analysis_590: GapAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionEvaluation {
    pub question_number: u32,
    pub topic: String,
    pub max_marks: f32,
    pub awarded_marks: f32,
    pub criteria_evaluations: Vec<CriterionEvaluation>,
    pub deductions: Vec<DeductionRecord>,
    pub red_pen_annotations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriterionStatus {
    Satisfied,
    Partial,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionEvaluation {
    pub step: String,
    pub max_marks: f32,
    pub awarded_marks: f32,
    pub status: CriterionStatus,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeductionRecord {
    pub rule_id: String,
    pub rule_name: String,
    pub penalty: f32,
    pub reason: String,
    pub question_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionScore {
    pub section_name: String,
    pub awarded_marks: f32,
    pub max_marks: f32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub target_percentage: f32,
    pub current_percentage: f32,
    pub marks_needed_for_590: f32,
    pub primary_loss_categories: Vec<LossCategorySummary>,
    pub actionable_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossCategorySummary {
    pub category: String,
    pub marks_lost: f32,
    pub occurrence_count: usize,
}

// ---------------------------------------------------------------------------
// 4. Student Paper Parsing Structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct StudentPaperMetadata {
    student_name: Option<String>,
    roll_number: Option<String>,
    target: Option<String>,
    subject: Option<String>,
}

#[derive(Debug, Clone)]
struct ParsedStudentQuestion {
    #[allow(dead_code)]
    number: u32,
    #[allow(dead_code)]
    title: String,
    full_text: String,
    statement_text: Option<String>,
    diagram_text: Option<String>,
    derivation_text: Option<String>,
    final_result_text: Option<String>,
    #[allow(dead_code)]
    units_text: Option<String>,
}

#[derive(Debug, Default)]
struct ExtractedQuestionSections {
    statement: Option<String>,
    diagram: Option<String>,
    derivation: Option<String>,
    final_result: Option<String>,
    units: Option<String>,
}

// ---------------------------------------------------------------------------
// 5. ExamReviewer Engine Implementation
// ---------------------------------------------------------------------------

pub struct ExamReviewer;

impl ExamReviewer {
    /// Loads an official State Board scoring rubric from a JSON file.
    pub fn load_rubric(path: &Path) -> Result<ExamRubric, ExamError> {
        let content = fs::read_to_string(path).map_err(|e| {
            ExamError::IoError(format!(
                "Failed to read rubric file {}: {}",
                path.display(),
                e
            ))
        })?;
        Self::load_rubric_from_str(&content)
    }

    /// Parses an official State Board scoring rubric from a JSON string.
    pub fn load_rubric_from_str(json_str: &str) -> Result<ExamRubric, ExamError> {
        serde_json::from_str(json_str).map_err(|e| {
            ExamError::RubricError(format!("Failed to deserialize rubric JSON: {}", e))
        })
    }

    /// Evaluates a student exam document (PDF or plaintext) against the provided rubric.
    pub fn evaluate(
        document_path: &Path,
        rubric: &ExamRubric,
    ) -> Result<EvaluationSheet, ExamError> {
        let text = if document_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pdf"))
            .unwrap_or(false)
        {
            // Read PDF bytes and parse via document engine
            let bytes = fs::read(document_path).map_err(|e| {
                ExamError::IoError(format!(
                    "Failed to read PDF file {}: {}",
                    document_path.display(),
                    e
                ))
            })?;
            Self::extract_text_from_pdf_bytes(&bytes)?
        } else {
            // Plain text or markdown
            fs::read_to_string(document_path).map_err(|e| {
                ExamError::IoError(format!(
                    "Failed to read exam file {}: {}",
                    document_path.display(),
                    e
                ))
            })?
        };

        Self::evaluate_text(&text, rubric)
    }

    /// Evaluates a student exam paper from in-memory PDF byte stream.
    pub fn evaluate_pdf_bytes(
        pdf_bytes: &[u8],
        rubric: &ExamRubric,
    ) -> Result<EvaluationSheet, ExamError> {
        let text = Self::extract_text_from_pdf_bytes(pdf_bytes)?;
        Self::evaluate_text(&text, rubric)
    }

    /// Extracts plaintext from PDF bytes using pure-Rust PDF decoding.
    fn extract_text_from_pdf_bytes(pdf_bytes: &[u8]) -> Result<String, ExamError> {
        let content = mindset_sys::fs::document::PdfEngine::parse(pdf_bytes)
            .map_err(|e| ExamError::ParseError(format!("Failed to parse PDF document: {}", e)))?;
        let text = content.extract_full_text();
        if text.trim().is_empty() {
            Err(ExamError::ParseError(
                "PDF parsing returned empty text".to_string(),
            ))
        } else {
            Ok(text)
        }
    }

    /// Evaluates raw student submission text against an official State Board scoring rubric.
    pub fn evaluate_text(
        student_text: &str,
        rubric: &ExamRubric,
    ) -> Result<EvaluationSheet, ExamError> {
        let metadata = Self::parse_metadata(student_text);
        let parsed_questions = Self::parse_student_questions(student_text);

        let mut itemized_scores = Vec::new();
        let mut all_red_pen_annotations = Vec::new();
        let mut total_awarded = 0.0f32;
        let mut total_max = 0.0f32;
        let mut total_deductions = 0.0f32;

        let mut loss_by_category: HashMap<String, (f32, usize)> = HashMap::new();

        for rubric_q in &rubric.questions {
            total_max += rubric_q.max_marks;
            let student_q = parsed_questions.get(&rubric_q.question_number);

            let (eval, q_loss_records) = Self::evaluate_question(rubric_q, student_q, rubric);

            total_awarded += eval.awarded_marks;
            for deduction in &eval.deductions {
                total_deductions += deduction.penalty;
            }

            for (cat, lost) in q_loss_records {
                let entry = loss_by_category.entry(cat).or_insert((0.0, 0));
                entry.0 += lost;
                entry.1 += 1;
            }

            for note in &eval.red_pen_annotations {
                all_red_pen_annotations.push(note.clone());
            }

            itemized_scores.push(eval);
        }

        let percentage = if total_max > 0.0 {
            (total_awarded / total_max) * 100.0
        } else {
            0.0
        };

        let achieved_590_crown = percentage >= rubric.state_board_crown_threshold_percentage;

        // Gap analysis computation
        let target_pct = rubric.state_board_crown_threshold_percentage;
        let crown_target_marks = (target_pct / 100.0) * total_max;
        let marks_needed = if achieved_590_crown {
            0.0
        } else {
            (crown_target_marks - total_awarded).max(0.0)
        };

        let mut primary_loss_categories = Vec::new();
        for (category, (marks_lost, count)) in loss_by_category {
            if marks_lost > 0.0 {
                primary_loss_categories.push(LossCategorySummary {
                    category,
                    marks_lost,
                    occurrence_count: count,
                });
            }
        }
        primary_loss_categories.sort_by(|a, b| {
            b.marks_lost
                .partial_cmp(&a.marks_lost)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut actionable_recommendations = Vec::new();
        if achieved_590_crown {
            actionable_recommendations.push(
                "Superb execution: Score meets the State Board 590/600 Crown criteria (>=98%). Maintain this level of precision on final board presentations.".to_string(),
            );
        } else {
            for loss in &primary_loss_categories {
                if loss.category.contains("SI Unit") {
                    actionable_recommendations.push(format!(
                        "Mandatory SI Units: Reclaim +{:.1} marks by writing explicit SI units (e.g., 'N m^2 C^-1', 'Tesla', 'Ohm') alongside every final boxed answer.",
                        loss.marks_lost
                    ));
                } else if loss.category.contains("Diagram") {
                    actionable_recommendations.push(format!(
                        "Diagram Annotations: Reclaim +{:.1} marks by always drawing branch current arrows, loop directions, and battery polarity signs (+/-).",
                        loss.marks_lost
                    ));
                } else if loss.category.contains("Derivation") {
                    actionable_recommendations.push(format!(
                        "Step-Wise Calculus: Reclaim +{:.1} marks by showing every intermediate substitution and surface/perimeter integral expansion.",
                        loss.marks_lost
                    ));
                }
            }
            if actionable_recommendations.is_empty() {
                actionable_recommendations.push(format!(
                    "Earn +{:.2} additional marks across descriptive sections to cross the 590/600 State Board Crown threshold.",
                    marks_needed
                ));
            }
        }

        // Section breakdown
        let section_breakdown = Self::compute_section_breakdown(&itemized_scores);

        Ok(EvaluationSheet {
            student_name: metadata.student_name,
            roll_number: metadata.roll_number,
            subject: rubric.subject.clone(),
            examination: rubric.examination.clone(),
            target_standard: rubric.benchmark_standard.clone(),
            total_score: (total_awarded * 100.0).round() / 100.0,
            max_marks: total_max,
            percentage: (percentage * 100.0).round() / 100.0,
            achieved_590_crown,
            itemized_scores,
            section_breakdown,
            total_deductions,
            red_pen_annotations: all_red_pen_annotations,
            gap_analysis_590: GapAnalysis {
                target_percentage: target_pct,
                current_percentage: (percentage * 100.0).round() / 100.0,
                marks_needed_for_590: (marks_needed * 100.0).round() / 100.0,
                primary_loss_categories,
                actionable_recommendations,
            },
        })
    }

    fn lookup_deduction_rule<'a>(
        rubric: &'a ExamRubric,
        rule_id: &str,
    ) -> Option<&'a DeductionRule> {
        rubric
            .strict_rubric_deduction_rules
            .iter()
            .find(|r| r.rule_id == rule_id)
    }

    fn normalize_math_spacing(text: &str) -> String {
        let lower = text.to_lowercase();
        let mut out = String::with_capacity(lower.len() + 16);
        let chars: Vec<char> = lower.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if matches!(c, '*' | '/' | '=' | '+' | '×') {
                if !out.ends_with(' ') {
                    out.push(' ');
                }
                out.push(if c == '×' { 'x' } else { c });
                if i + 1 < chars.len() && chars[i + 1] != ' ' {
                    out.push(' ');
                }
            } else {
                out.push(c);
            }
        }
        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn contains_concept_token(haystack_norm: &str, raw_kw: &str) -> bool {
        let kw = Self::normalize_math_spacing(raw_kw.trim());
        if kw.is_empty() {
            return false;
        }
        if kw.len() <= 3 && kw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            // Boundary-aware match for short physics symbols (e.g. "e", "q", "p", "ds", "dl")
            let bytes = haystack_norm.as_bytes();
            let kw_bytes = kw.as_bytes();
            let mut start = 0;
            while let Some(pos) = haystack_norm[start..].find(&kw) {
                let abs = start + pos;
                let end = abs + kw_bytes.len();
                let left_ok = abs == 0 || !bytes[abs - 1].is_ascii_alphanumeric();
                let right_ok = end == bytes.len()
                    || (!bytes[end].is_ascii_alphanumeric() && (bytes[end] != b'_' || kw == "q"));
                if left_ok && right_ok {
                    return true;
                }
                start = abs + 1;
            }
            false
        } else {
            haystack_norm.contains(&kw)
        }
    }

    fn extract_step_keywords(step: &str) -> Vec<String> {
        const STOPWORDS: &[&str] = &[
            "of",
            "the",
            "with",
            "and",
            "in",
            "on",
            "for",
            "to",
            "a",
            "an",
            "or",
            "is",
            "statement",
            "diagram",
            "derivation",
            "integration",
            "steps",
            "step",
            "final",
            "scalar",
            "vector",
            "equation",
            "standard",
            "si",
            "units",
            "unit",
            "cross",
            "product",
            "magnitude",
            "special",
            "cases",
            "under",
            "balanced",
            "condition",
            "along",
            "loop",
            "perimeter",
            "coordinate",
            "relation",
            "closed",
            "formula",
            "differential",
            "with",
            "and",
        ];

        let norm = Self::normalize_math_spacing(step);
        let mut out = Vec::new();

        for raw_tok in norm.split(|c: char| {
            c.is_whitespace() || matches!(c, ':' | ',' | ';' | '(' | ')' | '[' | ']' | '/')
        }) {
            let tok = raw_tok.trim_matches(|c: char| c == '.');
            if tok.is_empty() || STOPWORDS.contains(&tok) {
                continue;
            }
            if tok.len() == 1 && !matches!(tok, "p" | "q" | "r" | "s" | "e" | "b" | "i" | "x" | "n")
            {
                continue;
            }
            let normalized_kw = match tok {
                "gauss's" | "gauss" => "gauss",
                "q_enclosed" => "q",
                "closed_integral" => "oint",
                "integral" => "oint",
                _ => tok,
            };
            if !out.iter().any(|k: &String| k == normalized_kw) {
                out.push(normalized_kw.to_string());
            }
        }
        out
    }

    fn evaluate_criterion(
        criterion: &RubricCriterion,
        sq: &ParsedStudentQuestion,
    ) -> CriterionEvaluation {
        let step_lower = criterion.step.to_lowercase();
        let has_explicit_kw = !criterion.mandatory_keywords.is_empty();

        let required_keywords: Vec<String> = if has_explicit_kw {
            criterion
                .mandatory_keywords
                .iter()
                .map(|k| k.to_lowercase())
                .collect()
        } else {
            Self::extract_step_keywords(&criterion.step)
        };

        let is_diagram = step_lower.contains("diagram");
        let is_statement = step_lower.contains("statement");
        let is_final = step_lower.contains("final")
            || step_lower.contains("special")
            || ((step_lower.contains("balance") || step_lower.contains("equation"))
                && !step_lower.contains("derivation")
                && !step_lower.contains("loop"));
        let is_derivation = !is_final
            && (step_lower.contains("integration")
                || step_lower.contains("derivation")
                || step_lower.contains("loop")
                || step_lower.contains("magnitude")
                || step_lower.contains("torque"));

        let (structural_gate, target_text): (bool, String) = if is_diagram {
            let gate = sq.diagram_text.is_some();
            let txt = sq.diagram_text.clone().unwrap_or_default();
            (gate, txt)
        } else if is_statement {
            let stmt_txt = sq.statement_text.as_deref().unwrap_or(&sq.full_text);
            let has_substantive_stmt = sq.statement_text.is_some()
                || stmt_txt.lines().any(|l| {
                    l.trim().len() >= 12
                        && (l.contains('=')
                            || l.to_lowercase().contains("law")
                            || l.to_lowercase().contains("flux")
                            || l.to_lowercase().contains("rule"))
                });
            let combined = if has_explicit_kw {
                stmt_txt.to_string()
            } else {
                format!("{} {}", sq.title, stmt_txt)
            };
            (has_substantive_stmt, combined)
        } else if is_final {
            let final_txt = sq.final_result_text.as_deref().unwrap_or(&sq.full_text);
            let has_res = sq.final_result_text.is_some()
                || final_txt.contains('=')
                || final_txt.to_lowercase().contains("theta");
            let combined = if sq.final_result_text.is_some() {
                format!("{} {}", final_txt, sq.units_text.as_deref().unwrap_or(""))
            } else {
                sq.full_text.clone()
            };
            (has_res, combined)
        } else if is_derivation {
            let deriv_txt = sq.derivation_text.as_deref().unwrap_or(&sq.full_text);
            let has_alg = sq.derivation_text.is_some()
                || deriv_txt.contains('=')
                || deriv_txt.to_lowercase().contains("sin(")
                || deriv_txt.to_lowercase().contains("cos(")
                || deriv_txt.to_lowercase().contains("oint");
            (has_alg, deriv_txt.to_string())
        } else {
            let non_empty = !sq.full_text.trim().is_empty();
            (non_empty, sq.full_text.clone())
        };

        if !structural_gate || target_text.trim().is_empty() {
            let missing_note = if is_diagram {
                "Scientific schematic or circuit diagram missing.".to_string()
            } else if is_statement {
                "Statement of theorem or scientific law omitted.".to_string()
            } else if is_derivation {
                "Algebraic or calculus derivation steps missing.".to_string()
            } else {
                "Required answer content missing for rubric criterion.".to_string()
            };
            return CriterionEvaluation {
                step: criterion.step.clone(),
                max_marks: criterion.marks,
                awarded_marks: 0.0,
                status: CriterionStatus::Missing,
                notes: missing_note,
            };
        }

        let target_norm = Self::normalize_math_spacing(&target_text);
        let mut matched = 0usize;
        let mut missing_kws = Vec::new();

        for kw in &required_keywords {
            if Self::contains_concept_token(&target_norm, kw) {
                matched += 1;
            } else {
                missing_kws.push(kw.clone());
            }
        }

        let ratio = if required_keywords.is_empty() {
            if structural_gate {
                1.0
            } else {
                0.0
            }
        } else {
            matched as f32 / required_keywords.len() as f32
        };

        let (marks_awarded, status, note) = if ratio >= 0.75 {
            (
                criterion.marks,
                CriterionStatus::Satisfied,
                "Full step-wise credit verified against mandatory concepts.".to_string(),
            )
        } else if ratio >= 0.35 {
            let partial = ((criterion.marks * ratio) * 4.0).round() / 4.0;
            (
                partial.min(criterion.marks),
                CriterionStatus::Partial,
                format!(
                    "Partial credit ({}/{} concepts matched; missing: {}).",
                    matched,
                    required_keywords.len(),
                    missing_kws.join(", ")
                ),
            )
        } else {
            (
                0.0,
                CriterionStatus::Missing,
                format!(
                    "Insufficient conceptual coverage ({}/{} required keywords: {}).",
                    matched,
                    required_keywords.len(),
                    missing_kws.join(", ")
                ),
            )
        };

        CriterionEvaluation {
            step: criterion.step.clone(),
            max_marks: criterion.marks,
            awarded_marks: marks_awarded,
            status,
            notes: note,
        }
    }

    /// Evaluates an individual question against its rubric criteria and applies strict procedural deductions.
    fn evaluate_question(
        rubric_q: &RubricQuestion,
        student_q: Option<&ParsedStudentQuestion>,
        rubric: &ExamRubric,
    ) -> (QuestionEvaluation, Vec<(String, f32)>) {
        let mut criteria_evals = Vec::new();
        let mut deductions = Vec::new();
        let mut red_pen_notes = Vec::new();
        let mut loss_records = Vec::new();

        let Some(sq) = student_q else {
            // Unattempted question
            for c in &rubric_q.criteria {
                criteria_evals.push(CriterionEvaluation {
                    step: c.step.clone(),
                    max_marks: c.marks,
                    awarded_marks: 0.0,
                    status: CriterionStatus::Missing,
                    notes: "Unattempted question.".to_string(),
                });
                loss_records.push(("Unattempted Criteria".to_string(), c.marks));
            }
            red_pen_notes.push(format!(
                "Q{}: Unattempted. 0.0/{:.1} marks.",
                rubric_q.question_number, rubric_q.max_marks
            ));
            return (
                QuestionEvaluation {
                    question_number: rubric_q.question_number,
                    topic: rubric_q.topic.clone(),
                    max_marks: rubric_q.max_marks,
                    awarded_marks: 0.0,
                    criteria_evaluations: criteria_evals,
                    deductions: Vec::new(),
                    red_pen_annotations: red_pen_notes,
                },
                loss_records,
            );
        };

        let mut criteria_sum = 0.0f32;

        for criterion in &rubric_q.criteria {
            let eval = Self::evaluate_criterion(criterion, sq);
            if eval.awarded_marks < criterion.marks {
                let lost = criterion.marks - eval.awarded_marks;
                loss_records.push(("Incomplete Criteria".to_string(), lost));
            }
            criteria_sum += eval.awarded_marks;
            criteria_evals.push(eval);
        }

        // -----------------------------------------------------------------------
        // Strict Procedural Deductions (Dynamic from rubric.strict_rubric_deduction_rules)
        // -----------------------------------------------------------------------

        // Deduction 1: Missing or Incorrect SI Unit (DED-SI-UNIT)
        if let Some(rule) = Self::lookup_deduction_rule(rubric, "DED-SI-UNIT") {
            let _rubric_expects_unit = rubric_q.criteria.iter().any(|c| {
                let s = c.step.to_lowercase();
                s.contains("unit")
                    || s.contains("si ")
                    || s.contains("tesla")
                    || s.contains("n m^2 c^-1")
                    || s.contains("ohm")
                    || s.contains("joule")
                    || s.contains("watt")
                    || s.contains("newton")
                    || s.contains("farad")
                    || s.contains("coulomb")
            });

            let result_text = sq.final_result_text.as_deref().unwrap_or(&sq.full_text);
            let has_numerical_final = result_text.lines().any(|line| {
                let l = line.trim().to_lowercase();
                if l.is_empty()
                    || l.starts_with("case ")
                    || l.starts_with("theta")
                    || l.starts_with("from (")
                {
                    return false;
                }
                let has_digit = l.chars().any(|c| c.is_ascii_digit());
                if !has_digit {
                    return false;
                }
                let has_decimal_digits = l
                    .as_bytes()
                    .windows(3)
                    .any(|w| w[0].is_ascii_digit() && w[1] == b'.' && w[2].is_ascii_digit());
                let eq_rhs_has_digit = l
                    .split('=')
                    .nth(1)
                    .is_some_and(|rhs| rhs.chars().any(|c| c.is_ascii_digit()));
                l.contains("10^") || has_decimal_digits || eq_rhs_has_digit
            });

            if has_numerical_final && !Self::has_valid_si_unit(sq) {
                let penalty = rule.deduction_marks;
                deductions.push(DeductionRecord {
                    rule_id: rule.rule_id.clone(),
                    rule_name: rule.name.clone(),
                    penalty,
                    reason: "Final numerical or derived physical quantity lacks explicit standard SI unit."
                        .to_string(),
                    question_number: rubric_q.question_number,
                });
                red_pen_notes.push(format!(
                    "Q{}: [-{:.1} mark] Red Pen: Missing standard SI unit on final numerical value.",
                    rubric_q.question_number, penalty
                ));
                loss_records.push(("Missing SI Units".to_string(), penalty));
            }
        }

        // Deduction 2: Unlabelled or Incomplete Scientific Diagram (DED-DIAGRAM-LABEL)
        if let Some(rule) = Self::lookup_deduction_rule(rubric, "DED-DIAGRAM-LABEL") {
            let diagram_criteria: Vec<&RubricCriterion> = rubric_q
                .criteria
                .iter()
                .filter(|c| {
                    let l = c.step.to_lowercase();
                    l.contains("diagram") || l.contains("circuit")
                })
                .collect();

            if !diagram_criteria.is_empty() {
                if let Some(diag_raw) = &sq.diagram_text {
                    let diag = diag_raw.to_lowercase();
                    let rubric_diag_desc = diagram_criteria
                        .iter()
                        .map(|c| c.step.to_lowercase())
                        .collect::<Vec<_>>()
                        .join(" ");

                    let is_circuit_diagram = rubric_diag_desc.contains("circuit")
                        || rubric_diag_desc.contains("bridge")
                        || rubric_diag_desc.contains("galvanometer")
                        || rubric_diag_desc.contains("wheatstone");
                    let is_vector_or_ray = rubric_diag_desc.contains("vector")
                        || rubric_diag_desc.contains("arrow")
                        || rubric_diag_desc.contains("ray");

                    let missing_labels = if is_circuit_diagram {
                        let has_current_dir = diag.contains("current")
                            || diag.contains("direction")
                            || diag.contains("arrow")
                            || diag.contains("->")
                            || diag.contains("i1")
                            || diag.contains("i2")
                            || diag.contains("clockwise");
                        let has_polarity_or_source = diag.contains("polarity")
                            || diag.contains("(+/-)")
                            || diag.contains("battery")
                            || diag.contains("emf")
                            || diag.contains("terminal");
                        !has_current_dir || !has_polarity_or_source
                    } else if is_vector_or_ray {
                        let has_direction = diag.contains("arrow")
                            || diag.contains("arrows")
                            || diag.contains("->")
                            || diag.contains("outward")
                            || diag.contains("inward")
                            || diag.contains("radial")
                            || diag.contains("direction")
                            || diag.contains("vector");
                        !has_direction
                    } else {
                        let has_coord_labels = diag.contains("radius")
                            || diag.contains("axis")
                            || diag.contains("distance")
                            || diag.contains("theta");
                        !has_coord_labels
                    };

                    if missing_labels {
                        let penalty = rule.deduction_marks;
                        deductions.push(DeductionRecord {
                            rule_id: rule.rule_id.clone(),
                            rule_name: rule.name.clone(),
                            penalty,
                            reason: "Scientific schematic missing directional vectors/arrows, circuit current directions, or battery polarity labels."
                                .to_string(),
                            question_number: rubric_q.question_number,
                        });
                        red_pen_notes.push(format!(
                            "Q{}: [-{:.1} mark] Red Pen: Diagram incomplete - Missing directional arrows or polarity (+/-) labels.",
                            rubric_q.question_number, penalty
                        ));
                        loss_records.push(("Diagram Incompleteness".to_string(), penalty));
                    }
                }
            }
        }

        // Deduction 3: Skipped Intermediate Derivation Step (DED-DERIVATION-STEP)
        if let Some(rule) = Self::lookup_deduction_rule(rubric, "DED-DERIVATION-STEP") {
            let requires_integration = rubric_q.criteria.iter().any(|c| {
                let l = c.step.to_lowercase();
                l.contains("integration") || l.contains("integral")
            });
            let requires_loop_deriv = rubric_q.criteria.iter().any(|c| {
                let l = c.step.to_lowercase();
                l.contains("loop") && l.contains("derivation")
            });

            if (requires_integration || requires_loop_deriv) && criteria_sum > 0.0 {
                if let Some(deriv_raw) = &sq.derivation_text {
                    let deriv_lower = Self::normalize_math_spacing(deriv_raw);
                    let eq_lines = deriv_raw.lines().filter(|l| l.contains('=')).count();

                    let skipped_integration = requires_integration
                        && (eq_lines < 2
                            || (!deriv_lower.contains("4 * pi * r^2")
                                && !deriv_lower.contains("2 * pi * r")
                                && !deriv_lower.contains("ds * cos")
                                && !deriv_lower.contains("sin(")));

                    let skipped_loop =
                        requires_loop_deriv && (eq_lines < 2 || !deriv_lower.contains("ig = 0"));

                    if skipped_integration || skipped_loop {
                        let penalty = rule.deduction_marks;
                        deductions.push(DeductionRecord {
                            rule_id: rule.rule_id.clone(),
                            rule_name: rule.name.clone(),
                            penalty,
                            reason: "Skipped intermediate algebraic expansion, differential substitution, or null-deflection equation."
                                .to_string(),
                            question_number: rubric_q.question_number,
                        });
                        red_pen_notes.push(format!(
                            "Q{}: [-{:.1} mark] Red Pen: Skipped intermediate substitution or calculus step.",
                            rubric_q.question_number, penalty
                        ));
                        loss_records.push(("Skipped Derivation Steps".to_string(), penalty));
                    }
                }
            }
        }

        // Deduction 4: Improper Rounding / Significant Figures Truncation (DED-SIGNIFICANT-FIGS)
        if let Some(rule) = Self::lookup_deduction_rule(rubric, "DED-SIGNIFICANT-FIGS") {
            if let Some(final_txt) = &sq.final_result_text {
                let final_lower = final_txt.to_lowercase();
                let deriv_has_decimals = sq.derivation_text.as_deref().unwrap_or("").contains('.');
                let truncated_sci = final_lower.lines().any(|l| {
                    let trimmed = l.trim();
                    // Flag 1-sig-fig scientific answers like "= 1 * 10^5" or explicit "~ 6" rounding
                    let has_approx = trimmed.contains("approx") || trimmed.contains('~');
                    let single_digit_sci = trimmed.split("* 10^").next().is_some_and(|lhs| {
                        let tail = lhs.split('=').next_back().unwrap_or("").trim();
                        !tail.is_empty()
                            && tail.len() == 1
                            && tail.chars().all(|c| c.is_ascii_digit())
                    });
                    (has_approx && !trimmed.contains('.'))
                        || (deriv_has_decimals && single_digit_sci)
                });

                if truncated_sci {
                    let penalty = rule.deduction_marks;
                    deductions.push(DeductionRecord {
                        rule_id: rule.rule_id.clone(),
                        rule_name: rule.name.clone(),
                        penalty,
                        reason: "Final numerical answer suffers premature precision truncation (< 2 significant decimals)."
                            .to_string(),
                        question_number: rubric_q.question_number,
                    });
                    red_pen_notes.push(format!(
                        "Q{}: [-{:.1} mark] Red Pen: Precision truncation / insufficient significant figures.",
                        rubric_q.question_number, penalty
                    ));
                    loss_records.push(("Significant Figures Precision".to_string(), penalty));
                }
            }
        }

        // Total question deductions capped at criteria sum
        let total_q_deductions: f32 = deductions.iter().map(|d| d.penalty).sum();
        let final_q_awarded = (criteria_sum - total_q_deductions).max(0.0);

        if deductions.is_empty() && final_q_awarded >= rubric_q.max_marks {
            red_pen_notes.push(format!(
                "Q{}: Full marks {:.1}/{:.1} awarded. Rigorous derivation with verified SI units.",
                rubric_q.question_number, final_q_awarded, rubric_q.max_marks
            ));
        }

        (
            QuestionEvaluation {
                question_number: rubric_q.question_number,
                topic: rubric_q.topic.clone(),
                max_marks: rubric_q.max_marks,
                awarded_marks: (final_q_awarded * 100.0).round() / 100.0,
                criteria_evaluations: criteria_evals,
                deductions,
                red_pen_annotations: red_pen_notes,
            },
            loss_records,
        )
    }

    /// Checks if a question's final answer or units block contains an explicit valid standard SI unit.
    fn has_valid_si_unit(sq: &ParsedStudentQuestion) -> bool {
        let valid_units = [
            "n m^2 c^-1",
            "n m^2/c",
            "n m2 c-1",
            "n/c",
            "v/m",
            "tesla",
            "(t)",
            "wb m^-2",
            "wb/m^2",
            "weber",
            "ohm",
            "farad",
            "coulomb",
            "joule",
            "watt",
            "volt",
            "newton",
            "ampere",
            "henry",
            "hertz",
            "m/s",
            "j/k",
        ];

        let mut search_scope = String::new();
        if let Some(ref f) = sq.final_result_text {
            search_scope.push_str(f);
            search_scope.push('\n');
        }
        if let Some(ref u) = sq.units_text {
            search_scope.push_str(u);
            search_scope.push('\n');
        }
        if search_scope.is_empty() {
            let tail: Vec<&str> = sq.full_text.lines().rev().take(4).collect();
            for line in tail.into_iter().rev() {
                search_scope.push_str(line);
                search_scope.push('\n');
            }
        }

        let lower = search_scope.to_lowercase();
        valid_units.iter().any(|unit| lower.contains(unit))
    }

    /// Parses student metadata from the header of an exam paper.
    fn parse_metadata(text: &str) -> StudentPaperMetadata {
        let mut meta = StudentPaperMetadata::default();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.to_lowercase().starts_with("student name:") {
                let name_part = &trimmed["student name:".len()..].trim();
                if let Some(idx) = name_part.find('(') {
                    meta.student_name = Some(name_part[..idx].trim().to_string());
                } else {
                    meta.student_name = Some(name_part.to_string());
                }
            }
            if trimmed.to_lowercase().contains("roll no:") {
                if let Some(idx) = trimmed.to_lowercase().find("roll no:") {
                    let after = &trimmed[idx + "roll no:".len()..];
                    let roll = after.trim_matches(|c: char| c == ' ' || c == ')' || c == ':');
                    meta.roll_number = Some(roll.to_string());
                }
            }
            if trimmed.to_lowercase().starts_with("target:") {
                meta.target = Some(trimmed["target:".len()..].trim().to_string());
            }
            if trimmed.to_lowercase().starts_with("subject:") {
                meta.subject = Some(trimmed["subject:".len()..].trim().to_string());
            }
        }
        meta
    }

    /// Splits student text into individual question response blocks.
    fn parse_student_questions(text: &str) -> HashMap<u32, ParsedStudentQuestion> {
        let mut map = HashMap::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if let Some((q_num, title)) = Self::extract_question_header(line) {
                let mut q_lines = Vec::new();
                i += 1;
                while i < lines.len() {
                    let next_line = lines[i].trim();
                    if Self::extract_question_header(next_line).is_some()
                        || next_line.starts_with("==========") && q_lines.len() > 5
                    {
                        break;
                    }
                    q_lines.push(lines[i]);
                    i += 1;
                }

                let full_text = q_lines.join("\n");
                let sections = Self::extract_question_sections(&full_text);

                map.insert(
                    q_num,
                    ParsedStudentQuestion {
                        number: q_num,
                        title,
                        full_text,
                        statement_text: sections.statement,
                        diagram_text: sections.diagram,
                        derivation_text: sections.derivation,
                        final_result_text: sections.final_result,
                        units_text: sections.units,
                    },
                );
            } else {
                i += 1;
            }
        }

        map
    }

    /// Identifies whether a line is a question header (e.g. `--- Question 1: Gauss's Law ---`).
    fn extract_question_header(line: &str) -> Option<(u32, String)> {
        let trimmed = line.trim();
        let stripped = trimmed.trim_matches(|c: char| c == '-' || c == '=');
        let lower = stripped.trim().to_lowercase();

        if lower.starts_with("question ") || lower.starts_with("q") {
            let remainder = if lower.starts_with("question ") {
                &stripped.trim()["question ".len()..]
            } else {
                &stripped.trim()["q".len()..]
            };

            // Parse digits
            let mut num_str = String::new();
            let mut title_part = "";
            for (idx, ch) in remainder.char_indices() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                } else if ch == ':' || ch == '-' || ch == '.' || ch == ' ' {
                    title_part =
                        remainder[idx..].trim_matches(|c: char| c == ':' || c == '-' || c == ' ');
                    break;
                } else {
                    break;
                }
            }

            if let Ok(num) = num_str.parse::<u32>() {
                return Some((num, title_part.to_string()));
            }
        }
        None
    }

    /// Extracts actual multi-line structural sections (Statement, Diagram, Derivation, Final Result, Units)
    /// from a question's response.
    fn extract_question_sections(text: &str) -> ExtractedQuestionSections {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum SecKind {
            None,
            Statement,
            StatementAndDerivation,
            Diagram,
            Derivation,
            FinalResult,
            Units,
        }

        let mut stmt_lines = Vec::new();
        let mut diag_lines = Vec::new();
        let mut deriv_lines = Vec::new();
        let mut final_lines = Vec::new();
        let mut units_lines = Vec::new();

        let mut active = SecKind::None;
        let mut in_bracket_diagram = false;

        for line in text.lines() {
            let trimmed = line.trim();
            let lower = trimmed.to_lowercase();

            if in_bracket_diagram {
                diag_lines.push(line);
                if trimmed.contains(']') {
                    in_bracket_diagram = false;
                }
                continue;
            }

            if lower.starts_with("statement & derivation:") {
                active = SecKind::StatementAndDerivation;
                let rest = trimmed["statement & derivation:".len()..].trim();
                if !rest.is_empty() {
                    stmt_lines.push(rest);
                    deriv_lines.push(rest);
                }
                continue;
            } else if lower.starts_with("statement:") || lower.starts_with("statement of") {
                active = SecKind::Statement;
                if let Some((_, rest)) = trimmed.split_once(':') {
                    let r = rest.trim();
                    if !r.is_empty() {
                        stmt_lines.push(r);
                    }
                } else {
                    stmt_lines.push(trimmed);
                }
                continue;
            } else if lower.starts_with("[diagram:") {
                active = SecKind::Diagram;
                diag_lines.push(line);
                if !trimmed.contains(']') {
                    in_bracket_diagram = true;
                }
                continue;
            } else if lower.starts_with("diagram:") {
                active = SecKind::Diagram;
                let rest = trimmed["diagram:".len()..].trim();
                if !rest.is_empty() {
                    diag_lines.push(rest);
                    if rest.starts_with('[') && !rest.contains(']') {
                        in_bracket_diagram = true;
                    }
                }
                continue;
            } else if lower.starts_with("derivation")
                || lower.starts_with("integrating ")
                || lower.starts_with("applying kcl")
                || lower.starts_with("biot-savart law:")
                || lower.starts_with("torque on dipole:")
            {
                active = SecKind::Derivation;
                if let Some((_, rest)) = trimmed.split_once(':') {
                    let r = rest.trim();
                    if !r.is_empty() {
                        deriv_lines.push(r);
                    }
                } else {
                    deriv_lines.push(trimmed);
                }
                continue;
            } else if lower.starts_with("final result:")
                || lower.starts_with("final answer:")
                || lower.starts_with("special cases:")
                || lower.starts_with("hence,")
            {
                active = SecKind::FinalResult;
                if lower.starts_with("hence,") {
                    final_lines.push(trimmed);
                } else if let Some((_, rest)) = trimmed.split_once(':') {
                    let r = rest.trim();
                    if !r.is_empty() {
                        final_lines.push(r);
                    }
                }
                continue;
            } else if lower.starts_with("units:") {
                active = SecKind::Units;
                let rest = trimmed["units:".len()..].trim();
                if !rest.is_empty() {
                    units_lines.push(rest);
                }
                continue;
            } else if trimmed.starts_with("=====") {
                active = SecKind::None;
                continue;
            }

            match active {
                SecKind::Statement => stmt_lines.push(line),
                SecKind::StatementAndDerivation => {
                    stmt_lines.push(line);
                    deriv_lines.push(line);
                }
                SecKind::Diagram => {
                    if trimmed.starts_with("[DIAGRAM:") {
                        diag_lines.push(line);
                        if !trimmed.contains(']') {
                            in_bracket_diagram = true;
                        }
                    } else if !trimmed.is_empty() {
                        diag_lines.push(line);
                    }
                }
                SecKind::Derivation => deriv_lines.push(line),
                SecKind::FinalResult => final_lines.push(line),
                SecKind::Units => units_lines.push(line),
                SecKind::None => {
                    if lower.contains("[diagram:") {
                        diag_lines.push(line);
                        if !trimmed.contains(']') {
                            in_bracket_diagram = true;
                        }
                    }
                }
            }
        }

        let to_opt = |lines: Vec<&str>| -> Option<String> {
            let joined = lines.join("\n");
            if joined.trim().is_empty() {
                None
            } else {
                Some(joined.trim().to_string())
            }
        };

        ExtractedQuestionSections {
            statement: to_opt(stmt_lines),
            diagram: to_opt(diag_lines),
            derivation: to_opt(deriv_lines),
            final_result: to_opt(final_lines),
            units: to_opt(units_lines),
        }
    }

    /// Groups itemized question scores into broad examination sections.
    fn compute_section_breakdown(itemized: &[QuestionEvaluation]) -> Vec<SectionScore> {
        let mut sec_b_awarded = 0.0f32;
        let mut sec_b_max = 0.0f32;
        let mut sec_c_awarded = 0.0f32;
        let mut sec_c_max = 0.0f32;

        for q in itemized {
            if q.max_marks <= 3.0 {
                sec_b_awarded += q.awarded_marks;
                sec_b_max += q.max_marks;
            } else {
                sec_c_awarded += q.awarded_marks;
                sec_c_max += q.max_marks;
            }
        }

        let mut out = Vec::new();
        if sec_b_max > 0.0 {
            out.push(SectionScore {
                section_name: "Section B (Short Answer & 3-Mark Numericals)".to_string(),
                awarded_marks: sec_b_awarded,
                max_marks: sec_b_max,
                percentage: (sec_b_awarded / sec_b_max) * 100.0,
            });
        }
        if sec_c_max > 0.0 {
            out.push(SectionScore {
                section_name: "Section C (Long Derivations & 5-Mark Theoretic Problems)"
                    .to_string(),
                awarded_marks: sec_c_awarded,
                max_marks: sec_c_max,
                percentage: (sec_c_awarded / sec_c_max) * 100.0,
            });
        }
        out
    }
}

// ---------------------------------------------------------------------------
// 6. Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RUBRIC_JSON: &str = r#"{
      "board": "Tamil Nadu State Board",
      "examination": "Higher Secondary Examination (Class 12)",
      "subject": "Physics",
      "subject_code": "PHY-12-TN",
      "benchmark_standard": "590/600 High Achiever Ranker Cutoff",
      "theory_max_marks": 70,
      "passing_marks": 25,
      "state_board_crown_threshold_percentage": 98.0,
      "strict_rubric_deduction_rules": [
        {
          "rule_id": "DED-SI-UNIT",
          "name": "Missing or Incorrect SI Unit",
          "deduction_marks": 0.5,
          "description": "Mandatory 0.5 mark deduction if final numerical lacks explicit SI unit."
        },
        {
          "rule_id": "DED-DIAGRAM-LABEL",
          "name": "Unlabelled or Incomplete Scientific Diagram",
          "deduction_marks": 1.0,
          "description": "Mandatory 1.0 mark deduction for missing arrows or polarity signs."
        }
      ],
      "questions": [
        {
          "question_number": 1,
          "topic": "Gauss's Law & Electric Flux",
          "max_marks": 5.0,
          "criteria": [
            { "step": "Statement of Gauss's Law", "marks": 1.0 },
            { "step": "Closed Gaussian surface diagram with dS vector and E vector arrows", "marks": 1.0 },
            { "step": "Integration steps: flux = closed_integral(E . dS) = q_enclosed / epsilon_0", "marks": 2.0 },
            { "step": "Final scalar/vector equation with standard SI units (N m^2 C^-1)", "marks": 1.0 }
          ]
        },
        {
          "question_number": 2,
          "topic": "Electric Dipole in Uniform Field",
          "max_marks": 3.0,
          "criteria": [
            { "step": "Torque equation derivation: tau = p x E", "marks": 1.5 },
            { "step": "Cross product magnitude tau = p * E * sin(theta)", "marks": 0.5 },
            { "step": "Special cases: stable equilibrium and maximum torque", "marks": 1.0 }
          ]
        }
      ]
    }"#;

    const PERFECT_STUDENT_TEXT: &str = r#"
TAMIL NADU HIGHER SECONDARY EXAMINATION - CLASS 12 PHYSICS
STUDENT NAME: Arjun R (Roll No: 1240892)
TARGET: 590/600 (State Board Merit Rank)
SUBJECT: Physics (Theory)

--- Question 1: Gauss's Law & Electric Flux ---
Statement:
The total electric flux through closed surface is q / epsilon_0.
Diagram:
[DIAGRAM: Spherical Gaussian surface radius r with E and dS vectors and directional arrows.]
Derivation:
oint_S E . dS = E * oint_S dS = E * (4 * pi * r^2) = q / epsilon_0.
Final Result:
Total Electric Flux phi_E = 1.13 * 10^5 N m^2 C^-1.
Units: N m^2 C^-1

--- Question 2: Electric Dipole in Uniform Field ---
Statement & Derivation:
tau = p x E
Magnitude: tau = p * E * sin(theta).
Special Cases:
theta = 0: stable equilibrium.
theta = 90: maximum torque tau_max = pE.
"#;

    const DEDUCTIONS_STUDENT_TEXT: &str = r#"
TAMIL NADU HIGHER SECONDARY EXAMINATION - CLASS 12 PHYSICS
STUDENT NAME: Kumar S (Roll No: 1240915)
TARGET: 590/600 (Under Review)
SUBJECT: Physics (Theory)

--- Question 1: Gauss's Law & Electric Flux ---
Statement:
The total electric flux through closed surface is q / epsilon_0.
Diagram:
[DIAGRAM: Spherical Gaussian surface with radius r.]
Derivation:
oint_S E . dS = q / epsilon_0.
Final Result:
Total Electric Flux phi_E = 1.13 * 10^5

--- Question 2: Electric Dipole in Uniform Field ---
Statement & Derivation:
tau = p x E
Magnitude: tau = p * E * sin(theta).
Special Cases:
theta = 0: tau = 0.
"#;

    #[test]
    fn test_rubric_deserialization() {
        let rubric =
            ExamReviewer::load_rubric_from_str(SAMPLE_RUBRIC_JSON).expect("Deserialize rubric");
        assert_eq!(rubric.board, "Tamil Nadu State Board");
        assert_eq!(rubric.state_board_crown_threshold_percentage, 98.0);
        assert_eq!(rubric.questions.len(), 2);
    }

    #[test]
    fn test_perfect_student_evaluation_achieves_crown() {
        let rubric = ExamReviewer::load_rubric_from_str(SAMPLE_RUBRIC_JSON).expect("Rubric");
        let sheet = ExamReviewer::evaluate_text(PERFECT_STUDENT_TEXT, &rubric).expect("Evaluation");

        assert_eq!(sheet.student_name, Some("Arjun R".to_string()));
        assert_eq!(sheet.roll_number, Some("1240892".to_string()));
        assert_eq!(sheet.total_score, 8.0);
        assert_eq!(sheet.max_marks, 8.0);
        assert_eq!(sheet.percentage, 100.0);
        assert!(
            sheet.achieved_590_crown,
            "Perfect paper must achieve 590 State Board Crown"
        );
        assert_eq!(sheet.total_deductions, 0.0);
        assert_eq!(sheet.gap_analysis_590.marks_needed_for_590, 0.0);
    }

    #[test]
    fn test_procedural_deductions_applied_and_gap_analysis_generated() {
        let rubric = ExamReviewer::load_rubric_from_str(SAMPLE_RUBRIC_JSON).expect("Rubric");
        let sheet =
            ExamReviewer::evaluate_text(DEDUCTIONS_STUDENT_TEXT, &rubric).expect("Evaluation");

        assert_eq!(sheet.student_name, Some("Kumar S".to_string()));
        assert!(
            sheet.total_deductions >= 0.5,
            "Must record at least 0.5 deduction"
        );
        assert!(
            !sheet.achieved_590_crown,
            "Paper with deductions should miss 98% Crown threshold"
        );
        assert!(sheet.gap_analysis_590.marks_needed_for_590 > 0.0);

        let has_si_unit_note = sheet
            .red_pen_annotations
            .iter()
            .any(|n| n.contains("Missing standard SI unit") || n.contains("[-0.5 mark]"));
        assert!(
            has_si_unit_note,
            "Red pen annotations must note missing SI unit penalty"
        );
    }

    #[test]
    fn test_unattempted_questions_graded_zero() {
        let empty_text = "STUDENT NAME: Maya (Roll No: 999)";
        let rubric = ExamReviewer::load_rubric_from_str(SAMPLE_RUBRIC_JSON).expect("Rubric");
        let sheet = ExamReviewer::evaluate_text(empty_text, &rubric).expect("Evaluation");

        assert_eq!(sheet.total_score, 0.0);
        assert!(!sheet.achieved_590_crown);
        assert_eq!(sheet.itemized_scores.len(), 2);
        assert_eq!(sheet.itemized_scores[0].awarded_marks, 0.0);
    }

    #[test]
    fn test_workspace_benchmark_rubric_and_synthetic_pdfs() {
        let manifest_tests =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests");
        let rubric_manifest =
            manifest_tests.join("benchmarks/exam_rubrics/tn_stateboard_590_rubric.json");
        let candidate_rubric_paths = [
            std::path::PathBuf::from("tests/benchmarks/exam_rubrics/tn_stateboard_590_rubric.json"),
            std::path::PathBuf::from(
                "../tests/benchmarks/exam_rubrics/tn_stateboard_590_rubric.json",
            ),
            std::path::PathBuf::from(
                "../../tests/benchmarks/exam_rubrics/tn_stateboard_590_rubric.json",
            ),
            rubric_manifest,
        ];
        let rubric_path = candidate_rubric_paths
            .iter()
            .find(|p| p.exists())
            .expect("Workspace rubric JSON must exist");

        let rubric = ExamReviewer::load_rubric(rubric_path).expect("Load official rubric");
        assert_eq!(rubric.board, "Tamil Nadu State Board");
        assert_eq!(rubric.questions.len(), 4);

        // Resolve sample exam text paths
        let candidate_sample_paths = [
            (
                std::path::PathBuf::from(
                    "tests/benchmarks/sample_exams/tn_class12_physics_590.txt",
                ),
                std::path::PathBuf::from(
                    "tests/benchmarks/sample_exams/tn_class12_physics_590.pdf",
                ),
            ),
            (
                std::path::PathBuf::from(
                    "../tests/benchmarks/sample_exams/tn_class12_physics_590.txt",
                ),
                std::path::PathBuf::from(
                    "../tests/benchmarks/sample_exams/tn_class12_physics_590.pdf",
                ),
            ),
            (
                std::path::PathBuf::from(
                    "../../tests/benchmarks/sample_exams/tn_class12_physics_590.txt",
                ),
                std::path::PathBuf::from(
                    "../../tests/benchmarks/sample_exams/tn_class12_physics_590.pdf",
                ),
            ),
            (
                manifest_tests.join("benchmarks/sample_exams/tn_class12_physics_590.txt"),
                manifest_tests.join("benchmarks/sample_exams/tn_class12_physics_590.pdf"),
            ),
        ];

        let (txt_path, pdf_path) = candidate_sample_paths
            .iter()
            .find(|(txt, _)| txt.exists())
            .expect("Sample 590 text must exist");

        let perfect_text = fs::read_to_string(txt_path).expect("Read sample text");

        // Synthesize valid PDF binary if not already generated
        if !pdf_path.exists() {
            let page = mindset_sys::fs::document::PdfPage {
                page_number: 1,
                text: perfect_text.clone(),
            };
            let meta = mindset_sys::fs::document::PdfMetadata::default();
            let pdf_bytes = mindset_sys::fs::document::PdfEngine::generate_pdf(&[page], &meta)
                .expect("Synthesize PDF");
            let _ = fs::write(pdf_path, &pdf_bytes);
        }

        // Evaluate plaintext version
        let txt_sheet =
            ExamReviewer::evaluate(txt_path, &rubric).expect("Evaluate plaintext 590 paper");
        assert_eq!(txt_sheet.total_score, 18.0);
        assert_eq!(txt_sheet.percentage, 100.0);
        assert!(txt_sheet.achieved_590_crown);
        assert_eq!(txt_sheet.total_deductions, 0.0);

        // Evaluate PDF version
        if pdf_path.exists() {
            let pdf_sheet = ExamReviewer::evaluate(pdf_path, &rubric)
                .expect("Evaluate synthetic PDF 590 paper");
            assert!(pdf_sheet.total_score >= 18.0);
            assert!(pdf_sheet.achieved_590_crown);
        }

        // Test deductions paper
        let candidate_deduction_paths = [
            (
                std::path::PathBuf::from(
                    "tests/benchmarks/sample_exams/tn_class12_physics_deductions.txt",
                ),
                std::path::PathBuf::from(
                    "tests/benchmarks/sample_exams/tn_class12_physics_deductions.pdf",
                ),
            ),
            (
                std::path::PathBuf::from(
                    "../tests/benchmarks/sample_exams/tn_class12_physics_deductions.txt",
                ),
                std::path::PathBuf::from(
                    "../tests/benchmarks/sample_exams/tn_class12_physics_deductions.pdf",
                ),
            ),
            (
                std::path::PathBuf::from(
                    "../../tests/benchmarks/sample_exams/tn_class12_physics_deductions.txt",
                ),
                std::path::PathBuf::from(
                    "../../tests/benchmarks/sample_exams/tn_class12_physics_deductions.pdf",
                ),
            ),
            (
                manifest_tests.join("benchmarks/sample_exams/tn_class12_physics_deductions.txt"),
                manifest_tests.join("benchmarks/sample_exams/tn_class12_physics_deductions.pdf"),
            ),
        ];

        let (ded_txt_path, ded_pdf_path) = candidate_deduction_paths
            .iter()
            .find(|(txt, _)| txt.exists())
            .expect("Deductions text must exist");

        let ded_text = fs::read_to_string(ded_txt_path).expect("Read deductions text");

        // Synthesize deductions PDF
        if !ded_pdf_path.exists() {
            let page = mindset_sys::fs::document::PdfPage {
                page_number: 1,
                text: ded_text.clone(),
            };
            let meta = mindset_sys::fs::document::PdfMetadata::default();
            let pdf_bytes = mindset_sys::fs::document::PdfEngine::generate_pdf(&[page], &meta)
                .expect("Synthesize deductions PDF");
            let _ = fs::write(ded_pdf_path, &pdf_bytes);
        }

        let ded_sheet =
            ExamReviewer::evaluate(ded_txt_path, &rubric).expect("Evaluate deductions paper");

        assert_eq!(ded_sheet.total_score, 16.5);
        assert!(!ded_sheet.achieved_590_crown);
        assert_eq!(ded_sheet.total_deductions, 1.5);
        assert!(ded_sheet.gap_analysis_590.marks_needed_for_590 > 1.0);
        assert!(ded_sheet
            .red_pen_annotations
            .iter()
            .any(|a| a.contains("Missing standard SI unit")));
        assert!(ded_sheet
            .red_pen_annotations
            .iter()
            .any(|a| a.contains("Diagram incomplete")));
    }

    #[test]
    fn test_zero_baseline_unrelated_answer_and_dynamic_rubric_deductions() {
        let mut rubric = ExamReviewer::load_rubric_from_str(SAMPLE_RUBRIC_JSON).expect("Rubric");
        // Add DED-SIGNIFICANT-FIGS and DED-DERIVATION-STEP dynamically with custom penalties
        rubric.strict_rubric_deduction_rules.push(DeductionRule {
            rule_id: "DED-DERIVATION-STEP".to_string(),
            name: "Skipped Intermediate Step".to_string(),
            deduction_marks: 0.75,
            description: "Custom penalty check".to_string(),
        });
        rubric.strict_rubric_deduction_rules.push(DeductionRule {
            rule_id: "DED-SIGNIFICANT-FIGS".to_string(),
            name: "Improper Rounding".to_string(),
            deduction_marks: 0.25,
            description: "Custom sig fig penalty".to_string(),
        });

        // 1. Unrelated gibberish under Q2 must score 0.0 (never full marks)
        let unrelated = r#"
--- Question 1: Gauss's Law & Electric Flux ---
Statement:
Total flux through closed surface is q / epsilon_0.
Diagram:
[DIAGRAM: Spherical Gaussian surface radius r with outward E and dS vector arrows.]
Derivation:
oint_S E . dS = q / epsilon_0.
Final Result:
Total Electric Flux phi_E = 1 * 10^5 N m^2 C^-1 (approx)

--- Question 2: Electric Dipole in Uniform Field ---
Lorem ipsum dolor sit amet unrelated text without physics equations.
"#;
        let sheet = ExamReviewer::evaluate_text(unrelated, &rubric).expect("Eval");
        assert_eq!(
            sheet.itemized_scores[1].awarded_marks, 0.0,
            "Unrelated Q2 must score 0.0"
        );

        // Q1 skipped integration expansion ("4 * pi * r^2") -> DED-DERIVATION-STEP (-0.75)
        assert!(
            sheet.itemized_scores[0]
                .deductions
                .iter()
                .any(|d| d.rule_id == "DED-DERIVATION-STEP" && (d.penalty - 0.75).abs() < 1e-4),
            "Must apply dynamic DED-DERIVATION-STEP penalty from rubric"
        );
        // Q1 truncated 1-sig-fig approx -> DED-SIGNIFICANT-FIGS (-0.25)
        assert!(
            sheet.itemized_scores[0]
                .deductions
                .iter()
                .any(|d| d.rule_id == "DED-SIGNIFICANT-FIGS" && (d.penalty - 0.25).abs() < 1e-4),
            "Must apply dynamic DED-SIGNIFICANT-FIGS penalty from rubric"
        );
    }
}
