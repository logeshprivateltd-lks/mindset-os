//! MindSet OS Native "YouTube Study" App (`WindowId::YouTubeStudy`)
//!
//! Built directly inside MindSet OS (zero Debian external app clutter):
//! - Rotating Piped / Invidious mirror search with 3s timeout & zero UI stall
//! - Automatic fallback to curated offline syllabus playlists (TN 12th, NCERT, JEE, NEET, UPSC)
//! - Shorts-free distraction shield (`mindset_net::YoutubeGuard`)
//! - Side-by-side timestamped lecture notes & "Turn Lecture into Capsule" integration

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::theme::{
    glass_card, glass_elevated, h_space, modern_text_input, primary_button, px_space,
    secondary_button, toggle_button, ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

const PIPED_MIRRORS: &[&str] = &[
    "https://pipedapi.kavin.rocks",
    "https://pipedapi.adminforge.de",
    "https://api.piped.yt",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyVideoItem {
    pub video_id: String,
    pub title: String,
    pub channel: String,
    pub subject: String,
    pub duration_label: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct YouTubeStudyState {
    pub search_query: String,
    pub is_searching: bool,
    pub active_mirror: String,
    pub results: Vec<StudyVideoItem>,
    pub selected_video: Option<StudyVideoItem>,
    pub lecture_notes: String,
    pub status_banner: Option<String>,
}

impl Default for YouTubeStudyState {
    fn default() -> Self {
        let curated = curated_syllabus_lectures("");
        let first = curated.first().cloned();
        Self {
            search_query: String::new(),
            is_searching: false,
            active_mirror: "Curated Offline Syllabus".to_string(),
            results: curated,
            selected_video: first,
            lecture_notes: String::new(),
            status_banner: None,
        }
    }
}

pub fn curated_syllabus_lectures(query: &str) -> Vec<StudyVideoItem> {
    let all = vec![
        StudyVideoItem {
            video_id: "gauss-law-01".into(),
            title: "Gauss's Law & Electric Flux Complete Derivation (Class 12 Physics)".into(),
            channel: "Physics Wallah • NCERT / TN".into(),
            subject: "Physics".into(),
            duration_label: "42:15".into(),
            url: "https://www.youtube.com/results?search_query=Gauss+Law+Electric+Flux+Class+12"
                .into(),
        },
        StudyVideoItem {
            video_id: "nernst-eq-02".into(),
            title: "Electrochemistry — Nernst Equation & Cell EMF Numericals".into(),
            channel: "Vedantu JEE / NEET".into(),
            subject: "Chemistry".into(),
            duration_label: "36:40".into(),
            url: "https://www.youtube.com/results?search_query=Nernst+Equation+Electrochemistry"
                .into(),
        },
        StudyVideoItem {
            video_id: "wave-optics-03".into(),
            title: "Wave Optics: Young's Double Slit & Fringe Width (Chapter 10)".into(),
            channel: "Unacademy Class 12".into(),
            subject: "Physics".into(),
            duration_label: "48:10".into(),
            url: "https://www.youtube.com/results?search_query=Wave+Optics+YDSE+Class+12".into(),
        },
        StudyVideoItem {
            video_id: "matrices-04".into(),
            title: "Matrices & Determinants: Adjoint, Inverse & Cramer's Rule".into(),
            channel: "TN State Board Maths".into(),
            subject: "Mathematics".into(),
            duration_label: "39:05".into(),
            url: "https://www.youtube.com/results?search_query=Matrices+Inverse+Class+12+Maths"
                .into(),
        },
        StudyVideoItem {
            video_id: "genetics-05".into(),
            title: "Molecular Basis of Inheritance — DNA Transcription & Lac Operon".into(),
            channel: "NCERT Biology Line-by-Line".into(),
            subject: "Biology".into(),
            duration_label: "51:20".into(),
            url:
                "https://www.youtube.com/results?search_query=Molecular+Basis+of+Inheritance+NCERT"
                    .into(),
        },
    ];
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return all;
    }
    let filtered: Vec<StudyVideoItem> = all
        .iter()
        .filter(|v| {
            v.title.to_lowercase().contains(&q)
                || v.subject.to_lowercase().contains(&q)
                || v.channel.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();
    if filtered.is_empty() {
        all
    } else {
        filtered
    }
}

/// Searches rotating Piped API mirrors with short 3s timeouts, falling back cleanly
/// to curated offline educational playlists on network failure or rate-limit.
pub async fn search_educational_videos(query: String) -> (Vec<StudyVideoItem>, String) {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return (
            curated_syllabus_lectures(""),
            "Curated Syllabus Library".to_string(),
        );
    }

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    for mirror in PIPED_MIRRORS {
        let url = format!("{mirror}/search?q={trimmed}&filter=videos");
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
                        let mut parsed = Vec::new();
                        for item in items.iter().take(8) {
                            let rel_url = item.get("url").and_then(|u| u.as_str()).unwrap_or("");
                            if rel_url.contains("/shorts/") {
                                continue;
                            }
                            let title = item
                                .get("title")
                                .and_then(|t| t.as_str())
                                .unwrap_or("Study Lecture")
                                .to_string();
                            let channel = item
                                .get("uploaderName")
                                .and_then(|c| c.as_str())
                                .unwrap_or("Educational Channel")
                                .to_string();
                            let secs = item.get("duration").and_then(|d| d.as_u64()).unwrap_or(0);
                            if secs > 0 && secs < 90 {
                                // Skip sub-90s shorts/clips
                                continue;
                            }
                            let duration_label = if secs > 0 {
                                format!("{}:{:02}", secs / 60, secs % 60)
                            } else {
                                "Lecture".to_string()
                            };
                            let full_url = if rel_url.starts_with("http") {
                                rel_url.to_string()
                            } else {
                                format!("https://www.youtube.com{rel_url}")
                            };
                            parsed.push(StudyVideoItem {
                                video_id: rel_url.to_string(),
                                title,
                                channel,
                                subject: "Live Lecture".to_string(),
                                duration_label,
                                url: full_url,
                            });
                        }
                        if !parsed.is_empty() {
                            return (parsed, mirror.to_string());
                        }
                    }
                }
            }
        }
    }

    (
        curated_syllabus_lectures(trimmed),
        "Offline Syllabus Fallback (Mirrors Busy)".to_string(),
    )
}

pub fn view<'a>(state: &'a YouTubeStudyState, palette: ThemePalette) -> Element<'a, Message> {
    let header = row![
        column![
            text("YouTube Study — Native MS OS Player")
                .size(TYPE_SUBHEAD)
                .color(palette.text_primary),
            text(format!(
                "Shorts & Comments Blocked • Source: {}",
                state.active_mirror
            ))
            .size(TYPE_CAPTION)
            .color(palette.text_secondary),
        ]
        .spacing(2),
        h_space(),
        container(
            text("🛡️ MindSet Shield Active")
                .size(TYPE_CAPTION)
                .color(palette.success),
        )
        .padding([4, 10])
        .style(move |_t| glass_card(palette)),
    ]
    .align_y(Alignment::Center);

    let search_bar = row![
        text_input(
            "Search physics, chemistry, maths, or NCERT lectures...",
            &state.search_query
        )
        .on_input(Message::YouTubeSearchChanged)
        .on_submit(Message::YouTubeSearchSubmit)
        .style(modern_text_input(palette))
        .padding(8)
        .width(Length::Fill),
        button(
            text(if state.is_searching {
                "Searching..."
            } else {
                "Search"
            })
            .size(TYPE_BODY)
            .color(palette.accent_text),
        )
        .on_press(Message::YouTubeSearchSubmit)
        .style(primary_button(palette))
        .padding([8, 16]),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut list_col = column![].spacing(8);
    for (idx, item) in state.results.iter().enumerate() {
        let is_selected = state
            .selected_video
            .as_ref()
            .is_some_and(|s| s.video_id == item.video_id);
        list_col = list_col.push(
            button(
                column![
                    text(&item.title)
                        .size(TYPE_BODY)
                        .color(palette.text_primary),
                    row![
                        text(&item.channel)
                            .size(TYPE_CAPTION)
                            .color(palette.text_secondary),
                        h_space(),
                        text(&item.duration_label)
                            .size(TYPE_CAPTION)
                            .color(palette.accent),
                    ]
                    .align_y(Alignment::Center),
                ]
                .spacing(4),
            )
            .on_press(Message::YouTubeSelectVideo(idx))
            .style(toggle_button(palette, is_selected))
            .padding(10)
            .width(Length::Fill),
        );
    }

    let left_pane = container(scrollable(list_col).height(Length::Fill))
        .width(Length::FillPortion(5))
        .height(Length::Fill)
        .padding(10)
        .style(move |_t| glass_card(palette));

    let right_pane: Element<'a, Message> = if let Some(vid) = &state.selected_video {
        container(
            column![
                text(&vid.title)
                    .size(TYPE_SUBHEAD)
                    .color(palette.text_primary),
                text(format!("{} • {}", vid.channel, vid.duration_label))
                    .size(TYPE_CAPTION)
                    .color(palette.text_secondary),
                row![
                    button(
                        text("▶ Watch Lecture (Shielded)")
                            .size(TYPE_BODY)
                            .color(palette.accent_text),
                    )
                    .on_press(Message::LaunchMindSetBrowser(Some(vid.url.clone())))
                    .style(primary_button(palette))
                    .padding([8, 14]),
                    button(
                        text("⚡ Turn into Capsule")
                            .size(TYPE_BODY)
                            .color(palette.text_primary),
                    )
                    .on_press(Message::RunQuickOrbCommand(format!(
                        "create a 30 min capsule for {}",
                        vid.title
                    )))
                    .style(secondary_button(palette))
                    .padding([8, 14]),
                ]
                .spacing(8),
                px_space(0.0, 4.0),
                text("Lecture Notes (Saved to Study Vault)")
                    .size(TYPE_CAPTION)
                    .color(palette.accent),
                text_input("Jot formulas or timestamps here...", &state.lecture_notes)
                    .on_input(Message::YouTubeNotesChanged)
                    .style(modern_text_input(palette))
                    .padding(10)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .padding(14),
        )
        .width(Length::FillPortion(6))
        .height(Length::Fill)
        .style(move |_t| glass_card(palette))
        .into()
    } else {
        container(text("Select a lecture to start.").color(palette.text_muted))
            .width(Length::FillPortion(6))
            .padding(20)
            .into()
    };

    let body = row![left_pane, right_pane].spacing(12).height(Length::Fill);

    container(column![header, search_bar, body].spacing(12).padding(18))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette))
        .into()
}
