/**
 * MindSet OS — Standalone Recovery Mode Interface (/recovery)
 * Ultra-Lightweight Rescue Environment for Encrypted Snapshots,
 * 3-Second Update Rollback, SQLite Repairs & Driver Resets
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
} from '../../components/common/SystemIcons';
import type {
  SystemSnapshot,
  RecoveryActionResult,
  DatabaseRepairReport,
  DriverResetReport,
} from '@mindset/db-schema';
import { updaterClient } from '../../lib/updaterClient';

export const RecoveryModePage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<
    'snapshots' | 'rollback' | 'database' | 'hardware' | 'factory_reset'
  >('snapshots');

  const [snapshots, setSnapshots] = useState<SystemSnapshot[]>([]);
  const [selectedSnapshotId, setSelectedSnapshotId] = useState<string | null>(null);
  const [newSnapshotLabel, setNewSnapshotLabel] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [actionResult, setActionResult] = useState<RecoveryActionResult | null>(null);
  const [dbRepairReport, setDbRepairReport] = useState<DatabaseRepairReport | null>(null);
  const [driverReport, setDriverReport] = useState<DriverResetReport | null>(null);
  const [confirmPhrase, setConfirmPhrase] = useState('');
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const loadSnapshots = useCallback(async () => {
    setIsLoading(true);
    try {
      const list = await updaterClient.listSnapshots();
      setSnapshots(list);
      if (list.length > 0 && !selectedSnapshotId && list[0]) {
        setSelectedSnapshotId(list[0].id);
      }
    } catch (err) {
      console.error('Failed to load recovery snapshots:', err);
    } finally {
      setIsLoading(false);
    }
  }, [selectedSnapshotId]);

  useEffect(() => {
    loadSnapshots();
  }, [loadSnapshots]);

  const handleCreateSnapshot = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newSnapshotLabel.trim()) return;

    setIsLoading(true);
    setErrorMsg(null);
    try {
      const snap = await updaterClient.createSnapshot(newSnapshotLabel.trim(), false);
      setNewSnapshotLabel('');
      await loadSnapshots();
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

  const handleRestoreSnapshot = async (id: string) => {
    if (!confirm('Are you sure you want to restore this snapshot? Current unsaved work will be quarantined.')) {
      return;
    }
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.restoreSnapshot(id);
      setActionResult(res);
      await loadSnapshots();
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleRollbackUpdate = async () => {
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const res = await updaterClient.rollbackUpdate();
      setActionResult(res);
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

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

  const handleResetDrivers = async () => {
    setIsLoading(true);
    setErrorMsg(null);
    try {
      const rep = await updaterClient.resetDrivers();
      setDriverReport(rep);
      setActionResult({
        success: true,
        action: 'reset_drivers',
        message: 'Hardware drivers and display output reset to factory safe-mode defaults.',
        details: [
          'Display resolution forced to 1920x1080@60Hz safe mode.',
          'External USB Wi-Fi adapter (rtl88xxau) driver stack flushed and reloaded.',
          'Audio sink reset to default primary hardware output.',
        ],
        timestamp: new Date().toISOString(),
      });
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

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
      await loadSnapshots();
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
      <header className="px-6 py-4 border-b border-amber-900/30 bg-slate-950/80 backdrop-blur-md sticky top-0 z-20 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-amber-600 to-red-600 flex items-center justify-center shadow-lg shadow-amber-500/10">
            <ShieldAlert className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-lg font-semibold tracking-tight text-white flex items-center gap-2">
              MindSet OS Recovery Console
              <span className="text-[10px] px-2 py-0.5 rounded-full bg-amber-950/80 border border-amber-600/50 text-amber-300 font-mono">
                STANDALONE RESCUE
              </span>
            </h1>
            <p className="text-xs text-slate-400">
              Emergency recovery subsystem for rollbacks, encrypted snapshots, SQLite repairs & hardware resets
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

        {/* 2. Navigation Tabs */}
        <div className="flex flex-wrap items-center gap-2 border-b border-slate-800 pb-2">
          {[
            { id: 'snapshots', label: 'System Snapshots', icon: History, badge: `${snapshots.length}` },
            { id: 'rollback', label: 'Instant 3s Rollback', icon: RotateCcw, badge: 'A/B' },
            { id: 'database', label: 'Database Doctor', icon: Database, badge: 'SQLite' },
            { id: 'hardware', label: 'Hardware Driver Reset', icon: Monitor, badge: 'Safe-Mode' },
            { id: 'factory_reset', label: 'Factory Reset Guard', icon: AlertOctagon, badge: 'Protected' },
          ].map((tab) => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as typeof activeTab)}
                className={`flex items-center gap-2 px-3.5 py-2.5 rounded-xl text-xs font-medium transition-all ${
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

        {/* TAB 1: System Snapshots & Time-Machine */}
        {activeTab === 'snapshots' && (
          <div className="space-y-6">
            {/* Create Snapshot Form */}
            <form
              onSubmit={handleCreateSnapshot}
              className="p-4 rounded-2xl bg-slate-900/60 border border-slate-800 flex flex-col sm:flex-row items-center gap-3"
            >
              <div className="flex-1 w-full flex items-center gap-2 px-3 py-1.5 rounded-xl bg-slate-950/80 border border-slate-800">
                <FileArchive className="w-4 h-4 text-amber-400 flex-shrink-0" />
                <input
                  type="text"
                  placeholder="Enter snapshot description (e.g. Before Physics Revision Session)..."
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

            {/* Snapshots List */}
            <div className="space-y-3">
              <h2 className="text-xs font-semibold text-slate-400 uppercase tracking-wider">
                Available Restore Points ({snapshots.length})
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
                          SHA: {snap.checksum_sha256.substring(0, 16)}...
                        </span>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            handleRestoreSnapshot(snap.id);
                          }}
                          disabled={isLoading}
                          className="px-3 py-1 rounded-lg bg-amber-600/20 hover:bg-amber-600/30 text-amber-300 border border-amber-600/40 text-[11px] font-medium transition-colors"
                        >
                          Restore Point
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          </div>
        )}

        {/* TAB 2: Instant 3s Rollback */}
        {activeTab === 'rollback' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-gradient-to-tr from-amber-600 to-orange-600 flex items-center justify-center flex-shrink-0 shadow-lg shadow-amber-500/10">
                <RotateCcw className="w-6 h-6 text-white" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  3-Second Instant A/B Boot Rollback
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  Fedora Kinoite / ostree immutable deployment enables zero-risk system updates. If a newly installed patch triggers graphics driver corruption, kernel panics, or application faults, trigger this instant rollback to swap execution back to the previous deployment slot.
                </p>
              </div>
            </div>

            <div className="p-4 rounded-xl bg-slate-950/70 border border-slate-800/80 grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs">
              <div className="space-y-1">
                <span className="text-slate-400 text-[11px]">Primary Active Deployment</span>
                <span className="font-semibold text-white block">Slot A (Running Kernel 6.11.7)</span>
                <span className="text-slate-500 text-[11px] font-mono">commit: 8f3a90c1e82b71</span>
              </div>
              <div className="space-y-1">
                <span className="text-slate-400 text-[11px]">Rollback Standby Image</span>
                <span className="font-semibold text-amber-400 block">Slot B (Known-Good Baseline)</span>
                <span className="text-slate-500 text-[11px] font-mono">commit: 7b29a10fc41e90</span>
              </div>
            </div>

            <button
              onClick={handleRollbackUpdate}
              disabled={isLoading}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-amber-600 hover:bg-amber-500 text-white text-xs font-semibold shadow-lg shadow-amber-600/20 transition-all disabled:opacity-50"
            >
              <RotateCcw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
              Trigger 3-Second Rollback Now
            </button>
          </div>
        )}

        {/* TAB 3: Database Doctor */}
        {activeTab === 'database' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-cyan-600/20 border border-cyan-500/30 flex items-center justify-center flex-shrink-0">
                <Database className="w-6 h-6 text-cyan-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  SQLite Database & Vector Store Doctor
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
              Run Deep Database Repair
            </button>
          </div>
        )}

        {/* TAB 4: Hardware Driver Reset */}
        {activeTab === 'hardware' && (
          <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-6">
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 rounded-2xl bg-indigo-600/20 border border-indigo-500/30 flex items-center justify-center flex-shrink-0">
                <Monitor className="w-6 h-6 text-indigo-400" />
              </div>
              <div className="space-y-1">
                <h2 className="text-base font-semibold text-white">
                  Hardware & Safe-Mode Display Reset
                </h2>
                <p className="text-xs text-slate-400 leading-relaxed max-w-2xl">
                  If display resolution is set to an unsupported mode or external USB Wi-Fi drivers (<code className="font-mono text-indigo-300">rtl88xxau</code>) hang, this routine resets video output to standard 1080p@60Hz and re-probes network and audio interfaces.
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
              onClick={handleResetDrivers}
              disabled={isLoading}
              className="flex items-center gap-2 px-5 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition-all disabled:opacity-50"
            >
              <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
              Reset Drivers to Safe-Mode Defaults
            </button>
          </div>
        )}

        {/* TAB 5: Factory Reset Guard */}
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
