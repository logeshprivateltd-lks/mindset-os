import type { MoodCheckin } from '@mindset/db-schema';

export interface FreeTimeWindow {
  startTime: string;
  durationMinutes: number;
  recommendedActivity: string;
}

export class BalanceEngine {
  static evaluateFreeTime(availableMinutes: number, currentMood?: MoodCheckin): FreeTimeWindow {
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

