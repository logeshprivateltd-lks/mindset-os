/**
 * MindSet OS Balance Engine — Smart Free Time Planner
 * Generates optional, editable micro-break plans for detected free-time windows.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import {
  FreeTimeSlot,
  EnergyTier,
  MoodState,
  MicroBreakPlan,
  MicroBreakPhase,
} from './types';
import { ActivityRecommender } from './ActivityRecommender';

export class SmartFreeTimePlanner {
  private recommender: ActivityRecommender;

  constructor(recommender?: ActivityRecommender) {
    this.recommender = recommender ?? new ActivityRecommender();
  }

  /**
   * Generates a multi-phase micro-break plan adapted to the slot's usable duration
   */
  public generatePlan(
    slot: FreeTimeSlot,
    energyTier: EnergyTier = 'MEDIUM',
    mood: MoodState = 'Calm'
  ): MicroBreakPlan {
    const usable = Math.max(10, slot.usableMinutes);
    const phases: MicroBreakPhase[] = [];

    if (usable <= 20) {
      // 2-Phase Compact Reset
      phases.push({
        phaseIndex: 1,
        startMinute: 0,
        endMinute: 5,
        title: 'Step Away & Hydrate',
        description: 'Stand up, look away from screens, drink a glass of fresh water.',
        category: 'rest',
        icon: '💧',
      });
      phases.push({
        phaseIndex: 2,
        startMinute: 5,
        endMinute: usable,
        title: energyTier === 'HIGH' ? 'Brisk Stretch Burst' : 'Restorative Breathing',
        description: energyTier === 'HIGH'
          ? 'Full-body stretches, shoulder rolls, or quick hallway walk.'
          : '4-7-8 parasympathetic breathwork to clear mental clutter.',
        category: energyTier === 'HIGH' ? 'physical' : 'mental',
        icon: energyTier === 'HIGH' ? '🤸' : '🌿',
      });
    } else if (usable <= 45) {
      // 3-Phase Balanced Pause
      const decompressEnd = 8;
      const mainEnd = usable - 7;

      phases.push({
        phaseIndex: 1,
        startMinute: 0,
        endMinute: decompressEnd,
        title: 'Sensory Decompression',
        description: 'Step away from all displays, gently wash face or sip tea.',
        category: 'rest',
        icon: '☕',
      });

      // Pick top recommended activity for the main phase
      const topRec = this.recommender.recommendActivities({
        availableMinutes: mainEnd - decompressEnd,
        energyTier,
        mood,
        isWindDown: slot.isWindDownWindow,
      })[0];

      phases.push({
        phaseIndex: 2,
        startMinute: decompressEnd,
        endMinute: mainEnd,
        title: topRec ? topRec.title : 'Outdoor Walk & Sunshine',
        description: topRec
          ? topRec.description
          : 'Unhurried neighborhood walk without checking notifications.',
        category: topRec ? topRec.category : 'physical',
        icon: topRec?.category === 'creative' ? '🎨' : topRec?.category === 'intellectual' ? '🔭' : '🚶',
      });

      phases.push({
        phaseIndex: 3,
        startMinute: mainEnd,
        endMinute: usable,
        title: 'Gentle Return',
        description: 'Take 3 deep breaths, organize desk calmly, and transition with zero rush.',
        category: 'mental',
        icon: '🌱',
      });
    } else {
      // 4-Phase Full Restoration (60–90 min)
      const p1End = 10;
      const p2End = Math.floor(usable * 0.55);
      const p3End = usable - 8;

      phases.push({
        phaseIndex: 1,
        startMinute: 0,
        endMinute: p1End,
        title: 'Screen-Free Decompression',
        description: 'Physical eye rest, drink water, gentle neck and shoulder releases.',
        category: 'rest',
        icon: '💆',
      });

      const recs = this.recommender.recommendActivities({
        availableMinutes: p2End - p1End,
        energyTier,
        mood,
        isWindDown: slot.isWindDownWindow,
      });
      const mainRec = recs[0];
      const secondRec = recs[1];

      phases.push({
        phaseIndex: 2,
        startMinute: p1End,
        endMinute: p2End,
        title: mainRec ? mainRec.title : 'Outdoor Movement & Nature',
        description: mainRec
          ? mainRec.description
          : 'Unhurried walk or light cycling to oxygenate cerebral capillaries.',
        category: mainRec ? mainRec.category : 'physical',
        icon: '🌳',
      });

      phases.push({
        phaseIndex: 3,
        startMinute: p2End,
        endMinute: p3End,
        title: secondRec ? secondRec.title : 'Creative / Hobby Exploration',
        description: secondRec
          ? secondRec.description
          : 'Pencil sketching, acoustic music, or reading a non-academic chapter.',
        category: secondRec ? secondRec.category : 'creative',
        icon: '🎨',
      });

      phases.push({
        phaseIndex: 4,
        startMinute: p3End,
        endMinute: usable,
        title: 'Calm Transition Reset',
        description: 'Review the next focus target calmly with zero panic or hurry.',
        category: 'mental',
        icon: '🧘',
      });
    }

    return {
      id: `plan-${Date.now()}`,
      slotStartTime: slot.startTime,
      totalMinutes: usable,
      title: `${usable}-Minute MindSet Recharge Plan`,
      phases,
      createdAt: new Date().toISOString(),
      isAccepted: false,
    };
  }
}

