/**
 * MindSet OS Balance Engine — Types & Domain Models
 * 9-Dimension Balanced Scheduling, Energy Tiers, Mood States & Free-Time Definitions
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export type LifeDimension =
  | 'responsibilities' // School, classes, mandatory work (08:00–17:00)
  | 'goals'            // Exam milestones, Board exam prep (590/600)
  | 'learning'         // Self-directed learning, extra coding, science
  | 'physical'         // Outdoor movement, sports, walking, yoga
  | 'relaxation'       // Breathwork, meditation, quiet tea
  | 'entertainment'    // Intentional gaming, planned films, music
  | 'social'           // Family dinner, conversations, friends
  | 'unplanned'        // Protected spontaneous daily buffer (30–60m)
  | 'sleep';           // Night recovery (22:00–05:30)

export type EnergyTier = 'HIGH' | 'MEDIUM' | 'LOW' | 'VERY_LOW';

export type MoodState =
  | 'Happy'
  | 'Calm'
  | 'Excited'
  | 'Bored'
  | 'Tired'
  | 'Stressed'
  | 'Unmotivated'
  | 'Curious'
  | 'Creative';

export type RelaxationCategory =
  | 'mental'        // Guided breathwork, gratitude, quiet rest
  | 'physical'      // Walking, stretching, cycling
  | 'creative'      // Sketching, origami, writing, music
  | 'intellectual'  // Puzzles, astronomy, chess, science docs
  | 'social'        // Family time, dinner, calling friends
  | 'entertainment' // Planned movie, budgeted gaming, music
  | 'rest';         // Screen-free complete rest without guilt

export type HobbyCategory =
  | 'art'
  | 'tech'
  | 'music'
  | 'sports'
  | 'reading'
  | 'photography'
  | 'gardening'
  | 'cooking'
  | 'writing'
  | 'outdoor'
  | 'science'
  | 'diy';

export interface ScheduledBlock {
  id: string;
  title: string;
  startTime: string; // "HH:MM" 24h format
  endTime: string;   // "HH:MM" 24h format
  dimension?: LifeDimension;
  isMandatory?: boolean;
}

export interface FreeTimeSlot {
  startTime: string; // "HH:MM"
  endTime: string;   // "HH:MM"
  totalMinutes: number;
  usableMinutes: number;   // totalMinutes - bufferMinutes
  bufferMinutes: number;   // Protected unplanned buffer
  isWindDownWindow: boolean; // Overlaps 21:30-22:00 evening wind-down
}

export interface RecommendedActivity {
  id: string;
  title: string;
  description: string;
  category: RelaxationCategory;
  hobbyCategory?: HobbyCategory;
  energyTier: EnergyTier;
  suitableMoods: MoodState[];
  minDurationMinutes: number;
  maxDurationMinutes: number;
  benefits: string;
  tags: string[];
  isIntentionalEntertainment?: boolean;
  appOrToolSuggestion?: string;
}

export interface MicroBreakPhase {
  phaseIndex: number;
  startMinute: number;
  endMinute: number;
  title: string;
  description: string;
  category: RelaxationCategory;
  icon: string;
}

export interface MicroBreakPlan {
  id: string;
  slotStartTime: string;
  totalMinutes: number;
  title: string;
  phases: MicroBreakPhase[];
  createdAt: string;
  isAccepted?: boolean;
}

