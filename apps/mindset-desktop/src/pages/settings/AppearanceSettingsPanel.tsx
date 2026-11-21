/**
 * MindSet OS Personalization Studio — Appearance & Styling Panel
 * 8 Accent Color Presets, Custom Hex, Liquid Glass Opacity, Font Scaling & Motion Intensity
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import {
  usePersonalizationStore,
  ACCENT_PRESETS,
} from '../../stores/usePersonalizationStore';

export const AppearanceSettingsPanel: React.FC = () => {
  const {
    accentColor,
    fontScale,
    glassOpacity,
    motionIntensity,
    setAccentColor,
    setFontScale,
    setGlassOpacity,
    setMotionIntensity,
  } = usePersonalizationStore();

  const [customHex, setCustomHex] = useState(accentColor);

  const handleCustomHexSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (/^#[0-9A-Fa-f]{6}$/.test(customHex)) {
      setAccentColor(customHex);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h4 className="text-sm font-bold text-[var(--text-dominant)]">
          Appearance, Accent Palette & Liquid Glass
        </h4>
        <p className="text-xs text-[var(--text-muted)]">
          Live customisation of system accent highlights, translucent blur depth, and typographical scaling.
        </p>
      </div>

      {/* Accent Color Palette */}
      <div className="space-y-3">
        <div className="flex items-center justify-between text-xs">
          <span className="font-semibold text-[var(--text-dominant)]">System Accent Palette:</span>
          <span className="font-mono text-[var(--accent-primary)]">{accentColor}</span>
        </div>

        <div className="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
          {ACCENT_PRESETS.map((preset) => {
            const isSelected = accentColor.toLowerCase() === preset.hex.toLowerCase();
            return (
              <button
                key={preset.name}
                type="button"
                onClick={() => {
                  setAccentColor(preset.hex);
                  setCustomHex(preset.hex);
                }}
                className={`p-2.5 rounded-xl border flex items-center space-x-2.5 text-left transition-all ${
                  isSelected
                    ? 'border-[var(--accent-primary)] ring-2 ring-[var(--accent-primary)]/40 bg-[var(--surface-elevated)] shadow-md'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)]/50 hover:bg-[var(--surface-secondary)]'
                }`}
              >
                <span
                  className="w-5 h-5 rounded-full border border-white/20 shadow-sm flex-shrink-0"
                  style={{ backgroundColor: preset.hex }}
                />
                <span className="text-xs font-semibold text-[var(--text-dominant)] truncate">
                  {preset.name}
                </span>
              </button>
            );
          })}
        </div>

        {/* Custom Hex Color Picker Form */}
        <form
          onSubmit={handleCustomHexSubmit}
          className="flex items-center space-x-2 pt-1"
        >
          <input
            type="color"
            value={accentColor}
            onChange={(e) => {
              setAccentColor(e.target.value);
              setCustomHex(e.target.value);
            }}
            className="w-9 h-9 rounded-xl border border-[var(--border-subtle)] bg-transparent cursor-pointer"
          />
          <input
            type="text"
            value={customHex}
            onChange={(e) => setCustomHex(e.target.value)}
            placeholder="#8B9CFF"
            className="px-3 py-1.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-primary)] text-xs font-mono text-[var(--text-dominant)] focus:outline-none focus:border-[var(--accent-primary)] w-28"
          />
          <button
            type="submit"
            className="px-3 py-1.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] text-xs font-medium hover:border-[var(--accent-primary)] text-[var(--text-dominant)] transition-colors"
          >
            Apply Hex
          </button>
        </form>
      </div>

      {/* Liquid Glass Opacity & Font Scale */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {/* Glass Opacity */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-[var(--text-dominant)]">Liquid Glass Opacity:</span>
            <span className="font-mono text-[var(--accent-primary)]">{Math.round(glassOpacity * 100)}%</span>
          </div>
          <input
            type="range"
            min="0.5"
            max="1.0"
            step="0.05"
            value={glassOpacity}
            onChange={(e) => setGlassOpacity(parseFloat(e.target.value))}
            className="w-full accent-[var(--accent-primary)] cursor-pointer"
          />
          <div className="text-[10px] text-[var(--text-muted)] flex justify-between">
            <span>50% (Translucent)</span>
            <span>100% (Solid Matte)</span>
          </div>
        </div>

        {/* Font Scale */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-[var(--text-dominant)]">System Typographical Scale:</span>
            <span className="font-mono text-[var(--accent-primary)]">{Math.round(fontScale * 100)}%</span>
          </div>
          <input
            type="range"
            min="0.9"
            max="1.25"
            step="0.05"
            value={fontScale}
            onChange={(e) => setFontScale(parseFloat(e.target.value))}
            className="w-full accent-[var(--accent-primary)] cursor-pointer"
          />
          <div className="text-[10px] text-[var(--text-muted)] flex justify-between">
            <span>90% (Compact)</span>
            <span>125% (Large Accessible)</span>
          </div>
        </div>
      </div>

      {/* Motion & Micro-interactions */}
      <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex items-center justify-between">
        <div>
          <div className="text-xs font-bold text-[var(--text-dominant)]">Motion & Transitions</div>
          <div className="text-[10px] text-[var(--text-muted)]">
            Adjust animation intensity for vestibular comfort or high-speed navigation
          </div>
        </div>

        <div className="flex items-center space-x-1">
          {(['reduced', 'normal', 'energetic'] as const).map((mode) => (
            <button
              key={mode}
              type="button"
              onClick={() => setMotionIntensity(mode)}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold capitalize transition-all ${
                motionIntensity === mode
                  ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                  : 'bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              {mode}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};

