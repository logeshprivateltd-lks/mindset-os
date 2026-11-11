/**
 * MindSet OS Balance Engine
 * 9-Dimension Scheduling, Free-Time Detection, Energy/Mood Matching & Micro-Break Planning
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { MoodCheckin } from '@mindset/db-schema';

export * from './types';
export * from './FreeTimeDetector';
export * from './ActivityRecommender';
export * from './SmartFreeTimePlanner';

export interface LegacyFreeTimeWindow {
  startTime: string;
  durationMinutes: number;
  recommendedActivity: string;
}

export class BalanceEngine {
  static evaluateFreeTime(availableMinutes: number, currentMood?: MoodCheckin): LegacyFreeTimeWindow {
    let activity = 'Gentle walk or brief hydration break';
    
    if (currentMood) {
      if (currentMood.energy_tier === 'Low') {
        activity = '15-minute restorative power nap or progressive muscle relaxation';
      } else if (currentMood.energy_tier === 'Hyper') {
        activity = 'Brisk outdoor walk or quick physical exercise';
      } else if (currentMood.mood === 'Focused') {
        activity = 'Light review of textbook key concepts';
      }
    }

    return {
      startTime: new Date().toISOString(),
      durationMinutes: availableMinutes,
      recommendedActivity: activity,
    };
  }
}
