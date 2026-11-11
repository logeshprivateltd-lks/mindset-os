/**
 * MindSet OS Start Menu Overlay Component
 * Liquid Glass launcher triggered by the MindSet Start button or Super key
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { useWindowManagerStore } from './wm/useWindowManagerStore';
import { useActivityStore, ACTIVITIES, type ActivityId } from './activities/useActivityStore';
import { APP_REGISTRY } from './apps/AppRegistry';

export interface StartMenuOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  onLock: () => void;
  onOpenStatus: () => void;
  onOpenMemoryInspector?: () => void;
  onOpenOnboarding?: () => void;
  onOpenLanguageSettings?: () => void;
}

export const StartMenuOverlay: React.FC<StartMenuOverlayProps> = ({
  isOpen,
  onClose,
  onLock,
  onOpenStatus,
  onOpenMemoryInspector,
  onOpenOnboarding,
  onOpenLanguageSettings,
}) => {
  const { openWindow } = useWindowManagerStore();
  const { currentActivityId, switchActivity } = useActivityStore();

  const [searchQuery, setSearchQuery] = useState('');

  if (!isOpen) return null;

  const appList = Object.values(APP_REGISTRY);
  const filteredApps = appList.filter((a) =>
    a.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    a.description.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleLaunchApp = (appId: string) => {
    openWindow(appId, {}, currentActivityId);
    onClose();
  };

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-label="Start Menu"
      onClick={onClose}
      className="fixed inset-0 z-50 bg-black/40 backdrop-blur-sm flex items-end justify-start p-4 pb-14 animate-fadeIn select-none"
    >
      <div
        onClick={(e) => e.stopPropagation()}
        className="w-full max-w-lg rounded-2xl border shadow-2xl overflow-hidden backdrop-blur-2xl space-y-4 p-5"
        style={{
          backgroundColor: 'var(--glass-bg)',
          borderColor: 'var(--accent-primary)',
          boxShadow: '0 20px 60px rgba(0, 0, 0, 0.8), 0 0 24px var(--border-glow)',
        }}
      >
        {/* Search Bar */}
        <div className="relative">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search syllabus, apps, files, or settings..."
            className="w-full px-4 py-2.5 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent-primary)] outline-none text-xs text-[var(--text-dominant)] pl-9"
            autoFocus
          />
          <span className="absolute left-3 top-2.5 text-xs text-[var(--text-muted)]">🔍</span>
        </div>

        {/* Pinned Applications Grid */}
        <div className="space-y-2">
          <div className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
            Pinned Workspace Applications
          </div>

          <div className="grid grid-cols-3 gap-2">
            {filteredApps.map((app) => (
              <button
                key={app.appId}
                type="button"
                onClick={() => handleLaunchApp(app.appId)}
                className="p-3 rounded-xl border border-[var(--border-subtle)] hover:border-[var(--accent-primary)] bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] flex flex-col items-center text-center transition-all hover:scale-102"
              >
                <span className="text-2xl">{app.icon}</span>
                <span className="text-xs font-semibold text-[var(--text-dominant)] mt-1.5 line-clamp-1">
                  {app.title}
                </span>
                <span className="text-[10px] text-[var(--text-muted)] line-clamp-1">
                  {app.subjectTag || 'Native App'}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Activity Quick Switcher Strip */}
        <div className="pt-2 border-t border-[var(--border-subtle)] space-y-1.5">
          <div className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
            Current Activity Workspace
          </div>
          <div className="grid grid-cols-4 gap-1.5">
            {Object.values(ACTIVITIES).map((act) => {
              const isCur = currentActivityId === act.id;
              return (
                <button
                  key={act.id}
                  type="button"
                  onClick={() => {
                    switchActivity(act.id as ActivityId);
                    onClose();
                  }}
                  className={`p-2 rounded-lg border text-center transition-colors ${
                    isCur
                      ? 'border-[var(--accent-primary)] bg-[var(--surface-elevated)] text-[var(--accent-primary)] font-bold'
                      : 'border-[var(--border-subtle)] bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                  }`}
                >
                  <span className="text-base block">{act.icon}</span>
                  <span className="text-[10px] block truncate mt-0.5">{act.name}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Footer: User Profile Badge & Power Controls */}
        <div className="pt-3 border-t border-[var(--border-subtle)] flex items-center justify-between text-xs">
          {/* User Badge */}
          <div className="flex items-center space-x-2.5">
            <div className="w-8 h-8 rounded-full bg-[var(--accent-primary)]/20 border border-[var(--accent-primary)] flex items-center justify-center text-sm font-bold text-[var(--accent-primary)]">
              MS
            </div>
            <div>
              <div className="font-bold text-[var(--text-dominant)]">MindSet Scholar</div>
              <div className="text-[10px] text-[var(--text-muted)]">default_student • LKS PVT LTD</div>
            </div>
          </div>

          {/* Action Buttons */}
          <div className="flex items-center space-x-1.5">
            {onOpenLanguageSettings && (
              <button
                type="button"
                onClick={() => {
                  onOpenLanguageSettings();
                  onClose();
                }}
                title="Language & AI Engine Settings"
                className="p-2 rounded-lg bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] text-[var(--accent-primary)] border border-[var(--border-subtle)] transition-colors"
              >
                🌐
              </button>
            )}

            {onOpenMemoryInspector && (
              <button
                type="button"
                onClick={() => {
                  onOpenMemoryInspector();
                  onClose();
                }}
                title="AI Memory Inspector (3-Tier Epistemic Memory)"
                className="p-2 rounded-lg bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] text-[var(--accent-primary)] border border-[var(--border-subtle)] transition-colors"
              >
                🧠
              </button>
            )}

            {onOpenOnboarding && (
              <button
                type="button"
                onClick={() => {
                  onOpenOnboarding();
                  onClose();
                }}
                title="Re-run Setup Wizard (/onboarding)"
                className="p-2 rounded-lg bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] text-[var(--accent-secondary)] border border-[var(--border-subtle)] transition-colors"
              >
                ✨
              </button>
            )}

            <button
              type="button"
              onClick={() => {
                onOpenStatus();
                onClose();
              }}
              title="Open Kernel Diagnostics (/status)"
              className="p-2 rounded-lg bg-[var(--surface-primary)] hover:bg-[var(--surface-elevated)] text-[var(--text-muted)] hover:text-[var(--text-dominant)] border border-[var(--border-subtle)] transition-colors"
            >
              ⚙️
            </button>

            <button
              type="button"
              onClick={() => {
                onLock();
                onClose();
              }}
              title="Lock Screen (Super + L)"
              className="p-2 rounded-lg bg-[var(--surface-primary)] hover:bg-red-500/20 text-[var(--text-muted)] hover:text-red-400 border border-[var(--border-subtle)] transition-colors"
            >
              🔒
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

