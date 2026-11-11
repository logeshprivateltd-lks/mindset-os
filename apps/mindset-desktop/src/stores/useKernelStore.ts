import { create } from 'zustand';
import type {
  KernelBootPhase,
  HardwareCapabilities,
  ServiceHealthReport,
  RamBreakdown,
  KernelStatusResponse,
  EmergencyPurgeResult,
  KernelEvent,
} from '@mindset/db-schema';

export interface KernelStoreState {
  bootPhase: KernelBootPhase;
  isReady: boolean;
  isLocked: boolean;
  isStatusViewOpen: boolean;
  uptimeSecs: number;

  // Direct Telemetry Aliases for UI components
  rssMb: number;
  systemUsedPercent: number;
  totalMemoryMb: number;

  // Rich Breakdown & Matrix
  hardware: HardwareCapabilities | null;
  ramBreakdown: RamBreakdown;
  services: ServiceHealthReport[];
  eventHistory: KernelEvent[];
  lastPurgeResult: EmergencyPurgeResult | null;
  isPurging: boolean;
  priorityGoal: string;
  activeFocusMinutes: number;
  isFocusActive: boolean;

  // Actions
  fetchKernelStatus: () => Promise<void>;
  controlService: (serviceName: string, action: 'start' | 'stop' | 'restart') => Promise<boolean>;
  triggerRamPurge: () => Promise<EmergencyPurgeResult | null>;
  fetchEventHistory: (limit?: number) => Promise<void>;
  initEventListeners: () => Promise<() => void>;
  setBootPhase: (phase: KernelBootPhase) => void;
  setTelemetry: (rssMb: number, systemUsedPercent: number, totalMemoryMb: number, uptimeSecs: number) => void;
  setServices: (services: ServiceHealthReport[]) => void;
  setHardware: (hardware: HardwareCapabilities) => void;
  setIsLocked: (locked: boolean) => void;
  setIsStatusViewOpen: (open: boolean) => void;
  setPriorityGoal: (goal: string) => void;
  toggleFocusSession: () => void;
}

export const useKernelStore = create<KernelStoreState>((set, get) => ({
  bootPhase: 'BOOT_INIT',
  isReady: false,
  isLocked: true,
  isStatusViewOpen: false,
  uptimeSecs: 0,
  rssMb: 36.5,
  systemUsedPercent: 37.5,
  totalMemoryMb: 16384,

  hardware: {
    has_mic: true,
    has_webcam: true,
    has_fingerprint: false,
    has_bluetooth: true,
    has_wifi_monitor: true,
    wifi_adapter_name: 'Wi-Fi Adapter (Realtek/USB Auto-Probed)',
    is_external_wifi: false,
    detected_at_utc: new Date().toISOString(),
  },
  ramBreakdown: {
    os_shell_mb: 36.5,
    ollama_mb: 0.0,
    free_system_mb: 10240,
    total_system_mb: 16384,
    system_used_percent: 37.5,
  },
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
      name: 'HardwareProbeService',
      status: 'Healthy',
      estimated_ram_kb: 320,
      last_heartbeat_utc: new Date().toISOString(),
    },
    {
      name: 'OllamaHealthService',
      status: 'Healthy',
      estimated_ram_kb: 380,
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
  ],
  eventHistory: [],
  lastPurgeResult: null,
  isPurging: false,
  priorityGoal: 'Complete MindSet OS Kernel Calibrations & Low-RAM Audit',
  activeFocusMinutes: 45,
  isFocusActive: false,

  fetchKernelStatus: async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const status = await invoke<KernelStatusResponse>('get_kernel_status');
      set({
        bootPhase: (status.boot_stage as KernelBootPhase) || 'SYSTEM_READY',
        isReady: status.boot_stage === 'SYSTEM_READY',
        uptimeSecs: status.uptime_secs,
        hardware: status.hardware,
        ramBreakdown: status.ram_breakdown,
        rssMb: status.ram_breakdown.os_shell_mb,
        systemUsedPercent: status.ram_breakdown.system_used_percent,
        totalMemoryMb: status.ram_breakdown.total_system_mb,
        services: status.services,
      });
    } catch {
      // In web preview or before Tauri IPC connects
    }
  },

  controlService: async (serviceName: string, action: 'start' | 'stop' | 'restart') => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const updatedHealth = await invoke<ServiceHealthReport>('control_kernel_service', {
        serviceName,
        action,
      });

      set((state) => ({
        services: state.services.map((s) => (s.name === serviceName ? updatedHealth : s)),
      }));

      // Re-fetch kernel status to reflect updated matrix
      get().fetchKernelStatus();
      return true;
    } catch (err) {
      console.error(`Failed to ${action} service ${serviceName}:`, err);
      return false;
    }
  },

  triggerRamPurge: async () => {
    set({ isPurging: true });
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke<EmergencyPurgeResult>('trigger_emergency_ram_purge');
      set((state) => ({
        lastPurgeResult: result,
        isPurging: false,
        rssMb: result.current_rss_mb,
        ramBreakdown: {
          ...state.ramBreakdown,
          os_shell_mb: result.current_rss_mb,
        },
      }));
      get().fetchKernelStatus();
      get().fetchEventHistory();
      return result;
    } catch (err) {
      console.error('Failed to trigger emergency RAM purge:', err);
      set({ isPurging: false });
      return null;
    }
  },

  fetchEventHistory: async (limit = 50) => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const history = await invoke<KernelEvent[]>('get_kernel_event_history', { limit });
      set({ eventHistory: history });
    } catch {
      // Pure web preview
    }
  },

  initEventListeners: async () => {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      const unlisten = await listen<KernelEvent>('mindset://kernel-event', (event) => {
        const payload = event.payload;

        // Push into local event history stream
        set((state) => ({
          eventHistory: [payload, ...state.eventHistory.slice(0, 199)],
        }));

        if (payload.type === 'SystemReady') {
          set({
            bootPhase: 'SYSTEM_READY',
            isReady: true,
            hardware: payload.payload.hardware,
          });
        } else if (payload.type === 'HardwareStateChanged') {
          set({ hardware: payload.payload.capabilities });
        } else if (payload.type === 'EmergencyRamPurgeExecuted') {
          set((state) => ({
            rssMb: payload.payload.os_shell_rss_mb,
            ramBreakdown: {
              ...state.ramBreakdown,
              os_shell_mb: payload.payload.os_shell_rss_mb,
            },
          }));
        } else if (payload.type === 'ServiceStateChanged') {
          get().fetchKernelStatus();
        }
      });

      return unlisten;
    } catch {
      return () => {};
    }
  },

  setBootPhase: (phase) =>
    set({
      bootPhase: phase,
      isReady: phase === 'SYSTEM_READY',
    }),

  setTelemetry: (rssMb, systemUsedPercent, totalMemoryMb, uptimeSecs) =>
    set((state) => ({
      rssMb,
      systemUsedPercent,
      totalMemoryMb,
      uptimeSecs,
      ramBreakdown: {
        ...state.ramBreakdown,
        os_shell_mb: rssMb,
        system_used_percent: systemUsedPercent,
        total_system_mb: totalMemoryMb,
      },
    })),

  setServices: (services) => set({ services }),

  setHardware: (hardware) => set({ hardware }),

  setIsLocked: (isLocked) => set({ isLocked }),

  setIsStatusViewOpen: (isStatusViewOpen) => set({ isStatusViewOpen }),

  setPriorityGoal: (priorityGoal) => set({ priorityGoal }),

  toggleFocusSession: () =>
    set((state) => ({
      isFocusActive: !state.isFocusActive,
    })),
}));
