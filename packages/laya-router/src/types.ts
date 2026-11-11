/**
 * MindSet OS — Laya AI Router Types
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { UserProfile, TaskOrGoal, Habit, JournalEntry } from '@mindset/db-schema';

export type AgentPersona =
  | 'StudyAgent'
  | 'PlannerAgent'
  | 'HabitAgent'
  | 'MemoryAgent'
  | 'CoachAgent';

export interface PersonaConfig {
  id: AgentPersona;
  displayName: string;
  tagline: string;
  avatarIcon: string;
  systemBasePrompt: string;
  defaultTemperature: number;
  recommendedChips: string[];
}

export interface LayaMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export interface PersonalContextPayload {
  profile?: Partial<UserProfile> | null;
  tasks?: TaskOrGoal[];
  habits?: Habit[];
  journals?: JournalEntry[];
  currentActivity?: string;
  energyScore?: number;
  todayCompletedCount?: number;
}

export interface LayaChatOptions {
  model?: string;
  persona?: AgentPersona;
  messages: LayaMessage[];
  temperature?: number;
  format?: 'json';
  stream?: boolean;
  keep_alive?: string;
  contextData?: PersonalContextPayload;
  onChunk?: (chunk: LayaStreamChunk) => void;
}

export interface LayaStreamChunk {
  model: string;
  token: string;
  done: boolean;
  total_duration_ms?: number;
  prompt_eval_count?: number;
  eval_count?: number;
}

export interface LocalModelInfo {
  name: string;
  size_bytes: number;
  modified_at: string;
  parameter_size?: string;
  quantization_level?: string;
  family?: string;
}

export interface LayaToolDefinition {
  name: string;
  description: string;
  parametersSchema: Record<string, unknown>;
}

export type BenchmarkCriterion =
  | 'pure_tamil'
  | 'tanglish_code_switching'
  | 'natural_indian_english'
  | 'hindi_script'
  | 'stem_preservation'
  | 'profile_recall'
  | 'timetable_json'
  | 'balance_ai_rest';

export interface BenchmarkResult {
  id: string;
  criterion: BenchmarkCriterion;
  title: string;
  prompt: string;
  response: string;
  passed: boolean;
  score: number; // 0 to 100
  details: string;
  latencyMs: number;
}

export interface BenchmarkScoreCard {
  totalScore: number; // 0 to 100
  grade: 'A+' | 'A' | 'B' | 'C' | 'FAIL';
  averageLatencyMs: number;
  results: BenchmarkResult[];
  timestamp: string;
}
