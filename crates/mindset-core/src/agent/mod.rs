//! MindSet OS — Exam Scout AI Agent & Rubric Grader
//!
//! Subsystem for official exam forensics, ROI chapter mapping, dynamic question generation,
//! and multi-stage Proof-of-Study evaluation.

pub mod exam_reviewer;
pub mod exam_scout;
pub mod mcq_generator;
pub mod ollama_grader;

pub use exam_reviewer::{
    CriterionEvaluation, CriterionStatus, DeductionRecord, DeductionRule, EvaluationSheet,
    ExamError, ExamReviewer, ExamRubric, GapAnalysis, LossCategorySummary, QuestionEvaluation,
    RubricCriterion, RubricQuestion, SectionScore,
};
pub use exam_scout::{
    ChapterRoi, ExamIntelligenceReport, ExamProfile, ExamScoutAgent, ExamSection, RoiQuadrant,
    SubjectBreakdown,
};
pub use mcq_generator::{
    CompetitiveMcqGenerator, CompetitiveQuestion, DifficultyLevel, DistractorNote, ExamTarget,
    MarkingScheme, QuestionResult, QuestionType, StudentAnswer, StudentSubmission,
    TestEvaluationReport, TestPaper,
};
pub use ollama_grader::{
    AppliedQuestion, DynamicExamGenerator, FeynmanGradeResult, FeynmanGrader, FeynmanPrompt,
    GeneratedBossExam, RecallQuestion,
};
