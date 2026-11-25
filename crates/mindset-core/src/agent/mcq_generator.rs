//! MindSet OS — Competitive Exam MCQ Generator
//!
//! Generates and evaluates high-yield multiple-choice question papers for premier
//! competitive examinations:
//! - JEE Main & Advanced: +4/-1 negative marking, single-correct and numerical value
//!   questions with calculation distractor breakdowns explaining common traps.
//! - TNPSC Group 1 Preliminary: Bilingual (Tamil + English) questions covering
//!   Unit 8 (Tamil Heritage, Keeladi, Thirukkural) and Unit 9 (Development Administration).
//! - UPSC Civil Services Prelims: Multi-statement elimination format with tactical
//!   elimination strategy notes.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// 1. Core Domain Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExamTarget {
    JeeMain,
    JeeAdvanced,
    TnpscGroup1,
    UpscCse,
}

impl fmt::Display for ExamTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExamTarget::JeeMain => write!(f, "JEE Main"),
            ExamTarget::JeeAdvanced => write!(f, "JEE Advanced"),
            ExamTarget::TnpscGroup1 => write!(f, "TNPSC Group 1 Preliminary"),
            ExamTarget::UpscCse => write!(f, "UPSC Civil Services Preliminary"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionType {
    SingleCorrect,
    Numerical,
    MultiStatementElimination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Foundational,
    Moderate,
    Advanced,
    RankDecider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkingScheme {
    pub correct_marks: f32,
    pub incorrect_marks: f32,
    pub unattempted_marks: f32,
}

impl MarkingScheme {
    pub fn for_exam(target: ExamTarget) -> Self {
        match target {
            ExamTarget::JeeMain | ExamTarget::JeeAdvanced => Self {
                correct_marks: 4.0,
                incorrect_marks: -1.0,
                unattempted_marks: 0.0,
            },
            ExamTarget::TnpscGroup1 => Self {
                correct_marks: 1.5,
                incorrect_marks: 0.0,
                unattempted_marks: 0.0,
            },
            ExamTarget::UpscCse => Self {
                correct_marks: 2.0,
                incorrect_marks: -0.6666,
                unattempted_marks: 0.0,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Question Data Structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistractorNote {
    pub option_index: usize,
    pub option_label: String,
    pub trap_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveQuestion {
    pub id: String,
    pub exam_target: ExamTarget,
    pub subject: String,
    pub topic: String,
    pub difficulty: DifficultyLevel,
    pub question_type: QuestionType,

    // Primary English / Universal Question text
    pub question_text: String,
    #[serde(default)]
    pub options: Vec<String>,
    pub correct_option_index: Option<usize>,
    pub explanation: String,

    // JEE Distractor Breakdown
    #[serde(default)]
    pub distractor_breakdown: Vec<DistractorNote>,

    // JEE Numerical Value Questions (Section B)
    pub numerical_correct_value: Option<f64>,
    pub numerical_tolerance: Option<f64>,

    // TNPSC Bilingual fields (Unit 8 & Unit 9)
    #[serde(default)]
    pub is_bilingual: bool,
    pub question_ta: Option<String>,
    #[serde(default)]
    pub options_ta: Option<Vec<String>>,
    pub explanation_ta: Option<String>,

    // UPSC Multi-statement elimination fields
    pub statement_context: Option<String>,
    #[serde(default)]
    pub statements: Option<Vec<String>>,
    pub elimination_strategy: Option<String>,
}

// ---------------------------------------------------------------------------
// 3. Test Paper & Evaluation Sheet
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPaper {
    pub title: String,
    pub exam_target: ExamTarget,
    pub subject: String,
    pub topic: String,
    pub total_questions: usize,
    pub total_marks: f32,
    pub time_limit_minutes: u32,
    pub marking_scheme: MarkingScheme,
    pub questions: Vec<CompetitiveQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudentAnswer {
    Option(usize),
    Numerical(f64),
    Unattempted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentSubmission {
    pub answers: HashMap<String, StudentAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEvaluationReport {
    pub test_title: String,
    pub exam_target: ExamTarget,
    pub total_questions: usize,
    pub attempted_count: usize,
    pub correct_count: usize,
    pub incorrect_count: usize,
    pub unattempted_count: usize,
    pub raw_score: f32,
    pub max_marks: f32,
    pub percentage: f32,
    pub accuracy_percentage: f32,
    pub question_results: Vec<QuestionResult>,
    pub tactical_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionResult {
    pub question_id: String,
    pub topic: String,
    pub question_type: QuestionType,
    pub is_correct: bool,
    pub marks_awarded: f32,
    pub student_answer: String,
    pub correct_answer: String,
    pub explanation: String,
    pub distractor_or_elimination_insight: Option<String>,
}

// ---------------------------------------------------------------------------
// 4. Competitive MCQ Generator Engine
// ---------------------------------------------------------------------------

pub struct CompetitiveMcqGenerator {
    question_bank: Vec<CompetitiveQuestion>,
}

impl Default for CompetitiveMcqGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CompetitiveMcqGenerator {
    /// Creates a new generator instance populated with rich built-in curriculum banks.
    pub fn new() -> Self {
        let mut gen = Self {
            question_bank: Vec::new(),
        };
        gen.populate_default_curriculum();
        gen
    }

    /// Total number of questions currently loaded in the repository.
    pub fn question_count(&self) -> usize {
        self.question_bank.len()
    }

    /// Loads benchmark datasets from a given workspace directory.
    pub fn load_benchmarks(&mut self, directory: &Path) -> Result<usize, String> {
        if !directory.exists() {
            return Err(format!("Directory does not exist: {}", directory.display()));
        }

        let mut loaded = 0;
        let entries = fs::read_dir(directory).map_err(|e| {
            format!(
                "Failed to read benchmark dir {}: {}",
                directory.display(),
                e
            )
        })?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
            {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(count) = self.load_from_json(&content) {
                        loaded += count;
                    }
                }
            }
        }

        Ok(loaded)
    }

    /// Ingests questions from a JSON dataset string into the active generator repository.
    pub fn load_from_json(&mut self, json_str: &str) -> Result<usize, String> {
        let root: serde_json::Value =
            serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {}", e))?;

        let exam_name = root.get("exam_name").and_then(|v| v.as_str()).unwrap_or("");
        let default_target = if exam_name.to_lowercase().contains("jee") {
            if exam_name.to_lowercase().contains("advanced") {
                ExamTarget::JeeAdvanced
            } else {
                ExamTarget::JeeMain
            }
        } else if exam_name.to_lowercase().contains("tnpsc") {
            ExamTarget::TnpscGroup1
        } else if exam_name.to_lowercase().contains("upsc") {
            ExamTarget::UpscCse
        } else {
            ExamTarget::JeeMain
        };

        let subject = root
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("General Studies");
        let topic = root
            .get("topic")
            .and_then(|v| v.as_str())
            .unwrap_or("General");
        let is_bilingual = root
            .get("bilingual")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let questions_val = root
            .get("questions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Missing 'questions' array in JSON root".to_string())?;

        let mut added = 0;
        for q_val in questions_val {
            let id = q_val
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("GEN-Q")
                .to_string();
            let q_type_str = q_val
                .get("question_type")
                .and_then(|v| v.as_str())
                .unwrap_or("SingleCorrect");

            let question_type = match q_type_str {
                "Numerical" => QuestionType::Numerical,
                "MultiStatementElimination" => QuestionType::MultiStatementElimination,
                _ => {
                    if q_val.get("statements").is_some() {
                        QuestionType::MultiStatementElimination
                    } else if q_val.get("numerical_correct_value").is_some() {
                        QuestionType::Numerical
                    } else {
                        QuestionType::SingleCorrect
                    }
                }
            };

            let question_text = q_val
                .get("question_text")
                .or_else(|| q_val.get("question_en"))
                .or_else(|| q_val.get("statement_context"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let options: Vec<String> = q_val
                .get("options")
                .or_else(|| q_val.get("options_en"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let correct_option_index = q_val
                .get("correct_option_index")
                .and_then(|v| v.as_u64())
                .map(|u| u as usize);

            let explanation = q_val
                .get("explanation")
                .or_else(|| q_val.get("explanation_en"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let numerical_correct_value = q_val
                .get("numerical_correct_value")
                .and_then(|v| v.as_f64());
            let numerical_tolerance = q_val.get("tolerance").and_then(|v| v.as_f64());

            // Bilingual fields
            let question_ta = q_val
                .get("question_ta")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let options_ta: Option<Vec<String>> = q_val
                .get("options_ta")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                });
            let explanation_ta = q_val
                .get("explanation_ta")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // UPSC statement fields
            let statement_context = q_val
                .get("statement_context")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let statements: Option<Vec<String>> = q_val
                .get("statements")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                });
            let elimination_strategy = q_val
                .get("elimination_logic")
                .or_else(|| q_val.get("elimination_strategy"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let q = CompetitiveQuestion {
                id,
                exam_target: default_target,
                subject: subject.to_string(),
                topic: topic.to_string(),
                difficulty: DifficultyLevel::Moderate,
                question_type,
                question_text,
                options,
                correct_option_index,
                explanation,
                distractor_breakdown: Vec::new(),
                numerical_correct_value,
                numerical_tolerance,
                is_bilingual,
                question_ta,
                options_ta,
                explanation_ta,
                statement_context,
                statements,
                elimination_strategy,
            };

            if let Some(pos) = self
                .question_bank
                .iter()
                .position(|existing| existing.id == q.id)
            {
                self.question_bank[pos] = q;
            } else {
                self.question_bank.push(q);
                added += 1;
            }
        }

        Ok(added)
    }

    /// Generates a customized test paper for JEE Main or Advanced.
    pub fn generate_jee_paper(
        &self,
        subject: &str,
        topic: Option<&str>,
        single_choice_count: usize,
        numerical_count: usize,
    ) -> Result<TestPaper, String> {
        let sc_candidates: Vec<CompetitiveQuestion> = self
            .question_bank
            .iter()
            .filter(|q| {
                (q.exam_target == ExamTarget::JeeMain || q.exam_target == ExamTarget::JeeAdvanced)
                    && q.question_type == QuestionType::SingleCorrect
                    && (subject.is_empty() || q.subject.eq_ignore_ascii_case(subject))
                    && topic
                        .map(|t| q.topic.to_lowercase().contains(&t.to_lowercase()))
                        .unwrap_or(true)
            })
            .cloned()
            .collect();

        let num_candidates: Vec<CompetitiveQuestion> = self
            .question_bank
            .iter()
            .filter(|q| {
                (q.exam_target == ExamTarget::JeeMain || q.exam_target == ExamTarget::JeeAdvanced)
                    && q.question_type == QuestionType::Numerical
                    && (subject.is_empty() || q.subject.eq_ignore_ascii_case(subject))
                    && topic
                        .map(|t| q.topic.to_lowercase().contains(&t.to_lowercase()))
                        .unwrap_or(true)
            })
            .cloned()
            .collect();

        let mut selected = Vec::new();
        for q in sc_candidates.into_iter().take(single_choice_count) {
            selected.push(q);
        }
        for q in num_candidates.into_iter().take(numerical_count) {
            selected.push(q);
        }

        if selected.is_empty() {
            return Err("No JEE questions matching specified subject/topic filters".to_string());
        }

        let total_questions = selected.len();
        let total_marks = (total_questions as f32) * 4.0;

        Ok(TestPaper {
            title: format!("JEE Practice Test — {}", topic.unwrap_or(subject)),
            exam_target: ExamTarget::JeeAdvanced,
            subject: subject.to_string(),
            topic: topic.unwrap_or("All Topics").to_string(),
            total_questions,
            total_marks,
            time_limit_minutes: (total_questions as u32) * 3,
            marking_scheme: MarkingScheme::for_exam(ExamTarget::JeeAdvanced),
            questions: selected,
        })
    }

    /// Generates a bilingual TNPSC Group 1 question paper focusing on Unit 8 and Unit 9.
    pub fn generate_tnpsc_paper(
        &self,
        unit: Option<&str>,
        count: usize,
    ) -> Result<TestPaper, String> {
        let candidates: Vec<CompetitiveQuestion> = self
            .question_bank
            .iter()
            .filter(|q| {
                q.exam_target == ExamTarget::TnpscGroup1
                    && unit
                        .map(|u| q.topic.to_lowercase().contains(&u.to_lowercase()))
                        .unwrap_or(true)
            })
            .cloned()
            .collect();

        let selected: Vec<CompetitiveQuestion> = candidates.into_iter().take(count).collect();
        if selected.is_empty() {
            return Err("No TNPSC questions found matching unit filter".to_string());
        }

        let total_questions = selected.len();
        let total_marks = (total_questions as f32) * 1.5;

        Ok(TestPaper {
            title: format!(
                "TNPSC Group 1 Preliminary Test — {}",
                unit.unwrap_or("General Studies")
            ),
            exam_target: ExamTarget::TnpscGroup1,
            subject: "Tamil Nadu Administration & General Studies".to_string(),
            topic: unit.unwrap_or("Unit 8 & Unit 9").to_string(),
            total_questions,
            total_marks,
            time_limit_minutes: total_questions as u32,
            marking_scheme: MarkingScheme::for_exam(ExamTarget::TnpscGroup1),
            questions: selected,
        })
    }

    /// Generates a UPSC Civil Services Prelims test paper with multi-statement elimination questions.
    pub fn generate_upsc_paper(
        &self,
        topic: Option<&str>,
        count: usize,
    ) -> Result<TestPaper, String> {
        let candidates: Vec<CompetitiveQuestion> = self
            .question_bank
            .iter()
            .filter(|q| {
                q.exam_target == ExamTarget::UpscCse
                    && topic
                        .map(|t| q.topic.to_lowercase().contains(&t.to_lowercase()))
                        .unwrap_or(true)
            })
            .cloned()
            .collect();

        let selected: Vec<CompetitiveQuestion> = candidates.into_iter().take(count).collect();
        if selected.is_empty() {
            return Err("No UPSC questions found matching topic filter".to_string());
        }

        let total_questions = selected.len();
        let total_marks = (total_questions as f32) * 2.0;

        Ok(TestPaper {
            title: format!(
                "UPSC Civil Services Prelims Mock — {}",
                topic.unwrap_or("General Studies Paper I")
            ),
            exam_target: ExamTarget::UpscCse,
            subject: "General Studies Paper I".to_string(),
            topic: topic.unwrap_or("History & Polity").to_string(),
            total_questions,
            total_marks,
            time_limit_minutes: (total_questions as u32) * 2,
            marking_scheme: MarkingScheme::for_exam(ExamTarget::UpscCse),
            questions: selected,
        })
    }

    /// Evaluates a student's test submission, enforcing negative marking, numerical tolerances,
    /// and generating pointed distractor or elimination feedback.
    pub fn evaluate_submission(
        &self,
        paper: &TestPaper,
        submission: &StudentSubmission,
    ) -> TestEvaluationReport {
        let mut attempted = 0;
        let mut correct = 0;
        let mut incorrect = 0;
        let mut unattempted = 0;
        let mut raw_score = 0.0f32;
        let mut results = Vec::new();

        let mut distractor_traps_hit = Vec::new();

        for q in &paper.questions {
            let student_ans = submission
                .answers
                .get(&q.id)
                .unwrap_or(&StudentAnswer::Unattempted);
            let mut is_correct = false;
            let awarded_marks: f32;
            let mut student_str = "Unattempted".to_string();
            let mut correct_str = String::new();
            let mut trap_insight = None;

            match q.question_type {
                QuestionType::SingleCorrect | QuestionType::MultiStatementElimination => {
                    let correct_idx = q.correct_option_index.unwrap_or(0);
                    if let Some(opt) = q.options.get(correct_idx) {
                        correct_str = format!("({}) {}", Self::option_letter(correct_idx), opt);
                    }

                    match student_ans {
                        StudentAnswer::Option(chosen_idx) => {
                            attempted += 1;
                            if let Some(opt) = q.options.get(*chosen_idx) {
                                student_str =
                                    format!("({}) {}", Self::option_letter(*chosen_idx), opt);
                            } else {
                                student_str = format!("Option {}", chosen_idx);
                            }

                            if *chosen_idx == correct_idx {
                                is_correct = true;
                                correct += 1;
                                awarded_marks = paper.marking_scheme.correct_marks;
                            } else {
                                incorrect += 1;
                                awarded_marks = paper.marking_scheme.incorrect_marks;

                                // Extract distractor explanation if student fell for a known trap
                                if let Some(trap) = q
                                    .distractor_breakdown
                                    .iter()
                                    .find(|d| d.option_index == *chosen_idx)
                                {
                                    trap_insight =
                                        Some(format!("Trap Warning: {}", trap.trap_description));
                                    distractor_traps_hit.push(trap.trap_description.clone());
                                } else if let Some(elim) = &q.elimination_strategy {
                                    trap_insight = Some(format!("Elimination Insight: {}", elim));
                                }
                            }
                        }
                        _ => {
                            unattempted += 1;
                            awarded_marks = paper.marking_scheme.unattempted_marks;
                        }
                    }
                }
                QuestionType::Numerical => {
                    let target_val = q.numerical_correct_value.unwrap_or(0.0);
                    let tol = q.numerical_tolerance.unwrap_or(0.05);
                    correct_str = format!("{:.2} (tolerance: ±{:.2})", target_val, tol);

                    match student_ans {
                        StudentAnswer::Numerical(val) => {
                            attempted += 1;
                            student_str = format!("{:.2}", val);
                            if (val - target_val).abs() <= tol {
                                is_correct = true;
                                correct += 1;
                                awarded_marks = paper.marking_scheme.correct_marks;
                            } else {
                                incorrect += 1;
                                // In JEE numerical section, incorrect is typically 0 or -1 depending on round
                                awarded_marks = paper.marking_scheme.incorrect_marks;
                                trap_insight = Some(format!(
                                    "Numerical discrepancy: Target is {:.2}. Common error: incorrect decimal rounding or sign convention.",
                                    target_val
                                ));
                            }
                        }
                        _ => {
                            unattempted += 1;
                            awarded_marks = paper.marking_scheme.unattempted_marks;
                        }
                    }
                }
            }

            raw_score += awarded_marks;
            results.push(QuestionResult {
                question_id: q.id.clone(),
                topic: q.topic.clone(),
                question_type: q.question_type,
                is_correct,
                marks_awarded: awarded_marks,
                student_answer: student_str,
                correct_answer: correct_str,
                explanation: q.explanation.clone(),
                distractor_or_elimination_insight: trap_insight,
            });
        }

        let max_marks = paper.total_marks;
        let percentage = if max_marks > 0.0 {
            (raw_score / max_marks) * 100.0
        } else {
            0.0
        };

        let accuracy = if attempted > 0 {
            (correct as f32 / attempted as f32) * 100.0
        } else {
            0.0
        };

        let mut recommendations = Vec::new();
        if accuracy < 75.0 && attempted > 0 {
            recommendations.push(format!(
                "Negative Marking Alert: You lost {:.1} marks on {} incorrect attempts. Avoid speculative guessing when unsure.",
                (incorrect as f32) * paper.marking_scheme.incorrect_marks.abs(),
                incorrect
            ));
        }
        if !distractor_traps_hit.is_empty() {
            recommendations.push(
                "Review distractor explanations on calculation questions to avoid algebraic traps."
                    .to_string(),
            );
        }
        if paper.exam_target == ExamTarget::UpscCse && incorrect > 0 {
            recommendations.push(
                "Use the 2-stage elimination rule: First identify and discard extreme qualifier statements before choosing the option pair.".to_string(),
            );
        }

        TestEvaluationReport {
            test_title: paper.title.clone(),
            exam_target: paper.exam_target,
            total_questions: paper.total_questions,
            attempted_count: attempted,
            correct_count: correct,
            incorrect_count: incorrect,
            unattempted_count: unattempted,
            raw_score: (raw_score * 100.0).round() / 100.0,
            max_marks,
            percentage: (percentage * 100.0).round() / 100.0,
            accuracy_percentage: (accuracy * 100.0).round() / 100.0,
            question_results: results,
            tactical_recommendations: recommendations,
        }
    }

    fn option_letter(idx: usize) -> char {
        match idx {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => '?',
        }
    }

    /// Seeds built-in curriculum questions for JEE, TNPSC, and UPSC exams.
    fn populate_default_curriculum(&mut self) {
        // 1. JEE Advanced Electrostatics (Single Correct)
        self.question_bank.push(CompetitiveQuestion {
            id: "JEE-ELEC-01".to_string(),
            exam_target: ExamTarget::JeeAdvanced,
            subject: "Physics".to_string(),
            topic: "Electrostatics & Capacitance".to_string(),
            difficulty: DifficultyLevel::Moderate,
            question_type: QuestionType::SingleCorrect,
            question_text: "An electric dipole of moment p is placed at the origin with its axis along the x-axis. The electric field at a point (0, y, 0) on the equatorial line is directed along:".to_string(),
            options: vec![
                "+x direction".to_string(),
                "-x direction".to_string(),
                "+y direction".to_string(),
                "-y direction".to_string(),
            ],
            correct_option_index: Some(1),
            explanation: "On the equatorial plane of an electric dipole, the electric field E = -p / (4 * pi * epsilon_0 * y^3), which is antiparallel to dipole moment p. Since p points along +x, E points along -x.".to_string(),
            distractor_breakdown: vec![
                DistractorNote {
                    option_index: 0,
                    option_label: "+x direction".to_string(),
                    trap_description: "Confusing the equatorial field with the axial field (axial field is parallel to dipole moment p).".to_string(),
                },
                DistractorNote {
                    option_index: 2,
                    option_label: "+y direction".to_string(),
                    trap_description: "Assuming electric field points outward along the position vector of the equatorial point.".to_string(),
                },
            ],
            numerical_correct_value: None,
            numerical_tolerance: None,
            is_bilingual: false,
            question_ta: None,
            options_ta: None,
            explanation_ta: None,
            statement_context: None,
            statements: None,
            elimination_strategy: None,
        });

        // 2. JEE Advanced Numerical Problem
        self.question_bank.push(CompetitiveQuestion {
            id: "JEE-ELEC-04".to_string(),
            exam_target: ExamTarget::JeeAdvanced,
            subject: "Physics".to_string(),
            topic: "Electrostatics & Capacitance".to_string(),
            difficulty: DifficultyLevel::Advanced,
            question_type: QuestionType::Numerical,
            question_text: "Two point charges +4 uC and +1 uC are placed 30 cm apart in vacuum. At what distance (in cm) from the +4 uC charge on the line joining them is the net electric field zero?".to_string(),
            options: Vec::new(),
            correct_option_index: None,
            explanation: "k * 4 / x^2 = k * 1 / (30 - x)^2 => 2 / x = 1 / (30 - x) => 60 - 2x = x => 3x = 60 => x = 20 cm.".to_string(),
            distractor_breakdown: Vec::new(),
            numerical_correct_value: Some(20.0),
            numerical_tolerance: Some(0.1),
            is_bilingual: false,
            question_ta: None,
            options_ta: None,
            explanation_ta: None,
            statement_context: None,
            statements: None,
            elimination_strategy: None,
        });

        // 3. TNPSC Group 1 Unit 8: Thirukkural Governance (Bilingual)
        self.question_bank.push(CompetitiveQuestion {
            id: "TNPSC-U8-01".to_string(),
            exam_target: ExamTarget::TnpscGroup1,
            subject: "Tamil Nadu Heritage & Culture".to_string(),
            topic: "Unit 8: Thirukkural & Tamil Society".to_string(),
            difficulty: DifficultyLevel::Moderate,
            question_type: QuestionType::SingleCorrect,
            question_text: "According to Thirukkural, which four qualities are the essential ornaments of an ideal ruler/administrator?".to_string(),
            options: vec![
                "Courage, Generosity, Compassion, and Governance (அஞ்சாமை, ஈகை, அறிவு, ஊக்கம்)".to_string(),
                "Wealth, Power, Military, and Dominance (செல்வம், அதிகாரம், படை, ஆதிக்கம்)".to_string(),
                "Asceticism, Silence, Renunciation, and Solitude (துறவு, மெளனம், துறத்தல், தனிமை)".to_string(),
                "Diplomacy, Secrecy, Espionage, and Punishment (தூது, இரகசியம், ஒற்று, தண்டனை)".to_string(),
            ],
            correct_option_index: Some(0),
            explanation: "In Kural 382, Thiruvalluvar states: 'அஞ்சாமை ஈகை அறிவுஊக்கம் இந்நான்கும் எஞ்சாமை வேந்தர்க் கியல்பு' — Courage, generosity, wisdom, and diligence are indispensable to a noble ruler.".to_string(),
            distractor_breakdown: Vec::new(),
            numerical_correct_value: None,
            numerical_tolerance: None,
            is_bilingual: true,
            question_ta: Some("திருக்குறளின்படி, ஒரு சிறந்த ஆட்சியாளருக்கு இருக்க வேண்டிய நான்கு இன்றியமையாத பண்புகள் யாவை?".to_string()),
            options_ta: Some(vec![
                "அஞ்சாமை, ஈகை, அறிவு, ஊக்கம்".to_string(),
                "செல்வம், அதிகாரம், படை, ஆதிக்கம்".to_string(),
                "துறவு, மெளனம், துறத்தல், தனிமை".to_string(),
                "தூது, இரகசியம், ஒற்று, தண்டனை".to_string(),
            ]),
            explanation_ta: Some("குறள் 382: 'அஞ்சாமை ஈகை அறிவுஊக்கம் இந்நான்கும் எஞ்சாமை வேந்தர்க் கியல்பு' — துணிவு, கொடைத்தன்மை, நல்லறிவு, ஊக்கம் ஆகிய நான்கும் அரசனுக்குரிய தகுதிகளாகும்.".to_string()),
            statement_context: None,
            statements: None,
            elimination_strategy: None,
        });

        // 4. TNPSC Group 1 Unit 9: Development Administration (Bilingual)
        self.question_bank.push(CompetitiveQuestion {
            id: "TNPSC-U9-01".to_string(),
            exam_target: ExamTarget::TnpscGroup1,
            subject: "Development Administration in Tamil Nadu".to_string(),
            topic: "Unit 9: Human Development & Social Welfare".to_string(),
            difficulty: DifficultyLevel::Moderate,
            question_type: QuestionType::SingleCorrect,
            question_text: "Which pioneering social welfare scheme was initiated in Tamil Nadu in 1982 to improve school nutritional status and reduce dropout rates?".to_string(),
            options: vec![
                "Puratchi Thalaivar MGR Nutritious Meal Programme".to_string(),
                "Integrated Child Development Services (ICDS)".to_string(),
                "Sarva Shiksha Abhiyan".to_string(),
                "Moovalur Ramamirtham Ammaiyar Higher Education Assurance Scheme".to_string(),
            ],
            correct_option_index: Some(0),
            explanation: "Chief Minister M.G. Ramachandran launched the Nutritious Meal Programme on July 1, 1982, which drastically boosted school enrollment and maternal health across Tamil Nadu.".to_string(),
            distractor_breakdown: Vec::new(),
            numerical_correct_value: None,
            numerical_tolerance: None,
            is_bilingual: true,
            question_ta: Some("பள்ளி மாணவர்களின் ஊட்டச்சத்து நிலையை உயர்த்தவும், பள்ளி இடைநிற்றலைத் தடுக்கவும் 1982-ல் தமிழகத்தில் தொடங்கப்பட்ட முன்னோடி திட்டம் எது?".to_string()),
            options_ta: Some(vec![
                "புரட்சித் தலைவர் எம்.ஜி.ஆர் சத்துணவுத் திட்டம்".to_string(),
                "ஒருங்கிணைந்த குழந்தைகள் வளர்ச்சித் திட்டம் (ICDS)".to_string(),
                "அனைவருக்கும் கல்வி இயக்கம்".to_string(),
                "மூவலூர் ராமாமிர்தம் அம்மையார் உயர் கல்வி உறுதித் திட்டம்".to_string(),
            ]),
            explanation_ta: Some("1982 ஜூலை 1 அன்று முதலமைச்சர் எம்.ஜி. ராமச்சந்திரனால் புரட்சித் தலைவர் எம்.ஜி.ஆர் சத்துணவுத் திட்டம் தொடங்கப்பட்டது.".to_string()),
            statement_context: None,
            statements: None,
            elimination_strategy: None,
        });

        // 5. UPSC Multi-Statement Elimination Question (Modern History)
        self.question_bank.push(CompetitiveQuestion {
            id: "UPSC-HIST-01".to_string(),
            exam_target: ExamTarget::UpscCse,
            subject: "General Studies Paper I".to_string(),
            topic: "Modern Indian History & Freedom Struggle".to_string(),
            difficulty: DifficultyLevel::RankDecider,
            question_type: QuestionType::MultiStatementElimination,
            question_text: "With reference to the Cabinet Mission Plan of 1946, consider the following statements:".to_string(),
            options: vec![
                "1 and 2 only".to_string(),
                "2 and 3 only".to_string(),
                "1 and 3 only".to_string(),
                "1, 2 and 3".to_string(),
            ],
            correct_option_index: Some(0),
            explanation: "The Cabinet Mission (1946) rejected Pakistan and proposed provincial grouping (Sections A, B, C). It did not grant immediate independence in May 1946, but proposed an interim government and Constituent Assembly.".to_string(),
            distractor_breakdown: Vec::new(),
            numerical_correct_value: None,
            numerical_tolerance: None,
            is_bilingual: false,
            question_ta: None,
            options_ta: None,
            explanation_ta: None,
            statement_context: Some("With reference to the Cabinet Mission Plan of 1946, consider the following statements:".to_string()),
            statements: Some(vec![
                "1. It recommended an undivided India and rejected the demand for a separate Pakistan.".to_string(),
                "2. It proposed that provinces should be grouped into three sections (Sections A, B, and C).".to_string(),
                "3. It granted full independence to India with immediate effect from May 1946.".to_string(),
            ]),
            elimination_strategy: Some("Statement 3 is factually incorrect because immediate independence was not granted; the mission set up the framework for the Constituent Assembly. Eliminating statement 3 instantly eliminates options (B), (C), and (D), directly yielding (A) without needing to verify complex clauses.".to_string()),
        });
    }
}

// ---------------------------------------------------------------------------
// 5. Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_competitive_generator_initialization() {
        let gen = CompetitiveMcqGenerator::new();
        assert!(gen.question_count() >= 5);
    }

    #[test]
    fn test_jee_paper_generation_and_scoring() {
        let gen = CompetitiveMcqGenerator::new();
        let paper = gen
            .generate_jee_paper("Physics", Some("Electrostatics"), 1, 1)
            .expect("Generate JEE paper");

        assert_eq!(paper.exam_target, ExamTarget::JeeAdvanced);
        assert_eq!(paper.total_questions, 2);
        assert_eq!(paper.marking_scheme.correct_marks, 4.0);
        assert_eq!(paper.marking_scheme.incorrect_marks, -1.0);

        // Simulate student submission: Question 1 correct (Option 1), Question 2 correct numerical (20.02)
        let mut answers = HashMap::new();
        answers.insert("JEE-ELEC-01".to_string(), StudentAnswer::Option(1));
        answers.insert("JEE-ELEC-04".to_string(), StudentAnswer::Numerical(20.02));

        let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
        assert_eq!(report.correct_count, 2);
        assert_eq!(report.incorrect_count, 0);
        assert_eq!(report.raw_score, 8.0);
        assert_eq!(report.accuracy_percentage, 100.0);
    }

    #[test]
    fn test_jee_negative_marking_and_distractor_analysis() {
        let gen = CompetitiveMcqGenerator::new();
        let paper = gen
            .generate_jee_paper("Physics", Some("Electrostatics"), 1, 0)
            .expect("JEE paper");

        // Simulate student falling into axial dipole trap (Option 0)
        let mut answers = HashMap::new();
        answers.insert("JEE-ELEC-01".to_string(), StudentAnswer::Option(0));

        let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
        assert_eq!(report.correct_count, 0);
        assert_eq!(report.incorrect_count, 1);
        assert_eq!(report.raw_score, -1.0); // -1 mark applied

        let first_res = &report.question_results[0];
        assert!(!first_res.is_correct);
        assert!(first_res
            .distractor_or_elimination_insight
            .as_ref()
            .unwrap()
            .contains("Trap Warning"));
    }

    #[test]
    fn test_tnpsc_bilingual_paper_generation() {
        let gen = CompetitiveMcqGenerator::new();
        let paper = gen
            .generate_tnpsc_paper(Some("Unit 8"), 1)
            .expect("TNPSC paper");

        assert_eq!(paper.exam_target, ExamTarget::TnpscGroup1);
        assert_eq!(paper.marking_scheme.correct_marks, 1.5);
        assert_eq!(paper.marking_scheme.incorrect_marks, 0.0); // Zero negative marks

        let q = &paper.questions[0];
        assert!(q.is_bilingual);
        assert!(q.question_ta.is_some());
        assert!(q.options_ta.is_some());
        assert_eq!(q.options_ta.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_upsc_multi_statement_elimination_paper() {
        let gen = CompetitiveMcqGenerator::new();
        let paper = gen
            .generate_upsc_paper(Some("Modern"), 1)
            .expect("UPSC paper");

        assert_eq!(paper.exam_target, ExamTarget::UpscCse);
        assert_eq!(paper.marking_scheme.correct_marks, 2.0);

        let q = &paper.questions[0];
        assert_eq!(q.question_type, QuestionType::MultiStatementElimination);
        assert!(q.statements.is_some());
        assert!(q.elimination_strategy.is_some());

        // Correctly eliminate statement 3 -> select option 0
        let mut answers = HashMap::new();
        answers.insert(q.id.clone(), StudentAnswer::Option(0));

        let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
        assert_eq!(report.correct_count, 1);
        assert_eq!(report.raw_score, 2.0);
    }

    #[test]
    fn test_benchmark_mcq_datasets_ingestion_and_evaluation() {
        let mut gen = CompetitiveMcqGenerator::new();
        let initial_count = gen.question_count();

        let candidate_dirs = [
            "tests/benchmarks/mcq_datasets",
            "../tests/benchmarks/mcq_datasets",
            "../../tests/benchmarks/mcq_datasets",
            "C:/MINDSETOS/tests/benchmarks/mcq_datasets",
        ];

        let benchmark_dir = candidate_dirs
            .iter()
            .map(Path::new)
            .find(|p| p.exists())
            .expect("MCQ benchmark directory must exist");

        let loaded = gen
            .load_benchmarks(benchmark_dir)
            .expect("Load benchmark datasets");
        assert!(
            loaded >= 15,
            "Must load at least 15 questions from benchmarks, loaded {}",
            loaded
        );
        assert!(gen.question_count() >= initial_count + loaded);

        // Verify JEE Rotational Dynamics question was loaded and is solvable
        let jee_rot = gen
            .generate_jee_paper("Physics", Some("Rotational"), 1, 1)
            .expect("JEE Rotational paper");
        assert_eq!(jee_rot.questions.len(), 2);

        // Verify TNPSC bilingual Unit 8 & 9 loaded
        let tnpsc_paper = gen
            .generate_tnpsc_paper(Some("Unit 8"), 2)
            .expect("TNPSC paper");
        assert_eq!(tnpsc_paper.questions.len(), 2);
        for q in &tnpsc_paper.questions {
            assert!(q.is_bilingual);
            assert!(q.question_ta.is_some());
        }

        // Verify UPSC multi-statement question loaded
        let upsc_paper = gen
            .generate_upsc_paper(Some("History"), 2)
            .expect("UPSC paper");
        assert_eq!(upsc_paper.questions.len(), 2);
        for q in &upsc_paper.questions {
            assert_eq!(q.question_type, QuestionType::MultiStatementElimination);
        }
    }

    #[test]
    fn test_load_benchmarks_deduplicates_existing_ids() {
        let mut gen = CompetitiveMcqGenerator::new();
        let candidate_dirs = [
            "tests/benchmarks/mcq_datasets",
            "../tests/benchmarks/mcq_datasets",
            "../../tests/benchmarks/mcq_datasets",
            "C:/MINDSETOS/tests/benchmarks/mcq_datasets",
        ];

        let benchmark_dir = candidate_dirs
            .iter()
            .map(Path::new)
            .find(|p| p.exists())
            .expect("MCQ benchmark directory must exist");

        let first_loaded = gen
            .load_benchmarks(benchmark_dir)
            .expect("Load benchmark datasets first time");
        assert!(first_loaded >= 15);

        let count_after_first = gen.question_count();
        assert_eq!(
            gen.question_bank
                .iter()
                .filter(|q| q.id == "JEE-ELEC-01")
                .count(),
            1
        );
        assert_eq!(
            gen.question_bank
                .iter()
                .filter(|q| q.id == "JEE-ELEC-04")
                .count(),
            1
        );
        assert_eq!(
            gen.question_bank
                .iter()
                .filter(|q| q.id == "UPSC-HIST-01")
                .count(),
            1
        );

        let second_loaded = gen
            .load_benchmarks(benchmark_dir)
            .expect("Load benchmark datasets second time");
        assert_eq!(second_loaded, 0);
        assert_eq!(gen.question_count(), count_after_first);
    }
}
