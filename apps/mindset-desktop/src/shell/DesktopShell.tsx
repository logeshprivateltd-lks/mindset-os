/**
 * MindSet OS Master Desktop Shell Component
 * Coordinates the Desktop Canvas, Multi-Window Manager, KDE Plasma Activities,
 * Snap Layouts, Start Menu, and Persistent Taskbar Dock.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import { useWindowManagerStore } from './wm/useWindowManagerStore';
import { OsWindowFrame } from './wm/OsWindowFrame';
import { SnapLayoutOverlay } from './wm/SnapLayoutOverlay';
import { DesktopCanvas } from './DesktopCanvas';
import { TaskbarDock } from './TaskbarDock';
import { StartMenuOverlay } from './StartMenuOverlay';
import { ActivitySwitcherModal } from './activities/ActivitySwitcherModal';
import { useActivityStore } from './activities/useActivityStore';
import { APP_REGISTRY } from './apps/AppRegistry';

export interface DesktopShellProps {
  onLock: () => void;
  onOpenStatus: () => void;
}

export const DesktopShell: React.FC<DesktopShellProps> = ({ onLock, onOpenStatus }) => {
  const { windows, openWindow } = useWindowManagerStore();
  const { currentActivityId, toggleSwitcher } = useActivityStore();

  const [isStartMenuOpen, setIsStartMenuOpen] = useState(false);

  // Auto-launch Command Center on first boot
  useEffect(() => {
    if (windows.length === 0) {
      openWindow('dashboard', {
        title: 'Command Center & Telemetry',
        icon: '⚡',
        subjectTag: 'Core System',
      });
    }
  }, [openWindow, windows.length]);

  // Global hotkeys for Desktop Shell
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Super + Tab -> Activity Switcher
      if ((e.metaKey || e.altKey) && e.key === 'Tab') {
        e.preventDefault();
        toggleSwitcher();
      }
      // Escape closes Start Menu
      else if (e.key === 'Escape' && isStartMenuOpen) {
        e.preventDefault();
        setIsStartMenuOpen(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isStartMenuOpen, toggleSwitcher]);

  // Filter windows belonging to current activity workspace (or pinned across all)
  const visibleWindows = windows.filter(
    (w) => w.activityId === currentActivityId || w.pinnedToAllActivities
  );

  return (
    <div className="relative w-full h-full overflow-hidden select-none">
      {/* 1. Desktop Canvas (Wallpaper, desktop icons, context menu) */}
      <DesktopCanvas
        onOpenStatus={onOpenStatus}
        onLock={onLock}
      />

      {/* 2. Multi-Window Manager Workspace */}
      <div className="absolute inset-0 pointer-events-none">
        {visibleWindows.map((win) => {
          const appMeta = APP_REGISTRY[win.appId];
          const AppContent = appMeta?.component;

          return (
            <div key={win.id} className="pointer-events-auto">
              <OsWindowFrame window={win}>
                {AppContent ? <AppContent /> : <div className="p-4 text-xs">Application content</div>}
              </OsWindowFrame>
            </div>
          );
        })}
      </div>

      {/* 3. Snap Layout Translucent Liquid Glass Preview Overlay */}
      <SnapLayoutOverlay />

      {/* 4. KDE Plasma Activity Switcher Modal (Super + Tab) */}
      <ActivitySwitcherModal />

      {/* 5. Start Menu Launcher Overlay (Super) */}
      <StartMenuOverlay
        isOpen={isStartMenuOpen}
        onClose={() => setIsStartMenuOpen(false)}
        onLock={onLock}
        onOpenStatus={onOpenStatus}
      />

      {/* 6. Persistent Bottom Liquid Glass Taskbar Dock */}
      <TaskbarDock
        isStartMenuOpen={isStartMenuOpen}
        onToggleStartMenu={() => setIsStartMenuOpen(!isStartMenuOpen)}
        onOpenStatus={onOpenStatus}
      />
    </div>
  );
};
