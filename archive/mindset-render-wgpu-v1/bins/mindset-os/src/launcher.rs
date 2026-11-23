//! MindSet OS — Start / MindSet Flash Launcher (`launcher.rs`)
//! Floating visionOS launcher featuring live AI Hub prompt banner,
//! pinned 3D squircle app grid, recommended study documents, and power controls.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use mindset_render::SdfInstance;
use mindset_surface::WindowId;
use mindset_ui::LayoutRect;

pub struct LauncherClickTarget {
    pub target_window: Option<WindowId>,
    pub is_power_click: bool,
    pub is_ai_hub_click: bool,
}

pub fn render_mindset_launcher(
    draw: &mut ClippedDraw,
    screen_w: f32,
    screen_h: f32,
    search_query: &str,
) -> LayoutRect {
    let launcher_w = 540.0;
    let launcher_h = 510.0;
    let launcher_x = (screen_w - launcher_w) * 0.5;
    let launcher_y = (screen_h - launcher_h) * 0.44;
    let launcher_rect = LayoutRect::new(launcher_x, launcher_y, launcher_w, launcher_h);

    // 1. Frosted Liquid Glass Container
    draw.sdf.push(SdfInstance::glass_panel(
        launcher_rect.x,
        launcher_rect.y,
        launcher_rect.w,
        launcher_rect.h,
        24.0,
        [0.89, 0.94, 1.0, 0.78], // Frosted light visionOS glass
        [1.0, 1.0, 1.0, 0.65],   // Specular rim
    ));

    // Top rim highlight
    draw.sdf.push(SdfInstance::button(
        launcher_rect.x + 16.0,
        launcher_rect.y + 2.0,
        launcher_rect.w - 32.0,
        8.0,
        4.0,
        [1.0, 1.0, 1.0, 0.40],
        [0.0; 4],
        0.0,
    ));

    // 2. Search Bar
    let search_r = LayoutRect::new(
        launcher_rect.x + 24.0,
        launcher_rect.y + 20.0,
        launcher_rect.w - 48.0,
        36.0,
    );
    draw.sdf.push(SdfInstance::button(
        search_r.x,
        search_r.y,
        search_r.w,
        search_r.h,
        18.0,
        [1.0, 1.0, 1.0, 0.70],
        [1.0, 1.0, 1.0, 0.35],
        0.05,
    ));
    let display_query = if search_query.is_empty() {
        "🔍 Search for apps, files, and settings..."
    } else {
        search_query
    };
    draw.queue_text_bounded(
        display_query,
        search_r.x + 14.0,
        search_r.y + 10.0,
        search_r.w - 28.0,
        11.5,
        if search_query.is_empty() {
            [0.45, 0.52, 0.64, 1.0]
        } else {
            [0.12, 0.16, 0.23, 1.0]
        },
    );

    // 3. Top Banner: AI Hub & Weather
    let banner_y = search_r.y + search_r.h + 12.0;
    let ai_hub_w = 260.0;
    let ai_hub_r = LayoutRect::new(launcher_rect.x + 24.0, banner_y, ai_hub_w, 52.0);

    // AI Hub Banner with Colorful Liquid Gradient
    draw.sdf.push(SdfInstance::button(
        ai_hub_r.x,
        ai_hub_r.y,
        ai_hub_r.w,
        ai_hub_r.h,
        14.0,
        [0.22, 0.38, 0.85, 0.90],
        [0.55, 0.75, 1.0, 0.60],
        0.15,
    ));
    draw.queue_text(
        "🧠 AI Hub",
        ai_hub_r.x + 14.0,
        ai_hub_r.y + 12.0,
        13.0,
        [1.0, 1.0, 1.0, 1.0],
    );
    draw.queue_text(
        "Ask Maindy anything...",
        ai_hub_r.x + 14.0,
        ai_hub_r.y + 30.0,
        10.5,
        [0.85, 0.92, 1.0, 0.80],
    );
    let ask_btn = LayoutRect::new(
        ai_hub_r.x + ai_hub_r.w - 60.0,
        ai_hub_r.y + 14.0,
        48.0,
        24.0,
    );
    draw.push_btn(ask_btn, [1.0, 1.0, 1.0, 0.85], [0.0; 4]);
    draw.queue_text_centered_in_rect("Ask", ask_btn, 10.5, [0.15, 0.25, 0.65, 1.0]);

    // Weather Summary on Right
    let weather_sum_x = ai_hub_r.x + ai_hub_r.w + 16.0;
    let weather_sum_w = launcher_rect.w - 48.0 - ai_hub_w - 16.0;
    let weather_sum_r = LayoutRect::new(weather_sum_x, banner_y, weather_sum_w, 52.0);
    draw.sdf.push(SdfInstance::button(
        weather_sum_r.x,
        weather_sum_r.y,
        weather_sum_r.w,
        weather_sum_r.h,
        14.0,
        [1.0, 1.0, 1.0, 0.50],
        [1.0, 1.0, 1.0, 0.25],
        0.05,
    ));
    draw.queue_text(
        "⛅",
        weather_sum_r.x + 10.0,
        weather_sum_r.y + 16.0,
        18.0,
        [1.0, 0.78, 0.18, 1.0],
    );
    draw.queue_text(
        "Tuesday, April 2",
        weather_sum_r.x + 36.0,
        weather_sum_r.y + 12.0,
        11.0,
        [0.12, 0.16, 0.23, 1.0],
    );
    draw.queue_text(
        "18° Mostly Sunny",
        weather_sum_r.x + 36.0,
        weather_sum_r.y + 28.0,
        10.5,
        [0.35, 0.42, 0.54, 1.0],
    );

    // 4. Pinned Section: Grid of 10 App Icons (2 rows of 5)
    let pinned_y = banner_y + 64.0;
    draw.queue_text(
        "Pinned",
        launcher_rect.x + 28.0,
        pinned_y,
        12.0,
        [0.12, 0.16, 0.23, 1.0],
    );

    let app_icons = [
        ("Today", "🏠", [0.96, 0.62, 0.04, 1.0], WindowId::Today),
        (
            "Browser",
            "🌐",
            [0.0, 0.48, 1.0, 1.0],
            WindowId::SearchBrowser,
        ),
        ("Word", "📝", [0.15, 0.35, 0.85, 1.0], WindowId::StudyReader),
        (
            "Notes",
            "📑",
            [0.063, 0.725, 0.506, 1.0],
            WindowId::LayaAiChat,
        ),
        ("Tasks", "✓", [0.063, 0.725, 0.506, 1.0], WindowId::Tasks),
        (
            "Explorer",
            "📁",
            [0.984, 0.749, 0.141, 1.0],
            WindowId::FileVault,
        ),
        (
            "Stats",
            "📊",
            [0.063, 0.725, 0.506, 1.0],
            WindowId::ProgressStats,
        ),
        (
            "Doctor",
            "💻",
            [0.85, 0.25, 0.45, 1.0],
            WindowId::SystemMonitor,
        ),
        (
            "Settings",
            "⚙️",
            [0.45, 0.50, 0.60, 1.0],
            WindowId::Settings,
        ),
        (
            "Showcase",
            "🎨",
            [0.65, 0.35, 0.85, 1.0],
            WindowId::DesignSystem,
        ),
    ];

    let icon_w = 44.0;
    let grid_start_x = launcher_rect.x + 32.0;
    let col_spacing = (launcher_rect.w - 64.0 - icon_w * 5.0) / 4.0;

    for (idx, (name, sym, col, _)) in app_icons.iter().enumerate() {
        let row = idx / 5;
        let col_idx = idx % 5;
        let ix = grid_start_x + (col_idx as f32) * (icon_w + col_spacing);
        let iy = pinned_y + 20.0 + (row as f32) * 62.0;

        let icon_box = LayoutRect::new(ix, iy, icon_w, icon_w);
        draw.sdf.push(SdfInstance::button(
            icon_box.x,
            icon_box.y,
            icon_box.w,
            icon_box.h,
            11.0,
            *col,
            [1.0, 1.0, 1.0, 0.40],
            0.15,
        ));
        draw.queue_text_centered_in_rect(sym, icon_box, 16.0, [1.0, 1.0, 1.0, 1.0]);
        draw.queue_text_centered_in_rect(
            name,
            LayoutRect::new(ix - 12.0, iy + icon_w + 3.0, icon_w + 24.0, 14.0),
            10.0,
            [0.12, 0.16, 0.23, 1.0],
        );
    }

    // 5. Recommended Section
    let rec_y = pinned_y + 154.0;
    draw.queue_text(
        "Recommended",
        launcher_rect.x + 28.0,
        rec_y,
        12.0,
        [0.12, 0.16, 0.23, 1.0],
    );

    let rec_files = [
        ("Quantum_Mechanics_Lecture_04.pdf", "Today", "📄"),
        ("Quarterly_Report_Draft.docx", "Yesterday", "📝"),
        ("Deep_Work_Study_Playlist.flac", "2 Days Ago", "🎵"),
    ];

    for (f_idx, (fname, time_str, icon)) in rec_files.iter().enumerate() {
        let fy = rec_y + 18.0 + (f_idx as f32) * 26.0;
        let file_r = LayoutRect::new(launcher_rect.x + 24.0, fy, launcher_rect.w - 48.0, 22.0);

        draw.queue_text(
            icon,
            file_r.x + 6.0,
            file_r.y + 4.0,
            12.0,
            [0.20, 0.25, 0.35, 1.0],
        );
        draw.queue_text_bounded(
            fname,
            file_r.x + 26.0,
            file_r.y + 4.0,
            file_r.w - 120.0,
            11.0,
            [0.12, 0.16, 0.23, 1.0],
        );
        draw.queue_text(
            time_str,
            file_r.x + file_r.w - 80.0,
            file_r.y + 4.0,
            10.5,
            [0.45, 0.52, 0.64, 1.0],
        );
    }

    // 6. Bottom Bar: All Apps, Documents, Pictures, Power
    let bottom_y = launcher_rect.y + launcher_rect.h - 44.0;
    draw.sdf.push(SdfInstance::button(
        launcher_rect.x + 16.0,
        bottom_y,
        launcher_rect.w - 32.0,
        1.0,
        0.0,
        [0.12, 0.16, 0.23, 0.12],
        [0.0; 4],
        0.0,
    ));

    let btn_bar_y = bottom_y + 8.0;
    let pad_x = 20.0;
    let b1 = LayoutRect::new(launcher_rect.x + pad_x, btn_bar_y, 96.0, 26.0);
    draw.queue_text_centered_in_rect("▦ All Apps", b1, 11.0, [0.12, 0.16, 0.23, 1.0]);

    let b2 = LayoutRect::new(b1.x + b1.w + 8.0, btn_bar_y, 110.0, 26.0);
    draw.queue_text_centered_in_rect("📁 Documents", b2, 11.0, [0.12, 0.16, 0.23, 1.0]);

    let b3 = LayoutRect::new(b2.x + b2.w + 8.0, btn_bar_y, 98.0, 26.0);
    draw.queue_text_centered_in_rect("🖼️ Pictures", b3, 11.0, [0.12, 0.16, 0.23, 1.0]);

    let power_w = 76.0;
    let power_btn = LayoutRect::new(
        launcher_rect.x + launcher_rect.w - pad_x - power_w,
        btn_bar_y,
        power_w,
        26.0,
    );
    draw.push_btn(power_btn, [1.0, 1.0, 1.0, 0.60], [1.0, 1.0, 1.0, 0.25]);
    draw.queue_text_centered_in_rect("⏻ Power", power_btn, 11.0, [0.75, 0.20, 0.20, 1.0]);

    launcher_rect
}
