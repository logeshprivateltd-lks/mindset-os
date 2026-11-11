//! MindSet Connect — Desktop Subsystem Router & Tauri IPC Commands (`connect/mod.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod continuity;
pub mod hardware_bridge;
pub mod pairing;
pub mod remote_fs;
pub mod security_audit;
pub mod sync_engine;

use continuity::DesktopContinuityManager;
use hardware_bridge::DesktopHardwareBridge;
use pairing::PairingManager;
use remote_fs::RemoteFsManager;
use security_audit::ConnectSecurityEngine;
use sync_engine::DesktopSyncEngine;

use connect_protocol::{
    ClipboardSyncPayload, ConflictResolutionChoice, ConnectionMode, ContinuityItemType,
    ContinuityStateHandoff, CrossDeviceFocusEvent, DeviceRevocationRecord, DeviceType,
    FocusProfileType, QrPairingPayload, RemoteBrowseRequest, RemoteBrowseResponse,
    RemoteDeleteConfirmationGate, RemoteFilePreviewRequest, RemoteFilePreviewResponse,
    ScannedDocumentPayload, SelectiveFolderSyncStatus, SeminarClickerAction, SyncConflictItem,
    TrustedDevice, WirelessMicrophoneStatus, WirelessWebcamStatus, ConnectAuditEntry,
};
use std::sync::{Arc, OnceLock};
use tauri::command;

pub struct ConnectManager {
    pub pairing: Arc<PairingManager>,
    pub sync: Arc<DesktopSyncEngine>,
    pub remote_fs: Arc<RemoteFsManager>,
    pub continuity: Arc<DesktopContinuityManager>,
    pub hardware: Arc<DesktopHardwareBridge>,
    pub security: Arc<ConnectSecurityEngine>,
}

impl ConnectManager {
    pub fn new() -> Self {
        Self {
            pairing: Arc::new(PairingManager::new()),
            sync: Arc::new(DesktopSyncEngine::new()),
            remote_fs: Arc::new(RemoteFsManager::new()),
            continuity: Arc::new(DesktopContinuityManager::new()),
            hardware: Arc::new(DesktopHardwareBridge::new()),
            security: Arc::new(ConnectSecurityEngine::new()),
        }
    }
}

static GLOBAL_CONNECT: OnceLock<ConnectManager> = OnceLock::new();

fn get_connect() -> &'static ConnectManager {
    GLOBAL_CONNECT.get_or_init(ConnectManager::new)
}

#[command]
pub async fn connect_generate_pairing_qr(host_name: Option<String>) -> Result<QrPairingPayload, String> {
    let name = host_name.unwrap_or_else(|| "MindSet Laptop".to_string());
    Ok(get_connect().pairing.generate_qr_payload(&name))
}

#[command]
pub async fn connect_list_trusted_devices() -> Result<Vec<TrustedDevice>, String> {
    Ok(get_connect().pairing.list_trusted_devices())
}

#[command]
pub async fn connect_approve_pairing(
    device_id: String,
    device_name: String,
    device_type: DeviceType,
    public_key: String,
    ip_address: String,
    connection_mode: ConnectionMode,
) -> Result<TrustedDevice, String> {
    get_connect()
        .pairing
        .approve_device(&device_id, &device_name, device_type, &public_key, &ip_address, connection_mode)
}

#[command]
pub async fn connect_revoke_device(device_id: String, reason: Option<String>) -> Result<DeviceRevocationRecord, String> {
    let reason_str = reason.unwrap_or_else(|| "User initiated one-click revocation".to_string());
    get_connect().pairing.revoke_device(&device_id, &reason_str)
}

#[command]
pub async fn connect_get_sync_status() -> Result<Vec<SelectiveFolderSyncStatus>, String> {
    Ok(get_connect().sync.get_sync_status())
}

#[command]
pub async fn connect_get_active_conflicts() -> Result<Vec<SyncConflictItem>, String> {
    Ok(get_connect().sync.get_active_conflicts())
}

#[command]
pub async fn connect_resolve_conflict(
    conflict_id: String,
    choice: ConflictResolutionChoice,
) -> Result<SyncConflictItem, String> {
    get_connect().sync.resolve_conflict(&conflict_id, choice)
}

#[command]
pub async fn connect_browse_remote_folder(
    target_vault_root: String,
    relative_path: Option<String>,
) -> Result<RemoteBrowseResponse, String> {
    let req = RemoteBrowseRequest {
        client_device_id: "mindset-host-laptop-01".to_string(),
        target_vault_root,
        relative_path: relative_path.unwrap_or_default(),
        session_token: "session-active".to_string(),
    };
    Ok(get_connect().remote_fs.browse_directory(req))
}

#[command]
pub async fn connect_preview_remote_file(
    file_path: String,
    max_preview_bytes: Option<usize>,
) -> Result<RemoteFilePreviewResponse, String> {
    let req = RemoteFilePreviewRequest {
        client_device_id: "mindset-host-laptop-01".to_string(),
        file_path,
        max_preview_bytes: max_preview_bytes.unwrap_or(65536),
    };
    Ok(get_connect().remote_fs.get_file_preview(req))
}

#[command]
pub async fn connect_request_remote_delete(
    file_path: String,
    requesting_device_id: String,
) -> Result<RemoteDeleteConfirmationGate, String> {
    Ok(get_connect().remote_fs.request_delete_gate(&file_path, &requesting_device_id))
}

#[command]
pub async fn connect_get_continuity_state() -> Result<Option<ContinuityStateHandoff>, String> {
    Ok(get_connect().continuity.get_continuity_state())
}

#[command]
pub async fn connect_broadcast_continuity_state(
    item_type: ContinuityItemType,
    title: String,
    resource_identifier: String,
    last_read_page: Option<u32>,
    total_pages: Option<u32>,
    subject: Option<String>,
) -> Result<ContinuityStateHandoff, String> {
    Ok(get_connect().continuity.broadcast_continuity_state(
        item_type,
        &title,
        &resource_identifier,
        last_read_page,
        total_pages,
        subject,
    ))
}

#[command]
pub async fn connect_get_active_focus() -> Result<Option<CrossDeviceFocusEvent>, String> {
    Ok(get_connect().continuity.get_active_focus())
}

#[command]
pub async fn connect_sync_focus_session(
    is_active: bool,
    profile: FocusProfileType,
    task_title: String,
    duration_minutes: u32,
    remaining_seconds: u32,
) -> Result<CrossDeviceFocusEvent, String> {
    Ok(get_connect().continuity.sync_focus_session(
        is_active,
        profile,
        &task_title,
        duration_minutes,
        remaining_seconds,
    ))
}

#[command]
pub async fn connect_sync_clipboard(
    content: String,
    source_device_id: Option<String>,
    source_device_name: Option<String>,
) -> Result<ClipboardSyncPayload, String> {
    let dev_id = source_device_id.unwrap_or_else(|| "mindset-host-laptop-01".to_string());
    let dev_name = source_device_name.unwrap_or_else(|| "MindSet Laptop".to_string());
    Ok(get_connect().hardware.filter_and_sync_clipboard(&content, &dev_id, &dev_name))
}

#[command]
pub async fn connect_send_clicker_action(action: SeminarClickerAction) -> Result<String, String> {
    let cmd = get_connect().hardware.handle_clicker_action(action);
    Ok(cmd.command_id)
}

#[command]
pub async fn connect_get_hardware_bridge_status() -> Result<(WirelessWebcamStatus, WirelessMicrophoneStatus), String> {
    let cam = get_connect().hardware.get_webcam_status();
    let mic = get_connect().hardware.get_mic_status();
    Ok((cam, mic))
}

#[command]
pub async fn connect_toggle_webcam_stream(
    active: bool,
    resolution: Option<String>,
    use_front: Option<bool>,
) -> Result<WirelessWebcamStatus, String> {
    let res = resolution.unwrap_or_else(|| "1080p (1920x1080)".to_string());
    let front = use_front.unwrap_or(false);
    Ok(get_connect().hardware.toggle_webcam(active, &res, front))
}

#[command]
pub async fn connect_toggle_mic_stream(
    active: bool,
    noise_suppression: Option<bool>,
) -> Result<WirelessMicrophoneStatus, String> {
    let ns = noise_suppression.unwrap_or(true);
    Ok(get_connect().hardware.toggle_mic(active, ns))
}

#[command]
pub async fn connect_ingest_scanned_document(payload: ScannedDocumentPayload) -> Result<String, String> {
    get_connect().hardware.ingest_scanned_document(payload)
}

#[command]
pub async fn connect_get_security_audit_logs() -> Result<Vec<ConnectAuditEntry>, String> {
    Ok(get_connect().security.get_audit_logs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_qr_pairing_generation() {
        let pairing = PairingManager::new();
        let payload = pairing.generate_qr_payload("Test Laptop");
        assert_eq!(payload.host_device_name, "Test Laptop");
        assert!(payload.expires_at > Utc::now());
        assert!(!payload.local_ip_addresses.is_empty());
    }

    #[test]
    fn test_sensitive_clipboard_filter_rule_2() {
        let bridge = DesktopHardwareBridge::new();

        // 1. Pure OTP should be filtered
        let filtered_otp = bridge.filter_and_sync_clipboard("492810", "dev-phone", "Phone");
        assert!(filtered_otp.is_sensitive_filtered);
        assert!(filtered_otp.content.contains("OTP BLOCKED"));

        // 2. Secret token should be filtered
        let filtered_token = bridge.filter_and_sync_clipboard("Authorization: Bearer sk_live_secret12345", "dev-phone", "Phone");
        assert!(filtered_token.is_sensitive_filtered);
        assert!(filtered_token.content.contains("CREDENTIAL STRIPPED"));

        // 3. Normal study notes should pass untouched
        let note = "Formula for Kinetic Energy: KE = 0.5 * m * v^2";
        let filtered_note = bridge.filter_and_sync_clipboard(note, "dev-phone", "Phone");
        assert!(!filtered_note.is_sensitive_filtered);
        assert_eq!(filtered_note.content, note);
    }

    #[test]
    fn test_conflict_resolution_keep_both_rule_1() {
        let sync = DesktopSyncEngine::new();
        let resolved = sync.resolve_conflict("cnf-phys-notes-01", ConflictResolutionChoice::KeepBoth).unwrap();
        assert_eq!(resolved.resolution, Some(ConflictResolutionChoice::KeepBoth));
        assert!(resolved.preserved_conflict_filename.is_some());
        assert!(resolved.preserved_conflict_filename.as_ref().unwrap().contains(".conflict."));
    }

    #[test]
    fn test_continuity_handoff_and_focus_sync() {
        let continuity = DesktopContinuityManager::new();
        let handoff = continuity.broadcast_continuity_state(
            ContinuityItemType::PdfTextbook,
            "Class 12 Physics Ch 4",
            "vault/my-laptop/physics.pdf",
            Some(95),
            Some(210),
            Some("Physics".to_string()),
        );
        assert_eq!(handoff.last_read_page, Some(95));

        let focus = continuity.sync_focus_session(
            true,
            FocusProfileType::DeepStudy,
            "Wave Optics Derivation",
            45,
            2700,
        );
        assert!(focus.is_focus_active);
        assert_eq!(focus.remaining_seconds, 2700);
    }

    #[test]
    fn test_hardware_bridge_webcam_and_mic_toggles() {
        let bridge = DesktopHardwareBridge::new();
        let cam = bridge.toggle_webcam(true, "1080p (1920x1080)", false);
        assert!(cam.is_active);
        assert_eq!(cam.resolution, "1080p (1920x1080)");

        let mic = bridge.toggle_mic(true, true);
        assert!(mic.is_active);
        assert!(mic.noise_suppression);
    }
}
