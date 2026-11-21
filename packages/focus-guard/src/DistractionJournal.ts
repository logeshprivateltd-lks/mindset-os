/**
 * MindSet OS Focus Guard — Distraction Journal
 * Non-shaming behavioral tracking of interruptions and attention drift.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { DistractionEvent, DistractionStats } from './types';

export class DistractionJournal {
  private events: DistractionEvent[] = [];

  /**
   * Logs a distraction attempt or non-permitted window focus.
   */
  public log(sessionId: string, appName: string, wasBlocked: boolean): DistractionEvent {
    const id = typeof crypto !== 'undefined' && crypto.randomUUID
      ? crypto.randomUUID()
      : `dist_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;

    const event: DistractionEvent = {
      id,
      sessionId,
      appName,
      timestamp: Date.now(),
      wasBlocked,
    };

    this.events.push(event);
    return event;
  }

  /**
   * Marks when the student returned to the focus session.
   */
  public recordReturn(eventId: string): void {
    const ev = this.events.find((e) => e.id === eventId);
    if (ev && !ev.userReturnedMs) {
      ev.userReturnedMs = Math.max(0, Date.now() - ev.timestamp);
    }
  }

  /**
   * Retrieves all distraction events associated with a specific session.
   */
  public getForSession(sessionId: string): DistractionEvent[] {
    return this.events.filter((e) => e.sessionId === sessionId);
  }

  /**
   * Calculates gentle analytical statistics for reflection.
   */
  public getStats(sessionId: string): DistractionStats {
    const sessionEvents = this.getForSession(sessionId);
    const totalDistractions = sessionEvents.length;
    const blockedCount = sessionEvents.filter((e) => e.wasBlocked).length;

    // Calculate average return time
    const resolvedEvents = sessionEvents.filter((e) => typeof e.userReturnedMs === 'number');
    const averageReturnMs = resolvedEvents.length > 0
      ? Math.round(
          resolvedEvents.reduce((acc, curr) => acc + (curr.userReturnedMs || 0), 0) /
            resolvedEvents.length
        )
      : 0;

    // Identify top distracting app
    const countsByApp: Record<string, number> = {};
    for (const ev of sessionEvents) {
      countsByApp[ev.appName] = (countsByApp[ev.appName] || 0) + 1;
    }

    let topDistractingApp: string | null = null;
    let maxCount = 0;
    for (const [app, count] of Object.entries(countsByApp)) {
      if (count > maxCount) {
        maxCount = count;
        topDistractingApp = app;
      }
    }

    return {
      totalDistractions,
      blockedCount,
      averageReturnMs,
      topDistractingApp,
    };
  }

  /**
   * Clears internal journal state.
   */
  public clear(): void {
    this.events = [];
  }
}

