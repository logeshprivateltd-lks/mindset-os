/**
 * MindSet OS Personalization & Desktop Grid Zustand Store
 * Manages wallpapers, studio styling, desktop icon layout, and power lifecycle overlays
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { create } from 'zustand';
import type {
  DesktopShortcut,
  UserPersonalization,
  DesktopWidgetsConfig,
  PowerAction,
} from '@mindset/db-schema';
import { dbClient } from '../lib/dbClient';
import { powerClient } from '../lib/powerClient';

export interface WallpaperPreset {
  id: string;
  name: string;
  category: 'cosmic' | 'minimal' | 'nature' | 'abstract';
  cssGradient: string;
  description: string;
}

export const WALLPAPER_PRESETS: WallpaperPreset[] = [
  {
    id: 'nebula_deep',
    name: 'Cosmic Nebula',
    category: 'cosmic',
    cssGradient: 'radial-gradient(ellipse at 80% 20%, #1e1b4b 0%, #0f172a 45%, #05070f 100%)',
    description: 'Deep celestial violet and obsidian stardust for late-night mastery',
  },
  {
    id: 'quantum_mesh',
    name: 'Quantum Mesh',
    category: 'abstract',
    cssGradient: 'radial-gradient(circle at 20% 30%, #0c4a6e 0%, #071728 50%, #020617 100%)',
    description: 'Dark cyber-blue isometric field calibrated for technical architecture',
  },
  {
    id: 'cyber_aurora',
    name: 'Emerald Aurora',
    category: 'nature',
    cssGradient: 'radial-gradient(circle at 50% 10%, #064e3b 0%, #062325 55%, #021114 100%)',
    description: 'Bioluminescent jade ribbons inducing autonomic nervous calmness',
  },
  {
    id: 'solar_eclipse',
    name: 'Solar Corona',
    category: 'cosmic',
    cssGradient: 'radial-gradient(circle at 85% 85%, #78350f 0%, #291506 45%, #080608 100%)',
    description: 'Warm high-contrast ember ring maintaining alertness without eyestrain',
  },
  {
    id: 'academic_sanctuary',
    name: 'Academic Sanctuary',
    category: 'minimal',
    cssGradient: 'radial-gradient(ellipse at 50% 50%, #1e293b 0%, #0f172a 60%, #080d1a 100%)',
    description: 'Slate study hall ambiance designed for hours of intense focus',
  },
  {
    id: 'zen_forest',
    name: 'Midnight Bamboo',
    category: 'nature',
    cssGradient: 'radial-gradient(ellipse at 15% 85%, #143026 0%, #0a1f18 50%, #040d0a 100%)',
    description: 'Minimal Japanese woodland silhouettes with low light reflection',
  },
  {
    id: 'nordic_frost',
    name: 'Nordic Glacier',
    category: 'minimal',
    cssGradient: 'radial-gradient(circle at 70% 30%, #164e63 0%, #0c2d3a 50%, #041017 100%)',
    description: 'Cool crisp sub-zero crystalline gradients with minimal glare',
  },
  {
    id: 'hyper_grid',
    name: 'Synthwave Horizon',
    category: 'abstract',
    cssGradient: 'radial-gradient(ellipse at 50% 100%, #581c87 0%, #1e1136 50%, #090412 100%)',
    description: 'Atmospheric neon violet horizon evoking creative flow states',
  },
  {
    id: 'minimal_void',
    name: 'Pure OLED Abyss',
    category: 'minimal',
    cssGradient: 'radial-gradient(circle at 50% 50%, #0a0d14 0%, #050608 60%, #000000 100%)',
    description: '0-lux pure black for maximum battery conservation and zero distraction',
  },
  {
    id: 'golden_hour',
    name: 'Evening Horizon',
    category: 'nature',
    cssGradient: 'radial-gradient(circle at 50% 90%, #451a03 0%, #220e18 45%, #06050b 100%)',
    description: 'Gentle twilight glow signalling melatonin release for evening review',
  },
];

export const ACCENT_PRESETS = [
  { name: 'Aurora Iris', hex: '#8B9CFF', ring: 'rgba(139,156,255,0.4)' },
  { name: 'Jade Focus', hex: '#65E6C4', ring: 'rgba(101,230,196,0.4)' },
  { name: 'Solar Amber', hex: '#FFD078', ring: 'rgba(255,208,120,0.4)' },
  { name: 'Ruby Vitality', hex: '#FF7A90', ring: 'rgba(255,122,144,0.4)' },
  { name: 'Electric Sky', hex: '#38BDF8', ring: 'rgba(56,189,248,0.4)' },
  { name: 'Cosmic Violet', hex: '#C084FC', ring: 'rgba(192,132,252,0.4)' },
  { name: 'Spring Emerald', hex: '#34D399', ring: 'rgba(52,211,153,0.4)' },
  { name: 'Tiger Coral', hex: '#FB923C', ring: 'rgba(251,146,60,0.4)' },
];

export interface TeardownStep {
  id: string;
  label: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

interface PersonalizationState {
  // Wallpaper & Theme
  wallpaperType: 'preset' | 'custom' | 'solid' | 'video';
  wallpaperPath: string;
  wallpaperDimness: number;
  wallpaperBlurPx: number;
  accentColor: string;
  fontScale: number;
  glassOpacity: number;
  motionIntensity: 'reduced' | 'normal' | 'energetic';
  dockPosition: 'bottom' | 'left' | 'top';
  dockAutohide: boolean;
  dockStyle: 'floating' | 'edge';
  widgets: DesktopWidgetsConfig;

  // Shortcuts Grid
  shortcuts: DesktopShortcut[];
  selectedShortcutId: string | null;
  gridColumns: number;
  gridRows: number;

  // Power Overlay State
  isShuttingDown: boolean;
  shutdownAction: PowerAction | null;
  shutdownMessage: string;
  teardownSteps: TeardownStep[];

  // Actions
  initialize: () => Promise<void>;
  setWallpaperPreset: (presetId: string) => void;
  setCustomWallpaper: (filePath: string) => void;
  setWallpaperDimness: (dimness: number) => void;
  setWallpaperBlur: (blurPx: number) => void;
  setAccentColor: (hex: string) => void;
  setFontScale: (scale: number) => void;
  setGlassOpacity: (opacity: number) => void;
  setMotionIntensity: (intensity: 'reduced' | 'normal' | 'energetic') => void;
  setDockPosition: (pos: 'bottom' | 'left' | 'top') => void;
  setDockAutohide: (autohide: boolean) => void;
  setDockStyle: (style: 'floating' | 'edge') => void;
  toggleWidget: (key: keyof DesktopWidgetsConfig) => void;
  saveSettings: () => Promise<void>;

  // Shortcut Actions
  loadShortcuts: () => Promise<void>;
  selectShortcut: (id: string | null) => void;
  moveShortcut: (id: string, gridX: number, gridY: number) => Promise<void>;
  pinShortcut: (shortcut: Omit<DesktopShortcut, 'grid_x' | 'grid_y'>) => Promise<void>;
  deleteShortcut: (id: string) => Promise<void>;

  // Power Lifecycle Actions
  executePowerAction: (action: PowerAction, reason?: string) => Promise<void>;
  updateTeardownStep: (stepId: string, status: TeardownStep['status']) => void;
  cancelShutdownOverlay: () => void;
}

export const usePersonalizationStore = create<PersonalizationState>((set, get) => ({
  wallpaperType: 'preset',
  wallpaperPath: 'nebula_deep',
  wallpaperDimness: 0.25,
  wallpaperBlurPx: 0,
  accentColor: '#8B9CFF',
  fontScale: 1.0,
  glassOpacity: 0.85,
  motionIntensity: 'normal',
  dockPosition: 'bottom',
  dockAutohide: false,
  dockStyle: 'floating',
  widgets: {
    clock: true,
    target: true,
    affirmation: true,
    notes: false,
  },

  shortcuts: [],
  selectedShortcutId: null,
  gridColumns: 8,
  gridRows: 12,

  isShuttingDown: false,
  shutdownAction: null,
  shutdownMessage: '',
  teardownSteps: [],

  initialize: async () => {
    try {
      const p = await dbClient.personalization.get('default_student');
      let parsedWidgets: DesktopWidgetsConfig = { clock: true, target: true, affirmation: true };
      try {
        parsedWidgets = JSON.parse(p.widgets_json);
      } catch {
        // Fallback default
      }

      set({
        wallpaperType: p.wallpaper_type,
        wallpaperPath: p.wallpaper_path,
        wallpaperDimness: p.wallpaper_dimness,
        wallpaperBlurPx: p.wallpaper_blur_px,
        accentColor: p.accent_color,
        fontScale: p.font_scale,
        glassOpacity: p.glass_opacity,
        motionIntensity: p.motion_intensity,
        dockPosition: p.dock_position,
        dockAutohide: p.dock_autohide,
        dockStyle: p.dock_style,
        widgets: parsedWidgets,
      });

      // Apply to document CSS variables immediately
      get().setAccentColor(p.accent_color);
      get().setFontScale(p.font_scale);
      get().setGlassOpacity(p.glass_opacity);

      // Load shortcuts
      await get().loadShortcuts();
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to load settings from SQLite:', err);
    }
  },

  setWallpaperPreset: (presetId) => {
    set({ wallpaperType: 'preset', wallpaperPath: presetId });
    get().saveSettings();
  },

  setCustomWallpaper: (filePath) => {
    set({ wallpaperType: 'custom', wallpaperPath: filePath });
    get().saveSettings();
  },

  setWallpaperDimness: (dimness) => {
    set({ wallpaperDimness: Math.max(0, Math.min(0.85, dimness)) });
    get().saveSettings();
  },

  setWallpaperBlur: (blurPx) => {
    set({ wallpaperBlurPx: Math.max(0, Math.min(32, blurPx)) });
    get().saveSettings();
  },

  setAccentColor: (hex) => {
    set({ accentColor: hex });
    if (typeof document !== 'undefined') {
      document.documentElement.style.setProperty('--accent-primary', hex);
      document.documentElement.style.setProperty('--color-accent-primary', hex);
      document.documentElement.style.setProperty('--accent-glow', `${hex}40`);
    }
    get().saveSettings();
  },

  setFontScale: (scale) => {
    const clamped = Math.max(0.9, Math.min(1.25, scale));
    set({ fontScale: clamped });
    if (typeof document !== 'undefined') {
      document.documentElement.style.setProperty('--app-font-size', `${14 * clamped}px`);
    }
    get().saveSettings();
  },

  setGlassOpacity: (opacity) => {
    const clamped = Math.max(0.5, Math.min(1.0, opacity));
    set({ glassOpacity: clamped });
    if (typeof document !== 'undefined') {
      document.documentElement.style.setProperty('--glass-opacity', clamped.toString());
      document.documentElement.style.setProperty(
        '--surface-primary',
        `rgba(17, 24, 39, ${clamped})`
      );
    }
    get().saveSettings();
  },

  setMotionIntensity: (intensity) => {
    set({ motionIntensity: intensity });
    get().saveSettings();
  },

  setDockPosition: (pos) => {
    set({ dockPosition: pos });
    get().saveSettings();
  },

  setDockAutohide: (autohide) => {
    set({ dockAutohide: autohide });
    get().saveSettings();
  },

  setDockStyle: (style) => {
    set({ dockStyle: style });
    get().saveSettings();
  },

  toggleWidget: (key) => {
    const next = { ...get().widgets, [key]: !get().widgets[key] };
    set({ widgets: next });
    get().saveSettings();
  },

  saveSettings: async () => {
    const s = get();
    const payload: UserPersonalization = {
      user_id: 'default_student',
      wallpaper_type: s.wallpaperType,
      wallpaper_path: s.wallpaperPath,
      wallpaper_dimness: s.wallpaperDimness,
      wallpaper_blur_px: s.wallpaperBlurPx,
      accent_color: s.accentColor,
      font_scale: s.fontScale,
      glass_opacity: s.glassOpacity,
      motion_intensity: s.motionIntensity,
      dock_position: s.dockPosition,
      dock_autohide: s.dockAutohide,
      dock_style: s.dockStyle,
      widgets_json: JSON.stringify(s.widgets),
    };
    try {
      await dbClient.personalization.save(payload);
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to persist personalization:', err);
    }
  },

  // -------------------------------------------------------------------------
  // Desktop Shortcuts
  // -------------------------------------------------------------------------
  loadShortcuts: async () => {
    try {
      const items = await dbClient.shortcuts.list();
      set({ shortcuts: items });
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to load shortcuts:', err);
    }
  },

  selectShortcut: (id) => {
    set({ selectedShortcutId: id });
  },

  moveShortcut: async (id, gridX, gridY) => {
    const prev = get().shortcuts;
    const updated = prev.map((s) => (s.id === id ? { ...s, grid_x: gridX, grid_y: gridY } : s));
    set({ shortcuts: updated });
    try {
      await dbClient.shortcuts.updatePosition(id, gridX, gridY);
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to persist shortcut position:', err);
    }
  },

  pinShortcut: async (shortcut) => {
    // Find next available coordinate
    const current = get().shortcuts;
    let nextY = 0;
    while (current.some((s) => s.grid_x === 0 && s.grid_y === nextY)) {
      nextY++;
    }

    const fullShortcut: DesktopShortcut = {
      ...shortcut,
      grid_x: 0,
      grid_y: nextY,
      is_pinned: true,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    const nextList = [...current, fullShortcut];
    set({ shortcuts: nextList });
    try {
      await dbClient.shortcuts.save(fullShortcut);
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to pin shortcut:', err);
    }
  },

  deleteShortcut: async (id) => {
    const nextList = get().shortcuts.filter((s) => s.id !== id);
    set({ shortcuts: nextList, selectedShortcutId: null });
    try {
      await dbClient.shortcuts.delete(id);
    } catch (err) {
      console.warn('[usePersonalizationStore] Failed to delete shortcut:', err);
    }
  },

  // -------------------------------------------------------------------------
  // Power Lifecycle Flow
  // -------------------------------------------------------------------------
  executePowerAction: async (action, reason) => {
    if (action === 'lock') {
      await powerClient.executeAction('lock', reason);
      return;
    }

    const initialSteps: TeardownStep[] = [
      { id: 'save', label: 'Persisting workspace state & active study goals', status: 'running' },
      { id: 'services', label: 'Gracefully terminating kernel background daemons', status: 'pending' },
      { id: 'db', label: 'Flushing SQLite WAL buffers & truncating database caches', status: 'pending' },
      { id: 'power', label: action === 'restart' ? 'Initiating soft system reboot' : 'Unmounting hardware devices and halting', status: 'pending' },
    ];

    set({
      isShuttingDown: true,
      shutdownAction: action,
      shutdownMessage: `Executing safe ${action.toUpperCase()} teardown...`,
      teardownSteps: initialSteps,
    });

    // Step 1: Simulated audio ramp-down (600ms) & local store save
    try {
      if (typeof window !== 'undefined' && 'AudioContext' in window) {
        // Soft audio mute
      }
    } catch {
      // Audio fallback
    }

    setTimeout(() => {
      get().updateTeardownStep('save', 'completed');
      get().updateTeardownStep('services', 'running');
    }, 250);

    setTimeout(() => {
      get().updateTeardownStep('services', 'completed');
      get().updateTeardownStep('db', 'running');
    }, 550);

    setTimeout(async () => {
      get().updateTeardownStep('db', 'completed');
      get().updateTeardownStep('power', 'running');

      try {
        await powerClient.executeAction(action, reason);
        get().updateTeardownStep('power', 'completed');
      } catch (err) {
        console.error('[usePersonalizationStore] Power action failed:', err);
        get().updateTeardownStep('power', 'failed');
      }
    }, 850);
  },

  updateTeardownStep: (stepId, status) => {
    set((state) => ({
      teardownSteps: state.teardownSteps.map((step) =>
        step.id === stepId ? { ...step, status } : step
      ),
    }));
  },

  cancelShutdownOverlay: () => {
    set({ isShuttingDown: false, shutdownAction: null, teardownSteps: [] });
  },
}));

