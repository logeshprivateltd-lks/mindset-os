//! MindSet Connect — Desktop Module 1: Device Pairing & Identity (`pairing.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use connect_protocol::{
    ConnectionMode, DevicePermissions, DeviceRevocationRecord, DeviceType, QrPairingPayload,
    TrustedDevice,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub struct PairingManager {
    storage_dir: PathBuf,
}

impl PairingManager {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let storage_dir = home.join(".mindset_data").join("connect");
        let _ = fs::create_dir_all(&storage_dir);
        Self { storage_dir }
    }

    fn trusted_devices_path(&self) -> PathBuf {
        self.storage_dir.join("trusted_devices.json")
    }

    /// Generate ephemeral QR code pairing payload with 10-minute expiry
    pub fn generate_qr_payload(&self, host_name: &str) -> QrPairingPayload {
        let session_id = format!("ses-{}", Uuid::new_v4().simple());
        let challenge = format!("chn-{}", Uuid::new_v4());
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(10);

        // Determine local IP addresses (defaults to standard Wi-Fi LAN address)
        let local_ips = vec![
            "192.168.1.105".to_string(),
            "100.64.0.42".to_string(), // Tailscale / WireGuard fallback IP
        ];

        QrPairingPayload {
            version: 1,
            session_id,
            host_device_id: "mindset-host-laptop-01".to_string(),
            host_device_name: host_name.to_string(),
            public_key_ed25519: "ed25519:pub:7c9e8b1a4f0263d8...msos_master".to_string(),
            local_ip_addresses: local_ips,
            port: 52431,
            one_time_challenge: challenge,
            expires_at,
        }
    }

    /// List all trusted devices registered on this host
    pub fn list_trusted_devices(&self) -> Vec<TrustedDevice> {
        let path = self.trusted_devices_path();
        if !path.exists() {
            // Seed with sample paired mobile device for desktop preview
            let sample_devices = vec![
                TrustedDevice {
                    device_id: "dev-phone-galaxy-s24".to_string(),
                    device_name: "Logesh's Phone (Galaxy S24)".to_string(),
                    device_type: DeviceType::Mobile,
                    public_key_ed25519: "ed25519:pub:phone:9f42b38c...msos_phone".to_string(),
                    ip_address: "192.168.1.112".to_string(),
                    connection_mode: ConnectionMode::LocalWifi,
                    is_online: true,
                    permissions: DevicePermissions::default(),
                    paired_at: Utc::now() - chrono::Duration::days(14),
                    last_seen_at: Utc::now() - chrono::Duration::seconds(45),
                    is_revoked: false,
                },
                TrustedDevice {
                    device_id: "dev-tablet-tab-s9".to_string(),
                    device_name: "Study Tablet (Galaxy Tab S9)".to_string(),
                    device_type: DeviceType::Tablet,
                    public_key_ed25519: "ed25519:pub:tab:1d87e44a...msos_tab".to_string(),
                    ip_address: "100.64.0.88".to_string(),
                    connection_mode: ConnectionMode::TailscaleWireguard,
                    is_online: true,
                    permissions: DevicePermissions::default(),
                    paired_at: Utc::now() - chrono::Duration::days(3),
                    last_seen_at: Utc::now() - chrono::Duration::minutes(12),
                    is_revoked: false,
                },
            ];
            let _ = self.save_trusted_devices(&sample_devices);
            return sample_devices;
        }

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }

    /// Save trusted devices list
    pub fn save_trusted_devices(&self, devices: &[TrustedDevice]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(devices).map_err(|e| e.to_string())?;
        fs::write(self.trusted_devices_path(), json).map_err(|e| e.to_string())
    }

    /// Approve and register a new paired client device
    pub fn approve_device(
        &self,
        device_id: &str,
        device_name: &str,
        device_type: DeviceType,
        public_key: &str,
        ip_address: &str,
        connection_mode: ConnectionMode,
    ) -> Result<TrustedDevice, String> {
        let mut devices = self.list_trusted_devices();
        // Remove existing device with same id if any
        devices.retain(|d| d.device_id != device_id);

        let new_device = TrustedDevice {
            device_id: device_id.to_string(),
            device_name: device_name.to_string(),
            device_type,
            public_key_ed25519: public_key.to_string(),
            ip_address: ip_address.to_string(),
            connection_mode,
            is_online: true,
            permissions: DevicePermissions::default(),
            paired_at: Utc::now(),
            last_seen_at: Utc::now(),
            is_revoked: false,
        };

        devices.push(new_device.clone());
        self.save_trusted_devices(&devices)?;
        Ok(new_device)
    }

    /// One-click device revocation
    pub fn revoke_device(
        &self,
        device_id: &str,
        reason: &str,
    ) -> Result<DeviceRevocationRecord, String> {
        let mut devices = self.list_trusted_devices();
        let mut found = false;
        for d in devices.iter_mut() {
            if d.device_id == device_id {
                d.is_revoked = true;
                d.is_online = false;
                found = true;
            }
        }

        if !found {
            return Err(format!(
                "Device {} not found in trusted registry",
                device_id
            ));
        }

        self.save_trusted_devices(&devices)?;

        let record = DeviceRevocationRecord {
            device_id: device_id.to_string(),
            revoked_at: Utc::now(),
            reason: reason.to_string(),
            revoke_remote_tokens: true,
        };

        // Append to revocations log
        let revocations_file = self.storage_dir.join("revocations.jsonl");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&revocations_file)
        {
            if let Ok(line) = serde_json::to_string(&record) {
                let _ = writeln!(file, "{}", line);
            }
        }

        Ok(record)
    }
}
