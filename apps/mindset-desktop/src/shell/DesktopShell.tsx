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
  const { windows, openWindow } = useWindowManagerStore();
  const { currentActivityId, toggleSwitcher } = useActivityStore();

  const [isStartMenuOpen, setIsStartMenuOpen] = useState(false);
  const [isMemoryInspectorOpen, setIsMemoryInspectorOpen] = useState(false);
  const [isLanguageModalOpen, setIsLanguageModalOpen] = useState(false);
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
  // Escape -> close Start Menu
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.altKey) && e.key === 'Tab') {
        e.preventDefault();
        toggleSwitcher();
      } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'm') {
        e.preventDefault();
        setIsMemoryInspectorOpen((prev) => !prev);
      } else if (e.key === 'Escape' && isStartMenuOpen) {
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

      {/* 10. Persistent Bottom Liquid Glass Taskbar Dock */}
      <TaskbarDock
        isStartMenuOpen={isStartMenuOpen}
        onToggleStartMenu={() => setIsStartMenuOpen(!isStartMenuOpen)}
        onOpenStatus={onOpenStatus}
      />
    </div>
  );
};
