#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! MindSet OS — Native Desktop Visual Computing Shell (Iced 0.14.0)
//!
//! 100% Pure Rust + wgpu presentation layer, zero Electron/WebView overhead.
//! Target: < 60–80 MB Idle RSS. Single-model RAM discipline.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_os::{LaunchOptions, MindSetApp};

pub fn main() -> iced::Result {
    let _ = env_logger::try_init();
    let launch_opts = LaunchOptions::from_env();

    let win_settings = iced::window::Settings {
        decorations: false, // Pure OS desktop shell: zero host OS titlebar
        size: launch_opts.window_size,
        maximized: launch_opts.start_maximized,
        fullscreen: launch_opts.fullscreen,
        resizable: true,
        #[cfg(target_os = "linux")]
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "mindset-shell".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    iced::application(
        move || MindSetApp::new_with_options(launch_opts.clone()),
        MindSetApp::update,
        MindSetApp::view,
    )
    .title("MindSet OS — Uninterrupted Deep Focus")
    .window(win_settings)
    .scale_factor(MindSetApp::scale_factor)
    .theme(MindSetApp::theme)
    .subscription(MindSetApp::subscription)
    .font(include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf").as_slice())
    .font(include_bytes!("../../../assets/fonts/NotoSansTamil-Regular.ttf").as_slice())
    .font(include_bytes!("../../../assets/fonts/NotoSansMalayalam-Regular.ttf").as_slice())
    .run()
}
