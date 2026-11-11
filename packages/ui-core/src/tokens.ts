/**
 * MindSet OS Neuroaesthetic Design Tokens
 * 5-Theme Palettes, 3-Tier Visual Hierarchy, and Accessibility Specifications
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export type ThemeId =
  | 'aurora-focus'
  | 'solar-bloom'
  | 'forest-zen'
  | 'cosmic-arcade'
  | 'paper-studio';

export type DensityMode = 'compact' | 'comfortable' | 'spacious';
export type FontScale = 'sm' | 'md' | 'lg' | 'xl';
export type AnimationIntensity = 'none' | 'minimal' | 'full';

export interface ThemeMetadata {
  id: ThemeId;
  name: string;
  tagline: string;
  targetState: string;
  icon: string;
  colorScheme: 'dark' | 'light';
  previewColors: {
    bg: string;
    surface: string;
    accent: string;
    accentSecondary: string;
  };
}

export const THEME_REGISTRY: Record<ThemeId, ThemeMetadata> = {
  'aurora-focus': {
    id: 'aurora-focus',
    name: 'Aurora Focus',
    tagline: 'Default Deep Work & Cognitive Flow',
    targetState: 'Deep Study & Engineering Flow',
    icon: '🌌',
    colorScheme: 'dark',
    previewColors: {
      bg: '#07090D',
      surface: '#111827',
      accent: '#8B9CFF',
      accentSecondary: '#65E6C4',
    },
  },
  'solar-bloom': {
    id: 'solar-bloom',
    name: 'Solar Bloom',
    tagline: 'Warm Daylight & Morning Lectures',
    targetState: 'High-Alert Daytime Reading & Sprints',
    icon: '☀️',
    colorScheme: 'light',
    previewColors: {
      bg: '#FAF7EE',
      surface: '#FFFFFF',
      accent: '#E06D14',
      accentSecondary: '#3B9B78',
    },
  },
  'forest-zen': {
    id: 'forest-zen',
    name: 'Forest Zen',
    tagline: 'Biophilic Cortisol-Reducing Calm',
    targetState: 'Evening Reflection & Balance AI',
    icon: '🌿',
    colorScheme: 'dark',
    previewColors: {
      bg: '#09120E',
      surface: '#11201A',
      accent: '#52C794',
      accentSecondary: '#82D9B3',
    },
  },
  'cosmic-arcade': {
    id: 'cosmic-arcade',
    name: 'Cosmic Arcade',
    tagline: 'Synthwave Gamified Dopamine Mode',
    targetState: 'Interactive Quizzes & Skill Trees',
    icon: '🕹️',
    colorScheme: 'dark',
    previewColors: {
      bg: '#080514',
      surface: '#120E29',
      accent: '#D847FF',
      accentSecondary: '#00F0FF',
    },
  },
  'paper-studio': {
    id: 'paper-studio',
    name: 'Paper Studio',
    tagline: 'Tactile Editorial & Distraction-Free E-Ink',
    targetState: 'Long-Form Textbook Reading & Drafting',
    icon: '📜',
    colorScheme: 'light',
    previewColors: {
      bg: '#F5F2EB',
      surface: '#FAF8F5',
      accent: '#2B303A',
      accentSecondary: '#5C6479',
    },
  },
};

export const AURORA_TOKENS = {
  background: '#07090D',
  surfacePrimary: '#111827',
  surfaceSecondary: '#182235',
  surfaceElevated: '#202B40',
  accentPrimary: '#8B9CFF',
  accentSecondary: '#65E6C4',
  accentAchievement: '#FFD078',
  accentCreative: '#FF8EAA',
  accentDanger: '#FF6B6B',
  textDominant: '#F9FAFB',
  textSecondary: '#E5E7EB',
  textMuted: '#9CA3AF',
  textDim: '#6B7280',
  borderSubtle: '#1F2937',
  borderElevated: '#374151',
  borderHighlight: 'rgba(139, 156, 255, 0.35)',
  glassBackground: 'rgba(17, 24, 39, 0.75)',
  glassBorder: 'rgba(139, 156, 255, 0.15)',
} as const;

export type AuroraTokens = typeof AURORA_TOKENS;
