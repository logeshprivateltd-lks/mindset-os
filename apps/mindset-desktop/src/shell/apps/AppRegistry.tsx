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
import { FileExplorerPage, RagDocumentTutor } from '../../pages/files';
import { HabitsPage } from '../../pages/habits';
import { TimelinePage } from '../../pages/timeline';
import { GoalsAndSyllabusView, SmartPlannerPage } from '../../pages/tasks';

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
  files: {
    appId: 'files',
    title: 'Study Vault & Files',
    icon: '🗂️',
    subjectTag: 'Filesystem',
    component: FileExplorerPage,
    defaultWidth: 1040,
    defaultHeight: 680,
    description: 'Distraction-free file explorer with study roots, instant previews, and RAG indexing.',
  },
  tutor: {
    appId: 'tutor',
    title: 'RAG Document Tutor',
    icon: '🎓',
    subjectTag: 'StudyAgent',
    component: RagDocumentTutor,
    defaultWidth: 1120,
    defaultHeight: 720,
    description: 'Split-screen textbook reader with context-locked StudyAgent and [Page X] citations.',
  },
  habits: {
    appId: 'habits',
    title: 'Habits & Subconscious Conditioning',
    icon: '🔥',
    subjectTag: 'Mindset',
    component: HabitsPage,
    defaultWidth: 840,
    defaultHeight: 580,
    description: 'Non-punitive habit tracker with consistency heatmaps and morning/evening visualization.',
  },
  timeline: {
    appId: 'timeline',
    title: 'Chronological Activity Timeline',
    icon: '⏱️',
    subjectTag: 'Activity',
    component: TimelinePage,
    defaultWidth: 860,
    defaultHeight: 600,
    description: 'Immutable chronological activity stream logging tasks, habits, and focus blocks.',
  },
  goals: {
    appId: 'goals',
    title: 'Academic Goals & Syllabus (590/600)',
    icon: '🌳',
    subjectTag: 'Syllabus',
    component: GoalsAndSyllabusView,
    defaultWidth: 960,
    defaultHeight: 640,
    description: 'Class 12 target calculator and chapter mastery skill-tree with auto-scheduler.',
  },
  planner: {
    appId: 'planner',
    title: 'Smart Timetable & Planner',
    icon: '📅',
    subjectTag: 'Schedule',
    component: SmartPlannerPage,
    defaultWidth: 920,
    defaultHeight: 620,
    description: 'Configurable recurring schedule template with drag-and-drop task time-blocking.',
  },
};
