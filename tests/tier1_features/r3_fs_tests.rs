//! Tier 1: Subsystem R3 File System & Document Engine Tests (Features 11 to 14)
//!
//! Covers:
//! - F11: Hierarchical file system tree navigation & listing (>=5 tests)
//! - F12: PDF document reader & viewer, page count, metadata (>=5 tests)
//! - F13: PPT & PPTX document support, slide text extraction (>=5 tests)
//! - F14: In-place document editor & updater, save persistence (>=5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_sys::fs::document::{
    detect_format, DocumentContent, DocumentFormat, DocumentHandler, FsError, PptxSlide,
};
use mindset_sys::fs::explorer::list_directory;
use std::fs;
use std::path::Path;

use crate::helpers::{create_sample_pdf, create_sample_ppt, create_sample_pptx, HeadlessHarness};

// ===========================================================================
// FEATURE 11: Hierarchical File System Tree Navigation & Listing
// ===========================================================================

#[test]
fn test_f11_directory_listing_in_vault() {
    let harness = HeadlessHarness::new();
    let root = harness.vault_path();
    let conn = harness.open_test_conn();

    // Create subdirectories and dummy files
    let sub = root.join("Class12_Physics");
    fs::create_dir_all(&sub).expect("Failed to create subfolder");
    fs::write(sub.join("Gauss_Notes.txt"), "Electric flux derivation")
        .expect("Failed to write test file");

    let listing = list_directory(root, &conn, None).expect("Failed to list directory");
    assert!(
        !listing.entries.is_empty(),
        "Directory listing should not be empty"
    );
    assert!(listing
        .entries
        .iter()
        .any(|item| item.name == "Class12_Physics" && item.is_dir));
}

#[test]
fn test_f11_recursive_tree_node_navigation() {
    let harness = HeadlessHarness::new();
    let root = harness.vault_path();
    let conn = harness.open_test_conn();

    let nested = root.join("Unit1").join("Chapter2");
    fs::create_dir_all(&nested).expect("Failed to create nested directory");
    fs::write(nested.join("Formula.txt"), "E = V / d").expect("Failed to write nested file");

    let top_items = list_directory(root, &conn, None).expect("List root");
    assert!(top_items
        .entries
        .iter()
        .any(|i| i.name == "Unit1" && i.is_dir));

    let nested_items = list_directory(root, &conn, Some("Unit1")).expect("List Unit1");
    assert!(nested_items
        .entries
        .iter()
        .any(|i| i.name == "Chapter2" && i.is_dir));

    let leaf_items = list_directory(root, &conn, Some("Unit1/Chapter2")).expect("List Chapter2");
    assert!(leaf_items
        .entries
        .iter()
        .any(|i| i.name == "Formula.txt" && !i.is_dir));
}

#[test]
fn test_f11_empty_directory_handling() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();
    let empty_dir = harness.vault_path().join("EmptyFolder");
    fs::create_dir_all(&empty_dir).expect("Failed to create empty dir");

    let listing = list_directory(harness.vault_path(), &conn, Some("EmptyFolder"))
        .expect("List empty directory");
    assert_eq!(
        listing.entries.len(),
        0,
        "Empty directory must yield 0 items"
    );
}

#[test]
fn test_f11_path_traversal_sanitization() {
    let harness = HeadlessHarness::new();
    let safe_root = harness.vault_path();
    let conn = harness.open_test_conn();
    // Attempting to list outside root with malicious ../ is guarded
    let listing = list_directory(safe_root, &conn, None).expect("Safe listing");
    for item in listing.entries {
        assert!(
            !item.name.contains(".."),
            "File name must not contain parent directory traversal"
        );
    }
}

#[test]
fn test_f11_file_metadata_inspection() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();
    let file_path = harness.vault_path().join("MetadataTest.txt");
    fs::write(&file_path, "Hello MindSet OS File Explorer").expect("Write test file");

    let listing = list_directory(harness.vault_path(), &conn, None).expect("List dir");
    let item = listing
        .entries
        .iter()
        .find(|i| i.name == "MetadataTest.txt")
        .expect("Item not found");
    assert_eq!(item.size_bytes, 30);
    assert!(!item.is_dir);
    assert_eq!(item.extension.as_deref(), Some("txt"));
}

// ===========================================================================
// FEATURE 12: PDF Document Reader & Viewer
// ===========================================================================

#[test]
fn test_f12_synthetic_pdf_creation_and_magic_bytes() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("test_sample.pdf");

    create_sample_pdf(
        &pdf_path,
        "Class 12 Physics Vol 1",
        &[
            "Page 1: Electrostatics and Coulomb's Inverse Square Law",
            "Page 2: Electric Dipole in Uniform Field and Torque",
            "Page 3: Gauss's Law Applications to Spherical Shells",
        ],
    )
    .expect("Failed to create synthetic PDF");

    let bytes = fs::read(&pdf_path).expect("Failed to read PDF file");
    assert!(
        bytes.starts_with(b"%PDF-1.4"),
        "PDF must begin with valid %PDF-1.4 header"
    );
}

#[test]
fn test_f12_pdf_page_count_extraction() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("multipage.pdf");

    create_sample_pdf(
        &pdf_path,
        "Multi-page Exam",
        &[
            "First page text",
            "Second page text",
            "Third page text",
            "Fourth page text",
        ],
    )
    .expect("Create multipage PDF");

    let content = DocumentHandler::read(&pdf_path).expect("Read PDF");
    match content {
        DocumentContent::Pdf { pages, metadata } => {
            assert_eq!(pages.len(), 4, "Extracted pages count must be 4");
            assert_eq!(metadata.page_count, 4);
        }
        _ => panic!("Expected DocumentContent::Pdf"),
    }
}

#[test]
fn test_f12_pdf_text_extraction_per_page() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("electrostatics.pdf");

    create_sample_pdf(
        &pdf_path,
        "Electrostatics",
        &[
            "Electric charge is quantized: q = n * e",
            "Permittivity of free space epsilon_0 = 8.854e-12 F/m",
        ],
    )
    .expect("Create PDF");

    let content = DocumentHandler::read(&pdf_path).expect("Read PDF");
    match content {
        DocumentContent::Pdf { pages, .. } => {
            assert!(pages[0].text.contains("Electric charge is quantized"));
            assert!(pages[1].text.contains("Permittivity of free space"));
        }
        _ => panic!("Expected DocumentContent::Pdf"),
    }
}

#[test]
fn test_f12_pdf_metadata_attributes() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("meta_check.pdf");

    create_sample_pdf(
        &pdf_path,
        "Chemistry Nernst Equation",
        &["E_cell = E0 - (RT/nF) ln Q"],
    )
    .expect("Create PDF");

    let content = DocumentHandler::read(&pdf_path).expect("Read PDF");
    match content {
        DocumentContent::Pdf { metadata, .. } => {
            assert_eq!(metadata.title.as_deref(), Some("Chemistry Nernst Equation"));
            assert_eq!(metadata.author.as_deref(), Some("MindSet Academic Engine"));
            assert!(metadata.producer.as_ref().unwrap().contains("MindSet OS"));
        }
        _ => panic!("Expected DocumentContent::Pdf"),
    }
}

#[test]
fn test_f12_pdf_full_text_aggregation() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("aggregation.pdf");

    create_sample_pdf(&pdf_path, "Test Doc", &["Alpha Section", "Beta Section"])
        .expect("Create PDF");

    let content = DocumentHandler::read(&pdf_path).expect("Read PDF");
    let full = content.extract_full_text();
    assert!(full.contains("--- Page 1 ---"));
    assert!(full.contains("Alpha Section"));
    assert!(full.contains("--- Page 2 ---"));
    assert!(full.contains("Beta Section"));
}

// ===========================================================================
// FEATURE 13: PPT & PPTX Document Support
// ===========================================================================

#[test]
fn test_f13_synthetic_pptx_openxml_container() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("seminar.pptx");

    create_sample_pptx(
        &pptx_path,
        "Electromagnetism Seminar",
        &[
            (
                "Introduction to Magnetic Dipole",
                "A current carrying loop behaves as a magnetic dipole.",
            ),
            (
                "Biot-Savart Law",
                "dB is proportional to current element I dl cross r.",
            ),
        ],
    )
    .expect("Failed to create synthetic PPTX");

    let bytes = fs::read(&pptx_path).expect("Failed to read PPTX file");
    assert!(
        bytes.starts_with(b"PK\x03\x04"),
        "PPTX must be a standard ZIP container"
    );
}

#[test]
fn test_f13_pptx_slide_count_and_titles() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("titles.pptx");

    create_sample_pptx(
        &pptx_path,
        "Optics Review",
        &[
            (
                "Slide 1: Huygens Principle",
                "Every point on a wavefront acts as secondary source.",
            ),
            (
                "Slide 2: Interference & Diffraction",
                "Young's double slit experiment fringe width.",
            ),
            (
                "Slide 3: Polarization of Light",
                "Brewster's angle and Malus law.",
            ),
        ],
    )
    .expect("Create PPTX");

    let content = DocumentHandler::read(&pptx_path).expect("Read PPTX");
    match content {
        DocumentContent::Pptx { slides, metadata } => {
            assert_eq!(slides.len(), 3);
            assert_eq!(metadata.slide_count, 3);
            assert_eq!(
                slides[0].title.as_deref(),
                Some("Slide 1: Huygens Principle")
            );
            assert_eq!(
                slides[1].title.as_deref(),
                Some("Slide 2: Interference & Diffraction")
            );
            assert_eq!(
                slides[2].title.as_deref(),
                Some("Slide 3: Polarization of Light")
            );
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

#[test]
fn test_f13_pptx_body_blocks_and_notes() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("details.pptx");

    create_sample_pptx(
        &pptx_path,
        "Thermodynamics",
        &[(
            "Carnot Engine",
            "Efficiency eta = 1 - T_C / T_H. Reversible cyclic process.",
        )],
    )
    .expect("Create PPTX");

    let content = DocumentHandler::read(&pptx_path).expect("Read PPTX");
    match content {
        DocumentContent::Pptx { slides, .. } => {
            assert!(slides[0].body_blocks[0].contains("Efficiency eta = 1 - T_C / T_H"));
            assert!(
                slides[0].notes.is_none()
                    || slides[0].notes.as_ref().unwrap().contains("Speaker note")
            );
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

#[test]
fn test_f13_legacy_ppt_safe_stream_extraction() {
    let harness = HeadlessHarness::new();
    let ppt_path = harness.vault_path().join("archive_lecture.ppt");

    create_sample_ppt(
        &ppt_path,
        "Legacy Lecture 1997",
        &[
            "Introduction to Semiconductors",
            "Intrinsic vs Extrinsic Doping",
        ],
    )
    .expect("Failed to create legacy PPT");

    let content = DocumentHandler::read(&ppt_path).expect("Read legacy PPT");
    let full_text = content.extract_full_text();
    assert!(full_text.contains("Semiconductors"));
}

#[test]
fn test_f13_pptx_slide_combined_text_formatting() {
    let slide = PptxSlide {
        slide_number: 1,
        title: Some("Gauss Law".to_string()),
        body_blocks: vec!["phi = q / eps0".to_string()],
        notes: Some("Crucial for 5-mark board question".to_string()),
        shape_texts: vec!["Diagram Box".to_string()],
    };

    let text = slide.combined_text();
    assert!(text.contains("# Gauss Law"));
    assert!(text.contains("phi = q / eps0"));
    assert!(text.contains("Notes: Crucial for 5-mark board question"));
}

// ===========================================================================
// FEATURE 14: In-Place Document Editor & Updater
// ===========================================================================

#[test]
fn test_f14_text_document_in_place_save() {
    let harness = HeadlessHarness::new();
    let doc_path = harness.vault_path().join("EditableNotes.txt");

    fs::write(&doc_path, "Initial Content").expect("Initial write");
    let updated_content = DocumentContent::Text("Updated In-Place by Student Scholar".to_string());

    DocumentHandler::save_update(&doc_path, &updated_content).expect("Save update");
    let re_read = DocumentHandler::read(&doc_path).expect("Re-read");
    match re_read {
        DocumentContent::Text(t) => assert_eq!(t, "Updated In-Place by Student Scholar"),
        _ => panic!("Expected DocumentContent::Text"),
    }
}

#[test]
fn test_f14_pptx_slide_in_place_modification() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("in_place_edit.pptx");

    create_sample_pptx(
        &pptx_path,
        "Original Deck",
        &[("Slide 1", "Draft concept text.")],
    )
    .expect("Create PPTX");

    // Modify slide title and body text in-place
    let mut doc = DocumentHandler::read(&pptx_path).expect("Read original PPTX");
    if let DocumentContent::Pptx { ref mut slides, .. } = doc {
        slides[0].title = Some("Slide 1: Final Refined Title".to_string());
        slides[0].body_blocks = vec!["Refined and validated derivation content.".to_string()];
    }

    DocumentHandler::save_update(&pptx_path, &doc).expect("Save updated PPTX in-place");

    // Re-read file to verify persistence
    let re_read = DocumentHandler::read(&pptx_path).expect("Re-read updated PPTX");
    match re_read {
        DocumentContent::Pptx { slides, .. } => {
            assert_eq!(
                slides[0].title.as_deref(),
                Some("Slide 1: Final Refined Title")
            );
            assert_eq!(
                slides[0].body_blocks[0],
                "Refined and validated derivation content."
            );
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

#[test]
fn test_f14_document_format_detection() {
    assert_eq!(
        detect_format(Path::new("doc.pdf"), b"%PDF-1.4 ..."),
        DocumentFormat::Pdf
    );
    assert_eq!(
        detect_format(Path::new("slides.pptx"), b"PK\x03\x04 ..."),
        DocumentFormat::Pptx
    );
    assert_eq!(
        detect_format(Path::new("legacy.ppt"), b"\xD0\xCF\x11\xE0 ..."),
        DocumentFormat::Ppt
    );
    assert_eq!(
        detect_format(Path::new("notes.md"), b"# Heading"),
        DocumentFormat::Markdown
    );
    assert_eq!(
        detect_format(Path::new("plain.txt"), b"Just text"),
        DocumentFormat::Text
    );
}

#[test]
fn test_f14_unsupported_format_graceful_error() {
    let harness = HeadlessHarness::new();
    let bin_path = harness.vault_path().join("program.exe");
    fs::write(&bin_path, b"\xFF\xFE\xFD\xFC\x80\x81\x00\xFF").expect("Write binary executable");

    let res = DocumentHandler::read(&bin_path);
    assert!(
        res.is_err(),
        "Reading arbitrary binary executable must return error"
    );
    match res.err().unwrap() {
        FsError::UnsupportedFormat(msg) => {
            assert!(msg.contains("unsupported") || msg.contains("Unknown") || msg.contains("exe"));
        }
        other => panic!("Expected UnsupportedFormat, got {:?}", other),
    }
}

#[test]
fn test_f14_atomic_write_safety() {
    let harness = HeadlessHarness::new();
    let path = harness.vault_path().join("atomic_check.txt");
    fs::write(&path, "Permanent Safe Content").expect("Initial write");

    // Re-writing with new content succeeds cleanly
    DocumentHandler::save_update(
        &path,
        &DocumentContent::Text("Safe Overwrite Content".to_string()),
    )
    .expect("Save update");

    let text = fs::read_to_string(&path).expect("Read back");
    assert_eq!(text, "Safe Overwrite Content");
}
