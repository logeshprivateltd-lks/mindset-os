//! MindSet OS Preferences App (Responsive Two-Column Layout)
//!
//! Features responsive category sidebar navigation across Appearance,
//! Focus & Shields, Audio & Radios, and Hardware & Power, with clean glassmorphic
//! cards, smooth live preference binding, and bounded interactive controls.

use iced::widget::{button, column, container, row, scrollable, slider, text};
use iced::{Alignment, Element, Length};
use mindset_core::{ActiveOverlay, MindSetCoreState, PreferencesTab, ThemeMode};

use crate::storage::db::UserProfile;
use crate::theme::{
    danger_button, glass_card, glass_elevated, h_space, modern_slider, primary_button, px_space,
    secondary_button, toggle_button, v_space, ThemePalette,
};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    palette: ThemePalette,
    ai_activity_log: &'a [crate::storage::AiActivityRecord],
) -> Element<'a, Message> {
    let active_tab = core.preferences.active_tab;

    // Left Sidebar: Categories Navigation
    let sidebar_items = column![
        text("PREFERENCES").size(10).color(palette.text_muted),
        sidebar_button(
            "🎨 Appearance",
            active_tab == PreferencesTab::Appearance,
            Message::SelectPreferencesTab(0),
            palette,
        ),
        sidebar_button(
            "🎯 Focus & Shields",
            active_tab == PreferencesTab::FocusShields,
            Message::SelectPreferencesTab(1),
            palette,
        ),
        sidebar_button(
            "🔊 Audio & Radios",
            active_tab == PreferencesTab::AudioRadios,
            Message::SelectPreferencesTab(2),
            palette,
        ),
        sidebar_button(
            "🌐 Language & Region",
            active_tab == PreferencesTab::LanguageRegion,
            Message::SelectPreferencesTab(3),
            palette,
        ),
        sidebar_button(
            "🧠 Maindy AI & Keys",
            active_tab == PreferencesTab::MaindyAiKeys,
            Message::SelectPreferencesTab(4),
            palette,
        ),
        sidebar_button(
            "💻 Hardware & Power",
            active_tab == PreferencesTab::HardwarePower,
            Message::SelectPreferencesTab(5),
            palette,
        ),
        v_space(),
        button(
            row![
                text("✏️").size(12),
                text("Re-run Onboarding").size(11).color(palette.accent_text),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(Message::RerunOnboarding)
        .style(primary_button(palette))
        .padding([8, 12])
        .width(Length::Fill),
    ]
    .spacing(8)
    .width(200);

    let sidebar = container(sidebar_items)
        .padding(16)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette));

    // Right Content Area: Settings Cards according to active tab
    let content_stack: Element<'a, Message> = match active_tab {
        PreferencesTab::Appearance => view_appearance_tab(core, palette),
        PreferencesTab::FocusShields => view_focus_shields_tab(core, palette),
        PreferencesTab::AudioRadios => view_audio_radios_tab(core, palette),
        PreferencesTab::LanguageRegion => view_language_region_tab(profile, palette),
        PreferencesTab::MaindyAiKeys => view_maindy_ai_keys_tab(profile, palette, ai_activity_log),
        PreferencesTab::HardwarePower => view_hardware_power_tab(core, palette),
    };

    let scrollable_content = scrollable(content_stack)
        .width(Length::Fill)
        .height(Length::Fill);

    let main_layout = row![sidebar, scrollable_content]
        .spacing(16)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill);

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 1. Appearance Tab View
// ----------------------------------------------------------------------------
fn view_appearance_tab<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    // 1. Theme Selection Card
    let theme_label = text("Desktop Theme & Liquid Palette")
        .size(13)
        .color(palette.text_primary);

    let theme_buttons = row![
        theme_pill_button(
            ThemeMode::AlpineGlass,
            "🏔️ Alpine Glass",
            core.preferences.theme,
            palette
        ),
        theme_pill_button(
            ThemeMode::DarkSpace,
            "🌌 Dark Space",
            core.preferences.theme,
            palette
        ),
        theme_pill_button(
            ThemeMode::VioletNebula,
            "🔮 Violet Nebula",
            core.preferences.theme,
            palette
        ),
        theme_pill_button(
            ThemeMode::SolarAmber,
            "☀️ Solar Amber",
            core.preferences.theme,
            palette
        ),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let theme_card = container(
        column![
            theme_label,
            text("Select high-contrast liquidmorphism palette. Alpine Glass features high-opacity frosted glass with crisp readability.")
                .size(11)
                .color(palette.text_secondary),
            theme_buttons,
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 2. Global UI Scale Card
    let scale_val = core.preferences.ui_scale;
    let scale_card = container(
        column![
            row![
                text("Global UI Scale & HiDPI Density")
                    .size(13)
                    .color(palette.text_primary),
                h_space(),
                text(format!("{:.0}%", scale_val * 100.0))
                    .size(12)
                    .color(palette.accent),
            ]
            .align_y(Alignment::Center),
            text("Adjust interface geometry and text scaling uniformly from 80% to 145%.")
                .size(11)
                .color(palette.text_secondary),
            container(
                slider(0.80..=1.45, scale_val, Message::ScaleChanged)
                    .step(0.05)
                    .style(modern_slider(palette))
            )
            .width(360),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 3. Desktop Surface & Window Chrome
    let surface_card = container(
        column![
            text("Liquidmorphism Presentation Layer")
                .size(13)
                .color(palette.text_primary),
            text("Pure Rust + wgpu presentation layer with zero Electron or WebView overhead. Ultra-low idle memory footprint.")
                .size(11)
                .color(palette.text_secondary),
            row![
                button(text(if core.preferences.show_desktop {
                    "🖥️ Hide Desktop Surface"
                } else {
                    "🖥️ Show Desktop Surface"
                }).size(11))
                .on_press(Message::ToggleDesktop)
                .style(secondary_button(palette))
                .padding([6, 12]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![theme_card, scale_card, surface_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 2. Focus & Shields Tab View
// ----------------------------------------------------------------------------
fn view_focus_shields_tab<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let dnd_on = core.preferences.focus_dnd;

    // 1. Do Not Disturb Card
    let dnd_card = container(
        column![
            row![
                column![
                    text("🌙 Do Not Disturb (DND)")
                        .size(13)
                        .color(palette.text_primary),
                    text("Silence all non-critical notifications, banners, and toast sounds.")
                        .size(11)
                        .color(palette.text_secondary),
                ]
                .spacing(2),
                h_space(),
                button(text(if dnd_on { "ACTIVE" } else { "DISABLED" }).size(11).color(if dnd_on {
                    palette.accent_text
                } else {
                    palette.text_secondary
                }))
                .on_press(Message::ToggleDnd)
                .style(toggle_button(palette, dnd_on))
                .padding([6, 16]),
            ]
            .align_y(Alignment::Center),
        ]
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 2. Mindful Guard Distraction Shield Card
    let mindful_card = container(
        column![
            text("🛡️ Mindful Guard Attention Protection")
                .size(13)
                .color(palette.text_primary),
            text("Monitors active windows and intercepts distracting social feeds during focus blocks. Offers 15-second formula roast checks for bailouts.")
                .size(11)
                .color(palette.text_secondary),
            row![
                button(text("Test Mindful Guard Intercept").size(11))
                    .on_press(Message::SimulateDistraction)
                    .style(secondary_button(palette))
                    .padding([6, 12]),
                px_space(8.0, 0.0),
                button(text("🎬 Roast My Excuse Check").size(11))
                    .on_press(Message::TriggerExcuseRoast(
                        "I just need 5 minutes of Instagram for quick refreshment".to_string(),
                    ))
                    .style(secondary_button(palette))
                    .padding([6, 12]),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 3. Peripheral Tunnel Vision Card
    let tunnel_card = container(
        column![
            text("🎯 Peripheral Tunnel Vision")
                .size(13)
                .color(palette.text_primary),
            text("Suppresses peripheral desktop widgets, notification docks, and floating orbs to enforce single-task visual isolation.")
                .size(11)
                .color(palette.text_secondary),
            button(text("Toggle Tunnel Vision Mode").size(11))
                .on_press(Message::ToggleTunnelVision)
                .style(primary_button(palette))
                .padding([6, 14]),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 4. Pomodoro Interval Configuration
    let pomo_card = container(
        column![
            row![
                column![
                    text("⏱️ Pomodoro Session Defaults")
                        .size(13)
                        .color(palette.text_primary),
                    text(format!(
                        "Current interval: {} min study • {} min break",
                        core.pomodoro.duration_mins, core.pomodoro.break_mins
                    ))
                    .size(11)
                    .color(palette.text_secondary),
                ]
                .spacing(2),
                h_space(),
                button(text("Cycle Preset (25 / 50 / 90m)").size(11))
                    .on_press(Message::CyclePomodoroPreset)
                    .style(secondary_button(palette))
                    .padding([6, 12]),
            ]
            .align_y(Alignment::Center),
        ]
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![dnd_card, mindful_card, tunnel_card, pomo_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 3. Audio & Radios Tab View
// ----------------------------------------------------------------------------
fn view_audio_radios_tab<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let volume_val = core.preferences.volume;
    let airplane_on = core.preferences.airplane_mode;
    let wifi_on = core.preferences.wifi_enabled;
    let bt_on = core.preferences.bluetooth_enabled;

    // 1. Master Volume Card
    let volume_card = container(
        column![
            row![
                text("🔊 Master Audio Output")
                    .size(13)
                    .color(palette.text_primary),
                h_space(),
                text(format!("{:.0}%", volume_val * 100.0))
                    .size(12)
                    .color(palette.accent),
            ]
            .align_y(Alignment::Center),
            text("Adjust global system alert and ambient soundscape volume.")
                .size(11)
                .color(palette.text_secondary),
            container(
                slider(0.0..=1.0, volume_val, Message::VolumeChanged)
                    .step(0.05)
                    .style(modern_slider(palette))
            )
            .width(360),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 2. Procedural Ambient Rainfall Soundscape Card
    let ambient_card = container(
        column![
            text("🎧 Procedural Study Soundscape")
                .size(13)
                .color(palette.text_primary),
            text("Mathematical brown-noise rain generator running in <1 MB RAM. Zero pre-recorded audio loop repetition.")
                .size(11)
                .color(palette.text_secondary),
            button(text("Toggle Ambient Rainfall Soundscape").size(11))
                .on_press(Message::ToggleAudio)
                .style(primary_button(palette))
                .padding([6, 14]),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 3. Radios & Connectivity Card
    let wifi_btn = button(text(if airplane_on {
        "📶 Wi-Fi (Disabled by Airplane Mode)"
    } else if wifi_on {
        "📶 Wi-Fi: Connected (HomeWiFi 5G)"
    } else {
        "📶 Wi-Fi: OFF"
    }).size(11))
    .on_press(if airplane_on {
        Message::ToggleAirplaneMode
    } else {
        Message::ToggleWifi
    })
    .style(toggle_button(palette, wifi_on && !airplane_on))
    .padding([6, 12]);

    let bt_btn = button(text(if airplane_on {
        "ᛒ Bluetooth (Disabled by Airplane Mode)"
    } else if bt_on {
        "ᛒ Bluetooth: ON"
    } else {
        "ᛒ Bluetooth: OFF"
    }).size(11))
    .on_press(if airplane_on {
        Message::ToggleAirplaneMode
    } else {
        Message::ToggleBluetooth
    })
    .style(toggle_button(palette, bt_on && !airplane_on))
    .padding([6, 12]);

    let airplane_btn = button(text(if airplane_on {
        "✈️ Airplane Mode: ACTIVE"
    } else {
        "✈️ Airplane Mode: OFF"
    }).size(11))
    .on_press(Message::ToggleAirplaneMode)
    .style(toggle_button(palette, airplane_on))
    .padding([6, 12]);

    let radios_card = container(
        column![
            text("📡 Radios & Wireless Connectivity")
                .size(13)
                .color(palette.text_primary),
            text("Manage local Wi-Fi, Bluetooth, and global RF isolation.")
                .size(11)
                .color(palette.text_secondary),
            row![wifi_btn, bt_btn, airplane_btn]
                .spacing(8)
                .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![volume_card, ambient_card, radios_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 4. Language & Region Tab View
// ----------------------------------------------------------------------------
fn view_language_region_tab<'a>(
    profile: &'a UserProfile,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let curr_lang = profile.language_code.as_str();

    let lang_btn = |code: &'static str, name: &'static str, script: &'static str| {
        let is_sel = curr_lang == code;
        button(
            column![
                text(name).size(12).color(if is_sel { palette.accent_text } else { palette.text_primary }),
                text(script).size(10).color(if is_sel { palette.accent_text } else { palette.text_secondary }),
            ]
            .spacing(2)
            .align_x(Alignment::Center),
        )
        .on_press(Message::SetLanguage(code.to_string()))
        .style(toggle_button(palette, is_sel))
        .padding([8, 16])
    };

    let lang_card = container(
        column![
            text("🌐 System Interface Language & Locales").size(13).color(palette.text_primary),
            text("Switch operating system UI dynamically with zero tofu boxes. Supported locales include English, Tamil, and Malayalam with extensible JSON locale packs.")
                .size(11)
                .color(palette.text_secondary),
            row![
                lang_btn("en", "English", "Default (US/IN)"),
                lang_btn("ta", "Tamil", "தமிழ்"),
                lang_btn("ml", "Malayalam", "മലയാളം"),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let indic_shaping_card = container(
        column![
            row![
                text("🔤 Indic Script Shaping Engine").size(13).color(palette.text_primary),
                h_space(),
                text("ACTIVE").size(10).color(palette.success),
            ]
            .align_y(Alignment::Center),
            text("Shaping complex ligatures (uyirmei vowels & chillu letters) via cosmic-text + rustybuzz.")
                .size(11)
                .color(palette.text_secondary),
            row![
                text("Sample Tamil: வணக்கம் மாணவரே!").size(12).color(palette.accent),
                px_space(16.0, 0.0),
                text("Sample Malayalam: നമസ്കാരം!").size(12).color(palette.accent),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let board_card = container(
        column![
            text("🎯 Academic Region & Exam Board Target").size(13).color(palette.text_primary),
            text(format!("Current Track: {} • Target: {}/{}", profile.track_name(), profile.target_score_value, profile.target_score_max))
                .size(11)
                .color(palette.text_secondary),
            text("Regional syllabus goals automatically calibrate capsule generation, question banks, and syllabus scouts.")
                .size(10)
                .color(palette.text_muted),
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![lang_card, indic_shaping_card, board_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 5. Maindy AI & Keys Tab View
// ----------------------------------------------------------------------------
fn view_maindy_ai_keys_tab<'a>(
    profile: &'a UserProfile,
    palette: ThemePalette,
    ai_activity_log: &'a [crate::storage::AiActivityRecord],
) -> Element<'a, Message> {
    let curr_style = profile.maindy_style.as_str();

    let style_btn = |code: &'static str, label: &'static str| {
        let is_sel = curr_style == code;
        button(text(label).size(11).color(if is_sel { palette.accent_text } else { palette.text_primary }))
            .on_press(Message::SetMaindyStyle(code.to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([6, 12])
    };

    let style_card = container(
        column![
            text("🧠 Maindy AI Bilingual Conversation Style").size(13).color(palette.text_primary),
            text("Choose how Maindy speaks with you. Thanglish uses casual Tamil in English letters ('Vanakkam da mapla!').")
                .size(11)
                .color(palette.text_secondary),
            row![
                style_btn("thanglish", "🔥 Thanglish (Default)"),
                style_btn("ta", "தமிழ் Tamil"),
                style_btn("en", "English"),
                style_btn("ml", "മലയാളം Malayalam"),
                style_btn("auto", "⚡ Auto-Match"),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let byok_card = container(
        column![
            row![
                text("🔑 Bring-Your-Own-Key (BYOK) Cloud AI Slot").size(13).color(palette.text_primary),
                h_space(),
                text("HYBRID").size(10).color(palette.accent),
            ]
            .align_y(Alignment::Center),
            text("Optional: Enter your API key to unlock cloud models (Google Gemini 2.5 Flash, Groq, OpenAI). When left blank, MindSet OS automatically routes to free local Ollama or the built-in offline study helper.")
                .size(11)
                .color(palette.text_secondary),
            row![
                button(text("Gemini 2.5 Flash").size(11))
                    .on_press(Message::SetAiCloudProvider("gemini".into()))
                    .style(secondary_button(palette))
                    .padding([5, 10]),
                button(text("Groq (Llama 3.3)").size(11))
                    .on_press(Message::SetAiCloudProvider("groq".into()))
                    .style(secondary_button(palette))
                    .padding([5, 10]),
                button(text("OpenAI (GPT-4o-mini)").size(11))
                    .on_press(Message::SetAiCloudProvider("openai".into()))
                    .style(secondary_button(palette))
                    .padding([5, 10]),
                button(text("Custom Endpoint").size(11))
                    .on_press(Message::SetAiCloudProvider("custom".into()))
                    .style(secondary_button(palette))
                    .padding([5, 10]),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            row![
                button(text("⚡ Test API Connection").size(11))
                    .on_press(Message::TestAiConnection)
                    .style(primary_button(palette))
                    .padding([6, 14]),
                button(text("🗑️ Clear Key").size(11))
                    .on_press(Message::SetAiApiKey(String::new()))
                    .style(danger_button(palette))
                    .padding([6, 14]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let local_ai_card = container(
        column![
            row![
                text("💻 Free Local AI Fallback Chain").size(13).color(palette.text_primary),
                h_space(),
                text("READY").size(10).color(palette.success),
            ]
            .align_y(Alignment::Center),
            text("1. Local Ollama (http://127.0.0.1:11434) with low-RAM auto-eviction\n2. Local OpenAI-compatible server (LM Studio / llama.cpp)\n3. Zero-Failure Offline Multilingual Study & Formula Helper")
                .size(11)
                .color(palette.text_secondary),
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let tts_card = container(
        column![
            text("🔊 Text-to-Speech (TTS) Pronunciation Engine").size(13).color(palette.text_primary),
            text("Dual-text pronunciation pipeline: Displays Thanglish in chat while speaking native phonetics through Windows OneCore/SAPI or optional Sarvam AI.")
                .size(11)
                .color(palette.text_secondary),
            row![
                text("Voices:").size(11).color(palette.text_primary),
                text("ta-IN Valluvar • en-IN Heera • ml-IN").size(11).color(palette.accent),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let profile_card = container(
        column![
            row![
                text("👤 Student Profile & Personalization").size(13).color(palette.text_primary),
                h_space(),
                button(
                    row![
                        text("✏️").size(11),
                        text("Re-run Onboarding / Edit Profile").size(11).color(palette.accent_text),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center)
                )
                .on_press(Message::RerunOnboarding)
                .style(primary_button(palette))
                .padding([6, 14]),
            ]
            .align_y(Alignment::Center),
            text(format!(
                "Active Student: {}  •  Target: {}  •  Coaching: {}",
                profile.name,
                profile.target_exam_display(),
                profile.coaching_style
            ))
            .size(11)
            .color(palette.text_secondary),
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    let mut log_items = column![].spacing(6);

    if ai_activity_log.is_empty() {
        log_items = log_items.push(
            text("No agent actions recorded yet. When Maindy AI creates capsules or requests folder access, events will appear here.")
                .size(11)
                .color(palette.text_muted),
        );
    } else {
        for record in ai_activity_log.iter().take(15) {
            let status_color = if record.status.starts_with("Approved") {
                palette.success
            } else if record.status.contains("Denied") {
                palette.danger
            } else if record.status == "Auto" {
                palette.accent
            } else {
                palette.warning
            };

            let time_str = if record.timestamp.len() >= 19 {
                &record.timestamp[11..19]
            } else {
                &record.timestamp
            };

            let item_row = row![
                text(time_str).size(10).color(palette.text_muted),
                text(&record.action_type).size(11).color(palette.text_primary),
                text("•").size(10).color(palette.text_muted),
                text(&record.target_resource).size(10).color(palette.text_secondary),
                h_space(),
                container(
                    text(&record.status)
                        .size(9)
                        .color(palette.accent_text)
                )
                .padding([2, 6])
                .style(move |_t| container::Style {
                    background: Some(iced::Background::Color(status_color)),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            log_items = log_items.push(item_row);
        }
    }

    let activity_log_card = container(
        column![
            row![
                column![
                    text("🛡️ AI Activity & Permission Log").size(13).color(palette.text_primary),
                    text("Audit trail of all OS tool actions and folder inspections executed by Maindy AI.")
                        .size(11)
                        .color(palette.text_secondary),
                ],
                h_space(),
                row![
                    button(
                        row![
                            text("🚫").size(11),
                            text("Revoke All Permissions").size(11).color(palette.accent_text),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center)
                    )
                    .on_press(Message::RevokeAllAgentPermissions)
                    .style(danger_button(palette))
                    .padding([6, 12]),
                    button(
                        row![
                            text("🧹").size(11),
                            text("Clear Log").size(11),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center)
                    )
                    .on_press(Message::ClearAiActivityLog)
                    .style(secondary_button(palette))
                    .padding([6, 12]),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center),
            log_items,
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![profile_card, style_card, byok_card, local_ai_card, tts_card, activity_log_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

// ----------------------------------------------------------------------------
// 6. Hardware & Power Tab View
// ----------------------------------------------------------------------------
fn view_hardware_power_tab<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let brightness_val = core.preferences.brightness;

    // 1. Hardware Display Brightness Card
    let brightness_card = container(
        column![
            row![
                text("☀️ Display Brightness & Night Light")
                    .size(13)
                    .color(palette.text_primary),
                h_space(),
                text(format!("{:.0}%", brightness_val * 100.0))
                    .size(12)
                    .color(palette.accent),
            ]
            .align_y(Alignment::Center),
            text("Adjust display brightness smoothly with click-through scrim overlay protection.")
                .size(11)
                .color(palette.text_secondary),
            container(
                slider(0.20..=1.0, brightness_val, Message::BrightnessChanged)
                    .step(0.05)
                    .style(modern_slider(palette))
            )
            .width(360),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 2. Ollama Local AI & RAM Eviction Card
    let ai_card = container(
        column![
            text("🧠 Local AI Engine & Memory Discipline")
                .size(13)
                .color(palette.text_primary),
            text("Gemma-2-2B offline neural tutor running locally on 127.0.0.1:11434. Keep-alive timeout eviction maintains <80 MB base shell footprint.")
                .size(11)
                .color(palette.text_secondary),
            row![
                button(text("🧹 Evict Ollama Model from RAM").size(11))
                    .on_press(Message::UnloadOllamaModel)
                    .style(danger_button(palette))
                    .padding([6, 12]),
                button(text("⚡ Start Local Ollama").size(11))
                    .on_press(Message::StartOllama)
                    .style(secondary_button(palette))
                    .padding([6, 12]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    // 3. Power & Lock Session Card
    let power_card = container(
        column![
            text("⏻ Session Security & Power")
                .size(13)
                .color(palette.text_primary),
            text("Lock current desktop session or engage deep focus sleep state.")
                .size(11)
                .color(palette.text_secondary),
            row![
                button(text("🔒 Lock Screen (Super+L)").size(11))
                    .on_press(Message::ToggleOverlay(ActiveOverlay::LockScreen))
                    .style(secondary_button(palette))
                    .padding([6, 12]),
                button(text("🌙 Sleep & Deep Focus").size(11))
                    .on_press(Message::ToggleDnd)
                    .style(secondary_button(palette))
                    .padding([6, 12]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .max_width(720)
    .style(move |_t| glass_card(palette));

    column![brightness_card, ai_card, power_card]
        .spacing(16)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn sidebar_button<'a>(
    label: &'static str,
    is_active: bool,
    msg: Message,
    palette: ThemePalette,
) -> Element<'a, Message> {
    button(
        row![
            text(label).size(12).color(if is_active {
                palette.accent
            } else {
                palette.text_secondary
            }),
            h_space(),
            if is_active {
                text("●").size(8).color(palette.accent)
            } else {
                text("").size(8)
            },
        ]
        .align_y(Alignment::Center),
    )
    .on_press(msg)
    .style(toggle_button(palette, is_active))
    .padding([8, 14])
    .width(Length::Fill)
    .into()
}

fn theme_pill_button<'a>(
    mode: ThemeMode,
    label: &'static str,
    current_mode: ThemeMode,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let is_selected = mode == current_mode;
    button(text(label).size(11).color(if is_selected {
        palette.accent_text
    } else {
        palette.text_primary
    }))
    .on_press(Message::ThemeSelected(mode))
    .style(toggle_button(palette, is_selected))
    .padding([6, 12])
    .into()
}
