/**
 * MindSet OS — LivingProfileEngine
 * Consent-Driven Evolution: Detects milestones, routine drift, and emits confirmation cards.
 * STRICT RULE: Never mutates user_profile silently.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { UserProfile, TaskOrGoal, FocusSession, Habit } from '@mindset/db-schema';
import type { ProfileEvolutionSuggestion, RoutineSchedule, GoalItem } from './types';
import { PersonalContextBuilder } from './PersonalContextBuilder';

export class LivingProfileEngine {
  private pendingSuggestions: Map<string, ProfileEvolutionSuggestion> = new Map();

  /**
   * Retrieves all pending, un-actioned evolution suggestions
   */
  public getPendingSuggestions(): ProfileEvolutionSuggestion[] {
    return Array.from(this.pendingSuggestions.values()).filter(
      (s) => s.status === 'pending'
    );
  }

  /**
   * Clears or dismisses a suggestion without applying any changes
   */
  public dismissSuggestion(suggestionId: string): void {
    const existing = this.pendingSuggestions.get(suggestionId);
    if (existing) {
      existing.status = 'dismissed';
      this.pendingSuggestions.set(suggestionId, existing);
    }
  }

  /**
   * Evaluates recently completed tasks against active profile goals.
   * If a top goal is completed or key milestones are reached, creates an evolution suggestion.
   */
  public detectMilestoneEvolution(
    profile: UserProfile,
    completedTasks: TaskOrGoal[]
  ): ProfileEvolutionSuggestion | null {
    const p = PersonalContextBuilder.parseProfileMemory(profile);
    if (p.goals.length === 0) return null;

    for (const task of completedTasks) {
      // Check if task matches any top goal
      const matchedGoal = p.goals.find(
        (g) =>
          !g.completed &&
          (g.title.toLowerCase().includes(task.title.toLowerCase()) ||
            task.title.toLowerCase().includes(g.title.toLowerCase()) ||
            (task.parent_goal_id && task.parent_goal_id === g.id))
      );

      if (matchedGoal) {
        const suggestionId = `milestone-${matchedGoal.id}-${Date.now()}`;
        
        // Prepare updated goals where this goal is marked completed
        const updatedGoals: GoalItem[] = p.goals.map((g) =>
          g.id === matchedGoal.id ? { ...g, completed: true, progressPercent: 100 } : g
        );

        const suggestion: ProfileEvolutionSuggestion = {
          id: suggestionId,
          type: 'goal_completed',
          title: `Milestone Achieved: ${matchedGoal.title}`,
          reason: `You successfully completed the key task "${task.title}".`,
          promptMessage: `Fantastic achievement! You completed "${matchedGoal.title}". Would you like to mark this goal complete and archive it or define your next ambition?`,
          proposedChanges: {
            goals_json: JSON.stringify(updatedGoals),
            updated_at: new Date().toISOString(),
          },
          createdAt: new Date().toISOString(),
          status: 'pending',
        };

        this.pendingSuggestions.set(suggestionId, suggestion);
        return suggestion;
      }
    }

    return null;
  }

  /**
   * Analyzes recent focus sessions against the user's scheduled routine.
   * If the student consistently studies at a time different from their routine,
   * emits a non-intrusive suggestion to synchronize their schedule.
   */
  public detectRoutineDrift(
    profile: UserProfile,
    recentFocusSessions: FocusSession[]
  ): ProfileEvolutionSuggestion | null {
    if (recentFocusSessions.length < 3) return null;

    const p = PersonalContextBuilder.parseProfileMemory(profile);
    const routine = p.routine;

    // Analyze session start hours (0 - 23)
    const startHours = recentFocusSessions
      .map((s) => {
        try {
          const d = new Date(s.started_at);
          return isNaN(d.getTime()) ? null : d.getHours();
        } catch {
          return null;
        }
      })
      .filter((h): h is number => h !== null);

    if (startHours.length < 3) return null;

    // Check for consistent early morning sessions (before scheduled wake time)
    const wakeHour = parseInt(routine.wakeTime.split(':')[0] || '6', 10);
    const earlyMorningCount = startHours.filter((h) => h < wakeHour && h >= 4).length;

    if (earlyMorningCount >= 2) {
      const suggestionId = `routine-drift-early-${Date.now()}`;
      const proposedRoutine: RoutineSchedule = {
        ...routine,
        wakeTime: `${String(wakeHour - 1).padStart(2, '0')}:30`,
        preferredStudySlots: [
          {
            startTime: `${String(wakeHour - 1).padStart(2, '0')}:45`,
            endTime: `${String(wakeHour).padStart(2, '0')}:45`,
            label: 'Early Bird Focus',
          },
          ...routine.preferredStudySlots,
        ],
      };

      const suggestion: ProfileEvolutionSuggestion = {
        id: suggestionId,
        type: 'routine_drift',
        title: 'Early Focus Pattern Detected',
        reason: `You've completed ${earlyMorningCount} focus sessions before ${routine.wakeTime}.`,
        promptMessage: `We noticed you've been doing deep work early in the morning! Would you like to adjust your scheduled wake-up time to ${proposedRoutine.wakeTime} and add an Early Focus slot?`,
        proposedChanges: {
          routine_json: JSON.stringify(proposedRoutine),
          updated_at: new Date().toISOString(),
        },
        createdAt: new Date().toISOString(),
        status: 'pending',
      };

      this.pendingSuggestions.set(suggestionId, suggestion);
      return suggestion;
    }

    // Check for consistent evening/night sessions (after scheduled sleep time)
    const sleepHour = parseInt(routine.sleepTime.split(':')[0] || '22', 10);
    const lateNightCount = startHours.filter((h) => h >= sleepHour || h < 2).length;

    if (lateNightCount >= 2) {
      const suggestionId = `routine-drift-late-${Date.now()}`;
      const proposedRoutine: RoutineSchedule = {
        ...routine,
        sleepTime: `${String(Math.min(23, sleepHour + 1)).padStart(2, '0')}:30`,
      };

      const suggestion: ProfileEvolutionSuggestion = {
        id: suggestionId,
        type: 'routine_drift',
        title: 'Evening Focus Pattern Detected',
        reason: `You've completed ${lateNightCount} deep work sessions after your scheduled bedtime (${routine.sleepTime}).`,
        promptMessage: `Your focus sessions often extend past ${routine.sleepTime}. To keep your calendar realistic without sacrificing sleep hygiene, would you like to update your schedule?`,
        proposedChanges: {
          routine_json: JSON.stringify(proposedRoutine),
          updated_at: new Date().toISOString(),
        },
        createdAt: new Date().toISOString(),
        status: 'pending',
      };

      this.pendingSuggestions.set(suggestionId, suggestion);
      return suggestion;
    }

    return null;
  }

  /**
   * Detects notable habit streaks (e.g. 7-day, 21-day milestones)
   */
  public detectHabitMilestone(
    _profile: UserProfile,
    habits: Habit[]
  ): ProfileEvolutionSuggestion | null {
    for (const habit of habits) {
      if (habit.current_streak >= 7 && habit.current_streak % 7 === 0) {
        const suggestionId = `habit-streak-${habit.id}-${habit.current_streak}`;
        if (this.pendingSuggestions.has(suggestionId)) continue;

        const suggestion: ProfileEvolutionSuggestion = {
          id: suggestionId,
          type: 'habit_streak',
          title: `Streak Celebration: ${habit.title}`,
          reason: `${habit.current_streak}-day consistent streak!`,
          promptMessage: `Incredible dedication! You have maintained a ${habit.current_streak}-day streak on "${habit.title}". Keep up the momentum!`,
          proposedChanges: {},
          createdAt: new Date().toISOString(),
          status: 'pending',
        };

        this.pendingSuggestions.set(suggestionId, suggestion);
        return suggestion;
      }
    }

    return null;
  }

  /**
   * Applies an evolution suggestion ONLY upon explicit user approval.
   * If userApproved is false, the suggestion is discarded and user_profile is NOT modified.
   */
  public applySuggestion(
    profile: UserProfile,
    suggestion: ProfileEvolutionSuggestion,
    userApproved: boolean
  ): { updatedProfile: UserProfile; applied: boolean } {
    if (!userApproved) {
      suggestion.status = 'rejected';
      this.pendingSuggestions.set(suggestion.id, suggestion);
      return {
        updatedProfile: profile,
        applied: false,
      };
    }

    suggestion.status = 'accepted';
    this.pendingSuggestions.set(suggestion.id, suggestion);

    const updatedProfile: UserProfile = {
      ...profile,
      ...suggestion.proposedChanges,
      updated_at: new Date().toISOString(),
    };

    return {
      updatedProfile,
      applied: true,
    };
  }
}
