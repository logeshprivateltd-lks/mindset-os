/**
 * Aurora Focus Design Tokens
 * MindSet OS — Powered by LKS PVT LTD
 * Ultra-low-RAM, High-Contrast Cognitive Focus Palette
 */

export const AURORA_TOKENS = {
  // Base Backgrounds
  background: '#07090D',
  surfacePrimary: '#111827',
  surfaceSecondary: '#182235',
  surfaceElevated: '#202B40',

  // Accents
  accentPrimary: '#8B9CFF',     // Electric Indigo / Cognitive Flow
  accentSecondary: '#65E6C4',   // Emerald Teal / Energy & Calm
  accentAchievement: '#FFD078', // Solar Amber / Milestone & Streaks
  accentDanger: '#FF6B6B',      // Crimson / Lockdown & Alert

  // Typography
  textDominant: '#F9FAFB',
  textSecondary: '#E5E7EB',
  textMuted: '#9CA3AF',
  textDim: '#6B7280',

  // Borders & Outlines
  borderSubtle: '#1F2937',
  borderElevated: '#374151',
  borderHighlight: 'rgba(139, 156, 255, 0.35)',

  // Shadows & Glass
  glassBackground: 'rgba(17, 24, 39, 0.75)',
  glassBorder: 'rgba(139, 156, 255, 0.15)',
} as const;

export type AuroraTokens = typeof AURORA_TOKENS;

