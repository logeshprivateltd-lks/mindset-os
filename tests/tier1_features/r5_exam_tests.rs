//! Tier 1: Subsystem R5 Exam Reviewer & Competitive MCQ Generator Tests (Features 19 to 21)
//!
//! Covers:
//! - F19: State Board 590/600 AI exam reviewer & rubric deductions (>=5 tests)
//! - F20: Competitive exam MCQ generator (JEE, TNPSC, UPSC) (>=5 tests)
//! - F21: Workspace verification benchmarks in tests/benchmarks/ (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::agent::{
    CompetitiveMcqGenerator, ExamReviewer, ExamTarget, QuestionType, StudentAnswer,
    StudentSubmission,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

use crate::helpers::benchmark_dir;

// ===========================================================================
// FEATURE 19: State Board 590/600 AI Exam Reviewer & Rubric Deductions
// ===========================================================================

#[test]
fn test_f19_rubric_file_parsing() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    assert!(rubric_path.exists(), "Rubric JSON must exist");

    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric via ExamReviewer");
    assert_eq!(rubric.board, "Tamil Nadu State Board");
    assert_eq!(rubric.subject, "Physics");
    assert_eq!(rubric.state_board_crown_threshold_percentage, 98.0);
    assert!(rubric.strict_rubric_deduction_rules.len() >= 4);
    assert_eq!(rubric.questions.len(), 4);
}

#[test]
fn test_f19_perfect_student_paper_scores_full() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");
    let paper_path = benchmark_dir().join("sample_exams/tn_class12_physics_590.txt");

    let sheet = ExamReviewer::evaluate(&paper_path, &rubric).expect("Evaluate perfect paper");
    assert_eq!(sheet.total_score, 18.0);
    assert_eq!(sheet.max_marks, 18.0);
    assert_eq!(sheet.percentage, 100.0);
    assert!(
        sheet.achieved_590_crown,
        "Perfect 590/600 paper must earn State Board Crown"
    );
    assert_eq!(sheet.total_deductions, 0.0);
    assert_eq!(sheet.itemized_scores.len(), 4);
}

#[test]
fn test_f19_missing_si_unit_triggers_deduction() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");
    let paper_path = benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt");
    let raw = fs::read_to_string(&paper_path).expect("Read deductions paper");
    assert!(
        !raw.contains("Rule DED-"),
        "Student paper must contain genuine structural errors without marker comments"
    );

    let sheet = ExamReviewer::evaluate(&paper_path, &rubric).expect("Evaluate deductions paper");
    let q1_deduction = sheet.itemized_scores[0]
        .deductions
        .iter()
        .find(|d| d.rule_id == "DED-SI-UNIT")
        .expect("Q1 omitting SI unit must trigger DED-SI-UNIT deduction");
    assert!((q1_deduction.penalty - 0.5).abs() < 1e-4);
}

#[test]
fn test_f19_missing_diagram_label_triggers_deduction() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");
    let paper_path = benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt");
    let raw = fs::read_to_string(&paper_path).expect("Read deductions paper");
    assert!(!raw.contains("Rule DED-DIAGRAM-LABEL"));
    assert!(!raw.contains("Missing current direction arrows"));

    let sheet = ExamReviewer::evaluate(&paper_path, &rubric).expect("Evaluate deductions paper");
    let q3_deduction = sheet.itemized_scores[2]
        .deductions
        .iter()
        .find(|d| d.rule_id == "DED-DIAGRAM-LABEL")
        .expect("Q3 circuit omitting current arrows & polarity must trigger DED-DIAGRAM-LABEL");
    assert!((q3_deduction.penalty - 1.0).abs() < 1e-4);
}

#[test]
fn test_f19_590_600_gap_analysis() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");
    let paper_path = benchmark_dir().join("sample_exams/tn_class12_physics_deductions.txt");

    let sheet = ExamReviewer::evaluate(&paper_path, &rubric).expect("Evaluate deductions paper");
    assert_eq!(sheet.total_score, 16.5);
    assert_eq!(sheet.max_marks, 18.0);
    assert_eq!(sheet.total_deductions, 1.5);
    assert!(!sheet.achieved_590_crown, "91.67% must miss 98% Crown");
    assert!((sheet.percentage - 91.67).abs() < 0.1);
    assert!(sheet.gap_analysis_590.marks_needed_for_590 > 1.0);
    assert!(!sheet.gap_analysis_590.primary_loss_categories.is_empty());
    assert!(!sheet.gap_analysis_590.actionable_recommendations.is_empty());
}

// ===========================================================================
// FEATURE 20: Competitive Exam MCQ Generator
// ===========================================================================

#[test]
fn test_f20_jee_mcq_dataset_scoring_rules() {
    let mut gen = CompetitiveMcqGenerator::new();
    let loaded = gen
        .load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmark datasets");
    assert!(loaded >= 15);

    let paper = gen
        .generate_jee_paper("Physics", Some("Rotational"), 2, 0)
        .expect("Generate JEE Rotational Dynamics paper");
    assert_eq!(paper.exam_target, ExamTarget::JeeAdvanced);
    assert_eq!(paper.marking_scheme.correct_marks, 4.0);
    assert_eq!(paper.marking_scheme.incorrect_marks, -1.0);

    // 1 correct (+4), 1 wrong (-1) => +3.0
    let mut answers = HashMap::new();
    let q0 = &paper.questions[0];
    let q1 = &paper.questions[1];
    answers.insert(
        q0.id.clone(),
        StudentAnswer::Option(q0.correct_option_index.unwrap_or(0)),
    );
    answers.insert(
        q1.id.clone(),
        StudentAnswer::Option((q1.correct_option_index.unwrap_or(0) + 1) % 4),
    );

    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert_eq!(report.correct_count, 1);
    assert_eq!(report.incorrect_count, 1);
    assert_eq!(report.raw_score, 3.0);
}

#[test]
fn test_f20_jee_numerical_tolerance_matching() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_jee_paper("Physics", Some("Electrostatics"), 0, 1)
        .expect("Generate JEE numerical paper");
    let num_q = &paper.questions[0];
    assert_eq!(num_q.question_type, QuestionType::Numerical);

    // Within tolerance (20.05 vs 20.0 +- 0.1) -> +4.0
    let mut within_answers = HashMap::new();
    within_answers.insert(num_q.id.clone(), StudentAnswer::Numerical(20.05));
    let pass_report = gen.evaluate_submission(
        &paper,
        &StudentSubmission {
            answers: within_answers,
        },
    );
    assert_eq!(pass_report.correct_count, 1);
    assert_eq!(pass_report.raw_score, 4.0);

    // Outside tolerance (20.25) -> -1.0
    let mut outside_answers = HashMap::new();
    outside_answers.insert(num_q.id.clone(), StudentAnswer::Numerical(20.25));
    let fail_report = gen.evaluate_submission(
        &paper,
        &StudentSubmission {
            answers: outside_answers,
        },
    );
    assert_eq!(fail_report.incorrect_count, 1);
    assert_eq!(fail_report.raw_score, -1.0);
}

#[test]
fn test_f20_tnpsc_bilingual_options_and_keys() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_tnpsc_paper(None, 5)
        .expect("Generate TNPSC bilingual paper");
    assert_eq!(paper.exam_target, ExamTarget::TnpscGroup1);
    assert_eq!(paper.total_questions, 5);
    assert_eq!(paper.marking_scheme.correct_marks, 1.5);
    assert_eq!(paper.marking_scheme.incorrect_marks, 0.0);

    for q in &paper.questions {
        assert!(q.is_bilingual);
        assert!(!q.question_text.is_empty());
        assert!(q.question_ta.as_ref().is_some_and(|t| !t.is_empty()));
        assert_eq!(q.options.len(), 4);
        assert_eq!(q.options_ta.as_ref().map(|v| v.len()), Some(4));
        assert!(q.correct_option_index.is_some_and(|idx| idx < 4));
    }
}

#[test]
fn test_f20_upsc_multi_statement_elimination_format() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_upsc_paper(None, 5)
        .expect("Generate UPSC elimination paper");
    assert_eq!(paper.exam_target, ExamTarget::UpscCse);
    assert_eq!(paper.total_questions, 5);

    for q in &paper.questions {
        assert_eq!(q.question_type, QuestionType::MultiStatementElimination);
        assert!(q.statements.as_ref().is_some_and(|s| s.len() >= 3));
        assert!(q
            .elimination_strategy
            .as_ref()
            .is_some_and(|s| !s.is_empty()));
    }

    // Wrong choice must surface Elimination Insight
    let q0 = &paper.questions[0];
    let mut answers = HashMap::new();
    answers.insert(
        q0.id.clone(),
        StudentAnswer::Option((q0.correct_option_index.unwrap_or(0) + 1) % 4),
    );
    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert!(report.question_results[0]
        .distractor_or_elimination_insight
        .as_deref()
        .unwrap_or("")
        .contains("Elimination Insight"));
}

#[test]
fn test_f20_dynamic_exam_generator_question_counts() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let jee = gen
        .generate_jee_paper("Physics", Some("Electrostatics"), 3, 1)
        .expect("JEE paper");
    assert_eq!(jee.total_questions, 4);
    assert_eq!(jee.questions.len(), 4);

    let tnpsc = gen.generate_tnpsc_paper(None, 5).expect("TNPSC paper");
    assert_eq!(tnpsc.total_questions, 5);
    assert_eq!(tnpsc.questions.len(), 5);

    let upsc = gen.generate_upsc_paper(None, 5).expect("UPSC paper");
    assert_eq!(upsc.total_questions, 5);
    assert_eq!(upsc.questions.len(), 5);
}

// ===========================================================================
// FEATURE 21: Workspace Verification Benchmarks in tests/benchmarks/
// ===========================================================================

#[test]
fn test_f21_benchmark_directory_structure_integrity() {
    let b_dir = benchmark_dir();
    assert!(b_dir.exists(), "Benchmark dir must exist at {:?}", b_dir);
    assert!(b_dir.join("sample_exams").exists());
    assert!(b_dir.join("exam_rubrics").exists());
    assert!(b_dir.join("mcq_datasets").exists());
}

#[test]
fn test_f21_state_board_rubric_json_integrity() {
    let path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let content = fs::read_to_string(&path).expect("Read rubric");
    let v: Value = serde_json::from_str(&content).expect("Valid rubric json");
    assert_eq!(v["theory_max_marks"].as_u64(), Some(70));
}

#[test]
fn test_f21_jee_benchmark_dataset_integrity() {
    let path = benchmark_dir().join("mcq_datasets/jee_electrostatics.json");
    let content = fs::read_to_string(&path).expect("Read JEE dataset");
    let v: Value = serde_json::from_str(&content).expect("Valid JEE json");
    assert_eq!(v["questions"].as_array().unwrap().len(), 5);
}

#[test]
fn test_f21_tnpsc_benchmark_dataset_integrity() {
    let path = benchmark_dir().join("mcq_datasets/tnpsc_group1_polity.json");
    let content = fs::read_to_string(&path).expect("Read TNPSC dataset");
    let v: Value = serde_json::from_str(&content).expect("Valid TNPSC json");
    assert_eq!(v["questions"].as_array().unwrap().len(), 5);
}

#[test]
fn test_f21_upsc_benchmark_dataset_integrity() {
    let path = benchmark_dir().join("mcq_datasets/upsc_modern_history.json");
    let content = fs::read_to_string(&path).expect("Read UPSC dataset");
    let v: Value = serde_json::from_str(&content).expect("Valid UPSC json");
    assert_eq!(v["questions"].as_array().unwrap().len(), 5);
}
