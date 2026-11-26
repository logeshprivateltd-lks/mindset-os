//! MindStore — `.ms` Application Storefront (`WindowId::MindStore`)
//!
//! Browse, verify (SHA-256), install, launch, and uninstall `.ms` study bundles.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};
use mindset_sys::store::{MsPackageManager, StoreCatalogEntry};

use crate::theme::{
    danger_button, glass_card, glass_elevated, h_space, modern_text_input, primary_button,
    secondary_button, toggle_button, ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

#[derive(Debug, Clone)]
pub struct MindStoreState {
    pub search_query: String,
    pub selected_category: String,
    pub catalog: Vec<StoreCatalogEntry>,
    pub installed_ids: Vec<String>,
    pub status_message: Option<String>,
}

impl Default for MindStoreState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_category: "All".to_string(),
            catalog: MsPackageManager::builtin_catalog(),
            installed_ids: Vec::new(),
            status_message: None,
        }
    }
}

pub fn view<'a>(state: &'a MindStoreState, palette: ThemePalette) -> Element<'a, Message> {
    let header = row![
        column![
            text("MindStore — .ms App Ecosystem")
                .size(TYPE_SUBHEAD)
                .color(palette.text_primary),
            text("Verified .ms bundles • SHA-256 checked • Zero-bloat student tools")
                .size(TYPE_CAPTION)
                .color(palette.text_secondary),
        ]
        .spacing(2),
        h_space(),
        button(
            text("📦 Import Local .ms")
                .size(TYPE_CAPTION)
                .color(palette.text_primary),
        )
        .on_press(Message::MindStorePickLocalPackage)
        .style(secondary_button(palette))
        .padding([6, 12]),
    ]
    .align_y(Alignment::Center);

    let categories = ["All", "STEM Tools", "Syllabus Packs", "Focus Utilities"];
    let mut cat_row = row![].spacing(6);
    for cat in categories {
        let is_sel = state.selected_category == cat;
        cat_row = cat_row.push(
            button(text(cat).size(TYPE_CAPTION).color(if is_sel {
                palette.accent_text
            } else {
                palette.text_primary
            }))
            .on_press(Message::MindStoreSelectCategory(cat.to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([5, 10]),
        );
    }

    let search_bar = row![
        text_input("Search .ms apps and syllabus packs...", &state.search_query)
            .on_input(Message::MindStoreSearchChanged)
            .style(modern_text_input(palette))
            .padding(8)
            .width(Length::Fill),
        cat_row,
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let status_banner: Element<'a, Message> = if let Some(msg) = &state.status_message {
        container(text(msg).size(TYPE_CAPTION).color(palette.accent))
            .padding([6, 12])
            .width(Length::Fill)
            .style(move |_t| glass_card(palette))
            .into()
    } else {
        row![].into()
    };

    let q = state.search_query.trim().to_lowercase();
    let mut cards_col = column![].spacing(10);
    for entry in &state.catalog {
        let m = &entry.manifest;
        if state.selected_category != "All" && m.category != state.selected_category {
            continue;
        }
        if !q.is_empty()
            && !m.name.to_lowercase().contains(&q)
            && !m.summary.to_lowercase().contains(&q)
            && !m.category.to_lowercase().contains(&q)
        {
            continue;
        }
        let is_installed = state.installed_ids.iter().any(|id| id == &m.id);
        let actions: Element<'a, Message> = if is_installed {
            row![
                button(text("Open").size(TYPE_CAPTION).color(palette.accent_text))
                    .on_press(Message::MindStoreLaunchApp(m.id.clone()))
                    .style(primary_button(palette))
                    .padding([6, 12]),
                button(text("Uninstall").size(TYPE_CAPTION))
                    .on_press(Message::MindStoreUninstallApp(m.id.clone()))
                    .style(danger_button(palette))
                    .padding([6, 12]),
            ]
            .spacing(6)
            .into()
        } else {
            button(
                text("Install (.ms)")
                    .size(TYPE_CAPTION)
                    .color(palette.accent_text),
            )
            .on_press(Message::MindStoreInstallCatalogApp(m.id.clone()))
            .style(primary_button(palette))
            .padding([6, 14])
            .into()
        };

        cards_col = cards_col.push(
            container(
                row![
                    column![
                        row![
                            text(&m.name).size(TYPE_BODY).color(palette.text_primary),
                            text(format!("v{} • {}", m.version, m.kind.label()))
                                .size(TYPE_CAPTION)
                                .color(palette.accent),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        text(&m.summary)
                            .size(TYPE_CAPTION)
                            .color(palette.text_secondary),
                        text(format!(
                            "By {} • {} KB • Perms: {}",
                            m.author,
                            entry.size_kb,
                            m.permissions.join(", ")
                        ))
                        .size(9)
                        .color(palette.text_muted),
                    ]
                    .spacing(3)
                    .width(Length::Fill),
                    actions,
                ]
                .spacing(12)
                .align_y(Alignment::Center)
                .padding(12),
            )
            .width(Length::Fill)
            .style(move |_t| glass_card(palette)),
        );
    }

    container(
        column![
            header,
            search_bar,
            status_banner,
            scrollable(cards_col).height(Length::Fill),
        ]
        .spacing(12)
        .padding(18),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_t| glass_elevated(palette))
    .into()
}
