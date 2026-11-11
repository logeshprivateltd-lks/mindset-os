/**
 * MindSet OS Zustand Theme & Appearance Store
 * Real-time DOM attribute updates (<16ms) and persistent preference management
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { create } from 'zustand';
import type { ThemeId, DensityMode, FontScale, AnimationIntensity } from '../tokens';

export interface ThemeSuggestion {
  targetTheme: ThemeId;
  reason: string;
  sourceActivity: string;
  timestamp: number;
}

export interface ThemePreferences {
  theme: ThemeId;
  density: DensityMode;
  fontScale: FontScale;
  animationIntensity: AnimationIntensity;
  highContrast: boolean;
  reducedMotion: boolean;
}

export interface ThemeStoreState extends ThemePreferences {
  activeSuggestion: ThemeSuggestion | null;

  // Actions
  setTheme: (theme: ThemeId) => void;
  setDensity: (density: DensityMode) => void;
  setFontScale: (scale: FontScale) => void;
  setAnimationIntensity: (intensity: AnimationIntensity) => void;
  setHighContrast: (highContrast: boolean) => void;
  setReducedMotion: (reducedMotion: boolean) => void;

  // Activity-Aware Suggestions
  suggestTheme: (targetTheme: ThemeId, reason: string, sourceActivity?: string) => void;
  acceptSuggestion: () => void;
  dismissSuggestion: () => void;

  // DOM sync
  applyDomAttributes: () => void;
  loadStoredPreferences: () => void;
}

const STORAGE_KEY = 'mindset_neuroaesthetic_preferences';

function getInitialPreferences(): ThemePreferences {
  if (typeof window === 'undefined') {
    return {
      theme: 'aurora-focus',
      density: 'comfortable',
      fontScale: 'md',
      animationIntensity: 'full',
      highContrast: false,
      reducedMotion: false,
    };
  }

  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw);
      return {
        theme: parsed.theme || 'aurora-focus',
        density: parsed.density || 'comfortable',
        fontScale: parsed.fontScale || 'md',
        animationIntensity: parsed.animationIntensity || 'full',
        highContrast: Boolean(parsed.highContrast),
        reducedMotion: Boolean(parsed.reducedMotion),
      };
    }
  } catch {
    // fallback
  }

  const prefersReduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  return {
    theme: 'aurora-focus',
    density: 'comfortable',
    fontScale: 'md',
    animationIntensity: prefersReduced ? 'none' : 'full',
    highContrast: false,
    reducedMotion: prefersReduced,
  };
}

export const useThemeStore = create<ThemeStoreState>((set, get) => {
  const initial = getInitialPreferences();

  return {
    ...initial,
    activeSuggestion: null,

    applyDomAttributes: () => {
      if (typeof document === 'undefined') return;
      const root = document.documentElement;
      const state = get();

      root.setAttribute('data-theme', state.theme);
      root.setAttribute('data-density', state.density);
      root.setAttribute('data-font-scale', state.fontScale);
      root.setAttribute('data-high-contrast', String(state.highContrast));
      root.setAttribute('data-reduced-motion', String(state.reducedMotion));

      // Persist preferences locally
      try {
        const payload: ThemePreferences = {
          theme: state.theme,
          density: state.density,
          fontScale: state.fontScale,
          animationIntensity: state.animationIntensity,
          highContrast: state.highContrast,
          reducedMotion: state.reducedMotion,
        };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
      } catch {
        // quota exceeded or SSR
      }
    },

    loadStoredPreferences: () => {
      const prefs = getInitialPreferences();
      set({ ...prefs });
      get().applyDomAttributes();
    },

    setTheme: (theme: ThemeId) => {
      set({ theme, activeSuggestion: null });
      get().applyDomAttributes();
    },

    setDensity: (density: DensityMode) => {
      set({ density });
      get().applyDomAttributes();
    },

    setFontScale: (fontScale: FontScale) => {
      set({ fontScale });
      get().applyDomAttributes();
    },

    setAnimationIntensity: (animationIntensity: AnimationIntensity) => {
      const reducedMotion = animationIntensity === 'none';
      set({ animationIntensity, reducedMotion });
      get().applyDomAttributes();
    },

    setHighContrast: (highContrast: boolean) => {
      set({ highContrast });
      get().applyDomAttributes();
    },

    setReducedMotion: (reducedMotion: boolean) => {
      set({
        reducedMotion,
        animationIntensity: reducedMotion ? 'none' : get().animationIntensity,
      });
      get().applyDomAttributes();
    },

    suggestTheme: (targetTheme: ThemeId, reason: string, sourceActivity = 'Activity Monitor') => {
      // Don't suggest if user is already on that theme
      if (get().theme === targetTheme) return;
      set({
        activeSuggestion: {
          targetTheme,
          reason,
          sourceActivity,
          timestamp: Date.now(),
        },
      });
    },

    acceptSuggestion: () => {
      const suggestion = get().activeSuggestion;
      if (suggestion) {
        set({ theme: suggestion.targetTheme, activeSuggestion: null });
        get().applyDomAttributes();
      }
    },

    dismissSuggestion: () => {
      set({ activeSuggestion: null });
    },
  };
});

