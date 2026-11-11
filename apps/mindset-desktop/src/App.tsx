/**
 * MindSet OS Desktop Application Shell
 * Powered by LKS PVT LTD
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useEffect } from 'react';
import { useKernelStore } from './stores/useKernelStore';
import { TitleBar } from '@mindset/ui-core';
import { BootSplashGate } from './components/BootSplashGate';
import { SmartLockOverlay } from './pages/auth/SmartLockOverlay';
import { DesktopShell } from './shell/DesktopShell';
import { StatusViewerModal } from './components/StatusViewerModal';

export const App: React.FC = () => {
  const {
    bootPhase,
    isLocked,
    isPrivacyBlurred,
    rssMb,
    isStatusViewOpen,
    setIsLocked,
    lockSystem,
    togglePrivacyBlur,
    syncAuthStatus,
    setIsStatusViewOpen,
    fetchKernelStatus,
    initEventListeners,
  } = useKernelStore();

  // Periodic Kernel Status Refresh & initial auth check
  useEffect(() => {
    fetchKernelStatus();
    syncAuthStatus();
    const interval = setInterval(fetchKernelStatus, 3000);
    return () => clearInterval(interval);
  }, [fetchKernelStatus, syncAuthStatus]);

  // Subscribe to Kernel broadcast events
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    initEventListeners().then((cleanup) => {
      unlisten = cleanup;
    });
    return () => {
      if (unlisten) unlisten();
    };
  }, [initEventListeners]);

  // Global hotkeys:
  // Super + L / Ctrl + L / Ctrl + Alt + L to lock
  // Alt + B to toggle Privacy Blur Guard
  // Ctrl + S / Meta + S to open /status modal
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey || (e.altKey && e.ctrlKey)) && e.key.toLowerCase() === 'l') {
        e.preventDefault();
        lockSystem();
      } else if (e.altKey && e.key.toLowerCase() === 'b') {
        e.preventDefault();
        togglePrivacyBlur();
      } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
        e.preventDefault();
        setIsStatusViewOpen(!isStatusViewOpen);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isStatusViewOpen, lockSystem, togglePrivacyBlur, setIsStatusViewOpen]);

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

  return (
    <BootSplashGate>
      <div
        className="min-h-screen flex flex-col font-sans select-none overflow-x-hidden transition-colors"
        style={{ backgroundColor: 'var(--bg-base)', color: 'var(--text-dominant)' }}
      >
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
        <main className="flex-1 relative overflow-hidden">
          {isLocked ? (
            <SmartLockOverlay onUnlock={() => setIsLocked(false)} />
          ) : (
            <>
              <DesktopShell
                onLock={lockSystem}
                onOpenStatus={() => setIsStatusViewOpen(true)}
              />
              {/* Instant Privacy Blur Overlay (Alt + B) */}
              {isPrivacyBlurred && (
                <SmartLockOverlay
                  isPrivacyBlurredOnly
                  onUnlock={() => togglePrivacyBlur()}
                />
              )}
            </>
          )}
        </main>

        {/* /status Interactive Inspection & Control Modal */}
        <StatusViewerModal />
      </div>
    </BootSplashGate>
  );
};
