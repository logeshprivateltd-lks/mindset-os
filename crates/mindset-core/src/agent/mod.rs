//! MindSet OS — Exam Scout AI Agent & Rubric Grader
//!
//! Subsystem for official exam forensics, ROI chapter mapping, dynamic question generation,
//! and multi-stage Proof-of-Study evaluation.

pub mod exam_scout;
pub mod ollama_grader;

pub use exam_scout::{
    ChapterRoi, ExamIntelligenceReport, ExamProfile, ExamScoutAgent, ExamSection, RoiQuadrant,
    SubjectBreakdown,
};
pub use ollama_grader::{
    AppliedQuestion, DynamicExamGenerator, FeynmanGradeResult, FeynmanGrader, FeynmanPrompt,
    GeneratedBossExam, RecallQuestion,
};
