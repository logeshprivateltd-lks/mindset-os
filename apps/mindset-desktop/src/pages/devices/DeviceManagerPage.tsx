/**
 * MindSet OS — OS Hardware & Device Manager (/devices)
 * Hardware Topology, External USB Wi-Fi Diagnostics, Power Governors & Media Fallback Routing
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { Button } from '@mindset/ui-core';
import { deviceClient } from '../../lib/deviceClient';
import type {
  PeripheralReport,
  StorageDrive,
  DiskCleanupReport,
  NetworkDiagnosticsReport,
  DriverResetResult,
  PowerThermalReport,
  GovernorProfileType,
  MediaRoutingReport,
} from '@mindset/db-schema';

type DeviceTab = 'topology' | 'network' | 'power' | 'media';

export const DeviceManagerPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<DeviceTab>('topology');
  const [loading, setLoading] = useState<boolean>(true);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  // Subsystem Data States
  const [peripherals, setPeripherals] = useState<PeripheralReport | null>(null);
  const [networkReport, setNetworkReport] = useState<NetworkDiagnosticsReport | null>(null);
  const [powerReport, setPowerReport] = useState<PowerThermalReport | null>(null);
  const [mediaReport, setMediaReport] = useState<MediaRoutingReport | null>(null);

  // Interactive Diagnostic States
  const [isResettingDriver, setIsResettingDriver] = useState<boolean>(false);
  const [driverResetResult, setDriverResetResult] = useState<DriverResetResult | null>(null);
  const [diskCleanupReport, setDiskCleanupReport] = useState<DiskCleanupReport | null>(null);
  const [isCleaningDisk, setIsCleaningDisk] = useState<boolean>(false);
  const [showCleanupModal, setShowCleanupModal] = useState<boolean>(false);
  const [hotspotActive, setHotspotActive] = useState<boolean>(false);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3500);
  };

  // Asynchronous Hardware Probe
  const loadHardwareData = useCallback(async () => {
    setLoading(true);
    try {
      const [peri, net, pwr, med] = await Promise.all([
        deviceClient.getPeripherals(),
        deviceClient.getNetworkDiagnostics(),
        deviceClient.getPowerStatus(),
        deviceClient.getMediaRouting(),
      ]);
      setPeripherals(peri);
      setNetworkReport(net);
      setPowerReport(pwr);
      setMediaReport(med);
      setHotspotActive(net.adapters[0]?.hotspot_active ?? false);
    } catch (err) {
      console.error('[DeviceManager] Failed to probe hardware:', err);
      showToast('Error probing hardware sensors');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadHardwareData();
  }, [loadHardwareData]);

  // Safe Eject Handler
  const handleEjectDrive = async (drive: StorageDrive) => {
    if (drive.is_system) {
      showToast('Cannot eject primary system drive');
      return;
    }
    try {
      const msg = await deviceClient.ejectDrive(drive.mount_point);
      showToast(msg);
      await loadHardwareData();
    } catch (err) {
      showToast(`Eject failed: ${String(err)}`);
    }
  };

  // Disk Cleanup Trigger
  const handleTriggerDiskCleanup = async (executeClean: boolean) => {
    setIsCleaningDisk(true);
    try {
      const rep = await deviceClient.triggerDiskCleanup('C:\\', executeClean);
      setDiskCleanupReport(rep);
      if (executeClean) {
        showToast(`Freed ${(rep.total_cleanable_bytes / (1024 * 1024)).toFixed(0)} MB of disk space`);
        await loadHardwareData();
      } else {
        setShowCleanupModal(true);
      }
    } catch (err) {
      showToast(`Disk cleanup failed: ${String(err)}`);
    } finally {
      setIsCleaningDisk(false);
    }
  };

  // One-Click Wi-Fi Driver Reset
  const handleResetWifiDriver = async () => {
    const adapterName = networkReport?.primary_adapter ?? 'Wi-Fi (TP-Link Archer T2U Plus)';
    setIsResettingDriver(true);
    setDriverResetResult(null);
    try {
      const res = await deviceClient.resetWifiDriver(adapterName);
      setDriverResetResult(res);
      showToast(res.message);
      await loadHardwareData();
    } catch (err) {
      showToast(`Driver reset failed: ${String(err)}`);
    } finally {
      setIsResettingDriver(false);
    }
  };

  // Mobile Hotspot Toggle
  const handleToggleHotspot = async () => {
    const nextState = !hotspotActive;
    try {
      const ok = await deviceClient.toggleHotspot(nextState);
      setHotspotActive(ok);
      showToast(ok ? 'MindSet Connect Hotspot Enabled' : 'Hotspot Disabled');
    } catch (err) {
      showToast(`Hotspot toggle failed: ${String(err)}`);
    }
  };

  // Power Governor Profile Switch
  const handleSetGovernorProfile = async (profile: GovernorProfileType) => {
    try {
      const rep = await deviceClient.setPowerProfile(profile);
      setPowerReport(rep);
      showToast(`Power governor switched to: ${rep.display_name}`);
    } catch (err) {
      showToast(`Failed to switch power profile: ${String(err)}`);
    }
  };

  // Media Device Switch
  const handleSelectMediaDevice = async (kind: string, deviceId: string) => {
    try {
      const rep = await deviceClient.selectMediaDevice(kind, deviceId);
      setMediaReport(rep);
      showToast('Media routing updated successfully');
    } catch (err) {
      showToast(`Failed to route device: ${String(err)}`);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#07090D] text-white selection:bg-[#2563EB]/40 font-sans overflow-hidden">
      {/* --------------------------------------------------------------------- */}
      {/* Toast Notification Bar                                                */}
      {/* --------------------------------------------------------------------- */}
      {toastMessage && (
        <div className="fixed top-5 right-5 z-50 px-4 py-2.5 rounded-xl bg-[#1E293B]/95 text-white text-xs font-medium border border-cyan-500/30 shadow-2xl backdrop-blur-md flex items-center space-x-2 animate-fade-in">
          <span className="w-2 h-2 rounded-full bg-cyan-400 animate-ping" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* --------------------------------------------------------------------- */}
      {/* Header Bar                                                            */}
      {/* --------------------------------------------------------------------- */}
      <div className="flex-none px-6 py-4 border-b border-white/5 bg-[#0B0F17]/80 backdrop-blur-xl flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-cyan-600/30 to-blue-600/30 border border-cyan-500/30 flex items-center justify-center text-xl shadow-inner">
            🔌
          </div>
          <div>
            <h1 className="text-lg font-semibold tracking-tight text-white flex items-center space-x-2">
              <span>Hardware & Device Manager</span>
              <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
                PROBED ASYNC
              </span>
            </h1>
            <p className="text-xs text-white/50">
              Peripherals, TP-Link external Wi-Fi driver, thermal governors & media fallback bridge
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-2.5">
          <Button
            variant="secondary"
            size="sm"
            onClick={loadHardwareData}
            disabled={loading}
            className="text-xs border-white/10 hover:border-white/20 bg-white/5"
          >
            {loading ? 'Probing...' : '🔄 Refresh Hardware'}
          </Button>
          <Button
            variant="secondary"
            size="sm"
            onClick={() => handleTriggerDiskCleanup(false)}
            disabled={isCleaningDisk}
            className="text-xs border-cyan-500/30 hover:border-cyan-500/50 bg-cyan-500/10 text-cyan-300"
          >
            🧹 Disk Cleanup
          </Button>
        </div>
      </div>

      {/* --------------------------------------------------------------------- */}
      {/* Navigation Tabs Bar                                                   */}
      {/* --------------------------------------------------------------------- */}
      <div className="flex-none px-6 pt-3 border-b border-white/5 bg-[#0B0F17]/40 flex space-x-2">
        <button
          onClick={() => setActiveTab('topology')}
          className={`px-4 py-2.5 text-xs font-medium rounded-t-xl transition-all flex items-center space-x-2 border-t border-x ${
            activeTab === 'topology'
              ? 'bg-[#121824] text-cyan-300 border-white/10 border-b-transparent shadow-lg'
              : 'text-white/60 hover:text-white border-transparent'
          }`}
        >
          <span>🖥️</span>
          <span>Hardware Topology Map</span>
        </button>

        <button
          onClick={() => setActiveTab('network')}
          className={`px-4 py-2.5 text-xs font-medium rounded-t-xl transition-all flex items-center space-x-2 border-t border-x ${
            activeTab === 'network'
              ? 'bg-[#121824] text-cyan-300 border-white/10 border-b-transparent shadow-lg'
              : 'text-white/60 hover:text-white border-transparent'
          }`}
        >
          <span>📶</span>
          <span>Wi-Fi & Driver Troubleshooter</span>
          {networkReport?.is_external_wifi_detected && (
            <span className="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse" />
          )}
        </button>

        <button
          onClick={() => setActiveTab('power')}
          className={`px-4 py-2.5 text-xs font-medium rounded-t-xl transition-all flex items-center space-x-2 border-t border-x ${
            activeTab === 'power'
              ? 'bg-[#121824] text-cyan-300 border-white/10 border-b-transparent shadow-lg'
              : 'text-white/60 hover:text-white border-transparent'
          }`}
        >
          <span>⚡</span>
          <span>Power & Thermal Governor</span>
          <span className="text-[10px] font-mono px-1.5 py-0.2 rounded bg-white/10 text-white/70">
            {powerReport?.cpu_temp_celsius ?? 48}°C
          </span>
        </button>

        <button
          onClick={() => setActiveTab('media')}
          className={`px-4 py-2.5 text-xs font-medium rounded-t-xl transition-all flex items-center space-x-2 border-t border-x ${
            activeTab === 'media'
              ? 'bg-[#121824] text-cyan-300 border-white/10 border-b-transparent shadow-lg'
              : 'text-white/60 hover:text-white border-transparent'
          }`}
        >
          <span>🎙️</span>
          <span>Audio & Camera Fallbacks</span>
          {mediaReport?.mic_fallback_active && (
            <span className="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 border border-amber-500/30">
              Bridge Active
            </span>
          )}
        </button>
      </div>

      {/* --------------------------------------------------------------------- */}
      {/* Tab 1: Hardware Topology Map                                          */}
      {/* --------------------------------------------------------------------- */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {activeTab === 'topology' && (
          <div className="space-y-6">
            {/* Quick Status Bar */}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5 backdrop-blur-xl">
                <span className="text-[11px] text-white/50 uppercase tracking-wider block font-mono">
                  Displays Connected
                </span>
                <div className="mt-1 flex items-baseline justify-between">
                  <span className="text-2xl font-semibold font-mono tabular-nums text-white">
                    {peripherals?.displays.length ?? 1}
                  </span>
                  <span className="text-xs text-cyan-400 bg-cyan-400/10 px-2 py-0.5 rounded-full border border-cyan-400/20">
                    Liquid Retina
                  </span>
                </div>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5 backdrop-blur-xl">
                <span className="text-[11px] text-white/50 uppercase tracking-wider block font-mono">
                  Storage Volumes
                </span>
                <div className="mt-1 flex items-baseline justify-between">
                  <span className="text-2xl font-semibold font-mono tabular-nums text-white">
                    {peripherals?.storage_drives.length ?? 1}
                  </span>
                  <span className="text-xs text-emerald-400 bg-emerald-400/10 px-2 py-0.5 rounded-full border border-emerald-400/20">
                    Health Optimal
                  </span>
                </div>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5 backdrop-blur-xl">
                <span className="text-[11px] text-white/50 uppercase tracking-wider block font-mono">
                  USB Devices
                </span>
                <div className="mt-1 flex items-baseline justify-between">
                  <span className="text-2xl font-semibold font-mono tabular-nums text-white">
                    {peripherals?.usb_devices.length ?? 4}
                  </span>
                  <span className="text-xs text-blue-400 bg-blue-400/10 px-2 py-0.5 rounded-full border border-blue-400/20">
                    USB 3.0 High-Speed
                  </span>
                </div>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5 backdrop-blur-xl">
                <span className="text-[11px] text-white/50 uppercase tracking-wider block font-mono">
                  Thermal Sensor
                </span>
                <div className="mt-1 flex items-baseline justify-between">
                  <span className="text-2xl font-semibold font-mono tabular-nums text-white">
                    {powerReport?.cpu_temp_celsius ?? 48.5}°C
                  </span>
                  <span className="text-xs text-emerald-400 bg-emerald-400/10 px-2 py-0.5 rounded-full border border-emerald-400/20">
                    Whisper Fan
                  </span>
                </div>
              </div>
            </div>

            {/* Displays Card */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <h2 className="text-sm font-semibold text-white/90 mb-3 flex items-center space-x-2">
                <span>🖥️</span>
                <span>Monitors & Display Output</span>
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {peripherals?.displays.map((disp) => (
                  <div
                    key={disp.id}
                    className="p-4 rounded-xl bg-white/[0.03] border border-white/5 hover:border-cyan-500/30 transition-all flex items-start justify-between"
                  >
                    <div>
                      <div className="flex items-center space-x-2">
                        <span className="font-medium text-sm text-white">{disp.name}</span>
                        {disp.is_primary && (
                          <span className="text-[10px] px-1.5 py-0.5 rounded bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 font-mono">
                            PRIMARY
                          </span>
                        )}
                      </div>
                      <p className="text-xs text-white/50 mt-1 font-mono tabular-nums">
                        {disp.resolution} • {disp.refresh_rate_hz} Hz
                      </p>
                      <span className="inline-block mt-2 text-[10px] text-white/40 bg-white/5 px-2 py-0.5 rounded">
                        Interface: {disp.connection_type}
                      </span>
                    </div>
                    <span className="text-xs text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded-full border border-emerald-500/20 font-mono">
                      Active
                    </span>
                  </div>
                ))}
              </div>
            </div>

            {/* Storage Partitions & Volumes */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <div className="flex items-center justify-between mb-3">
                <h2 className="text-sm font-semibold text-white/90 flex items-center space-x-2">
                  <span>💾</span>
                  <span>Disk Storage & Partitions</span>
                </h2>
                <Button
                  variant="secondary"
                  size="sm"
                  onClick={() => handleTriggerDiskCleanup(false)}
                  className="text-xs border-white/10 hover:border-white/20 bg-white/5"
                >
                  Analyze Cleanable Space
                </Button>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {peripherals?.storage_drives.map((drive) => {
                  const totalGb = (drive.total_bytes / (1024 * 1024 * 1024)).toFixed(1);
                  const availGb = (drive.available_bytes / (1024 * 1024 * 1024)).toFixed(1);
                  const usedGb = (drive.used_bytes / (1024 * 1024 * 1024)).toFixed(1);

                  return (
                    <div
                      key={drive.id}
                      className="p-4 rounded-xl bg-white/[0.03] border border-white/5 flex flex-col justify-between"
                    >
                      <div>
                        <div className="flex items-center justify-between">
                          <span className="font-medium text-sm text-white">{drive.name}</span>
                          {drive.is_system ? (
                            <span className="text-[10px] px-2 py-0.5 rounded bg-blue-500/20 text-blue-300 border border-blue-500/30 font-mono">
                              SYSTEM ({drive.fs_type})
                            </span>
                          ) : (
                            <Button
                              variant="secondary"
                              size="sm"
                              onClick={() => handleEjectDrive(drive)}
                              className="text-[10px] py-0.5 px-2 border-amber-500/30 text-amber-300 hover:bg-amber-500/10"
                            >
                              ⏏️ Safe Eject
                            </Button>
                          )}
                        </div>

                        <p className="text-xs text-white/50 mt-1 font-mono tabular-nums">
                          Mount: {drive.mount_point} • {availGb} GB free of {totalGb} GB
                        </p>

                        {/* Progress Bar */}
                        <div className="mt-3 w-full h-2 rounded-full bg-white/5 overflow-hidden">
                          <div
                            className={`h-full rounded-full transition-all ${
                              drive.used_percent > 85 ? 'bg-rose-500' : 'bg-cyan-500'
                            }`}
                            style={{ width: `${Math.min(100, drive.used_percent)}%` }}
                          />
                        </div>
                      </div>

                      <div className="mt-3 pt-2 border-t border-white/5 flex items-center justify-between text-[11px] text-white/40 font-mono">
                        <span>Used: {usedGb} GB ({drive.used_percent.toFixed(1)}%)</span>
                        <span>{drive.is_removable ? 'Removable Drive' : 'NVMe Solid-State'}</span>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Connected USB Peripherals */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <h2 className="text-sm font-semibold text-white/90 mb-3 flex items-center space-x-2">
                <span>🔌</span>
                <span>Connected USB Peripherals & Controllers</span>
              </h2>

              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                {peripherals?.usb_devices.map((dev) => (
                  <div
                    key={dev.id}
                    className="p-3.5 rounded-xl bg-white/[0.03] border border-white/5 hover:border-cyan-500/20 transition-all"
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex items-center space-x-2">
                        <span className="text-base">
                          {dev.device_type === 'network'
                            ? '📶'
                            : dev.device_type === 'keyboard'
                            ? '⌨️'
                            : dev.device_type === 'mouse'
                            ? '🖱️'
                            : dev.device_type === 'storage'
                            ? '💾'
                            : dev.device_type === 'printer'
                            ? '🖨️'
                            : '🔌'}
                        </span>
                        <span className="text-xs font-medium text-white truncate max-w-[180px]">
                          {dev.name}
                        </span>
                      </div>
                      <span className="text-[10px] text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded font-mono">
                        {dev.status}
                      </span>
                    </div>

                    <div className="mt-2 text-[10px] text-white/40 font-mono space-y-0.5">
                      <div>Port: {dev.connected_port ?? 'USB 3.0'}</div>
                      {dev.vendor_id && dev.product_id && (
                        <div>
                          VID:PID {dev.vendor_id}:{dev.product_id}
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* ------------------------------------------------------------------- */}
        {/* Tab 2: Wi-Fi & Network Diagnostics (TP-Link rtl88xxau)              */}
        {/* ------------------------------------------------------------------- */}
        {activeTab === 'network' && (
          <div className="space-y-6">
            {/* Primary Wi-Fi Banner */}
            <div className="p-6 rounded-2xl bg-gradient-to-r from-cyan-950/40 via-[#0F1420] to-[#0F1420] border border-cyan-500/20 backdrop-blur-xl relative overflow-hidden">
              <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div className="space-y-2">
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-mono px-2 py-0.5 rounded-full bg-cyan-500/20 text-cyan-300 border border-cyan-500/30">
                      EXTERNAL USB WI-FI
                    </span>
                    <span className="text-xs font-mono px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                      rtl88xxau DKMS ACTIVE
                    </span>
                  </div>
                  <h2 className="text-xl font-semibold text-white tracking-tight">
                    {networkReport?.adapters[0]?.adapter_name ?? 'Wi-Fi (TP-Link Archer T2U Plus)'}
                  </h2>
                  <p className="text-xs text-white/60 max-w-xl">
                    {networkReport?.adapters[0]?.chipset ?? 'Realtek RTL8811AU / RTL8821AU High Gain Wireless'}
                  </p>
                </div>

                <div className="flex items-center space-x-3">
                  <Button
                    variant="primary"
                    size="sm"
                    onClick={handleResetWifiDriver}
                    disabled={isResettingDriver}
                    className="bg-cyan-500 hover:bg-cyan-400 text-black font-semibold text-xs shadow-lg shadow-cyan-500/20"
                  >
                    {isResettingDriver ? 'Restarting Stack...' : '⚡ Reset Wi-Fi Driver'}
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={handleToggleHotspot}
                    className={`text-xs border-white/10 ${
                      hotspotActive ? 'bg-emerald-500/20 text-emerald-300 border-emerald-500/30' : 'bg-white/5'
                    }`}
                  >
                    {hotspotActive ? '📡 Hotspot On' : '📡 Enable Hotspot'}
                  </Button>
                </div>
              </div>

              {/* Signal & Metrics Grid */}
              <div className="mt-6 pt-5 border-t border-white/5 grid grid-cols-2 sm:grid-cols-4 gap-4">
                <div>
                  <span className="text-[11px] text-white/40 font-mono block">Signal Strength</span>
                  <div className="mt-1 flex items-baseline space-x-1.5">
                    <span className="text-xl font-bold font-mono tabular-nums text-white">
                      {networkReport?.adapters[0]?.signal_strength_dbm ?? -54}
                    </span>
                    <span className="text-xs text-white/50">dBm</span>
                  </div>
                  <span className="text-[10px] text-emerald-400">Excellent Reception</span>
                </div>

                <div>
                  <span className="text-[11px] text-white/40 font-mono block">Link Quality</span>
                  <div className="mt-1 flex items-baseline space-x-1.5">
                    <span className="text-xl font-bold font-mono tabular-nums text-cyan-400">
                      {networkReport?.adapters[0]?.link_quality_percent ?? 88}%
                    </span>
                  </div>
                  <span className="text-[10px] text-white/50">5 GHz AC Channel</span>
                </div>

                <div>
                  <span className="text-[11px] text-white/40 font-mono block">Gateway & DNS Ping</span>
                  <div className="mt-1 flex items-baseline space-x-1.5">
                    <span className="text-xl font-bold font-mono tabular-nums text-emerald-400">
                      {networkReport?.latency_ms ?? 18}
                    </span>
                    <span className="text-xs text-white/50">ms</span>
                  </div>
                  <span className="text-[10px] text-emerald-400">Zero Packet Loss</span>
                </div>

                <div>
                  <span className="text-[11px] text-white/40 font-mono block">IP Address</span>
                  <div className="mt-1 font-mono text-sm text-white tabular-nums">
                    {networkReport?.adapters[0]?.ip_address ?? '192.168.1.142'}
                  </div>
                  <span className="text-[10px] text-white/40 font-mono">
                    DNS: {networkReport?.adapters[0]?.dns_servers.join(', ') || '1.1.1.1'}
                  </span>
                </div>
              </div>
            </div>

            {/* Diagnostic Logs & Troubleshooting Wizard */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl space-y-4">
              <h3 className="text-sm font-semibold text-white/90 flex items-center space-x-2">
                <span>🛠️</span>
                <span>Automatic Wi-Fi Diagnostic & Action Recommendations</span>
              </h3>

              <div className="space-y-2">
                {networkReport?.suggested_actions.map((act, idx) => (
                  <div
                    key={idx}
                    className="p-3 rounded-xl bg-white/[0.03] border border-white/5 flex items-center space-x-3 text-xs text-white/80"
                  >
                    <span className="text-cyan-400">✓</span>
                    <span>{act}</span>
                  </div>
                ))}
              </div>

              {driverResetResult && (
                <div className="mt-4 p-4 rounded-xl bg-cyan-950/30 border border-cyan-500/30 space-y-2">
                  <div className="flex items-center justify-between text-xs text-cyan-300 font-semibold">
                    <span>Diagnostic Action Log:</span>
                    <span>Status: {driverResetResult.success ? 'Success' : 'Requires Elevation'}</span>
                  </div>
                  <ul className="text-xs font-mono text-white/70 space-y-1 list-disc list-inside">
                    {driverResetResult.steps_taken.map((s, i) => (
                      <li key={i}>{s}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>
        )}

        {/* ------------------------------------------------------------------- */}
        {/* Tab 3: Power, Battery & Thermal Governor                            */}
        {/* ------------------------------------------------------------------- */}
        {activeTab === 'power' && (
          <div className="space-y-6">
            {/* Governor Profile Selector */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <h2 className="text-sm font-semibold text-white/90 mb-3 flex items-center space-x-2">
                <span>⚡</span>
                <span>Active Power & Thermal Governor Profile</span>
              </h2>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                {/* Eco Study Mode */}
                <div
                  onClick={() => handleSetGovernorProfile('EcoStudy')}
                  className={`p-4 rounded-xl border cursor-pointer transition-all ${
                    powerReport?.active_profile === 'EcoStudy'
                      ? 'bg-emerald-950/40 border-emerald-500/40 shadow-lg shadow-emerald-950/20'
                      : 'bg-white/[0.02] border-white/5 hover:border-white/10'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <span className="font-semibold text-sm text-white flex items-center space-x-2">
                      <span>🍃</span>
                      <span>Eco Study Mode</span>
                    </span>
                    {powerReport?.active_profile === 'EcoStudy' && (
                      <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300">
                        ACTIVE
                      </span>
                    )}
                  </div>
                  <p className="text-xs text-white/60 mt-2">
                    Whisper-silent fans, battery extended up to 6.5 hours. CPU throttled at 65% for calm reading & notes.
                  </p>
                  <div className="mt-3 text-[11px] font-mono text-emerald-400">
                    Acoustics: 1200 RPM • Temp: &lt;45°C
                  </div>
                </div>

                {/* Balanced Mode */}
                <div
                  onClick={() => handleSetGovernorProfile('Balanced')}
                  className={`p-4 rounded-xl border cursor-pointer transition-all ${
                    powerReport?.active_profile === 'Balanced'
                      ? 'bg-cyan-950/40 border-cyan-500/40 shadow-lg shadow-cyan-950/20'
                      : 'bg-white/[0.02] border-white/5 hover:border-white/10'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <span className="font-semibold text-sm text-white flex items-center space-x-2">
                      <span>⚖️</span>
                      <span>Balanced Mode</span>
                    </span>
                    {powerReport?.active_profile === 'Balanced' && (
                      <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-500/20 text-cyan-300">
                        ACTIVE
                      </span>
                    )}
                  </div>
                  <p className="text-xs text-white/60 mt-2">
                    Dynamic hardware scaling for responsive multitasking, browser tabs, and desktop navigation.
                  </p>
                  <div className="mt-3 text-[11px] font-mono text-cyan-400">
                    Acoustics: 2100 RPM • Temp: ~50°C
                  </div>
                </div>

                {/* AI Boost Mode */}
                <div
                  onClick={() => handleSetGovernorProfile('AiBoost')}
                  className={`p-4 rounded-xl border cursor-pointer transition-all ${
                    powerReport?.active_profile === 'AiBoost'
                      ? 'bg-purple-950/40 border-purple-500/40 shadow-lg shadow-purple-950/20'
                      : 'bg-white/[0.02] border-white/5 hover:border-white/10'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <span className="font-semibold text-sm text-white flex items-center space-x-2">
                      <span>🚀</span>
                      <span>AI Boost Mode</span>
                    </span>
                    {powerReport?.active_profile === 'AiBoost' && (
                      <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-purple-500/20 text-purple-300">
                        ACTIVE
                      </span>
                    )}
                  </div>
                  <p className="text-xs text-white/60 mt-2">
                    Maximum CPU/GPU threads uncapped for high-token Ollama local AI tutoring and RAG embedding.
                  </p>
                  <div className="mt-3 text-[11px] font-mono text-purple-400">
                    Active Cooling: 3600 RPM • Max Compute
                  </div>
                </div>
              </div>
            </div>

            {/* Live Sensors Grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5">
                <span className="text-[11px] text-white/50 font-mono uppercase">Battery Capacity</span>
                <div className="mt-1 flex items-baseline space-x-2">
                  <span className="text-2xl font-bold font-mono tabular-nums text-white">
                    {powerReport?.battery_percent ?? 84}%
                  </span>
                  <span className="text-xs text-emerald-400">
                    {powerReport?.is_charging ? '⚡ Charging' : 'On Battery'}
                  </span>
                </div>
                <p className="text-xs text-white/40 mt-1 font-mono tabular-nums">
                  ~{powerReport?.time_remaining_minutes ?? 260} mins remaining
                </p>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5">
                <span className="text-[11px] text-white/50 font-mono uppercase">CPU Temperature</span>
                <div className="mt-1 flex items-baseline space-x-2">
                  <span
                    className={`text-2xl font-bold font-mono tabular-nums ${
                      (powerReport?.cpu_temp_celsius ?? 50) > 75
                        ? 'text-rose-400'
                        : (powerReport?.cpu_temp_celsius ?? 50) > 55
                        ? 'text-amber-400'
                        : 'text-emerald-400'
                    }`}
                  >
                    {powerReport?.cpu_temp_celsius ?? 48.5}°C
                  </span>
                </div>
                <p className="text-xs text-white/40 mt-1">Thermal throttle: Inactive</p>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5">
                <span className="text-[11px] text-white/50 font-mono uppercase">Fan Acoustics</span>
                <div className="mt-1 flex items-baseline space-x-2">
                  <span className="text-2xl font-bold font-mono tabular-nums text-white">
                    {powerReport?.fan_speed_rpm ?? 1950}
                  </span>
                  <span className="text-xs text-white/50">RPM</span>
                </div>
                <p className="text-xs text-white/40 mt-1">PWM Stepped Curve</p>
              </div>

              <div className="p-4 rounded-2xl bg-[#0F1420]/80 border border-white/5">
                <span className="text-[11px] text-white/50 font-mono uppercase">Battery Health</span>
                <div className="mt-1 flex items-baseline space-x-2">
                  <span className="text-2xl font-bold font-mono tabular-nums text-emerald-400">
                    {powerReport?.battery_health_percent ?? 94.2}%
                  </span>
                </div>
                <p className="text-xs text-white/40 mt-1">54,200 mWh / 57,500 mWh</p>
              </div>
            </div>
          </div>
        )}

        {/* ------------------------------------------------------------------- */}
        {/* Tab 4: Media & Hardware Fallbacks                                   */}
        {/* ------------------------------------------------------------------- */}
        {activeTab === 'media' && (
          <div className="space-y-6">
            {/* Fallback Banners */}
            <div className="p-5 rounded-2xl bg-amber-950/20 border border-amber-500/30 backdrop-blur-xl space-y-3">
              <h2 className="text-sm font-semibold text-amber-300 flex items-center space-x-2">
                <span>🛡️</span>
                <span>Automatic Hardware Fallback Routing Status</span>
              </h2>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                <div className="p-3 rounded-xl bg-white/[0.03] border border-amber-500/20">
                  <div className="text-[11px] text-white/50 font-mono uppercase">Microphone Routing</div>
                  <div className="mt-1 text-xs font-medium text-amber-200">
                    {mediaReport?.mic_fallback_label}
                  </div>
                </div>

                <div className="p-3 rounded-xl bg-white/[0.03] border border-amber-500/20">
                  <div className="text-[11px] text-white/50 font-mono uppercase">Camera / Vision Routing</div>
                  <div className="mt-1 text-xs font-medium text-amber-200">
                    {mediaReport?.camera_fallback_label}
                  </div>
                </div>

                <div className="p-3 rounded-xl bg-white/[0.03] border border-amber-500/20">
                  <div className="text-[11px] text-white/50 font-mono uppercase">Wireless Peripherals</div>
                  <div className="mt-1 text-xs font-medium text-amber-200">
                    {mediaReport?.bluetooth_fallback_label}
                  </div>
                </div>
              </div>
            </div>

            {/* Audio Inputs */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <h3 className="text-sm font-semibold text-white/90 mb-3 flex items-center space-x-2">
                <span>🎙️</span>
                <span>Audio Input (Microphones & Voice Typing)</span>
              </h3>
              <div className="space-y-2">
                {mediaReport?.audio_inputs.map((mic) => (
                  <div
                    key={mic.id}
                    onClick={() => handleSelectMediaDevice('audio_input', mic.id)}
                    className={`p-3.5 rounded-xl border cursor-pointer transition-all flex items-center justify-between ${
                      mic.is_default
                        ? 'bg-cyan-950/30 border-cyan-500/40 text-cyan-200'
                        : 'bg-white/[0.02] border-white/5 hover:border-white/10 text-white/80'
                    }`}
                  >
                    <div>
                      <div className="flex items-center space-x-2">
                        <span className="font-medium text-xs text-white">{mic.name}</span>
                        {mic.is_fallback_virtual && (
                          <span className="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 font-mono">
                            VIRTUAL FALLBACK
                          </span>
                        )}
                      </div>
                      <span className="text-[11px] text-white/40 font-mono">{mic.status}</span>
                    </div>
                    {mic.is_default && (
                      <span className="text-xs text-cyan-400 font-mono">✓ Active</span>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Video Inputs */}
            <div className="p-5 rounded-2xl bg-[#0F1420]/90 border border-white/10 backdrop-blur-xl">
              <h3 className="text-sm font-semibold text-white/90 mb-3 flex items-center space-x-2">
                <span>📹</span>
                <span>Video Inputs (Webcams & MindSet Connect Streams)</span>
              </h3>
              <div className="space-y-2">
                {mediaReport?.video_inputs.map((cam) => (
                  <div
                    key={cam.id}
                    onClick={() => handleSelectMediaDevice('video_input', cam.id)}
                    className={`p-3.5 rounded-xl border cursor-pointer transition-all flex items-center justify-between ${
                      cam.is_default
                        ? 'bg-cyan-950/30 border-cyan-500/40 text-cyan-200'
                        : 'bg-white/[0.02] border-white/5 hover:border-white/10 text-white/80'
                    }`}
                  >
                    <div>
                      <div className="flex items-center space-x-2">
                        <span className="font-medium text-xs text-white">{cam.name}</span>
                        {cam.is_fallback_virtual && (
                          <span className="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-300 font-mono">
                            CONNECT BRIDGE
                          </span>
                        )}
                      </div>
                      <span className="text-[11px] text-white/40 font-mono">{cam.status}</span>
                    </div>
                    {cam.is_default && (
                      <span className="text-xs text-cyan-400 font-mono">✓ Active</span>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* --------------------------------------------------------------------- */}
      {/* Disk Cleanup Slide-over Modal                                         */}
      {/* --------------------------------------------------------------------- */}
      {showCleanupModal && diskCleanupReport && (
        <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
          <div className="w-full max-w-md rounded-2xl bg-[#0F1420] border border-cyan-500/30 p-6 space-y-4 shadow-2xl">
            <div className="flex items-center justify-between">
              <h3 className="text-base font-semibold text-white flex items-center space-x-2">
                <span>🧹</span>
                <span>Disk Space Cleanup Analysis</span>
              </h3>
              <button
                onClick={() => setShowCleanupModal(false)}
                className="text-white/40 hover:text-white text-sm"
              >
                ✕
              </button>
            </div>

            <p className="text-xs text-white/60">
              Cleanable caches and obsolete files on drive{' '}
              <span className="font-mono text-cyan-400">{diskCleanupReport.drive_mount}</span>:
            </p>

            <div className="space-y-2.5">
              {diskCleanupReport.categories.map((cat, i) => (
                <div key={i} className="p-3 rounded-xl bg-white/[0.03] border border-white/5">
                  <div className="flex items-center justify-between text-xs font-medium text-white">
                    <span>{cat.name}</span>
                    <span className="font-mono text-cyan-400 tabular-nums">
                      {(cat.bytes / (1024 * 1024)).toFixed(0)} MB
                    </span>
                  </div>
                  <p className="text-[11px] text-white/40 mt-1">{cat.description}</p>
                </div>
              ))}
            </div>

            <div className="pt-2 border-t border-white/5 flex items-center justify-between text-xs text-white/70">
              <span>Total Reclaimable Space:</span>
              <span className="font-bold text-emerald-400 font-mono tabular-nums">
                {(diskCleanupReport.total_cleanable_bytes / (1024 * 1024)).toFixed(0)} MB
              </span>
            </div>

            <div className="flex items-center justify-end space-x-2 pt-2">
              <Button
                variant="secondary"
                size="sm"
                onClick={() => setShowCleanupModal(false)}
                className="text-xs"
              >
                Cancel
              </Button>
              <Button
                variant="primary"
                size="sm"
                onClick={() => {
                  setShowCleanupModal(false);
                  handleTriggerDiskCleanup(true);
                }}
                className="text-xs bg-cyan-500 hover:bg-cyan-400 text-black font-semibold"
              >
                Clean Now
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
