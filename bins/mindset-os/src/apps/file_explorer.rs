//! Native Student File Explorer App (Real std::fs File Browser & Document Reader/Editor)
//!
//! Provides local filesystem navigation across Documents, Study Capsules,
//! Downloads, and Formula Sheets, with:
//! - Hierarchical collapsible directory tree sidebar (expand/collapse `▶` / `▼` navigation)
//! - Multi-page PDF Document Reader with page navigation (`◀ Prev`, `Page X of Y`, `Next ▶`, Jump)
//! - Interactive PPTX Presentation Slide Viewer & Editor with in-place text editing and repacking
//! - In-place Text/Markdown Editor with "Save Changes" persistence
//! - Double-click directory traversal and live vault search.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use iced::mouse;
use iced::widget::{button, column, container, mouse_area, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};

use mindset_sys::fs::document::{
    DocumentContent, DocumentFormat, DocumentHandler, PdfMetadata, PdfPage, PptxMetadata, PptxSlide,
};

use crate::theme::{
    glass_card, glass_elevated, h_space, modern_text_input, primary_button, px_space,
    secondary_button, toggle_button, ThemePalette,
};
use crate::Message;

// ----------------------------------------------------------------------------
// 1. Domain Data Structures
// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub extension: String,
}

#[derive(Clone, Debug)]
pub struct TreeDirectoryNode {
    pub name: String,
    pub path: PathBuf,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<TreeDirectoryNode>,
}

#[derive(Clone)]
pub struct VaultSnapshot {
    pub current_dir: PathBuf,
    pub selected_sidebar: String,
    pub selected_name: Option<String>,
    pub selected_file_path: Option<PathBuf>,
    pub is_selected_dir: bool,

    // Document State
    pub active_doc_format: DocumentFormat,
    pub pdf_page_current: usize,
    pub pdf_page_total: usize,
    pub pdf_page_text: String,
    pub pdf_jump_input: String,

    pub pptx_slide_current: usize,
    pub pptx_slide_total: usize,
    pub pptx_slides_summary: Vec<(usize, String)>,
    pub pptx_edit_title: String,
    pub pptx_edit_body: String,
    pub pptx_notes: Option<String>,

    pub text_buffer: String,
    pub save_status: Option<String>,

    pub preview_metadata: Vec<(String, String)>,
    pub tree_nodes: Vec<TreeDirectoryNode>,
    pub search_query: String,
}

struct FileVaultState {
    current_dir: PathBuf,
    selected_sidebar: String,
    selected_name: Option<String>,
    selected_path: Option<PathBuf>,
    is_selected_dir: bool,
    search_query: String,

    // Document Engine
    active_document: Option<DocumentContent>,
    doc_format: DocumentFormat,

    // PDF State
    pdf_current_page: usize,
    pdf_jump_input: String,

    // PPTX State
    pptx_selected_slide: usize,
    pptx_edit_title: String,
    pptx_edit_body: String,

    // Text Editor State
    text_edit_buffer: String,

    // Save Feedback
    save_status: Option<String>,

    // Tree State
    expanded_tree_paths: HashSet<PathBuf>,

    preview_metadata: Vec<(String, String)>,
    last_click: Option<(String, Instant)>,
}

impl FileVaultState {
    fn new() -> Self {
        let root = default_vault_dir("Study Capsules");
        ensure_starter_files(&root);

        let mut expanded = HashSet::new();
        expanded.insert(root.clone());

        let mut state = Self {
            current_dir: root.clone(),
            selected_sidebar: "Study Capsules".to_string(),
            selected_name: None,
            selected_path: None,
            is_selected_dir: false,
            search_query: String::new(),
            active_document: None,
            doc_format: DocumentFormat::Unknown,
            pdf_current_page: 1,
            pdf_jump_input: "1".to_string(),
            pptx_selected_slide: 1,
            pptx_edit_title: String::new(),
            pptx_edit_body: String::new(),
            text_edit_buffer: String::new(),
            save_status: None,
            expanded_tree_paths: expanded,
            preview_metadata: Vec::new(),
            last_click: None,
        };

        let readme_path = root.join("README_STUDY_VAULT.txt");
        if readme_path.exists() {
            state.select_file_internal("README_STUDY_VAULT.txt", &readme_path);
        }

        state
    }

    fn select_file_internal(&mut self, name: &str, path: &Path) {
        self.selected_name = Some(name.to_string());
        self.selected_path = Some(path.to_path_buf());
        self.is_selected_dir = false;
        self.save_status = None;

        let meta = fs::metadata(path).ok();
        let size_bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified_str = meta
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let duration = t.elapsed().unwrap_or(Duration::from_secs(0));
                if duration.as_secs() < 3600 {
                    format!("{}m ago", duration.as_secs() / 60)
                } else {
                    format!("{}h ago", duration.as_secs() / 3600)
                }
            })
            .unwrap_or_else(|| "Recent".to_string());

        let ext = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        // Load document content via pure-Rust DocumentHandler
        match DocumentHandler::read(path) {
            Ok(content) => {
                match &content {
                    DocumentContent::Pdf { pages, metadata } => {
                        self.doc_format = DocumentFormat::Pdf;
                        self.pdf_current_page = 1;
                        self.pdf_jump_input = "1".to_string();

                        self.preview_metadata = vec![
                            ("File Name".into(), name.to_string()),
                            ("Type".into(), "PDF Academic Document".into()),
                            (
                                "Title".into(),
                                metadata.title.clone().unwrap_or_else(|| "Untitled".into()),
                            ),
                            (
                                "Author".into(),
                                metadata
                                    .author
                                    .clone()
                                    .unwrap_or_else(|| "MindSet Scholar".into()),
                            ),
                            ("Pages".into(), format!("{} pages", pages.len())),
                            (
                                "Size".into(),
                                if size_bytes < 1024 {
                                    format!("{} B", size_bytes)
                                } else {
                                    format!("{:.1} KB", size_bytes as f32 / 1024.0)
                                },
                            ),
                            ("Modified".into(), modified_str),
                        ];
                    }
                    DocumentContent::Pptx { slides, metadata } => {
                        self.doc_format = DocumentFormat::Pptx;
                        self.pptx_selected_slide = 1;
                        if let Some(first) = slides.first() {
                            self.pptx_edit_title = first.title.clone().unwrap_or_default();
                            self.pptx_edit_body = first.body_blocks.join("\n\n");
                        } else {
                            self.pptx_edit_title.clear();
                            self.pptx_edit_body.clear();
                        }

                        self.preview_metadata = vec![
                            ("File Name".into(), name.to_string()),
                            ("Type".into(), "PowerPoint Presentation (OpenXML)".into()),
                            (
                                "Title".into(),
                                metadata
                                    .title
                                    .clone()
                                    .unwrap_or_else(|| "Lecture Presentation".into()),
                            ),
                            ("Slides".into(), format!("{} slides", slides.len())),
                            (
                                "Size".into(),
                                if size_bytes < 1024 {
                                    format!("{} B", size_bytes)
                                } else {
                                    format!("{:.1} KB", size_bytes as f32 / 1024.0)
                                },
                            ),
                            ("Modified".into(), modified_str),
                        ];
                    }
                    DocumentContent::Text(text) => {
                        self.doc_format = if ext == "md" {
                            DocumentFormat::Markdown
                        } else {
                            DocumentFormat::Text
                        };
                        self.text_edit_buffer = text.clone();

                        self.preview_metadata = vec![
                            ("File Name".into(), name.to_string()),
                            (
                                "Type".into(),
                                if ext == "md" {
                                    "Markdown Study Notes".into()
                                } else {
                                    "Plain Text Document".into()
                                },
                            ),
                            ("Lines".into(), format!("{} lines", text.lines().count())),
                            ("Chars".into(), format!("{} chars", text.len())),
                            (
                                "Size".into(),
                                if size_bytes < 1024 {
                                    format!("{} B", size_bytes)
                                } else {
                                    format!("{:.1} KB", size_bytes as f32 / 1024.0)
                                },
                            ),
                            ("Modified".into(), modified_str),
                        ];
                    }
                }
                self.active_document = Some(content);
            }
            Err(e) => {
                self.doc_format = DocumentFormat::Unknown;
                self.active_document = None;
                self.preview_metadata = vec![
                    ("File Name".into(), name.to_string()),
                    ("Type".into(), "Binary Asset".into()),
                    ("Error".into(), format!("{}", e)),
                ];
            }
        }
    }

    fn select_dir_internal(&mut self, name: &str, path: &Path) {
        self.selected_name = Some(name.to_string());
        self.selected_path = Some(path.to_path_buf());
        self.is_selected_dir = true;
        self.active_document = None;
        self.doc_format = DocumentFormat::Unknown;
        self.save_status = None;

        let count = fs::read_dir(path).map(|r| r.flatten().count()).unwrap_or(0);

        self.preview_metadata = vec![
            ("Folder Name".into(), name.to_string()),
            ("Type".into(), "Local Vault Folder".into()),
            ("Items Inside".into(), format!("{} entries", count)),
            ("Path".into(), path.to_string_lossy().to_string()),
        ];
    }
}

static VAULT_STATE: Mutex<Option<FileVaultState>> = Mutex::new(None);

fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut FileVaultState) -> R,
{
    let mut lock = VAULT_STATE.lock().unwrap_or_else(|e| e.into_inner());
    let state = lock.get_or_insert_with(FileVaultState::new);
    f(state)
}

// ----------------------------------------------------------------------------
// 2. Public API called by Message Dispatcher
// ----------------------------------------------------------------------------

pub fn open_directory(target_dir: &Path) {
    with_state(|state| {
        let _ = fs::create_dir_all(target_dir);
        ensure_starter_files(target_dir);
        state.current_dir = target_dir.to_path_buf();
        state.expanded_tree_paths.insert(target_dir.to_path_buf());
        state.selected_sidebar = "Linux Study Vault".to_string();
        state.selected_name = None;
        state.selected_path = None;
        state.active_document = None;
        state.save_status = None;
        state.preview_metadata.clear();
    });
}

pub fn select_sidebar(folder_name: &str) {
    with_state(|state| {
        state.selected_sidebar = folder_name.to_string();
        let target = default_vault_dir(folder_name);
        ensure_starter_files(&target);
        state.current_dir = target;
        state.selected_name = None;
        state.selected_path = None;
        state.active_document = None;
        state.save_status = None;
        state.preview_metadata.clear();
    });
}

pub fn entry_clicked(name: &str, is_dir: bool) {
    with_state(|state| {
        let now = Instant::now();
        let is_double_click = if let Some((last_name, instant)) = &state.last_click {
            last_name == name && now.duration_since(*instant) < Duration::from_millis(500)
        } else {
            false
        };

        state.last_click = Some((name.to_string(), now));
        let entry_path = state.current_dir.join(name);

        if is_dir {
            if is_double_click {
                ensure_starter_files(&entry_path);
                state.current_dir = entry_path;
                state.selected_name = None;
                state.selected_path = None;
                state.active_document = None;
                state.preview_metadata.clear();
            } else {
                state.select_dir_internal(name, &entry_path);
            }
        } else {
            state.select_file_internal(name, &entry_path);
        }
    });
}

pub fn open_folder(name_or_path: &str) {
    with_state(|state| {
        let raw = Path::new(name_or_path);
        let candidate = if raw.is_absolute() {
            raw.to_path_buf()
        } else {
            state.current_dir.join(raw)
        };
        let vault_root = default_vault_dir(&state.selected_sidebar);
        if let (Ok(canon_cand), Ok(canon_root)) =
            (fs::canonicalize(&candidate), fs::canonicalize(&vault_root))
        {
            if canon_cand.starts_with(&canon_root) && canon_cand.is_dir() {
                ensure_starter_files(&canon_cand);
                state.current_dir = canon_cand;
                state.selected_name = None;
                state.selected_path = None;
                state.active_document = None;
                state.preview_metadata.clear();
            }
        }
    });
}

pub fn navigate_up() {
    with_state(|state| {
        let vault_root = default_vault_dir(&state.selected_sidebar);
        if let (Ok(canon_curr), Ok(canon_root)) = (
            fs::canonicalize(&state.current_dir),
            fs::canonicalize(&vault_root),
        ) {
            if canon_curr != canon_root && canon_curr.starts_with(&canon_root) {
                if let Some(parent) = canon_curr.parent() {
                    if parent.starts_with(&canon_root) {
                        state.current_dir = parent.to_path_buf();
                        state.selected_name = None;
                        state.selected_path = None;
                        state.active_document = None;
                        state.preview_metadata.clear();
                    }
                }
            }
        }
    });
}

pub fn set_search(query: &str) {
    with_state(|state| {
        state.search_query = query.to_string();
    });
}

pub fn toggle_tree_node(path_str: &str) {
    with_state(|state| {
        let p = PathBuf::from(path_str);
        if state.expanded_tree_paths.contains(&p) {
            state.expanded_tree_paths.remove(&p);
        } else {
            state.expanded_tree_paths.insert(p);
        }
    });
}

pub fn pdf_prev_page() {
    with_state(|state| {
        if state.pdf_current_page > 1 {
            state.pdf_current_page -= 1;
            state.pdf_jump_input = state.pdf_current_page.to_string();
        }
    });
}

pub fn pdf_next_page() {
    with_state(|state| {
        if let Some(DocumentContent::Pdf { pages, .. }) = &state.active_document {
            if state.pdf_current_page < pages.len() {
                state.pdf_current_page += 1;
                state.pdf_jump_input = state.pdf_current_page.to_string();
            }
        }
    });
}

pub fn pdf_jump_page(val: &str) {
    with_state(|state| {
        state.pdf_jump_input = val.to_string();
        if let Ok(num) = val.trim().parse::<usize>() {
            if let Some(DocumentContent::Pdf { pages, .. }) = &state.active_document {
                if num >= 1 && num <= pages.len() {
                    state.pdf_current_page = num;
                }
            }
        }
    });
}

pub fn pptx_select_slide(idx: usize) {
    with_state(|state| {
        if let Some(DocumentContent::Pptx { slides, .. }) = &state.active_document {
            if let Some(slide) = slides.iter().find(|s| s.slide_number == idx) {
                state.pptx_selected_slide = idx;
                state.pptx_edit_title = slide.title.clone().unwrap_or_default();
                state.pptx_edit_body = slide.body_blocks.join("\n\n");
                state.save_status = None;
            }
        }
    });
}

pub fn pptx_title_changed(val: &str) {
    with_state(|state| {
        state.pptx_edit_title = val.to_string();
    });
}

pub fn pptx_body_changed(val: &str) {
    with_state(|state| {
        state.pptx_edit_body = val.to_string();
    });
}

pub fn pptx_save_slide() {
    with_state(|state| {
        let curr_idx = state.pptx_selected_slide;
        let new_title = state.pptx_edit_title.trim().to_string();
        let new_body_lines: Vec<String> = state
            .pptx_edit_body
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if let Some(DocumentContent::Pptx {
            ref mut slides,
            ref metadata,
        }) = &mut state.active_document
        {
            if let Some(s) = slides.iter_mut().find(|s| s.slide_number == curr_idx) {
                s.title = if new_title.is_empty() {
                    None
                } else {
                    Some(new_title)
                };
                s.body_blocks = new_body_lines;
            }

            if let Some(path) = &state.selected_path {
                let updated_doc = DocumentContent::Pptx {
                    slides: slides.clone(),
                    metadata: metadata.clone(),
                };
                match DocumentHandler::save_update(path, &updated_doc) {
                    Ok(()) => {
                        state.save_status =
                            Some("✓ Slide updated & saved to .pptx archive".to_string());
                    }
                    Err(e) => {
                        state.save_status = Some(format!("Error saving slide: {}", e));
                    }
                }
            }
        }
    });
}

pub fn text_edit_changed(val: &str) {
    with_state(|state| {
        state.text_edit_buffer = val.to_string();
    });
}

pub fn text_save() {
    with_state(|state| {
        if let Some(path) = &state.selected_path {
            let doc = DocumentContent::Text(state.text_edit_buffer.clone());
            match DocumentHandler::save_update(path, &doc) {
                Ok(()) => {
                    state.active_document = Some(doc);
                    state.save_status = Some("✓ Document changes saved to disk".to_string());
                }
                Err(e) => {
                    state.save_status = Some(format!("Error saving document: {}", e));
                }
            }
        }
    });
}

// ----------------------------------------------------------------------------
// 3. Tree View Builder
// ----------------------------------------------------------------------------

fn build_directory_tree(
    root: &Path,
    expanded_paths: &HashSet<PathBuf>,
    depth: usize,
) -> Vec<TreeDirectoryNode> {
    if depth > 3 {
        return Vec::new();
    }

    let mut nodes = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        let mut dirs = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if !name.starts_with('.') {
                    dirs.push((name, path));
                }
            }
        }
        dirs.sort_by_key(|a| a.0.to_lowercase());

        for (name, path) in dirs {
            let is_expanded = expanded_paths.contains(&path);
            let children = if is_expanded {
                build_directory_tree(&path, expanded_paths, depth + 1)
            } else {
                Vec::new()
            };
            nodes.push(TreeDirectoryNode {
                name,
                path,
                is_expanded,
                depth,
                children,
            });
        }
    }
    nodes
}

// ----------------------------------------------------------------------------
// 4. View Function
// ----------------------------------------------------------------------------

pub fn view<'a>(palette: ThemePalette) -> Element<'a, Message> {
    let snapshot = with_state(|state| {
        let tree_root = default_vault_dir(&state.selected_sidebar);
        let tree_nodes = build_directory_tree(&tree_root, &state.expanded_tree_paths, 0);

        let (pdf_page_total, pdf_page_text) = match &state.active_document {
            Some(DocumentContent::Pdf { pages, .. }) => {
                let total = pages.len();
                let text = pages
                    .iter()
                    .find(|p| p.page_number == state.pdf_current_page)
                    .map(|p| p.text.clone())
                    .unwrap_or_else(|| "No text on this page.".to_string());
                (total, text)
            }
            _ => (0, String::new()),
        };

        let (pptx_slide_total, pptx_slides_summary, pptx_notes) = match &state.active_document {
            Some(DocumentContent::Pptx { slides, .. }) => {
                let total = slides.len();
                let summary: Vec<(usize, String)> = slides
                    .iter()
                    .map(|s| {
                        (
                            s.slide_number,
                            s.title
                                .clone()
                                .unwrap_or_else(|| format!("Slide {}", s.slide_number)),
                        )
                    })
                    .collect();
                let notes = slides
                    .iter()
                    .find(|s| s.slide_number == state.pptx_selected_slide)
                    .and_then(|s| s.notes.clone());
                (total, summary, notes)
            }
            _ => (0, Vec::new(), None),
        };

        VaultSnapshot {
            current_dir: state.current_dir.clone(),
            selected_sidebar: state.selected_sidebar.clone(),
            selected_name: state.selected_name.clone(),
            selected_file_path: state.selected_path.clone(),
            is_selected_dir: state.is_selected_dir,
            active_doc_format: state.doc_format,
            pdf_page_current: state.pdf_current_page,
            pdf_page_total,
            pdf_page_text,
            pdf_jump_input: state.pdf_jump_input.clone(),
            pptx_slide_current: state.pptx_selected_slide,
            pptx_slide_total,
            pptx_slides_summary,
            pptx_edit_title: state.pptx_edit_title.clone(),
            pptx_edit_body: state.pptx_edit_body.clone(),
            pptx_notes,
            text_buffer: state.text_edit_buffer.clone(),
            save_status: state.save_status.clone(),
            preview_metadata: state.preview_metadata.clone(),
            tree_nodes,
            search_query: state.search_query.clone(),
        }
    });

    let search_str = snapshot.search_query.clone();

    let current_dir = snapshot.current_dir;

    // Read current directory entries
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(&current_dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let is_dir = path.is_dir();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let size = if is_dir {
                0
            } else {
                entry.metadata().map(|m| m.len()).unwrap_or(0)
            };

            if search_str.is_empty() || name.to_lowercase().contains(&search_str.to_lowercase()) {
                entries.push(FileEntry {
                    name,
                    is_dir,
                    size_bytes: size,
                    extension: ext,
                });
            }
        }
    }

    entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    // ------------------------------------------------------------------------
    // 1. Top Navigation & Search Bar
    // ------------------------------------------------------------------------
    let vault_root = default_vault_dir(&snapshot.selected_sidebar);
    let can_go_up = match (
        fs::canonicalize(&current_dir),
        fs::canonicalize(&vault_root),
    ) {
        (Ok(cc), Ok(cr)) => cc != cr && cc.starts_with(&cr),
        _ => current_dir != vault_root && current_dir.parent().is_some(),
    };
    let mut up_btn = button(text("⬆ Up").size(11))
        .style(secondary_button(palette))
        .padding([4, 8]);
    if can_go_up {
        up_btn = up_btn.on_press(Message::VaultNavigateUp);
    }

    let breadcrumb_text = text(format!("📂 {}", current_dir.to_string_lossy()))
        .size(11)
        .color(palette.text_primary);

    let search_bar = text_input("Search files in vault...", &search_str)
        .on_input(Message::VaultSearchChanged)
        .style(modern_text_input(palette))
        .padding(6)
        .width(220);

    let top_nav = row![up_btn, breadcrumb_text, h_space(), search_bar]
        .spacing(10)
        .align_y(Alignment::Center)
        .padding([8, 12]);

    let top_nav_bar = container(top_nav)
        .width(Length::Fill)
        .style(move |_t| glass_elevated(palette));

    // ------------------------------------------------------------------------
    // 2. Left Sidebar: Quick Folders + Hierarchical Directory Tree
    // ------------------------------------------------------------------------
    let sidebar_folders = [
        ("📁 Documents", "Documents"),
        ("📑 Study Capsules", "Study Capsules"),
        ("📥 Downloads", "Downloads"),
        ("📝 Formula Sheets", "Formula Sheets"),
    ];

    let mut sidebar_col = column![
        text("STUDY VAULT").size(10).color(palette.text_muted),
        px_space(0.0, 2.0),
    ]
    .spacing(4);

    for (label, key) in sidebar_folders {
        let is_selected = snapshot.selected_sidebar == key;
        let folder_btn = button(
            row![
                text(label).size(11).color(if is_selected {
                    palette.accent_text
                } else {
                    palette.text_primary
                }),
                h_space(),
                if is_selected {
                    text("●").size(8).color(palette.accent)
                } else {
                    text("").size(8)
                },
            ]
            .align_y(Alignment::Center),
        )
        .on_press(Message::VaultSelectSidebar(key.to_string()))
        .style(toggle_button(palette, is_selected))
        .padding([5, 8])
        .width(Length::Fill);

        sidebar_col =
            sidebar_col.push(mouse_area(folder_btn).interaction(mouse::Interaction::Pointer));
    }

    // Expandable Hierarchical Directory Tree Section
    sidebar_col = sidebar_col.push(px_space(0.0, 8.0));
    sidebar_col = sidebar_col.push(text("DIRECTORY TREE").size(9).color(palette.text_muted));

    let tree_col = column![].spacing(2);
    fn render_tree_nodes<'a>(
        nodes: &[TreeDirectoryNode],
        mut tree_col: iced::widget::Column<'a, Message>,
        palette: ThemePalette,
        current_dir_str: &str,
    ) -> iced::widget::Column<'a, Message> {
        for node in nodes {
            let path_str = node.path.to_string_lossy().to_string();
            let is_active = path_str == current_dir_str;
            let arrow_icon = if node.is_expanded { "▼" } else { "▶" };

            let toggle_btn = button(text(arrow_icon).size(9).color(palette.accent))
                .on_press(Message::VaultToggleTreeNode(path_str.clone()))
                .padding([2, 4])
                .style(secondary_button(palette));

            let dir_btn = button(
                row![
                    text("📁").size(10),
                    text(node.name.clone()).size(10).color(if is_active {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .on_press(Message::VaultOpenFolder(path_str))
            .padding([2, 6])
            .style(toggle_button(palette, is_active))
            .width(Length::Fill);

            let indent_space = px_space((node.depth * 12) as f32, 0.0);
            let tree_row = row![indent_space, toggle_btn, dir_btn]
                .spacing(4)
                .align_y(Alignment::Center);

            tree_col = tree_col.push(tree_row);

            if node.is_expanded && !node.children.is_empty() {
                tree_col = render_tree_nodes(&node.children, tree_col, palette, current_dir_str);
            }
        }
        tree_col
    }

    let cur_str = current_dir.to_string_lossy().to_string();
    let tree_col = render_tree_nodes(&snapshot.tree_nodes, tree_col, palette, &cur_str);

    sidebar_col = sidebar_col.push(scrollable(tree_col).height(Length::Fill));

    let sidebar = container(sidebar_col.width(220))
        .padding(10)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette));

    // ------------------------------------------------------------------------
    // 3. Center File Grid / List
    // ------------------------------------------------------------------------
    let mut file_rows = column![].spacing(4);
    if entries.is_empty() {
        file_rows = file_rows.push(
            container(
                text("No files in this folder matching search.")
                    .size(11)
                    .color(palette.text_muted),
            )
            .padding(20),
        );
    } else {
        for entry in entries {
            let is_selected = snapshot.selected_name.as_deref() == Some(&entry.name);
            let icon = if entry.is_dir {
                "📁"
            } else {
                match entry.extension.as_str() {
                    "pdf" => "📕",
                    "pptx" | "ppt" => "📊",
                    "notes" | "txt" | "md" => "📝",
                    "capsule" => "📑",
                    "rs" => "🦀",
                    _ => "📄",
                }
            };

            let size_str = if entry.is_dir {
                "Folder".to_string()
            } else {
                format!("{:.1} KB", entry.size_bytes as f32 / 1024.0)
            };

            let row_content = row![
                text(icon).size(14),
                text(entry.name.clone()).size(11).color(if is_selected {
                    palette.accent_text
                } else {
                    palette.text_primary
                }),
                h_space(),
                text(size_str).size(10).color(if is_selected {
                    palette.accent_text
                } else {
                    palette.text_muted
                }),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            let file_btn = button(row_content)
                .on_press(Message::VaultEntryClicked(entry.name, entry.is_dir))
                .style(toggle_button(palette, is_selected))
                .padding([6, 10])
                .width(Length::Fill);

            let file_row = mouse_area(file_btn).interaction(mouse::Interaction::Pointer);
            file_rows = file_rows.push(file_row);
        }
    }

    let file_list_box = container(scrollable(file_rows).height(Length::Fill))
        .padding(10)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(move |_t| glass_card(palette));

    // ------------------------------------------------------------------------
    // 4. Right Preview Pane: Document Reader & In-Place Editor
    // ------------------------------------------------------------------------
    let preview_inner: Element<'a, Message> = if let Some(selected_name) = snapshot.selected_name {
        if snapshot.is_selected_dir {
            // Folder Inspection View
            let count_str = snapshot
                .preview_metadata
                .iter()
                .find(|(k, _)| k == "Items Inside")
                .map(|(_, v)| v.as_str())
                .unwrap_or("0 entries");

            let open_btn = button(text("📂 Open Folder").size(11))
                .on_press(Message::VaultOpenFolder(selected_name.clone()))
                .style(primary_button(palette))
                .padding([6, 14]);

            column![
                text(format!("Folder: {}", selected_name))
                    .size(13)
                    .color(palette.text_primary),
                text(format!("Contains {}.", count_str))
                    .size(11)
                    .color(palette.text_secondary),
                px_space(0.0, 8.0),
                open_btn,
                h_space(),
                text("Double-click in list or click 'Open Folder' to enter.")
                    .size(9)
                    .color(palette.text_muted),
            ]
            .spacing(6)
            .height(Length::Fill)
            .into()
        } else {
            // Document View by Format
            match snapshot.active_doc_format {
                DocumentFormat::Pdf => {
                    // --- Multi-Page PDF Reader View ---
                    let prev_btn = if snapshot.pdf_page_current > 1 {
                        button(text("◀ Prev").size(10))
                            .on_press(Message::VaultPdfPrevPage)
                            .style(secondary_button(palette))
                            .padding([4, 8])
                    } else {
                        button(text("◀ Prev").size(10))
                            .style(secondary_button(palette))
                            .padding([4, 8])
                    };

                    let next_btn = if snapshot.pdf_page_current < snapshot.pdf_page_total {
                        button(text("Next ▶").size(10))
                            .on_press(Message::VaultPdfNextPage)
                            .style(secondary_button(palette))
                            .padding([4, 8])
                    } else {
                        button(text("Next ▶").size(10))
                            .style(secondary_button(palette))
                            .padding([4, 8])
                    };

                    let page_label = text(format!(
                        "Page {} of {}",
                        snapshot.pdf_page_current, snapshot.pdf_page_total
                    ))
                    .size(11)
                    .color(palette.text_primary);

                    let jump_input = text_input("Page #", &snapshot.pdf_jump_input)
                        .on_input(Message::VaultPdfJumpPage)
                        .style(modern_text_input(palette))
                        .padding(4)
                        .width(60);

                    let nav_toolbar = row![
                        prev_btn,
                        page_label,
                        next_btn,
                        h_space(),
                        text("Go to:").size(10).color(palette.text_muted),
                        jump_input,
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);

                    let page_view_box = container(
                        scrollable(
                            column![text(snapshot.pdf_page_text)
                                .size(11)
                                .color(palette.text_primary)]
                            .spacing(6),
                        )
                        .height(Length::Fill),
                    )
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_t| glass_elevated(palette));

                    column![
                        row![
                            text("📕 PDF Reader").size(12).color(palette.accent),
                            h_space(),
                            text(selected_name).size(11).color(palette.text_primary),
                        ]
                        .align_y(Alignment::Center),
                        nav_toolbar,
                        page_view_box,
                    ]
                    .spacing(8)
                    .height(Length::Fill)
                    .into()
                }

                DocumentFormat::Pptx | DocumentFormat::Ppt => {
                    // --- PPTX Slide Viewer & Editor View ---
                    let mut slide_pills = row![].spacing(4);
                    for (slide_num, title) in &snapshot.pptx_slides_summary {
                        let is_active = *slide_num == snapshot.pptx_slide_current;
                        let pill_btn =
                            button(text(format!("{}. {}", slide_num, title)).size(9).color(
                                if is_active {
                                    palette.accent_text
                                } else {
                                    palette.text_primary
                                },
                            ))
                            .on_press(Message::VaultPptxSelectSlide(*slide_num))
                            .style(toggle_button(palette, is_active))
                            .padding([3, 6]);
                        slide_pills = slide_pills.push(pill_btn);
                    }

                    let slide_carousel = scrollable(slide_pills).width(Length::Fill);

                    let title_input = text_input("Slide Title...", &snapshot.pptx_edit_title)
                        .on_input(Message::VaultPptxTitleChanged)
                        .style(modern_text_input(palette))
                        .padding(6);

                    let body_input = text_input("Slide Body Text...", &snapshot.pptx_edit_body)
                        .on_input(Message::VaultPptxBodyChanged)
                        .style(modern_text_input(palette))
                        .padding(6);

                    let save_btn = button(text("💾 Save & Update Slide").size(11))
                        .on_press(Message::VaultPptxSaveSlide)
                        .style(primary_button(palette))
                        .padding([5, 12]);

                    let status_elem: Element<'a, Message> =
                        if let Some(msg) = snapshot.save_status.clone() {
                            text(msg).size(10).color(palette.success).into()
                        } else {
                            text("").size(10).into()
                        };

                    let editor_col = column![
                        text("SLIDE TITLE").size(9).color(palette.text_muted),
                        title_input,
                        px_space(0.0, 4.0),
                        text("SLIDE CONTENT").size(9).color(palette.text_muted),
                        body_input,
                        px_space(0.0, 6.0),
                        row![save_btn, h_space(), status_elem].align_y(Alignment::Center),
                    ]
                    .spacing(4);

                    let content_box = container(scrollable(editor_col).height(Length::Fill))
                        .padding(10)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(move |_t| glass_elevated(palette));

                    column![
                        row![
                            text("📊 Presentation Editor")
                                .size(12)
                                .color(palette.accent),
                            h_space(),
                            text(selected_name).size(11).color(palette.text_primary),
                        ]
                        .align_y(Alignment::Center),
                        slide_carousel,
                        content_box,
                    ]
                    .spacing(8)
                    .height(Length::Fill)
                    .into()
                }

                DocumentFormat::Markdown | DocumentFormat::Text => {
                    // --- Text / Markdown In-Place Editor ---
                    let edit_input = text_input("Document Text Content...", &snapshot.text_buffer)
                        .on_input(Message::VaultTextEditChanged)
                        .style(modern_text_input(palette))
                        .padding(8);

                    let save_btn = button(text("💾 Save Changes").size(11))
                        .on_press(Message::VaultTextSave)
                        .style(primary_button(palette))
                        .padding([5, 12]);

                    let status_elem: Element<'a, Message> =
                        if let Some(msg) = snapshot.save_status.clone() {
                            text(msg).size(10).color(palette.success).into()
                        } else {
                            text("").size(10).into()
                        };

                    let editor_box = container(
                        column![
                            edit_input,
                            px_space(0.0, 6.0),
                            row![save_btn, h_space(), status_elem].align_y(Alignment::Center),
                        ]
                        .spacing(6),
                    )
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_t| glass_elevated(palette));

                    column![
                        row![
                            text("📝 Document Editor").size(12).color(palette.accent),
                            h_space(),
                            text(selected_name).size(11).color(palette.text_primary),
                        ]
                        .align_y(Alignment::Center),
                        editor_box,
                    ]
                    .spacing(8)
                    .height(Length::Fill)
                    .into()
                }

                DocumentFormat::Unknown => {
                    // Fallback Binary Metadata View
                    let mut meta_col = column![
                        text(selected_name).size(13).color(palette.text_primary),
                        px_space(0.0, 4.0),
                    ]
                    .spacing(2);

                    for (k, v) in snapshot.preview_metadata {
                        meta_col = meta_col.push(
                            row![
                                text(k).size(9).color(palette.text_muted).width(70),
                                text(v).size(9).color(palette.text_secondary),
                            ]
                            .spacing(4),
                        );
                    }

                    container(meta_col)
                        .padding(10)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(move |_t| glass_elevated(palette))
                        .into()
                }
            }
        }
    } else {
        column![
            text("Document Viewer & Editor").size(12).color(palette.text_primary),
            text("Click any PDF, PPTX, or Markdown document to read, edit, and update.")
                .size(10)
                .color(palette.text_secondary),
            h_space(),
            text("• Pure-Rust PDF & OpenXML Engine\n• In-place slide editing & repacking\n• 100% offline Study Vault")
                .size(9)
                .color(palette.text_muted),
        ]
        .spacing(8)
        .into()
    };

    let preview_pane = container(preview_inner)
        .padding(12)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(move |_t| glass_card(palette));

    let body_layout = row![sidebar, file_list_box, preview_pane]
        .spacing(12)
        .height(Length::Fill);

    let main_layout = column![top_nav_bar, body_layout]
        .spacing(10)
        .padding(12)
        .width(Length::Fill)
        .height(Length::Fill);

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 5. Helpers: Seed default starter study files & determine directories
// ----------------------------------------------------------------------------

fn default_vault_dir(name: &str) -> PathBuf {
    let vault_root = crate::ai::maindy_system_bridge::vault_dir();
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    match name {
        "Documents" => dirs::document_dir().unwrap_or_else(|| home.join("Documents")),
        "Downloads" => dirs::download_dir().unwrap_or_else(|| home.join("Downloads")),
        "Formula Sheets" => vault_root.join("Formula Sheets"),
        _ => vault_root,
    }
}

fn ensure_starter_files(dir: &Path) {
    if let Ok(()) = fs::create_dir_all(dir) {
        let dir_str = dir.to_string_lossy();
        if dir_str.contains("Formula Sheets") {
            let formula_file = dir.join("Physics_HighYield_Formulas.txt");
            if !formula_file.exists() {
                let _ = fs::write(
                    &formula_file,
                    "MINDSET OS — HIGH-YIELD PHYSICS FORMULA SHEET\n\n\
                    1. Gauss's Law: ∮ E · dA = Q_enclosed / ε₀\n\
                    2. Point Charge Field: E = q / (4πε₀ r²)\n\
                    3. Snell's Law: sin(i) / sin(r) = v₁ / v₂ = n₂ / n₁\n\
                    4. Nernst Equation: E = E° - (0.0591 / n) log₁₀(Q)\n\
                    5. Photoelectric Effect: hf = W₀ + KE_max\n",
                );
            }
        } else if dir_str.contains("Study Capsules") || dir_str.contains("vault") {
            // Seed nested subject directory hierarchy for testing directory tree
            let physics_dir = dir.join("Physics");
            let _ = fs::create_dir_all(&physics_dir);
            let electro_dir = physics_dir.join("Electrostatics");
            let _ = fs::create_dir_all(&electro_dir);

            let chem_dir = dir.join("Chemistry");
            let _ = fs::create_dir_all(&chem_dir);

            // Seed starter PDF
            let pdf_path = dir.join("Class12_Physics_Exam_Sample.pdf");
            if !pdf_path.exists() {
                let pages = vec![
                    PdfPage {
                        page_number: 1,
                        text: "MINDSET OS — CLASS 12 PHYSICS EXAMINATION\n\
                        Section A: Electrostatics & Potential\n\
                        Q1: Define Electric Flux through an enclosed Gaussian surface.\n\
                        Formula: Flux = Q_enc / epsilon_0\n\
                        Standard Marks: 5 / 5 Target".to_string(),
                    },
                    PdfPage {
                        page_number: 2,
                        text: "Section B: Current Electricity & Magnetism\n\
                        Q2: State Ampere's Circuital Law and derive the magnetic field inside a solenoid.\n\
                        Formula: B = mu_0 * n * I\n\
                        Marking Scheme: Complete derivation with clear diagrams yields full score.".to_string(),
                    },
                ];
                let meta = PdfMetadata {
                    title: Some("Class 12 Physics Exam Sample".to_string()),
                    author: Some("State Board Faculty".to_string()),
                    page_count: 2,
                    ..Default::default()
                };
                let content = DocumentContent::Pdf {
                    pages,
                    metadata: meta,
                };
                let _ = DocumentHandler::save_update(&pdf_path, &content);
            }

            // Seed starter PPTX
            let pptx_path = dir.join("Electromagnetism_Mastery_Presentation.pptx");
            if !pptx_path.exists() {
                let slides = vec![
                    PptxSlide {
                        slide_number: 1,
                        title: Some("Electromagnetism Lecture Series".to_string()),
                        body_blocks: vec![
                            "Chapter 4: Moving Charges & Magnetism".to_string(),
                            "Class 12 High-Yield Exam Preparation".to_string(),
                        ],
                        notes: Some("Welcome the students and introduce Lorentz force".to_string()),
                        shape_texts: vec!["National Rank Target 500".to_string()],
                    },
                    PptxSlide {
                        slide_number: 2,
                        title: Some("Biot-Savart Law".to_string()),
                        body_blocks: vec![
                            "dB = (mu_0 / 4pi) * (I dl x r) / r^3".to_string(),
                            "Key Application: Field along axis of circular loop".to_string(),
                        ],
                        notes: None,
                        shape_texts: Vec::new(),
                    },
                ];
                let meta = PptxMetadata {
                    title: Some("Electromagnetism Mastery".to_string()),
                    author: Some("MindSet OS Faculty".to_string()),
                    slide_count: 2,
                    slide_width: Some(12192000),
                    slide_height: Some(6858000),
                };
                let content = DocumentContent::Pptx {
                    slides,
                    metadata: meta,
                };
                let _ = DocumentHandler::save_update(&pptx_path, &content);
            }

            let starter = dir.join("README_STUDY_VAULT.txt");
            if !starter.exists() {
                let _ = fs::write(
                    &starter,
                    "MindSet OS Study Vault\n\n\
                    Store your subject notes, exported .capsule files, and revision summaries here.\n\
                    All files in this folder are automatically accessible offline and cross-platform.\n\n\
                    • 100% Local Pure-Rust Document Engine\n\
                    • Multi-Page PDF Reader & Interactive PPTX Editor\n\
                    • Zero Cloud Leakage\n",
                );
            }
            let cap_starter = dir.join("Electrochemistry_Daniell_Cell.capsule");
            if !cap_starter.exists() {
                let _ = fs::write(
                    &cap_starter,
                    "[STUDY CAPSULE: Daniell Cell & Nernst Equation]\n\
                    Subject: Chemistry — Class 12 Electrochemistry\n\
                    Target Mastery: 95%\n\
                    Reaction: Zn(s) + Cu²⁺(aq) ⇌ Zn²⁺(aq) + Cu(s)\n\
                    Standard EMF E°: 1.10 V at 298 K\n\
                    Derived Formula: E_cell = 1.10 - (0.0591 / 2) · log₁₀([Zn²⁺] / [Cu²⁺])\n",
                );
            }
        } else if dir_str.contains("Documents") {
            let doc_file = dir.join("Syllabus_Mastery_Plan.txt");
            if !doc_file.exists() {
                let _ = fs::write(
                    &doc_file,
                    "[MINDSET OS ACADEMIC TARGET]\n\
                    Current Goal: Top 500 National Rank\n\
                    Target Exam: TN Class 12 Board & JEE Advanced\n\
                    Weekly Hours Target: 28 Hours\n\
                    Status: On Track (62% Syllabus Covered)\n",
                );
            }
        } else if dir_str.contains("Downloads") {
            let dl_file = dir.join("NCERT_Sample_Problems.txt");
            if !dl_file.exists() {
                let _ = fs::write(
                    &dl_file,
                    "[NCERT SAMPLE PROBLEMS]\n\
                    Problem 1.1: What is the force between two small charged spheres having charges of 2x10^-7 C and 3x10^-7 C placed 30 cm apart in air?\n\
                    Solution: F = (1 / 4πε₀) · (q₁ q₂ / r²) = 9x10⁹ · (6x10⁻¹⁴ / 0.09) = 6x10⁻³ N (Repulsive).\n",
                );
            }
        }
    }
}

// ----------------------------------------------------------------------------
// 6. Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_vault_state_and_navigation() {
        select_sidebar("Documents");
        set_search("Sample");
        navigate_up();
        toggle_tree_node("Study Capsules");
    }

    #[test]
    fn test_pdf_reader_controls() {
        pdf_prev_page();
        pdf_next_page();
        pdf_jump_page("2");
    }

    #[test]
    fn test_pptx_editor_controls() {
        pptx_select_slide(1);
        pptx_title_changed("New Quantum Slide");
        pptx_body_changed("Schrodinger Wave Equation Derivation");
        pptx_save_slide();
    }

    #[test]
    fn test_text_editor_controls() {
        text_edit_changed("Updated formula text content");
        text_save();
    }
}
