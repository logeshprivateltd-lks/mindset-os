import React, { useEffect, useState } from 'react';
import { useKernelStore } from '../stores/useKernelStore';
import type { KernelBootPhase } from '@mindset/db-schema';

interface BootSplashGateProps {
  children: React.ReactNode;
}

const PHASES: Array<{
  id: KernelBootPhase;
  label: string;
  subtext: string;
  badge: string;
}> = [
  {
    id: 'BOOT_INIT',
    label: '1. Vault & Storage Initializer',
    subtext: 'Validating ~/.mindset_data/ layout & vault directories',
    badge: 'Paths Ready',
  },
  {
    id: 'DB_MIGRATE',
    label: '2. SQLite Low-RAM Migrations',
    subtext: 'WAL mode, 32MB mmap limit, 8MB cache applied',
    badge: 'WAL Active',
  },
  {
    id: 'HARDWARE_PROBE',
    label: '3. Sensory Hardware Probe',
    subtext: 'Webcam, Mic, Bluetooth, & USB Wi-Fi (Realtek) scan',
    badge: 'Hardware Verified',
  },
  {
    id: 'SERVICE_REGISTER',
    label: '4. Service Registry Spawning',
    subtext: 'MemoryWatchdog (<120MB), Scheduler, OllamaHealth',
    badge: 'Services Running',
  },
  {
    id: 'SYSTEM_READY',
    label: '5. System Kernel Ready',
    subtext: 'Typed Event Bus active, webview bridge unlocked',
    badge: 'Kernel Primed',
  },
];

export const BootSplashGate: React.FC<BootSplashGateProps> = ({ children }) => {
  const {
    bootPhase,
    isReady,
    rssMb,
    hardware,
    fetchKernelStatus,
    initEventListeners,
  } = useKernelStore();

  const [fadeComplete, setFadeComplete] = useState(false);
  const [isTransitioning, setIsTransitioning] = useState(false);

  // Initialize kernel event listener and initial status poll
  useEffect(() => {
    let unlistenFn: (() => void) | undefined;

    const setup = async () => {
      unlistenFn = await initEventListeners();
      await fetchKernelStatus();
    };

    setup();

    return () => {
      if (unlistenFn) unlistenFn();
    };
  }, [initEventListeners, fetchKernelStatus]);

  // When system reaches SYSTEM_READY, wait slightly and transition with 220ms fade
  useEffect(() => {
    if (isReady || bootPhase === 'SYSTEM_READY') {
      const timer = setTimeout(() => {
        setIsTransitioning(true);
        const fadeTimer = setTimeout(() => {
          setFadeComplete(true);
        }, 220); // 220ms smooth fade transition
        return () => clearTimeout(fadeTimer);
      }, 400);

      return () => clearTimeout(timer);
    }
  }, [isReady, bootPhase]);

  if (fadeComplete) {
    return <>{children}</>;
  }

  const currentIdx = PHASES.findIndex((p) => p.id === bootPhase);
  const effectiveIdx = currentIdx === -1 ? 0 : currentIdx;
  const progressPercent = Math.min(100, Math.round(((effectiveIdx + 1) / PHASES.length) * 100));

  return (
    <div
      className={`fixed inset-0 z-50 flex items-center justify-center bg-[#07090D] px-6 select-none transition-opacity duration-[220ms] ease-out ${
        isTransitioning ? 'opacity-0 pointer-events-none' : 'opacity-100'
      }`}
    >
      <div className="w-full max-w-lg p-6 sm:p-8 rounded-2xl bg-[#111827] border border-[#1F2937] shadow-[0_12px_48px_rgba(0,0,0,0.7)] space-y-6">
        {/* Header & Logo */}
        <div className="flex items-center justify-between border-b border-[#1F2937] pb-4">
          <div className="flex items-center space-x-3">
            <div className="relative flex items-center justify-center w-8 h-8 rounded-xl bg-[#182235] border border-[#8B9CFF]/40 shadow-[0_0_16px_rgba(139,156,255,0.25)]">
              <div className="w-3 h-3 rounded-full bg-[#8B9CFF] animate-pulse" />
            </div>
            <div>
              <h1 className="text-sm font-semibold text-[#F9FAFB] tracking-wide">
                MindSet OS <span className="text-[#8B9CFF]">Kernel</span>
              </h1>
              <p className="text-[11px] text-[#9CA3AF]">LKS PVT LTD — Ultra-Low-RAM Native Architecture</p>
            </div>
          </div>

          <div className="text-right">
            <span className="text-[10px] text-[#9CA3AF] uppercase tracking-wider block">RSS Budget</span>
            <span className="text-xs font-mono font-bold text-[#65E6C4]">{rssMb.toFixed(1)} MB / 120 MB</span>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="space-y-1.5">
          <div className="flex items-center justify-between text-xs">
            <span className="text-[#9CA3AF]">Boot Sequence Progress</span>
            <span className="font-mono text-[#8B9CFF] font-semibold">{progressPercent}%</span>
          </div>
          <div className="h-1.5 w-full bg-[#182235] rounded-full overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-[#8B9CFF] via-[#65E6C4] to-[#FFD078] transition-all duration-300 rounded-full"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
        </div>

        {/* 5 Deterministic Phases List */}
        <div className="space-y-2.5">
          {PHASES.map((p, idx) => {
            const isDone = effectiveIdx > idx || bootPhase === 'SYSTEM_READY';
            const isCurrent = effectiveIdx === idx && bootPhase !== 'SYSTEM_READY';

            return (
              <div
                key={p.id}
                className={`p-3 rounded-xl border transition-all duration-200 ${
                  isCurrent
                    ? 'bg-[#182235] border-[#8B9CFF]/50 shadow-[0_0_14px_rgba(139,156,255,0.12)]'
                    : isDone
                    ? 'bg-[#111827] border-[#1F2937]'
                    : 'bg-[#07090D] border-[#1F2937]/50 opacity-40'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span
                      className={`text-xs ${
                        isDone ? 'text-[#65E6C4]' : isCurrent ? 'text-[#8B9CFF] animate-spin' : 'text-[#6B7280]'
                      }`}
                    >
                      {isDone ? '✓' : isCurrent ? '◓' : '○'}
                    </span>
                    <span
                      className={`text-xs font-medium ${
                        isCurrent ? 'text-[#8B9CFF]' : isDone ? 'text-[#F9FAFB]' : 'text-[#6B7280]'
                      }`}
                    >
                      {p.label}
                    </span>
                  </div>

                  <span
                    className={`text-[10px] px-2 py-0.5 rounded font-mono ${
                      isDone
                        ? 'bg-[#65E6C4]/10 text-[#65E6C4] border border-[#65E6C4]/30'
                        : isCurrent
                        ? 'bg-[#8B9CFF]/10 text-[#8B9CFF] border border-[#8B9CFF]/30 animate-pulse'
                        : 'bg-[#1F2937] text-[#6B7280]'
                    }`}
                  >
                    {isDone ? p.badge : isCurrent ? 'Executing...' : 'Waiting'}
                  </span>
                </div>
                <p className="text-[11px] text-[#9CA3AF] mt-1 pl-4">{p.subtext}</p>
              </div>
            );
          })}
        </div>

        {/* Hardware Status Strip */}
        <div className="p-3 rounded-xl bg-[#182235] border border-[#202B40] flex items-center justify-between text-[11px]">
          <div className="flex items-center space-x-3 text-[#9CA3AF]">
            <span>Sensors:</span>
            <span className={hardware?.has_webcam ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
              📷 {hardware?.has_webcam ? 'Webcam' : 'No Cam'}
            </span>
            <span className={hardware?.has_mic ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
              🎙️ {hardware?.has_mic ? 'Mic' : 'No Mic'}
            </span>
            <span className={hardware?.has_wifi_monitor ? 'text-[#8B9CFF]' : 'text-[#6B7280]'}>
              📶 {hardware?.is_external_wifi ? 'USB Wi-Fi' : 'WLAN'}
            </span>
          </div>

          <span className="font-mono text-[#9CA3AF] text-[10px]">
            {hardware?.wifi_adapter_name || 'Generic Network'}
          </span>
        </div>
      </div>
    </div>
  );
};

