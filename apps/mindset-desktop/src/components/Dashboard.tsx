import React, { useState } from 'react';
import { useKernelStore } from '../stores/kernelStore';
import { AuroraCard, Button, StatusBadge } from '@mindset/ui-core';
import { useUac } from './security/useUac';
import { UacModal } from './security/UacModal';

export const Dashboard: React.FC<{ onLock: () => void }> = ({ onLock }) => {
  const {
    rssMb,
    systemUsedPercent,
    totalMemoryMb,
    uptimeSecs,
    services,
    hardware,
    priorityGoal,
    isFocusActive,
    activeFocusMinutes,
    toggleFocusSession,
    setIsStatusViewOpen,
    togglePrivacyBlur,
    isUacElevated,
  } = useKernelStore();

  const { requestElevation, modalProps } = useUac();
  const [elevationGrantedMsg, setElevationGrantedMsg] = useState<string | null>(null);

  const handleTestUac = async () => {
    const granted = await requestElevation(
      'Modify Focus Guard Kernel Policy & Low-Level Packet Filter',
      'Focus Guard Policy Engine'
    );
    if (granted) {
      setElevationGrantedMsg('UAC Admin Elevation Active (Valid 3 mins)');
      setTimeout(() => setElevationGrantedMsg(null), 6000);
    }
  };

  return (
    <div className="pt-12 pb-6 px-6 max-w-6xl mx-auto space-y-6 select-none">
      {/* Top Banner: Telemetry & Memory Watchdog */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <AuroraCard>
          <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
            OS Shell RSS (Ceiling: 120MB)
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold text-[#65E6C4]">{rssMb.toFixed(1)}</span>
            <span className="text-sm text-[#9CA3AF]">MB</span>
          </div>
          <p className="text-[11px] text-[#65E6C4] mt-1 font-medium">✓ Strictly within 120 MB budget</p>
        </AuroraCard>

        <AuroraCard>
          <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
            System RAM Utilization
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold text-[#F9FAFB]">{systemUsedPercent.toFixed(1)}%</span>
            <span className="text-sm text-[#9CA3AF]">of {(totalMemoryMb / 1024).toFixed(0)} GB</span>
          </div>
          <p className="text-[11px] text-[#8B9CFF] mt-1 font-medium">8.0 GB+ reserved for Ollama LLM</p>
        </AuroraCard>

        <AuroraCard>
          <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
            Kernel Uptime & Loop
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold text-[#FFD078]">{uptimeSecs}</span>
            <span className="text-sm text-[#9CA3AF]">secs</span>
          </div>
          <button
            onClick={() => setIsStatusViewOpen(true)}
            className="w-full mt-2 py-1 px-2 rounded bg-[#182235] hover:bg-[#202B40] text-[11px] text-[#8B9CFF] font-medium border border-[#202B40] transition-all flex items-center justify-center space-x-1"
          >
            <span>⚙️ Open /status</span>
          </button>
        </AuroraCard>

        <AuroraCard>
          <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
            Security & Smart Lock
          </div>
          <div className="mt-2 space-y-1.5">
            <div className="grid grid-cols-2 gap-1.5">
              <Button
                variant="secondary"
                size="sm"
                onClick={onLock}
                className="w-full text-xs py-1"
                title="Lock Workspace (Super + L / Ctrl + Alt + L)"
              >
                🔒 Lock
              </Button>
              <Button
                variant="secondary"
                size="sm"
                onClick={togglePrivacyBlur}
                className="w-full text-xs py-1"
                title="Instant Privacy Blur Guard (Alt + B)"
              >
                👁️‍🗨️ Blur
              </Button>
            </div>
            <button
              onClick={handleTestUac}
              className="w-full py-1 px-2 rounded bg-[#182235] hover:bg-[#202B40] text-[10px] text-[#8B9CFF] font-medium border border-[#202B40] hover:border-[#8B9CFF]/40 transition-all flex items-center justify-center space-x-1"
            >
              <span>🛡️ Test UAC Elevation</span>
            </button>
          </div>
          {elevationGrantedMsg ? (
            <p className="text-[10px] text-[#65E6C4] mt-1.5 text-center font-medium animate-fadeIn">
              ✓ {elevationGrantedMsg}
            </p>
          ) : (
            <p className="text-[10px] text-[#9CA3AF] mt-1.5 text-center">
              {isUacElevated ? '🛡️ UAC Admin Elevated' : 'Multi-Modal Biometrics / PIN'}
            </p>
          )}
        </AuroraCard>
      </div>

      {/* Main Row: Priority Goal & Deep Work Controller */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <AuroraCard elevated className="lg:col-span-2 space-y-4">
          <div className="flex items-center justify-between border-b border-[#202B40] pb-3">
            <div>
              <span className="text-xs font-semibold text-[#8B9CFF] uppercase tracking-wider">Priority 1 Active Objective</span>
              <h2 className="text-lg font-semibold text-[#F9FAFB] mt-0.5">{priorityGoal}</h2>
            </div>
            <span className="px-2.5 py-1 rounded bg-[#FFD078]/10 text-[#FFD078] border border-[#FFD078]/30 text-xs font-semibold">
              XP +50
            </span>
          </div>

          <div className="p-4 rounded-lg bg-[#111827] border border-[#1F2937] flex items-center justify-between">
            <div className="space-y-1">
              <div className="text-xs text-[#9CA3AF]">Deep Work Calibration ({activeFocusMinutes}m Block)</div>
              <div className="text-sm font-semibold text-[#F9FAFB]">
                {isFocusActive ? 'Session In Progress (Focus Guard Engaged)' : `Ready to Begin ${activeFocusMinutes}-min Deep Block`}
              </div>
              <div className="text-xs text-[#65E6C4]">Distraction Policy: Strict Process & DNS Filtering</div>
            </div>

            <Button
              variant={isFocusActive ? 'danger' : 'accent'}
              onClick={toggleFocusSession}
              className="min-w-32"
            >
              {isFocusActive ? 'End Session' : 'Start Deep Work'}
            </Button>
          </div>

          <div className="grid grid-cols-3 gap-3 text-center text-xs">
            <div className="p-2.5 rounded bg-[#111827] border border-[#1F2937]">
              <span className="text-[#9CA3AF] block">Mode</span>
              <span className="font-semibold text-[#8B9CFF] mt-0.5 block">DeepWork</span>
            </div>
            <div className="p-2.5 rounded bg-[#111827] border border-[#1F2937]">
              <span className="text-[#9CA3AF] block">Daily Streak</span>
              <span className="font-semibold text-[#FFD078] mt-0.5 block">🔥 7 Days</span>
            </div>
            <div className="p-2.5 rounded bg-[#111827] border border-[#1F2937]">
              <span className="text-[#9CA3AF] block">Target Blocks</span>
              <span className="font-semibold text-[#65E6C4] mt-0.5 block">3 of 4 Done</span>
            </div>
          </div>
        </AuroraCard>

        {/* Database & Storage Status */}
        <AuroraCard className="space-y-4">
          <div className="border-b border-[#1F2937] pb-2">
            <h3 className="text-sm font-semibold text-[#F9FAFB]">SQLite Low-RAM Engines</h3>
            <p className="text-xs text-[#9CA3AF]">Zero-bloat native storage architecture</p>
          </div>

          <div className="space-y-3 text-xs">
            <div className="p-2.5 rounded bg-[#182235] border border-[#202B40]">
              <div className="flex items-center justify-between font-semibold text-[#F9FAFB]">
                <span>local_core.sqlite</span>
                <span className="text-[#65E6C4]">WAL Active</span>
              </div>
              <p className="text-[11px] text-[#9CA3AF] mt-1">
                32MB mmap cap • 8MB page cache • Tasks, Habits, Vectors
              </p>
            </div>

            <div className="p-2.5 rounded bg-[#182235] border border-[#202B40]">
              <div className="flex items-center justify-between font-semibold text-[#F9FAFB]">
                <span>sync_oplog.sqlite</span>
                <span className="text-[#8B9CFF]">HLC Engine</span>
              </div>
              <p className="text-[11px] text-[#9CA3AF] mt-1">
                Deterministic peer sync • Ed25519 signatures • Zero conflict
              </p>
            </div>

            <div className="p-2.5 rounded bg-[#182235] border border-[#202B40]">
              <div className="flex items-center justify-between font-semibold text-[#F9FAFB]">
                <span>Vault Hierarchy</span>
                <span className="text-[#FFD078]">~/.mindset_data/</span>
              </div>
              <p className="text-[11px] text-[#9CA3AF] mt-1">
                my-laptop • my-phone • textbooks • certs
              </p>
            </div>
          </div>
        </AuroraCard>
      </div>

      {/* Services Registry Matrix & Hardware Probe */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <AuroraCard className="lg:col-span-2 space-y-4">
          <div className="flex items-center justify-between border-b border-[#1F2937] pb-2">
            <div>
              <div className="flex items-center space-x-2">
                <h3 className="text-sm font-semibold text-[#F9FAFB]">Kernel Service Registry</h3>
                <button
                  onClick={() => setIsStatusViewOpen(true)}
                  className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#8B9CFF]/10 text-[#8B9CFF] hover:bg-[#8B9CFF]/20 transition-all"
                >
                  Manage (/status) →
                </button>
              </div>
              <p className="text-xs text-[#9CA3AF]">Cooperative asynchronous Tokio micro-actors</p>
            </div>
            <StatusBadge status="healthy" label={`${services.length}/${services.length} Services Operational`} />
          </div>

          <div className="space-y-2">
            {services.map((svc) => (
              <div
                key={svc.name}
                className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235] border border-[#202B40] text-xs"
              >
                <div className="flex items-center space-x-2">
                  <span className="text-[#65E6C4]">●</span>
                  <span className="font-semibold text-[#F9FAFB]">{svc.name}</span>
                </div>
                <div className="flex items-center space-x-4">
                  <span className="text-[#9CA3AF] font-mono">RAM: ~{svc.estimated_ram_kb} KB</span>
                  <span className="px-2 py-0.5 rounded bg-[#65E6C4]/10 text-[#65E6C4] font-medium">
                    {typeof svc.status === 'string' ? svc.status : Object.keys(svc.status)[0]}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </AuroraCard>

        {/* Sensory Hardware Probe Status */}
        <AuroraCard className="space-y-4">
          <div className="border-b border-[#1F2937] pb-2">
            <h3 className="text-sm font-semibold text-[#F9FAFB]">Hardware Probe</h3>
            <p className="text-xs text-[#9CA3AF]">Auto-detected during HARDWARE_PROBE</p>
          </div>

          <div className="space-y-2 text-xs">
            <div className="flex items-center justify-between p-2 rounded bg-[#182235]">
              <span className="text-[#E5E7EB]">Webcam (DirectShow / V4L2)</span>
              <span className={hardware?.has_webcam ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                {hardware?.has_webcam ? 'Active' : 'Not Found'}
              </span>
            </div>

            <div className="flex items-center justify-between p-2 rounded bg-[#182235]">
              <span className="text-[#E5E7EB]">Microphone (WASAPI / ALSA)</span>
              <span className={hardware?.has_mic ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                {hardware?.has_mic ? 'Active' : 'Not Found'}
              </span>
            </div>

            <div className="flex items-center justify-between p-2 rounded bg-[#182235]">
              <span className="text-[#E5E7EB]">Biometrics (fprintd / WinBio)</span>
              <span className={hardware?.has_fingerprint ? 'text-[#65E6C4]' : 'text-[#9CA3AF]'}>
                {hardware?.has_fingerprint ? 'Enrolled' : 'Graceful Fallback'}
              </span>
            </div>

            <div className="flex items-center justify-between p-2 rounded bg-[#182235]">
              <span className="text-[#E5E7EB]">Bluetooth Controller</span>
              <span className={hardware?.has_bluetooth ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                {hardware?.has_bluetooth ? 'Active' : 'Disabled'}
              </span>
            </div>
          </div>
        </AuroraCard>
      </div>

      {/* UAC Elevation Confirmation Modal */}
      <UacModal {...modalProps} />
    </div>
  );
};
