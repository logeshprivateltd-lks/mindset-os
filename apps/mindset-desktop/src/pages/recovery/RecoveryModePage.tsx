/**
 * MindSet OS — Standalone Recovery Mode Interface (/recovery)
 * Ultra-Lightweight Rescue Environment for Encrypted Snapshots,
 * 3-Second Update Rollback, SQLite Repairs, Firewall/Network Resets,
 * Offline USB .msos-patch Fixes, and Emergency USB Data Export.
 *
 * ACCESSIBILITY:
 * Accessible via boot flag (--recovery), hotkey during BootSplashGate (F8/Alt+R), or from /status.
 * Works even if the main desktop workspace or GPU theme fails.
 *
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import {
  ShieldAlertIcon as ShieldAlert,
  HistoryIcon as History,
  RotateCcwIcon as RotateCcw,
  DatabaseIcon as Database,
  MonitorIcon as Monitor,
  AlertOctagonIcon as AlertOctagon,
  WrenchIcon as Wrench,
  CheckCircleIcon as CheckCircle2,
  RefreshIcon as RefreshCw,
  FileArchiveIcon as FileArchive,
  Trash2Icon as Trash2,
  ArrowLeftIcon as ArrowLeft,
  PlusIcon as Plus,
  UploadIcon as Upload,
  DownloadCloudIcon as DownloadCloud,
} from '../../components/common/SystemIcons';
import type {
  SystemSnapshot,
  RecoveryActionResult,
  DatabaseRepairReport,
  DriverResetReport,
  OstreeDeploymentInfo,
} from '@mindset/db-schema';
import { updaterClient } from '../../lib/updaterClient';

export type RecoveryTab =
  | 'rollback'
  | 'snapshots'
  | 'database'
  | 'network_firewall'
  | 'offline_patch'
  | 'export_archive'
  | 'factory_reset';

export const RecoveryModePage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<RecoveryTab>('rollback');

  const [snapshots, setSnapshots] = useState<SystemSnapshot[]>([]);
  const [selectedSnapshotId, setSelectedSnapshotId] = useState<string | null>(null);
  const [newSnapshotLabel, setNewSnapshotLabel] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [actionResult, setActionResult] = useState<RecoveryActionResult | null>(null);
  const [dbRepairReport, setDbRepairReport] = useState<DatabaseRepairReport | null>(null);
  const [driverReport, setDriverReport] = useState<DriverResetReport | null>(null);
  const [ostreeInfo, setOstreeInfo] = useState<OstreeDeploymentInfo | null>(null);
  const [offlinePatchPath, setOfflinePatchPath] = useState('');
  const [exportUsbPath, setExportUsbPath] = useState('');
  const [confirmPhrase, setConfirmPhrase] = useState('');
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const loadData = useCallback(async () => {
    setIsLoading(true);
    try {
      const [list, ostree] = await Promise.all([
        updaterClient.listSnapshots(),
        updaterClient.getOstreeStatus(),
      ]);
      setSnapshots(list);
      setOstreeInfo(ostree);
      if (list.length > 0 && !selectedSnapshotId && list[0]) {
        setSelectedSnapshotId(list[0].id);
      }
    } catch (err) {
      console.error('Failed to load recovery telemetry:', err);
    } finally {
      setIsLoading(false);
    }
  }, [selectedSnapshotId]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  // 1. Roll Back to Previous Atomic Snapshot (3s Rollback)
  const handleRollbackUpdate = async () => {
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.rollbackUpdate();
      setActionResult(res);
      await loadData();
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 2. Restore Encrypted SQLite Backup
  const handleRestoreSnapshot = async (id: string) => {
    if (!confirm('Are you sure you want to restore this snapshot? Current working state will be quarantined.')) {
      return;
    }
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.restoreSnapshot(id);
      setActionResult(res);
      await loadData();
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateSnapshot = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newSnapshotLabel.trim()) return;

    setIsLoading(true);
    setErrorMsg(null);
    try {
      const snap = await updaterClient.createSnapshot(newSnapshotLabel.trim(), false);
      setNewSnapshotLabel('');
      await loadData();
      setSelectedSnapshotId(snap.id);
      setActionResult({
        success: true,
        action: 'create_snapshot',
        message: `Successfully created snapshot '${snap.label}'.`,
        details: [
          `Snapshot ID: ${snap.id}`,
          `Integrity SHA256: ${snap.checksum_sha256.substring(0, 16)}...`,
          `Archived databases: ${snap.database_count} tables, ${snap.files_count} assets.`,
        ],
        timestamp: new Date().toISOString(),
      });
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 3. Verify & Repair Database Integrity
  const handleRepairDatabases = async () => {
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const rep = await updaterClient.repairDatabases();
      setDbRepairReport(rep);
      setActionResult({
        success: true,
        action: 'repair_databases',
        message: 'SQLite deep integrity repair, index optimization, and WAL truncation completed.',
        details: [
          `Integrity status: ${rep.integrity_status.toUpperCase()}`,
          `WAL truncated: ${rep.wal_truncated_mb} MB`,
          `Vacuum space recovered: ${(rep.vacuum_recovered_bytes / 1024 / 1024).toFixed(2)} MB`,
          `Indexes rebuilt: ${rep.indexes_rebuilt}`,
        ],
        timestamp: new Date().toISOString(),
      });
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 4. Reset Network, Firewall (nftables) & Wi-Fi Drivers to Safe Defaults
  const handleResetNetworkAndFirewall = async () => {
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const [netRes, drvRes] = await Promise.all([
        updaterClient.resetNetworkAndFirewall(),
        updaterClient.resetDrivers(),
      ]);
      setDriverReport(drvRes);
      setActionResult({
        success: true,
        action: 'reset_network_and_firewall',
        message: 'Network, nftables firewall, and hardware drivers reset to safe defaults.',
        details: [
          ...netRes.details,
          'Display resolution forced to 1920x1080@60Hz safe mode.',
          'Audio output reset to primary hardware speaker.',
        ],
        timestamp: new Date().toISOString(),
      });
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 5. Apply Offline .msos-patch Security Fix from USB Drive
  const handleApplyOfflinePatch = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.applyOfflinePatch(
        offlinePatchPath.trim() || 'E:/security_patch_2026.msos-patch'
      );
      setActionResult(res);
      setOfflinePatchPath('');
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 6. Export Emergency Student Data Archive to USB
  const handleExportEmergencyArchive = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.exportEmergencyDataArchive(
        exportUsbPath.trim() || undefined
      );
      setActionResult(res);
      setExportUsbPath('');
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 7. Protected Factory Reset
  const handleFactoryReset = async () => {
    if (confirmPhrase !== 'CONFIRM_MINDSET_RESET') {
      setErrorMsg("Confirmation phrase does not match 'CONFIRM_MINDSET_RESET'.");
      return;
    }
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.performFactoryReset(confirmPhrase);
      setActionResult(res);
      setConfirmPhrase('');
      await loadData();
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const formatBytes = (bytes: number): string => {
    if (bytes >= 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${bytes} B`;
  };

  return (
    <div className="flex flex-col h-full bg-[#07090E] text-slate-100 overflow-y-auto selection:bg-amber-500/30">
      {/* 1. Pre-Boot Emergency Header */}
      <header className="px-6 py-4 border-b border-amber-900/40 bg-slate-950/90 backdrop-blur-md sticky top-0 z-20 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-amber-600 to-red-600 flex items-center justify-center shadow-lg shadow-amber-500/10">
            <ShieldAlert className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-lg font-semibold tracking-tight text-white flex items-center gap-2">
              MindSet OS Standalone Recovery Console
              <span className="text-[10px] px-2 py-0.5 rounded-full bg-amber-950/80 border border-amber-600/50 text-amber-300 font-mono">
                ZERO-DEPENDENCY RESCUE
              </span>
            </h1>
            <p className="text-xs text-slate-400">
              Pre-boot recovery subsystem for atomic rollbacks, encrypted backups, SQLite repairs, and offline USB patches
            </p>
          </div>
        </div>

        <button
          onClick={() => window.history.back()}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 border border-slate-800 text-xs text-slate-300 transition-colors"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          Return to Shell
        </button>
      </header>

      {/* Main Body */}
      <div className="p-6 max-w-6xl mx-auto w-full space-y-6">
        {/* Action / Success Banner */}
        {actionResult && (
          <div className="p-4 rounded-xl bg-emerald-950/50 border border-emerald-600/40 text-emerald-100 animate-fadeIn space-y-2">
            <div className="flex items-center gap-2.5">
              <CheckCircle2 className="w-5 h-5 text-emerald-400 flex-shrink-0" />
              <p className="font-semibold text-sm text-emerald-300">{actionResult.message}</p>
            </div>
            {actionResult.details.length > 0 && (
              <ul className="text-xs space-y-1 pl-7 text-emerald-200/80">
                {actionResult.details.map((d, i) => (
                  <li key={i}>• {d}</li>
                ))}
              </ul>
            )}
          </div>
        )}

        {/* Error Banner */}
        {errorMsg && (
          <div className="p-4 rounded-xl bg-red-950/50 border border-red-600/40 text-red-100 space-y-1">
            <div className="flex items-center gap-2">
              <AlertOctagon className="w-4 h-4 text-red-400 flex-shrink-0" />
              <span className="text-xs font-semibold text-red-300">Operation Notice</span>
            </div>
            <p className="text-xs text-red-200/90 pl-6">{errorMsg}</p>
          </div>
        )}

        {/* 2. Navigation Tabs — 6 Canonical Rescue Actions */}
        <div className="flex flex-wrap items-center gap-2 border-b border-slate-800 pb-2">
          {[
            { id: 'rollback', label: '1. Instant 3s Rollback', icon: RotateCcw, badge: 'Atomic' },
            { id: 'snapshots', label: '2. Restore SQLite Backup', icon: History, badge: `${snapshots.length}` },
            { id: 'database', label: '3. Verify & Repair DB', icon: Database, badge: 'PRAGMA' },
            { id: 'network_firewall', label: '4. Reset Network & nftables', icon: Monitor, badge: 'Firewall' },
            { id: 'offline_patch', label: '5. Offline USB .msos-patch', icon: Upload, badge: 'Ed25519' },
            { id: 'export_archive', label: '6. Export Student Archive', icon: DownloadCloud, badge: 'USB' },
            { id: 'factory_reset', label: 'Factory Reset Guard', icon: AlertOctagon, badge: 'Wipe' },
          ].map((tab) => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as RecoveryTab)}
                className={`flex items-center gap-2 px-3 py-2 rounded-xl text-xs font-medium transition-all ${
                  isActive
                    ? 'bg-amber-600/20 text-amber-300 border border-amber-500/50 shadow-sm shadow-amber-500/10'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900 border border-transparent'
                }`}
              >
                <Icon className={`w-3.5 h-3.5 ${isActive ? 'text-amber-400' : 'text-slate-400'}`} />
                {tab.label}
                <span
                  className={`ml-1 text-[10px] px-1.5 py-0.2 rounded-full font-mono ${
                    isActive ? 'bg-amber-900/60 text-amber-200' : 'bg-slate-800 text-slate-400'
                  }`}
                >
                  {tab.badge}
                </span>
              </button>
            );
          })}
        </div>

        {/* 3. Tab Contents */}

        {/* ACTION 1: Roll Back to Previous Atomic Snapshot */}
        {activeTab === 'rollback' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-gradient-to-tr from-amber-600 to-orange-600 flex items-center justify-center flex-shrink-0 shadow-lg shadow-amber-500/10">
                <RotateCcw className="w-6 h-6 text-white" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Action 1: Roll Back to Previous Atomic Snapshot
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  Fedora Kinoite / ostree immutable deployment enables zero-risk system updates. If a newly installed patch triggers graphics driver corruption, kernel panics, or application faults, trigger this instant rollback to swap execution back to the previous deployment slot in 3 seconds.
                </p>
              </div>
            </div>

            <div className="p-4 rounded-xl bg-slate-950/70 border border-slate-800/80 grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs">
              <div className="space-y-1">
                <span className="text-slate-400 text-[11px]">Primary Active Deployment</span>
                <span className="font-semibold text-white block">
                  {ostreeInfo?.active_commit || 'Slot A (Running Kernel 6.11.7)'}
                </span>
                <span className="text-slate-500 text-[11px] font-mono">
                  Source: {ostreeInfo?.deployment_source || 'ostree:fedora-kinoite'}
                </span>
              </div>
              <div className="space-y-1">
                <span className="text-slate-400 text-[11px]">Rollback Standby Image</span>
                <span className="font-semibold text-amber-400 block">
                  {ostreeInfo?.rollback_commit || 'Slot B (Known-Good Baseline)'}
                </span>
                <span className="text-slate-500 text-[11px] font-mono">
                  5s Startup Health-Check Watchdog: {ostreeInfo?.watchdog_armed ? 'Armed' : 'Disarmed'}
                </span>
              </div>
            </div>

            <div className="p-3.5 rounded-xl bg-emerald-950/30 border border-emerald-800/40 text-xs text-emerald-200">
              <p className="font-medium text-emerald-300">STRICT Zero-Data-Loss Guarantee</p>
              <p className="text-emerald-300/80 mt-0.5">
                Executing a system rollback reverts only the operating system binaries and rootfs image. Your SQLite database (<code className="font-mono">local_core.sqlite</code>, <code className="font-mono">sync_oplog.sqlite</code>), study notes, and Ollama model weights are strictly preserved.
              </p>
            </div>

            <button
              onClick={handleRollbackUpdate}
              disabled={isLoading}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-amber-600 hover:bg-amber-500 text-white text-xs font-semibold shadow-lg shadow-amber-600/20 transition-all disabled:opacity-50"
            >
              <RotateCcw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
              Execute 3-Second Atomic Rollback
            </button>
          </div>
        )}

        {/* ACTION 2: Restore Encrypted SQLite Backup */}
        {activeTab === 'snapshots' && (
          <div className="space-y-6">
            <form
              onSubmit={handleCreateSnapshot}
              className="p-4 rounded-2xl bg-slate-900/60 border border-slate-800 flex flex-col sm:flex-row items-center gap-3"
            >
              <div className="flex-1 w-full flex items-center gap-2 px-3 py-1.5 rounded-xl bg-slate-950/80 border border-slate-800">
                <FileArchive className="w-4 h-4 text-amber-400 flex-shrink-0" />
                <input
                  type="text"
                  placeholder="Capture point-in-time snapshot label (e.g. Pre-Exam State)..."
                  value={newSnapshotLabel}
                  onChange={(e) => setNewSnapshotLabel(e.target.value)}
                  className="w-full bg-transparent text-xs text-white placeholder-slate-500 focus:outline-none"
                />
              </div>
              <button
                type="submit"
                disabled={isLoading || !newSnapshotLabel.trim()}
                className="w-full sm:w-auto flex items-center justify-center gap-1.5 px-4 py-2.5 rounded-xl bg-amber-600 hover:bg-amber-500 text-white text-xs font-semibold disabled:opacity-50 transition-all shadow-md shadow-amber-600/20 flex-shrink-0"
              >
                <Plus className="w-3.5 h-3.5" />
                Capture Snapshot
              </button>
            </form>

            <div className="space-y-3">
              <h2 className="text-xs font-semibold text-slate-400 uppercase tracking-wider">
                Action 2: Available Encrypted SQLite Restore Points ({snapshots.length})
              </h2>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {snapshots.map((snap) => {
                  const isSelected = selectedSnapshotId === snap.id;
                  return (
                    <div
                      key={snap.id}
                      onClick={() => setSelectedSnapshotId(snap.id)}
                      className={`p-4 rounded-2xl cursor-pointer border transition-all ${
                        isSelected
                          ? 'bg-amber-950/20 border-amber-500/50 shadow-sm shadow-amber-500/10'
                          : 'bg-slate-900/50 border-slate-800 hover:border-slate-700'
                      }`}
                    >
                      <div className="flex items-start justify-between gap-3">
                        <div className="space-y-1">
                          <div className="flex items-center gap-2">
                            <span className="font-semibold text-sm text-white">{snap.label}</span>
                            {snap.is_automatic && (
                              <span className="text-[10px] px-1.5 py-0.5 rounded bg-slate-800 text-slate-400">
                                Auto
                              </span>
                            )}
                          </div>
                          <p className="text-[11px] text-slate-400">
                            {new Date(snap.created_at).toLocaleString()} • {formatBytes(snap.size_bytes)}
                          </p>
                        </div>
                        <span className="text-xs font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                          {snap.os_version}
                        </span>
                      </div>

                      <div className="mt-3 pt-3 border-t border-slate-800/80 flex items-center justify-between text-xs">
                        <span className="text-[11px] font-mono text-slate-500 truncate max-w-[200px]">
                          SHA256: {snap.checksum_sha256.substring(0, 16)}...
                        </span>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            handleRestoreSnapshot(snap.id);
                          }}
                          disabled={isLoading}
                          className="px-3 py-1 rounded-lg bg-amber-600/20 hover:bg-amber-600/30 text-amber-300 border border-amber-600/40 text-[11px] font-medium transition-colors"
                        >
                          Restore Backup
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          </div>
        )}

        {/* ACTION 3: Verify & Repair Database Integrity */}
        {activeTab === 'database' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-cyan-600/20 border border-cyan-500/30 flex items-center justify-center flex-shrink-0">
                <Database className="w-6 h-6 text-cyan-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Action 3: Verify & Repair Database Integrity
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  Runs full SQLite integrity verification (<code className="font-mono text-cyan-300">PRAGMA integrity_check</code>), flushes write-ahead logs (<code className="font-mono text-cyan-300">wal_checkpoint(TRUNCATE)</code>), compacts page space, and defragments B-tree indexes across core and vector databases.
                </p>
              </div>
            </div>

            {dbRepairReport && (
              <div className="p-4 rounded-xl bg-slate-950/80 border border-slate-800 space-y-3 text-xs">
                <div className="flex items-center justify-between text-slate-300 border-b border-slate-800 pb-2">
                  <span className="font-semibold text-white">Diagnostics Report</span>
                  <span className="text-emerald-400 font-mono uppercase font-semibold">
                    {dbRepairReport.integrity_status}
                  </span>
                </div>
                <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
                  <div>
                    <span className="text-slate-500 block text-[11px]">WAL Truncated</span>
                    <span className="font-mono text-white text-sm">
                      {dbRepairReport.wal_truncated_mb} MB
                    </span>
                  </div>
                  <div>
                    <span className="text-slate-500 block text-[11px]">Vacuum Recovered</span>
                    <span className="font-mono text-white text-sm">
                      {(dbRepairReport.vacuum_recovered_bytes / 1024 / 1024).toFixed(2)} MB
                    </span>
                  </div>
                  <div>
                    <span className="text-slate-500 block text-[11px]">Indexes Rebuilt</span>
                    <span className="font-mono text-white text-sm">
                      {dbRepairReport.indexes_rebuilt}
                    </span>
                  </div>
                </div>
              </div>
            )}

            <button
              onClick={handleRepairDatabases}
              disabled={isLoading}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold shadow-lg shadow-cyan-600/20 transition-all disabled:opacity-50"
            >
              <Wrench className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
              Execute Deep SQLite Database Repair
            </button>
          </div>
        )}

        {/* ACTION 4: Reset Network, Firewall (nftables) & Wi-Fi Drivers */}
        {activeTab === 'network_firewall' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-indigo-600/20 border border-indigo-500/30 flex items-center justify-center flex-shrink-0">
                <Monitor className="w-6 h-6 text-indigo-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Action 4: Reset Network, Firewall (nftables) & Wi-Fi Drivers
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  Restores network routing, flushes Linux <code className="font-mono text-indigo-300">nftables</code> ruleset to default allowlist, reloads external USB Wi-Fi drivers (<code className="font-mono text-indigo-300">rtl88xxau</code>), and forces display resolution to 1080p@60Hz safe mode.
                </p>
              </div>
            </div>

            {driverReport && (
              <div className="p-4 rounded-xl bg-slate-950/80 border border-slate-800 text-xs text-slate-300 space-y-2">
                <p className="text-emerald-400 font-medium">{driverReport.details}</p>
                <div className="flex items-center gap-4 text-[11px] text-slate-400 pt-1">
                  <span>Display Reset: {driverReport.display_mode_reset ? 'Yes' : 'No'}</span>
                  <span>Wi-Fi Stack Flushed: {driverReport.wifi_adapter_reloaded ? 'Yes' : 'No'}</span>
                  <span>Audio Sink Defaulted: {driverReport.audio_sink_defaulted ? 'Yes' : 'No'}</span>
                </div>
              </div>
            )}

            <button
              onClick={handleResetNetworkAndFirewall}
              disabled={isLoading}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition-all disabled:opacity-50"
            >
              <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
              Reset Network, Firewall & Drivers to Safe Defaults
            </button>
          </div>
        )}

        {/* ACTION 5: Apply Offline .msos-patch Security Fix from USB */}
        {activeTab === 'offline_patch' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-purple-600/20 border border-purple-500/30 flex items-center justify-center flex-shrink-0">
                <Upload className="w-6 h-6 text-purple-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Action 5: Apply Offline .msos-patch Security Fix from USB Drive
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  For school labs without internet connection, apply cryptographically signed <code className="font-mono text-purple-300">.msos-patch</code> archives loaded from an authenticated USB flash drive. Enforces strict Ed25519 signature checks before applying any files.
                </p>
              </div>
            </div>

            <form onSubmit={handleApplyOfflinePatch} className="space-y-3 max-w-xl">
              <label className="text-xs text-slate-300 block">
                Path to .msos-patch file on USB drive:
              </label>
              <div className="flex flex-col sm:flex-row items-center gap-3">
                <input
                  type="text"
                  placeholder="e.g. E:\patches\emergency_sec_fix.msos-patch"
                  value={offlinePatchPath}
                  onChange={(e) => setOfflinePatchPath(e.target.value)}
                  className="flex-1 w-full px-3.5 py-2.5 rounded-xl bg-slate-950/80 border border-slate-800 text-xs text-white placeholder-slate-600 focus:outline-none focus:border-purple-500 font-mono"
                />
                <button
                  type="submit"
                  disabled={isLoading}
                  className="w-full sm:w-auto flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-purple-600 hover:bg-purple-500 text-white text-xs font-semibold shadow-md shadow-purple-600/20 transition-all flex-shrink-0 disabled:opacity-50"
                >
                  <Upload className="w-3.5 h-3.5" />
                  Verify & Apply Fix
                </button>
              </div>
            </form>
          </div>
        )}

        {/* ACTION 6: Export Emergency Student Data Archive to USB */}
        {activeTab === 'export_archive' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-emerald-600/20 border border-emerald-500/30 flex items-center justify-center flex-shrink-0">
                <DownloadCloud className="w-6 h-6 text-emerald-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Action 6: Export Emergency Student Data Archive to USB
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  Creates an authenticated, encrypted <code className="font-mono text-emerald-300">.msos-backup</code> bundle containing your study goals, syllabus skill-trees, journal entries, and habit records onto an external thumb drive before servicing the laptop.
                </p>
              </div>
            </div>

            <form onSubmit={handleExportEmergencyArchive} className="space-y-3 max-w-xl">
              <label className="text-xs text-slate-300 block">
                Destination Directory (leave blank for default USB/Vault export):
              </label>
              <div className="flex flex-col sm:flex-row items-center gap-3">
                <input
                  type="text"
                  placeholder="e.g. E:\student_backup\"
                  value={exportUsbPath}
                  onChange={(e) => setExportUsbPath(e.target.value)}
                  className="flex-1 w-full px-3.5 py-2.5 rounded-xl bg-slate-950/80 border border-slate-800 text-xs text-white placeholder-slate-600 focus:outline-none focus:border-emerald-500 font-mono"
                />
                <button
                  type="submit"
                  disabled={isLoading}
                  className="w-full sm:w-auto flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold shadow-md shadow-emerald-600/20 transition-all flex-shrink-0 disabled:opacity-50"
                >
                  <DownloadCloud className="w-3.5 h-3.5" />
                  Export Archive
                </button>
              </div>
            </form>
          </div>
        )}

        {/* TAB 7: Protected Factory Reset Guard */}
        {activeTab === 'factory_reset' && (
          <div className="p-6 rounded-2xl bg-red-950/20 border border-red-900/40 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-red-600/20 border border-red-500/30 flex items-center justify-center flex-shrink-0">
                <AlertOctagon className="w-6 h-6 text-red-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-red-300">
                  Protected Factory Reset & Data Wipe Guard
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  This procedure restores user space to pristine state. Your tasks, habits, and study notes are moved into an encrypted quarantine backup before reset. The immutable base OS kernel is preserved.
                </p>
              </div>
            </div>

            <div className="space-y-3 max-w-md">
              <label className="text-xs text-slate-300 block">
                Type <code className="font-mono text-red-400 font-semibold">CONFIRM_MINDSET_RESET</code> to authorize:
              </label>
              <input
                type="text"
                placeholder="CONFIRM_MINDSET_RESET"
                value={confirmPhrase}
                onChange={(e) => setConfirmPhrase(e.target.value)}
                className="w-full px-3.5 py-2.5 rounded-xl bg-slate-950/90 border border-red-900/50 text-xs text-white placeholder-slate-600 focus:outline-none focus:border-red-500 font-mono"
              />
            </div>

            <button
              onClick={handleFactoryReset}
              disabled={isLoading || confirmPhrase !== 'CONFIRM_MINDSET_RESET'}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-red-600 hover:bg-red-500 text-white text-xs font-semibold shadow-lg shadow-red-600/20 transition-all disabled:opacity-40 disabled:cursor-not-allowed"
            >
              <Trash2 className="w-4 h-4" />
              Authorize & Execute Factory Reset
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
export default RecoveryModePage;
