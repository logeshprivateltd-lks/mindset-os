/**
 * MindSet OS Native App Registry
 * Maps appId to metadata and rendering components inside OsWindowFrame
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { StudyReaderApp } from './StudyReaderApp';
import { TerminalApp } from './TerminalApp';
import { TasksGoalsApp } from './TasksGoalsApp';
import { JournalNotesApp } from './JournalNotesApp';
import { BalanceRelaxApp } from './BalanceRelaxApp';
import { DashboardApp } from './DashboardApp';

export interface AppMetadata {
  appId: string;
  title: string;
  icon: string;
  subjectTag?: string;
  component: React.ComponentType;
  defaultWidth?: number;
  defaultHeight?: number;
  description: string;
}

export const APP_REGISTRY: Record<string, AppMetadata> = {
  dashboard: {
    appId: 'dashboard',
    title: 'Command Center & Telemetry',
    icon: '⚡',
    subjectTag: 'Core System',
    component: DashboardApp,
    defaultWidth: 960,
    defaultHeight: 640,
    description: 'Central command dashboard with 3-tier visual hierarchy and 5-theme switcher.',
  },
  reader: {
    appId: 'reader',
    title: 'Textbook & Study Reader',
    icon: '📖',
    subjectTag: 'Math / Physics',
    component: StudyReaderApp,
    defaultWidth: 920,
    defaultHeight: 620,
    description: 'Textbook PDF reader with syllabus chapter navigation and focus controls.',
  },
  terminal: {
    appId: 'terminal',
    title: 'MindSet OS Terminal',
    icon: '💻',
    subjectTag: 'Shell',
    component: TerminalApp,
    defaultWidth: 720,
    defaultHeight: 460,
    description: 'Native Rust diagnostic CLI and kernel telemetry monitor.',
  },
  tasks: {
    appId: 'tasks',
    title: 'Tasks & Syllabus Goals',
    icon: '🎯',
    subjectTag: 'Productivity',
    component: TasksGoalsApp,
    defaultWidth: 760,
    defaultHeight: 520,
    description: 'Syllabus milestones, daily task checklists, and exam targets.',
  },
  journal: {
    appId: 'journal',
    title: 'Daily Journal & Habits',
    icon: '✍️',
    subjectTag: 'Reflection',
    component: JournalNotesApp,
    defaultWidth: 800,
    defaultHeight: 560,
    description: 'Distraction-free Markdown journaling with non-punitive streak tracking.',
  },
  balance: {
    appId: 'balance',
    title: 'MindSet Balance AI',
    icon: '🌿',
    subjectTag: 'Wellbeing',
    component: BalanceRelaxApp,
    defaultWidth: 640,
    defaultHeight: 520,
    description: '4-7-8 cortisol-reducing breathwork guide and restorative ambient sounds.',
  },
};
