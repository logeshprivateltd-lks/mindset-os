/**
 * MindSet OS Balance Engine — Activity Recommender
 * Multi-dimensional recommendation engine matching Duration, Energy Tier, Mood, and Anti-Repetition.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import {
  EnergyTier,
  MoodState,
  RelaxationCategory,
  HobbyCategory,
  RecommendedActivity,
} from './types';

export interface RecommendationCriteria {
  availableMinutes: number;
  energyTier?: EnergyTier;
  mood?: MoodState;
  preferredCategory?: RelaxationCategory;
  recentActivityIds?: string[];
  isWindDown?: boolean;
}

export interface HobbyChallenge {
  id: string;
  category: HobbyCategory;
  title: string;
  durationMinutes: number;
  difficulty: 'Beginner' | 'Intermediate';
  prompt: string;
  materialsNeeded: string;
}

export const ACTIVITIES_DATABASE: RecommendedActivity[] = [
  // 1. MENTAL RELAXATION
  {
    id: 'act-mental-1',
    title: '4-7-8 Parasympathetic Breathwork',
    description: 'Inhale 4s, hold 7s, exhale 8s to calm the nervous system and lower cortisol.',
    category: 'mental',
    energyTier: 'LOW',
    suitableMoods: ['Stressed', 'Tired', 'Calm', 'Unmotivated'],
    minDurationMinutes: 5,
    maxDurationMinutes: 15,
    benefits: 'Vagus nerve stimulation, drops heart rate, reduces test anxiety',
    tags: ['breathwork', 'cortisol', 'calm'],
  },
  {
    id: 'act-mental-2',
    title: 'Box Breathing Focus Reset',
    description: '4s inhale, 4s hold, 4s exhale, 4s hold. Restores autonomic stability.',
    category: 'mental',
    energyTier: 'MEDIUM',
    suitableMoods: ['Stressed', 'Curious', 'Calm'],
    minDurationMinutes: 5,
    maxDurationMinutes: 12,
    benefits: 'Enhances cognitive clarity and emotional poise',
    tags: ['box-breathing', 'mindfulness'],
  },
  {
    id: 'act-mental-3',
    title: 'Warm Tea / Hydration Pause',
    description: 'Prepare a warm drink or herbal infusion slowly with zero screens.',
    category: 'mental',
    energyTier: 'VERY_LOW',
    suitableMoods: ['Tired', 'Stressed', 'Calm', 'Bored'],
    minDurationMinutes: 10,
    maxDurationMinutes: 20,
    benefits: 'Physical hydration and peaceful sensory grounding',
    tags: ['tea', 'sensory', 'rest'],
  },

  // 2. PHYSICAL RELAXATION & MOVEMENT
  {
    id: 'act-phys-1',
    title: 'Fresh Air Neighborhood Walk',
    description: 'Unstructured, unhurried walk outside with eyes scanning far horizons.',
    category: 'physical',
    hobbyCategory: 'outdoor',
    energyTier: 'MEDIUM',
    suitableMoods: ['Bored', 'Tired', 'Unmotivated', 'Happy', 'Calm'],
    minDurationMinutes: 15,
    maxDurationMinutes: 40,
    benefits: 'Relieves optic strain, circulates lymphatic fluid, stimulates dopamine',
    tags: ['walk', 'nature', 'circadian'],
  },
  {
    id: 'act-phys-2',
    title: 'Spine & Hamstring Restorative Stretch',
    description: 'Gentle full-body sequence opening chest, neck, and hip flexors.',
    category: 'physical',
    energyTier: 'LOW',
    suitableMoods: ['Tired', 'Stressed', 'Calm'],
    minDurationMinutes: 10,
    maxDurationMinutes: 25,
    benefits: 'Reverses desk slouching and improves spinal blood flow',
    tags: ['stretch', 'posture', 'yoga'],
  },
  {
    id: 'act-phys-3',
    title: 'Energizing Calisthenics / Movement Burst',
    description: 'Quick set of jumping jacks, bodyweight squats, or brisk skipping.',
    category: 'physical',
    hobbyCategory: 'sports',
    energyTier: 'HIGH',
    suitableMoods: ['Excited', 'Bored', 'Unmotivated', 'Happy'],
    minDurationMinutes: 10,
    maxDurationMinutes: 20,
    benefits: 'Surges endorphins and sharpens mental processing speed',
    tags: ['sports', 'cardio', 'burst'],
  },

  // 3. CREATIVE ACTIVITIES
  {
    id: 'act-creative-1',
    title: 'Geometric / Nature Pencil Sketching',
    description: 'Draw a real leaf, cup, or architectural shadow with simple graphite pencil.',
    category: 'creative',
    hobbyCategory: 'art',
    energyTier: 'MEDIUM',
    suitableMoods: ['Creative', 'Calm', 'Bored', 'Curious'],
    minDurationMinutes: 15,
    maxDurationMinutes: 45,
    benefits: 'Activates right hemisphere visual synthesis and fine motor mindfulness',
    tags: ['sketch', 'art', 'focus'],
  },
  {
    id: 'act-creative-2',
    title: 'Acoustic Guitar / Keyboard Fingerpicking',
    description: 'Play a gentle melody, ragam, or chord progression without pressure.',
    category: 'creative',
    hobbyCategory: 'music',
    energyTier: 'MEDIUM',
    suitableMoods: ['Creative', 'Happy', 'Calm'],
    minDurationMinutes: 15,
    maxDurationMinutes: 45,
    benefits: 'Dual-hemisphere musical engagement and emotional expression',
    tags: ['music', 'instrument', 'flow'],
  },
  {
    id: 'act-creative-3',
    title: 'Micro-Fiction / Creative Paragraph Writing',
    description: 'Write a 150-word story starting with: "The train had just left when..."',
    category: 'creative',
    hobbyCategory: 'writing',
    energyTier: 'HIGH',
    suitableMoods: ['Creative', 'Curious', 'Excited'],
    minDurationMinutes: 15,
    maxDurationMinutes: 30,
    benefits: 'Strengthens verbal imagination and metaphor construction',
    tags: ['writing', 'imagination'],
  },

  // 4. INTELLECTUAL ENTERTAINMENT
  {
    id: 'act-intellectual-1',
    title: 'James Webb Space Telescope Imagery Exploration',
    description: 'Inspect deep-field gravitational lensing and stellar nurseries.',
    category: 'intellectual',
    hobbyCategory: 'science',
    energyTier: 'LOW',
    suitableMoods: ['Curious', 'Calm', 'Bored'],
    minDurationMinutes: 15,
    maxDurationMinutes: 35,
    benefits: 'Inspires cosmic awe and healthy perspective on daily stressors',
    tags: ['space', 'astronomy', 'science'],
  },
  {
    id: 'act-intellectual-2',
    title: 'Tactical Chess Puzzle Session',
    description: 'Solve 3–5 interactive endgame or tactical combination puzzles.',
    category: 'intellectual',
    energyTier: 'MEDIUM',
    suitableMoods: ['Curious', 'Focused', 'Happy', 'Excited'] as any,
    minDurationMinutes: 10,
    maxDurationMinutes: 25,
    benefits: 'Pattern recognition exercise with zero academic pressure',
    tags: ['chess', 'puzzles', 'logic'],
  },

  // 5. SOCIAL & FAMILY
  {
    id: 'act-social-1',
    title: 'Family Tea or Kitchen Catch-Up',
    description: 'Help prepare an evening snack or chat with parents/grandparents about their day.',
    category: 'social',
    hobbyCategory: 'cooking',
    energyTier: 'MEDIUM',
    suitableMoods: ['Happy', 'Tired', 'Calm', 'Stressed'],
    minDurationMinutes: 15,
    maxDurationMinutes: 40,
    benefits: 'Deepens family bond and grounds attention in real-world human connection',
    tags: ['family', 'bonding', 'conversation'],
  },
  {
    id: 'act-social-2',
    title: 'Quick Call with a Supportive Friend',
    description: 'Voice call a friend just to laugh, check in, and share non-exam thoughts.',
    category: 'social',
    energyTier: 'MEDIUM',
    suitableMoods: ['Happy', 'Bored', 'Unmotivated', 'Excited'],
    minDurationMinutes: 10,
    maxDurationMinutes: 25,
    benefits: 'Oxytocin release and conversational decompression',
    tags: ['friends', 'call', 'connection'],
  },

  // 6. INTENTIONAL ENTERTAINMENT (FOCUS GUARD SYNCED)
  {
    id: 'act-ent-1',
    title: 'Curated 30-Minute Gaming Session',
    description: 'Play a planned, immersive indie or puzzle game with a pre-set timer.',
    category: 'entertainment',
    hobbyCategory: 'tech',
    energyTier: 'HIGH',
    suitableMoods: ['Excited', 'Bored', 'Happy'],
    minDurationMinutes: 20,
    maxDurationMinutes: 45,
    benefits: 'Intentional play without guilt; automatically whitelisted in Focus Guard',
    tags: ['gaming', 'budgeted', 'intentional'],
    isIntentionalEntertainment: true,
    appOrToolSuggestion: 'Steam / Chess Client / Retro Emulator',
  },
  {
    id: 'act-ent-2',
    title: 'Immersive Music Album Listening (No Multitasking)',
    description: 'Put on headphones and listen to a complete musical work from start to finish.',
    category: 'entertainment',
    hobbyCategory: 'music',
    energyTier: 'LOW',
    suitableMoods: ['Calm', 'Tired', 'Creative', 'Happy'],
    minDurationMinutes: 20,
    maxDurationMinutes: 45,
    benefits: 'Acoustic pleasure without algorithmic feed stimulation',
    tags: ['music', 'headphones', 'album'],
    isIntentionalEntertainment: true,
  },

  // 7. COMPLETE REST
  {
    id: 'act-rest-1',
    title: 'Zero-Output Pure Rest (Savasana)',
    description: 'Lie down flat on a mat or bed in a dim room. No phone, no audio, no goals.',
    category: 'rest',
    energyTier: 'VERY_LOW',
    suitableMoods: ['Tired', 'Stressed', 'Unmotivated'],
    minDurationMinutes: 10,
    maxDurationMinutes: 30,
    benefits: 'Consolidates short-term neural traces into long-term hippocampus storage',
    tags: ['rest', 'savasana', 'zero-output'],
  },
  {
    id: 'act-rest-2',
    title: 'Balcony / Window Cloud Watching',
    description: 'Sit by an open window watching natural light and weather changes.',
    category: 'rest',
    hobbyCategory: 'outdoor',
    energyTier: 'VERY_LOW',
    suitableMoods: ['Tired', 'Calm', 'Bored', 'Unmotivated'],
    minDurationMinutes: 10,
    maxDurationMinutes: 20,
    benefits: 'Allows mind wandering (default mode network) crucial for creative leaps',
    tags: ['clouds', 'nature', 'decompression'],
  },
];

export const HOBBY_CHALLENGES: HobbyChallenge[] = [
  {
    id: 'hob-art-1',
    category: 'art',
    title: 'Three-Shape Object Drawing',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Look at any item on your table. Draw it using only circles, rectangles, and triangles.',
    materialsNeeded: 'Pencil and plain paper',
  },
  {
    id: 'hob-tech-1',
    category: 'tech',
    title: 'Terminal Ascii Art Generator',
    durationMinutes: 15,
    difficulty: 'Beginner',
    prompt: 'Write a small script or open MindSet Terminal to print a clean geometric ASCII banner.',
    materialsNeeded: 'MindSet Terminal',
  },
  {
    id: 'hob-music-1',
    category: 'music',
    title: 'Rhythm Clapping on Odd Signatures',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Clap a 5/4 or 7/8 rhythm pattern (1-2, 1-2-3) consistently for 3 minutes.',
    materialsNeeded: 'Hands and a steady heartbeat',
  },
  {
    id: 'hob-sports-1',
    category: 'sports',
    title: 'Wall-Sit & Plank Endurance Test',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Do a 45-second wall sit followed by a 45-second forearm plank.',
    materialsNeeded: 'Clear wall space',
  },
  {
    id: 'hob-reading-1',
    category: 'reading',
    title: 'Historical Biography Dip',
    durationMinutes: 15,
    difficulty: 'Beginner',
    prompt: 'Read one short chapter from a non-academic biography (e.g. APJ Abdul Kalam / Feynman).',
    materialsNeeded: 'Any physical or digital book',
  },
  {
    id: 'hob-photo-1',
    category: 'photography',
    title: 'Macro Shadow & Texture Hunting',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Take 3 photos capturing intriguing shadows cast by window blinds or desk plants.',
    materialsNeeded: 'Phone camera or digital camera',
  },
  {
    id: 'hob-garden-1',
    category: 'gardening',
    title: 'Leaf Inspection & Soil Hydration',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Check the soil moisture of a potted plant, gently dust leaves, and mist with water.',
    materialsNeeded: 'Potted plant and small water cup',
  },
  {
    id: 'hob-cooking-1',
    category: 'cooking',
    title: 'Authentic Filter Coffee Pour',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Brew a fresh cup of coffee or herbal infusion with deliberate, calm attentiveness.',
    materialsNeeded: 'Coffee or tea supplies',
  },
  {
    id: 'hob-writing-1',
    category: 'writing',
    title: 'Six-Word Micro Story',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Compose three independent stories that each use exactly six words.',
    materialsNeeded: 'Notebook or MindSet Journal',
  },
  {
    id: 'hob-outdoor-1',
    category: 'outdoor',
    title: 'Local Bird / Tree Identification',
    durationMinutes: 15,
    difficulty: 'Beginner',
    prompt: 'Observe two distinct birds or trees visible from your home and note their distinguishing traits.',
    materialsNeeded: 'Open eyes and curiosity',
  },
  {
    id: 'hob-science-1',
    category: 'science',
    title: 'Optical Diffraction with a CD/DVD',
    durationMinutes: 10,
    difficulty: 'Beginner',
    prompt: 'Reflect flashlight or sunlight off an old optical disc to see clean spectral rainbows.',
    materialsNeeded: 'Old CD/DVD and light source',
  },
  {
    id: 'hob-diy-1',
    category: 'diy',
    title: 'Classic Origami Crane / Box',
    durationMinutes: 12,
    difficulty: 'Beginner',
    prompt: 'Fold a square piece of paper into a geometric crane or desk organizer box.',
    materialsNeeded: 'One square sheet of paper',
  },
];

export class ActivityRecommender {
  private activities: RecommendedActivity[];

  constructor(customActivities?: RecommendedActivity[]) {
    this.activities = customActivities ?? ACTIVITIES_DATABASE;
  }

  /**
   * Recommends balanced activities respecting duration, energy, mood, and anti-repetition
   */
  public recommendActivities(criteria: RecommendationCriteria): RecommendedActivity[] {
    const {
      availableMinutes,
      energyTier,
      mood,
      preferredCategory,
      recentActivityIds = [],
      isWindDown = false,
    } = criteria;

    let candidates = [...this.activities];

    // Filter duration: activity minDuration must fit within available time
    candidates = candidates.filter((a) => a.minDurationMinutes <= availableMinutes);

    // If evening wind-down, exclude HIGH energy stimulation or intense gaming
    if (isWindDown) {
      candidates = candidates.filter(
        (a) => a.energyTier !== 'HIGH' && a.category !== 'entertainment'
      );
    }

    // Preferred category filter
    if (preferredCategory) {
      const catMatches = candidates.filter((a) => a.category === preferredCategory);
      if (catMatches.length > 0) candidates = catMatches;
    }

    // Score candidates based on energy fit, mood match, and novelty
    const scored = candidates.map((activity) => {
      let score = 10;

      // Energy alignment
      if (energyTier) {
        if (activity.energyTier === energyTier) {
          score += 15;
        } else if (
          (energyTier === 'VERY_LOW' && activity.energyTier === 'LOW') ||
          (energyTier === 'HIGH' && activity.energyTier === 'MEDIUM')
        ) {
          score += 8;
        } else if (energyTier === 'VERY_LOW' && activity.energyTier === 'HIGH') {
          score -= 20; // Severe mismatch
        }
      }

      // Mood alignment
      if (mood && activity.suitableMoods.includes(mood)) {
        score += 12;
      }

      // Anti-repetition penalty
      if (recentActivityIds.includes(activity.id)) {
        score -= 25;
      }

      return { activity, score };
    });

    // Sort by descending score
    scored.sort((a, b) => b.score - a.score);

    return scored.map((s) => s.activity);
  }

  /**
   * Returns beginner hobby challenges
   */
  public getHobbyChallenges(category?: HobbyCategory): HobbyChallenge[] {
    if (category) {
      return HOBBY_CHALLENGES.filter((c) => c.category === category);
    }
    return HOBBY_CHALLENGES;
  }
}
