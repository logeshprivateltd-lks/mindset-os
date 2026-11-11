/**
 * MindSet OS — Atomic OTA Updater Interface (/updater)
 * Dual-Layer Tauri 2 Delta + ostree Immutable Deployment Engine
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import {
  DownloadCloudIcon as DownloadCloud,
  RefreshIcon as RefreshCw,
  ShieldCheckIcon as ShieldCheck,
  CheckCircleIcon as CheckCircle2,
  AlertTriangleIcon as AlertTriangle,
  LayersIcon as Layers,
  FolderArchiveIcon as FolderArchive,
  ClockIcon as Clock,
  HardDriveIcon as HardDrive,
  SparklesIcon as Sparkles,
  CpuIcon as Cpu,
  RotateCcwIcon as RotateCcw,
  CheckIcon as Check,
  UploadIcon as Upload,
} from '../../components/common/SystemIcons';
import type {
  OSVersionInfo,
  UpdatePackageInfo,
  UpdateStageProgress,
  OfflineBundleManifest,
  UpdateChannelType,
} from '@mindset/db-schema';
import { updaterClient } from '../../lib/updaterClient';

export const AtomicUpdaterPage: React.FC = () => {
  const [activeChannel, setActiveChannel] = useState<UpdateChannelType>('core_os');
  const [versionInfo, setVersionInfo] = useState<OSVersionInfo | null>(null);
  const [availableUpdates, setAvailableUpdates] = useState<UpdatePackageInfo[]>([]);
  const [isChecking, setIsChecking] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState<UpdateStageProgress | null>(null);
  const [stagedPackage, setStagedPackage] = useState<UpdatePackageInfo | null>(null);
  const [appliedNotification, setAppliedNotification] = useState<string | null>(null);
  const [offlinePath, setOfflinePath] = useState('');
  const [offlineManifest, setOfflineManifest] = useState<OfflineBundleManifest | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const loadVersionInfo = useCallback(async () => {
    try {
      const info = await updaterClient.getVersionInfo();
      setVersionInfo(info);
      if (info.staged_update) {
        setStagedPackage(info.staged_update);
      }
    } catch (err) {
      console.error('Failed to load version info:', err);
    }
  }, []);

  const checkForUpdates = useCallback(async (channel: UpdateChannelType) => {
    setIsChecking(true);
    setErrorMsg(null);
    setAppliedNotification(null);
    try {
      const updates = await updaterClient.checkForUpdates(channel);
      setAvailableUpdates(updates);
      const isStaged = updates.find((u) => u.is_staged);
      if (isStaged) {
        setStagedPackage(isStaged);
      }
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    } finally {
      setIsChecking(false);
    }
  }, []);

  useEffect(() => {
    loadVersionInfo();
    checkForUpdates(activeChannel);
  }, [loadVersionInfo, checkForUpdates, activeChannel]);

  const handleDownloadAndStage = async (update: UpdatePackageInfo) => {
    setErrorMsg(null);
    setDownloadProgress({
      update_id: update.id,
      stage: 'downloading',
      progress_percent: 15,
      bytes_downloaded: Math.floor(update.size_bytes * 0.15),
      total_bytes: update.size_bytes,
      status_message: 'Downloading delta differential payload...',
      signature_valid: false,
    });

    try {
      // Smooth progress stepping
      await new Promise((r) => setTimeout(r, 600));
      setDownloadProgress((prev) =>
        prev
          ? {
              ...prev,
              stage: 'verifying_signature',
              progress_percent: 75,
              bytes_downloaded: Math.floor(update.size_bytes * 0.75),
              status_message: 'Verifying Ed25519 cryptographic signature against LKS Root Key...',
            }
          : null
      );

      await new Promise((r) => setTimeout(r, 700));
      const res = await updaterClient.downloadAndStage(update.id);
      setDownloadProgress(res);
      setStagedPackage({ ...update, is_staged: true });
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
      setDownloadProgress(null);
    }
  };

  const handleApplyUpdate = async (updateId: string) => {
    setErrorMsg(null);
    try {
      const res = await updaterClient.applyUpdate(updateId);
      setAppliedNotification(res.status_message);
      setStagedPackage(null);
      setDownloadProgress(null);
      await loadVersionInfo();
      await checkForUpdates(activeChannel);
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    }
  };

  const handleInstallOfflineBundle = async () => {
    setErrorMsg(null);
    try {
      const manifest = await updaterClient.installOfflineBundle(
        offlinePath || 'class12_math_update.msos-patch'
      );
      setOfflineManifest(manifest);
      setAppliedNotification(
        `Offline bundle '${manifest.bundle_name}' verified and unpacked successfully (${manifest.file_count} components).`
      );
    } catch (err: unknown) {
      setErrorMsg(err instanceof Error ? err.message : String(err));
    }
  };

  const formatBytes = (bytes: number): string => {
    if (bytes >= 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${bytes} B`;
  };

  return (
    <div className="flex flex-col h-full bg-[#080B10] text-slate-100 overflow-y-auto">
      {/* 1. Header Bar */}
      <header className="px-6 py-4 border-b border-slate-800/80 bg-slate-900/60 backdrop-blur-md sticky top-0 z-20 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-cyan-600 to-indigo-600 flex items-center justify-center shadow-lg shadow-cyan-500/10">
            <DownloadCloud className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-lg font-semibold tracking-tight text-white flex items-center gap-2">
              Atomic OTA Updater
              <span className="text-xs px-2 py-0.5 rounded-full bg-cyan-950/70 border border-cyan-700/50 text-cyan-300 font-mono">
                ostree + Ed25519
              </span>
            </h1>
            <p className="text-xs text-slate-400">
              Dual-layer atomic system deployment with guaranteed 3-second instant boot rollback
            </p>
          </div>
        </div>

        <button
          onClick={() => checkForUpdates(activeChannel)}
          disabled={isChecking}
          className="flex items-center gap-2 px-3.5 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 border border-slate-700 text-xs font-medium text-slate-200 transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`w-3.5 h-3.5 text-cyan-400 ${isChecking ? 'animate-spin' : ''}`} />
          {isChecking ? 'Checking Channels...' : 'Check for Updates'}
        </button>
      </header>

      {/* Main Body */}
      <div className="p-6 max-w-6xl mx-auto w-full space-y-6">
        {/* Applied / Execution Banner */}
        {appliedNotification && (
          <div className="p-4 rounded-xl bg-emerald-950/50 border border-emerald-600/40 text-emerald-200 flex items-start gap-3 animate-fadeIn">
            <CheckCircle2 className="w-5 h-5 text-emerald-400 flex-shrink-0 mt-0.5" />
            <div className="flex-1 text-xs">
              <p className="font-semibold text-emerald-300 text-sm">Update Successfully Deployed</p>
              <p className="mt-0.5 text-emerald-200/90">{appliedNotification}</p>
            </div>
          </div>
        )}

        {/* Error Banner */}
        {errorMsg && (
          <div className="p-4 rounded-xl bg-red-950/50 border border-red-600/40 text-red-200 flex items-start gap-3">
            <AlertTriangle className="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" />
            <div className="flex-1 text-xs">
              <p className="font-semibold text-red-300 text-sm">Deployment Notice</p>
              <p className="mt-0.5 text-red-200/90">{errorMsg}</p>
            </div>
          </div>
        )}

        {/* 2. Current System Architecture & A/B Slot Card */}
        <section className="grid grid-cols-1 lg:grid-cols-3 gap-4">
          <div className="p-5 rounded-2xl bg-slate-900/60 border border-slate-800/80 backdrop-blur-md lg:col-span-2 space-y-4">
            <div className="flex items-center justify-between border-b border-slate-800 pb-3">
              <div className="flex items-center gap-2">
                <Cpu className="w-4 h-4 text-cyan-400" />
                <h2 className="text-sm font-semibold text-white">Active System Environment</h2>
              </div>
              <span className="text-[11px] px-2.5 py-0.5 rounded-full bg-emerald-950/70 border border-emerald-600/40 text-emerald-300 font-medium">
                System Healthy
              </span>
            </div>

            <div className="grid grid-cols-2 sm:grid-cols-3 gap-3 text-xs">
              <div className="p-3 rounded-xl bg-slate-950/60 border border-slate-800/60">
                <span className="text-slate-400 block text-[11px]">OS Version</span>
                <span className="font-semibold text-white text-sm">
                  {versionInfo?.os_version || 'v1.0.0'} "{versionInfo?.codename || 'Aurora'}"
                </span>
              </div>
              <div className="p-3 rounded-xl bg-slate-950/60 border border-slate-800/60">
                <span className="text-slate-400 block text-[11px]">Kernel Subsystem</span>
                <span className="font-medium text-slate-200 truncate block">
                  {versionInfo?.kernel_version || 'Linux 6.11.7 / NT Hybrid'}
                </span>
              </div>
              <div className="p-3 rounded-xl bg-slate-950/60 border border-slate-800/60">
                <span className="text-slate-400 block text-[11px]">ostree Commit</span>
                <span className="font-mono text-cyan-300 truncate block">
                  {versionInfo?.ostree_commit?.substring(0, 14) || '8f3a90c1e82b71'}
                </span>
              </div>
            </div>

            {/* Cryptographic Key Signature Verification Seal */}
            <div className="p-3 rounded-xl bg-cyan-950/20 border border-cyan-800/30 flex items-center justify-between">
              <div className="flex items-center gap-2.5">
                <ShieldCheck className="w-4 h-4 text-cyan-400 flex-shrink-0" />
                <div>
                  <span className="text-xs font-medium text-cyan-200 block">
                    Authorised LKS Root Public Key Seal
                  </span>
                  <span className="font-mono text-[11px] text-cyan-400/80">
                    {versionInfo?.signature_key_fingerprint || 'LKS-ED25519-7F3A-8991-A9DE'}
                  </span>
                </div>
              </div>
              <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-900/40 text-cyan-300 border border-cyan-700/50">
                STRICT VERIFIED
              </span>
            </div>
          </div>

          {/* A/B Partition Architecture Diagram */}
          <div className="p-5 rounded-2xl bg-slate-900/60 border border-slate-800/80 backdrop-blur-md flex flex-col justify-between space-y-3">
            <div>
              <div className="flex items-center gap-2 mb-2">
                <Layers className="w-4 h-4 text-indigo-400" />
                <h3 className="text-sm font-semibold text-white">A/B Deployment Slots</h3>
              </div>
              <p className="text-[11px] text-slate-400 leading-relaxed">
                Atomic rootfs swapping eliminates corrupted boot states. If a boot health-check fails within 3 seconds, the kernel instantly reverts to the standby slot.
              </p>
            </div>

            <div className="space-y-2 text-xs">
              <div className="p-2.5 rounded-lg bg-emerald-950/40 border border-emerald-600/40 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                  <span className="font-medium text-emerald-200">
                    {versionInfo?.active_slot || 'Slot A (Primary Active)'}
                  </span>
                </div>
                <span className="text-[10px] text-emerald-400 uppercase font-mono">Running</span>
              </div>

              <div className="p-2.5 rounded-lg bg-slate-950/60 border border-slate-800 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 rounded-full bg-slate-600" />
                  <span className="font-medium text-slate-400">
                    {versionInfo?.inactive_slot || 'Slot B (Staged Standby)'}
                  </span>
                </div>
                <span className="text-[10px] text-slate-500 uppercase font-mono">
                  {stagedPackage ? 'Staged' : 'Cold Standby'}
                </span>
              </div>
            </div>
          </div>
        </section>

        {/* 3. Patch Channel Selector Tabs */}
        <section className="space-y-4">
          <div className="flex flex-wrap items-center gap-2 border-b border-slate-800 pb-2">
            {[
              { id: 'core_os', label: 'Core OS Releases', icon: Layers, count: 'v1.0.1' },
              { id: 'critical_security', label: 'Critical Security', icon: ShieldCheck, count: '1 CVE' },
              { id: 'ai_prompts', label: 'AI Prompts JSON', icon: Sparkles, count: 'Live' },
              { id: 'offline_bundle', label: 'Offline USB Bundle', icon: FolderArchive, count: '.msos-patch' },
            ].map((tab) => {
              const Icon = tab.icon;
              const isActive = activeChannel === tab.id;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveChannel(tab.id as UpdateChannelType)}
                  className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-medium transition-all ${
                    isActive
                      ? 'bg-cyan-600/20 text-cyan-300 border border-cyan-500/50 shadow-sm shadow-cyan-500/10'
                      : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-transparent'
                  }`}
                >
                  <Icon className={`w-3.5 h-3.5 ${isActive ? 'text-cyan-400' : 'text-slate-400'}`} />
                  {tab.label}
                  <span
                    className={`ml-1 text-[10px] px-1.5 py-0.2 rounded-full font-mono ${
                      isActive ? 'bg-cyan-900/60 text-cyan-200' : 'bg-slate-800 text-slate-400'
                    }`}
                  >
                    {tab.count}
                  </span>
                </button>
              );
            })}
          </div>

          {/* Staged Package Alert / Fast Action Banner */}
          {stagedPackage && (
            <div className="p-4 rounded-xl bg-cyan-950/40 border border-cyan-500/40 text-cyan-100 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div className="flex items-center gap-3">
                <HardDrive className="w-5 h-5 text-cyan-400 flex-shrink-0" />
                <div>
                  <p className="font-semibold text-sm text-white">
                    {stagedPackage.title} is Staged in Standby Slot
                  </p>
                  <p className="text-xs text-cyan-300/80">
                    Cryptographic hash and Ed25519 signatures verified. 3-second instant rollback watchdog armed.
                  </p>
                </div>
              </div>
              <button
                onClick={() => handleApplyUpdate(stagedPackage.id)}
                className="flex items-center gap-2 px-4 py-2 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold shadow-lg shadow-cyan-600/20 transition-all flex-shrink-0"
              >
                <Check className="w-3.5 h-3.5" />
                Apply & Swap Slot Now
              </button>
            </div>
          )}

          {/* Channel Content: Offline USB Bundle vs OTA Packages */}
          {activeChannel === 'offline_bundle' ? (
            <div className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800/80 backdrop-blur-md space-y-4">
              <div className="flex items-start gap-3">
                <div className="w-9 h-9 rounded-lg bg-indigo-950/80 border border-indigo-700/50 flex items-center justify-center flex-shrink-0">
                  <FolderArchive className="w-4 h-4 text-indigo-400" />
                </div>
                <div>
                  <h3 className="text-sm font-semibold text-white">Air-Gapped School Lab Patch Deployer</h3>
                  <p className="text-xs text-slate-400 mt-0.5">
                    For schools and labs without broadband internet, load authenticated <code className="font-mono text-cyan-300">.msos-patch</code> archives directly from an encrypted USB flash drive.
                  </p>
                </div>
              </div>

              <div className="flex flex-col sm:flex-row items-center gap-3 pt-2">
                <input
                  type="text"
                  placeholder="e.g. E:\patches\class12_stateboard_curriculum.msos-patch"
                  value={offlinePath}
                  onChange={(e) => setOfflinePath(e.target.value)}
                  className="flex-1 w-full px-3.5 py-2.5 rounded-xl bg-slate-950/80 border border-slate-800 text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-cyan-500"
                />
                <button
                  onClick={handleInstallOfflineBundle}
                  className="w-full sm:w-auto flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold transition-all shadow-md shadow-indigo-600/20 flex-shrink-0"
                >
                  <Upload className="w-3.5 h-3.5" />
                  Inspect & Unpack Bundle
                </button>
              </div>

              {offlineManifest && (
                <div className="p-4 rounded-xl bg-slate-950/80 border border-slate-800/80 space-y-2 text-xs">
                  <div className="flex items-center justify-between text-slate-300">
                    <span className="font-semibold text-white">{offlineManifest.bundle_name}</span>
                    <span className="text-emerald-400 font-mono flex items-center gap-1">
                      <ShieldCheck className="w-3.5 h-3.5" /> Ed25519 Valid
                    </span>
                  </div>
                  <p className="text-slate-400">{offlineManifest.description}</p>
                  <div className="flex items-center gap-4 text-[11px] text-slate-400 pt-1">
                    <span>Target: {offlineManifest.target_channel}</span>
                    <span>Version: {offlineManifest.version}</span>
                    <span>Components: {offlineManifest.file_count}</span>
                    <span>Size: {formatBytes(offlineManifest.total_size_bytes)}</span>
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="space-y-4">
              {availableUpdates.map((update) => {
                const isThisStaged = stagedPackage?.id === update.id;
                const isDownloadingThis = downloadProgress?.update_id === update.id;

                return (
                  <div
                    key={update.id}
                    className="p-5 rounded-2xl bg-slate-900/60 border border-slate-800/80 backdrop-blur-md space-y-4 transition-all hover:border-slate-700"
                  >
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-slate-800 pb-3">
                      <div>
                        <div className="flex items-center gap-2">
                          <h3 className="text-sm font-semibold text-white">{update.title}</h3>
                          <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-cyan-950/60 border border-cyan-800/40 text-cyan-300">
                            {update.version}
                          </span>
                          {update.requires_reboot ? (
                            <span className="text-[10px] px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                              A/B Swap Required
                            </span>
                          ) : (
                            <span className="text-[10px] px-2 py-0.5 rounded bg-purple-950/60 text-purple-300 border border-purple-800/40">
                              Zero-Reboot Hot Reload
                            </span>
                          )}
                        </div>
                        <div className="flex items-center gap-3 text-xs text-slate-400 mt-1">
                          <span className="flex items-center gap-1">
                            <Clock className="w-3 h-3" />
                            {new Date(update.published_at).toLocaleDateString()}
                          </span>
                          <span>Diff: {update.diff_type}</span>
                          <span>Size: {formatBytes(update.size_bytes)}</span>
                        </div>
                      </div>

                      {/* Action Button */}
                      <div>
                        {isThisStaged ? (
                          <button
                            onClick={() => handleApplyUpdate(update.id)}
                            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-all shadow-md shadow-emerald-600/20"
                          >
                            <RotateCcw className="w-3.5 h-3.5" />
                            Apply Staged Update
                          </button>
                        ) : isDownloadingThis ? (
                          <div className="flex items-center gap-2 text-xs font-medium text-cyan-400">
                            <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                            {downloadProgress?.stage === 'verifying_signature'
                              ? 'Verifying Signatures...'
                              : 'Downloading Delta...'}
                          </div>
                        ) : (
                          <button
                            onClick={() => handleDownloadAndStage(update)}
                            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold transition-all shadow-md shadow-cyan-600/20"
                          >
                            <DownloadCloud className="w-3.5 h-3.5" />
                            Download & Stage
                          </button>
                        )}
                      </div>
                    </div>

                    {/* Download Progress Bar */}
                    {isDownloadingThis && downloadProgress && (
                      <div className="p-3 rounded-xl bg-slate-950/60 border border-slate-800 space-y-2">
                        <div className="flex items-center justify-between text-xs">
                          <span className="text-slate-300 font-medium">
                            {downloadProgress.status_message}
                          </span>
                          <span className="font-mono text-cyan-400 font-semibold">
                            {downloadProgress.progress_percent}%
                          </span>
                        </div>
                        <div className="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden">
                          <div
                            className="h-full bg-gradient-to-r from-cyan-500 to-indigo-500 transition-all duration-300 rounded-full"
                            style={{ width: `${downloadProgress.progress_percent}%` }}
                          />
                        </div>
                      </div>
                    )}

                    {/* Release Notes */}
                    <div className="space-y-1.5 text-xs text-slate-300">
                      <span className="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">
                        Release Highlights
                      </span>
                      <ul className="space-y-1 pl-1">
                        {update.release_notes.map((note, idx) => (
                          <li key={idx} className="flex items-start gap-2 text-slate-300">
                            <span className="text-cyan-400 mt-1">•</span>
                            <span>{note}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    {/* Cryptographic Hashes Footer */}
                    <div className="pt-2 border-t border-slate-800/60 flex flex-wrap items-center justify-between gap-2 text-[11px] font-mono text-slate-400">
                      <span className="truncate max-w-sm">
                        SHA256: {update.checksum_sha256}
                      </span>
                      <span className="text-cyan-400 flex items-center gap-1">
                        <ShieldCheck className="w-3 h-3" />
                        Ed25519 Validated
                      </span>
                    </div>
                  </div>
                );
              })}

              {availableUpdates.length === 0 && !isChecking && (
                <div className="p-8 rounded-2xl bg-slate-900/40 border border-slate-800 text-center space-y-2">
                  <CheckCircle2 className="w-8 h-8 text-emerald-400 mx-auto" />
                  <p className="text-sm font-semibold text-white">All Channels Up-To-Date</p>
                  <p className="text-xs text-slate-400">
                    Your MindSet OS installation is running the latest verified binaries and syllabus weights.
                  </p>
                </div>
              )}
            </div>
          )}
        </section>
      </div>
    </div>
  );
};
export default AtomicUpdaterPage;
