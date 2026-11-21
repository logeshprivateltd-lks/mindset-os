/**
 * MindSet OS Focus Guard — Focus Session Manager
 * Orchestrates immersive focus states with non-shaming discipline.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { FocusSession, FocusProgress } from './types';

export class FocusSessionManager {
  /**
   * Starts a new immersive focus session.
   */
  public startSession(
    taskTitle: string,
    plannedDurationMs: number,
    profileId: string = 'default'
  ): FocusSession {
    const id = typeof crypto !== 'undefined' && crypto.randomUUID
      ? crypto.randomUUID()
      : `session_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;

    return {
      id,
      profileId,
      startTime: Date.now(),
      plannedDurationMs,
      status: 'active',
      pauseCount: 0,
      interruptionCount: 0,
      taskTitle,
      xpEarned: 0,
    };
  }

  /**
   * Pauses an active session and increments pause counter.
   */
  public pauseSession(session: FocusSession): FocusSession {
    if (session.status !== 'active') return session;
    return {
      ...session,
      status: 'paused',
      pauseCount: session.pauseCount + 1,
    };
  }

  /**
   * Resumes a paused session.
   */
  public resumeSession(session: FocusSession): FocusSession {
    if (session.status !== 'paused') return session;
    return {
      ...session,
      status: 'active',
    };
  }

  /**
   * Ends a focus session and computes earned XP.
   * Rule: 1 XP per full minute completed.
   * Bonus: +20 XP if 100% completed without interruptions.
   */
  public endSession(
    session: FocusSession,
    wasInterrupted: boolean = false,
    endTimeMs?: number
  ): FocusSession {
    const now = endTimeMs ?? Date.now();
    const elapsedMs = Math.max(0, now - session.startTime);
    const completedMinutes = Math.floor(elapsedMs / (60 * 1000));
    const isFullCompletion = elapsedMs >= session.plannedDurationMs && !wasInterrupted;

    let xpEarned = completedMinutes;
    if (isFullCompletion && session.interruptionCount === 0) {
      xpEarned += 20; // Flow state perfection bonus
    }

    return {
      ...session,
      actualEndTime: now,
      status: wasInterrupted ? 'interrupted' : 'completed',
      xpEarned,
    };
  }

  /**
   * Computes progress metrics and tabular countdown values.
   */
  public calculateProgress(session: FocusSession, currentTimeMs?: number): FocusProgress {
    const now = session.actualEndTime ?? currentTimeMs ?? Date.now();
    const elapsedMs = Math.max(0, now - session.startTime);
    const remainingMs = Math.max(0, session.plannedDurationMs - elapsedMs);
    const rawPercent = (elapsedMs / Math.max(1, session.plannedDurationMs)) * 100;
    const percentComplete = Math.min(100, Math.max(0, Math.round(rawPercent)));

    return {
      elapsedMs,
      remainingMs,
      percentComplete,
      formattedElapsed: this.formatDuration(elapsedMs),
      formattedRemaining: this.formatDuration(remainingMs),
    };
  }

  /**
   * Formats milliseconds into standard MM:SS or HH:MM:SS string
   */
  public formatDuration(ms: number): string {
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    const pad = (n: number) => n.toString().padStart(2, '0');
    if (hours > 0) {
      return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
    }
    return `${pad(minutes)}:${pad(seconds)}`;
  }
}

