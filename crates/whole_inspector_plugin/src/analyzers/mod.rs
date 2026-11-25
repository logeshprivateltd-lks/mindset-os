use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod ai_agent_tts;
pub mod deps_build;
pub mod files_hygiene;
pub mod iso_readiness;
pub mod os_capabilities;
pub mod student_features;
pub mod ui_theme_i18n;

/// Metadata for a single cataloged workspace file.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub rel_path: String,
    pub abs_path: PathBuf,
    pub size_bytes: u64,
    pub line_count: usize,
    pub extension: String,
    pub is_text: bool,
}

/// In-memory indexed snapshot of workspace files for high-speed, read-only analysis.
#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    pub root: PathBuf,
    pub files: Vec<FileInfo>,
}

impl WorkspaceContext {
    /// Recursively indexes the entire workspace tree in read-only mode,
    /// skipping build artifacts (`target`) and version control (`.git`).
    pub fn scan(root: &Path) -> Self {
        let mut files = Vec::new();

        for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Prune target and .git to keep inspection lightning fast and pure
            name != "target" && name != ".git"
        }) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let abs_path = entry.path().to_path_buf();
            let rel = match abs_path.strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => abs_path.to_string_lossy().replace('\\', "/"),
            };

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size_bytes = metadata.len();
            let ext = abs_path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let is_text = matches!(
                ext.as_str(),
                "rs" | "toml"
                    | "json"
                    | "md"
                    | "txt"
                    | "sh"
                    | "hook"
                    | "chroot"
                    | "binary"
                    | "yml"
                    | "yaml"
                    | "desktop"
                    | "conf"
                    | "xml"
                    | "svg"
                    | "wgsl"
                    | "ps1"
            ) || rel.contains(".hook.")
                || rel.contains(".list.");

            let line_count = if is_text && size_bytes < 10 * 1024 * 1024 {
                match fs::read_to_string(&abs_path) {
                    Ok(s) => s.lines().count(),
                    Err(_) => 0,
                }
            } else {
                0
            };

            files.push(FileInfo {
                rel_path: rel,
                abs_path,
                size_bytes,
                line_count,
                extension: ext,
                is_text,
            });
        }

        Self {
            root: root.to_path_buf(),
            files,
        }
    }

    /// Read file content as UTF-8 string (read-only).
    pub fn read_file(&self, rel_path: &str) -> Option<String> {
        let normalized = rel_path.replace('\\', "/");
        let abs = self.root.join(&normalized);
        fs::read_to_string(abs).ok()
    }

    /// Read file content as raw bytes (read-only).
    pub fn read_file_bytes(&self, rel_path: &str) -> Option<Vec<u8>> {
        let normalized = rel_path.replace('\\', "/");
        let abs = self.root.join(&normalized);
        fs::read(abs).ok()
    }

    /// Check if a relative path exists in the indexed workspace.
    pub fn has_file(&self, rel_path: &str) -> bool {
        let normalized = rel_path.replace('\\', "/");
        self.files.iter().any(|f| f.rel_path == normalized)
    }

    /// Find all 1-based line numbers and line content matching a substring in a file.
    pub fn find_lines_matching(&self, rel_path: &str, substring: &str) -> Vec<(usize, String)> {
        let content = match self.read_file(rel_path) {
            Some(c) => c,
            None => return Vec::new(),
        };

        content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(substring))
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    }

    /// Find all 1-based line numbers and line content matching a regex pattern in a file.
    pub fn find_lines_regex(&self, rel_path: &str, re: &Regex) -> Vec<(usize, String)> {
        let content = match self.read_file(rel_path) {
            Some(c) => c,
            None => return Vec::new(),
        };

        content
            .lines()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    }

    /// Find the first occurrence (1-based line number and content) of a substring in a file.
    pub fn find_first_occurrence(
        &self,
        rel_path: &str,
        substring: &str,
    ) -> Option<(usize, String)> {
        let content = self.read_file(rel_path)?;
        for (i, line) in content.lines().enumerate() {
            if line.contains(substring) {
                return Some((i + 1, line.to_string()));
            }
        }
        None
    }

    /// Search across all indexed files matching an optional extension for a substring.
    pub fn search_all_files(
        &self,
        extension: Option<&str>,
        substring: &str,
    ) -> Vec<(String, usize, String)> {
        let mut matches = Vec::new();

        for file in &self.files {
            if let Some(ext) = extension {
                if file.extension != ext {
                    continue;
                }
            } else if !file.is_text {
                continue;
            }

            let content = match self.read_file(&file.rel_path) {
                Some(c) => c,
                None => continue,
            };

            for (i, line) in content.lines().enumerate() {
                if line.contains(substring) {
                    matches.push((file.rel_path.clone(), i + 1, line.to_string()));
                }
            }
        }

        matches
    }

    /// Search across all indexed files matching an optional extension for a regex pattern.
    pub fn search_all_files_regex(
        &self,
        extension: Option<&str>,
        re: &Regex,
    ) -> Vec<(String, usize, String)> {
        let mut matches = Vec::new();

        for file in &self.files {
            if let Some(ext) = extension {
                if file.extension != ext {
                    continue;
                }
            } else if !file.is_text {
                continue;
            }

            let content = match self.read_file(&file.rel_path) {
                Some(c) => c,
                None => continue,
            };

            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    matches.push((file.rel_path.clone(), i + 1, line.to_string()));
                }
            }
        }

        matches
    }
}
