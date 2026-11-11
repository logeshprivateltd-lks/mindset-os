/**
 * MindSet OS — Language & Regional AI Settings Modal
 * Modal container wrapping LanguageSettingsPanel for Desktop Shell and Start Menu
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import type { UserProfile } from '@mindset/db-schema';
import { GlassSurface } from '@mindset/ui-core';
import { LanguageSettingsPanel } from './LanguageSettingsPanel';

export interface LanguageSettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSaved?: (profile: UserProfile) => void;
}

export const LanguageSettingsModal: React.FC<LanguageSettingsModalProps> = ({
  isOpen,
  onClose,
  onSaved,
}) => {
  if (!isOpen) return null;

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-label="Language & Region Settings"
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm select-none animate-fadeIn"
    >
      <GlassSurface
        variant="modal"
        className="w-full max-w-4xl p-6 rounded-2xl relative shadow-2xl border flex flex-col max-h-[90vh] overflow-y-auto"
        style={{
          borderColor: 'var(--accent-primary)',
          backgroundColor: 'var(--surface-primary)',
          color: 'var(--text-dominant)',
        }}
      >
        {/* Top Header */}
        <div className="flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]">
          <div className="flex items-center gap-2">
            <span className="text-2xl">🌐</span>
            <div>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Language & Conversational AI Engine
              </h2>
              <p className="text-xs text-[var(--text-muted)]">
                Dual-layer configuration: Decoupled OS Interface Language & MindSet AI Speech Style
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 rounded-lg bg-[var(--surface-elevated)] hover:bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] transition"
          >
            ✕ Close
          </button>
        </div>

        {/* Embedded Panel */}
        <LanguageSettingsPanel
          onSaved={(profile) => {
            if (onSaved) onSaved(profile);
          }}
        />
      </GlassSurface>
    </div>
  );
};
