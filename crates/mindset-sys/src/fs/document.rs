//! MindSet OS — Pure-Rust High-Performance Document Engine
//!
//! Provides comprehensive reading, parsing, text extraction, editing,
//! and writing/updating for:
//! - PDF Documents (multi-page text layers, metadata, standard PDF object generation)
//! - Modern PPTX Presentations (OpenXML ZIP container, slide XML parsing & in-place text editing)
//! - Legacy PPT Presentations (safe binary stream text extraction fallback)
//! - Plain Text & Markdown Documents
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Cursor, Read, Write};
use std::path::{Path, PathBuf};

use chrono::Utc;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

// ---------------------------------------------------------------------------
// 1. Error Types
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum FsError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Unsupported document format: {0}")]
    UnsupportedFormat(String),

    #[error("Document parse error: {0}")]
    ParseError(String),

    #[error("Archive error: {0}")]
    ArchiveError(String),

    #[error("Corrupted document: {0}")]
    CorruptedDocument(String),

    #[error("Document write error: {0}")]
    WriteError(String),

    #[error("File not found: {0}")]
    NotFound(String),
}

// ---------------------------------------------------------------------------
// 2. Domain Data Structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentFormat {
    Pdf,
    Pptx,
    Ppt,
    Text,
    Markdown,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub page_count: usize,
    pub creation_date: Option<String>,
    pub producer: Option<String>,
    pub subject: Option<String>,
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            page_count: 0,
            creation_date: Some(Utc::now().to_rfc3339()),
            producer: Some("MindSet OS Document Engine".to_string()),
            subject: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfPage {
    pub page_number: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PptxMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub slide_count: usize,
    pub slide_width: Option<u32>,
    pub slide_height: Option<u32>,
}

impl Default for PptxMetadata {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            slide_count: 0,
            slide_width: Some(12192000), // Default 16:9 EMU (1280x720 pt)
            slide_height: Some(6858000),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PptxSlide {
    pub slide_number: usize,
    pub title: Option<String>,
    pub body_blocks: Vec<String>,
    pub notes: Option<String>,
    pub shape_texts: Vec<String>,
}

impl PptxSlide {
    /// Combines all text components of the slide into a single formatted preview.
    pub fn combined_text(&self) -> String {
        let mut out = String::new();
        if let Some(t) = &self.title {
            out.push_str(&format!("# {}\n\n", t));
        }
        for b in &self.body_blocks {
            out.push_str(b);
            out.push_str("\n\n");
        }
        for s in &self.shape_texts {
            out.push_str(s);
            out.push('\n');
        }
        if let Some(n) = &self.notes {
            out.push_str(&format!("\n[Notes: {}]", n));
        }
        out.trim().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentContent {
    Text(String),
    Pdf {
        pages: Vec<PdfPage>,
        metadata: PdfMetadata,
    },
    Pptx {
        slides: Vec<PptxSlide>,
        metadata: PptxMetadata,
    },
}

impl DocumentContent {
    /// Returns total pages or slides count for paginated documents.
    pub fn count(&self) -> usize {
        match self {
            DocumentContent::Text(_) => 1,
            DocumentContent::Pdf { pages, .. } => pages.len(),
            DocumentContent::Pptx { slides, .. } => slides.len(),
        }
    }

    /// Returns a full text extract across all pages or slides.
    pub fn extract_full_text(&self) -> String {
        match self {
            DocumentContent::Text(t) => t.clone(),
            DocumentContent::Pdf { pages, .. } => pages
                .iter()
                .map(|p| format!("--- Page {} ---\n{}", p.page_number, p.text))
                .collect::<Vec<_>>()
                .join("\n\n"),
            DocumentContent::Pptx { slides, .. } => slides
                .iter()
                .map(|s| format!("--- Slide {} ---\n{}", s.slide_number, s.combined_text()))
                .collect::<Vec<_>>()
                .join("\n\n"),
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Format Detection
// ---------------------------------------------------------------------------

pub fn detect_format(path: &Path, bytes: &[u8]) -> DocumentFormat {
    // 1. Check magic bytes
    if bytes.starts_with(b"%PDF") {
        return DocumentFormat::Pdf;
    }
    if bytes.starts_with(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
        return DocumentFormat::Ppt;
    }
    if bytes.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
        // Zip archive: inspect if PPTX (strictly require ppt/ directory prefix)
        if let Ok(mut archive) = ZipArchive::new(Cursor::new(bytes)) {
            let mut is_pptx = false;
            for i in 0..archive.len() {
                if let Ok(file) = archive.by_index(i) {
                    let name = file.name();
                    if name.starts_with("ppt/") {
                        is_pptx = true;
                        break;
                    }
                }
            }
            if is_pptx {
                return DocumentFormat::Pptx;
            }
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext == "pptx" {
            return DocumentFormat::Pptx;
        }
    }

    // 2. Extension Fallback
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "pdf" => DocumentFormat::Pdf,
        "pptx" => DocumentFormat::Pptx,
        "ppt" => DocumentFormat::Ppt,
        "md" => DocumentFormat::Markdown,
        "txt" | "capsule" | "notes" | "rs" | "json" | "toml" | "csv" | "log" => {
            DocumentFormat::Text
        }
        _ => {
            if std::str::from_utf8(bytes).is_ok() {
                DocumentFormat::Text
            } else {
                DocumentFormat::Unknown
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Pure-Rust PDF Engine
// ---------------------------------------------------------------------------

pub struct PdfEngine;

impl PdfEngine {
    /// Parses a PDF byte buffer, extracting metadata, pages, and text.
    pub fn parse(bytes: &[u8]) -> Result<DocumentContent, FsError> {
        if !bytes.starts_with(b"%PDF") {
            return Err(FsError::ParseError(
                "Missing %PDF magic header in document stream".to_string(),
            ));
        }

        if find_subsequence(bytes, b"obj").is_none() || find_subsequence(bytes, b"endobj").is_none()
        {
            return Err(FsError::CorruptedDocument(
                "PDF stream contains no valid PDF objects".to_string(),
            ));
        }

        let raw_str = String::from_utf8_lossy(bytes);

        // 1. Extract Info metadata
        let metadata = Self::extract_metadata(&raw_str, bytes);

        // 2. Extract page contents (rejects corrupted streams / empty garbage without leaking raw syntax)
        let pages = Self::extract_pages(&raw_str, bytes)?;

        if pages.is_empty() {
            return Err(FsError::CorruptedDocument(
                "PDF contains no valid pages or extractable content streams".to_string(),
            ));
        }

        let page_count = pages.len();
        let mut meta = metadata;
        meta.page_count = page_count;

        Ok(DocumentContent::Pdf {
            pages,
            metadata: meta,
        })
    }

    fn extract_metadata(raw_str: &str, _bytes: &[u8]) -> PdfMetadata {
        let mut meta = PdfMetadata {
            title: None,
            author: None,
            page_count: 0,
            creation_date: None,
            producer: None,
            subject: None,
        };

        // Scan for /Title (...)
        meta.title = Self::find_dict_string(raw_str, "/Title");
        meta.author = Self::find_dict_string(raw_str, "/Author");
        meta.creation_date = Self::find_dict_string(raw_str, "/CreationDate");
        meta.producer = Self::find_dict_string(raw_str, "/Producer");
        meta.subject = Self::find_dict_string(raw_str, "/Subject");

        meta
    }

    fn find_dict_string(haystack: &str, key: &str) -> Option<String> {
        if let Some(pos) = haystack.find(key) {
            let slice = &haystack[pos + key.len()..];
            let trimmed = slice.trim_start();
            if trimmed.starts_with('(') {
                // Paren string: (Title Here)
                let mut depth = 0;
                let mut out = String::new();
                let mut chars = trimmed.chars();
                while let Some(c) = chars.next() {
                    match c {
                        '(' => {
                            if depth > 0 {
                                out.push('(');
                            }
                            depth += 1;
                        }
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            } else {
                                out.push(')');
                            }
                        }
                        '\\' => {
                            if let Some(next) = chars.next() {
                                match next {
                                    'n' => out.push('\n'),
                                    'r' => out.push('\r'),
                                    't' => out.push('\t'),
                                    '\\' => out.push('\\'),
                                    '(' => out.push('('),
                                    ')' => out.push(')'),
                                    other => out.push(other),
                                }
                            }
                        }
                        other => {
                            if depth > 0 {
                                out.push(other);
                            }
                        }
                    }
                }
                if !out.trim().is_empty() {
                    return Some(out.trim().to_string());
                }
            } else if trimmed.starts_with('<') {
                // Hex string: <FEFF...>
                if let Some(end_hex) = trimmed.find('>') {
                    let hex_content = &trimmed[1..end_hex];
                    if let Some(decoded_bytes) = decode_hex_str(hex_content) {
                        // Check for UTF-16BE BOM
                        if decoded_bytes.starts_with(&[0xFE, 0xFF]) {
                            let u16s: Vec<u16> = decoded_bytes[2..]
                                .chunks_exact(2)
                                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                                .collect();
                            if let Ok(s) = String::from_utf16(&u16s) {
                                return Some(s);
                            }
                        }
                        if let Ok(s) = String::from_utf8(decoded_bytes) {
                            return Some(s);
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_pages(_raw_str: &str, bytes: &[u8]) -> Result<Vec<PdfPage>, FsError> {
        let mut pages = Vec::new();

        // 1. Locate all stream ... endstream blocks and decompress if FlateDecode
        let streams = Self::extract_streams(bytes)?;

        // 2. Extract text from each stream
        for stream_data in streams {
            let text = Self::extract_text_from_content_stream(&stream_data);
            if !text.trim().is_empty() {
                pages.push(PdfPage {
                    page_number: pages.len() + 1,
                    text: text.trim().to_string(),
                });
            } else if Self::is_valid_blank_content_stream(&stream_data) {
                pages.push(PdfPage {
                    page_number: pages.len() + 1,
                    text: String::new(),
                });
            }
        }

        Ok(pages)
    }

    fn is_valid_blank_content_stream(stream_bytes: &[u8]) -> bool {
        let s = String::from_utf8_lossy(stream_bytes);
        s.contains("BT") && s.contains("ET")
    }

    fn extract_streams(bytes: &[u8]) -> Result<Vec<Vec<u8>>, FsError> {
        let mut streams = Vec::new();
        let stream_marker = b"stream";
        let endstream_marker = b"endstream";

        let mut cursor = 0;
        while cursor < bytes.len() {
            if let Some(stream_idx) = find_subsequence(&bytes[cursor..], stream_marker) {
                let stream_start = cursor + stream_idx + stream_marker.len();
                // Skip CRLF or LF after stream keyword
                let mut data_start = stream_start;
                if data_start < bytes.len() && bytes[data_start] == b'\r' {
                    data_start += 1;
                }
                if data_start < bytes.len() && bytes[data_start] == b'\n' {
                    data_start += 1;
                }

                if let Some(end_idx) = find_subsequence(&bytes[data_start..], endstream_marker) {
                    // Check preceding dictionary to see if /Length or FlateDecode is specified
                    let dict_window_start = cursor.max(stream_start.saturating_sub(512));
                    let dict_window =
                        String::from_utf8_lossy(&bytes[dict_window_start..stream_start]);
                    let is_flate = dict_window.contains("FlateDecode");

                    // Check if stream dictionary specifies a direct /Length <N>
                    let mut exact_len = None;
                    if let Some(pos) = dict_window.find("/Length") {
                        let after = &dict_window[pos + 7..];
                        let trimmed = after.trim_start();
                        let digit_count =
                            trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
                        if digit_count > 0 {
                            if let Ok(val) = trimmed[..digit_count].parse::<usize>() {
                                let remainder = trimmed[digit_count..].trim_start();
                                // Check if this is an indirect reference like "12 0 R"
                                let is_indirect = {
                                    let mut words = remainder.split_whitespace();
                                    let w1 = words.next();
                                    let w2 = words.next();
                                    w1.is_some_and(|s| s.chars().all(|c| c.is_ascii_digit()))
                                        && w2.is_some_and(|s| s == "R")
                                };
                                if !is_indirect {
                                    exact_len = Some(val);
                                }
                            }
                        }
                    }

                    // Determine stream payload slice without stripping binary payload bytes
                    let raw_slice = if let Some(len) = exact_len {
                        if data_start + len <= data_start + end_idx
                            && data_start + len <= bytes.len()
                        {
                            &bytes[data_start..data_start + len]
                        } else {
                            // Fallback before endstream: strip at most the single line ending immediately before endstream
                            let mut data_end = data_start + end_idx;
                            if data_end >= data_start + 2
                                && &bytes[data_end - 2..data_end] == b"\r\n"
                            {
                                data_end -= 2;
                            } else if data_end >= data_start + 1
                                && (bytes[data_end - 1] == b'\n' || bytes[data_end - 1] == b'\r')
                            {
                                data_end -= 1;
                            }
                            &bytes[data_start..data_end]
                        }
                    } else {
                        // Fallback before endstream: strip at most the single line ending immediately before endstream
                        let mut data_end = data_start + end_idx;
                        if data_end >= data_start + 2 && &bytes[data_end - 2..data_end] == b"\r\n" {
                            data_end -= 2;
                        } else if data_end >= data_start + 1
                            && (bytes[data_end - 1] == b'\n' || bytes[data_end - 1] == b'\r')
                        {
                            data_end -= 1;
                        }
                        &bytes[data_start..data_end]
                    };

                    let decompressed = if is_flate {
                        let mut decoder = ZlibDecoder::new(raw_slice);
                        let mut buf = Vec::new();
                        if decoder.read_to_end(&mut buf).is_ok() {
                            buf
                        } else {
                            // If exact_len slice failed to decompress, try fallback boundary
                            let mut fallback_end = data_start + end_idx;
                            if fallback_end >= data_start + 2
                                && &bytes[fallback_end - 2..fallback_end] == b"\r\n"
                            {
                                fallback_end -= 2;
                            } else if fallback_end >= data_start + 1
                                && (bytes[fallback_end - 1] == b'\n'
                                    || bytes[fallback_end - 1] == b'\r')
                            {
                                fallback_end -= 1;
                            }
                            let fallback_slice = &bytes[data_start..fallback_end];
                            let mut fallback_decoder = ZlibDecoder::new(fallback_slice);
                            let mut fallback_buf = Vec::new();
                            if fallback_decoder.read_to_end(&mut fallback_buf).is_ok() {
                                fallback_buf
                            } else {
                                return Err(FsError::CorruptedDocument(
                                    "Corrupted FlateDecode stream in PDF".to_string(),
                                ));
                            }
                        }
                    } else {
                        raw_slice.to_vec()
                    };

                    streams.push(decompressed);
                    cursor = data_start + end_idx + endstream_marker.len();
                } else {
                    return Err(FsError::CorruptedDocument(
                        "Unterminated PDF stream missing endstream".to_string(),
                    ));
                }
            } else {
                break;
            }
        }

        Ok(streams)
    }

    fn decode_pdf_string_bytes(raw_bytes: &[u8]) -> String {
        if raw_bytes.starts_with(&[0xFE, 0xFF]) {
            let u16s: Vec<u16> = raw_bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            return String::from_utf16_lossy(&u16s);
        }
        if let Ok(s) = std::str::from_utf8(raw_bytes) {
            return s.to_string();
        }
        // Fallback: Windows-1252 / WinAnsi for 0x80..=0x9F, Latin-1 for remainder
        raw_bytes
            .iter()
            .map(|&b| match b {
                0x80 => '\u{20AC}',
                0x82 => '\u{201A}',
                0x83 => '\u{0192}',
                0x84 => '\u{201E}',
                0x85 => '\u{2026}',
                0x86 => '\u{2020}',
                0x87 => '\u{2021}',
                0x88 => '\u{02C6}',
                0x89 => '\u{2030}',
                0x8A => '\u{0160}',
                0x8B => '\u{2039}',
                0x8C => '\u{0152}',
                0x8E => '\u{017D}',
                0x91 => '\u{2018}',
                0x92 => '\u{2019}',
                0x93 => '\u{201C}',
                0x94 => '\u{201D}',
                0x95 => '\u{2022}',
                0x96 => '\u{2013}',
                0x97 => '\u{2014}',
                0x98 => '\u{02DC}',
                0x99 => '\u{2122}',
                0x9A => '\u{0161}',
                0x9B => '\u{203A}',
                0x9C => '\u{0153}',
                0x9E => '\u{017E}',
                0x9F => '\u{0178}',
                other => other as char,
            })
            .collect()
    }

    fn extract_text_from_content_stream(stream_bytes: &[u8]) -> String {
        let mut out = String::new();
        let mut i = 0;
        let len = stream_bytes.len();

        while i < len {
            let b = stream_bytes[i];
            if b == b'(' {
                // Literal string ( ... ) accumulated as raw bytes to preserve UTF-8 multi-byte sequences
                i += 1;
                let mut raw_bytes: Vec<u8> = Vec::new();
                let mut depth = 1;
                while i < len && depth > 0 {
                    match stream_bytes[i] {
                        b'(' => {
                            depth += 1;
                            raw_bytes.push(b'(');
                        }
                        b')' => {
                            depth -= 1;
                            if depth > 0 {
                                raw_bytes.push(b')');
                            }
                        }
                        b'\\' => {
                            i += 1;
                            if i < len {
                                match stream_bytes[i] {
                                    b'n' => raw_bytes.push(b'\n'),
                                    b'r' => raw_bytes.push(b'\r'),
                                    b't' => raw_bytes.push(b'\t'),
                                    b'b' => raw_bytes.push(0x08),
                                    b'f' => raw_bytes.push(0x0C),
                                    b'\\' => raw_bytes.push(b'\\'),
                                    b'(' => raw_bytes.push(b'('),
                                    b')' => raw_bytes.push(b')'),
                                    b'\r' => {
                                        if i + 1 < len && stream_bytes[i + 1] == b'\n' {
                                            i += 1;
                                        }
                                    }
                                    b'\n' => {}
                                    d @ b'0'..=b'7' => {
                                        let mut oct = (d - b'0') as u16;
                                        for _ in 0..2 {
                                            if i + 1 < len
                                                && (b'0'..=b'7').contains(&stream_bytes[i + 1])
                                            {
                                                i += 1;
                                                oct = oct * 8 + (stream_bytes[i] - b'0') as u16;
                                            } else {
                                                break;
                                            }
                                        }
                                        raw_bytes.push((oct & 0xFF) as u8);
                                    }
                                    other => raw_bytes.push(other),
                                }
                            }
                        }
                        ch => {
                            raw_bytes.push(ch);
                        }
                    }
                    i += 1;
                }

                let str_val = Self::decode_pdf_string_bytes(&raw_bytes);

                // Check following operator: Tj, ', ", or TJ element
                let mut lookahead = i;
                while lookahead < len
                    && (stream_bytes[lookahead] == b' '
                        || stream_bytes[lookahead] == b'\r'
                        || stream_bytes[lookahead] == b'\n')
                {
                    lookahead += 1;
                }

                if lookahead < len {
                    if stream_bytes[lookahead..].starts_with(b"Tj") {
                        out.push_str(&str_val);
                        out.push(' ');
                        i = lookahead + 2;
                    } else if stream_bytes[lookahead..].starts_with(b"'")
                        || stream_bytes[lookahead..].starts_with(b"\"")
                    {
                        out.push('\n');
                        out.push_str(&str_val);
                        i = lookahead + 1;
                    } else {
                        out.push_str(&str_val);
                    }
                } else {
                    out.push_str(&str_val);
                }
            } else if b == b'<' && i + 1 < len && stream_bytes[i + 1] != b'<' {
                // Hex string <...>
                i += 1;
                let hex_start = i;
                while i < len && stream_bytes[i] != b'>' {
                    i += 1;
                }
                let hex_slice = String::from_utf8_lossy(&stream_bytes[hex_start..i]);
                if i < len && stream_bytes[i] == b'>' {
                    i += 1;
                }
                if let Some(decoded_bytes) = decode_hex_str(&hex_slice) {
                    let str_val = Self::decode_pdf_string_bytes(&decoded_bytes);
                    let mut lookahead = i;
                    while lookahead < len
                        && (stream_bytes[lookahead] == b' '
                            || stream_bytes[lookahead] == b'\r'
                            || stream_bytes[lookahead] == b'\n')
                    {
                        lookahead += 1;
                    }
                    if lookahead < len && stream_bytes[lookahead..].starts_with(b"Tj") {
                        out.push_str(&str_val);
                        out.push(' ');
                        i = lookahead + 2;
                    } else if lookahead < len
                        && (stream_bytes[lookahead..].starts_with(b"'")
                            || stream_bytes[lookahead..].starts_with(b"\""))
                    {
                        out.push('\n');
                        out.push_str(&str_val);
                        i = lookahead + 1;
                    } else {
                        out.push_str(&str_val);
                    }
                }
            } else if stream_bytes[i..].starts_with(b"T*") {
                out.push('\n');
                i += 2;
            } else if stream_bytes[i..].starts_with(b"Td") || stream_bytes[i..].starts_with(b"TD") {
                out.push(' ');
                i += 2;
            } else {
                i += 1;
            }
        }

        // Clean up text spacing
        let cleaned: Vec<&str> = out
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        cleaned.join("\n")
    }

    /// Generates a fully valid, standard-compliant PDF document byte stream
    /// containing the given pages and metadata.
    pub fn generate_pdf(pages: &[PdfPage], metadata: &PdfMetadata) -> Result<Vec<u8>, FsError> {
        let mut buf = Vec::new();

        // 1. Header
        buf.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

        let blank_fallback = [PdfPage {
            page_number: 1,
            text: String::new(),
        }];
        let effective_pages: &[PdfPage] = if pages.is_empty() {
            &blank_fallback
        } else {
            pages
        };

        let mut offsets = Vec::new();
        let num_pages = effective_pages.len();

        // Object ID allocation:
        // 1: Catalog
        // 2: Pages Tree
        // 3: Font (Helvetica)
        // For page i (0-based):
        //   Page object id: 4 + 2*i
        //   Content stream id: 5 + 2*i
        // Info object id: 4 + 2*num_pages

        // Object 1: Catalog
        offsets.push(buf.len());
        writeln!(buf, "1 0 obj")?;
        writeln!(buf, "<< /Type /Catalog /Pages 2 0 R >>")?;
        writeln!(buf, "endobj")?;

        // Object 2: Pages Tree
        offsets.push(buf.len());
        writeln!(buf, "2 0 obj")?;
        write!(buf, "<< /Type /Pages /Kids [")?;
        for i in 0..num_pages {
            write!(buf, "{} 0 R ", 4 + 2 * i)?;
        }
        writeln!(buf, "] /Count {} >>", num_pages)?;
        writeln!(buf, "endobj")?;

        // Object 3: Standard Base14 Font (Helvetica)
        offsets.push(buf.len());
        writeln!(buf, "3 0 obj")?;
        writeln!(
            buf,
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
        )?;
        writeln!(buf, "endobj")?;

        // Write Pages and Content Streams
        for (i, page) in effective_pages.iter().enumerate() {
            let page_obj_id = 4 + 2 * i;
            let stream_obj_id = 5 + 2 * i;

            // Page Object
            offsets.push(buf.len());
            writeln!(buf, "{} 0 obj", page_obj_id)?;
            writeln!(
                buf,
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",
                stream_obj_id
            )?;
            writeln!(buf, "endobj")?;

            // Build content stream: BT /F1 12 Tf 72 720 Td (text) ' ET
            let mut stream_content = Vec::new();
            writeln!(stream_content, "BT")?;
            writeln!(stream_content, "/F1 12 Tf")?;
            writeln!(stream_content, "72 720 Td")?;
            writeln!(stream_content, "14.4 TL")?;

            for line in page.text.lines() {
                let escaped = line
                    .replace('\\', "\\\\")
                    .replace('(', "\\(")
                    .replace(')', "\\)");
                writeln!(stream_content, "({}) '", escaped)?;
            }
            writeln!(stream_content, "ET")?;

            // Compress stream with flate2 for space efficiency and spec compliance
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&stream_content)?;
            let compressed = encoder.finish()?;

            // Content Stream Object
            offsets.push(buf.len());
            writeln!(buf, "{} 0 obj", stream_obj_id)?;
            writeln!(
                buf,
                "<< /Length {} /Filter /FlateDecode >>",
                compressed.len()
            )?;
            writeln!(buf, "stream")?;
            buf.extend_from_slice(&compressed);
            writeln!(buf, "\nendstream")?;
            writeln!(buf, "endobj")?;
        }

        // Info Object
        let info_obj_id = 4 + 2 * num_pages;
        offsets.push(buf.len());
        writeln!(buf, "{} 0 obj", info_obj_id)?;
        write!(buf, "<<")?;
        if let Some(title) = &metadata.title {
            write!(buf, " /Title ({})", title.replace('(', "").replace(')', ""))?;
        }
        if let Some(author) = &metadata.author {
            write!(
                buf,
                " /Author ({})",
                author.replace('(', "").replace(')', "")
            )?;
        }
        if let Some(producer) = &metadata.producer {
            write!(
                buf,
                " /Producer ({})",
                producer.replace('(', "").replace(')', "")
            )?;
        }
        if let Some(date) = &metadata.creation_date {
            write!(
                buf,
                " /CreationDate (D:{})",
                date.replace('-', "").replace(':', "").replace('Z', "")
            )?;
        }
        writeln!(buf, " >>")?;
        writeln!(buf, "endobj")?;

        // Cross-Reference Table
        let xref_offset = buf.len();
        let total_objs = info_obj_id + 1;
        writeln!(buf, "xref")?;
        writeln!(buf, "0 {}", total_objs)?;
        writeln!(buf, "0000000000 65535 f ")?;
        for off in &offsets {
            writeln!(buf, "{:010} 00000 n ", off)?;
        }

        // Trailer
        writeln!(buf, "trailer")?;
        writeln!(
            buf,
            "<< /Size {} /Root 1 0 R /Info {} 0 R >>",
            total_objs, info_obj_id
        )?;
        writeln!(buf, "startxref")?;
        writeln!(buf, "{}", xref_offset)?;
        writeln!(buf, "%%EOF")?;

        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// 5. Pure-Rust PPTX OpenXML Engine
// ---------------------------------------------------------------------------

pub struct PptxEngine;

impl PptxEngine {
    /// Reads and unpacks a `.pptx` container, parsing OpenXML slides and metadata.
    pub fn parse(bytes: &[u8]) -> Result<DocumentContent, FsError> {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| FsError::ArchiveError(format!("Invalid PPTX ZIP archive: {}", e)))?;

        // 1. Read metadata from docProps/core.xml if present
        let mut meta = PptxMetadata::default();
        if let Ok(mut core_file) = archive.by_name("docProps/core.xml") {
            let mut xml = String::new();
            if core_file.read_to_string(&mut xml).is_ok() {
                Self::extract_core_metadata(&xml, &mut meta);
            }
        }

        // 2. Read slide dimensions from ppt/presentation.xml if present
        if let Ok(mut pres_file) = archive.by_name("ppt/presentation.xml") {
            let mut xml = String::new();
            if pres_file.read_to_string(&mut xml).is_ok() {
                Self::extract_presentation_metadata(&xml, &mut meta);
            }
        }

        // 3. Discover and sort all slide files: ppt/slides/slide{N}.xml
        let mut slide_names = Vec::new();
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let name = file.name().to_string();
                if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                    slide_names.push(name);
                }
            }
        }

        // Sort slide files by natural numeric order (slide1, slide2, slide10, etc.)
        slide_names.sort_by_key(|name| {
            let num_part: String = name
                .trim_start_matches("ppt/slides/slide")
                .trim_end_matches(".xml")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            num_part.parse::<usize>().unwrap_or(0)
        });

        // 4. Parse each slide XML
        let mut slides = Vec::new();
        for (idx, slide_name) in slide_names.iter().enumerate() {
            let mut slide_xml = String::new();
            if let Ok(mut sf) = archive.by_name(slide_name) {
                sf.read_to_string(&mut slide_xml).map_err(|e| {
                    FsError::ParseError(format!("Failed reading {}: {}", slide_name, e))
                })?;
            }

            let slide_num = slide_name
                .trim_start_matches("ppt/slides/slide")
                .trim_end_matches(".xml")
                .parse::<usize>()
                .unwrap_or(idx + 1);

            // Check for corresponding notes slide (by slide_num, falling back to idx + 1)
            let note_candidates = [
                format!("ppt/notesSlides/notesSlide{}.xml", slide_num),
                format!("ppt/notesSlides/notesSlide{}.xml", idx + 1),
            ];
            let mut note_text = None;
            for note_name in &note_candidates {
                if let Ok(mut nf) = archive.by_name(note_name) {
                    let mut note_xml = String::new();
                    if nf.read_to_string(&mut note_xml).is_ok() {
                        let extracted = Self::extract_plain_xml_text(&note_xml);
                        if !extracted.trim().is_empty() {
                            note_text = Some(extracted.trim().to_string());
                            break;
                        }
                    }
                }
            }

            let slide = Self::try_parse_slide_xml(slide_num, &slide_xml, note_text)?;
            slides.push(slide);
        }

        meta.slide_count = slides.len();

        Ok(DocumentContent::Pptx {
            slides,
            metadata: meta,
        })
    }

    fn extract_core_metadata(xml: &str, meta: &mut PptxMetadata) {
        let mut reader = Reader::from_str(xml);
        let mut current_tag = String::new();
        let mut current_text = String::new();

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Start(e) => {
                    let name = e.name();
                    current_tag = String::from_utf8_lossy(name.as_ref()).to_string();
                    if current_tag.ends_with(":title")
                        || current_tag == "title"
                        || current_tag.ends_with(":creator")
                        || current_tag == "creator"
                    {
                        current_text.clear();
                    }
                }
                Event::Text(e) => {
                    if current_tag.ends_with(":title")
                        || current_tag == "title"
                        || current_tag.ends_with(":creator")
                        || current_tag == "creator"
                    {
                        let raw = std::str::from_utf8(&e).unwrap_or("");
                        let val = quick_xml::escape::unescape(raw).unwrap_or_default();
                        current_text.push_str(&val);
                    }
                }
                Event::GeneralRef(ref_bytes) => {
                    if current_tag.ends_with(":title")
                        || current_tag == "title"
                        || current_tag.ends_with(":creator")
                        || current_tag == "creator"
                    {
                        let raw = std::str::from_utf8(ref_bytes.as_ref()).unwrap_or("");
                        current_text.push_str(&resolve_xml_entity(raw));
                    }
                }
                Event::End(e) => {
                    let name = e.name();
                    let tag = String::from_utf8_lossy(name.as_ref()).to_string();
                    if tag.ends_with(":title") || tag == "title" {
                        let t = current_text.trim();
                        if !t.is_empty() {
                            meta.title = Some(t.to_string());
                        }
                    } else if tag.ends_with(":creator") || tag == "creator" {
                        let a = current_text.trim();
                        if !a.is_empty() {
                            meta.author = Some(a.to_string());
                        }
                    }
                    if tag == current_tag {
                        current_tag.clear();
                        current_text.clear();
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
    }

    fn extract_presentation_metadata(xml: &str, meta: &mut PptxMetadata) {
        let mut reader = Reader::from_str(xml);
        while let Ok(event) = reader.read_event() {
            match event {
                Event::Start(e) | Event::Empty(e) => {
                    let qname = e.name();
                    let name = String::from_utf8_lossy(qname.as_ref()).to_string();
                    if name.ends_with(":sldSz") || name == "sldSz" {
                        for attr in e.attributes().flatten() {
                            let k = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let v = String::from_utf8_lossy(&attr.value).to_string();
                            if k == "cx" {
                                meta.slide_width = v.parse().ok();
                            } else if k == "cy" {
                                meta.slide_height = v.parse().ok();
                            }
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
    }

    #[cfg(test)]
    fn parse_slide_xml(slide_number: usize, xml: &str, notes: Option<String>) -> PptxSlide {
        Self::try_parse_slide_xml(slide_number, xml, notes).unwrap_or(PptxSlide {
            slide_number,
            title: None,
            body_blocks: Vec::new(),
            notes: None,
            shape_texts: Vec::new(),
        })
    }

    fn try_parse_slide_xml(
        slide_number: usize,
        xml: &str,
        notes: Option<String>,
    ) -> Result<PptxSlide, FsError> {
        let mut reader = Reader::from_str(xml);

        let mut title: Option<String> = None;
        let mut body_blocks: Vec<String> = Vec::new();
        let mut shape_texts: Vec<String> = Vec::new();

        let mut in_title_shape = false;
        let mut in_body_shape = false;
        let mut saw_placeholder = false;
        let mut in_text_run = false;
        let mut new_text_run = false;
        let mut just_had_entity = false;
        let mut current_paragraph = String::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    // Check placeholder type (ph type="title" or "ctrTitle")
                    if tag.ends_with(":ph") || tag == "ph" {
                        saw_placeholder = true;
                        for attr in e.attributes().flatten() {
                            let k = String::from_utf8_lossy(attr.key.as_ref());
                            let v = String::from_utf8_lossy(&attr.value);
                            if k == "type" {
                                if v == "title" || v == "ctrTitle" {
                                    in_title_shape = true;
                                    in_body_shape = false;
                                } else if v == "body" || v == "subTitle" {
                                    in_body_shape = true;
                                    in_title_shape = false;
                                }
                            }
                        }
                    }

                    if tag.ends_with(":t") || tag == "t" {
                        in_text_run = true;
                        new_text_run = true;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text_run {
                        let raw = std::str::from_utf8(&e).unwrap_or("");
                        let text = quick_xml::escape::unescape(raw).unwrap_or_default();
                        if !text.is_empty() {
                            if new_text_run
                                && !just_had_entity
                                && !current_paragraph.is_empty()
                                && !current_paragraph.ends_with(char::is_whitespace)
                                && !text.starts_with(char::is_whitespace)
                            {
                                current_paragraph.push(' ');
                            }
                            current_paragraph.push_str(&text);
                            new_text_run = false;
                            just_had_entity = false;
                        }
                    }
                }
                Ok(Event::GeneralRef(ref_bytes)) => {
                    if in_text_run {
                        let raw = std::str::from_utf8(ref_bytes.as_ref()).unwrap_or("");
                        current_paragraph.push_str(&resolve_xml_entity(raw));
                        just_had_entity = true;
                    }
                }
                Ok(Event::End(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if tag.ends_with(":t") || tag == "t" {
                        in_text_run = false;
                        new_text_run = false;
                    } else if tag.ends_with(":p") || tag == "p" {
                        let p = current_paragraph.trim().to_string();
                        if !p.is_empty() {
                            if in_title_shape && title.is_none() {
                                title = Some(p);
                            } else if in_body_shape {
                                body_blocks.push(p);
                            } else {
                                shape_texts.push(p);
                            }
                        }
                        current_paragraph.clear();
                        just_had_entity = false;
                        new_text_run = false;
                    } else if tag.ends_with(":sp") || tag == "sp" {
                        in_title_shape = false;
                        in_body_shape = false;
                    }
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(e) => {
                    return Err(FsError::ParseError(format!(
                        "Malformed XML in slide {}: {}",
                        slide_number, e
                    )));
                }
            }
        }

        // Only promote first free shape text into title if slide had no placeholder markers at all
        if title.is_none() && !saw_placeholder && body_blocks.is_empty() && !shape_texts.is_empty()
        {
            title = Some(shape_texts.remove(0));
        }

        Ok(PptxSlide {
            slide_number,
            title,
            body_blocks,
            notes,
            shape_texts,
        })
    }

    fn extract_plain_xml_text(xml: &str) -> String {
        let mut reader = Reader::from_str(xml);
        let mut out = String::new();
        let mut just_had_entity = false;

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Text(e) => {
                    let raw = std::str::from_utf8(&e).unwrap_or("");
                    let t = quick_xml::escape::unescape(raw).unwrap_or_default();
                    if !t.is_empty() {
                        if !just_had_entity
                            && !out.is_empty()
                            && !out.ends_with(char::is_whitespace)
                            && !t.starts_with(char::is_whitespace)
                        {
                            out.push(' ');
                        }
                        out.push_str(&t);
                        just_had_entity = false;
                    }
                }
                Event::GeneralRef(ref_bytes) => {
                    let raw = std::str::from_utf8(ref_bytes.as_ref()).unwrap_or("");
                    out.push_str(&resolve_xml_entity(raw));
                    just_had_entity = true;
                }
                Event::Eof => break,
                _ => {}
            }
        }
        out.trim().to_string()
    }

    /// Updates or generates a `.pptx` container byte vector.
    /// If an original PPTX byte stream is provided, updates slide XML files in place
    /// and repacks into a clean archive.
    /// If `original_bytes` is None, scaffolds a valid standard OpenXML PPTX container.
    pub fn save_or_update(
        slides: &[PptxSlide],
        metadata: &PptxMetadata,
        original_bytes: Option<&[u8]>,
    ) -> Result<Vec<u8>, FsError> {
        let mut out_buf = Cursor::new(Vec::new());

        if let Some(src) = original_bytes {
            // Update existing archive in-place while cleaning up deleted slides/notes and updating metadata
            let mut src_archive = ZipArchive::new(Cursor::new(src))
                .map_err(|e| FsError::ArchiveError(format!("Cannot read source PPTX: {}", e)))?;
            let mut writer = ZipWriter::new(&mut out_buf);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

            let mut updated_slides_map: HashMap<String, String> = HashMap::new();
            let mut updated_notes_map: HashMap<String, String> = HashMap::new();
            let mut updated_slide_rels_map: HashMap<String, String> = HashMap::new();

            for (idx, slide) in slides.iter().enumerate() {
                let slide_num = if slide.slide_number > 0 {
                    slide.slide_number
                } else {
                    idx + 1
                };
                let slide_path = format!("ppt/slides/slide{}.xml", slide_num);
                updated_slides_map.insert(slide_path, Self::generate_slide_xml(slide));

                if let Some(note) = &slide.notes {
                    if !note.trim().is_empty() {
                        updated_notes_map.insert(
                            format!("ppt/notesSlides/notesSlide{}.xml", slide_num),
                            Self::generate_notes_xml(note),
                        );
                        updated_slide_rels_map.insert(
                            format!("ppt/slides/_rels/slide{}.xml.rels", slide_num),
                            Self::generate_slide_rels_xml(slide_num),
                        );
                    }
                }
            }

            let mut existing_pres_rels_extra = String::new();
            if let Ok(mut f) = src_archive.by_name("ppt/_rels/presentation.xml.rels") {
                let mut xml = String::new();
                if f.read_to_string(&mut xml).is_ok() {
                    for line in xml.lines() {
                        let t = line.trim();
                        if t.starts_with("<Relationship ")
                            && !t.contains("relationships/slide\"")
                            && !t.contains("Target=\"slides/slide")
                        {
                            existing_pres_rels_extra.push_str("  ");
                            existing_pres_rels_extra.push_str(t);
                            existing_pres_rels_extra.push('\n');
                        }
                    }
                }
            }

            let mut existing_content_types_extra = String::new();
            if let Ok(mut f) = src_archive.by_name("[Content_Types].xml") {
                let mut xml = String::new();
                if f.read_to_string(&mut xml).is_ok() {
                    for line in xml.lines() {
                        let t = line.trim();
                        if t.starts_with("<Override ")
                            && !t.contains("/ppt/slides/slide")
                            && !t.contains("/ppt/notesSlides/notesSlide")
                            && !t.contains("/ppt/presentation.xml")
                            && !t.contains("/docProps/core.xml")
                        {
                            existing_content_types_extra.push_str("  ");
                            existing_content_types_extra.push_str(t);
                            existing_content_types_extra.push('\n');
                        }
                    }
                }
            }

            // Copy all non-slide/non-notes/non-regenerated files (e.g. themes, masters, media)
            for i in 0..src_archive.len() {
                let mut file = src_archive.by_index(i).map_err(|e| {
                    FsError::ArchiveError(format!("Archive entry read error: {}", e))
                })?;
                let name = file.name().to_string();

                let is_managed_entry = (name.starts_with("ppt/slides/slide")
                    && name.ends_with(".xml"))
                    || (name.starts_with("ppt/notesSlides/notesSlide") && name.ends_with(".xml"))
                    || (name.starts_with("ppt/slides/_rels/slide") && name.ends_with(".xml.rels"))
                    || name == "docProps/core.xml"
                    || name == "ppt/presentation.xml"
                    || name == "ppt/_rels/presentation.xml.rels"
                    || name == "[Content_Types].xml";

                if is_managed_entry {
                    continue;
                }

                let mut data = Vec::new();
                file.read_to_end(&mut data)
                    .map_err(|e| FsError::ArchiveError(e.to_string()))?;
                writer
                    .start_file(&name, options)
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
                writer
                    .write_all(&data)
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
            }

            // Write active slides, notes, and slide rels
            for (path, xml) in updated_slides_map {
                writer
                    .start_file(&path, options)
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
                writer
                    .write_all(xml.as_bytes())
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
            }
            for (path, xml) in updated_notes_map {
                writer
                    .start_file(&path, options)
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
                writer
                    .write_all(xml.as_bytes())
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
            }
            for (path, xml) in updated_slide_rels_map {
                writer
                    .start_file(&path, options)
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
                writer
                    .write_all(xml.as_bytes())
                    .map_err(|e| FsError::WriteError(e.to_string()))?;
            }

            // Write updated [Content_Types].xml, docProps/core.xml, ppt/presentation.xml, ppt/_rels/presentation.xml.rels
            let content_types =
                Self::generate_content_types_xml(slides, &existing_content_types_extra);
            writer
                .start_file("[Content_Types].xml", options)
                .map_err(|e| FsError::WriteError(e.to_string()))?;
            writer
                .write_all(content_types.as_bytes())
                .map_err(|e| FsError::WriteError(e.to_string()))?;

            let core_xml = Self::generate_core_xml(metadata);
            writer
                .start_file("docProps/core.xml", options)
                .map_err(|e| FsError::WriteError(e.to_string()))?;
            writer
                .write_all(core_xml.as_bytes())
                .map_err(|e| FsError::WriteError(e.to_string()))?;

            let pres_xml = Self::generate_presentation_xml(slides, metadata);
            writer
                .start_file("ppt/presentation.xml", options)
                .map_err(|e| FsError::WriteError(e.to_string()))?;
            writer
                .write_all(pres_xml.as_bytes())
                .map_err(|e| FsError::WriteError(e.to_string()))?;

            let pres_rels = Self::generate_presentation_rels_xml(slides, &existing_pres_rels_extra);
            writer
                .start_file("ppt/_rels/presentation.xml.rels", options)
                .map_err(|e| FsError::WriteError(e.to_string()))?;
            writer
                .write_all(pres_rels.as_bytes())
                .map_err(|e| FsError::WriteError(e.to_string()))?;

            writer
                .finish()
                .map_err(|e| FsError::WriteError(format!("Zip finalize error: {}", e)))?;
        } else {
            // Scaffold new minimal standard PPTX container
            Self::scaffold_new_pptx(&mut out_buf, slides, metadata)?;
        }

        Ok(out_buf.into_inner())
    }

    fn generate_core_xml(metadata: &PptxMetadata) -> String {
        let title_esc =
            quick_xml::escape::escape(metadata.title.as_deref().unwrap_or("Presentation"));
        let author_esc =
            quick_xml::escape::escape(metadata.author.as_deref().unwrap_or("MindSet OS User"));
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/">
  <dc:title>{}</dc:title>
  <dc:creator>{}</dc:creator>
</cp:coreProperties>"#,
            title_esc, author_esc
        )
    }

    fn generate_content_types_xml(slides: &[PptxSlide], extra_overrides: &str) -> String {
        let mut content_types = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
"#,
        );
        content_types.push_str(extra_overrides);
        for (i, s) in slides.iter().enumerate() {
            let num = if s.slide_number > 0 {
                s.slide_number
            } else {
                i + 1
            };
            content_types.push_str(&format!(
                "  <Override PartName=\"/ppt/slides/slide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>\n",
                num
            ));
            if s.notes.as_deref().is_some_and(|n| !n.trim().is_empty()) {
                content_types.push_str(&format!(
                    "  <Override PartName=\"/ppt/notesSlides/notesSlide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\"/>\n",
                    num
                ));
            }
        }
        content_types.push_str("</Types>");
        content_types
    }

    fn generate_presentation_xml(slides: &[PptxSlide], metadata: &PptxMetadata) -> String {
        let mut sld_id_lst = String::new();
        for (i, _s) in slides.iter().enumerate() {
            sld_id_lst.push_str(&format!(
                "    <p:sldId id=\"{}\" r:id=\"rId{}\"/>\n",
                256 + i,
                i + 1
            ));
        }
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst/>
  <p:sldIdLst>
{sld_ids}  </p:sldIdLst>
  <p:sldSz cx="{cx}" cy="{cy}"/>
</p:presentation>"#,
            sld_ids = sld_id_lst,
            cx = metadata.slide_width.unwrap_or(12192000),
            cy = metadata.slide_height.unwrap_or(6858000)
        )
    }

    fn generate_presentation_rels_xml(slides: &[PptxSlide], extra_rels: &str) -> String {
        let mut pres_rels = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
"#,
        );
        for (i, s) in slides.iter().enumerate() {
            let num = if s.slide_number > 0 {
                s.slide_number
            } else {
                i + 1
            };
            pres_rels.push_str(&format!(
                "  <Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{}.xml\"/>\n",
                i + 1,
                num
            ));
        }
        pres_rels.push_str(extra_rels);
        pres_rels.push_str("</Relationships>");
        pres_rels
    }

    fn generate_notes_xml(note: &str) -> String {
        let esc = quick_xml::escape::escape(note);
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Notes Placeholder 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>{}</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:notes>"#,
            esc
        )
    }

    fn generate_slide_rels_xml(slide_num: usize) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" Target="../notesSlides/notesSlide{}.xml"/>
</Relationships>"#,
            slide_num
        )
    }

    fn generate_slide_xml(slide: &PptxSlide) -> String {
        let title_shape = if let Some(t) = &slide.title {
            let title_escaped = quick_xml::escape::escape(t);
            format!(
                r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:rPr lang="en-US" sz="3200" b="1"/><a:t>{title}</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
"#,
                title = title_escaped
            )
        } else {
            String::new()
        };

        let mut body_paragraphs = String::new();
        for b in &slide.body_blocks {
            let esc = quick_xml::escape::escape(b);
            body_paragraphs.push_str(&format!(
                "<a:p><a:r><a:rPr lang=\"en-US\" sz=\"1800\"/><a:t>{}</a:t></a:r></a:p>",
                esc
            ));
        }

        let extra_shapes = if !slide.shape_texts.is_empty() {
            let mut shape_paragraphs = String::new();
            for s in &slide.shape_texts {
                let esc = quick_xml::escape::escape(s);
                shape_paragraphs.push_str(&format!(
                    "<a:p><a:r><a:rPr lang=\"en-US\" sz=\"1400\"/><a:t>{}</a:t></a:r></a:p>",
                    esc
                ));
            }
            format!(
                r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="4" name="ShapeText 4"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          {shape_paragraphs}
        </p:txBody>
      </p:sp>
"#,
                shape_paragraphs = shape_paragraphs
            )
        } else {
            String::new()
        };

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
{title_shape}      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Content 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          {body}
        </p:txBody>
      </p:sp>
{extra_shapes}    </p:spTree>
  </p:cSld>
</p:sld>"#,
            title_shape = title_shape,
            body = body_paragraphs,
            extra_shapes = extra_shapes
        )
    }

    fn scaffold_new_pptx(
        writer_target: &mut Cursor<Vec<u8>>,
        slides: &[PptxSlide],
        metadata: &PptxMetadata,
    ) -> Result<(), FsError> {
        let mut writer = ZipWriter::new(writer_target);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

        // 1. [Content_Types].xml
        let content_types = Self::generate_content_types_xml(slides, "");
        writer
            .start_file("[Content_Types].xml", options)
            .map_err(|e| FsError::WriteError(e.to_string()))?;
        writer
            .write_all(content_types.as_bytes())
            .map_err(|e| FsError::WriteError(e.to_string()))?;

        // 2. _rels/.rels
        let root_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
</Relationships>"#;
        writer
            .start_file("_rels/.rels", options)
            .map_err(|e| FsError::WriteError(e.to_string()))?;
        writer
            .write_all(root_rels.as_bytes())
            .map_err(|e| FsError::WriteError(e.to_string()))?;

        // 3. docProps/core.xml
        let core_xml = Self::generate_core_xml(metadata);
        writer
            .start_file("docProps/core.xml", options)
            .map_err(|e| FsError::WriteError(e.to_string()))?;
        writer
            .write_all(core_xml.as_bytes())
            .map_err(|e| FsError::WriteError(e.to_string()))?;

        // 4. ppt/presentation.xml
        let pres_xml = Self::generate_presentation_xml(slides, metadata);
        writer
            .start_file("ppt/presentation.xml", options)
            .map_err(|e| FsError::WriteError(e.to_string()))?;
        writer
            .write_all(pres_xml.as_bytes())
            .map_err(|e| FsError::WriteError(e.to_string()))?;

        // 5. ppt/_rels/presentation.xml.rels
        let pres_rels = Self::generate_presentation_rels_xml(slides, "");
        writer
            .start_file("ppt/_rels/presentation.xml.rels", options)
            .map_err(|e| FsError::WriteError(e.to_string()))?;
        writer
            .write_all(pres_rels.as_bytes())
            .map_err(|e| FsError::WriteError(e.to_string()))?;

        // 6. Slides and Notes
        for (i, s) in slides.iter().enumerate() {
            let num = if s.slide_number > 0 {
                s.slide_number
            } else {
                i + 1
            };
            let slide_file_path = format!("ppt/slides/slide{}.xml", num);
            let slide_xml = Self::generate_slide_xml(s);
            writer
                .start_file(&slide_file_path, options)
                .map_err(|e| FsError::WriteError(e.to_string()))?;
            writer
                .write_all(slide_xml.as_bytes())
                .map_err(|e| FsError::WriteError(e.to_string()))?;

            if let Some(note) = &s.notes {
                if !note.trim().is_empty() {
                    let note_path = format!("ppt/notesSlides/notesSlide{}.xml", num);
                    let note_xml = Self::generate_notes_xml(note);
                    writer
                        .start_file(&note_path, options)
                        .map_err(|e| FsError::WriteError(e.to_string()))?;
                    writer
                        .write_all(note_xml.as_bytes())
                        .map_err(|e| FsError::WriteError(e.to_string()))?;

                    let rels_path = format!("ppt/slides/_rels/slide{}.xml.rels", num);
                    let rels_xml = Self::generate_slide_rels_xml(num);
                    writer
                        .start_file(&rels_path, options)
                        .map_err(|e| FsError::WriteError(e.to_string()))?;
                    writer
                        .write_all(rels_xml.as_bytes())
                        .map_err(|e| FsError::WriteError(e.to_string()))?;
                }
            }
        }

        writer
            .finish()
            .map_err(|e| FsError::WriteError(format!("Zip finalize error: {}", e)))?;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 6. Legacy PPT Engine (Safe Binary Stream Extraction Fallback)
// ---------------------------------------------------------------------------

pub struct LegacyPptEngine;

impl LegacyPptEngine {
    /// Safe binary parser for legacy .ppt OLE2 Compound Document streams.
    /// Extracts text atoms and slide markers without panics on corrupted files.
    pub fn parse(bytes: &[u8]) -> Result<DocumentContent, FsError> {
        let mut slides = Vec::new();
        let mut current_slide_texts: Vec<String> = Vec::new();

        let mut i = 0;
        let len = bytes.len();

        while i + 8 <= len {
            // PPT Record Header:
            // 2 bytes: recVer (4 bits) + recInstance (12 bits)
            // 2 bytes: recType (e.g. 0x0F9F = Slide, 0x0FA8 = TextCharsAtom, 0x0FA9 = TextBytesAtom)
            // 4 bytes: recLen
            let rec_type = u16::from_le_bytes([bytes[i + 2], bytes[i + 3]]);
            let rec_len =
                u32::from_le_bytes([bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]])
                    as usize;

            let data_start = i + 8;
            let data_end = (data_start + rec_len).min(len);

            match rec_type {
                0x0F9F => {
                    // Slide Container marker: boundary for a new slide
                    if !current_slide_texts.is_empty() {
                        let title = current_slide_texts.first().cloned();
                        let body = if current_slide_texts.len() > 1 {
                            current_slide_texts[1..].to_vec()
                        } else {
                            Vec::new()
                        };
                        slides.push(PptxSlide {
                            slide_number: slides.len() + 1,
                            title,
                            body_blocks: body,
                            notes: None,
                            shape_texts: Vec::new(),
                        });
                        current_slide_texts.clear();
                    }
                }
                0x0FA8 => {
                    // TextCharsAtom: UTF-16LE text
                    if data_end > data_start {
                        let text_bytes = &bytes[data_start..data_end];
                        let u16s: Vec<u16> = text_bytes
                            .chunks_exact(2)
                            .map(|c| u16::from_le_bytes([c[0], c[1]]))
                            .collect();
                        if let Ok(s) = String::from_utf16(&u16s) {
                            let clean = s.trim().to_string();
                            if !clean.is_empty() && clean.len() >= 2 {
                                current_slide_texts.push(clean);
                            }
                        }
                    }
                }
                0x0FA9 => {
                    // TextBytesAtom: 8-bit ANSI text
                    if data_end > data_start {
                        let text_bytes = &bytes[data_start..data_end];
                        let s = String::from_utf8_lossy(text_bytes).trim().to_string();
                        if !s.is_empty() && s.len() >= 2 {
                            current_slide_texts.push(s);
                        }
                    }
                }
                _ => {}
            }

            // Advance cursor: ensure forward progress to prevent infinite loop
            if rec_len > 0 && data_end > i {
                i = data_end;
            } else {
                i += 1;
            }
        }

        // Flush trailing slide texts
        if !current_slide_texts.is_empty() {
            let title = current_slide_texts.first().cloned();
            let body = if current_slide_texts.len() > 1 {
                current_slide_texts[1..].to_vec()
            } else {
                Vec::new()
            };
            slides.push(PptxSlide {
                slide_number: slides.len() + 1,
                title,
                body_blocks: body,
                notes: None,
                shape_texts: Vec::new(),
            });
        }

        // Fallback: If no structured PPT records were decoded, extract printable ASCII sequences
        if slides.is_empty() {
            let fallback_text = Self::extract_printable_strings(bytes);
            if !fallback_text.is_empty() {
                slides.push(PptxSlide {
                    slide_number: 1,
                    title: Some("Legacy Presentation".to_string()),
                    body_blocks: vec![fallback_text],
                    notes: None,
                    shape_texts: Vec::new(),
                });
            } else {
                slides.push(PptxSlide {
                    slide_number: 1,
                    title: Some("Legacy Presentation (Binary)".to_string()),
                    body_blocks: vec!["[Binary stream: no printable text detected]".to_string()],
                    notes: None,
                    shape_texts: Vec::new(),
                });
            }
        }

        let slide_count = slides.len();
        Ok(DocumentContent::Pptx {
            slides,
            metadata: PptxMetadata {
                title: Some("Legacy PPT Document".to_string()),
                author: None,
                slide_count,
                slide_width: Some(9144000), // Standard 4:3 EMU
                slide_height: Some(6858000),
            },
        })
    }

    fn extract_printable_strings(bytes: &[u8]) -> String {
        let mut lines = Vec::new();
        let mut cur = String::new();
        for &b in bytes {
            if b.is_ascii_graphic() || b == b' ' {
                cur.push(b as char);
            } else {
                if cur.len() >= 4 {
                    lines.push(cur.clone());
                }
                cur.clear();
            }
        }
        if cur.len() >= 4 {
            lines.push(cur);
        }
        lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// 7. Unified Document Handler
// ---------------------------------------------------------------------------

pub struct DocumentHandler;

impl DocumentHandler {
    /// Reads and parses a document from the specified filesystem path.
    pub fn read(path: &Path) -> Result<DocumentContent, FsError> {
        if !path.exists() {
            return Err(FsError::NotFound(path.display().to_string()));
        }

        let bytes = fs::read(path)?;
        let format = detect_format(path, &bytes);

        Self::read_bytes(&bytes, Some(format))
    }

    /// Reads and parses a document from raw bytes with an optional format hint.
    pub fn read_bytes(
        bytes: &[u8],
        format_hint: Option<DocumentFormat>,
    ) -> Result<DocumentContent, FsError> {
        let dummy_path = PathBuf::from("document");
        let format = format_hint.unwrap_or_else(|| detect_format(&dummy_path, bytes));

        match format {
            DocumentFormat::Pdf => PdfEngine::parse(bytes),
            DocumentFormat::Pptx => PptxEngine::parse(bytes),
            DocumentFormat::Ppt => LegacyPptEngine::parse(bytes),
            DocumentFormat::Text | DocumentFormat::Markdown => {
                let text = String::from_utf8(bytes.to_vec()).or_else(|_| {
                    Ok::<String, FsError>(String::from_utf8_lossy(bytes).to_string())
                })?;
                Ok(DocumentContent::Text(text))
            }
            DocumentFormat::Unknown => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    Ok(DocumentContent::Text(text))
                } else {
                    Err(FsError::UnsupportedFormat(
                        "Unknown or unsupported binary document format".to_string(),
                    ))
                }
            }
        }
    }

    /// Saves or updates document content back to disk.
    pub fn save_update(path: &Path, content: &DocumentContent) -> Result<(), FsError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let original_bytes = if path.exists() {
            fs::read(path).ok()
        } else {
            None
        };

        let output_bytes = match content {
            DocumentContent::Text(t) => t.as_bytes().to_vec(),
            DocumentContent::Pdf { pages, metadata } => PdfEngine::generate_pdf(pages, metadata)?,
            DocumentContent::Pptx { slides, metadata } => {
                PptxEngine::save_or_update(slides, metadata, original_bytes.as_deref())?
            }
        };

        // Atomic write: write to temp file then rename
        let temp_path = path.with_extension(format!(
            "{}.tmp",
            path.extension().and_then(|e| e.to_str()).unwrap_or("file")
        ));
        fs::write(&temp_path, &output_bytes)?;
        fs::rename(&temp_path, path)?;

        Ok(())
    }

    /// Serializes document content into valid bytes in memory.
    pub fn write_bytes(content: &DocumentContent) -> Result<Vec<u8>, FsError> {
        match content {
            DocumentContent::Text(t) => Ok(t.as_bytes().to_vec()),
            DocumentContent::Pdf { pages, metadata } => PdfEngine::generate_pdf(pages, metadata),
            DocumentContent::Pptx { slides, metadata } => {
                PptxEngine::save_or_update(slides, metadata, None)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 8. Helper Utilities
// ---------------------------------------------------------------------------

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn decode_hex_str(hex: &str) -> Option<Vec<u8>> {
    let clean: String = hex.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    if clean.len() % 2 != 0 {
        return None;
    }
    (0..clean.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean[i..i + 2], 16).ok())
        .collect()
}

fn resolve_xml_entity(name: &str) -> String {
    match name {
        "amp" => "&".to_string(),
        "apos" => "'".to_string(),
        "quot" => "\"".to_string(),
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        n if n.starts_with("#x") || n.starts_with("#X") => u32::from_str_radix(&n[2..], 16)
            .ok()
            .and_then(char::from_u32)
            .map(|c| c.to_string())
            .unwrap_or_default(),
        n if n.starts_with('#') => n[1..]
            .parse::<u32>()
            .ok()
            .and_then(char::from_u32)
            .map(|c| c.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

// ---------------------------------------------------------------------------
// 9. Comprehensive Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_generation_and_parse_roundtrip() {
        let pages = vec![
            PdfPage {
                page_number: 1,
                text: "MindSet OS — Examination Syllabus 2026\nPhysics and Mathematics Chapter 10"
                    .to_string(),
            },
            PdfPage {
                page_number: 2,
                text: "Quantum Mechanics & Electric Flux\nGauss Law: flux = Q / epsilon_0"
                    .to_string(),
            },
        ];

        let metadata = PdfMetadata {
            title: Some("Syllabus Mastery".to_string()),
            author: Some("MindSet Scholar".to_string()),
            page_count: 2,
            creation_date: Some("2026-11-24".to_string()),
            producer: Some("MindSet Engine".to_string()),
            subject: Some("Physics".to_string()),
        };

        // Generate PDF bytes
        let pdf_bytes = PdfEngine::generate_pdf(&pages, &metadata).expect("PDF generation failed");
        assert!(pdf_bytes.starts_with(b"%PDF-1.4"));
        assert!(pdf_bytes.ends_with(b"%%EOF\n"));

        // Parse generated PDF back
        let doc = PdfEngine::parse(&pdf_bytes).expect("PDF parsing failed");
        match doc {
            DocumentContent::Pdf {
                pages: parsed_pages,
                metadata: parsed_meta,
            } => {
                assert_eq!(parsed_pages.len(), 2);
                assert_eq!(parsed_pages[0].page_number, 1);
                assert!(parsed_pages[0].text.contains("MindSet OS"));
                assert!(parsed_pages[0].text.contains("Syllabus 2026"));
                assert_eq!(parsed_pages[1].page_number, 2);
                assert!(parsed_pages[1].text.contains("Quantum Mechanics"));
                assert_eq!(parsed_meta.title.as_deref(), Some("Syllabus Mastery"));
                assert_eq!(parsed_meta.author.as_deref(), Some("MindSet Scholar"));
            }
            _ => panic!("Expected PDF document content"),
        }
    }

    #[test]
    fn test_pptx_scaffold_parse_and_edit_cycle() {
        let slides = vec![
            PptxSlide {
                slide_number: 1,
                title: Some("Welcome to MindSet OS".to_string()),
                body_blocks: vec!["Pure-Rust Desktop Operating System".to_string()],
                notes: Some("Opening slide remarks".to_string()),
                shape_texts: vec!["Confidential".to_string()],
            },
            PptxSlide {
                slide_number: 2,
                title: Some("Architecture Highlights".to_string()),
                body_blocks: vec![
                    "Zero-RAM liquidmorphism shaders".to_string(),
                    "100% offline study vault".to_string(),
                ],
                notes: None,
                shape_texts: Vec::new(),
            },
        ];

        let meta = PptxMetadata {
            title: Some("MindSet OS Overview".to_string()),
            author: Some("LKS Engineering".to_string()),
            slide_count: 2,
            slide_width: Some(12192000),
            slide_height: Some(6858000),
        };

        // 1. Scaffold new PPTX
        let pptx_bytes =
            PptxEngine::save_or_update(&slides, &meta, None).expect("PPTX scaffold failed");
        assert!(pptx_bytes.starts_with(&[0x50, 0x4B, 0x03, 0x04])); // PKZip magic

        // 2. Parse PPTX
        let doc = PptxEngine::parse(&pptx_bytes).expect("PPTX parse failed");
        match &doc {
            DocumentContent::Pptx {
                slides: read_slides,
                metadata: read_meta,
            } => {
                assert_eq!(read_slides.len(), 2);
                assert_eq!(
                    read_slides[0].title.as_deref(),
                    Some("Welcome to MindSet OS")
                );
                assert!(read_slides[0].body_blocks[0].contains("Pure-Rust Desktop"));
                assert_eq!(
                    read_slides[1].title.as_deref(),
                    Some("Architecture Highlights")
                );
                assert_eq!(read_meta.title.as_deref(), Some("MindSet OS Overview"));
            }
            _ => panic!("Expected PPTX content"),
        }

        // 3. Edit slide 1 and update in-place
        let mut edited_slides = slides.clone();
        edited_slides[0].title = Some("UPDATED: MindSet OS Production Launch".to_string());
        edited_slides[0].body_blocks = vec!["Over 500 National Rank Target".to_string()];

        let updated_pptx_bytes =
            PptxEngine::save_or_update(&edited_slides, &meta, Some(&pptx_bytes))
                .expect("PPTX in-place update failed");

        // 4. Re-parse updated PPTX and verify changes persisted
        let updated_doc =
            PptxEngine::parse(&updated_pptx_bytes).expect("Updated PPTX parse failed");
        match updated_doc {
            DocumentContent::Pptx {
                slides: re_read_slides,
                ..
            } => {
                assert_eq!(re_read_slides.len(), 2);
                assert_eq!(
                    re_read_slides[0].title.as_deref(),
                    Some("UPDATED: MindSet OS Production Launch")
                );
                assert_eq!(
                    re_read_slides[0].body_blocks[0],
                    "Over 500 National Rank Target"
                );
                // Slide 2 remains untouched
                assert_eq!(
                    re_read_slides[1].title.as_deref(),
                    Some("Architecture Highlights")
                );
            }
            _ => panic!("Expected PPTX content"),
        }
    }

    #[test]
    fn test_legacy_ppt_binary_fallback() {
        // Construct mock OLE2 binary stream with Slide and TextCharsAtom / TextBytesAtom
        let mut mock_ppt = vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]; // OLE2 Header
        mock_ppt.extend_from_slice(&[0x00; 16]); // padding

        // Record 1: Slide Container (recType = 0x0F9F, len = 0)
        mock_ppt.extend_from_slice(&[0x0F, 0x00, 0x9F, 0x0F, 0x00, 0x00, 0x00, 0x00]);

        // Record 2: TextBytesAtom (recType = 0x0FA9, len = 14)
        let text_bytes = b"Electrostatics";
        let len_bytes = (text_bytes.len() as u32).to_le_bytes();
        mock_ppt.extend_from_slice(&[0x00, 0x00, 0xA9, 0x0F]);
        mock_ppt.extend_from_slice(&len_bytes);
        mock_ppt.extend_from_slice(text_bytes);

        let doc = LegacyPptEngine::parse(&mock_ppt).expect("Legacy PPT parse failed");
        match doc {
            DocumentContent::Pptx { slides, .. } => {
                assert!(!slides.is_empty());
                let first_slide = &slides[0];
                assert!(
                    first_slide.title.as_deref() == Some("Electrostatics")
                        || first_slide
                            .body_blocks
                            .iter()
                            .any(|b| b.contains("Electrostatics"))
                );
            }
            _ => panic!("Expected presentation content"),
        }
    }

    #[test]
    fn test_document_handler_format_detection() {
        let pdf_sample = b"%PDF-1.7\n1 0 obj...";
        assert_eq!(
            detect_format(Path::new("doc.unknown"), pdf_sample),
            DocumentFormat::Pdf
        );

        let ppt_sample = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00];
        assert_eq!(
            detect_format(Path::new("slides.bin"), ppt_sample),
            DocumentFormat::Ppt
        );

        let text_sample = b"# Formula Sheet\nE = mc^2";
        assert_eq!(
            detect_format(Path::new("sheet.md"), text_sample),
            DocumentFormat::Markdown
        );
        assert_eq!(
            detect_format(Path::new("sheet.txt"), text_sample),
            DocumentFormat::Text
        );
    }

    #[test]
    fn test_document_handler_save_update_roundtrip() {
        let temp_dir = std::env::temp_dir().join(format!("mindset_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);

        let test_file = temp_dir.join("test_document.pdf");
        let pages = vec![PdfPage {
            page_number: 1,
            text: "Atomic Structure and Photoelectric Effect".to_string(),
        }];
        let meta = PdfMetadata {
            title: Some("Chemistry Unit 2".to_string()),
            ..Default::default()
        };
        let content = DocumentContent::Pdf {
            pages,
            metadata: meta,
        };

        // Save
        DocumentHandler::save_update(&test_file, &content).expect("Save update failed");
        assert!(test_file.exists());

        // Read
        let read_content = DocumentHandler::read(&test_file).expect("Read failed");
        match read_content {
            DocumentContent::Pdf { pages: p, .. } => {
                assert_eq!(p.len(), 1);
                assert!(p[0].text.contains("Atomic Structure"));
            }
            _ => panic!("Expected PDF content"),
        }

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_pdf_flatedecode_stream_ending_in_newline_byte_adler32() {
        // 1. Generate 4-page PDF with varied academic formulas
        let pages = vec![
            PdfPage {
                page_number: 1,
                text: "Coulomb's Law: F = k * q1 * q2 / r^2\nElectric Field: E = F / q0"
                    .to_string(),
            },
            PdfPage {
                page_number: 2,
                text: "Quantum Mechanics & Wavefunctions\nSchrodinger Equation: H psi = E psi"
                    .to_string(),
            },
            PdfPage {
                page_number: 3,
                text: "Thermodynamics: dU = dQ - dW\nCarnot Efficiency: eta = 1 - Tc/Th"
                    .to_string(),
            },
            PdfPage {
                page_number: 4,
                text: "Optics and Snell's Law: n1 * sin(theta1) = n2 * sin(theta2)".to_string(),
            },
        ];

        let metadata = PdfMetadata {
            title: Some("Physics Comprehensive Examination 2026".to_string()),
            author: Some("MindSet Faculty".to_string()),
            page_count: 4,
            creation_date: Some("2026-11-24".to_string()),
            producer: Some("MindSet Engine".to_string()),
            subject: Some("Physics".to_string()),
        };

        let pdf_bytes = PdfEngine::generate_pdf(&pages, &metadata).expect("PDF generation failed");
        let parsed = PdfEngine::parse(&pdf_bytes).expect("PDF parse failed");

        match parsed {
            DocumentContent::Pdf {
                pages: parsed_pages,
                metadata: parsed_meta,
            } => {
                assert_eq!(parsed_pages.len(), 4);
                assert!(
                    parsed_pages[0].text.contains("Coulomb's Law"),
                    "Page 1 missing Coulomb's Law: {}",
                    parsed_pages[0].text
                );
                assert!(
                    parsed_pages[1].text.contains("Quantum Mechanics"),
                    "Page 2 missing Quantum Mechanics: {}",
                    parsed_pages[1].text
                );
                assert!(
                    parsed_pages[2].text.contains("Thermodynamics"),
                    "Page 3 missing Thermodynamics: {}",
                    parsed_pages[2].text
                );
                assert!(
                    parsed_pages[3].text.contains("Snell's Law"),
                    "Page 4 missing Snell's Law: {}",
                    parsed_pages[3].text
                );
                assert_eq!(
                    parsed_meta.title.as_deref(),
                    Some("Physics Comprehensive Examination 2026")
                );
            }
            _ => panic!("Expected PDF document"),
        }

        // 2. Direct test: synthesize a FlateDecode stream whose compressed Adler-32 ends in 0x0A
        // In RFC 1950 zlib format, the last byte is the low 8 bits of s1 = (1 + sum(bytes)) % 65521.
        let base_payload = b"BT /F1 12 Tf 72 720 Td (Coulomb Adler-32 Edge Case) ' ET\n";
        let byte_sum: u32 = base_payload.iter().map(|&b| b as u32).sum();
        let current_low = (1 + byte_sum) % 256;
        let delta = (10 + 256 - current_low) % 256;

        let mut raw_payload = base_payload.to_vec();
        if delta == 0 {
            raw_payload.push(128);
            raw_payload.push(128);
        } else {
            raw_payload.push(delta as u8);
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&raw_payload).expect("Compression failed");
        let target_compressed = encoder.finish().expect("Finish failed");

        assert_eq!(
            *target_compressed.last().unwrap(),
            0x0A,
            "Target compressed stream must end in 0x0A"
        );

        let mut synthetic_pdf = Vec::new();
        synthetic_pdf.extend_from_slice(b"%PDF-1.4\n1 0 obj\n<< /Length ");
        synthetic_pdf.extend_from_slice(target_compressed.len().to_string().as_bytes());
        synthetic_pdf.extend_from_slice(b" /Filter /FlateDecode >>\nstream\n");
        synthetic_pdf.extend_from_slice(&target_compressed);
        synthetic_pdf.extend_from_slice(b"\nendstream\nendobj\n%%EOF\n");

        let streams = PdfEngine::extract_streams(&synthetic_pdf).expect("Extract streams");
        assert_eq!(streams.len(), 1, "Must extract 1 stream");
        let decompressed_str = String::from_utf8_lossy(&streams[0]);
        assert!(
            decompressed_str.contains("Coulomb Adler-32 Edge Case"),
            "Decompressed stream must contain valid text, got raw bytes: {:?}",
            &streams[0][..streams[0].len().min(30)]
        );
    }

    #[test]
    fn test_pptx_xml_entity_handling_and_no_spurious_spaces() {
        // 1. Raw XML with entities in title, body runs, and mixed text
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:txBody>
          <a:p>
            <a:r><a:t>Snell&apos;s Law</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Content 2"/>
          <p:nvPr><p:ph type="body"/></p:nvPr>
        </p:nvSpPr>
        <p:txBody>
          <a:p>
            <a:r><a:t>Quantum Mechanics &amp; Wavefunctions</a:t></a:r>
          </a:p>
          <a:p>
            <a:r><a:t>Formula: a &lt; b &amp;&amp; c &gt; d &quot;Quoted&quot;</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let parsed_slide = PptxEngine::parse_slide_xml(1, slide_xml, None);
        assert_eq!(parsed_slide.title.as_deref(), Some("Snell's Law"));
        assert_eq!(parsed_slide.body_blocks.len(), 2);
        assert_eq!(
            parsed_slide.body_blocks[0],
            "Quantum Mechanics & Wavefunctions"
        );
        assert_eq!(
            parsed_slide.body_blocks[1],
            "Formula: a < b && c > d \"Quoted\""
        );

        // 2. Metadata XML with entities in title and creator
        let core_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/">
  <dc:title>Physics &amp; Snell&apos;s Law</dc:title>
  <dc:creator>Prof. O&apos;Connor &amp; Dr. Smith</dc:creator>
</cp:coreProperties>"#;

        let mut meta = PptxMetadata::default();
        PptxEngine::extract_core_metadata(core_xml, &mut meta);
        assert_eq!(meta.title.as_deref(), Some("Physics & Snell's Law"));
        assert_eq!(meta.author.as_deref(), Some("Prof. O'Connor & Dr. Smith"));

        // 3. Plain XML text with notes containing entities
        let notes_xml = r#"<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:p><a:r><a:t>Review Snell&apos;s Law &amp; Brewster&apos;s Angle</a:t></a:r></a:p></p:notes>"#;
        let notes_text = PptxEngine::extract_plain_xml_text(notes_xml);
        assert_eq!(notes_text, "Review Snell's Law & Brewster's Angle");
    }

    #[test]
    fn test_format_detection_docx_openxml_not_classified_as_pptx() {
        // Construct a ZIP package with [Content_Types].xml and word/document.xml (DOCX)
        let mut buf = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut buf);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

            writer
                .start_file("[Content_Types].xml", options)
                .expect("Failed start file");
            writer
                .write_all(b"<Types><Override PartName=\"/word/document.xml\"/></Types>")
                .expect("Write failed");

            writer
                .start_file("word/document.xml", options)
                .expect("Failed start file");
            writer.write_all(b"<w:document><w:body><w:p><w:r><w:t>DOCX content</w:t></w:r></w:p></w:body></w:document>").expect("Write failed");

            writer.finish().expect("Finish failed");
        }
        let docx_zip_bytes = buf.into_inner();

        // 1. Without extension: must NOT be classified as PPTX
        let unknown_fmt = detect_format(Path::new("unknown_file.bin"), &docx_zip_bytes);
        assert_ne!(
            unknown_fmt,
            DocumentFormat::Pptx,
            "DOCX zip must not be detected as PPTX"
        );

        // 2. With docx extension: must NOT be classified as PPTX
        let docx_fmt = detect_format(Path::new("lecture.docx"), &docx_zip_bytes);
        assert_ne!(
            docx_fmt,
            DocumentFormat::Pptx,
            "lecture.docx must not be detected as PPTX"
        );

        // 3. A real PPTX zip with ppt/ must be classified as PPTX even without extension
        let mut pptx_buf = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut pptx_buf);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

            writer
                .start_file("[Content_Types].xml", options)
                .expect("Failed start file");
            writer
                .write_all(b"<Types><Override PartName=\"/ppt/presentation.xml\"/></Types>")
                .expect("Write failed");

            writer
                .start_file("ppt/presentation.xml", options)
                .expect("Failed start file");
            writer
                .write_all(b"<p:presentation/>")
                .expect("Write failed");

            writer.finish().expect("Finish failed");
        }
        let pptx_zip_bytes = pptx_buf.into_inner();
        let pptx_fmt = detect_format(Path::new("unknown_slides.bin"), &pptx_zip_bytes);
        assert_eq!(
            pptx_fmt,
            DocumentFormat::Pptx,
            "Zip with ppt/ must be detected as PPTX"
        );
    }

    #[test]
    fn test_pdf_utf8_roundtrip_empty_pages_and_corrupted_rejection() {
        // 1. UTF-8 roundtrip (em-dash, Tamil, Greek)
        let pages = vec![PdfPage {
            page_number: 1,
            text: "Gauss — மின்புலம் φ_E = q / ε_0".to_string(),
        }];
        let pdf_bytes =
            PdfEngine::generate_pdf(&pages, &PdfMetadata::default()).expect("UTF-8 PDF gen");
        match PdfEngine::parse(&pdf_bytes).expect("UTF-8 PDF parse") {
            DocumentContent::Pdf { pages: p, .. } => {
                assert_eq!(p.len(), 1);
                assert!(p[0].text.contains("—"));
                assert!(p[0].text.contains("மின்புலம்"));
                assert!(p[0].text.contains("ε_0"));
            }
            _ => panic!("Expected PDF"),
        }

        // 2. Empty pages generate_pdf emits valid 1 blank page & consistent xref
        let blank_bytes =
            PdfEngine::generate_pdf(&[], &PdfMetadata::default()).expect("Empty pages PDF gen");
        let blank_str = String::from_utf8_lossy(&blank_bytes);
        assert!(blank_str.contains("xref\n0 7\n"));
        assert!(blank_str.contains("/Count 1"));
        match PdfEngine::parse(&blank_bytes).expect("Blank PDF parse") {
            DocumentContent::Pdf { pages: p, metadata } => {
                assert_eq!(p.len(), 1);
                assert_eq!(metadata.page_count, 1);
            }
            _ => panic!("Expected PDF"),
        }

        // 3. Corrupted/empty PDFs must return Err(FsError::CorruptedDocument) rather than raw syntax
        let corrupt = b"%PDF-1.4\n1 0 obj\n<< /Filter /FlateDecode /Length 5 >>\nstream\nXXXXX\nendstream\nendobj\n";
        assert!(matches!(
            PdfEngine::parse(corrupt),
            Err(FsError::CorruptedDocument(_))
        ));
        assert!(matches!(
            PdfEngine::parse(b"%PDF-1.4\ngarbage"),
            Err(FsError::CorruptedDocument(_))
        ));
    }

    #[test]
    fn test_pptx_roundtrip_notes_shapes_untitled_metadata_and_slide_deletion() {
        let slides = vec![
            PptxSlide {
                slide_number: 1,
                title: None,
                body_blocks: vec!["Body bullet A".to_string()],
                notes: Some("Speaker note 1".to_string()),
                shape_texts: vec!["Callout Badge".to_string()],
            },
            PptxSlide {
                slide_number: 2,
                title: Some("Slide Two".to_string()),
                body_blocks: vec!["Body bullet B".to_string()],
                notes: Some("Speaker note 2".to_string()),
                shape_texts: Vec::new(),
            },
        ];
        let meta = PptxMetadata {
            title: Some("Original Deck".to_string()),
            author: Some("Author A".to_string()),
            ..Default::default()
        };

        let bytes = PptxEngine::save_or_update(&slides, &meta, None).expect("scaffold");
        let parsed = PptxEngine::parse(&bytes).expect("parse");
        match parsed {
            DocumentContent::Pptx { slides: s, .. } => {
                assert_eq!(s.len(), 2);
                assert_eq!(s[0].title, None, "title: None must be preserved");
                assert_eq!(s[0].body_blocks, vec!["Body bullet A"]);
                assert_eq!(s[0].shape_texts, vec!["Callout Badge"]);
                assert_eq!(s[0].notes.as_deref(), Some("Speaker note 1"));
                assert_eq!(s[1].notes.as_deref(), Some("Speaker note 2"));
            }
            _ => panic!("Expected PPTX"),
        }

        // Delete slide 2 and update metadata in-place
        let updated_slides = vec![PptxSlide {
            slide_number: 1,
            title: None,
            body_blocks: vec!["Edited Body".to_string()],
            notes: Some("Updated note 1".to_string()),
            shape_texts: vec!["Badge 2".to_string()],
        }];
        let updated_meta = PptxMetadata {
            title: Some("Updated Deck".to_string()),
            author: Some("Author B".to_string()),
            ..Default::default()
        };

        let updated_bytes =
            PptxEngine::save_or_update(&updated_slides, &updated_meta, Some(&bytes))
                .expect("update in-place");
        match PptxEngine::parse(&updated_bytes).expect("reparse") {
            DocumentContent::Pptx {
                slides: s,
                metadata: m,
            } => {
                assert_eq!(s.len(), 1, "Deleted slide 2 must be removed");
                assert_eq!(s[0].title, None);
                assert_eq!(s[0].body_blocks, vec!["Edited Body"]);
                assert_eq!(s[0].shape_texts, vec!["Badge 2"]);
                assert_eq!(s[0].notes.as_deref(), Some("Updated note 1"));
                assert_eq!(m.title.as_deref(), Some("Updated Deck"));
                assert_eq!(m.author.as_deref(), Some("Author B"));
                assert_eq!(m.slide_count, 1);
            }
            _ => panic!("Expected PPTX"),
        }
    }
}
