/**
 * MindSet OS KDE Plasma-Style Activity Workspace Switcher Modal
 * Triggered via Super + Tab to seamlessly transition between isolated study environments
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useEffect } from 'react';
import { useActivityStore, ACTIVITIES, type ActivityId } from './useActivityStore';
import { THEME_REGISTRY } from '@mindset/ui-core';

export const ActivitySwitcherModal: React.FC = () => {
  const { currentActivityId, isSwitcherOpen, switchActivity, toggleSwitcher } = useActivityStore();

  const activityList = Object.values(ACTIVITIES);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Super + Tab or Alt + Tab triggers activity switcher
      if ((e.metaKey || e.altKey) && e.key === 'Tab') {
        e.preventDefault();
        toggleSwitcher(!isSwitcherOpen);
      } else if (e.key === 'Escape' && isSwitcherOpen) {
        e.preventDefault();
        toggleSwitcher(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isSwitcherOpen, toggleSwitcher]);

  if (!isSwitcherOpen) return null;

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby="activity-switcher-title"
      onClick={() => toggleSwitcher(false)}
      className="fixed inset-0 z-50 flex items-center justify-center p-6 bg-black/65 backdrop-blur-2xl animate-fadeIn select-none"
    >
      <div
        onClick={(e) => e.stopPropagation()}
        className="w-full max-w-4xl p-6 rounded-2xl border shadow-2xl space-y-6"
        style={{
          backgroundColor: 'var(--surface-primary)',
          borderColor: 'var(--accent-primary)',
          boxShadow: '0 24px 64px rgba(0, 0, 0, 0.8), 0 0 32px var(--border-glow)',
        }}
      >
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[var(--border-subtle)] pb-4">
          <div className="flex items-center space-x-3">
            <span className="text-2xl">🌌</span>
            <div>
              <h2 id="activity-switcher-title" className="text-lg font-bold text-[var(--text-dominant)]">
                KDE Plasma Activity Workspaces
              </h2>
              <p className="text-xs text-[var(--text-muted)]">
                Isolated window workspaces & neuroaesthetic themes • Hotkey: <kbd className="font-mono text-[var(--accent-primary)]">Super + Tab</kbd>
              </p>
            </div>
          </div>

          <button
            type="button"
            onClick={() => toggleSwitcher(false)}
            className="w-8 h-8 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            ✕
          </button>
        </div>

        {/* 4 Activity Cards Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {activityList.map((act) => {
            const isActive = currentActivityId === act.id;
            const themeMeta = THEME_REGISTRY[act.suggestedTheme];

            return (
              <div
                key={act.id}
                onClick={() => switchActivity(act.id as ActivityId)}
                className={`p-5 rounded-xl border text-left cursor-pointer transition-all duration-200 ${
                  isActive
                    ? 'border-[var(--accent-primary)] shadow-[0_0_24px_var(--border-glow)] scale-[1.02] bg-[var(--surface-elevated)]'
                    : 'border-[var(--border-subtle)] hover:border-[var(--border-elevated)] hover:scale-[1.01] bg-[var(--surface-secondary)]'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2.5">
                    <span className="text-3xl p-2 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)]">
                      {act.icon}
                    </span>
                    <div>
                      <h3 className="text-base font-bold text-[var(--text-dominant)]">{act.name}</h3>
                      <p className="text-xs text-[var(--text-muted)]">{act.tagline}</p>
                    </div>
                  </div>

                  {isActive && (
                    <span className="px-2 py-0.5 rounded text-xs font-bold font-mono bg-[var(--accent-primary)] text-[#07090D]">
                      Current
                    </span>
                  )}
                </div>

                <p className="text-xs text-[var(--text-muted)] mt-3 leading-relaxed">
                  {act.description}
                </p>

                {/* Footer Badges */}
                <div className="flex items-center justify-between mt-4 pt-3 border-t border-[var(--border-subtle)] text-[11px]">
                  <div className="flex items-center space-x-1.5">
                    <span className="text-[var(--text-muted)]">Theme:</span>
                    <span
                      className="px-2 py-0.5 rounded font-medium border"
                      style={{
                        backgroundColor: themeMeta?.previewColors.bg,
                        borderColor: themeMeta?.previewColors.accent,
                        color: themeMeta?.colorScheme === 'dark' ? '#F9FAFB' : '#1C1917',
                      }}
                    >
                      {themeMeta?.icon} {themeMeta?.name}
                    </span>
                  </div>

                  <span className="text-[var(--accent-secondary)] font-medium">
                    {act.pinnedApps.length} Pinned Tools →
                  </span>
                </div>
              </div>
            );
          })}
        </div>

        <div className="text-center text-[11px] text-[var(--text-muted)] pt-1">
          Switching activities isolates workspace windows without closing background study timers or kernel threads.
        </div>
      </div>
    </div>
  );
};

