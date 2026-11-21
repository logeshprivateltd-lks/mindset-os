/**
 * MindSet OS Personalization Studio — Desktop Interactive Widgets Panel
 * Toggles and configuration for Hero Clock, Exam Target Card, Affirmation, and Desktop Scratchpad
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { usePersonalizationStore } from '../../stores/usePersonalizationStore';

export const DesktopWidgetsPanel: React.FC = () => {
  const { widgets, toggleWidget } = usePersonalizationStore();

  const widgetDefinitions = [
    {
      key: 'clock' as const,
      icon: '⏱️',
      title: 'Hero Clock & Date Pill',
      desc: 'High-visibility digital chronometer with seconds precision and daily calendar date.',
    },
    {
      key: 'target' as const,
      icon: '🎯',
      title: 'Class 12 Exam Target Card (590/600)',
      desc: 'Ambient visual anchor tracking your Board Exam and JEE preparation trajectory.',
    },
    {
      key: 'affirmation' as const,
      icon: '🌿',
      title: 'Daily Student Stoic Affirmation',
      desc: 'Non-shaming micro-affirmation calibrated to regulate pre-exam nervous tension.',
    },
    {
      key: 'notes' as const,
      icon: '📝',
      title: 'Desktop Sticky Scratchpad',
      desc: 'Persistent floating pad for rapid formula scratchwork and temporary focus notes.',
    },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h4 className="text-sm font-bold text-[var(--text-dominant)]">
          Desktop Interactive Ambient Widgets
        </h4>
        <p className="text-xs text-[var(--text-muted)]">
          Zero-distraction ambient cards placed directly on the desktop canvas behind open application windows.
        </p>
      </div>

      {/* Widget Cards List */}
      <div className="space-y-3">
        {widgetDefinitions.map((w) => {
          const isEnabled = !!widgets[w.key];
          return (
            <div
              key={w.key}
              className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex items-center justify-between gap-4 transition-all hover:border-[var(--accent-primary)]/40"
            >
              <div className="flex items-start space-x-3.5">
                <div className="w-10 h-10 rounded-xl bg-[var(--surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-xl flex-shrink-0">
                  {w.icon}
                </div>
                <div>
                  <div className="text-xs font-bold text-[var(--text-dominant)] flex items-center space-x-2">
                    <span>{w.title}</span>
                    {isEnabled && (
                      <span className="px-1.5 py-0.2 rounded text-[9px] font-mono bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                        ACTIVE
                      </span>
                    )}
                  </div>
                  <div className="text-[11px] text-[var(--text-muted)] mt-0.5 leading-relaxed">
                    {w.desc}
                  </div>
                </div>
              </div>

              {/* Toggle Switch */}
              <button
                type="button"
                role="switch"
                aria-checked={isEnabled}
                onClick={() => toggleWidget(w.key)}
                className={`w-12 h-6 rounded-full transition-colors relative flex-shrink-0 ${
                  isEnabled ? 'bg-[var(--accent-primary)]' : 'bg-slate-700'
                }`}
              >
                <span
                  className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow-md transform transition-transform ${
                    isEnabled ? 'translate-x-6' : 'translate-x-0'
                  }`}
                />
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
};

