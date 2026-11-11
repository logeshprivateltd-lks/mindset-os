//! User Profile & Granular Permissions Repository
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::hlc_oplog::{execute_with_oplog, HybridLogicalClock};
use crate::kernel::EventBus;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub role: String,
    pub interface_lang: String,
    pub ai_lang: String,
    pub conversation_style: String,
    pub goals_json: String,
    pub focus_areas_json: String,
    pub routine_json: String,
    pub preferences_json: String,
    pub interests_json: String,
    pub permissions_json: String,
    pub updated_at: String,
}

pub fn get_profile(conn: &Connection, user_id: &str) -> Result<Option<UserProfile>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            user_id, display_name, role, interface_lang, ai_lang, 
            conversation_style, goals_json, focus_areas_json, routine_json, 
            preferences_json, interests_json, permissions_json, updated_at
        FROM user_profile WHERE user_id = ?1
        "#,
    )?;

    let mut rows = stmt.query(params![user_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(UserProfile {
            user_id: row.get(0)?,
            display_name: row.get(1)?,
            role: row.get(2)?,
            interface_lang: row.get(3)?,
            ai_lang: row.get(4)?,
            conversation_style: row.get(5)?,
            goals_json: row.get(6)?,
            focus_areas_json: row.get(7)?,
            routine_json: row.get(8)?,
            preferences_json: row.get(9)?,
            interests_json: row.get(10)?,
            permissions_json: row.get(11)?,
            updated_at: row.get(12)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_profile(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut profile: UserProfile,
) -> Result<UserProfile, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    profile.updated_at = now.clone();

    let payload_json = serde_json::to_string(&profile).unwrap_or_default();
    let p_clone = profile.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "user_profile",
        &p_clone.user_id,
        "UPDATE",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO user_profile (
                    user_id, display_name, role, interface_lang, ai_lang, 
                    conversation_style, goals_json, focus_areas_json, routine_json, 
                    preferences_json, interests_json, permissions_json, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                ON CONFLICT(user_id) DO UPDATE SET
                    display_name = excluded.display_name,
                    role = excluded.role,
                    interface_lang = excluded.interface_lang,
                    ai_lang = excluded.ai_lang,
                    conversation_style = excluded.conversation_style,
                    goals_json = excluded.goals_json,
                    focus_areas_json = excluded.focus_areas_json,
                    routine_json = excluded.routine_json,
                    preferences_json = excluded.preferences_json,
                    interests_json = excluded.interests_json,
                    permissions_json = excluded.permissions_json,
                    updated_at = excluded.updated_at
                "#,
                params![
                    p_clone.user_id,
                    p_clone.display_name,
                    p_clone.role,
                    p_clone.interface_lang,
                    p_clone.ai_lang,
                    p_clone.conversation_style,
                    p_clone.goals_json,
                    p_clone.focus_areas_json,
                    p_clone.routine_json,
                    p_clone.preferences_json,
                    p_clone.interests_json,
                    p_clone.permissions_json,
                    p_clone.updated_at
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(profile)
}

pub fn update_permissions(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    user_id: &str,
    permissions_json: &str,
) -> Result<(), rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    let payload = serde_json::json!({
        "permissions_json": permissions_json,
        "updated_at": now
    });
    let payload_str = payload.to_string();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "user_profile",
        user_id,
        "UPDATE",
        &payload_str,
        |conn| {
            conn.execute(
                "UPDATE user_profile SET permissions_json = ?1, updated_at = ?2 WHERE user_id = ?3",
                params![permissions_json, now, user_id],
            )?;
            Ok(())
        },
    )
}

