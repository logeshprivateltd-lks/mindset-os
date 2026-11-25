//! Mock Data & Synthetic Document Generators for MindSet OS Tests
//!
//! Provides deterministic generator functions for synthetic PDFs, PPTX slides,
//! student profiles, Odyssey nodes, and exam evaluation sheets.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_core::odyssey::{NodeStatus, NodeType, OdysseyNode};
use mindset_os::storage::db::{CapsuleRecord, UserProfile};
use mindset_sys::fs::document::{
    DocumentContent, DocumentHandler, PdfMetadata, PdfPage, PptxMetadata, PptxSlide,
};
use std::path::{Path, PathBuf};

/// Resolves the repository workspace root reliably from CARGO_MANIFEST_DIR (`tests/` -> parent).
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Resolves a workspace-relative asset or documentation path reliably across Windows and Linux CI.
pub fn workspace_asset(rel_path: &str) -> PathBuf {
    let direct = PathBuf::from(rel_path);
    if direct.exists() {
        return direct;
    }
    workspace_root().join(rel_path)
}

/// Resolves the benchmarks directory reliably regardless of invocation working directory
pub fn benchmark_dir() -> PathBuf {
    let p1 = PathBuf::from("benchmarks");
    if p1.exists() {
        return p1;
    }
    let p2 = PathBuf::from("tests/benchmarks");
    if p2.exists() {
        return p2;
    }
    let manifest_bench = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benchmarks");
    if manifest_bench.exists() {
        return manifest_bench;
    }
    workspace_root().join("tests/benchmarks")
}

/// Generates a realistic high-scoring student profile targeting 590/600
pub fn create_test_profile(name: &str, score_target: u32) -> UserProfile {
    UserProfile {
        name: name.to_string(),
        career_track: "Class12BoardsTN600".to_string(),
        target_score_value: score_target,
        target_score_max: 600,
        exam_date: "2027-03-01".to_string(),
        weekday_study_hours: 4,
        weekend_study_hours: 7,
        peak_energy_window: "Morning (06:00 - 08:30)".to_string(),
        weak_topics: vec![
            "Gauss Law & Flux".to_string(),
            "Electrochemistry Nernst Eq".to_string(),
            "Vector Cross Products".to_string(),
        ],
        coaching_style: "Friendly Thanglish Bro".to_string(),
        maindy_style: "thanglish".to_string(),
        onboarding_completed: true,
        mild_motion_enabled: true,
        target_label: "TN Class 12 Board Target • 590/600 Target Protected".to_string(),
        ..Default::default()
    }
}

/// Generates a mock Study Capsule for testing
pub fn create_test_capsule(id: &str, subject: &str, unit: &str) -> CapsuleRecord {
    CapsuleRecord {
        id: id.to_string(),
        target_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        subject: subject.to_string(),
        unit: unit.to_string(),
        estimated_minutes: 40,
        xp_reward: 50,
        status: "In Progress".to_string(),
        stage: "Heavy Lift Concept".to_string(),
    }
}

/// Generates a test Odyssey journey node
pub fn create_test_odyssey_node(id: &str, title: &str, marks: u32, max_marks: u32) -> OdysseyNode {
    let mut node = OdysseyNode {
        id: id.to_string(),
        unit_id: "unit-1-electromagnetism".to_string(),
        unit_title: "Unit 1: Electromagnetism & Potentials".to_string(),
        title: title.to_string(),
        subject: "Physics".to_string(),
        description: "Test description for syllabus progression".to_string(),
        node_type: NodeType::Theory,
        status: NodeStatus::InProgress,
        marks: 0,
        max_marks,
        xp: 50,
        stars_earned: 0,
        is_crown_mastered: false,
        estimated_minutes: 35,
    };
    node.update_evaluation(marks);
    node
}

/// Creates a valid synthetic PDF document on disk using pure-Rust PdfEngine
pub fn create_sample_pdf(
    path: &Path,
    title: &str,
    page_texts: &[&str],
) -> Result<(), mindset_sys::fs::document::FsError> {
    let pages: Vec<PdfPage> = page_texts
        .iter()
        .enumerate()
        .map(|(i, text)| PdfPage {
            page_number: i + 1,
            text: text.to_string(),
        })
        .collect();
    let metadata = PdfMetadata {
        title: Some(title.to_string()),
        author: Some("MindSet Academic Engine".to_string()),
        page_count: pages.len(),
        creation_date: Some("2026-03-01".to_string()),
        producer: Some("MindSet OS PDF Engine".to_string()),
        subject: None,
    };
    DocumentHandler::save_update(path, &DocumentContent::Pdf { pages, metadata })
}

/// Creates a valid synthetic PPTX presentation on disk using pure-Rust PptxEngine
pub fn create_sample_pptx(
    path: &Path,
    title: &str,
    slides: &[(&str, &str)],
) -> Result<(), mindset_sys::fs::document::FsError> {
    let pptx_slides: Vec<PptxSlide> = slides
        .iter()
        .enumerate()
        .map(|(i, (stitle, sbody))| PptxSlide {
            slide_number: i + 1,
            title: Some(stitle.to_string()),
            body_blocks: vec![sbody.to_string()],
            notes: None,
            shape_texts: Vec::new(),
        })
        .collect();
    let metadata = PptxMetadata {
        title: Some(title.to_string()),
        author: Some("MindSet Presentation Engine".to_string()),
        slide_count: pptx_slides.len(),
        slide_width: Some(9144000),
        slide_height: Some(6858000),
    };
    DocumentHandler::save_update(
        path,
        &DocumentContent::Pptx {
            slides: pptx_slides,
            metadata,
        },
    )
}

/// Creates a valid synthetic legacy PPT file on disk
pub fn create_sample_ppt(
    path: &Path,
    _title: &str,
    slide_texts: &[&str],
) -> Result<(), mindset_sys::fs::document::FsError> {
    // Generate valid legacy PPT with CFB header and TextBytesAtom records (0x0FA9)
    let mut bytes = vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]; // OLE magic
    for text in slide_texts {
        // Slide container header: 0x0F9F
        bytes.extend_from_slice(&[0x0F, 0x00, 0x9F, 0x0F, 0x00, 0x00, 0x00, 0x00]);
        // TextBytesAtom: 0x0FA9
        let text_bytes = text.as_bytes();
        let len = text_bytes.len() as u32;
        bytes.extend_from_slice(&[0x00, 0x00, 0xA9, 0x0F]);
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(text_bytes);
    }
    std::fs::write(path, bytes)?;
    Ok(())
}
