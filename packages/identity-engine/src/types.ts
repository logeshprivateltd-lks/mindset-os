/**
 * MindSet OS — Identity, Multilingual & 3-Tier Memory Types
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { UserProfile } from '@mindset/db-schema';
export type { SupportedLocale } from '@mindset/i18n-locales';

// ---------------------------------------------------------------------------
// 1. Student Goals & Ambitions
// ---------------------------------------------------------------------------

export interface GoalItem {
  id: string;
  title: string;
  category?: string;
  priority: 1 | 2 | 3;
  targetScore?: number | string;
  targetDate?: string;
  completed: boolean;
  progressPercent: number;
}

// ---------------------------------------------------------------------------
// 2. Routine & Schedule
// ---------------------------------------------------------------------------

export interface RoutineCommitment {
  title: string;
  startTime: string; // e.g., "08:30"
  endTime: string;   // e.g., "16:00"
  days: number[];    // 0=Sun, 1=Mon, ..., 6=Sat
}

export interface StudySlot {
  startTime: string; // e.g., "06:30"
  endTime: string;   // e.g., "08:00"
  label?: string;
}

export interface RoutineSchedule {
  wakeTime: string;  // e.g., "06:00"
  sleepTime: string; // e.g., "22:30"
  commitments: RoutineCommitment[];
  preferredStudySlots: StudySlot[];
  breakDurationMinutes: number;
}

// ---------------------------------------------------------------------------
// 3. Learning Styles & AI Communication Tone
// ---------------------------------------------------------------------------

export type LearningStyle =
  | 'step_by_step'
  | 'visual'
  | 'practical_examples'
  | 'reading_notes'
  | 'interactive';

export type AiTone =
  | 'friendly_direct'
  | 'encouraging'
  | 'analytical'
  | 'concise';

export type PlanningStyle = 'flexible' | 'structured';

export interface StudentPreferences {
  learningStyle: LearningStyle;
  aiTone: AiTone;
  planningStyle: PlanningStyle;
  soundscape?: string;
  tanglishEnabled: boolean;
  themePreference?: string;
}

// ---------------------------------------------------------------------------
// 4. Three-Tier Memory Architecture
// ---------------------------------------------------------------------------

/** Level 1: Durable Personal Profile Memory */
export interface MemoryLevel1_Profile {
  userId: string;
  displayName: string;
  role: string;
  interfaceLang: string;
  aiLang: string;
  conversationStyle: string;
  goals: GoalItem[];
  focusAreas: string[];
  routine: RoutineSchedule;
  preferences: StudentPreferences;
  interests: string[];
  permissions: Record<string, boolean>;
  updatedAt: string;
}

/** Level 2: Volatile Active Working Context */
export interface MemoryLevel2_ActiveContext {
  currentTime: string;
  availableFreeMinutes: number;
  nextCommitment: {
    title: string;
    startTime: string;
    minutesRemaining: number;
  } | null;
  activeFocusMode: string | null;
  pendingTasks: Array<{
    id: string;
    title: string;
    priority: number;
    subjectTag?: string | null;
  }>;
  recentMood: string | null;
  recentEnergy: string | null;
  activeActivity: string | null;
  lastCompletedTask?: string | null;
}

/** Level 3: Long-Term Epistemic Progress Memory */
export interface MemoryLevel3_Record {
  id: string;
  sourceType:
    | 'goal_milestone'
    | 'syllabus_topic'
    | 'journal_reflection'
    | 'exam_score'
    | 'concept_mastery';
  title: string;
  details: string;
  timestamp: string;
  tags: string[];
  vectorId?: string;
  score?: number;
}

// ---------------------------------------------------------------------------
// 5. Prompt Injection & Sanitization
// ---------------------------------------------------------------------------

export interface PromptSanitizationResult {
  sanitized: string;
  flaggedViolations: string[];
  hadSuspiciousPatterns: boolean;
}

export interface ContextBuilderOptions {
  maxContextTokens?: number;
  includeLevel3Memories?: boolean;
  enforceCloudConsent?: boolean;
}

// ---------------------------------------------------------------------------
// 6. Context Assembly Results
// ---------------------------------------------------------------------------

export interface AssembledPromptResult {
  systemPrompt: string;
  userContextXml: string;
  fullComposedPrompt: string;
  tokenEstimate: number;
  contextTimestamp: string;
  cloudConsentPassed: boolean;
  warnings: string[];
}

// ---------------------------------------------------------------------------
// 7. Living Profile Evolution
// ---------------------------------------------------------------------------

export type EvolutionType =
  | 'goal_completed'
  | 'routine_drift'
  | 'focus_shift'
  | 'habit_streak';

export interface ProfileEvolutionSuggestion {
  id: string;
  type: EvolutionType;
  title: string;
  reason: string;
  promptMessage: string;
  proposedChanges: Partial<UserProfile>;
  createdAt: string;
  status: 'pending' | 'accepted' | 'rejected' | 'dismissed';
}

// ---------------------------------------------------------------------------
// 8. Memory Inspector & Export
// ---------------------------------------------------------------------------

export interface MemoryInspectionFilter {
  sourceType?: string;
  searchTerm?: string;
  tag?: string;
  limit?: number;
}

export interface MemoryExportPayload {
  exportedAt: string;
  version: string;
  level1Profile: MemoryLevel1_Profile;
  level2ContextSnapshot: MemoryLevel2_ActiveContext | null;
  level3Memories: MemoryLevel3_Record[];
}
