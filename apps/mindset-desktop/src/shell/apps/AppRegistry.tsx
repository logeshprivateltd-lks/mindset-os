/**
 * MindSet OS Native App Registry
 * Maps appId to metadata and rendering components inside OsWindowFrame
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { StudyReaderApp } from './StudyReaderApp';
import { TerminalApp } from './TerminalApp';
import { TasksGoalsApp } from './TasksGoalsApp';
import { DashboardApp } from './DashboardApp';
import { FileExplorerPage, RagDocumentTutor } from '../../pages/files';
import { HabitsPage } from '../../pages/habits';
import { TimelinePage } from '../../pages/timeline';
import { GoalsAndSyllabusView, SmartPlannerPage } from '../../pages/tasks';
import { FocusSessionPage, FocusGuardDashboard } from '../../pages/focus';
import { BalanceAiPage } from '../../pages/balance';
import { JournalPage } from '../../pages/journal';
import { InsightsPage } from '../../pages/insights';
import { MoodCheckinModal } from '../../pages/mood';
import { SettingsPage } from '../../pages/settings';
import { AppHubPage } from '../../pages/app-hub';
import { DeviceManagerPage } from '../../pages/devices';
import { SystemStatusPage } from '../../pages/status';
import { AtomicUpdaterPage } from '../../pages/updater';
import { RecoveryModePage } from '../../pages/recovery';
import { MindSetConnectPage } from '../../pages/connect/MindSetConnectPage';
import { OfflineLecturePlayer } from '../../components/utilities/OfflineLecturePlayer';

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
    title: 'Daily Journal & Gratitude',
    icon: '✍️',
    subjectTag: 'Reflection',
    component: JournalPage,
    defaultWidth: 920,
    defaultHeight: 620,
    description: 'Distraction-free Markdown journaling with daily gratitude prompts and local SQLite encryption.',
  },
  balance: {
    appId: 'balance',
    title: 'MindSet Balance AI',
    icon: '🌿',
    subjectTag: 'Wellbeing',
    component: BalanceAiPage,
    defaultWidth: 960,
    defaultHeight: 640,
    description: '4-7-8 cortisol breathwork, smart free-time planner, 12-hobby explorer, and intentional play.',
  },
  insights: {
    appId: 'insights',
    title: 'Weekly Life-Balance Analytics',
    icon: '📊',
    subjectTag: 'Analytics',
    component: InsightsPage,
    defaultWidth: 960,
    defaultHeight: 640,
    description: 'Interactive Recharts 9-dimension life balance radar, focus trends, and supportive Coach observations.',
  },
  mood: {
    appId: 'mood',
    title: 'Private Mood & Energy Check-in',
    icon: '🌈',
    subjectTag: 'Wellbeing',
    component: MoodCheckinModal,
    defaultWidth: 640,
    defaultHeight: 540,
    description: '9-mood and 4-energy private state check-in with restorative guidance and SQLite logging.',
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
  focus: {
    appId: 'focus',
    title: 'Immersive Focus Session',
    icon: '⏳',
    subjectTag: 'Focus',
    component: FocusSessionPage,
    defaultWidth: 960,
    defaultHeight: 640,
    description: 'Fullscreen distraction-free Aurora countdown timer with offline Web Audio ambient sound synthesizer.',
  },
  'focus-guard': {
    appId: 'focus-guard',
    title: 'Focus Guard & Distraction Shield',
    icon: '🛡️',
    subjectTag: 'Protection',
    component: FocusGuardDashboard,
    defaultWidth: 880,
    defaultHeight: 620,
    description: 'Non-shaming distraction shield, gaming budget controls, and emergency VIP communication allowlist.',
  },
  settings: {
    appId: 'settings',
    title: 'OS Settings Hub',
    icon: '⚙️',
    subjectTag: 'System',
    component: SettingsPage,
    defaultWidth: 960,
    defaultHeight: 640,
    description: '10-domain configuration: Neuroaesthetic themes, display, audio, local AI, biometrics, and WCAG accessibility.',
  },
  'app-hub': {
    appId: 'app-hub',
    title: 'MindSet App Hub',
    icon: '📦',
    subjectTag: 'Software',
    component: AppHubPage,
    defaultWidth: 1040,
    defaultHeight: 680,
    description: 'Curated distraction-free software catalog for STEM, coding, and productivity with per-process permission controls.',
  },
  player: {
    appId: 'player',
    title: 'Offline Lecture Player',
    icon: '🎬',
    subjectTag: 'Media',
    component: OfflineLecturePlayer,
    defaultWidth: 920,
    defaultHeight: 620,
    description: 'Distraction-free local video player with 0.75x–2.5x speed control and timestamp bookmarking to study notes.',
  },
  devices: {
    appId: 'devices',
    title: 'Hardware & Device Manager',
    icon: '🔌',
    subjectTag: 'Hardware',
    component: DeviceManagerPage,
    defaultWidth: 1040,
    defaultHeight: 680,
    description: 'Hardware topology, USB peripherals, Wi-Fi driver diagnostics, power governors, and media fallback routing.',
  },
  status: {
    appId: 'status',
    title: 'Task Manager & Resource Monitor',
    icon: '📊',
    subjectTag: 'System',
    component: SystemStatusPage,
    defaultWidth: 1060,
    defaultHeight: 700,
    description: 'btop++ telemetry monitor, <120 MB OS Shell ceiling verification, cgroups v2 limiter, and self-healing doctor.',
  },
  updater: {
    appId: 'updater',
    title: 'Atomic OTA Updater',
    icon: '🚀',
    subjectTag: 'System',
    component: AtomicUpdaterPage,
    defaultWidth: 1040,
    defaultHeight: 680,
    description: 'Dual-layer Tauri 2 delta updater with Ed25519 signature checks, ostree A/B slots, and 3-second rollback.',
  },
  recovery: {
    appId: 'recovery',
    title: 'MindSet Recovery Console',
    icon: '🛟',
    subjectTag: 'System',
    component: RecoveryModePage,
    defaultWidth: 1040,
    defaultHeight: 680,
    description: 'Pre-boot standalone rescue environment for encrypted snapshots, SQLite repairs, and driver safe-mode resets.',
  },
  connect: {
    appId: 'connect',
    title: 'MindSet Connect Cross-Device Hub',
    icon: '📱',
    subjectTag: 'System',
    component: MindSetConnectPage,
    defaultWidth: 1080,
    defaultHeight: 700,
    description: '6-Module cross-device service for wireless webcam/mic streaming, continuity state handoff, and selective sync.',
  },
};
