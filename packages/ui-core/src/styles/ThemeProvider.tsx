/**
 * MindSet OS ThemeProvider Component
 * Enforces instantaneous <16ms theme injection and displays Activity-Aware suggestions
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useEffect } from 'react';
import { useThemeStore } from './useThemeStore';
import { THEME_REGISTRY } from '../tokens';

export interface ThemeProviderProps {
  children: React.ReactNode;
  defaultTheme?: string;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const {
    theme,
    applyDomAttributes,
    loadStoredPreferences,
    activeSuggestion,
    acceptSuggestion,
    dismissSuggestion,
  } = useThemeStore();

  useEffect(() => {
    loadStoredPreferences();
    applyDomAttributes();
  }, [loadStoredPreferences, applyDomAttributes]);

  const targetMeta = activeSuggestion ? THEME_REGISTRY[activeSuggestion.targetTheme] : null;

  return (
    <>
      {children}

      {/* Activity-Aware Theme Suggestion Toast (Non-intrusive / Opt-in only) */}
      {activeSuggestion && targetMeta && (
        <aside
          role="region"
          aria-label="Theme switch suggestion"
          aria-live="polite"
          className="fixed bottom-6 right-6 z-50 max-w-sm w-full p-4 rounded-xl border shadow-2xl backdrop-blur-md animate-fadeIn transition-all"
          style={{
            backgroundColor: 'var(--glass-bg)',
            borderColor: 'var(--accent-primary)',
            boxShadow: '0 12px 36px rgba(0, 0, 0, 0.6), 0 0 16px var(--border-glow)',
          }}
        >
          <div className="flex items-start space-x-3">
            <span className="text-2xl" role="img" aria-hidden="true">
              {targetMeta.icon}
            </span>
            <div className="flex-1">
              <div className="flex items-center justify-between">
                <h4 className="text-xs font-bold uppercase tracking-wider text-[var(--accent-primary)]">
                  Context Suggestion
                </h4>
                <button
                  type="button"
                  onClick={dismissSuggestion}
                  className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1"
                  aria-label="Dismiss suggestion"
                >
                  ✕
                </button>
              </div>
              <p className="text-xs font-medium text-[var(--text-dominant)] mt-0.5">
                Switch to <span className="font-bold">{targetMeta.name}</span>?
              </p>
              <p className="text-[11px] text-[var(--text-muted)] mt-1 leading-snug">
                {activeSuggestion.reason}
              </p>

              <div className="flex items-center space-x-2 mt-3">
                <button
                  type="button"
                  onClick={acceptSuggestion}
                  className="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all hover:scale-102"
                  style={{
                    backgroundColor: 'var(--accent-primary)',
                    color: theme === 'solar-bloom' || theme === 'paper-studio' ? '#FFFFFF' : '#07090D',
                  }}
                >
                  Switch Theme
                </button>
                <button
                  type="button"
                  onClick={dismissSuggestion}
                  className="px-2.5 py-1.5 rounded-lg text-xs font-medium text-[var(--text-muted)] hover:text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
                >
                  Keep {THEME_REGISTRY[theme]?.name || 'Current'}
                </button>
              </div>
            </div>
          </div>
        </aside>
      )}
    </>
  );
};

