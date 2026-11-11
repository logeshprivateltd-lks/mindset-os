/**
 * MindSet OS Window TitleBar Component
 * Frameless window custom titlebar using semantic theme variables
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { useThemeStore } from '../styles/useThemeStore';
import { THEME_REGISTRY, type ThemeId } from '../tokens';

export interface TitleBarProps {
  title?: string;
  subtitle?: string;
  memoryUsageMb?: number;
  kernelPhase?: string;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onClose?: () => void;
}

export const TitleBar: React.FC<TitleBarProps> = ({
  title = "MindSet OS — Powered by LKS PVT LTD",
  subtitle = "Aurora Core v0.1.0",
  memoryUsageMb,
  kernelPhase,
  onMinimize,
  onMaximize,
  onClose,
}) => {
  const { theme, setTheme } = useThemeStore();
  const themeMeta = THEME_REGISTRY[theme];

  const cycleTheme = () => {
    const themeIds: ThemeId[] = [
      'aurora-focus',
      'solar-bloom',
      'forest-zen',
      'cosmic-arcade',
      'paper-studio',
    ];
    const currentIndex = themeIds.indexOf(theme);
    const nextTheme = themeIds[(currentIndex + 1) % themeIds.length] || 'aurora-focus';
    setTheme(nextTheme);
  };

  return (
    <header
      data-tauri-drag-region
      className="flex items-center justify-between h-9 px-3 border-b select-none text-xs z-50 fixed top-0 left-0 right-0 backdrop-blur-md transition-colors"
      style={{
        backgroundColor: 'var(--glass-bg)',
        borderColor: 'var(--border-subtle)',
        color: 'var(--text-muted)',
        WebkitAppRegion: 'drag',
      } as React.CSSProperties}
    >
      <div className="flex items-center space-x-2 pointer-events-none" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        <div
          className="w-2.5 h-2.5 rounded-full shadow-[0_0_8px_var(--border-glow)] animate-pulse"
          style={{ backgroundColor: 'var(--accent-primary)' }}
        />
        <span className="font-semibold tracking-wide" style={{ color: 'var(--text-dominant)' }}>
          {title}
        </span>
        <span className="opacity-40">|</span>
        <span style={{ color: 'var(--text-muted)' }}>{subtitle}</span>
      </div>

      <div className="flex items-center space-x-2.5" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        {/* Quick Theme Switcher Pill in Titlebar */}
        <button
          type="button"
          onClick={cycleTheme}
          title={`Active Theme: ${themeMeta?.name} (${themeMeta?.tagline}). Click to cycle.`}
          className="flex items-center space-x-1 px-2 py-0.5 rounded border transition-all hover:scale-105"
          style={{
            backgroundColor: 'var(--surface-elevated)',
            borderColor: 'var(--border-subtle)',
            color: 'var(--text-dominant)',
          }}
        >
          <span>{themeMeta?.icon}</span>
          <span className="font-semibold text-[10px] hidden sm:inline">{themeMeta?.name}</span>
        </button>

        {kernelPhase && (
          <div
            className="flex items-center space-x-1.5 px-2 py-0.5 rounded border text-[10px]"
            style={{
              backgroundColor: 'var(--surface-secondary)',
              borderColor: 'var(--border-subtle)',
              color: 'var(--text-dominant)',
            }}
          >
            <span style={{ color: 'var(--accent-secondary)' }}>●</span>
            <span className="font-mono">{kernelPhase}</span>
          </div>
        )}

        {typeof memoryUsageMb === 'number' && (
          <div
            className="flex items-center space-x-1.5 px-2 py-0.5 rounded border text-[10px]"
            style={{
              backgroundColor: 'var(--surface-primary)',
              borderColor: 'var(--border-subtle)',
              color: 'var(--text-muted)',
            }}
          >
            <span>RSS:</span>
            <span
              className={`font-mono font-semibold tabular-nums ${
                memoryUsageMb > 100 ? 'text-[#FFD078]' : 'text-[var(--accent-secondary)]'
              }`}
            >
              {memoryUsageMb.toFixed(1)} MB
            </span>
          </div>
        )}

        <div className="flex items-center space-x-1 pl-1">
          <button
            type="button"
            onClick={onMinimize}
            className="w-6 h-6 flex items-center justify-center rounded transition-colors hover:bg-[var(--surface-elevated)]"
            style={{ color: 'var(--text-muted)' }}
            aria-label="Minimize"
          >
            <svg width="10" height="2" viewBox="0 0 10 2" fill="currentColor">
              <rect width="10" height="2" rx="1" />
            </svg>
          </button>
          <button
            type="button"
            onClick={onMaximize}
            className="w-6 h-6 flex items-center justify-center rounded transition-colors hover:bg-[var(--surface-elevated)]"
            style={{ color: 'var(--text-muted)' }}
            aria-label="Maximize"
          >
            <svg width="9" height="9" viewBox="0 0 9 9" fill="none" stroke="currentColor" strokeWidth="1.5">
              <rect x="0.75" y="0.75" width="7.5" height="7.5" rx="1" />
            </svg>
          </button>
          <button
            type="button"
            onClick={onClose}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-red-500/20 hover:text-red-400 transition-colors"
            style={{ color: 'var(--text-muted)' }}
            aria-label="Close"
          >
            <svg width="9" height="9" viewBox="0 0 9 9" stroke="currentColor" strokeWidth="1.5">
              <line x1="1" y1="1" x2="8" y2="8" />
              <line x1="8" y1="1" x2="1" y2="8" />
            </svg>
          </button>
        </div>
      </div>
    </header>
  );
};
