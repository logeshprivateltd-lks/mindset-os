import { create } from 'zustand';
import type {
  KernelBootPhase,
  HardwareCapabilities,
  ServiceHealthReport,
} from '@mindset/db-schema';

export interface KernelStoreState {
  bootPhase: KernelBootPhase;
  isReady: boolean;
  isLocked: boolean;
  rssMb: number;
  systemUsedPercent: number;
  totalMemoryMb: number;
  uptimeSecs: number;
  services: ServiceHealthReport[];
  hardware: HardwareCapabilities | null;
  priorityGoal: string;
  activeFocusMinutes: number;
  isFocusActive: boolean;

  setBootPhase: (phase: KernelBootPhase) => void;
  setTelemetry: (rss: number, sysPercent: number, total: number, uptime: number) => void;
  setServices: (services: ServiceHealthReport[]) => void;
  setHardware: (hardware: HardwareCapabilities) => void;
  setPriorityGoal: (goal: string) => void;
  setIsLocked: (locked: boolean) => void;
  toggleFocusSession: () => void;
}

export const useKernelStore = create<KernelStoreState>((set) => ({
  bootPhase: 'BOOT_INIT',
  isReady: false,
  isLocked: true,
  rssMb: 36.8,
  systemUsedPercent: 42.1,
  totalMemoryMb: 16384,
  uptimeSecs: 0,
  services: [
    {
      name: 'SchedulerService',
      status: 'Healthy',
      estimated_ram_kb: 512,
      last_heartbeat_utc: new Date().toISOString(),
    },
    {
      name: 'MemoryWatchdogService',
      status: 'Healthy',
      estimated_ram_kb: 768,
      last_heartbeat_utc: new Date().toISOString(),
    },
    {
      name: 'AuthService',
      status: 'Healthy',
      estimated_ram_kb: 420,
      last_heartbeat_utc: new Date().toISOString(),
    },
    {
      name: 'StorageService',
      status: 'Healthy',
      estimated_ram_kb: 610,
      last_heartbeat_utc: new Date().toISOString(),
    },
    {
      name: 'OllamaHealthService',
      status: 'Healthy',
      estimated_ram_kb: 380,
      last_heartbeat_utc: new Date().toISOString(),
    },
  ],
  hardware: {
    has_mic: true,
    has_webcam: true,
    has_fingerprint: false,
    has_bluetooth: true,
    has_wifi_monitor: false,
    detected_at_utc: new Date().toISOString(),
  },
  priorityGoal: 'Complete MindSet OS Workspace Setup & Focus Calibration',
  activeFocusMinutes: 45,
  isFocusActive: false,

  setBootPhase: (phase) =>
    set({
      bootPhase: phase,
      isReady: phase === 'SYSTEM_READY',
    }),

  setTelemetry: (rssMb, systemUsedPercent, totalMemoryMb, uptimeSecs) =>
    set({ rssMb, systemUsedPercent, totalMemoryMb, uptimeSecs }),

  setServices: (services) => set({ services }),

  setHardware: (hardware) => set({ hardware }),

  setPriorityGoal: (priorityGoal) => set({ priorityGoal }),

  setIsLocked: (isLocked) => set({ isLocked }),

  toggleFocusSession: () =>
    set((state) => ({ isFocusActive: !state.isFocusActive })),
}));

