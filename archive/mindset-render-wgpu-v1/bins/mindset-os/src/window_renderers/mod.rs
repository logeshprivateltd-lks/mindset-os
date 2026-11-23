//! MindSet OS Window Renderers Module

pub mod browser;
pub mod design_system;
pub mod monitor;
pub mod settings;
pub mod study;
pub mod tasks_stats;
pub mod vault;

pub use browser::render_search_browser_content;
pub use monitor::render_monitor_content;
pub use settings::render_settings_content;
pub use study::{
    render_laya_chat_content, render_maindy_chat_content, render_study_reader_content,
};
pub use tasks_stats::{render_stats_content, render_tasks_content};
pub use vault::render_file_vault_content;
