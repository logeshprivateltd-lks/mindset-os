/**
 * MindSet OS Focus Guard — XP & Gamification Engine
 * Celebrates student effort and steady habit formation without toxic competition.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { XpMilestone, XpProgress, XpUpdateResult } from './types';

export class XpEngine {
  public static readonly MILESTONES: XpMilestone[] = [
    {
      level: 1,
      xpRequired: 0,
      badge: 'Starter 🌱',
      rewardMessage: 'Every focused journey begins with a single calm step.',
    },
    {
      level: 2,
      xpRequired: 100,
      badge: 'Scholar 📚',
      rewardMessage: "You're building consistent daily focus habits!",
    },
    {
      level: 3,
      xpRequired: 300,
      badge: 'Achiever ⚡',
      rewardMessage: 'Consistency is your greatest superpower!',
    },
    {
      level: 4,
      xpRequired: 600,
      badge: 'Master 🧘',
      rewardMessage: 'Deep, uninterrupted focus comes naturally to you now.',
    },
    {
      level: 5,
      xpRequired: 1000,
      badge: 'Legend 🌟',
      rewardMessage: 'You have mastered intentional time and mindful focus!',
    },
  ];

  /**
   * Evaluates the student's current milestone based on accumulated XP.
   */
  public static getLevel(totalXp: number): XpMilestone {
    const milestones = [...XpEngine.MILESTONES].reverse();
    for (const milestone of milestones) {
      if (totalXp >= milestone.xpRequired) {
        return milestone;
      }
    }
    return XpEngine.MILESTONES[0]!;
  }

  /**
   * Retrieves the upcoming milestone to provide a clear, achievable goal.
   */
  public static getNextMilestone(totalXp: number): XpMilestone | null {
    for (const milestone of XpEngine.MILESTONES) {
      if (totalXp < milestone.xpRequired) {
        return milestone;
      }
    }
    return null; // Top level reached
  }

  /**
   * Calculates percentage progress towards the next milestone tier.
   */
  public static getProgressToNext(totalXp: number): XpProgress {
    const current = this.getLevel(totalXp);
    const next = this.getNextMilestone(totalXp);
    const progressInTier = Math.max(0, totalXp - current.xpRequired);

    if (!next) {
      return {
        percent: 100,
        xpNeeded: 0,
        currentLevel: current.level,
        xpIntoLevel: progressInTier,
      };
    }

    const tierRange = next.xpRequired - current.xpRequired;
    const percent = Math.min(100, Math.round((progressInTier / tierRange) * 100));
    const xpNeeded = Math.max(0, next.xpRequired - totalXp);

    return {
      percent,
      xpNeeded,
      currentLevel: current.level,
      xpIntoLevel: progressInTier,
    };
  }

  /**
   * Alias for getProgressToNext for intuitive access
   */
  public static getProgress(totalXp: number): XpProgress {
    return this.getProgressToNext(totalXp);
  }

  /**
   * Awards XP, checks for level-up transitions, and returns updated state.
   */
  public static addXp(currentTotal: number, earned: number): XpUpdateResult {
    const prevLevel = this.getLevel(currentTotal);
    const newTotal = Math.max(0, currentTotal + earned);
    const currentMilestone = this.getLevel(newTotal);
    const nextMilestone = this.getNextMilestone(newTotal);
    const leveledUp = currentMilestone.level > prevLevel.level;

    return {
      newTotal,
      newTotalXp: newTotal,
      leveledUp,
      currentMilestone,
      nextMilestone,
    };
  }

  /**
   * Alias for addXp
   */
  public static awardXp(currentTotal: number, earned: number): XpUpdateResult {
    return this.addXp(currentTotal, earned);
  }
}

