/**
 * MindSet OS KDE Plasma-Style Activity Workspaces Store
 * Dedicated activity environments isolating open windows, shortcuts, and neuroaesthetic themes
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { create } from 'zustand';
import type { ThemeId } from '@mindset/ui-core';
import { useThemeStore } from '@mindset/ui-core';

export type ActivityId =
  | 'deep-study'
  | 'coding-projects'
  | 'planning-reflection'
  | 'relaxation';

export interface ActivityDefinition {
  id: ActivityId;
  name: string;
  icon: string;
  tagline: string;
  suggestedTheme: ThemeId;
  description: string;
  pinnedApps: string[];
  desktopShortcuts: Array<{
    id: string;
    title: string;
    icon: string;
    appId: string;
  }>;
}

export const ACTIVITIES: Record<ActivityId, ActivityDefinition> = {
  'deep-study': {
    id: 'deep-study',
    name: 'Deep Study',
    icon: '📖',
    tagline: 'Syllabus Focus & Textbook Ingestion',
    suggestedTheme: 'aurora-focus',
    description: 'Textbook PDF Reader, Focus Guard timer, StudyAgent AI tutor, and Exam Targets.',
    pinnedApps: ['reader', 'tasks', 'terminal', 'balance'],
    desktopShortcuts: [
      { id: 'sc-1', title: 'NCERT Biology Vol II.pdf', icon: '📄', appId: 'reader' },
      { id: 'sc-2', title: 'Differential Equations', icon: '📐', appId: 'tasks' },
      { id: 'sc-3', title: 'Focus Guard 45m Block', icon: '⏱️', appId: 'tasks' },
      { id: 'sc-4', title: 'MindSet Terminal', icon: '💻', appId: 'terminal' },
    ],
  },
  'coding-projects': {
    id: 'coding-projects',
    name: 'Coding & Projects',
    icon: '💻',
    tagline: 'Native Rust & Systems Engineering',
    suggestedTheme: 'aurora-focus',
    description: 'MindSet Terminal, Code Explorer, Architecture Specs, and Low-RAM Profiling.',
    pinnedApps: ['terminal', 'tasks', 'reader'],
    desktopShortcuts: [
      { id: 'sc-dev-1', title: 'MindSet Terminal', icon: '⚡', appId: 'terminal' },
      { id: 'sc-dev-2', title: 'Architecture Specs', icon: '📑', appId: 'reader' },
      { id: 'sc-dev-3', title: 'Release Milestones', icon: '🎯', appId: 'tasks' },
    ],
  },
  'planning-reflection': {
    id: 'planning-reflection',
    name: 'Planning & Reflection',
    icon: '🌿',
    tagline: 'Daily Habits & Mindful Review',
    suggestedTheme: 'forest-zen',
    description: 'Daily Journal, Habit Tracker, Mood Check-ins, and Non-punitive Streaks.',
    pinnedApps: ['journal', 'tasks', 'balance'],
    desktopShortcuts: [
      { id: 'sc-plan-1', title: 'Daily Evening Journal', icon: '✍️', appId: 'journal' },
      { id: 'sc-plan-2', title: 'Habit & Sleep Tracker', icon: '🌱', appId: 'journal' },
      { id: 'sc-plan-3', title: 'Mood Check-in', icon: '🧘', appId: 'balance' },
    ],
  },
  'relaxation': {
    id: 'relaxation',
    name: 'Controlled Relaxation',
    icon: '☕',
    tagline: 'Cortisol Reduction & Hobby Discovery',
    suggestedTheme: 'forest-zen',
    description: 'MindSet Balance AI, Gentle Breathwork, and Restorative Ambient Audio.',
    pinnedApps: ['balance', 'journal'],
    desktopShortcuts: [
      { id: 'sc-rel-1', title: 'MindSet Balance AI', icon: '🍵', appId: 'balance' },
      { id: 'sc-rel-2', title: 'Evening Breathwork', icon: '🌬️', appId: 'balance' },
      { id: 'sc-rel-3', title: 'Reflective Journal', icon: '📖', appId: 'journal' },
    ],
  },
};

export interface ActivityStoreState {
  currentActivityId: ActivityId;
  isSwitcherOpen: boolean;

  // Actions
  switchActivity: (activityId: ActivityId, applyTheme?: boolean) => void;
  toggleSwitcher: (open?: boolean) => void;
}

export const useActivityStore = create<ActivityStoreState>((set) => ({
  currentActivityId: 'deep-study',
  isSwitcherOpen: false,

  switchActivity: (activityId, applyTheme = true) => {
    const act = ACTIVITIES[activityId];
    if (!act) return;

    set({ currentActivityId: activityId, isSwitcherOpen: false });

    if (applyTheme) {
      // Suggest or switch to the activity's curated neuroaesthetic theme
      useThemeStore.getState().setTheme(act.suggestedTheme);
    }
  },

  toggleSwitcher: (open) => {
    set((state) => ({
      isSwitcherOpen: open !== undefined ? open : !state.isSwitcherOpen,
    }));
  },
}));
