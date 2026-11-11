/**
 * MindSet OS Persistent Taskbar Dock Component
 * Bottom Liquid Glass dock with Start Menu, active app running indicators, and interactive system tray
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { useWindowManagerStore } from './wm/useWindowManagerStore';
import { useActivityStore, ACTIVITIES } from './activities/useActivityStore';
import { APP_REGISTRY } from './apps/AppRegistry';
import { TabularTimer } from '@mindset/ui-core';
import { useKernelStore } from '../stores/kernelStore';

export interface TaskbarDockProps {
  onToggleStartMenu: () => void;
  isStartMenuOpen: boolean;
  onOpenStatus: () => void;
}

export const TaskbarDock: React.FC<TaskbarDockProps> = ({
  onToggleStartMenu,
  isStartMenuOpen,
  onOpenStatus,
}) => {
  const { windows, activeWindowId, openWindow, minimizeWindow, bringToFront } = useWindowManagerStore();
  const { currentActivityId, toggleSwitcher } = useActivityStore();
  const { isFocusActive, hardware } = useKernelStore();

  const [isAudioFlyoutOpen, setIsAudioFlyoutOpen] = useState(false);
  const [audioVolume, setAudioVolume] = useState(80);
  const [isWifiFlyoutOpen, setIsWifiFlyoutOpen] = useState(false);

  const activeActivity = ACTIVITIES[currentActivityId] || ACTIVITIES['deep-study'];
  const pinnedAppIds = activeActivity.pinnedApps;

  const handleAppIconClick = (appId: string) => {
    const existing = windows.find((w) => w.appId === appId);
    if (!existing) {
      openWindow(appId, {}, currentActivityId);
    } else if (existing.isMinimized) {
      bringToFront(existing.id);
    } else if (activeWindowId === existing.id) {
      minimizeWindow(existing.id);
    } else {
      bringToFront(existing.id);
    }
  };

  return (
    <footer
      className="fixed bottom-0 left-0 right-0 h-12 px-3 border-t flex items-center justify-between select-none z-40 backdrop-blur-xl transition-colors"
      style={{
        backgroundColor: 'var(--glass-bg)',
        borderColor: 'var(--border-subtle)',
        boxShadow: '0 -4px 24px rgba(0, 0, 0, 0.4)',
      }}
    >
      {/* --------------------------------------------------------- */}
      {/* LEFT SECTION: START BUTTON & ACTIVITY WORKSPACE PILL */}
      {/* --------------------------------------------------------- */}
      <div className="flex items-center space-x-2">
        {/* MindSet Start Button (Super) */}
        <button
          type="button"
          onClick={onToggleStartMenu}
          title="MindSet Start Menu (Super)"
          className={`h-9 px-3 rounded-xl flex items-center space-x-2 border transition-all ${
            isStartMenuOpen
              ? 'border-[var(--accent-primary)] bg-[var(--surface-elevated)] shadow-[0_0_12px_var(--border-glow)]'
              : 'border-transparent hover:border-[var(--border-subtle)] hover:bg-[var(--surface-secondary)]'
          }`}
        >
          <span className="w-2.5 h-2.5 rounded-full bg-[var(--accent-primary)] shadow-[0_0_8px_var(--accent-primary)] animate-pulse" />
          <span className="font-bold text-xs tracking-wider text-[var(--text-dominant)]">
            MINDSET
          </span>
        </button>

        {/* Activity Workspace Switcher Pill */}
        <button
          type="button"
          onClick={() => toggleSwitcher(true)}
          title={`Active Workspace: ${activeActivity.name} (Super + Tab)`}
          className="h-8 px-2.5 rounded-lg border border-[var(--border-subtle)] hover:border-[var(--accent-primary)] bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] flex items-center space-x-1.5 text-xs text-[var(--text-dominant)] transition-colors"
        >
          <span>{activeActivity.icon}</span>
          <span className="font-semibold text-[11px] hidden sm:inline">{activeActivity.name}</span>
          <span className="text-[10px] text-[var(--text-muted)] font-mono">⇥</span>
        </button>
      </div>

      {/* --------------------------------------------------------- */}
      {/* CENTER SECTION: RUNNING & PINNED APP DOCK */}
      {/* --------------------------------------------------------- */}
      <div className="flex items-center space-x-1">
        {pinnedAppIds.map((appId) => {
          const appMeta = APP_REGISTRY[appId];
          if (!appMeta) return null;

          const win = windows.find((w) => w.appId === appId);
          const isOpen = Boolean(win);
          const isFocused = isOpen && activeWindowId === win?.id && !win?.isMinimized;

          return (
            <button
              key={appId}
              type="button"
              onClick={() => handleAppIconClick(appId)}
              title={`${appMeta.title} (${isOpen ? (isFocused ? 'Active' : 'Running') : 'Launch'})`}
              className={`relative w-10 h-10 rounded-xl flex items-center justify-center text-lg border transition-all duration-150 ${
                isFocused
                  ? 'border-[var(--accent-primary)] bg-[var(--surface-elevated)] shadow-[0_0_12px_var(--border-glow)] scale-105'
                  : isOpen
                  ? 'border-[var(--border-subtle)] bg-[var(--surface-primary)]'
                  : 'border-transparent hover:bg-[var(--surface-secondary)] opacity-80 hover:opacity-100'
              }`}
            >
              <span>{appMeta.icon}</span>

              {/* Active Indicator Dot below icon */}
              {isOpen && (
                <span
                  className="absolute bottom-1 w-1.5 h-1.5 rounded-full"
                  style={{
                    backgroundColor: isFocused
                      ? 'var(--accent-primary)'
                      : 'var(--text-muted)',
                  }}
                />
              )}
            </button>
          );
        })}
      </div>

      {/* --------------------------------------------------------- */}
      {/* RIGHT SECTION: INTERACTIVE SYSTEM TRAY */}
      {/* --------------------------------------------------------- */}
      <div className="flex items-center space-x-2 relative text-xs">
        {/* Active Focus Session Countdown Pill */}
        {isFocusActive && (
          <div className="flex items-center space-x-1.5 px-2.5 py-1 rounded-full border border-[var(--accent-secondary)] bg-[var(--surface-primary)] text-[var(--accent-secondary)] font-mono text-[11px] shadow-[0_0_10px_var(--border-glow)]">
            <span className="w-1.5 h-1.5 rounded-full bg-[var(--accent-secondary)] animate-ping" />
            <span>Deep Work Active</span>
          </div>
        )}

        {/* Audio Volume Slider Popup Button */}
        <div className="relative">
          <button
            type="button"
            onClick={() => setIsAudioFlyoutOpen(!isAudioFlyoutOpen)}
            title={`Audio Output: ${audioVolume}%`}
            className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            🔊
          </button>

          {isAudioFlyoutOpen && (
            <div
              onMouseLeave={() => setIsAudioFlyoutOpen(false)}
              className="absolute bottom-10 right-0 p-3 rounded-xl border shadow-2xl backdrop-blur-xl animate-fadeIn w-48 space-y-2"
              style={{
                backgroundColor: 'var(--glass-bg)',
                borderColor: 'var(--accent-primary)',
              }}
            >
              <div className="flex justify-between text-[11px]">
                <span className="text-[var(--text-muted)]">Master Volume</span>
                <span className="font-mono font-bold text-[var(--text-dominant)]">{audioVolume}%</span>
              </div>
              <input
                type="range"
                min="0"
                max="100"
                value={audioVolume}
                onChange={(e) => setAudioVolume(Number(e.target.value))}
                className="w-full accent-[var(--accent-primary)] cursor-pointer"
              />
            </div>
          )}
        </div>

        {/* Wi-Fi / Hardware Status Indicator */}
        <div className="relative">
          <button
            type="button"
            onClick={() => setIsWifiFlyoutOpen(!isWifiFlyoutOpen)}
            title="Network & Wireless Status"
            className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            📶
          </button>

          {isWifiFlyoutOpen && (
            <div
              onMouseLeave={() => setIsWifiFlyoutOpen(false)}
              className="absolute bottom-10 right-0 p-3 rounded-xl border shadow-2xl backdrop-blur-xl animate-fadeIn w-56 space-y-1.5 text-left"
              style={{
                backgroundColor: 'var(--glass-bg)',
                borderColor: 'var(--accent-primary)',
              }}
            >
              <div className="text-[10px] font-bold uppercase tracking-wider text-[var(--accent-primary)]">
                Network & Sensor Status
              </div>
              <div className="text-xs font-semibold text-[var(--text-dominant)]">
                {hardware?.wifi_adapter_name || 'Wi-Fi Adapter (Online)'}
              </div>
              <div className="text-[11px] text-[var(--text-muted)]">
                Signal: 100% • Local Peer Sync Active
              </div>
            </div>
          )}
        </div>

        {/* Live Tabular Clock Readout */}
        <button
          type="button"
          onClick={onOpenStatus}
          title="Open Kernel Telemetry (/status)"
          className="px-2.5 py-1 rounded-lg border border-[var(--border-subtle)] hover:border-[var(--accent-primary)] bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] flex items-center space-x-1.5 transition-colors"
        >
          <TabularTimer mode="clock" size="sm" />
        </button>
      </div>
    </footer>
  );
};
