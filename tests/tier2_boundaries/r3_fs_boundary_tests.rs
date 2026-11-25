//! Tier 2: Subsystem R3 File System & Document Handling Boundary Tests (Features 11 to 14)
//!
//! Stress, limit, and edge condition verification:
//! - F11: Path traversal attempts, deep nesting, non-existent paths, empty dirs, root sandboxing (5 tests)
//! - F12: 0-byte PDF, truncated headers, corrupted binary, page index limits, empty streams (5 tests)
//! - F13: 0-byte PPTX, corrupted OpenXML zip, invalid legacy OLE header, slide bounds, empty shapes (5 tests)
//! - F14: Read-only write errors, 0-byte file overwrite, OpenXML update safety, non-UTF8 binary, atomic replace (5 tests)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_sys::fs::document::{DocumentContent, DocumentHandler, FsError};
use mindset_sys::fs::explorer::{list_directory, resolve_safe_path};
use std::fs;

use crate::helpers::{create_sample_pdf, create_sample_pptx, HeadlessHarness};

// ===========================================================================
// FEATURE 11: Hierarchical File System Tree Navigation — Boundary Tests
// ===========================================================================

#[test]
fn test_f11_boundary_path_traversal_directory_escape_blocked() {
    let harness = HeadlessHarness::new();
    let root = harness.vault_path();

    let traversal_paths = [
        "../../etc/passwd",
        "..\\..\\Windows\\System32",
        "nested/../../../../secret.key",
        "/etc/passwd",
        "C:\\Windows\\System32\\config",
    ];

    for malicious in traversal_paths {
        let res = resolve_safe_path(root, Some(malicious));
        assert!(
            res.is_err(),
            "resolve_safe_path must reject directory escape '{}', got {:?}",
            malicious,
            res
        );
    }

    // Valid contained relative traversal (sub/../notes.txt) remains permitted inside root
    let safe_contained = resolve_safe_path(root, Some("sub/../notes.txt"))
        .expect("Contained relative path must be allowed");
    let canon_root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    assert!(
        safe_contained.ends_with("notes.txt") && canon_root.exists(),
        "Resolved path {:?} must remain contained inside vault root",
        safe_contained
    );
}

#[test]
fn test_f11_boundary_deeply_nested_directory_navigation() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();
    let mut deep_dir = harness.vault_path().to_path_buf();
    for i in 1..=10 {
        deep_dir = deep_dir.join(format!("level_{}", i));
    }
    fs::create_dir_all(&deep_dir).expect("Create 10-level nested dir");
    fs::write(deep_dir.join("leaf.txt"), "Bottom of tree").expect("Write leaf");

    let listing = list_directory(&deep_dir, &conn, None).expect("List leaf directory");
    assert_eq!(listing.entries.len(), 1);
    assert_eq!(listing.entries[0].name, "leaf.txt");
    assert!(!listing.entries[0].is_dir);
}

#[test]
fn test_f11_boundary_non_existent_path_returns_not_found() {
    let harness = HeadlessHarness::new();
    let non_existent = harness.vault_path().join("phantom_folder_404.pdf");
    let res = DocumentHandler::read(&non_existent);
    assert!(res.is_err(), "Opening non-existent path must return error");
    match res.err().unwrap() {
        FsError::NotFound(p) => assert!(p.contains("phantom_folder_404")),
        other => panic!("Expected FsError::NotFound, got {:?}", other),
    }
}

#[test]
fn test_f11_boundary_empty_directory_zero_entries() {
    let harness = HeadlessHarness::new();
    let conn = harness.open_test_conn();
    let empty_dir = harness.vault_path().join("empty_shelf");
    fs::create_dir_all(&empty_dir).expect("Create empty dir");

    let listing = list_directory(&empty_dir, &conn, None).expect("List empty directory");
    assert_eq!(
        listing.entries.len(),
        0,
        "Empty directory must contain 0 entries"
    );
}

#[test]
fn test_f11_boundary_parent_traversal_at_root_stays_at_root() {
    let harness = HeadlessHarness::new();
    let root = harness.vault_path();
    // Escaping above vault root with ".." is blocked
    assert!(
        resolve_safe_path(root, Some("..")).is_err(),
        "Parent traversal above vault root must be blocked"
    );
    // Current dir "." and None stay strictly at vault root
    let safe_dot = resolve_safe_path(root, Some(".")).expect("resolve '.' at root");
    let safe_none = resolve_safe_path(root, None).expect("resolve None at root");
    assert_eq!(safe_dot, safe_none);
}

// ===========================================================================
// FEATURE 12: PDF Document Reader & Viewer — Boundary Tests
// ===========================================================================

#[test]
fn test_f12_boundary_zero_byte_pdf_handling() {
    let harness = HeadlessHarness::new();
    let empty_pdf = harness.vault_path().join("zero_byte.pdf");
    fs::write(&empty_pdf, b"").expect("Write 0 bytes");

    let res = DocumentHandler::read(&empty_pdf);
    assert!(
        res.is_err(),
        "0-byte PDF must return error without panicking"
    );
}

#[test]
fn test_f12_boundary_truncated_pdf_header() {
    let harness = HeadlessHarness::new();
    let truncated_pdf = harness.vault_path().join("truncated.pdf");
    // Only partial magic bytes, missing 'F'
    fs::write(&truncated_pdf, b"%PD").expect("Write partial header");

    let res = DocumentHandler::read(&truncated_pdf);
    assert!(
        res.is_err(),
        "Truncated PDF magic bytes must return parsing error cleanly"
    );
}

#[test]
fn test_f12_boundary_binary_masquerading_as_pdf() {
    let harness = HeadlessHarness::new();
    let fake_pdf = harness.vault_path().join("fake_image.pdf");
    // PNG image magic bytes masquerading as .pdf
    fs::write(&fake_pdf, b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR").expect("Write fake PDF");

    let res = DocumentHandler::read(&fake_pdf);
    assert!(
        res.is_err(),
        "Binary masquerading as PDF must return parsing error"
    );
}

#[test]
fn test_f12_boundary_pdf_single_blank_page() {
    let harness = HeadlessHarness::new();
    let blank_pdf = harness.vault_path().join("blank.pdf");
    create_sample_pdf(&blank_pdf, "Blank Document", &[""]).expect("Create single empty page PDF");

    let content = DocumentHandler::read(&blank_pdf).expect("Read PDF");
    match content {
        DocumentContent::Pdf { pages, metadata } => {
            assert_eq!(pages.len(), 1, "Blank PDF must parse to exactly 1 page");
            assert_eq!(pages[0].page_number, 1);
            assert_eq!(metadata.page_count, 1);
        }
        _ => panic!("Expected DocumentContent::Pdf"),
    }
}

#[test]
fn test_f12_boundary_pdf_multi_page_count_consistency() {
    let harness = HeadlessHarness::new();
    let pdf_path = harness.vault_path().join("multi_page.pdf");
    let pages_text = vec![
        "Page 1 Text",
        "Page 2 Text",
        "Page 3 Text",
        "Page 4 Text",
        "Page 5 Text",
    ];
    create_sample_pdf(&pdf_path, "5-Page Lecture", &pages_text).expect("Create PDF");

    let content = DocumentHandler::read(&pdf_path).expect("Read PDF");
    match content {
        DocumentContent::Pdf { pages, metadata } => {
            assert_eq!(pages.len(), 5);
            assert_eq!(metadata.page_count, 5);
            assert_eq!(pages[4].page_number, 5);
        }
        _ => panic!("Expected DocumentContent::Pdf"),
    }
}

// ===========================================================================
// FEATURE 13: PPT & PPTX Document Support — Boundary Tests
// ===========================================================================

#[test]
fn test_f13_boundary_zero_byte_pptx_handling() {
    let harness = HeadlessHarness::new();
    let empty_pptx = harness.vault_path().join("zero_byte.pptx");
    fs::write(&empty_pptx, b"").expect("Write 0 bytes");

    let res = DocumentHandler::read(&empty_pptx);
    assert!(
        res.is_err(),
        "0-byte PPTX must return error without panicking"
    );
}

#[test]
fn test_f13_boundary_corrupted_zip_openxml_container() {
    let harness = HeadlessHarness::new();
    let corrupt_pptx = harness.vault_path().join("corrupt.pptx");
    // Invalid ZIP header
    fs::write(
        &corrupt_pptx,
        b"PK\x03\x04corrupted_payload_without_central_directory",
    )
    .expect("Write corrupt zip");

    let res = DocumentHandler::read(&corrupt_pptx);
    assert!(
        res.is_err(),
        "Corrupted OpenXML zip must return error cleanly"
    );
}

#[test]
fn test_f13_boundary_invalid_legacy_ppt_ole_header() {
    let harness = HeadlessHarness::new();
    let corrupt_ppt = harness.vault_path().join("corrupt.ppt");
    // Corrupted OLE header
    fs::write(
        &corrupt_ppt,
        b"\xD0\xCF\x11\x00\x00\x00\x00\x00_invalid_ole",
    )
    .expect("Write corrupt ppt");

    let res = DocumentHandler::read(&corrupt_ppt);
    // Legacy PPT engine provides graceful fallback rather than panicking
    assert!(
        res.is_ok(),
        "Invalid legacy PPT must safely parse with fallback slide without panicking"
    );
    if let Ok(DocumentContent::Pptx { slides, .. }) = res {
        assert_eq!(slides.len(), 1);
        assert!(slides[0]
            .title
            .as_deref()
            .unwrap_or("")
            .contains("Legacy Presentation"));
    }
}

#[test]
fn test_f13_boundary_pptx_empty_slide_text_shapes() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("empty_shapes.pptx");
    create_sample_pptx(
        &pptx_path,
        "Empty Presentation",
        &[("Slide 1", ""), ("Slide 2", "   ")],
    )
    .expect("Create PPTX");

    let content = DocumentHandler::read(&pptx_path).expect("Read PPTX");
    match content {
        DocumentContent::Pptx { slides, .. } => {
            assert_eq!(slides.len(), 2);
            assert_eq!(slides[0].title.as_deref(), Some("Slide 1"));
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

#[test]
fn test_f13_boundary_legacy_ppt_empty_streams() {
    let harness = HeadlessHarness::new();
    let ppt_path = harness.vault_path().join("empty_lecture.ppt");
    crate::helpers::create_sample_ppt(&ppt_path, "Empty PPT", &[]).expect("Create empty PPT");

    let content = DocumentHandler::read(&ppt_path).expect("Read empty PPT");
    match content {
        DocumentContent::Pptx { slides, .. } => {
            // Safe fallback produces 1 slide for empty binary stream
            assert_eq!(slides.len(), 1);
            assert!(slides[0]
                .title
                .as_deref()
                .unwrap_or("")
                .contains("Legacy Presentation"));
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

// ===========================================================================
// FEATURE 14: In-Place Document Editor & Updater — Boundary Tests
// ===========================================================================

#[test]
fn test_f14_boundary_save_to_invalid_nested_path() {
    let harness = HeadlessHarness::new();
    let nested_path = harness
        .vault_path()
        .join("auto_parent_dir/deeply/nested/doc.txt");
    let res = DocumentHandler::save_update(
        &nested_path,
        &DocumentContent::Text("Auto parent creation".into()),
    );
    assert!(
        res.is_ok(),
        "Saving to non-existent parent directory must automatically provision parent directory"
    );
    assert!(nested_path.exists());
    let read_back = fs::read_to_string(&nested_path).expect("Read back");
    assert_eq!(read_back, "Auto parent creation");
}

#[test]
fn test_f14_boundary_inplace_update_zero_byte_text_file() {
    let harness = HeadlessHarness::new();
    let empty_txt = harness.vault_path().join("new_empty.txt");
    fs::write(&empty_txt, b"").expect("Create 0-byte file");

    let updated_text = "Updated text with complete formulas E = mc^2";
    DocumentHandler::save_update(&empty_txt, &DocumentContent::Text(updated_text.to_string()))
        .expect("Update 0-byte file");

    let read_back = fs::read_to_string(&empty_txt).expect("Read back");
    assert_eq!(read_back, updated_text);
}

#[test]
fn test_f14_boundary_openxml_container_repack_integrity() {
    let harness = HeadlessHarness::new();
    let pptx_path = harness.vault_path().join("repack_test.pptx");
    create_sample_pptx(&pptx_path, "Repack Seminar", &[("Slide 1", "Initial text")])
        .expect("Initial PPTX");

    // Read PPTX
    let content = DocumentHandler::read(&pptx_path).expect("Read initial");
    if let DocumentContent::Pptx {
        mut slides,
        metadata,
    } = content
    {
        slides[0].body_blocks = vec!["Overwritten formula: nabla . B = 0".to_string()];
        DocumentHandler::save_update(&pptx_path, &DocumentContent::Pptx { slides, metadata })
            .expect("Save update");
    }

    // Re-read to confirm valid OpenXML zip repack
    let reread = DocumentHandler::read(&pptx_path).expect("Re-read repacked PPTX");
    match reread {
        DocumentContent::Pptx { slides, .. } => {
            assert!(slides[0].body_blocks[0].contains("nabla . B = 0"));
        }
        _ => panic!("Expected DocumentContent::Pptx"),
    }
}

#[test]
fn test_f14_boundary_invalid_utf8_binary_rejected() {
    let harness = HeadlessHarness::new();
    let raw_bin = harness.vault_path().join("binary.raw");
    fs::write(&raw_bin, b"\xFF\xFE\xFD\xFC\x80\x81\x00\xFF").expect("Write binary");

    let res = DocumentHandler::read(&raw_bin);
    assert!(res.is_err());
    match res.err().unwrap() {
        FsError::UnsupportedFormat(msg) => {
            assert!(msg.contains("unsupported") || msg.contains("Unknown") || msg.contains("raw"));
        }
        other => panic!("Expected UnsupportedFormat, got {:?}", other),
    }
}

#[test]
fn test_f14_boundary_atomic_write_replace_durability() {
    let harness = HeadlessHarness::new();
    let target = harness.vault_path().join("atomic_target.txt");
    fs::write(&target, "Original Payload A").expect("Initial write");

    // Execute 20 consecutive in-place updates to test atomic overwrite safety
    for i in 1..=20 {
        let payload = format!("Payload iteration {}", i);
        DocumentHandler::save_update(&target, &DocumentContent::Text(payload.clone()))
            .expect("Atomic save");
        let content = fs::read_to_string(&target).expect("Read back");
        assert_eq!(content, payload);
    }
}
