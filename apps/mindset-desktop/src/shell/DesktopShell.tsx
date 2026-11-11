/**
 * MindSet OS Master Desktop Shell Component
 * Coordinates the Desktop Canvas, Multi-Window Manager, KDE Plasma Activities,
 * Snap Layouts, Start Menu, Living Profile Evolution, and Persistent Taskbar Dock.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useMemo } from 'react';
import type { UserProfile } from '@mindset/db-schema';
import { LivingProfileEngine, ProfileEvolutionSuggestion } from '@mindset/identity-engine';
import { useWindowManagerStore } from './wm/useWindowManagerStore';
import { OsWindowFrame } from './wm/OsWindowFrame';
import { SnapLayoutOverlay } from './wm/SnapLayoutOverlay';
import { DesktopCanvas } from './DesktopCanvas';
import { TaskbarDock } from './TaskbarDock';
import { StartMenuOverlay } from './StartMenuOverlay';
import { ActivitySwitcherModal } from './activities/ActivitySwitcherModal';
import { useActivityStore } from './activities/useActivityStore';
import { APP_REGISTRY } from './apps/AppRegistry';
import { MemoryInspectorModal } from '../components/MemoryInspectorModal';
import { LivingProfileSuggestionCard } from '../components/LivingProfileSuggestionCard';
import { LanguageSettingsModal } from '../components/settings/LanguageSettingsModal';
import { FloatingAiOrb } from '../components/floating-ai';
import { GlobalCommandPalette } from '../components/command-palette';
import { FocusInterruptionOverlay } from '../components/focus-guard';
import { ScientificCalculatorModal, AccessibilityController } from '../components/utilities';
import { useKernelStore } from '../stores/useKernelStore';
import { navigationRouter } from '../navigation/navigationRouter';
import { dbClient } from '../lib/dbClient';

export interface DesktopShellProps {
  onLock: () => void;
  onOpenStatus: () => void;
  onOpenOnboarding?: () => void;
}

export const DesktopShell: React.FC<DesktopShellProps> = ({
  onLock,
  onOpenStatus,
  onOpenOnboarding,
}) => {
  const { windows, openWindow, bringToFront } = useWindowManagerStore();
  const { currentActivityId, toggleSwitcher } = useActivityStore();
  const {
    isFocusInterruptionOpen,
    interruptedAppName,
    priorityGoal,
    dismissFocusInterruption,
  } = useKernelStore();

  const [isStartMenuOpen, setIsStartMenuOpen] = useState(false);
  const [isMemoryInspectorOpen, setIsMemoryInspectorOpen] = useState(false);
  const [isLanguageModalOpen, setIsLanguageModalOpen] = useState(false);
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false);
  const [isCalculatorOpen, setIsCalculatorOpen] = useState(false);
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null);
  const [activeSuggestion, setActiveSuggestion] = useState<ProfileEvolutionSuggestion | null>(null);

  const livingEngine = useMemo(() => new LivingProfileEngine(), []);

  // Fetch initial profile & check for completed tasks to trigger living profile evolution
  useEffect(() => {
    dbClient.profile.get().then(async (p) => {
      if (p) {
        setUserProfile(p);
        // Check completed tasks to see if any milestone suggestion is needed
        try {
          const completedTasks = await dbClient.tasks.list('completed');
          if (completedTasks && completedTasks.length > 0) {
            const suggestion = livingEngine.detectMilestoneEvolution(p, completedTasks);
            if (suggestion) {
              setActiveSuggestion(suggestion);
            }
          }
        } catch {
          // Ignore
        }
      }
    });
  }, [livingEngine]);

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

  // Global hotkeys for Desktop Shell:
  // Super + Tab -> Activity Switcher
  // Ctrl + M / Super + M -> AI Memory Inspector
  // Ctrl + K / Super + K -> Global Command Palette
  // Escape -> close Start Menu / Command Palette
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.altKey) && e.key === 'Tab') {
        e.preventDefault();
        toggleSwitcher();
      } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'm') {
        e.preventDefault();
        setIsMemoryInspectorOpen((prev) => !prev);
      } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setIsCommandPaletteOpen((prev) => !prev);
      } else if (e.altKey && e.key.toLowerCase() === 'c') {
        e.preventDefault();
        setIsCalculatorOpen((prev) => !prev);
      } else if (e.key === 'Escape') {
        if (isStartMenuOpen) {
          e.preventDefault();
          setIsStartMenuOpen(false);
        }
        if (isCommandPaletteOpen) {
          e.preventDefault();
          setIsCommandPaletteOpen(false);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isStartMenuOpen, isCommandPaletteOpen, toggleSwitcher]);

  // Subscribe to canonical route navigation (/mind, /tasks, /files, /status, etc.)
  useEffect(() => {
    const unsubscribe = navigationRouter.subscribe(({ route }) => {
      switch (route) {
        case '/':
        case '/dashboard': {
          const win = windows.find((w) => w.appId === 'dashboard');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('dashboard', { title: 'Command Center', icon: '⚡' }, currentActivityId);
          }
          break;
        }
        case '/tasks': {
          const win = windows.find((w) => w.appId === 'tasks');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('tasks', { title: 'Tasks & Goals', icon: '🎯' }, currentActivityId);
          }
          break;
        }
        case '/habits': {
          const win = windows.find((w) => w.appId === 'habits');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('habits', { title: 'Habits & Conditioning', icon: '🔥' }, currentActivityId);
          }
          break;
        }
        case '/timeline': {
          const win = windows.find((w) => w.appId === 'timeline');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('timeline', { title: 'Activity Timeline', icon: '⏱️' }, currentActivityId);
          }
          break;
        }
        case '/journal': {
          const win = windows.find((w) => w.appId === 'journal');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('journal', { title: 'Daily Journal & Reflection', icon: '✍️' }, currentActivityId);
          }
          break;
        }
        case '/files': {
          const win = windows.find((w) => w.appId === 'files');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('files', { title: 'Study Vault & Files', icon: '🗂️' }, currentActivityId);
          }
          break;
        }
        case '/mood': {
          const win = windows.find((w) => w.appId === 'mood');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('mood', { title: 'Private Mood & Energy Check-in', icon: '🌈' }, currentActivityId);
          }
          break;
        }
        case '/balance': {
          const win = windows.find((w) => w.appId === 'balance');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('balance', { title: 'MindSet Balance AI', icon: '🌿' }, currentActivityId);
          }
          break;
        }
        case '/focus': {
          const win = windows.find((w) => w.appId === 'focus');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('focus', { title: 'Immersive Focus', icon: '⏳' }, currentActivityId);
          }
          break;
        }
        case '/focus-guard': {
          const win = windows.find((w) => w.appId === 'focus-guard');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('focus-guard', { title: 'Focus Guard & Shield', icon: '🛡️' }, currentActivityId);
          }
          break;
        }
        case '/insights': {
          const win = windows.find((w) => w.appId === 'insights');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('insights', { title: 'Weekly Life-Balance Analytics', icon: '📊' }, currentActivityId);
          }
          break;
        }
        case '/mind':
          setIsMemoryInspectorOpen(true);
          break;
        case '/settings': {
          const win = windows.find((w) => w.appId === 'settings');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('settings', { title: 'OS Settings Hub', icon: '⚙️' }, currentActivityId);
          }
          break;
        }
        case '/app-hub': {
          const win = windows.find((w) => w.appId === 'app-hub');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('app-hub', { title: 'MindSet App Hub', icon: '📦' }, currentActivityId);
          }
          break;
        }
        case '/devices': {
          const win = windows.find((w) => w.appId === 'devices');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('devices', { title: 'Hardware & Device Manager', icon: '🔌' }, currentActivityId);
          }
          break;
        }
        case '/status': {
          const win = windows.find((w) => w.appId === 'status');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('status', { title: 'Task Manager & Resource Monitor', icon: '📊' }, currentActivityId);
          }
          break;
        }
        case '/updater': {
          const win = windows.find((w) => w.appId === 'updater');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('updater', { title: 'Atomic OTA Updater', icon: '🚀' }, currentActivityId);
          }
          break;
        }
        case '/recovery': {
          const win = windows.find((w) => w.appId === 'recovery');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('recovery', { title: 'MindSet Recovery Console', icon: '🛟' }, currentActivityId);
          }
          break;
        }
        case '/connect': {
          const win = windows.find((w) => w.appId === 'connect');
          if (win) {
            bringToFront(win.id);
          } else {
            openWindow('connect', { title: 'MindSet Connect Cross-Device Hub', icon: '📱' }, currentActivityId);
          }
          break;
        }
      }
    });

    return () => unsubscribe();
  }, [windows, currentActivityId, openWindow, bringToFront, onOpenStatus]);

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

      {/* 3. Living User Profile Non-Intrusive Evolution Suggestion */}
      {activeSuggestion && userProfile && (
        <div className="absolute top-12 right-6 z-40 pointer-events-auto">
          <LivingProfileSuggestionCard
            suggestion={activeSuggestion}
            profile={userProfile}
            onActionComplete={(updated) => {
              if (updated) setUserProfile(updated);
              setActiveSuggestion(null);
            }}
          />
        </div>
      )}

      {/* 4. Snap Layout Translucent Liquid Glass Preview Overlay */}
      <SnapLayoutOverlay />

      {/* 5. KDE Plasma Activity Switcher Modal (Super + Tab) */}
      <ActivitySwitcherModal />

      {/* 6. Start Menu Launcher Overlay (Super) */}
      <StartMenuOverlay
        isOpen={isStartMenuOpen}
        onClose={() => setIsStartMenuOpen(false)}
        onLock={onLock}
        onOpenStatus={onOpenStatus}
        onOpenMemoryInspector={() => setIsMemoryInspectorOpen(true)}
        onOpenOnboarding={onOpenOnboarding}
        onOpenLanguageSettings={() => setIsLanguageModalOpen(true)}
      />

      {/* 7. Sovereign AI Memory Inspector Modal (Ctrl + M) */}
      <MemoryInspectorModal
        isOpen={isMemoryInspectorOpen}
        onClose={() => setIsMemoryInspectorOpen(false)}
      />

      {/* 8. Dual-Layer Language & AI Speech Settings Modal */}
      <LanguageSettingsModal
        isOpen={isLanguageModalOpen}
        onClose={() => setIsLanguageModalOpen(false)}
        onSaved={(p) => setUserProfile(p)}
      />

      {/* 9. Always-On-Top Floating AI Orb & Screen Doubt Solver (Alt + Space, Alt + S) */}
      <FloatingAiOrb />

      {/* 10. Global Command Palette & Unified Search (Ctrl + K) */}
      <GlobalCommandPalette
        isOpen={isCommandPaletteOpen}
        onClose={() => setIsCommandPaletteOpen(false)}
      />

      {/* 11. Non-Shaming Focus Guard Interruption Screen */}
      <FocusInterruptionOverlay
        isOpen={isFocusInterruptionOpen}
        blockedAppName={interruptedAppName}
        currentTaskTitle={priorityGoal}
        onReturnToPrevious={dismissFocusInterruption}
        onOpenStudyTask={() => {
          dismissFocusInterruption();
          const focusWin = windows.find((w) => w.appId === 'focus');
          if (focusWin) {
            bringToFront(focusWin.id);
          } else {
            openWindow('focus', { title: 'Immersive Focus', icon: '⏳' }, currentActivityId);
          }
        }}
      />

      {/* 12. Scientific Calculator & Physics Unit Converter Modal (Alt + C) */}
      <ScientificCalculatorModal
        isOpen={isCalculatorOpen}
        onClose={() => setIsCalculatorOpen(false)}
      />

      {/* 13. WCAG 2.2 AA Accessibility Controller (Screen Reader, Magnifier, Live Captions, Alt + A) */}
      <AccessibilityController />

      {/* 14. Persistent Bottom Liquid Glass Taskbar Dock */}
      <TaskbarDock
        isStartMenuOpen={isStartMenuOpen}
        onToggleStartMenu={() => setIsStartMenuOpen(!isStartMenuOpen)}
        onOpenStatus={onOpenStatus}
        onOpenCommandPalette={() => setIsCommandPaletteOpen(true)}
      />
    </div>
  );
};
