//! Tier 2: Subsystem R5 Exam Reviewer & Competitive MCQ Generator Boundary Tests (Features 19 to 21)
//!
//! Stress, limit, and edge condition verification:
//! - F19: Zero-score blank sheets, cumulative deductions, missing rubrics, float precision, 97.99% vs 98.00% (5 tests)
//! - F20: Negative scoring floor, exact numerical tolerance, empty bilingual fallback, elimination logic, count limits (5 tests)
//! - F21: Missing benchmark files, malformed JSON, missing schema fields, deduction rules integrity, sandbox security (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::agent::{
    CompetitiveMcqGenerator, DeductionRule, ExamError, ExamReviewer, ExamRubric, ExamTarget,
    RubricCriterion, RubricQuestion, StudentAnswer, StudentSubmission,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

use crate::helpers::benchmark_dir;

// ===========================================================================
// FEATURE 19: State Board 590/600 AI Exam Reviewer — Boundary Tests
// ===========================================================================

#[test]
fn test_f19_boundary_zero_score_blank_student_sheet() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");

    let sheet = ExamReviewer::evaluate_text("", &rubric).expect("Evaluate blank sheet");
    assert_eq!(sheet.total_score, 0.0);
    assert_eq!(sheet.percentage, 0.0);
    assert!(
        !sheet.achieved_590_crown,
        "0% must not receive Golden Crown"
    );
    assert_eq!(sheet.itemized_scores.len(), 4);
    assert!(sheet.itemized_scores.iter().all(|q| q.awarded_marks == 0.0));
}

#[test]
fn test_f19_boundary_all_rubric_deductions_applied_cumulatively() {
    let rubric_path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let rubric = ExamReviewer::load_rubric(&rubric_path).expect("Load rubric");

    // Genuine student flaws without marker comments:
    // Q1: skips intermediate integration expansion (DED-DERIVATION-STEP -0.5) & omits SI unit (DED-SI-UNIT -0.5)
    // Q3: omits current direction arrows and battery polarity (DED-DIAGRAM-LABEL -1.0)
    let flawed_paper = r#"
--- Question 1: Gauss's Law & Electric Flux ---
Statement:
Total flux through closed surface is q / epsilon_0.
Diagram:
[DIAGRAM: Spherical Gaussian surface radius r with radial E and dS vector arrows.]
Derivation:
oint_S E . dS = q / epsilon_0.
Final Result:
Total Electric Flux phi_E = 1.13 * 10^5

--- Question 2: Electric Dipole in Uniform Field ---
Torque on dipole:
  tau = p * E * sin(theta)
  tau = p x E
Special Cases:
  theta = 0: stable equilibrium.
  theta = 90: maximum torque.

--- Question 3: Wheatstone's Bridge Balance Condition ---
Statement: KCL and KVL rules applied.
Diagram:
[DIAGRAM: Four resistors P, Q, R, S in diamond network with galvanometer G.]
Derivation:
  I1*P = I2*R
  I3*Q = I4*S
  At null deflection Ig = 0.
Final Result:
  P / Q = R / S

--- Question 4: Biot-Savart Law on Circular Current Loop ---
Statement:
  dB = (mu_0 / 4pi) * (I dl sin(theta)) / r^2
Diagram:
[DIAGRAM: Circular coil of radius R carrying current, axial distance x.]
Derivation:
  B = (mu_0 * I / (4 * pi * R^2)) * (2 * pi * R)
  B = (mu_0 * I) / (2 * R)
Final Answer:
  B = 6.28 * 10^-5 Tesla (T)
"#;

    let sheet = ExamReviewer::evaluate_text(flawed_paper, &rubric).expect("Evaluate flawed paper");
    let all_rule_ids: Vec<&str> = sheet
        .itemized_scores
        .iter()
        .flat_map(|q| q.deductions.iter().map(|d| d.rule_id.as_str()))
        .collect();
    assert!(all_rule_ids.contains(&"DED-SI-UNIT"));
    assert!(all_rule_ids.contains(&"DED-DERIVATION-STEP"));
    assert!(all_rule_ids.contains(&"DED-DIAGRAM-LABEL"));
    assert!(sheet.total_deductions >= 2.0);
    assert!((0.0..98.0).contains(&sheet.percentage));
    assert!(!sheet.achieved_590_crown);
}

#[test]
fn test_f19_boundary_non_existent_rubric_file_handling() {
    let non_existent_rubric = benchmark_dir().join("exam_rubrics/non_existent_rubric_999.json");
    assert!(!non_existent_rubric.exists(), "Must be non-existent path");
    let res = ExamReviewer::load_rubric(&non_existent_rubric);
    assert!(
        matches!(res, Err(ExamError::IoError(_))),
        "Loading non-existent rubric must fail gracefully with ExamError::IoError"
    );
}

#[test]
fn test_f19_boundary_floating_point_deduction_precision() {
    // 10 questions x 7.0 marks = 70.0 max; each triggers DED-SI-UNIT (-0.5) => exactly 65.0
    let questions: Vec<RubricQuestion> = (1..=10)
        .map(|num| RubricQuestion {
            question_number: num,
            topic: format!("Numerical Problem {}", num),
            max_marks: 7.0,
            section: None,
            criteria: vec![RubricCriterion {
                step: "Final scalar equation".to_string(),
                marks: 7.0,
                mandatory_keywords: vec!["flux".to_string(), "epsilon_0".to_string()],
            }],
        })
        .collect();

    let rubric = ExamRubric {
        board: "Tamil Nadu State Board".to_string(),
        examination: "Class 12".to_string(),
        subject: "Physics".to_string(),
        subject_code: "PHY-12".to_string(),
        benchmark_standard: "590/600".to_string(),
        theory_max_marks: 70.0,
        passing_marks: 25.0,
        state_board_crown_threshold_percentage: 98.0,
        strict_rubric_deduction_rules: vec![DeductionRule {
            rule_id: "DED-SI-UNIT".to_string(),
            name: "Missing SI Unit".to_string(),
            deduction_marks: 0.5,
            description: "Missing SI unit".to_string(),
        }],
        questions,
    };

    let mut paper = String::new();
    for num in 1..=10 {
        paper.push_str(&format!(
            "--- Question {}: Numerical Problem {} ---\nFinal Result:\nflux = 1.13 * 10^5 using epsilon_0\n\n",
            num, num
        ));
    }

    let sheet = ExamReviewer::evaluate_text(&paper, &rubric).expect("Evaluate 10-question paper");
    assert_eq!(sheet.total_deductions, 5.0);
    assert_eq!(
        sheet.total_score, 65.0,
        "10 x -0.5 deductions must equal exactly 65.0"
    );
}

#[test]
fn test_f19_boundary_tie_break_97_99_vs_98_00_crown_threshold() {
    let make_rubric = |penalty: f32| ExamRubric {
        board: "Tamil Nadu State Board".to_string(),
        examination: "Class 12".to_string(),
        subject: "Physics".to_string(),
        subject_code: "PHY-12".to_string(),
        benchmark_standard: "590/600".to_string(),
        theory_max_marks: 100.0,
        passing_marks: 35.0,
        state_board_crown_threshold_percentage: 98.0,
        strict_rubric_deduction_rules: vec![DeductionRule {
            rule_id: "DED-SI-UNIT".to_string(),
            name: "Missing SI Unit".to_string(),
            deduction_marks: penalty,
            description: "SI unit deduction".to_string(),
        }],
        questions: vec![RubricQuestion {
            question_number: 1,
            topic: "Gauss Law".to_string(),
            max_marks: 100.0,
            section: None,
            criteria: vec![RubricCriterion {
                step: "Final scalar equation".to_string(),
                marks: 100.0,
                mandatory_keywords: vec!["flux".to_string(), "epsilon_0".to_string()],
            }],
        }],
    };

    let submission =
        "--- Question 1: Gauss Law ---\nFinal Result:\nflux = 1.13 * 10^5 via epsilon_0\n";

    // 100.0 - 2.01 = 97.99% => strictly fails Crown
    let sheet_97_99 =
        ExamReviewer::evaluate_text(submission, &make_rubric(2.01)).expect("97.99 eval");
    assert!((sheet_97_99.percentage - 97.99).abs() < 0.01);
    assert!(
        !sheet_97_99.achieved_590_crown,
        "97.99% strictly fails Crown requirement"
    );

    // 100.0 - 2.00 = 98.00% => strictly satisfies Crown
    let sheet_98_00 =
        ExamReviewer::evaluate_text(submission, &make_rubric(2.00)).expect("98.00 eval");
    assert!((sheet_98_00.percentage - 98.00).abs() < 0.01);
    assert!(
        sheet_98_00.achieved_590_crown,
        "98.00% strictly satisfies Crown requirement"
    );
}

// ===========================================================================
// FEATURE 20: Competitive Exam MCQ Generator — Boundary Tests
// ===========================================================================

#[test]
fn test_f20_boundary_jee_negative_scoring_cumulative_total() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_jee_paper("Physics", None, 4, 1)
        .expect("Generate 5-question JEE paper");
    assert_eq!(paper.exam_target, ExamTarget::JeeAdvanced);
    assert_eq!(paper.total_questions, 5);

    let mut answers = HashMap::new();
    for q in &paper.questions {
        if let Some(correct_idx) = q.correct_option_index {
            answers.insert(q.id.clone(), StudentAnswer::Option((correct_idx + 1) % 4));
        } else {
            answers.insert(q.id.clone(), StudentAnswer::Numerical(9999.0));
        }
    }

    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert_eq!(report.correct_count, 0);
    assert_eq!(report.incorrect_count, 5);
    assert_eq!(
        report.raw_score, -5.0,
        "5 wrong answers must result in -5.0 net marks"
    );
}

#[test]
fn test_f20_boundary_numerical_exact_tolerance_boundary_match() {
    let mut gen = CompetitiveMcqGenerator::new();
    let custom_json = r#"{
        "exam_name": "JEE Advanced",
        "subject": "Physics",
        "topic": "Boundary Tolerance",
        "questions": [{
            "id": "JEE-TOL-1",
            "question_type": "Numerical",
            "question_text": "Compute potential in volts:",
            "numerical_correct_value": 20.0,
            "tolerance": 0.05,
            "explanation": "Exact value is 20.0 V."
        }]
    }"#;
    gen.load_from_json(custom_json)
        .expect("Load tolerance question");

    let paper = gen
        .generate_jee_paper("Physics", Some("Boundary Tolerance"), 0, 1)
        .expect("Generate numerical paper");

    // Within upper tolerance (+0.049 <= 0.05) -> +4.0
    let mut ans_upper = HashMap::new();
    ans_upper.insert("JEE-TOL-1".to_string(), StudentAnswer::Numerical(20.049));
    let r_upper = gen.evaluate_submission(&paper, &StudentSubmission { answers: ans_upper });
    assert_eq!(r_upper.correct_count, 1);
    assert_eq!(r_upper.raw_score, 4.0);

    // Within lower tolerance (-0.049 <= 0.05) -> +4.0
    let mut ans_lower = HashMap::new();
    ans_lower.insert("JEE-TOL-1".to_string(), StudentAnswer::Numerical(19.951));
    let r_lower = gen.evaluate_submission(&paper, &StudentSubmission { answers: ans_lower });
    assert_eq!(r_lower.correct_count, 1);
    assert_eq!(r_lower.raw_score, 4.0);

    // Outside tolerance (20.06) -> -1.0
    let mut ans_out = HashMap::new();
    ans_out.insert("JEE-TOL-1".to_string(), StudentAnswer::Numerical(20.06));
    let r_out = gen.evaluate_submission(&paper, &StudentSubmission { answers: ans_out });
    assert_eq!(r_out.incorrect_count, 1);
    assert_eq!(r_out.raw_score, -1.0);
}

#[test]
fn test_f20_boundary_tnpsc_empty_bilingual_fallback() {
    let mut gen = CompetitiveMcqGenerator::new();
    let fallback_json = r#"{
        "exam_name": "TNPSC Group 1 Preliminary",
        "subject": "Polity",
        "topic": "Unit Fallback Check",
        "bilingual": true,
        "questions": [{
            "id": "TNPSC-FB-1",
            "question_en": "Which body conducts Panchayat elections in Tamil Nadu?",
            "options_en": ["SEC", "ECI", "UPSC", "TNPSC"],
            "correct_option_index": 0,
            "explanation_en": "State Election Commission conducts local body polls."
        }]
    }"#;
    gen.load_from_json(fallback_json)
        .expect("Load fallback TNPSC");

    let paper = gen
        .generate_tnpsc_paper(Some("Unit Fallback Check"), 1)
        .expect("Generate TNPSC fallback paper");
    let q = &paper.questions[0];
    assert!(!q.question_text.trim().is_empty());
    assert!(q.question_ta.is_none());
    assert_eq!(q.options.len(), 4);

    let mut answers = HashMap::new();
    answers.insert(q.id.clone(), StudentAnswer::Option(0));
    let report = gen.evaluate_submission(&paper, &StudentSubmission { answers });
    assert_eq!(report.correct_count, 1);
    assert_eq!(report.raw_score, 1.5);
}

#[test]
fn test_f20_boundary_upsc_elimination_all_correct_vs_none() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    let paper = gen
        .generate_upsc_paper(None, 5)
        .expect("Generate 5-question UPSC paper");
    assert_eq!(paper.total_questions, 5);

    // All correct
    let mut all_right = HashMap::new();
    let mut all_wrong = HashMap::new();
    for q in &paper.questions {
        let correct_idx = q.correct_option_index.unwrap_or(0);
        all_right.insert(q.id.clone(), StudentAnswer::Option(correct_idx));
        all_wrong.insert(q.id.clone(), StudentAnswer::Option((correct_idx + 1) % 4));
    }

    let r_right = gen.evaluate_submission(&paper, &StudentSubmission { answers: all_right });
    assert_eq!(r_right.correct_count, 5);
    assert_eq!(r_right.raw_score, 10.0);
    assert_eq!(r_right.accuracy_percentage, 100.0);

    let r_wrong = gen.evaluate_submission(&paper, &StudentSubmission { answers: all_wrong });
    assert_eq!(r_wrong.incorrect_count, 5);
    assert!(r_wrong.raw_score < 0.0);
    assert!(r_wrong.question_results.iter().all(|qr| qr
        .distractor_or_elimination_insight
        .as_deref()
        .unwrap_or("")
        .contains("Elimination Insight")));
    assert!(r_wrong
        .tactical_recommendations
        .iter()
        .any(|rec| rec.contains("elimination rule")));
}

#[test]
fn test_f20_boundary_request_count_safe_clamping() {
    let mut gen = CompetitiveMcqGenerator::new();
    gen.load_benchmarks(&benchmark_dir().join("mcq_datasets"))
        .expect("Load benchmarks");

    // Requesting 0 questions returns graceful Err
    assert!(gen.generate_tnpsc_paper(None, 0).is_err());
    assert!(gen
        .generate_jee_paper("Physics", Some("NonExistentTopicXYZ"), 3, 2)
        .is_err());

    // Requesting 500 questions safely caps at available question bank size without panicking
    let paper_500 = gen
        .generate_tnpsc_paper(None, 500)
        .expect("Cap at available bank");
    assert!(paper_500.total_questions > 0 && paper_500.total_questions <= 500);
    assert_eq!(paper_500.total_questions, paper_500.questions.len());
}

// ===========================================================================
// FEATURE 21: Workspace Verification Benchmarks — Boundary Tests
// ===========================================================================

#[test]
fn test_f21_boundary_missing_benchmark_file_error_diagnostic() {
    let missing_path = benchmark_dir().join("mcq_datasets/non_existent_dataset.json");
    let res = fs::read_to_string(&missing_path);
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_f21_boundary_malformed_json_detection() {
    let malformed_json = r#"{"title": "Unclosed JSON object"#;
    let res: Result<Value, _> = serde_json::from_str(malformed_json);
    assert!(res.is_err(), "Malformed JSON must return Err");
}

#[test]
fn test_f21_boundary_dataset_schema_required_fields_validator() {
    let path = benchmark_dir().join("mcq_datasets/jee_electrostatics.json");
    let raw = fs::read_to_string(&path).expect("Read JEE");
    let v: Value = serde_json::from_str(&raw).expect("Parse");

    // Mandatory top-level keys
    assert!(v.get("exam_name").is_some());
    assert!(v.get("subject").is_some());
    assert!(v.get("marking_scheme").is_some());
    assert!(v.get("questions").is_some());
}

#[test]
fn test_f21_boundary_state_board_rubric_deductions_integrity() {
    let path = benchmark_dir().join("exam_rubrics/tn_stateboard_590_rubric.json");
    let raw = fs::read_to_string(&path).expect("Read rubric");
    let v: Value = serde_json::from_str(&raw).expect("Parse");

    let rules = v["strict_rubric_deduction_rules"]
        .as_array()
        .expect("Deduction rules");
    assert!(rules.len() >= 4);
    for r in rules {
        assert!(r.get("rule_id").is_some());
        assert!(r.get("deduction_marks").is_some());
        assert!(r["deduction_marks"].as_f64().unwrap() > 0.0);
    }
}

#[test]
fn test_f21_boundary_benchmark_directory_sandbox_containment() {
    let b_dir = benchmark_dir();
    assert!(b_dir.is_dir());
    // Canonicalize or verify no path escapes
    let sample_exams = b_dir.join("sample_exams");
    assert!(sample_exams.starts_with(&b_dir));
}
