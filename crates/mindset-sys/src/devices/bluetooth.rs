//! MindSet OS — Bluetooth & Wireless Peripherals Manager
//! Manages Bluetooth LE and Classic peripherals: Headphones, Keyboards, Companion Phones, Projectors
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BtDeviceType {
    Headphones,
    Keyboard,
    Phone,
    ProjectorAudio,
    Mouse,
}

impl BtDeviceType {
    pub fn label(&self) -> &'static str {
        match self {
            BtDeviceType::Headphones => "Audio / ANC",
            BtDeviceType::Keyboard => "Input / HID",
            BtDeviceType::Phone => "Mobile / Link",
            BtDeviceType::ProjectorAudio => "Audio / Cast",
            BtDeviceType::Mouse => "Pointer / HID",
        }
    }

    pub fn icon_badge(&self) -> &'static str {
        match self {
            BtDeviceType::Headphones => "[HP]",
            BtDeviceType::Keyboard => "[KB]",
            BtDeviceType::Phone => "[PH]",
            BtDeviceType::ProjectorAudio => "[PR]",
            BtDeviceType::Mouse => "[MS]",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub device_type: BtDeviceType,
    pub is_connected: bool,
    pub battery_percent: Option<u8>,
    pub signal_rssi: i32,
    pub profile_info: String,
}

pub fn default_nearby_devices() -> Vec<BluetoothDevice> {
    vec![
        BluetoothDevice {
            id: "bt_stylus_05".to_string(),
            name: "LKS Smart Stylus Pen (BLE 5.2)".to_string(),
            device_type: BtDeviceType::Keyboard,
            is_connected: false,
            battery_percent: Some(95),
            signal_rssi: -42,
            profile_info: "Precision Stylus HID / Tilt & Pressure".to_string(),
        },
        BluetoothDevice {
            id: "bt_sony_06".to_string(),
            name: "Sony WH-1000XM5 ANC Headset".to_string(),
            device_type: BtDeviceType::Headphones,
            is_connected: false,
            battery_percent: Some(70),
            signal_rssi: -58,
            profile_info: "LDAC High-Resolution Audio / Dual Connect".to_string(),
        },
        BluetoothDevice {
            id: "bt_spk_07".to_string(),
            name: "Classroom Bluetooth Speaker".to_string(),
            device_type: BtDeviceType::ProjectorAudio,
            is_connected: false,
            battery_percent: None,
            signal_rssi: -72,
            profile_info: "Stereo Audio Sink (A2DP)".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothManager {
    pub enabled: bool,
    pub is_scanning: bool,
    pub scan_timer: f32,
    pub devices: Vec<BluetoothDevice>,
    #[serde(default = "default_nearby_devices")]
    pub nearby_devices: Vec<BluetoothDevice>,
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
            is_scanning: false,
            scan_timer: 0.0,
            devices: vec![
                BluetoothDevice {
                    id: "bt_hp_01".to_string(),
                    name: "Wireless Study Headphones (ANC)".to_string(),
                    device_type: BtDeviceType::Headphones,
                    is_connected: true,
                    battery_percent: Some(85),
                    signal_rssi: -52,
                    profile_info: "High-Fidelity A2DP / Low Latency SBC".to_string(),
                },
                BluetoothDevice {
                    id: "bt_kb_02".to_string(),
                    name: "External Mechanical Keyboard".to_string(),
                    device_type: BtDeviceType::Keyboard,
                    is_connected: true,
                    battery_percent: Some(92),
                    signal_rssi: -48,
                    profile_info: "Low-Latency HID over GATT (BLE)".to_string(),
                },
                BluetoothDevice {
                    id: "bt_ph_03".to_string(),
                    name: "Student Android Companion Phone".to_string(),
                    device_type: BtDeviceType::Phone,
                    is_connected: false,
                    battery_percent: Some(78),
                    signal_rssi: -65,
                    profile_info: "MindSet Connect Sync / Notification Bridge".to_string(),
                },
                BluetoothDevice {
                    id: "bt_pr_04".to_string(),
                    name: "Classroom Smart Projector Audio".to_string(),
                    device_type: BtDeviceType::ProjectorAudio,
                    is_connected: false,
                    battery_percent: None,
                    signal_rssi: -74,
                    profile_info: "Standard Audio Sink (A2DP)".to_string(),
                },
            ],
            nearby_devices: default_nearby_devices(),
        }
    }

    pub fn pair_nearby_device(&mut self, id: &str) -> bool {
        if let Some(pos) = self.nearby_devices.iter().position(|d| d.id == id) {
            let mut dev = self.nearby_devices.remove(pos);
            dev.is_connected = true;
            self.devices.push(dev);
            true
        } else {
            false
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
        if !self.enabled {
            self.is_scanning = false;
            for dev in &mut self.devices {
                dev.is_connected = false;
            }
        }
    }

    pub fn start_scan(&mut self) {
        if self.enabled {
            self.is_scanning = true;
            self.scan_timer = 4.0; // Scan for 4 seconds
        }
    }

    pub fn toggle_device_connection(&mut self, id: &str) -> bool {
        if !self.enabled {
            return false;
        }
        if let Some(dev) = self.devices.iter_mut().find(|d| d.id == id) {
            dev.is_connected = !dev.is_connected;
            dev.is_connected
        } else {
            false
        }
    }

    pub fn connected_count(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.devices.iter().filter(|d| d.is_connected).count()
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_scanning {
            self.scan_timer -= dt;
            if self.scan_timer <= 0.0 {
                self.is_scanning = false;
                self.scan_timer = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_manager_toggle_and_connection() {
        let mut mgr = BluetoothManager::new();
        assert!(mgr.enabled);
        assert_eq!(mgr.connected_count(), 2);

        // Toggle connection of phone
        let state = mgr.toggle_device_connection("bt_ph_03");
        assert!(state);
        assert_eq!(mgr.connected_count(), 3);

        // Disconnect headphones
        let state2 = mgr.toggle_device_connection("bt_hp_01");
        assert!(!state2);
        assert_eq!(mgr.connected_count(), 2);

        // Start scan
        mgr.start_scan();
        assert!(mgr.is_scanning);
        mgr.update(4.5);
        assert!(!mgr.is_scanning);

        // Disable Bluetooth
        mgr.toggle_enabled();
        assert!(!mgr.enabled);
        assert_eq!(mgr.connected_count(), 0);
    }

    #[test]
    fn test_nearby_device_pairing() {
        let mut mgr = BluetoothManager::new();
        let init_nearby_count = mgr.nearby_devices.len();
        let init_dev_count = mgr.devices.len();
        assert!(init_nearby_count >= 3);

        let paired = mgr.pair_nearby_device("bt_stylus_05");
        assert!(paired);
        assert_eq!(mgr.nearby_devices.len(), init_nearby_count - 1);
        assert_eq!(mgr.devices.len(), init_dev_count + 1);
        assert!(mgr
            .devices
            .iter()
            .any(|d| d.id == "bt_stylus_05" && d.is_connected));
    }
}
