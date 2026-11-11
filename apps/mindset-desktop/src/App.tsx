import React, { useEffect, useState } from 'react';
import { useKernelStore } from './stores/kernelStore';
import { TitleBar } from '@mindset/ui-core';
import { BootSequencer } from './components/BootSequencer';
import { LockScreen } from './components/LockScreen';
import { Dashboard } from './components/Dashboard';

export const App: React.FC = () => {
  const {
    bootPhase,
    isLocked,
    rssMb,
    setBootPhase,
    setIsLocked,
    setTelemetry,
  } = useKernelStore();

  const [hasBooted, setHasBooted] = useState(false);

  // Tauri IPC invocations with fallback
  useEffect(() => {
    // Dynamic import to support both Tauri native and standard web development
    const fetchTelemetry = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const telemetry = await invoke<{
          process_rss_mb: number;
          system_used_percent: number;
          total_memory_mb: number;
          uptime_secs: number;
        }>('get_system_telemetry');

        setTelemetry(
          telemetry.process_rss_mb,
          telemetry.system_used_percent,
          telemetry.total_memory_mb,
          telemetry.uptime_secs
        );
      } catch {
        // Running in pure web mode or before Tauri connects
      }
    };

    const interval = setInterval(fetchTelemetry, 3000);
    return () => clearInterval(interval);
  }, [setTelemetry]);

  // Global hotkey: Ctrl+L / Meta+L to lock screen
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'l') {
        e.preventDefault();
        setIsLocked(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [setIsLocked]);

  const handleMinimize = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('window_minimize');
    } catch {
      console.log('Window minimize triggered');
    }
  };

  const handleMaximize = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('window_maximize');
    } catch {
      console.log('Window maximize triggered');
    }
  };

  const handleClose = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('window_close');
    } catch {
      console.log('Window close triggered');
    }
  };

  const handleBootComplete = () => {
    setBootPhase('SYSTEM_READY');
    setHasBooted(true);
  };

  return (
    <div className="min-h-screen bg-[#07090D] text-[#F9FAFB] flex flex-col font-sans select-none overflow-x-hidden">
      {/* Frameless Window Custom Titlebar */}
      <TitleBar
        title="MindSet OS — Powered by LKS PVT LTD"
        subtitle="Aurora Core v0.1.0"
        memoryUsageMb={rssMb}
        kernelPhase={bootPhase}
        onMinimize={handleMinimize}
        onMaximize={handleMaximize}
        onClose={handleClose}
      />

      {/* Main OS Surface */}
      <main className="flex-1 relative overflow-y-auto">
        {!hasBooted && bootPhase !== 'SYSTEM_READY' ? (
          <BootSequencer onComplete={handleBootComplete} />
        ) : isLocked ? (
          <LockScreen onUnlock={() => setIsLocked(false)} />
        ) : (
          <Dashboard onLock={() => setIsLocked(true)} />
        )}
      </main>
    </div>
  );
};
