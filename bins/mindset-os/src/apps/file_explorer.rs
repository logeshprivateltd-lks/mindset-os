//! Native Student File Explorer App (Real std::fs File Browser)
//!
//! Provides local filesystem navigation across Documents, Study Capsules,
//! Downloads, and Formula Sheets, with interactive selection state, instant
//! file text/metadata preview, double-click directory traversal, and live search.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use iced::mouse;
use iced::widget::{button, column, container, mouse_area, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};

use crate::theme::{
    glass_card, glass_elevated, h_space, modern_text_input, primary_button, px_space,
    secondary_button, toggle_button, ThemePalette,
};
use crate::Message;

pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub extension: String,
}

#[derive(Clone)]
pub struct VaultSnapshot {
    pub current_dir: PathBuf,
    pub selected_sidebar: String,
    pub selected_name: Option<String>,
    pub selected_file_path: Option<PathBuf>,
    pub is_selected_dir: bool,
    pub preview_content: Option<String>,
    pub preview_metadata: Vec<(String, String)>,
}

struct FileVaultState {
    current_dir: PathBuf,
    selected_sidebar: String,
    selected_name: Option<String>,
    selected_path: Option<PathBuf>,
    is_selected_dir: bool,
    preview_content: Option<String>,
    preview_metadata: Vec<(String, String)>,
    last_click: Option<(String, Instant)>,
}

impl FileVaultState {
    fn new() -> Self {
        let root = default_vault_dir("Study Capsules");
        ensure_starter_files(&root);

        let mut state = Self {
            current_dir: root.clone(),
            selected_sidebar: "Study Capsules".to_string(),
            selected_name: None,
            selected_path: None,
            is_selected_dir: false,
            preview_content: None,
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

        let type_desc = match ext.as_str() {
            "txt" => "Plain Text Document",
            "md" => "Markdown Study Notes",
            "capsule" => "MindSet Study Capsule",
            "pdf" => "PDF Examination Document",
            "notes" => "Revision Note Sheet",
            "rs" => "Rust Source File",
            _ => "Academic Document",
        };

        self.preview_metadata = vec![
            ("File Name".into(), name.to_string()),
            ("Type".into(), type_desc.into()),
            (
                "Size".into(),
                if size_bytes < 1024 {
                    format!("{} B", size_bytes)
                } else {
                    format!("{:.1} KB", size_bytes as f32 / 1024.0)
                },
            ),
            ("Modified".into(), modified_str),
            ("Path".into(), path.to_string_lossy().to_string()),
        ];

        // Read file safely capped at 64 KB (65,536 bytes) to prevent GUI thread stalls
        let read_res = fs::File::open(path).and_then(|file| {
            use std::io::Read;
            let mut handle = file.take(65536);
            let mut buffer = Vec::new();
            handle.read_to_end(&mut buffer)?;
            Ok(buffer)
        });

        match read_res {
            Ok(bytes) => {
                if let Ok(text) = String::from_utf8(bytes.clone()) {
                    let preview = if size_bytes > 65536 {
                        format!(
                            "{}\n\n[... File preview capped at 64 KB (total {} bytes) ...]",
                            text, size_bytes
                        )
                    } else {
                        text
                    };
                    self.preview_content = Some(preview);
                } else {
                    let sample =
                        String::from_utf8_lossy(&bytes[..bytes.len().min(1024)]).to_string();
                    self.preview_content = Some(format!(
                        "[Binary / Formatted Asset: {} bytes]\nPreview snippet:\n{}",
                        size_bytes, sample
                    ));
                }
            }
            Err(e) => {
                self.preview_content = Some(format!("Could not read file preview: {}", e));
            }
        }
    }

    fn select_dir_internal(&mut self, name: &str, path: &Path) {
        self.selected_name = Some(name.to_string());
        self.selected_path = Some(path.to_path_buf());
        self.is_selected_dir = true;

        let count = fs::read_dir(path)
            .map(|r| r.flatten().count())
            .unwrap_or(0);

        self.preview_metadata = vec![
            ("Folder Name".into(), name.to_string()),
            ("Type".into(), "Local Vault Folder".into()),
            ("Items Inside".into(), format!("{} entries", count)),
            ("Path".into(), path.to_string_lossy().to_string()),
        ];

        self.preview_content = Some(format!(
            "Folder: {}\nContains {} items.\n\nDouble-click the folder in the file list or click 'Open Folder' below to navigate into this directory.",
            name, count
        ));
    }
}

static VAULT_STATE: Mutex<Option<FileVaultState>> = Mutex::new(None);
static CURRENT_SEARCH: Mutex<&'static str> = Mutex::new("");

fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut FileVaultState) -> R,
{
    let mut lock = VAULT_STATE.lock().unwrap();
    if lock.is_none() {
        *lock = Some(FileVaultState::new());
    }
    f(lock.as_mut().unwrap())
}

// Public API called by Message dispatcher
pub fn select_sidebar(folder_name: &str) {
    with_state(|state| {
        state.selected_sidebar = folder_name.to_string();
        let target = default_vault_dir(folder_name);
        ensure_starter_files(&target);
        state.current_dir = target;
        state.selected_name = None;
        state.selected_path = None;
        state.preview_content = None;
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
                state.preview_content = None;
                state.preview_metadata.clear();
            } else {
                state.select_dir_internal(name, &entry_path);
            }
        } else {
            state.select_file_internal(name, &entry_path);
        }
    });
}

pub fn open_folder(name: &str) {
    with_state(|state| {
        let entry_path = state.current_dir.join(name);
        if entry_path.is_dir() {
            ensure_starter_files(&entry_path);
            state.current_dir = entry_path;
            state.selected_name = None;
            state.selected_path = None;
            state.preview_content = None;
            state.preview_metadata.clear();
        }
    });
}

pub fn navigate_up() {
    with_state(|state| {
        if let Some(parent) = state.current_dir.parent() {
            state.current_dir = parent.to_path_buf();
            state.selected_name = None;
            state.selected_path = None;
            state.preview_content = None;
            state.preview_metadata.clear();
        }
    });
}

pub fn set_search(query: &str) {
    let leaked: &'static str = Box::leak(query.to_string().into_boxed_str());
    let mut lock = CURRENT_SEARCH.lock().unwrap();
    *lock = leaked;
}

// ----------------------------------------------------------------------------
// View Function
// ----------------------------------------------------------------------------
pub fn view<'a>(palette: ThemePalette) -> Element<'a, Message> {
    let snapshot = with_state(|state| VaultSnapshot {
        current_dir: state.current_dir.clone(),
        selected_sidebar: state.selected_sidebar.clone(),
        selected_name: state.selected_name.clone(),
        selected_file_path: state.selected_path.clone(),
        is_selected_dir: state.is_selected_dir,
        preview_content: state.preview_content.clone(),
        preview_metadata: state.preview_metadata.clone(),
    });

    let search_str = {
        let lock = CURRENT_SEARCH.lock().unwrap();
        *lock
    };

    let current_dir = snapshot.current_dir;

    // Read directory entries
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

    // 1. Breadcrumbs & Search Filter Bar
    let can_go_up = current_dir.parent().is_some();
    let mut up_btn = button(text("⬆ Up").size(11))
        .style(secondary_button(palette))
        .padding([4, 8]);
    if can_go_up {
        up_btn = up_btn.on_press(Message::VaultNavigateUp);
    }

    let breadcrumb_text = text(format!("📂 {}", current_dir.to_string_lossy()))
        .size(11)
        .color(palette.text_primary);

    let search_bar = text_input("Search files in vault...", search_str)
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

    // 2. Left Quick Folders Sidebar
    let sidebar_folders = [
        ("📁 Documents", "Documents"),
        ("📑 Study Capsules", "Study Capsules"),
        ("📥 Downloads", "Downloads"),
        ("📝 Formula Sheets", "Formula Sheets"),
    ];

    let mut sidebar_col = column![text("STUDY VAULT").size(10).color(palette.text_muted)].spacing(6);

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
        .padding([6, 10])
        .width(Length::Fill);

        sidebar_col = sidebar_col.push(
            mouse_area(folder_btn).interaction(mouse::Interaction::Pointer),
        );
    }

    let sidebar = container(sidebar_col.width(190))
        .padding(12)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette));

    // 3. Center File Grid / List
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

    // 4. Right Preview Pane
    let preview_inner: Element<'a, Message> = if let Some(selected_name) = snapshot.selected_name {
        let mut meta_col = column![
            text(selected_name.clone()).size(13).color(palette.text_primary),
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

        let action_btn: Element<'a, Message> = if snapshot.is_selected_dir {
            button(text("📂 Open Folder").size(11))
                .on_press(Message::VaultOpenFolder(selected_name))
                .style(primary_button(palette))
                .padding([5, 12])
                .into()
        } else {
            container(text("✓ Instant Preview Active").size(10).color(palette.success)).into()
        };

        let preview_body = snapshot
            .preview_content
            .unwrap_or_else(|| "No preview available.".into());
        let content_box = container(
            scrollable(text(preview_body).size(10).color(palette.text_primary))
                .height(Length::Fill),
        )
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette));

        column![meta_col, action_btn, content_box]
            .spacing(8)
            .height(Length::Fill)
            .into()
    } else {
        column![
            text("File Preview").size(12).color(palette.text_primary),
            text("Click any folder or file to inspect metadata and view instant notes preview.")
                .size(10)
                .color(palette.text_secondary),
            h_space(),
            text("• 100% Rust Native Local Vault\n• Zero cloud leaks")
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
// Helpers: Seed default starter study files & determine directories
// ----------------------------------------------------------------------------
fn default_vault_dir(name: &str) -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    match name {
        "Documents" => dirs::document_dir().unwrap_or_else(|| home.join("Documents")),
        "Downloads" => dirs::download_dir().unwrap_or_else(|| home.join("Downloads")),
        "Formula Sheets" => home.join("Study Capsules").join("Formula Sheets"),
        _ => home.join("Study Capsules"),
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
        } else if dir_str.contains("Study Capsules") {
            let starter = dir.join("README_STUDY_VAULT.txt");
            if !starter.exists() {
                let _ = fs::write(
                    &starter,
                    "MindSet OS Study Vault\n\n\
                    Store your subject notes, exported .capsule files, and revision summaries here.\n\
                    All files in this folder are automatically accessible offline and cross-platform.\n\n\
                    • 100% Local SQLite & File Storage\n\
                    • Zero Cloud Leakage\n\
                    • Synchronized with Dynamic Exam Scout\n",
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
