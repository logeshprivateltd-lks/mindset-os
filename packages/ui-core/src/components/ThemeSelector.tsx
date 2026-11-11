/**
 * MindSet OS Neuroaesthetic Theme Selector Component
 * Allows instant, zero-layout-shift switching across all 5 themes, densities, and font scales.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { useThemeStore } from '../styles/useThemeStore';
import { THEME_REGISTRY, type DensityMode, type FontScale } from '../tokens';

export interface ThemeSelectorProps {
  className?: string;
  compact?: boolean;
}

export const ThemeSelector: React.FC<ThemeSelectorProps> = ({ className = '', compact = false }) => {
  const {
    theme: currentTheme,
    density,
    fontScale,
    highContrast,
    reducedMotion,
    setTheme,
    setDensity,
    setFontScale,
    setHighContrast,
    setReducedMotion,
    suggestTheme,
  } = useThemeStore();

  const themes = Object.values(THEME_REGISTRY);

  if (compact) {
    return (
      <div className={`flex items-center space-x-1 ${className}`}>
        {themes.map((t) => {
          const isSelected = currentTheme === t.id;
          return (
            <button
              key={t.id}
              type="button"
              onClick={() => setTheme(t.id)}
              title={`${t.name} — ${t.tagline}`}
              className={`w-7 h-7 rounded-lg flex items-center justify-center text-xs transition-all ${
                isSelected
                  ? 'ring-2 ring-[var(--accent-primary)] scale-110 shadow-md font-bold'
                  : 'opacity-70 hover:opacity-100 hover:scale-105'
              }`}
              style={{
                backgroundColor: t.previewColors.bg,
                borderColor: isSelected ? t.previewColors.accent : 'transparent',
              }}
            >
              <span>{t.icon}</span>
            </button>
          );
        })}
      </div>
    );
  }

  return (
    <div
      className={`p-4 rounded-xl border space-y-4 select-none ${className}`}
      style={{
        backgroundColor: 'var(--surface-primary)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      {/* 1. Theme Palette Grid */}
      <div>
        <div className="text-xs font-bold uppercase tracking-wider mb-2 text-[var(--accent-primary)]">
          Neuroaesthetic Themes (5 Palettes)
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-5 gap-2">
          {themes.map((t) => {
            const isSelected = currentTheme === t.id;
            return (
              <button
                key={t.id}
                type="button"
                onClick={() => setTheme(t.id)}
                className={`p-2.5 rounded-xl border text-left transition-all ${
                  isSelected
                    ? 'border-[var(--accent-primary)] shadow-[0_0_16px_var(--border-glow)] scale-[1.02]'
                    : 'border-[var(--border-subtle)] hover:border-[var(--border-elevated)] hover:scale-[1.01]'
                }`}
                style={{
                  backgroundColor: isSelected ? 'var(--surface-elevated)' : 'var(--surface-secondary)',
                }}
              >
                <div className="flex items-center justify-between">
                  <span className="text-lg">{t.icon}</span>
                  {isSelected && (
                    <span className="text-[10px] font-bold px-1.5 py-0.5 rounded text-[var(--accent-primary)] bg-[var(--surface-primary)]">
                      Active
                    </span>
                  )}
                </div>

                <div className="font-bold text-xs mt-1.5 text-[var(--text-dominant)]">{t.name}</div>
                <div className="text-[10px] text-[var(--text-muted)] line-clamp-1 mt-0.5">
                  {t.targetState}
                </div>

                {/* Color Swatch Strip */}
                <div className="flex items-center space-x-1 mt-2.5 pt-1.5 border-t border-[var(--border-subtle)]">
                  <span
                    className="w-3.5 h-3.5 rounded-full border border-black/20"
                    style={{ backgroundColor: t.previewColors.bg }}
                    title="Background"
                  />
                  <span
                    className="w-3.5 h-3.5 rounded-full border border-black/20"
                    style={{ backgroundColor: t.previewColors.surface }}
                    title="Surface"
                  />
                  <span
                    className="w-3.5 h-3.5 rounded-full"
                    style={{ backgroundColor: t.previewColors.accent }}
                    title="Accent Primary"
                  />
                  <span
                    className="w-3.5 h-3.5 rounded-full"
                    style={{ backgroundColor: t.previewColors.accentSecondary }}
                    title="Accent Secondary"
                  />
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* 2. Interface Density & Font Scale Controls */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-3 border-t border-[var(--border-subtle)]">
        {/* Density Segmented Control */}
        <div>
          <label className="text-[11px] font-semibold text-[var(--text-muted)] uppercase tracking-wider block mb-1.5">
            Interface Density
          </label>
          <div className="flex rounded-lg p-1 bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
            {(['compact', 'comfortable', 'spacious'] as DensityMode[]).map((d) => (
              <button
                key={d}
                type="button"
                onClick={() => setDensity(d)}
                className={`flex-1 py-1 text-xs font-semibold capitalize rounded-md transition-colors ${
                  density === d
                    ? 'bg-[var(--surface-elevated)] text-[var(--accent-primary)] shadow-sm'
                    : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {d}
              </button>
            ))}
          </div>
        </div>

        {/* Font Scale Segmented Control */}
        <div>
          <label className="text-[11px] font-semibold text-[var(--text-muted)] uppercase tracking-wider block mb-1.5">
            Font Scaling
          </label>
          <div className="flex rounded-lg p-1 bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
            {(['sm', 'md', 'lg', 'xl'] as FontScale[]).map((scale) => (
              <button
                key={scale}
                type="button"
                onClick={() => setFontScale(scale)}
                className={`flex-1 py-1 text-xs font-semibold uppercase rounded-md transition-colors ${
                  fontScale === scale
                    ? 'bg-[var(--surface-elevated)] text-[var(--accent-primary)] shadow-sm'
                    : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {scale}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* 3. Accessibility & Activity Suggestion Test Strip */}
      <div className="flex flex-wrap items-center justify-between gap-3 pt-3 border-t border-[var(--border-subtle)] text-xs">
        <div className="flex items-center space-x-4">
          <label className="flex items-center space-x-2 cursor-pointer">
            <input
              type="checkbox"
              checked={highContrast}
              onChange={(e) => setHighContrast(e.target.checked)}
              className="rounded accent-[var(--accent-primary)]"
            />
            <span className="text-[var(--text-dominant)]">High Contrast (WCAG)</span>
          </label>

          <label className="flex items-center space-x-2 cursor-pointer">
            <input
              type="checkbox"
              checked={reducedMotion}
              onChange={(e) => setReducedMotion(e.target.checked)}
              className="rounded accent-[var(--accent-primary)]"
            />
            <span className="text-[var(--text-dominant)]">Reduced Motion</span>
          </label>
        </div>

        {/* Quick Context Suggestion Simulator Buttons */}
        <div className="flex items-center space-x-2">
          <span className="text-[10px] text-[var(--text-muted)] uppercase font-semibold">
            Simulate Activity:
          </span>
          <button
            type="button"
            onClick={() =>
              suggestTheme(
                'paper-studio',
                'Detected NCERT Biology PDF in Vault Reader. Paper Studio optimizes typographic contrast for sustained reading.',
                'PDF Reader'
              )
            }
            className="px-2 py-0.5 rounded text-[11px] border border-[var(--border-subtle)] hover:bg-[var(--surface-elevated)] text-[var(--text-dominant)] transition-colors"
          >
            📜 PDF Reading
          </button>
          <button
            type="button"
            onClick={() =>
              suggestTheme(
                'forest-zen',
                'Completed a 45-minute Deep Work sprint. Forest Zen reduces cortisol during relaxation and reflection.',
                'Rest Period'
              )
            }
            className="px-2 py-0.5 rounded text-[11px] border border-[var(--border-subtle)] hover:bg-[var(--surface-elevated)] text-[var(--text-dominant)] transition-colors"
          >
            🌿 Rest Break
          </button>
          <button
            type="button"
            onClick={() =>
              suggestTheme(
                'cosmic-arcade',
                'Milestone quiz started: Differential Equations Speed Run. Cosmic Arcade activates focus dopamine.',
                'Quiz Arena'
              )
            }
            className="px-2 py-0.5 rounded text-[11px] border border-[var(--border-subtle)] hover:bg-[var(--surface-elevated)] text-[var(--text-dominant)] transition-colors"
          >
            🕹️ Quiz Sprints
          </button>
        </div>
      </div>
    </div>
  );
};
