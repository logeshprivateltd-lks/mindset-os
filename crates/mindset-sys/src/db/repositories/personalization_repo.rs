//! MindSet Personalization & Desktop Shortcuts Repository
//! High performance SQLite CRUD with low-RAM footprint
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::DbError;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopShortcut {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub target_type: String, // "app", "file", "url", "folder", "action"
    pub target_payload: String,
    pub grid_x: i32,
    pub grid_y: i32,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPersonalization {
    pub user_id: String,
    pub wallpaper_type: String, // "preset", "custom", "solid", "video"
    pub wallpaper_path: String,
    pub wallpaper_dimness: f64,
    pub wallpaper_blur_px: i32,
    pub accent_color: String,
    pub font_scale: f64,
    pub glass_opacity: f64,
    pub motion_intensity: String,
    pub dock_position: String,
    pub dock_autohide: bool,
    pub dock_style: String,
    pub widgets_json: String,
    pub updated_at: String,
}

pub fn list_shortcuts(conn: &Connection) -> Result<Vec<DesktopShortcut>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, title, icon, target_type, target_payload, grid_x, grid_y, is_pinned, created_at, updated_at 
         FROM desktop_shortcuts ORDER BY grid_y ASC, grid_x ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        let is_pinned_int: i32 = row.get(7)?;
        Ok(DesktopShortcut {
            id: row.get(0)?,
            title: row.get(1)?,
            icon: row.get(2)?,
            target_type: row.get(3)?,
            target_payload: row.get(4)?,
            grid_x: row.get(5)?,
            grid_y: row.get(6)?,
            is_pinned: is_pinned_int != 0,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut shortcuts = Vec::new();
    for r in rows {
        shortcuts.push(r?);
    }
    Ok(shortcuts)
}

pub fn save_shortcut(conn: &Connection, sc: &DesktopShortcut) -> Result<(), DbError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO desktop_shortcuts (
            id, title, icon, target_type, target_payload, grid_x, grid_y, is_pinned, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            icon = excluded.icon,
            target_type = excluded.target_type,
            target_payload = excluded.target_payload,
            grid_x = excluded.grid_x,
            grid_y = excluded.grid_y,
            is_pinned = excluded.is_pinned,
            updated_at = excluded.updated_at
        "#,
        params![
            sc.id,
            sc.title,
            sc.icon,
            sc.target_type,
            sc.target_payload,
            sc.grid_x,
            sc.grid_y,
            if sc.is_pinned { 1 } else { 0 },
            if sc.created_at.is_empty() { &now } else { &sc.created_at },
            now,
        ],
    )?;
    Ok(())
}

pub fn update_shortcut_coords(
    conn: &Connection,
    id: &str,
    grid_x: i32,
    grid_y: i32,
) -> Result<(), DbError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE desktop_shortcuts SET grid_x = ?1, grid_y = ?2, updated_at = ?3 WHERE id = ?4",
        params![grid_x, grid_y, now, id],
    )?;
    Ok(())
}

pub fn delete_shortcut(conn: &Connection, id: &str) -> Result<(), DbError> {
    conn.execute("DELETE FROM desktop_shortcuts WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_personalization(
    conn: &Connection,
    user_id: &str,
) -> Result<UserPersonalization, DbError> {
    let res = conn.query_row(
        r#"
        SELECT user_id, wallpaper_type, wallpaper_path, wallpaper_dimness, wallpaper_blur_px,
               accent_color, font_scale, glass_opacity, motion_intensity, dock_position,
               dock_autohide, dock_style, widgets_json, updated_at
        FROM user_personalization
        WHERE user_id = ?1
        "#,
        params![user_id],
        |row| {
            let dock_autohide_int: i32 = row.get(10)?;
            Ok(UserPersonalization {
                user_id: row.get(0)?,
                wallpaper_type: row.get(1)?,
                wallpaper_path: row.get(2)?,
                wallpaper_dimness: row.get(3)?,
                wallpaper_blur_px: row.get(4)?,
                accent_color: row.get(5)?,
                font_scale: row.get(6)?,
                glass_opacity: row.get(7)?,
                motion_intensity: row.get(8)?,
                dock_position: row.get(9)?,
                dock_autohide: dock_autohide_int != 0,
                dock_style: row.get(11)?,
                widgets_json: row.get(12)?,
                updated_at: row.get(13)?,
            })
        },
    );

    match res {
        Ok(p) => Ok(p),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Return default
            Ok(UserPersonalization {
                user_id: user_id.to_string(),
                wallpaper_type: "preset".to_string(),
                wallpaper_path: "nebula_deep".to_string(),
                wallpaper_dimness: 0.25,
                wallpaper_blur_px: 0,
                accent_color: "#8B9CFF".to_string(),
                font_scale: 1.0,
                glass_opacity: 0.85,
                motion_intensity: "normal".to_string(),
                dock_position: "bottom".to_string(),
                dock_autohide: false,
                dock_style: "floating".to_string(),
                widgets_json: r#"{"clock":true,"target":true,"affirmation":true}"#.to_string(),
                updated_at: Utc::now().to_rfc3339(),
            })
        }
        Err(e) => Err(DbError::Sqlite(e)),
    }
}

pub fn save_personalization(conn: &Connection, p: &UserPersonalization) -> Result<(), DbError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO user_personalization (
            user_id, wallpaper_type, wallpaper_path, wallpaper_dimness, wallpaper_blur_px,
            accent_color, font_scale, glass_opacity, motion_intensity, dock_position,
            dock_autohide, dock_style, widgets_json, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        ON CONFLICT(user_id) DO UPDATE SET
            wallpaper_type = excluded.wallpaper_type,
            wallpaper_path = excluded.wallpaper_path,
            wallpaper_dimness = excluded.wallpaper_dimness,
            wallpaper_blur_px = excluded.wallpaper_blur_px,
            accent_color = excluded.accent_color,
            font_scale = excluded.font_scale,
            glass_opacity = excluded.glass_opacity,
            motion_intensity = excluded.motion_intensity,
            dock_position = excluded.dock_position,
            dock_autohide = excluded.dock_autohide,
            dock_style = excluded.dock_style,
            widgets_json = excluded.widgets_json,
            updated_at = excluded.updated_at
        "#,
        params![
            p.user_id,
            p.wallpaper_type,
            p.wallpaper_path,
            p.wallpaper_dimness,
            p.wallpaper_blur_px,
            p.accent_color,
            p.font_scale,
            p.glass_opacity,
            p.motion_intensity,
            p.dock_position,
            if p.dock_autohide { 1 } else { 0 },
            p.dock_style,
            p.widgets_json,
            now,
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_shortcuts_and_personalization_crud() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::db::migrations::MigrationManager::run_migrations(Path::new("dummy"), &mut conn)
            .unwrap();

        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO user_profile (user_id, display_name, updated_at) VALUES ('default_student', 'Test Scholar', ?1)",
            rusqlite::params![now],
        ).unwrap();

        // 1. Verify default shortcuts were seeded
        let shortcuts = list_shortcuts(&conn).unwrap();
        assert_eq!(shortcuts.len(), 6);
        assert_eq!(shortcuts[0].id, "sc_command_center");

        // 2. Save new shortcut
        let custom_sc = DesktopShortcut {
            id: "sc_custom".to_string(),
            title: "Physics Lab".to_string(),
            icon: "🧪".to_string(),
            target_type: "app".to_string(),
            target_payload: "notes".to_string(),
            grid_x: 1,
            grid_y: 2,
            is_pinned: true,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        save_shortcut(&conn, &custom_sc).unwrap();

        let updated_list = list_shortcuts(&conn).unwrap();
        assert_eq!(updated_list.len(), 7);

        // 3. Update coordinates
        update_shortcut_coords(&conn, "sc_custom", 2, 3).unwrap();
        let list_after_move = list_shortcuts(&conn).unwrap();
        let moved = list_after_move
            .iter()
            .find(|s| s.id == "sc_custom")
            .unwrap();
        assert_eq!(moved.grid_x, 2);
        assert_eq!(moved.grid_y, 3);

        // 4. Delete shortcut
        delete_shortcut(&conn, "sc_custom").unwrap();
        let list_after_del = list_shortcuts(&conn).unwrap();
        assert_eq!(list_after_del.len(), 6);

        // 5. Personalization settings
        let mut p = get_personalization(&conn, "default_student").unwrap();
        assert_eq!(p.wallpaper_path, "nebula_deep");
        assert_eq!(p.accent_color, "#8B9CFF");

        p.wallpaper_path = "quantum_mesh".to_string();
        p.wallpaper_dimness = 0.40;
        p.accent_color = "#65E6C4".to_string();
        save_personalization(&conn, &p).unwrap();

        let p2 = get_personalization(&conn, "default_student").unwrap();
        assert_eq!(p2.wallpaper_path, "quantum_mesh");
        assert_eq!(p2.wallpaper_dimness, 0.40);
        assert_eq!(p2.accent_color, "#65E6C4");
    }
}
