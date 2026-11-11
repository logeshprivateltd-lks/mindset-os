import React from 'react';
import { useKernelStore } from '../stores/kernelStore';

export const BootSequencer: React.FC<{ onComplete: () => void }> = ({ onComplete }) => {
  const { bootPhase, rssMb } = useKernelStore();

  const phases = [
    { id: 'BOOT_INIT', label: '1. Vault & Storage Initializer', desc: 'Validating ~/.mindset_data/ hierarchy' },
    { id: 'DB_MIGRATE', label: '2. SQLite Low-RAM Migrations', desc: 'WAL mode, 32MB mmap, 8MB cache applied' },
    { id: 'HARDWARE_PROBE', label: '3. Sensory Hardware Probe', desc: 'Webcam, Mic, Bluetooth, Biometrics' },
    { id: 'SERVICE_REGISTER', label: '4. Service Registry Spawning', desc: 'Scheduler, MemoryWatchdog, Auth, Storage' },
    { id: 'SYSTEM_READY', label: '5. System Kernel Ready', desc: 'Event bus active, frontend unlocked' },
  ];

  const currentIdx = phases.findIndex((p) => p.id === bootPhase);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen px-6 bg-[#07090D] select-none">
      <div className="w-full max-w-md p-6 rounded-xl bg-[#111827] border border-[#1F2937] shadow-[0_8px_32px_rgba(0,0,0,0.5)]">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-4 h-4 rounded-full bg-[#8B9CFF] shadow-[0_0_12px_#8B9CFF] animate-pulse" />
          <div>
            <h1 className="text-base font-semibold text-[#F9FAFB] tracking-wide">MindSet OS Kernel Boot</h1>
            <p className="text-xs text-[#9CA3AF]">Powered by LKS PVT LTD — Target RAM: &lt;120 MB</p>
          </div>
        </div>

        <div className="space-y-3 mb-6">
          {phases.map((p, idx) => {
            const isDone = currentIdx >= idx;
            const isCurrent = currentIdx === idx;

            return (
              <div
                key={p.id}
                className={`p-3 rounded-lg border transition-all duration-300 ${
                  isCurrent
                    ? 'bg-[#182235] border-[#8B9CFF]/50 shadow-[0_0_12px_rgba(139,156,255,0.15)]'
                    : isDone
                    ? 'bg-[#111827] border-[#1F2937] opacity-80'
                    : 'bg-[#07090D] border-[#1F2937] opacity-40'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className={`text-xs font-medium ${isCurrent ? 'text-[#8B9CFF]' : isDone ? 'text-[#65E6C4]' : 'text-[#9CA3AF]'}`}>
                    {p.label}
                  </span>
                  {isDone ? (
                    <span className="text-xs text-[#65E6C4]">✓ Ready</span>
                  ) : isCurrent ? (
                    <span className="text-xs text-[#8B9CFF] animate-pulse">Running...</span>
                  ) : (
                    <span className="text-xs text-[#6B7280]">Pending</span>
                  )}
                </div>
                <p className="text-[11px] text-[#9CA3AF] mt-1">{p.desc}</p>
              </div>
            );
          })}
        </div>

        <div className="flex items-center justify-between pt-4 border-t border-[#1F2937] text-xs">
          <div className="flex items-center space-x-2">
            <span className="text-[#9CA3AF]">Cold Memory:</span>
            <span className="font-mono text-[#65E6C4] font-semibold">{rssMb.toFixed(1)} MB</span>
          </div>

          <button
            onClick={onComplete}
            className="px-4 py-1.5 rounded bg-[#8B9CFF] hover:bg-[#A3B1FF] text-[#07090D] text-xs font-semibold transition-all shadow-[0_0_12px_rgba(139,156,255,0.3)]"
          >
            Launch OS Shell →
          </button>
        </div>
      </div>
    </div>
  );
};
