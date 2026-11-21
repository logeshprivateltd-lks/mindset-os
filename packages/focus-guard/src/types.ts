/**
 * MindSet OS Focus Guard — Types & Interfaces
 * Gentle, Non-Shaming Attention Protection & Focus Orchestration
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export interface FocusProfile {
  id: string;
  name: string;
  allowedApps: string[];
  blockedCategories: string[];
  strictMode: boolean;
  emergencyContactsAllowed: boolean;
}

export interface FocusSession {
  id: string;
  profileId: string;
  startTime: number;
  plannedDurationMs: number;
  actualEndTime?: number;
  status: 'active' | 'paused' | 'completed' | 'interrupted';
  pauseCount: number;
  interruptionCount: number;
  taskTitle: string;
  xpEarned?: number;
}

export interface DistractionEvent {
  id: string;
  sessionId: string;
  appName: string;
  timestamp: number;
  wasBlocked: boolean;
  userReturnedMs?: number;
}

export interface FocusGuardPolicy {
  mode: 'gentle' | 'moderate' | 'strict' | 'exam';
  notificationSuppression: boolean;
  ambientSoundEnabled: boolean;
  breakRemindersEnabled: boolean;
}

export interface NotificationGuardRule {
  appId: string;
  allow: boolean;
  alwaysAllowEmergency: boolean;
}

export interface XpMilestone {
  level: number;
  xpRequired: number;
  badge: string;
  rewardMessage: string;
}

export interface FocusProgress {
  elapsedMs: number;
  remainingMs: number;
  percentComplete: number;
  formattedElapsed: string;
  formattedRemaining: string;
}

export interface DistractionStats {
  totalDistractions: number;
  blockedCount: number;
  averageReturnMs: number;
  topDistractingApp: string | null;
}

export interface XpProgress {
  percent: number;
  xpNeeded: number;
  currentLevel: number;
  xpIntoLevel: number;
}

export interface XpUpdateResult {
  newTotal: number;
  newTotalXp: number;
  leveledUp: boolean;
  currentMilestone: XpMilestone;
  nextMilestone: XpMilestone | null;
}

