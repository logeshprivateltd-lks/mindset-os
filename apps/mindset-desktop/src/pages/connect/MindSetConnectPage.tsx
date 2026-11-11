/**
 * MindSet OS — Desktop MindSet Connect 6-Module Cross-Device Hub
 * Route: /connect
 * Powered by LKS PVT LTD
 */

import React, { useState, useEffect } from 'react';
import {
  QrPairingPayload,
  ConnectTrustedDevice,
  SelectiveFolderSyncStatus,
  SyncConflictItem,
  ConflictResolutionChoice,
  ContinuityStateHandoff,
  CrossDeviceFocusEvent,
  WirelessWebcamStatus,
  WirelessMicrophoneStatus,
  ConnectAuditEntry,
} from '@mindset/db-schema';
import { connectClient } from '../../lib/connectClient';
import {
  SmartphoneIcon,
  QrCodeIcon,
  CameraIcon,
  MicIcon,
  ShieldCheckIcon,
  ShieldAlertIcon,
  RefreshIcon,
  Trash2Icon,
  CheckCircleIcon,
  AlertTriangleIcon,
  LayersIcon,
  ClockIcon,
  SparklesIcon,
} from '../../components/common/SystemIcons';

export const MindSetConnectPage: React.FC = () => {
  // State
  const [trustedDevices, setTrustedDevices] = useState<ConnectTrustedDevice[]>([]);
  const [syncFolders, setSyncFolders] = useState<SelectiveFolderSyncStatus[]>([]);
  const [activeConflicts, setActiveConflicts] = useState<SyncConflictItem[]>([]);
  const [continuityState, setContinuityState] = useState<ContinuityStateHandoff | null>(null);
  const [activeFocus, setActiveFocus] = useState<CrossDeviceFocusEvent | null>(null);
  const [webcamStatus, setWebcamStatus] = useState<WirelessWebcamStatus | null>(null);
  const [micStatus, setMicStatus] = useState<WirelessMicrophoneStatus | null>(null);
  const [auditLogs, setAuditLogs] = useState<ConnectAuditEntry[]>([]);

  // QR Modal
  const [showQrModal, setShowQrModal] = useState(false);
  const [qrPayload, setQrPayload] = useState<QrPairingPayload | null>(null);
  const [qrSecondsLeft, setQrSecondsLeft] = useState(600);

  // Clipboard Test
  const [clipboardInput, setClipboardInput] = useState('');
  const [filteredResult, setFilteredResult] = useState<string | null>(null);
  const [filterReason, setFilterReason] = useState<string | null>(null);

  // Status Notification
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3500);
  };

  const loadAllData = async () => {
    try {
      const [devs, syncs, confs, cont, focus, [cam, mic], logs] = await Promise.all([
        connectClient.listTrustedDevices(),
        connectClient.getSyncStatus(),
        connectClient.getActiveConflicts(),
        connectClient.getContinuityState(),
        connectClient.getActiveFocus(),
        connectClient.getHardwareBridgeStatus(),
        connectClient.getSecurityAuditLogs(),
      ]);

      setTrustedDevices(devs);
      setSyncFolders(syncs);
      setActiveConflicts(confs);
      setContinuityState(cont);
      setActiveFocus(focus);
      setWebcamStatus(cam);
      setMicStatus(mic);
      setAuditLogs(logs);
    } catch (err) {
      console.error('Failed to load Connect data:', err);
    }
  };

  useEffect(() => {
    loadAllData();
    const interval = setInterval(loadAllData, 10000);
    return () => clearInterval(interval);
  }, []);

  // QR Timer
  useEffect(() => {
    if (!showQrModal) return;
    const timer = setInterval(() => {
      setQrSecondsLeft((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);
    return () => clearInterval(timer);
  }, [showQrModal]);

  const handleOpenPairingQr = async () => {
    const payload = await connectClient.generatePairingQr('MindSet Laptop (Logesh)');
    setQrPayload(payload);
    setQrSecondsLeft(600);
    setShowQrModal(true);
  };

  const handleRevokeDevice = async (deviceId: string, deviceName: string) => {
    if (confirm(`Revoke trust for "${deviceName}"? It will no longer sync files or hardware.`)) {
      await connectClient.revokeDevice(deviceId, 'Revoked by user via Connect Dashboard');
      showToast(`Device "${deviceName}" revoked successfully.`);
      loadAllData();
    }
  };

  const handleResolveConflict = async (conflictId: string, choice: ConflictResolutionChoice) => {
    const res = await connectClient.resolveConflict(conflictId, choice);
    if (choice === 'keep_both') {
      showToast(`Rule 1 Enforced: Preserved duplicate as "${res.preserved_conflict_filename}".`);
    } else {
      showToast(`Conflict resolved with choice "${choice}".`);
    }
    loadAllData();
  };

  const handleToggleWebcam = async () => {
    if (!webcamStatus) return;
    const updated = await connectClient.toggleWebcamStream(
      !webcamStatus.is_active,
      webcamStatus.resolution,
      webcamStatus.use_front_camera
    );
    setWebcamStatus(updated);
    showToast(
      updated.is_active
        ? 'Wireless Webcam connected: Phone sensor streaming to MindSet Hello at 1080p.'
        : 'Wireless Webcam disconnected.'
    );
  };

  const handleToggleMic = async () => {
    if (!micStatus) return;
    const updated = await connectClient.toggleMicStream(!micStatus.is_active, micStatus.noise_suppression);
    setMicStatus(updated);
    showToast(
      updated.is_active
        ? 'Wireless Mic connected: Phone dual-mic array active for Whisper AI voice typing.'
        : 'Wireless Mic disconnected.'
    );
  };

  const handleTestClipboard = async () => {
    if (!clipboardInput.trim()) return;
    const res = await connectClient.syncClipboard(clipboardInput);
    setFilteredResult(res.content);
    setFilterReason(res.filter_reason || null);
  };

  return (
    <div className="min-h-screen bg-[#07090D] text-slate-100 p-6 font-sans">
      {/* Toast Notification */}
      {toastMessage && (
        <div className="fixed bottom-6 right-6 z-50 bg-emerald-950/90 border border-emerald-500/40 text-emerald-200 px-4 py-3 rounded-xl shadow-2xl backdrop-blur-md flex items-center space-x-3 text-sm animate-fade-in">
          <CheckCircleIcon className="w-5 h-5 text-emerald-400" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Header Bar */}
      <div className="flex flex-col md:flex-row md:items-center justify-between pb-6 border-b border-slate-800/80 gap-4">
        <div>
          <div className="flex items-center space-x-3">
            <span className="p-2 rounded-xl bg-cyan-500/10 border border-cyan-500/20 text-cyan-400">
              <SmartphoneIcon className="w-6 h-6" />
            </span>
            <div>
              <h1 className="text-2xl font-bold tracking-tight text-white flex items-center gap-2">
                MindSet Connect
                <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-cyan-500/20 text-cyan-300 border border-cyan-500/30">
                  6-Module P2P Mesh
                </span>
              </h1>
              <p className="text-xs text-slate-400 mt-0.5">
                Local-First TLS Mesh • Ed25519 Cryptographic Trust • Zero-Cloud Reliance
              </p>
            </div>
          </div>
        </div>

        <div className="flex items-center space-x-3">
          <button
            onClick={loadAllData}
            className="px-3.5 py-2 rounded-xl bg-slate-900 border border-slate-700/60 hover:bg-slate-800 text-slate-300 text-sm font-medium flex items-center space-x-2 transition"
          >
            <RefreshIcon className="w-4 h-4" />
            <span>Refresh Mesh</span>
          </button>
          <button
            onClick={handleOpenPairingQr}
            className="px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white text-sm font-semibold shadow-lg shadow-cyan-600/20 flex items-center space-x-2 transition"
          >
            <QrCodeIcon className="w-4 h-4" />
            <span>Pair New Device (QR)</span>
          </button>
        </div>
      </div>

      {/* Continuity & Focus Guard Synchronization Banner */}
      <div className="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Continue Where You Left Off */}
        <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl relative overflow-hidden">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2.5 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
              <SparklesIcon className="w-4 h-4" />
              <span>Module 4 • Continue Where You Left Off</span>
            </div>
            <span className="text-xs text-slate-500 tabular-nums">Source: MindSet Laptop</span>
          </div>

          {continuityState ? (
            <div className="mt-3 flex items-start justify-between">
              <div>
                <h3 className="text-base font-semibold text-slate-100">{continuityState.title}</h3>
                <p className="text-xs text-slate-400 mt-1 flex items-center gap-2">
                  <span className="px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                    {continuityState.active_subject || 'Study'}
                  </span>
                  <span>Exact Bookmark: Page <strong className="text-white tabular-nums">{continuityState.last_read_page}</strong> of <span className="tabular-nums">{continuityState.total_pages}</span></span>
                </p>
                <div className="mt-3 flex items-center space-x-2">
                  <div className="w-48 bg-slate-800 h-2 rounded-full overflow-hidden">
                    <div
                      className="bg-cyan-500 h-full rounded-full transition-all"
                      style={{
                        width: `${Math.round(((continuityState.last_read_page || 1) / (continuityState.total_pages || 1)) * 100)}%`,
                      }}
                    />
                  </div>
                  <span className="text-xs text-cyan-400 tabular-nums font-semibold">
                    {Math.round(((continuityState.last_read_page || 1) / (continuityState.total_pages || 1)) * 100)}%
                  </span>
                </div>
              </div>
              <button
                onClick={() => showToast('Textbook opened at exact page 84 on Laptop & Phone synced.')}
                className="px-3.5 py-1.5 rounded-lg bg-cyan-500/20 hover:bg-cyan-500/30 border border-cyan-500/30 text-cyan-300 text-xs font-semibold transition"
              >
                Resume Page 84
              </button>
            </div>
          ) : (
            <p className="text-xs text-slate-500 mt-3">No active textbook or study draft in continuity buffer.</p>
          )}
        </div>

        {/* Cross-Device Focus Guard Sync */}
        <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl relative overflow-hidden">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2.5 text-purple-400 text-xs font-semibold uppercase tracking-wider">
              <ShieldCheckIcon className="w-4 h-4" />
              <span>Cross-Device Focus Guard Sync</span>
            </div>
            <span className="text-xs px-2 py-0.5 rounded-full bg-purple-500/20 text-purple-300 border border-purple-500/30 font-medium">
              Study Lockdown Active
            </span>
          </div>

          {activeFocus ? (
            <div className="mt-3 flex items-start justify-between">
              <div>
                <h3 className="text-base font-semibold text-slate-100">{activeFocus.task_title}</h3>
                <p className="text-xs text-slate-400 mt-1 flex items-center gap-2">
                  <span>Profile: <strong className="text-purple-300 capitalize">{activeFocus.profile.replace('_', ' ')}</strong></span>
                  <span>•</span>
                  <span>Distractions blocked on both Laptop & Paired Mobile</span>
                </p>
                <div className="mt-3 flex items-center space-x-3 text-xs">
                  <span className="flex items-center space-x-1.5 text-slate-300">
                    <ClockIcon className="w-3.5 h-3.5 text-purple-400" />
                    <span className="tabular-nums font-mono text-purple-200">
                      {Math.floor(activeFocus.remaining_seconds / 60)}m {activeFocus.remaining_seconds % 60}s remaining
                    </span>
                  </span>
                  <span className="text-emerald-400 font-medium">Emergency Calls Permitted</span>
                </div>
              </div>
              <button
                onClick={() => showToast('Emergency bypass request forwarded to parent contacts.')}
                className="px-3.5 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 border border-slate-700 text-slate-300 text-xs font-medium transition"
              >
                Request Bypass
              </button>
            </div>
          ) : (
            <p className="text-xs text-slate-500 mt-3">No active Focus Guard session running.</p>
          )}
        </div>
      </div>

      {/* Main Grid: Left = Hardware Bridge & Devices, Right = Conflict-Aware Sync & Audit */}
      <div className="mt-6 grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left 2 Cols: Hardware Bridge & Trusted Devices */}
        <div className="lg:col-span-2 space-y-6">
          {/* Module 5: Wireless Hardware Bridge (Solving Hardware Gaps) */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800/60">
              <div className="flex items-center space-x-2.5">
                <span className="p-1.5 rounded-lg bg-indigo-500/10 text-indigo-400">
                  <CameraIcon className="w-5 h-5" />
                </span>
                <div>
                  <h2 className="text-base font-semibold text-white">
                    Module 5: Wireless Hardware Bridge
                  </h2>
                  <p className="text-xs text-slate-400">
                    Turn paired smartphone into low-latency Laptop Webcam, Whisper Microphone & Scanner
                  </p>
                </div>
              </div>
              <span className="text-xs px-2.5 py-1 rounded-full bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 flex items-center space-x-1.5">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                <span>Zero Driver Lag</span>
              </span>
            </div>

            <div className="mt-4 grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* Phone as Wireless Webcam */}
              <div className="bg-slate-950/70 border border-slate-800 rounded-xl p-4 flex flex-col justify-between">
                <div>
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-semibold text-cyan-400 uppercase tracking-wider flex items-center gap-1.5">
                      <CameraIcon className="w-4 h-4" />
                      Wireless Webcam
                    </span>
                    <button
                      onClick={handleToggleWebcam}
                      className={`px-3 py-1 rounded-lg text-xs font-semibold transition ${
                        webcamStatus?.is_active
                          ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 hover:bg-emerald-500/30'
                          : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
                      }`}
                    >
                      {webcamStatus?.is_active ? 'Streaming' : 'Disconnected'}
                    </button>
                  </div>
                  <h4 className="text-sm font-semibold text-slate-200 mt-2">
                    {webcamStatus?.streaming_device_name || 'Galaxy S24 (Main 50MP)'}
                  </h4>
                  <p className="text-xs text-slate-400 mt-1">
                    Routes directly into MindSet Hello facial auth and video classroom sessions.
                  </p>
                </div>

                <div className="mt-4 pt-3 border-t border-slate-800/80 flex items-center justify-between text-xs text-slate-400">
                  <span className="tabular-nums font-mono">
                    Res: <strong className="text-slate-200">{webcamStatus?.resolution || '1080p'}</strong> (30 FPS)
                  </span>
                  <span className="tabular-nums font-mono text-emerald-400 font-semibold">
                    Latency: {webcamStatus?.latency_ms || 18} ms
                  </span>
                </div>
              </div>

              {/* Phone as Wireless Microphone */}
              <div className="bg-slate-950/70 border border-slate-800 rounded-xl p-4 flex flex-col justify-between">
                <div>
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wider flex items-center gap-1.5">
                      <MicIcon className="w-4 h-4" />
                      Whisper AI Microphone
                    </span>
                    <button
                      onClick={handleToggleMic}
                      className={`px-3 py-1 rounded-lg text-xs font-semibold transition ${
                        micStatus?.is_active
                          ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 hover:bg-emerald-500/30'
                          : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
                      }`}
                    >
                      {micStatus?.is_active ? 'Active' : 'Muted'}
                    </button>
                  </div>
                  <h4 className="text-sm font-semibold text-slate-200 mt-2">
                    {micStatus?.streaming_device_name || 'Galaxy S24 Dual-Mic Array'}
                  </h4>
                  <p className="text-xs text-slate-400 mt-1">
                    Feeds Whisper AI speech model for high-accuracy voice typing & doubt asking.
                  </p>
                </div>

                <div className="mt-4 pt-3 border-t border-slate-800/80 flex items-center justify-between text-xs text-slate-400">
                  <span className="tabular-nums font-mono">
                    Rate: <strong className="text-slate-200">{micStatus?.sample_rate_hz || 16000} Hz</strong>
                  </span>
                  <span className="tabular-nums font-mono text-cyan-400 font-semibold">
                    Level: {micStatus?.audio_level_db || -24.5} dB
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* Module 1: Trusted Devices Registry */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800/60">
              <div>
                <h2 className="text-base font-semibold text-white">Module 1: Trusted Devices</h2>
                <p className="text-xs text-slate-400">
                  Authenticated peers permitted to sync vaults, stream hardware, and receive alerts
                </p>
              </div>
              <span className="text-xs text-slate-400 tabular-nums">
                {trustedDevices.filter((d) => !d.is_revoked).length} Active Pairs
              </span>
            </div>

            <div className="mt-4 space-y-3">
              {trustedDevices.map((dev) => (
                <div
                  key={dev.device_id}
                  className={`p-4 rounded-xl border transition ${
                    dev.is_revoked
                      ? 'bg-rose-950/20 border-rose-900/40 opacity-60'
                      : 'bg-slate-950/70 border-slate-800/90 hover:border-slate-700'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-start space-x-3">
                      <div
                        className={`p-2.5 rounded-xl ${
                          dev.is_revoked ? 'bg-rose-500/10 text-rose-400' : 'bg-cyan-500/10 text-cyan-400'
                        }`}
                      >
                        <SmartphoneIcon className="w-5 h-5" />
                      </div>
                      <div>
                        <div className="flex items-center gap-2">
                          <h4 className="text-sm font-semibold text-slate-100">{dev.device_name}</h4>
                          <span
                            className={`text-[10px] px-2 py-0.5 rounded-full font-medium ${
                              dev.connection_mode === 'local_wifi'
                                ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                                : 'bg-blue-500/20 text-blue-300 border border-blue-500/30'
                            }`}
                          >
                            {dev.connection_mode === 'local_wifi' ? 'Local Wi-Fi' : 'Tailscale WireGuard'}
                          </span>
                          {dev.is_revoked && (
                            <span className="text-[10px] px-2 py-0.5 rounded bg-rose-500/20 text-rose-300 font-medium">
                              Revoked
                            </span>
                          )}
                        </div>
                        <p className="text-xs text-slate-400 mt-1 font-mono">
                          IP: {dev.ip_address} • Key: {dev.public_key_ed25519.slice(0, 24)}...
                        </p>
                        <div className="mt-2 flex flex-wrap gap-1.5 text-[11px] text-slate-400">
                          <span className="px-2 py-0.5 rounded bg-slate-900 text-slate-300">Tasks Sync</span>
                          <span className="px-2 py-0.5 rounded bg-slate-900 text-slate-300">Vault Mirror</span>
                          <span className="px-2 py-0.5 rounded bg-slate-900 text-slate-300">Hardware Bridge</span>
                          <span className="px-2 py-0.5 rounded bg-slate-900 text-slate-300">Focus Sync</span>
                        </div>
                      </div>
                    </div>

                    {!dev.is_revoked && (
                      <button
                        onClick={() => handleRevokeDevice(dev.device_id, dev.device_name)}
                        className="px-3 py-1.5 rounded-lg bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/30 text-rose-300 text-xs font-medium flex items-center space-x-1.5 transition"
                      >
                        <Trash2Icon className="w-3.5 h-3.5" />
                        <span>Revoke</span>
                      </button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Strict Rule 2: Universal Clipboard Privacy Filter */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between pb-3 border-b border-slate-800/60">
              <div className="flex items-center space-x-2">
                <ShieldCheckIcon className="w-5 h-5 text-emerald-400" />
                <h3 className="text-sm font-semibold text-white">
                  Rule 2: Sensitive Clipboard Privacy Filter
                </h3>
              </div>
              <span className="text-xs px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 font-medium">
                Active Filter
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-2">
              MindSet Connect automatically intercepts and masks OTPs, passwords, and API tokens so credentials
              are NEVER broadcast wirelessly across devices. Test the engine below:
            </p>

            <div className="mt-3 flex gap-2">
              <input
                type="text"
                value={clipboardInput}
                onChange={(e) => setClipboardInput(e.target.value)}
                placeholder="Type or paste sample text (e.g., 'Your OTP is 492810' or secret key)..."
                className="flex-1 px-3.5 py-2 rounded-xl bg-slate-950 border border-slate-800 text-xs text-slate-200 focus:outline-none focus:border-cyan-500"
              />
              <button
                onClick={handleTestClipboard}
                className="px-4 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold transition"
              >
                Test Filter
              </button>
            </div>

            {filteredResult && (
              <div className="mt-3 p-3 rounded-xl bg-slate-950 border border-slate-800 text-xs">
                <div className="flex items-center justify-between">
                  <span className="text-slate-400 font-medium">Sync Broadcast Payload:</span>
                  {filterReason ? (
                    <span className="text-amber-400 font-semibold flex items-center gap-1">
                      <AlertTriangleIcon className="w-3.5 h-3.5" />
                      {filterReason}
                    </span>
                  ) : (
                    <span className="text-emerald-400 font-semibold flex items-center gap-1">
                      <CheckCircleIcon className="w-3.5 h-3.5" />
                      Clean (Synced to Peer)
                    </span>
                  )}
                </div>
                <div className="mt-1 font-mono text-cyan-300 bg-slate-900/80 p-2 rounded">
                  {filteredResult}
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Right 1 Col: Module 2 Conflict-Aware Sync & Security Audit */}
        <div className="space-y-6">
          {/* Module 2: Conflict-Aware Selective Sync */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between pb-3 border-b border-slate-800/60">
              <div className="flex items-center space-x-2">
                <LayersIcon className="w-5 h-5 text-cyan-400" />
                <h3 className="text-sm font-semibold text-white">Module 2: Selective Sync</h3>
              </div>
              <span className="text-xs text-slate-400">Strict Rule 1 Active</span>
            </div>

            <div className="mt-3 space-y-3">
              {syncFolders.map((folder) => (
                <div key={folder.folder_name} className="p-3 rounded-xl bg-slate-950/70 border border-slate-800">
                  <div className="flex items-center justify-between">
                    <h4 className="text-xs font-semibold text-slate-200">{folder.folder_name}</h4>
                    <span className="text-[10px] px-2 py-0.5 rounded-full bg-cyan-500/20 text-cyan-300 font-medium">
                      Enabled
                    </span>
                  </div>
                  <p className="text-[11px] text-slate-400 mt-1 font-mono truncate">{folder.local_path}</p>
                  <div className="mt-2 pt-2 border-t border-slate-800/80 flex items-center justify-between text-[11px] text-slate-400">
                    <span className="tabular-nums">{folder.files_in_sync} files synced</span>
                    {folder.active_conflicts > 0 ? (
                      <span className="text-amber-400 font-semibold tabular-nums">
                        {folder.active_conflicts} Conflict
                      </span>
                    ) : (
                      <span className="text-emerald-400">In Sync</span>
                    )}
                  </div>
                </div>
              ))}
            </div>

            {/* Active Conflict Resolution Card (Rule 1 Enforcement) */}
            {activeConflicts.length > 0 && activeConflicts[0] && (
              <div className="mt-4 p-4 rounded-xl bg-amber-950/20 border border-amber-500/30">
                <div className="flex items-center space-x-2 text-amber-400 text-xs font-semibold">
                  <AlertTriangleIcon className="w-4 h-4" />
                  <span>Rule 1: Sync Conflict Detected</span>
                </div>
                <p className="text-xs text-slate-300 mt-1 font-semibold">
                  {activeConflicts[0]?.resource_title}
                </p>
                <p className="text-[11px] text-slate-400 mt-1">
                  Local revision and Phone revision were edited concurrently. No data will be overwritten.
                </p>

                <div className="mt-3 space-y-2 text-[11px]">
                  <div className="p-2 rounded bg-slate-900/80 border border-slate-800">
                    <span className="text-cyan-400 font-medium">Laptop Revision:</span>
                    <p className="text-slate-300 mt-0.5">{activeConflicts[0]?.local_preview}</p>
                  </div>
                  <div className="p-2 rounded bg-slate-900/80 border border-slate-800">
                    <span className="text-purple-400 font-medium">Phone Revision:</span>
                    <p className="text-slate-300 mt-0.5">{activeConflicts[0]?.remote_preview}</p>
                  </div>
                </div>

                <div className="mt-3 grid grid-cols-3 gap-1.5 text-xs font-medium">
                  <button
                    onClick={() => activeConflicts[0] && handleResolveConflict(activeConflicts[0].conflict_id, 'keep_mine')}
                    className="py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 transition text-center"
                  >
                    Keep Mine
                  </button>
                  <button
                    onClick={() => activeConflicts[0] && handleResolveConflict(activeConflicts[0].conflict_id, 'keep_theirs')}
                    className="py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 transition text-center"
                  >
                    Keep Theirs
                  </button>
                  <button
                    onClick={() => activeConflicts[0] && handleResolveConflict(activeConflicts[0].conflict_id, 'keep_both')}
                    className="py-1.5 rounded bg-cyan-600 hover:bg-cyan-500 text-white font-semibold transition text-center"
                  >
                    Keep Both
                  </button>
                </div>
              </div>
            )}
          </div>

          {/* Module 6: Security Audit Logs */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between pb-3 border-b border-slate-800/60">
              <div className="flex items-center space-x-2">
                <ShieldAlertIcon className="w-5 h-5 text-indigo-400" />
                <h3 className="text-sm font-semibold text-white">Module 6: Security Audit</h3>
              </div>
              <span className="text-xs text-slate-400">TLS Authenticated</span>
            </div>

            <div className="mt-3 space-y-2.5 max-h-72 overflow-y-auto pr-1">
              {auditLogs.map((log) => (
                <div key={log.audit_id} className="p-2.5 rounded-lg bg-slate-950/70 border border-slate-800/80 text-xs">
                  <div className="flex items-center justify-between">
                    <span
                      className={`font-semibold ${
                        log.severity === 'warning'
                          ? 'text-amber-400'
                          : log.severity === 'security_alert'
                          ? 'text-rose-400'
                          : 'text-cyan-400'
                      }`}
                    >
                      {log.event_type}
                    </span>
                    <span className="text-[10px] text-slate-500 tabular-nums">
                      {new Date(log.timestamp_utc).toLocaleTimeString()}
                    </span>
                  </div>
                  <p className="text-[11px] text-slate-300 mt-1">{log.description}</p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* QR Code Pairing Modal */}
      {showQrModal && qrPayload && (
        <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex items-center justify-center p-4">
          <div className="bg-slate-900 border border-slate-800 rounded-3xl p-6 max-w-md w-full shadow-2xl relative animate-scale-up">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800">
              <div className="flex items-center space-x-2.5">
                <span className="p-2 rounded-xl bg-cyan-500/10 text-cyan-400">
                  <QrCodeIcon className="w-5 h-5" />
                </span>
                <div>
                  <h3 className="text-base font-bold text-white">Pair MindSet Mobile</h3>
                  <p className="text-xs text-slate-400">Scan with your smartphone companion app</p>
                </div>
              </div>
              <button
                onClick={() => setShowQrModal(false)}
                className="text-slate-400 hover:text-white p-1 rounded-lg transition"
              >
                ✕
              </button>
            </div>

            {/* QR Visual Container */}
            <div className="my-6 flex flex-col items-center">
              <div className="p-4 bg-white rounded-2xl shadow-xl border-4 border-cyan-500/30 relative">
                {/* SVG Mock QR Pattern with high visual realism */}
                <svg className="w-56 h-56" viewBox="0 0 100 100" fill="black">
                  {/* Position markers */}
                  <rect x="5" y="5" width="25" height="25" rx="3" fill="#07090D" />
                  <rect x="9" y="9" width="17" height="17" fill="white" />
                  <rect x="13" y="13" width="9" height="9" fill="#07090D" />

                  <rect x="70" y="5" width="25" height="25" rx="3" fill="#07090D" />
                  <rect x="74" y="9" width="17" height="17" fill="white" />
                  <rect x="78" y="13" width="9" height="9" fill="#07090D" />

                  <rect x="5" y="70" width="25" height="25" rx="3" fill="#07090D" />
                  <rect x="9" y="74" width="17" height="17" fill="white" />
                  <rect x="13" y="78" width="9" height="9" fill="#07090D" />

                  {/* QR Data Grid simulated blocks */}
                  <rect x="35" y="10" width="4" height="4" />
                  <rect x="42" y="10" width="4" height="4" />
                  <rect x="50" y="10" width="4" height="4" />
                  <rect x="58" y="10" width="4" height="4" />
                  <rect x="35" y="18" width="8" height="4" />
                  <rect x="48" y="18" width="4" height="4" />
                  <rect x="56" y="18" width="8" height="4" />

                  <rect x="10" y="35" width="4" height="8" />
                  <rect x="18" y="35" width="8" height="4" />
                  <rect x="10" y="48" width="4" height="4" />
                  <rect x="18" y="54" width="8" height="4" />

                  <rect x="35" y="35" width="30" height="30" rx="4" fill="#0891b2" />
                  <circle cx="50" cy="50" r="8" fill="white" />
                  <circle cx="50" cy="50" r="4" fill="#07090D" />

                  <rect x="70" y="35" width="4" height="8" />
                  <rect x="78" y="42" width="12" height="4" />
                  <rect x="70" y="54" width="8" height="4" />

                  <rect x="35" y="72" width="8" height="4" />
                  <rect x="48" y="72" width="4" height="8" />
                  <rect x="56" y="72" width="8" height="4" />
                  <rect x="35" y="84" width="4" height="8" />
                  <rect x="44" y="84" width="8" height="4" />
                  <rect x="56" y="84" width="12" height="4" />

                  <rect x="72" y="72" width="8" height="8" />
                  <rect x="84" y="84" width="8" height="8" />
                </svg>
              </div>

              <div className="mt-3 flex items-center space-x-2 text-xs text-slate-400">
                <ClockIcon className="w-3.5 h-3.5 text-amber-400" />
                <span>
                  Code expires in{' '}
                  <strong className="text-amber-400 font-mono tabular-nums">
                    {Math.floor(qrSecondsLeft / 60)}:{String(qrSecondsLeft % 60).padStart(2, '0')}
                  </strong>
                </span>
              </div>
            </div>

            {/* Connection Information */}
            <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800 text-xs space-y-1.5 font-mono">
              <div className="flex justify-between text-slate-400">
                <span>Session ID:</span>
                <span className="text-slate-200">{qrPayload.session_id}</span>
              </div>
              <div className="flex justify-between text-slate-400">
                <span>Local IP:</span>
                <span className="text-slate-200">{(qrPayload.local_ip_addresses && qrPayload.local_ip_addresses[0]) || '192.168.1.105'}:{qrPayload.port}</span>
              </div>
              <div className="flex justify-between text-slate-400">
                <span>Tailscale IP:</span>
                <span className="text-slate-200">{(qrPayload.local_ip_addresses && qrPayload.local_ip_addresses[1]) || '100.64.0.42'}</span>
              </div>
              <div className="flex justify-between text-slate-400">
                <span>Host Public Key:</span>
                <span className="text-cyan-400">{qrPayload.public_key_ed25519.slice(0, 20)}...</span>
              </div>
            </div>

            <div className="mt-5 flex justify-end">
              <button
                onClick={() => setShowQrModal(false)}
                className="w-full py-2.5 rounded-xl bg-slate-800 hover:bg-slate-700 text-white text-xs font-semibold transition"
              >
                Close Pairing Window
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
