//! Local Document Chunker, Term-Vector Cosine Retriever & Study Vault Engine.
//! Connects to ~/.mindset_data/vault/ and local_core.sqlite for grounded AI answers.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFileInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagMatch {
    pub file_name: String,
    pub chunk_index: usize,
    pub snippet: String,
    pub score: f32,
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

    /// Lists all markdown and text study files in the vault.
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
                if ext == "md" || ext == "txt" {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let metadata = entry.metadata().ok();
                    let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);

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

    /// Searches all vault files and returns top-K relevant chunks with cosine similarity scores.
    pub fn search(&self, query: &str, top_k: usize) -> Result<Vec<RagMatch>, String> {
        let notes = self.list_notes()?;
        let mut all_chunks = Vec::new();

        for note in notes {
            if let Ok(content) = self.read_note(&note.name) {
                let chunks = Self::chunk_text(&content, 600);
                for (idx, c) in chunks.into_iter().enumerate() {
                    all_chunks.push((note.name.clone(), idx, c));
                }
            }
        }

        if all_chunks.is_empty() {
            return Ok(Vec::new());
        }

        // Compute TF vector for query
        let q_words = Self::tokenize(query);
        let mut scored_matches = Vec::new();

        for (file_name, chunk_idx, chunk_text) in all_chunks {
            let c_words = Self::tokenize(&chunk_text);
            let score = Self::compute_word_overlap_cosine(&q_words, &c_words);
            if score > 0.05 {
                scored_matches.push(RagMatch {
                    file_name,
                    chunk_index: chunk_idx,
                    snippet: chunk_text,
                    score,
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

    /// Builds a formatted grounded context string for prompt injection.
    pub fn build_context_citations(&self, query: &str, top_k: usize) -> String {
        match self.search(query, top_k) {
            Ok(matches) if !matches.is_empty() => {
                let mut out = String::new();
                for m in matches {
                    out.push_str(&format!(
                        "Source: {} (Chunk #{}, Match: {:.0}%):\n{}\n\n",
                        m.file_name,
                        m.chunk_index + 1,
                        m.score * 100.0,
                        m.snippet.trim()
                    ));
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
}
