//! Local Document Chunker, Term-Vector Cosine Retriever & Study Vault Engine.
//! Connects to ~/.mindset_data/vault/ and local_core.sqlite for grounded AI answers.
//! Pure-Rust PDF Extraction and Page-Level Citation Anchoring for Maindy AI Study Workstation.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RagError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("PDF parse error: {0}")]
    PdfParse(String),

    #[error("Vault error: {0}")]
    Vault(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFileInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub preview: String,
}

/// A structured chunk extracted from an ingested PDF document with page citation anchors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfDocumentChunk {
    pub file_name: String,
    pub page_number: u32,
    pub heading: Option<String>,
    pub text_content: String,
    pub formula_tokens: Vec<String>,
}

/// A high-precision PDF citation search hit with exact page-level attribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfCitationMatch {
    pub chunk: PdfDocumentChunk,
    pub score: f32,
    pub citation_label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RagMatch {
    pub file_name: String,
    pub chunk_index: usize,
    pub snippet: String,
    pub score: f32,
    #[serde(default)]
    pub page_number: Option<u32>,
    #[serde(default)]
    pub heading: Option<String>,
    #[serde(default)]
    pub formula_tokens: Vec<String>,
}

#[derive(Debug, Clone)]
struct SearchCandidateChunk {
    file_name: String,
    chunk_index: usize,
    text: String,
    page_number: Option<u32>,
    heading: Option<String>,
    formula_tokens: Vec<String>,
}

pub struct RagVault {
    vault_dir: PathBuf,
}

impl Default for RagVault {
    fn default() -> Self {
        Self::new()
    }
}

impl RagVault {
    pub fn new() -> Self {
        let vault_dir = Self::default_vault_path();
        Self { vault_dir }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { vault_dir: path }
    }

    /// Default storage directory: ~/.mindset_data/vault/
    pub fn default_vault_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".mindset_data").join("vault")
    }

    /// Returns the active vault directory path.
    pub fn vault_dir(&self) -> &Path {
        &self.vault_dir
    }

    /// Ensures the vault directory exists and seeds 4 default study guides on first boot.
    pub fn initialize(&self) -> Result<(), String> {
        if !self.vault_dir.exists() {
            fs::create_dir_all(&self.vault_dir)
                .map_err(|e| format!("Failed to create vault directory: {}", e))?;
        }

        self.seed_default_notes_if_missing()?;
        Ok(())
    }

    /// Seeds 4 comprehensive Class 12 / undergraduate study notes.
    fn seed_default_notes_if_missing(&self) -> Result<(), String> {
        let seed_files = [
            ("01_Wave_Optics_Notes.md", SEED_WAVE_OPTICS),
            ("02_Electrochemistry_Formulas.md", SEED_ELECTROCHEMISTRY),
            ("03_Vector_Algebra_Summary.md", SEED_VECTOR_ALGEBRA),
            ("04_Python_SQLite_Guide.md", SEED_PYTHON_SQLITE),
        ];

        for (filename, content) in seed_files {
            let file_path = self.vault_dir.join(filename);
            if !file_path.exists() {
                fs::write(&file_path, content)
                    .map_err(|e| format!("Failed to seed {}: {}", filename, e))?;
            }
        }

        Ok(())
    }

    /// Lists all markdown, text, and PDF study files in the vault.
    pub fn list_notes(&self) -> Result<Vec<VaultFileInfo>, String> {
        if !self.vault_dir.exists() {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        let entries = fs::read_dir(&self.vault_dir)
            .map_err(|e| format!("Failed to read vault directory: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if ext == "md" || ext == "txt" || ext == "pdf" {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let metadata = entry.metadata().ok();
                    let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);

                    if ext == "pdf" {
                        let preview = format!("[PDF Textbook • {} KB]", size_bytes / 1024);
                        results.push(VaultFileInfo {
                            name,
                            path: path.to_string_lossy().to_string(),
                            size_bytes,
                            line_count: 0,
                            preview,
                        });
                    } else {
                        let content = fs::read_to_string(&path).unwrap_or_default();
                        let line_count = content.lines().count();
                        let preview: String = content.chars().take(120).collect();

                        results.push(VaultFileInfo {
                            name,
                            path: path.to_string_lossy().to_string(),
                            size_bytes,
                            line_count,
                            preview,
                        });
                    }
                }
            }
        }

        results.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(results)
    }

    /// Reads a note from the vault.
    pub fn read_note(&self, name: &str) -> Result<String, String> {
        let file_path = self.vault_dir.join(name);
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read {}: {}", name, e))
    }

    /// Writes or updates a note in the vault.
    pub fn write_note(&self, name: &str, content: &str) -> Result<(), String> {
        let file_path = self.vault_dir.join(name);
        fs::write(&file_path, content).map_err(|e| format!("Failed to write {}: {}", name, e))
    }

    /// Ingests an uploaded PDF from disk into the RAG vault.
    /// Extracts page-level text, parses headings and formulas, generates structured chunks with citation anchors,
    /// and saves them into the persistent vault index.
    pub fn ingest_pdf(&self, path: &Path) -> Result<Vec<PdfDocumentChunk>, RagError> {
        if !path.exists() {
            return Err(RagError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("PDF file not found at path: {:?}", path),
            )));
        }

        let file_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("document.pdf")
            .to_string();

        let bytes = fs::read(path)?;
        self.ingest_pdf_bytes(&file_name, &bytes)
    }

    /// Ingests a PDF document from raw byte buffer.
    /// Extracts page streams, detects formulas and section headings, and produces page-anchored chunks.
    pub fn ingest_pdf_bytes(
        &self,
        file_name: &str,
        bytes: &[u8],
    ) -> Result<Vec<PdfDocumentChunk>, RagError> {
        let doc_content = mindset_sys::fs::document::PdfEngine::parse(bytes)
            .map_err(|e| RagError::PdfParse(e.to_string()))?;

        let mut chunks = Vec::new();

        match doc_content {
            mindset_sys::fs::DocumentContent::Pdf { pages, .. } => {
                for page in pages {
                    let page_number = page.page_number as u32;
                    let page_text_raw = page.text;

                    // Repair any Latin-1 byte-as-char decoding from PDF stream to recover true UTF-8
                    let page_text = if page_text_raw
                        .chars()
                        .any(|c| (c as u32) > 127 && (c as u32) <= 255)
                    {
                        let bytes: Vec<u8> = page_text_raw.chars().map(|c| c as u8).collect();
                        String::from_utf8(bytes).unwrap_or(page_text_raw)
                    } else {
                        page_text_raw
                    };

                    let text = page_text.trim();
                    if text.is_empty() {
                        continue;
                    }

                    // Extract page-level heading or topic
                    let page_heading = Self::extract_heading_for_chunk(text);

                    // Split page content into semantic paragraph-sized chunks
                    let page_chunks = Self::chunk_text(text, 500);

                    for chunk_text in page_chunks {
                        let heading = Self::extract_heading_for_chunk(&chunk_text)
                            .or_else(|| page_heading.clone());
                        let formula_tokens = Self::extract_formula_tokens(&chunk_text);

                        chunks.push(PdfDocumentChunk {
                            file_name: file_name.to_string(),
                            page_number,
                            heading,
                            text_content: chunk_text,
                            formula_tokens,
                        });
                    }
                }
            }
            _ => {
                return Err(RagError::PdfParse(
                    "Unexpected document content returned by PDF engine".to_string(),
                ));
            }
        }

        // Persist newly indexed PDF chunks to the vault index
        if !chunks.is_empty() {
            let _ = self.save_pdf_chunks_index(&chunks);
        }

        Ok(chunks)
    }

    /// Extracts mathematical and scientific formula tokens from chunk text.
    pub fn extract_formula_tokens(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let math_symbols = [
            '=', '∫', '∑', 'λ', 'β', 'π', 'θ', 'Δ', 'ε', 'μ', 'ω', '²', '³', '√', '±', '×', '·',
        ];

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check if line contains mathematical equals or relation symbols
            let has_math_char = trimmed.chars().any(|c| math_symbols.contains(&c));
            let has_eq = trimmed.contains('=') || trimmed.contains("->") || trimmed.contains("=>");

            if (has_math_char && has_eq)
                || trimmed.starts_with("Formula:")
                || trimmed.starts_with("Equation:")
                || trimmed.contains("E_cell")
                || trimmed.contains("ΔG°")
                || trimmed.contains("Phi =")
            {
                let cleaned = trimmed
                    .trim_start_matches("Formula:")
                    .trim_start_matches("Equation:")
                    .trim_start_matches('-')
                    .trim_start_matches('*')
                    .trim()
                    .to_string();

                if cleaned.len() >= 3 && !tokens.contains(&cleaned) {
                    tokens.push(cleaned);
                }
            }
        }

        tokens
    }

    /// Extracts section or chapter heading candidate from chunk text.
    pub fn extract_heading_for_chunk(text: &str) -> Option<String> {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ")
                || trimmed.starts_with("## ")
                || trimmed.starts_with("### ")
            {
                let h = trimmed.trim_start_matches('#').trim().to_string();
                if !h.is_empty() {
                    return Some(h);
                }
            }
            if trimmed.starts_with("Chapter ")
                || trimmed.starts_with("Section ")
                || trimmed.starts_with("Unit ")
            {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    fn pdf_index_file(&self) -> PathBuf {
        self.vault_dir.join(".pdf_chunks_index.json")
    }

    /// Saves indexed chunks into the persistent JSON index file in vault_dir.
    pub fn save_pdf_chunks_index(&self, new_chunks: &[PdfDocumentChunk]) -> Result<(), RagError> {
        let index_path = self.pdf_index_file();
        let mut all_chunks = self.load_pdf_chunks_index();

        // Remove previous chunks for the same file names to avoid duplication on re-indexing
        let incoming_files: Vec<&str> = new_chunks.iter().map(|c| c.file_name.as_str()).collect();
        all_chunks.retain(|c| !incoming_files.contains(&c.file_name.as_str()));

        all_chunks.extend_from_slice(new_chunks);

        if let Ok(json) = serde_json::to_string_pretty(&all_chunks) {
            let _ = fs::write(&index_path, json);
        }

        Ok(())
    }

    /// Loads all indexed PDF document chunks from the persistent index file.
    pub fn load_pdf_chunks_index(&self) -> Vec<PdfDocumentChunk> {
        let index_path = self.pdf_index_file();
        if !index_path.exists() {
            return Vec::new();
        }

        if let Ok(content) = fs::read_to_string(&index_path) {
            if let Ok(chunks) = serde_json::from_str::<Vec<PdfDocumentChunk>>(&content) {
                return chunks;
            }
        }

        Vec::new()
    }

    /// Searches indexed PDF chunks and returns exact page citations.
    pub fn search_pdf_chunks(
        &self,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<PdfCitationMatch>, RagError> {
        let chunks = self.load_pdf_chunks_index();
        if chunks.is_empty() {
            return Ok(Vec::new());
        }

        let q_words = Self::tokenize(query);
        let mut matches = Vec::new();

        for chunk in chunks {
            let c_words = Self::tokenize(&chunk.text_content);
            let score = Self::compute_word_overlap_cosine(&q_words, &c_words);
            if score > 0.05 {
                let heading_part = chunk
                    .heading
                    .as_ref()
                    .map(|h| format!(", Section: {}", h))
                    .unwrap_or_default();
                let citation_label = format!(
                    "{} (Page {}{})",
                    chunk.file_name, chunk.page_number, heading_part
                );
                matches.push(PdfCitationMatch {
                    chunk,
                    score,
                    citation_label,
                });
            }
        }

        matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        matches.truncate(top_k);

        Ok(matches)
    }

    /// Chunks document text by paragraphs or sentences with length bounds.
    /// Safely handles paragraphs larger than max_chars by splitting across line breaks or whitespace.
    pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
        let max_len = if max_chars == 0 { 600 } else { max_chars };
        let mut chunks = Vec::new();
        let mut current = String::new();

        for paragraph in text.split("\n\n") {
            let p = paragraph.trim();
            if p.is_empty() {
                continue;
            }

            // If a single paragraph exceeds max_chars, split it across lines / sentences
            if p.len() > max_len {
                if !current.is_empty() {
                    chunks.push(current.trim().to_string());
                    current = String::new();
                }

                for line in p.lines() {
                    let l = line.trim();
                    if l.is_empty() {
                        continue;
                    }
                    if current.len() + l.len() > max_len && !current.is_empty() {
                        chunks.push(current.trim().to_string());
                        current = String::new();
                    }
                    if l.len() > max_len {
                        // Line itself is oversized: chunk by words/whitespace
                        for word in l.split_whitespace() {
                            if current.len() + word.len() + 1 > max_len && !current.is_empty() {
                                chunks.push(current.trim().to_string());
                                current = String::new();
                            }
                            if !current.is_empty() {
                                current.push(' ');
                            }
                            current.push_str(word);
                        }
                    } else {
                        if !current.is_empty() {
                            current.push('\n');
                        }
                        current.push_str(l);
                    }
                }
                continue;
            }

            if current.len() + p.len() > max_len && !current.is_empty() {
                chunks.push(current.trim().to_string());
                current = String::new();
            }

            if !current.is_empty() {
                current.push_str("\n\n");
            }
            current.push_str(p);
        }

        if !current.is_empty() {
            chunks.push(current.trim().to_string());
        }

        chunks
    }

    /// Searches all vault files (including indexed PDF textbooks) and returns top-K relevant chunks with cosine similarity scores.
    pub fn search(&self, query: &str, top_k: usize) -> Result<Vec<RagMatch>, String> {
        let notes = self.list_notes()?;
        let mut all_chunks: Vec<SearchCandidateChunk> = Vec::new();

        for note in notes {
            if note.name.ends_with(".pdf") {
                continue; // Handled below via indexed chunks
            }
            if let Ok(content) = self.read_note(&note.name) {
                let chunks = Self::chunk_text(&content, 600);
                for (idx, c) in chunks.into_iter().enumerate() {
                    let heading = Self::extract_heading_for_chunk(&c);
                    let formula_tokens = Self::extract_formula_tokens(&c);
                    all_chunks.push(SearchCandidateChunk {
                        file_name: note.name.clone(),
                        chunk_index: idx,
                        text: c,
                        page_number: None,
                        heading,
                        formula_tokens,
                    });
                }
            }
        }

        // Include all indexed PDF chunks with page citation anchors
        let pdf_chunks = self.load_pdf_chunks_index();
        for (idx, chunk) in pdf_chunks.into_iter().enumerate() {
            all_chunks.push(SearchCandidateChunk {
                file_name: chunk.file_name,
                chunk_index: idx,
                text: chunk.text_content,
                page_number: Some(chunk.page_number),
                heading: chunk.heading,
                formula_tokens: chunk.formula_tokens,
            });
        }

        if all_chunks.is_empty() {
            return Ok(Vec::new());
        }

        // Compute TF vector for query
        let q_words = Self::tokenize(query);
        let mut scored_matches = Vec::new();

        for candidate in all_chunks {
            let c_words = Self::tokenize(&candidate.text);
            let score = Self::compute_word_overlap_cosine(&q_words, &c_words);
            if score > 0.05 {
                scored_matches.push(RagMatch {
                    file_name: candidate.file_name,
                    chunk_index: candidate.chunk_index,
                    snippet: candidate.text,
                    score,
                    page_number: candidate.page_number,
                    heading: candidate.heading,
                    formula_tokens: candidate.formula_tokens,
                });
            }
        }

        scored_matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored_matches.truncate(top_k);

        Ok(scored_matches)
    }

    /// Builds a formatted grounded context string for prompt injection with exact page-level citations.
    pub fn build_context_citations(&self, query: &str, top_k: usize) -> String {
        match self.search(query, top_k) {
            Ok(matches) if !matches.is_empty() => {
                let mut out = String::new();
                for m in matches {
                    if let Some(pg) = m.page_number {
                        let heading_str = m
                            .heading
                            .as_ref()
                            .map(|h| format!(" [{}]", h))
                            .unwrap_or_default();
                        let formulas_str = if !m.formula_tokens.is_empty() {
                            format!("\nFormulas: {}", m.formula_tokens.join("; "))
                        } else {
                            String::new()
                        };
                        out.push_str(&format!(
                            "Source: {}{} (Page {}, Match: {:.0}%):\n{}{}\n\n",
                            m.file_name,
                            heading_str,
                            pg,
                            m.score * 100.0,
                            m.snippet.trim(),
                            formulas_str
                        ));
                    } else {
                        let heading_str = m
                            .heading
                            .as_ref()
                            .map(|h| format!(" [{}]", h))
                            .unwrap_or_default();
                        let formulas_str = if !m.formula_tokens.is_empty() {
                            format!("\nFormulas: {}", m.formula_tokens.join("; "))
                        } else {
                            String::new()
                        };
                        out.push_str(&format!(
                            "Source: {}{} (Chunk #{}, Match: {:.0}%):\n{}{}\n\n",
                            m.file_name,
                            heading_str,
                            m.chunk_index + 1,
                            m.score * 100.0,
                            m.snippet.trim(),
                            formulas_str
                        ));
                    }
                }
                out
            }
            _ => String::new(),
        }
    }

    fn tokenize(text: &str) -> HashMap<String, f32> {
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            let cleaned: String = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            if cleaned.len() >= 3 {
                *counts.entry(cleaned).or_insert(0.0) += 1.0;
            }
        }
        counts
    }

    /// Computes cosine similarity between two term-frequency vectors.
    /// Strictly guards against zero-vector division by zero and returns 0.0 for degenerate inputs.
    pub fn compute_word_overlap_cosine(a: &HashMap<String, f32>, b: &HashMap<String, f32>) -> f32 {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (k, va) in a {
            norm_a += va * va;
            if let Some(vb) = b.get(k) {
                dot += va * vb;
            }
        }

        for vb in b.values() {
            norm_b += vb * vb;
        }

        let denominator = norm_a.sqrt() * norm_b.sqrt();
        if denominator <= f32::EPSILON || !denominator.is_finite() {
            return 0.0;
        }

        let sim = dot / denominator;
        if sim.is_nan() || sim.is_infinite() {
            0.0
        } else {
            sim.clamp(0.0, 1.0)
        }
    }

    /// Computes standard cosine similarity between two arbitrary f32 slices.
    /// Strictly guards against empty slices, mismatched dimensions, zero magnitude vectors, and NaNs.
    pub fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
        if v1.is_empty() || v1.len() != v2.len() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for (a, b) in v1.iter().zip(v2.iter()) {
            if a.is_nan() || b.is_nan() {
                continue;
            }
            dot += a * b;
            norm1 += a * a;
            norm2 += b * b;
        }

        let denom = norm1.sqrt() * norm2.sqrt();
        if denom <= f32::EPSILON || !denom.is_finite() {
            return 0.0;
        }

        let sim = dot / denom;
        if sim.is_nan() || sim.is_infinite() {
            0.0
        } else {
            sim.clamp(-1.0, 1.0)
        }
    }
}

// ─── Default Study Notes Content ──────────────────────────────────────────────

const SEED_WAVE_OPTICS: &str = "\
# Wave Optics & Interference of Light (Class 12 Physics)

## 1. Huygens' Principle
Every point on a wavefront behaves as a secondary source of spherical wavelets. The envelope of these secondary wavelets at a later time gives the new wavefront position.
- Planar Wavefront: Originates from distant point source (e.g. sunlight).
- Spherical Wavefront: Originates from a localized point source.

## 2. Young's Double-Slit Experiment (YDSE)
Coherent sources produce sustained interference fringes on a screen placed at distance D from slits separated by distance d.

### Fringe Width Formula:
β = λD / d
Where:
- β = Fringe width (distance between two consecutive bright or dark fringes)
- λ = Wavelength of monochromatic light source
- D = Distance between slit plane and screen
- d = Separation between the two coherent slits

### Conditions for Constructive Interference (Bright Fringes):
Path Difference Δx = nλ (for n = 0, ±1, ±2, ...)
Position of nth bright fringe: y_n = n(λD / d)

### Conditions for Destructive Interference (Dark Fringes):
Path Difference Δx = (2n - 1)(λ / 2)
Position of nth dark fringe: y'_n = (2n - 1)(λD / 2d)
";

const SEED_ELECTROCHEMISTRY: &str = "\
# Electrochemistry & Nernst Equation (Class 12 Chemistry)

## 1. Galvanic Cells & Standard EMF
A galvanic cell converts chemical energy from spontaneous redox reactions into electrical energy.
Cell notation: Zn(s) | Zn²⁺(aq) || Cu²⁺(aq) | Cu(s)
Standard Cell Potential:
E°_cell = E°_cathode - E°_anode = E°(Cu²⁺/Cu) - E°(Zn²⁺/Zn) = +0.34V - (-0.76V) = +1.10V

## 2. Nernst Equation for Non-Standard Concentrations
Relates electrode / cell potential to ion concentrations at temperature T:
E_cell = E°_cell - (RT / nF) * ln(Q)

At 298 K (25 °C):
E_cell = E°_cell - (0.0591 / n) * log₁₀(Q)
Where:
- R = 8.314 J/(mol·K)
- T = 298.15 K
- F = Faraday's constant ≈ 96485 C/mol
- n = Number of moles of electrons transferred in the balanced redox reaction
- Q = Reaction quotient [Products]^p / [Reactants]^r

## 3. Gibbs Free Energy & Equilibrium Constant
ΔG° = -n * F * E°_cell
ΔG° = -2.303 * R * T * log₁₀(K_c)
For spontaneous cell reaction: E°_cell > 0 and ΔG° < 0.
";

const SEED_VECTOR_ALGEBRA: &str = "\
# Vector Algebra & 3D Geometry (Class 12 Mathematics)

## 1. Dot Product (Scalar Product)
a · b = |a| |b| cos(θ)
Component form: a · b = a₁b₁ + a₂b₂ + a₃b₃
- Orthogonality Condition: a ⊥ b <=> a · b = 0
- Projection of a on b: proj_b(a) = (a · b) / |b|

## 2. Cross Product (Vector Product)
a × b = |a| |b| sin(θ) n̂
Component form via 3x3 determinant:
|  i   j   k  |
| a₁  a₂  a₃ |
| b₁  b₂  b₃ |
- Collinearity Condition: a || b <=> a × b = 0
- Area of parallelogram spanned by a and b: Area = |a × b|
- Area of triangle spanned by vectors: Area = 0.5 * |a × b|

## 3. Direction Cosines & Ratios
If line makes angles α, β, γ with positive X, Y, Z axes:
l = cos(α), m = cos(β), n = cos(γ)
Identity: l² + m² + n² = 1
";

const SEED_PYTHON_SQLITE: &str = "\
# Computer Science: Python SQLite & Systems Architecture

## 1. WAL Mode (Write-Ahead Logging)
SQLite's Write-Ahead Logging delivers concurrent readers alongside a single active writer without reader-writer locks.
```sql
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA wal_autocheckpoint=1000;
```

## 2. Hybrid Logical Clocks (HLC)
Combines physical system time with a monotonic logical counter to ensure strict causality across distributed nodes:
hlc_timestamp = max(physical_now, last_time + 1)

## 3. Parameterized Safe Queries
Never interpolate strings into raw SQL statements. Use prepared statements with parameter binding (?1, ?2) to eliminate injection vectors.
";

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_rag_vault_initialization_and_seeding() {
        let temp_dir = std::env::temp_dir().join("mindset_test_vault");
        let _ = fs::remove_dir_all(&temp_dir);

        let vault = RagVault::with_path(temp_dir.clone());
        vault.initialize().expect("Failed to initialize vault");

        let notes = vault.list_notes().expect("Failed to list notes");
        assert_eq!(notes.len(), 4, "Should have 4 seeded notes");
        assert!(notes.iter().any(|n| n.name == "01_Wave_Optics_Notes.md"));
        assert!(notes
            .iter()
            .any(|n| n.name == "02_Electrochemistry_Formulas.md"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_rag_vault_chunk_and_search() {
        let temp_dir = std::env::temp_dir().join("mindset_test_vault_search");
        let _ = fs::remove_dir_all(&temp_dir);

        let vault = RagVault::with_path(temp_dir.clone());
        vault.initialize().expect("Failed to initialize vault");

        let matches = vault
            .search("Young's double slit fringe width beta", 2)
            .expect("Search failed");
        assert!(!matches.is_empty(), "Should find matches for wave optics");
        assert_eq!(matches[0].file_name, "01_Wave_Optics_Notes.md");
        assert!(matches[0].snippet.contains("β = λD / d"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_formula_extraction_and_heading_detection() {
        let sample = "
# Chapter 10: Wave Optics
Interference of coherent light waves produces sustained fringes.
Formula: β = λD / d
Where D is distance and d is slit width.
Equation: E = mc²
Another equation: ΔG° = -n * F * E°_cell
";
        let heading = RagVault::extract_heading_for_chunk(sample);
        assert_eq!(heading, Some("Chapter 10: Wave Optics".to_string()));

        let formulas = RagVault::extract_formula_tokens(sample);
        assert!(!formulas.is_empty(), "Should extract formulas");
        assert!(formulas.iter().any(|f| f.contains("β = λD / d")));
        assert!(formulas.iter().any(|f| f.contains("E = mc²")));
        assert!(formulas.iter().any(|f| f.contains("ΔG°")));
    }

    #[test]
    fn test_ingest_pdf_bytes_and_page_citations() {
        let temp_dir = std::env::temp_dir().join("mindset_test_pdf_vault");
        let _ = fs::remove_dir_all(&temp_dir);

        let vault = RagVault::with_path(temp_dir.clone());
        vault.initialize().expect("Failed to initialize vault");

        // Generate synthetic PDF with 2 pages
        let pages = vec![
            mindset_sys::fs::PdfPage {
                page_number: 1,
                text: "Chapter 10: Wave Optics\nYoung's Double Slit Experiment\nFormula: β = λD / d\nFringe width calculation on screen.".to_string(),
            },
            mindset_sys::fs::PdfPage {
                page_number: 2,
                text: "Chapter 12: Electrochemistry\nNernst Equation\nEquation: E_cell = E°_cell - (0.0591 / n) * log(Q)\nElectrochemical cell potential at non-standard state.".to_string(),
            },
        ];
        let metadata = mindset_sys::fs::PdfMetadata::default();
        let pdf_bytes = mindset_sys::fs::PdfEngine::generate_pdf(&pages, &metadata)
            .expect("Synthetic PDF generation failed");

        let chunks = vault
            .ingest_pdf_bytes("NCERT_Physics_Vol1.pdf", &pdf_bytes)
            .expect("PDF ingestion failed");

        assert_eq!(chunks.len(), 2, "Should create 2 page chunks");
        assert_eq!(chunks[0].file_name, "NCERT_Physics_Vol1.pdf");
        assert_eq!(chunks[0].page_number, 1);
        assert!(chunks[0]
            .formula_tokens
            .iter()
            .any(|f| f.contains("β = λD / d")));

        assert_eq!(chunks[1].page_number, 2);
        assert!(chunks[1]
            .formula_tokens
            .iter()
            .any(|f| f.contains("E_cell")));

        // Search specifically across PDF chunks
        let citations = vault
            .search_pdf_chunks("Young's double slit fringe width", 2)
            .expect("PDF search failed");
        assert!(!citations.is_empty());
        assert_eq!(citations[0].chunk.page_number, 1);
        assert!(citations[0].citation_label.contains("Page 1"));

        // Test context citation generation
        let context = vault.build_context_citations("Nernst equation cell potential", 2);
        assert!(
            context.contains("Page 2"),
            "Context should contain Page 2 anchor"
        );
        assert!(context.contains("NCERT_Physics_Vol1.pdf"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
