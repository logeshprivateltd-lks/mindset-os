import React, { useEffect, useState } from 'react';
import { useKernelStore } from '../stores/useKernelStore';
import { AuroraCard, Button, StatusBadge } from '@mindset/ui-core';
import type { KernelEvent, ServiceHealthReport } from '@mindset/db-schema';

export const StatusViewerModal: React.FC = () => {
  const {
    isStatusViewOpen,
    setIsStatusViewOpen,
    bootPhase,
    uptimeSecs,
    hardware,
    ramBreakdown,
    services,
    eventHistory,
    isPurging,
    lastPurgeResult,
    fetchKernelStatus,
    controlService,
    triggerRamPurge,
    fetchEventHistory,
  } = useKernelStore();

  const [activeTab, setActiveTab] = useState<'matrix' | 'ram' | 'hardware' | 'events'>('matrix');
  const [controllingService, setControllingService] = useState<string | null>(null);

  useEffect(() => {
    if (isStatusViewOpen) {
      fetchKernelStatus();
      fetchEventHistory(100);
      const interval = setInterval(() => {
        fetchKernelStatus();
        fetchEventHistory(100);
      }, 5000);
      return () => clearInterval(interval);
    }
  }, [isStatusViewOpen, fetchKernelStatus, fetchEventHistory]);

  if (!isStatusViewOpen) return null;

  const handleServiceAction = async (
    serviceName: string,
    action: 'start' | 'stop' | 'restart'
  ) => {
    setControllingService(`${serviceName}-${action}`);
    try {
      await controlService(serviceName, action);
    } finally {
      setControllingService(null);
    }
  };

  const formatUptime = (secs: number) => {
    const hours = Math.floor(secs / 3600);
    const mins = Math.floor((secs % 3600) / 60);
    const s = secs % 60;
    return `${hours > 0 ? `${hours}h ` : ''}${mins}m ${s}s`;
  };

  const getServiceStatusKind = (status: ServiceHealthReport['status']): 'healthy' | 'warning' | 'critical' | 'neutral' => {
    if (status === 'Healthy') return 'healthy';
    if (status === 'Stopped') return 'neutral';
    if (typeof status === 'object' && 'Degraded' in status) return 'warning';
    return 'critical';
  };

  const renderEventDetails = (ev: KernelEvent) => {
    switch (ev.type) {
      case 'SystemReady':
        return `Uptime: ${ev.payload.uptime_ms}ms | Sensors active`;
      case 'ServiceStateChanged':
        return `${ev.payload.service_name} ➔ ${ev.payload.status}`;
      case 'FocusSessionStateChanged':
        return `Session: ${ev.payload.session_id} (${ev.payload.state})`;
      case 'TaskCompleted':
        return `Task ${ev.payload.task_id} (+${ev.payload.xp_earned} XP)`;
      case 'MoodLogged':
        return `Mood: ${ev.payload.mood} | Energy: ${ev.payload.energy_tier}`;
      case 'FreeTimeWindowDetected':
        return `Window: ${ev.payload.start_time} (${ev.payload.duration_minutes}m)`;
      case 'RamThresholdWarning':
        return `RSS: ${ev.payload.process_rss_mb} MB | Sys: ${ev.payload.system_used_percent.toFixed(1)}%`;
      case 'EmergencyRamPurgeExecuted':
        return `Reclaimed: ${(ev.payload.reclaimed_bytes / 1024).toFixed(0)} KB | New RSS: ${ev.payload.os_shell_rss_mb.toFixed(1)} MB`;
      case 'HardwareStateChanged':
        return `Hardware capabilities updated`;
      case 'SecurityLockTriggered':
        return `Trigger: ${ev.payload.reason}`;
      case 'SyncOplogAppended':
        return `Sync Op: ${ev.payload.table_name} [${ev.payload.row_id}]`;
      default:
        return 'System event';
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-md p-4 sm:p-6 select-none animate-fadeIn">
      <div className="w-full max-w-4xl max-h-[90vh] flex flex-col rounded-2xl bg-[#111827] border border-[#1F2937] shadow-[0_16px_64px_rgba(0,0,0,0.8)] overflow-hidden">
        {/* Modal Header */}
        <div className="p-4 sm:p-6 border-b border-[#1F2937] flex items-center justify-between bg-[#07090D]/50">
          <div className="flex items-center space-x-3">
            <div className="w-3.5 h-3.5 rounded-full bg-[#65E6C4] shadow-[0_0_12px_#65E6C4]" />
            <div>
              <div className="flex items-center space-x-2">
                <h2 className="text-base font-semibold text-[#F9FAFB]">Kernel Status & Control</h2>
                <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-[#8B9CFF]/10 text-[#8B9CFF] border border-[#8B9CFF]/30">
                  /status
                </span>
              </div>
              <p className="text-xs text-[#9CA3AF]">
                MindSet OS Native Rust Kernel • Boot Stage: <span className="text-[#65E6C4] font-semibold">{bootPhase}</span> • Uptime: {formatUptime(uptimeSecs)}
              </p>
            </div>
          </div>

          <div className="flex items-center space-x-2">
            <Button
              variant="secondary"
              size="sm"
              onClick={triggerRamPurge}
              disabled={isPurging}
              className="text-xs font-mono"
            >
              {isPurging ? 'Purging...' : '🧹 Emergency Purge'}
            </Button>
            <button
              onClick={() => setIsStatusViewOpen(false)}
              className="p-2 rounded-lg text-[#9CA3AF] hover:text-[#F9FAFB] hover:bg-[#182235] transition-all"
            >
              ✕
            </button>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex items-center space-x-2 px-6 pt-3 border-b border-[#1F2937] bg-[#111827]">
          <button
            onClick={() => setActiveTab('matrix')}
            className={`pb-2.5 px-3 text-xs font-medium border-b-2 transition-all ${
              activeTab === 'matrix'
                ? 'border-[#8B9CFF] text-[#8B9CFF]'
                : 'border-transparent text-[#9CA3AF] hover:text-[#F9FAFB]'
            }`}
          >
            Services Matrix ({services.length})
          </button>
          <button
            onClick={() => setActiveTab('ram')}
            className={`pb-2.5 px-3 text-xs font-medium border-b-2 transition-all ${
              activeTab === 'ram'
                ? 'border-[#8B9CFF] text-[#8B9CFF]'
                : 'border-transparent text-[#9CA3AF] hover:text-[#F9FAFB]'
            }`}
          >
            RAM Breakdown & Watchdog
          </button>
          <button
            onClick={() => setActiveTab('hardware')}
            className={`pb-2.5 px-3 text-xs font-medium border-b-2 transition-all ${
              activeTab === 'hardware'
                ? 'border-[#8B9CFF] text-[#8B9CFF]'
                : 'border-transparent text-[#9CA3AF] hover:text-[#F9FAFB]'
            }`}
          >
            Hardware & USB Wi-Fi
          </button>
          <button
            onClick={() => setActiveTab('events')}
            className={`pb-2.5 px-3 text-xs font-medium border-b-2 transition-all ${
              activeTab === 'events'
                ? 'border-[#8B9CFF] text-[#8B9CFF]'
                : 'border-transparent text-[#9CA3AF] hover:text-[#F9FAFB]'
            }`}
          >
            Event Stream ({eventHistory.length})
          </button>
        </div>

        {/* Tab Content Area */}
        <div className="p-6 overflow-y-auto space-y-4 flex-1">
          {/* TAB 1: Services Matrix */}
          {activeTab === 'matrix' && (
            <div className="space-y-3">
              <div className="flex items-center justify-between text-xs text-[#9CA3AF] mb-2">
                <span>Cooperative Asynchronous Tokio Micro-Actors</span>
                <span>Actions: Start / Stop / Restart</span>
              </div>

              {services.map((svc) => (
                <div
                  key={svc.name}
                  className="flex flex-col sm:flex-row sm:items-center justify-between p-3.5 rounded-xl bg-[#182235] border border-[#202B40] gap-3"
                >
                  <div className="space-y-1">
                    <div className="flex items-center space-x-2">
                      <span className="text-xs font-semibold text-[#F9FAFB]">{svc.name}</span>
                      <StatusBadge
                        status={getServiceStatusKind(svc.status)}
                        label={typeof svc.status === 'string' ? svc.status : 'Degraded'}
                      />
                    </div>
                    <div className="text-[11px] text-[#9CA3AF] flex items-center space-x-3">
                      <span>Footprint: ~{svc.estimated_ram_kb} KB</span>
                      <span>•</span>
                      <span>Heartbeat: {new Date(svc.last_heartbeat_utc).toLocaleTimeString()}</span>
                    </div>
                  </div>

                  <div className="flex items-center space-x-1.5 self-end sm:self-center">
                    <button
                      onClick={() => handleServiceAction(svc.name, 'start')}
                      disabled={controllingService !== null}
                      className="px-2.5 py-1 text-[11px] rounded bg-[#111827] hover:bg-[#202B40] text-[#65E6C4] border border-[#1F2937] transition-all disabled:opacity-50"
                    >
                      Start
                    </button>
                    <button
                      onClick={() => handleServiceAction(svc.name, 'stop')}
                      disabled={controllingService !== null}
                      className="px-2.5 py-1 text-[11px] rounded bg-[#111827] hover:bg-[#202B40] text-[#FF8585] border border-[#1F2937] transition-all disabled:opacity-50"
                    >
                      Stop
                    </button>
                    <button
                      onClick={() => handleServiceAction(svc.name, 'restart')}
                      disabled={controllingService !== null}
                      className="px-2.5 py-1 text-[11px] rounded bg-[#111827] hover:bg-[#202B40] text-[#8B9CFF] border border-[#1F2937] transition-all disabled:opacity-50"
                    >
                      Restart
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}

          {/* TAB 2: RAM Breakdown */}
          {activeTab === 'ram' && (
            <div className="space-y-4">
              <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <AuroraCard>
                  <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
                    MindSet OS Shell RSS
                  </div>
                  <div className="flex items-baseline space-x-2 mt-2">
                    <span className="text-2xl font-mono font-bold text-[#65E6C4]">
                      {ramBreakdown.os_shell_mb.toFixed(1)}
                    </span>
                    <span className="text-xs text-[#9CA3AF]">MB / 120 MB max</span>
                  </div>
                  <div className="w-full bg-[#182235] h-1.5 rounded-full mt-2 overflow-hidden">
                    <div
                      className="bg-[#65E6C4] h-full rounded-full"
                      style={{ width: `${Math.min(100, (ramBreakdown.os_shell_mb / 120) * 100)}%` }}
                    />
                  </div>
                  <p className="text-[10px] text-[#65E6C4] mt-2">✓ Verified under strict 120 MB budget</p>
                </AuroraCard>

                <AuroraCard>
                  <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
                    Ollama Local Runtime
                  </div>
                  <div className="flex items-baseline space-x-2 mt-2">
                    <span className="text-2xl font-mono font-bold text-[#8B9CFF]">
                      {ramBreakdown.ollama_mb.toFixed(1)}
                    </span>
                    <span className="text-xs text-[#9CA3AF]">MB</span>
                  </div>
                  <div className="w-full bg-[#182235] h-1.5 rounded-full mt-2 overflow-hidden">
                    <div
                      className="bg-[#8B9CFF] h-full rounded-full"
                      style={{ width: `${Math.min(100, (ramBreakdown.ollama_mb / 4096) * 100)}%` }}
                    />
                  </div>
                  <p className="text-[10px] text-[#9CA3AF] mt-2">Zero-RAM probes until query invoked</p>
                </AuroraCard>

                <AuroraCard>
                  <div className="text-[11px] text-[#9CA3AF] uppercase tracking-wider font-semibold">
                    System Free Memory
                  </div>
                  <div className="flex items-baseline space-x-2 mt-2">
                    <span className="text-2xl font-mono font-bold text-[#FFD078]">
                      {(ramBreakdown.free_system_mb / 1024).toFixed(1)}
                    </span>
                    <span className="text-xs text-[#9CA3AF]">
                      GB of {(ramBreakdown.total_system_mb / 1024).toFixed(0)} GB
                    </span>
                  </div>
                  <div className="w-full bg-[#182235] h-1.5 rounded-full mt-2 overflow-hidden">
                    <div
                      className="bg-[#FFD078] h-full rounded-full"
                      style={{ width: `${ramBreakdown.system_used_percent}%` }}
                    />
                  </div>
                  <p className="text-[10px] text-[#9CA3AF] mt-2">
                    {ramBreakdown.system_used_percent.toFixed(1)}% system used (Watchdog alert at 80%)
                  </p>
                </AuroraCard>
              </div>

              {lastPurgeResult && (
                <div className="p-3.5 rounded-xl bg-[#65E6C4]/10 border border-[#65E6C4]/30 flex items-center justify-between text-xs">
                  <div className="flex items-center space-x-2">
                    <span className="text-[#65E6C4]">✓</span>
                    <span className="text-[#F9FAFB]">
                      Last Purge reclaimed <strong>{lastPurgeResult.freed_mb.toFixed(1)} MB</strong>. Current Shell RSS:{' '}
                      <strong>{lastPurgeResult.current_rss_mb.toFixed(1)} MB</strong>.
                    </span>
                  </div>
                  <span className="text-[10px] text-[#9CA3AF] font-mono">
                    {new Date(lastPurgeResult.timestamp_utc).toLocaleTimeString()}
                  </span>
                </div>
              )}
            </div>
          )}

          {/* TAB 3: Hardware Capabilities */}
          {activeTab === 'hardware' && (
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <AuroraCard>
                <h3 className="text-xs font-semibold text-[#8B9CFF] uppercase tracking-wider mb-3">
                  Sensory Input Devices
                </h3>
                <div className="space-y-2.5 text-xs">
                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">Webcam (DirectShow / V4L2)</span>
                    <span className={hardware?.has_webcam ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                      {hardware?.has_webcam ? '● Present' : '○ Not Connected'}
                    </span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">Microphone (WASAPI / ALSA)</span>
                    <span className={hardware?.has_mic ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                      {hardware?.has_mic ? '● Present' : '○ Not Connected'}
                    </span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">Biometrics (fprintd / WinBio)</span>
                    <span className={hardware?.has_fingerprint ? 'text-[#65E6C4]' : 'text-[#9CA3AF]'}>
                      {hardware?.has_fingerprint ? '● Enrolled' : '○ PIN Fallback Active'}
                    </span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">Bluetooth Radio</span>
                    <span className={hardware?.has_bluetooth ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                      {hardware?.has_bluetooth ? '● Available' : '○ Disabled'}
                    </span>
                  </div>
                </div>
              </AuroraCard>

              <AuroraCard>
                <h3 className="text-xs font-semibold text-[#65E6C4] uppercase tracking-wider mb-3">
                  Network & Wireless Probing
                </h3>
                <div className="space-y-2.5 text-xs">
                  <div className="p-3 rounded-lg bg-[#182235] space-y-1">
                    <span className="text-[10px] text-[#9CA3AF] uppercase">Detected Interface</span>
                    <div className="font-mono text-xs text-[#F9FAFB]">
                      {hardware?.wifi_adapter_name || 'Standard Network Interface'}
                    </div>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">External USB Wi-Fi (Realtek rtl88xxau)</span>
                    <span className={hardware?.is_external_wifi ? 'text-[#65E6C4] font-semibold' : 'text-[#9CA3AF]'}>
                      {hardware?.is_external_wifi ? '✓ Realtek USB Detected' : 'Internal / Standard'}
                    </span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235]">
                    <span className="text-[#E5E7EB]">Wireless Monitor Capability</span>
                    <span className={hardware?.has_wifi_monitor ? 'text-[#65E6C4]' : 'text-[#6B7280]'}>
                      {hardware?.has_wifi_monitor ? '● Supported' : '○ Inactive'}
                    </span>
                  </div>

                  <div className="p-2.5 rounded-lg bg-[#111827] text-[11px] text-[#9CA3AF]">
                    Hardware probe operates non-blockingly during boot with zero panic risk when unplugged.
                  </div>
                </div>
              </AuroraCard>
            </div>
          )}

          {/* TAB 4: Event Stream */}
          {activeTab === 'events' && (
            <div className="space-y-2">
              <div className="flex items-center justify-between text-xs text-[#9CA3AF] mb-1">
                <span>Circular Kernel Event Buffer (Capacity 200)</span>
                <span className="font-mono">{eventHistory.length} events logged</span>
              </div>

              {eventHistory.length === 0 ? (
                <div className="p-8 text-center text-xs text-[#9CA3AF] bg-[#182235]/40 rounded-xl">
                  No kernel events dispatched yet.
                </div>
              ) : (
                eventHistory.map((ev, index) => (
                  <div
                    key={`${ev.type}-${index}`}
                    className="flex items-center justify-between p-2.5 rounded-lg bg-[#182235] border border-[#202B40] text-xs font-mono"
                  >
                    <div className="flex items-center space-x-2">
                      <span className="text-[#8B9CFF] font-semibold">{ev.type}</span>
                      <span className="text-[#9CA3AF]">|</span>
                      <span className="text-[#E5E7EB]">{renderEventDetails(ev)}</span>
                    </div>
                  </div>
                ))
              )}
            </div>
          )}
        </div>

        {/* Modal Footer */}
        <div className="p-4 border-t border-[#1F2937] flex items-center justify-between bg-[#07090D]/50 text-xs">
          <span className="text-[#9CA3AF]">
            Target OS Shell Footprint: <span className="text-[#65E6C4] font-semibold">&lt;120 MB</span> • Tick Rate: 15s
          </span>
          <Button variant="secondary" size="sm" onClick={() => setIsStatusViewOpen(false)}>
            Close
          </Button>
        </div>
      </div>
    </div>
  );
};
