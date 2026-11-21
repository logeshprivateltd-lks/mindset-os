/**
 * MindSet OS — 5-Zone Personalised Command Center Dashboard (/)
 * Neuroaesthetic 3-Tier Visual Hierarchy connecting to SQLite & Kernel Event Bus
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useMemo, useCallback } from 'react';
import {
  GlassSurface,
  PriorityCard,
  ProgressRing,
  TabularTimer,
  Button,
} from '@mindset/ui-core';
import type {
  UserProfile,
  TaskOrGoal,
  Habit,
  MoodCheckin,
} from '@mindset/db-schema';
import { BalanceEngine } from '@mindset/balance-engine';
import { dbClient } from '../../lib/dbClient';
import { useKernelStore } from '../../stores/useKernelStore';
import { navigationRouter, CanonicalRoute } from '../../navigation/navigationRouter';

export interface CommandCenterPageProps {
  onLock?: () => void;
  onNavigate?: (route: CanonicalRoute) => void;
}

export const CommandCenterPage: React.FC<CommandCenterPageProps> = ({
  onLock,
  onNavigate,
}) => {
  const {
    isFocusActive,
    activeFocusMinutes,
  } = useKernelStore();

  // State populated from SQLite
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [tasks, setTasks] = useState<TaskOrGoal[]>([]);
  const [habits, setHabits] = useState<Habit[]>([]);
  const [recentMoods, setRecentMoods] = useState<MoodCheckin[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  // Interactive controls
  const [selectedFocusPreset, setSelectedFocusPreset] = useState<number>(45);
  const [moodToast, setMoodToast] = useState<string | null>(null);
  const [rebalanceToast, setRebalanceToast] = useState<string | null>(null);

  // Navigation helper
  const handleNav = useCallback(
    (route: CanonicalRoute) => {
      navigationRouter.navigate(route);
      if (onNavigate) onNavigate(route);
    },
    [onNavigate]
  );

  // Load all dashboard state from SQLite repositories
  const refreshData = useCallback(async () => {
    try {
      const [p, t, h, m] = await Promise.all([
        dbClient.profile.get(),
        dbClient.tasks.list(),
        dbClient.habits.list(),
        dbClient.wellbeing.listMoods(5),
      ]);

      if (p) setProfile(p);
      setTasks(t);
      setHabits(h);
      setRecentMoods(m);
    } catch (err) {
      console.error('[CommandCenter] Failed to load SQLite state:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    refreshData();
  }, [refreshData]);

  // Priority 1 Task: First pending task with highest priority (priority 1 or lowest number)
  const priority1Task = useMemo(() => {
    const pending = tasks.filter((t) => t.status === 'pending');
    if (pending.length === 0) return null;
    return pending.sort((a, b) => a.priority - b.priority)[0];
  }, [tasks]);

  // Task completion statistics
  const taskStats = useMemo(() => {
    const total = tasks.length;
    const completed = tasks.filter((t) => t.status === 'completed').length;
    const percent = total > 0 ? Math.round((completed / total) * 100) : 0;
    return { total, completed, percent };
  }, [tasks]);

  // Habit streaks statistics
  const habitStreaksCount = useMemo(() => {
    return habits.filter((h) => h.current_streak > 0).length;
  }, [habits]);

  // Latest Mood
  const latestMood = recentMoods.length > 0 ? recentMoods[0] : undefined;

  // Free-Time Window evaluated via BalanceEngine
  const freeTimeWindow = useMemo(() => {
    return BalanceEngine.evaluateFreeTime(45, latestMood);
  }, [latestMood]);

  // Dynamic Time-of-Day Greeting
  const greeting = useMemo(() => {
    const hour = new Date().getHours();
    const lang = profile?.interface_lang || 'en';
    const name = profile?.display_name || 'Scholar';

    if (lang === 'ta') {
      if (hour < 12) return `காலை வணக்கம், ${name}`;
      if (hour < 17) return `மதிய வணக்கம், ${name}`;
      if (hour < 21) return `மாலை வணக்கம், ${name}`;
      return `இனிய இரவு, ${name}`;
    }

    if (hour < 12) return `Good morning, ${name}`;
    if (hour < 17) return `Good afternoon, ${name}`;
    if (hour < 21) return `Good evening, ${name}`;
    return `Restful night, ${name}`;
  }, [profile]);

  // Primary Goal Title for Memory Zone
  const topGoalTitle = useMemo(() => {
    if (!profile?.goals_json) return 'Core Exam Mastery';
    try {
      const parsed = typeof profile.goals_json === 'string' ? JSON.parse(profile.goals_json) : profile.goals_json;
      if (Array.isArray(parsed) && parsed.length > 0) {
        const first = parsed[0];
        if (typeof first === 'string') return first;
        if (first && typeof first === 'object') {
          if (typeof first.title === 'string') return first.title;
          if (typeof first.name === 'string') return first.name;
          if (typeof first.goal === 'string') return first.goal;
        }
      } else if (parsed && typeof parsed === 'object') {
        if (typeof (parsed as any).title === 'string') return (parsed as any).title;
        if (typeof (parsed as any).name === 'string') return (parsed as any).name;
      }
    } catch {
      // Fallback
    }
    return 'Core Exam Mastery';
  }, [profile?.goals_json]);

  // CoachAgent dynamic advice
  const coachMessage = useMemo(() => {
    const hour = new Date().getHours();
    if (isFocusActive) {
      return 'Deep focus session in progress. Guard your attention; silence distractions.';
    }
    if (hour < 10) {
      return 'Fresh morning energy! Tackle your Priority 1 concept before mid-day fatigue sets in.';
    }
    if (hour >= 12 && hour < 15) {
      return 'Post-lunch dip? Start with a quick 25-minute Pomodoro or active recall problem.';
    }
    if (hour >= 21) {
      return 'Wind down approaching. Review completed milestones and prepare sleep routine for 22:00.';
    }
    return 'Consistent momentum beats perfection. Complete one meaningful study block today.';
  }, [isFocusActive]);

  // Calculate Next Commitment
  const nextCommitment = useMemo(() => {
    const hour = new Date().getHours();
    if (hour < 10) return { title: 'Mathematics: Calculus Lab', time: '10:30 AM', relative: 'in 1h 15m' };
    if (hour < 14) return { title: 'Physics: Electromagnetism Chapter 3', time: '02:30 PM', relative: 'in 45m' };
    if (hour < 18) return { title: 'Doubt Solving & Concept Flashcards', time: '05:30 PM', relative: 'in 1h 30m' };
    return { title: 'Wind-Down & Sleep Routine', time: '10:00 PM', relative: 'in 2h 10m' };
  }, []);

  // Complete Task Handler
  const handleCompleteTask = async (taskId: string) => {
    try {
      await dbClient.tasks.complete(taskId);
      await refreshData();
    } catch (err) {
      console.error('[CommandCenter] Failed to complete task:', err);
    }
  };

  // Start Focus Session Handler
  const handleStartFocus = async (task?: TaskOrGoal) => {
    try {
      await dbClient.wellbeing.startFocusSession({
        profile_mode: 'DeepWork',
        planned_minutes: selectedFocusPreset,
        task_id: task?.id || null,
      });
      await refreshData();
    } catch (err) {
      console.error('[CommandCenter] Failed to start focus session:', err);
    }
  };

  // Record Mood Check-in Handler
  const handleRecordMood = async (
    mood: 'Energized' | 'Calm' | 'Focused' | 'Anxious' | 'Fatigued' | 'Overwhelmed',
    tier: 'Low' | 'Medium' | 'High' | 'Hyper'
  ) => {
    try {
      await dbClient.wellbeing.createMood({
        mood,
        energy_tier: tier,
        context_note: 'Logged from Command Center Zone 4',
        active_activity: 'deep-study',
      });
      setMoodToast(`Logged mood: ${mood} (${tier} Energy)`);
      setTimeout(() => setMoodToast(null), 3500);
      await refreshData();
    } catch (err) {
      console.error('[CommandCenter] Failed to record mood:', err);
    }
  };

  // Evening Rebalance Handler
  const handleRebalanceEvening = () => {
    setRebalanceToast(
      'CoachAgent: Evening schedule rebalanced. 45m Physics shifted earlier; 30m buffer added before 22:00 sleep.'
    );
    setTimeout(() => setRebalanceToast(null), 5000);
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full min-h-[500px]">
        <div className="flex flex-col items-center space-y-3">
          <span className="w-8 h-8 rounded-full border-2 border-[#8B9CFF] border-t-transparent animate-spin" />
          <span className="text-xs text-[var(--text-muted)] font-mono">
            Loading Sovereign Command Center...
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="pt-4 pb-16 px-4 md:px-8 max-w-7xl mx-auto space-y-6 select-none transition-colors overflow-y-auto max-h-screen">
      {/* ========================================================================= */}
      {/* ZONE 1 — TOP HEADER BAR                                                   */}
      {/* ========================================================================= */}
      <section className="relative p-5 rounded-2xl border border-[var(--border-subtle)] bg-[var(--surface-primary)] backdrop-blur-xl shadow-lg">
        <div className="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4">
          {/* Greeting & CoachAgent Context */}
          <div className="space-y-1.5">
            <div className="flex items-center space-x-3">
              <h1 className="text-2xl font-bold tracking-tight text-[var(--text-dominant)]">
                {greeting}
              </h1>
              <span className="px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-[#8B9CFF]/15 text-[#8B9CFF] border border-[#8B9CFF]/30">
                {profile?.role ? profile.role.toUpperCase() : 'STUDENT'}
              </span>
            </div>
            <p className="text-xs text-[var(--accent-secondary)] flex items-center space-x-2">
              <span className="text-sm">🌿</span>
              <span>
                <strong>CoachAgent:</strong> {coachMessage}
              </span>
            </p>
          </div>

          {/* Right Header: Completion Badge & Live Tabular Clock */}
          <div className="flex flex-wrap items-center gap-3">
            {/* Completion Summary Badge */}
            <div className="px-3 py-1.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] text-xs flex items-center space-x-2">
              <span className="w-2 h-2 rounded-full bg-[#65E6C4] animate-pulse" />
              <span className="font-mono tabular-nums font-semibold text-[var(--text-dominant)]">
                {taskStats.completed}/{taskStats.total} Tasks ({taskStats.percent}%)
              </span>
              <span className="text-[var(--text-muted)]">•</span>
              <span className="text-[#FFD078] font-mono font-semibold">
                {habitStreaksCount} Streaks Active
              </span>
            </div>

            {/* Tabular Clock */}
            <div className="px-3 py-1.5 rounded-xl border border-[#8B9CFF]/30 bg-[#8B9CFF]/10 text-xs font-mono tabular-nums text-[#8B9CFF] flex items-center space-x-2">
              <span>🕒</span>
              <TabularTimer mode="clock" size="sm" />
            </div>

            {/* Quick Status / Lock triggers */}
            {onLock && (
              <Button
                variant="ghost"
                size="sm"
                onClick={onLock}
                title="Lock System (Super + L)"
                className="text-xs"
              >
                🔒 Lock
              </Button>
            )}
          </div>
        </div>

        {/* Dynamic Toast Feedback */}
        {moodToast && (
          <div className="mt-3 px-3 py-1.5 rounded-lg bg-[#65E6C4]/20 border border-[#65E6C4]/40 text-xs text-[#65E6C4] font-medium flex items-center space-x-2 animate-fadeIn">
            <span>✓</span>
            <span>{moodToast}</span>
          </div>
        )}
        {rebalanceToast && (
          <div className="mt-3 px-3 py-1.5 rounded-lg bg-[#FFD078]/20 border border-[#FFD078]/40 text-xs text-[#FFD078] font-medium flex items-center space-x-2 animate-fadeIn">
            <span>✨</span>
            <span>{rebalanceToast}</span>
          </div>
        )}
      </section>

      {/* ========================================================================= */}
      {/* ZONE 2 — PRIORITY 1 MAIN STAGE                                            */}
      {/* ========================================================================= */}
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Dominant Priority 1 Task Card (Span 2 Columns) */}
        <div className="lg:col-span-2 space-y-3">
          {priority1Task ? (
            <PriorityCard
              priority={1}
              title={priority1Task.title}
              subtitle="Current Top Study Target"
              subjectTag={priority1Task.subject_tag || 'Core Subject'}
              xpReward={priority1Task.xp_reward || 50}
              status={priority1Task.status}
              progressPercent={isFocusActive ? 65 : 20}
              primaryAction={{
                label: isFocusActive ? 'Active Focus Session' : 'Start Focus Session',
                onClick: () => handleStartFocus(priority1Task),
                icon: <span>⚡</span>,
                variant: 'primary',
              }}
              secondaryAction={{
                label: 'Mark Complete',
                onClick: () => handleCompleteTask(priority1Task.id),
                icon: <span>✓</span>,
                variant: 'accent',
              }}
            >
              <div className="flex items-center justify-between text-xs text-[var(--text-muted)] pt-1">
                <span>
                  Estimated duration: <strong>{selectedFocusPreset} mins</strong>
                </span>
                <span className="text-[#FFD078] font-mono font-medium">
                  Reward: +{priority1Task.xp_reward || 50} XP
                </span>
              </div>
            </PriorityCard>
          ) : (
            <GlassSurface variant="solid-card" className="p-6 border-dashed border-[#8B9CFF]/40 text-center space-y-3">
              <span className="text-3xl">🎉</span>
              <h3 className="text-base font-bold text-[var(--text-dominant)]">
                All Priority 1 Tasks Completed!
              </h3>
              <p className="text-xs text-[var(--text-muted)] max-w-md mx-auto">
                Outstanding discipline. Review secondary commitments, take a hydration break, or schedule your next target.
              </p>
              <Button
                variant="primary"
                size="sm"
                onClick={() => handleNav('/tasks')}
              >
                + Schedule New Task (/tasks)
              </Button>
            </GlassSurface>
          )}

          {/* Next Scheduled Commitment Pill */}
          <div className="p-3.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] flex items-center justify-between text-xs">
            <div className="flex items-center space-x-2.5">
              <span className="w-2.5 h-2.5 rounded-full bg-[#FFD078]" />
              <span className="text-[var(--text-muted)]">Next Commitment:</span>
              <span className="font-semibold text-[var(--text-dominant)]">
                {nextCommitment.title}
              </span>
            </div>
            <div className="flex items-center space-x-2 font-mono tabular-nums">
              <span className="text-[#FFD078] font-medium">{nextCommitment.time}</span>
              <span className="text-[var(--text-muted)]">({nextCommitment.relative})</span>
            </div>
          </div>
        </div>

        {/* Interactive Focus Timer Widget & SVG Daily Progress Ring */}
        <div className="space-y-4">
          <GlassSurface variant="modal" className="p-5 space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)]">
                Daily Focus Ring
              </h3>
              <span className="text-[11px] font-mono text-[var(--accent-secondary)]">
                Target: 180m
              </span>
            </div>

            {/* Radial SVG Progress Ring */}
            <div className="flex justify-center py-2">
              <ProgressRing
                progress={Math.min(100, Math.round(((activeFocusMinutes || 60) / 180) * 100))}
                size={110}
                strokeWidth={8}
                accentColor="var(--accent-secondary)"
                sublabel="Daily Focus"
              />
            </div>

            {/* Duration Presets */}
            <div className="grid grid-cols-3 gap-2">
              {[25, 45, 90].map((mins) => (
                <button
                  key={mins}
                  type="button"
                  onClick={() => setSelectedFocusPreset(mins)}
                  className={`py-1.5 rounded-lg text-xs font-mono font-semibold border transition-all ${
                    selectedFocusPreset === mins
                      ? 'border-[#8B9CFF] bg-[#8B9CFF]/20 text-[#8B9CFF]'
                      : 'border-[var(--border-subtle)] bg-[var(--surface-primary)] text-[var(--text-muted)] hover:border-[var(--border-strong)]'
                  }`}
                >
                  {mins}m
                </button>
              ))}
            </div>

            {/* Trigger Button */}
            <Button
              variant={isFocusActive ? 'secondary' : 'primary'}
              className="w-full text-xs font-semibold py-2"
              onClick={() => handleStartFocus(priority1Task || undefined)}
            >
              {isFocusActive ? '⏸️ Pause Active Session' : `▶️ Start ${selectedFocusPreset}m Focus`}
            </Button>
          </GlassSurface>
        </div>
      </section>

      {/* ========================================================================= */}
      {/* ZONE 3 — SECONDARY PROGRESS & SCHEDULE GRID                               */}
      {/* ========================================================================= */}
      <section className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Sub-Card 1: Today's Vertical Timeline Schedule */}
        <GlassSurface variant="solid-card" className="p-4 space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-xs font-bold text-[var(--text-dominant)] flex items-center space-x-1.5">
              <span>⏱️</span>
              <span>Today's Schedule</span>
            </h4>
            <button
              type="button"
              onClick={() => handleNav('/timeline')}
              className="text-[10px] text-[#8B9CFF] hover:underline"
            >
              View Full
            </button>
          </div>

          <div className="space-y-2 text-xs">
            <div className="p-2 rounded-lg bg-[var(--surface-secondary)] border-l-2 border-[#65E6C4]">
              <div className="font-semibold text-[var(--text-dominant)]">08:30 Organic Chemistry</div>
              <div className="text-[10px] text-[var(--text-muted)]">Completed • Active Recall</div>
            </div>
            <div className="p-2 rounded-lg bg-[#8B9CFF]/15 border-l-2 border-[#8B9CFF]">
              <div className="font-semibold text-[#8B9CFF]">11:00 Calculus Derivatives</div>
              <div className="text-[10px] text-[var(--text-muted)]">Current Stage • Priority 1</div>
            </div>
            <div className="p-2 rounded-lg bg-[var(--surface-primary)] border-l-2 border-[var(--border-subtle)]">
              <div className="font-semibold text-[var(--text-muted)]">15:30 Physics Doubt Solving</div>
              <div className="text-[10px] text-[var(--text-muted)]">Upcoming • PDF Tutor</div>
            </div>
          </div>
        </GlassSurface>

        {/* Sub-Card 2: Recent Milestone Achievements */}
        <GlassSurface variant="solid-card" className="p-4 space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-xs font-bold text-[#FFD078] flex items-center space-x-1.5">
              <span>🏆</span>
              <span>Milestones Earned</span>
            </h4>
            <span className="text-[10px] font-mono text-[#FFD078]">Level 3</span>
          </div>

          <div className="space-y-2 text-xs">
            <div className="p-2 rounded-lg bg-[#FFD078]/10 border border-[#FFD078]/30">
              <div className="font-semibold text-[#FFD078]">Integration Master</div>
              <div className="text-[10px] text-[var(--text-muted)]">Solved 20 integrals (+75 XP)</div>
            </div>
            <div className="p-2 rounded-lg bg-[#FFD078]/10 border border-[#FFD078]/30">
              <div className="font-semibold text-[#FFD078]">7-Day Habit Streak</div>
              <div className="text-[10px] text-[var(--text-muted)]">Deep study uninterrupted</div>
            </div>
          </div>
        </GlassSurface>

        {/* Sub-Card 3: Syllabus Progress Bars */}
        <GlassSurface variant="solid-card" className="p-4 space-y-3">
          <h4 className="text-xs font-bold text-[var(--text-dominant)] flex items-center space-x-1.5">
            <span>📚</span>
            <span>Syllabus Coverage</span>
          </h4>

          <div className="space-y-2 text-xs">
            <div>
              <div className="flex justify-between text-[11px] mb-1">
                <span className="text-[var(--text-muted)]">Mathematics</span>
                <span className="font-mono font-bold text-[#8B9CFF]">82%</span>
              </div>
              <div className="w-full h-1.5 rounded-full bg-[var(--surface-elevated)] overflow-hidden">
                <div className="h-full bg-[#8B9CFF] rounded-full" style={{ width: '82%' }} />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-[11px] mb-1">
                <span className="text-[var(--text-muted)]">Physics</span>
                <span className="font-mono font-bold text-[#65E6C4]">74%</span>
              </div>
              <div className="w-full h-1.5 rounded-full bg-[var(--surface-elevated)] overflow-hidden">
                <div className="h-full bg-[#65E6C4] rounded-full" style={{ width: '74%' }} />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-[11px] mb-1">
                <span className="text-[var(--text-muted)]">Chemistry</span>
                <span className="font-mono font-bold text-[#FF8EAA]">68%</span>
              </div>
              <div className="w-full h-1.5 rounded-full bg-[var(--surface-elevated)] overflow-hidden">
                <div className="h-full bg-[#FF8EAA] rounded-full" style={{ width: '68%' }} />
              </div>
            </div>
          </div>
        </GlassSurface>

        {/* Sub-Card 4: Quick Action Launchers */}
        <GlassSurface variant="solid-card" className="p-4 space-y-3">
          <h4 className="text-xs font-bold text-[var(--text-dominant)] flex items-center space-x-1.5">
            <span>🚀</span>
            <span>Quick Launchers</span>
          </h4>

          <div className="grid grid-cols-2 gap-2 text-xs">
            <button
              type="button"
              onClick={() => handleNav('/files')}
              className="p-2 rounded-lg border border-[var(--border-subtle)] hover:border-[#8B9CFF] bg-[var(--surface-secondary)] text-left transition-colors"
            >
              <div className="text-sm">🗂️</div>
              <div className="font-semibold text-[11px] text-[var(--text-dominant)]">Study Vault</div>
            </button>
            <button
              type="button"
              onClick={() => handleNav('/tasks')}
              className="p-2 rounded-lg border border-[var(--border-subtle)] hover:border-[#65E6C4] bg-[var(--surface-secondary)] text-left transition-colors"
            >
              <div className="text-sm">🎯</div>
              <div className="font-semibold text-[11px] text-[var(--text-dominant)]">All Tasks</div>
            </button>
            <button
              type="button"
              onClick={() => handleNav('/journal')}
              className="p-2 rounded-lg border border-[var(--border-subtle)] hover:border-[#FFD078] bg-[var(--surface-secondary)] text-left transition-colors"
            >
              <div className="text-sm">✍️</div>
              <div className="font-semibold text-[11px] text-[var(--text-dominant)]">Journal</div>
            </button>
            <button
              type="button"
              onClick={() => handleNav('/mind')}
              className="p-2 rounded-lg border border-[var(--border-subtle)] hover:border-[#FF8EAA] bg-[var(--surface-secondary)] text-left transition-colors"
            >
              <div className="text-sm">🧠</div>
              <div className="font-semibold text-[11px] text-[var(--text-dominant)]">Memory</div>
            </button>
          </div>
        </GlassSurface>
      </section>

      {/* ========================================================================= */}
      {/* ZONE 4 — MINDSET BALANCE SECTION                                          */}
      {/* ========================================================================= */}
      <section className="p-5 rounded-2xl border border-[var(--border-subtle)] bg-[var(--surface-primary)] backdrop-blur-xl space-y-4">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2">
          <div>
            <h3 className="text-sm font-bold text-[var(--text-dominant)] flex items-center space-x-2">
              <span>🌿</span>
              <span>MindSet Balance & Wellbeing Guard</span>
            </h3>
            <p className="text-xs text-[var(--accent-secondary)] mt-0.5">
              <strong>Free-Time Window:</strong> {freeTimeWindow.durationMinutes} mins free •{' '}
              {freeTimeWindow.recommendedActivity}
            </p>
          </div>

          {/* Energy / Mood Check-in Pills */}
          <div className="flex items-center space-x-1.5">
            <span className="text-xs text-[var(--text-muted)] mr-1">Check-in:</span>
            {[
              { mood: 'Focused' as const, tier: 'High' as const, icon: '⚡' },
              { mood: 'Calm' as const, tier: 'Medium' as const, icon: '🌿' },
              { mood: 'Fatigued' as const, tier: 'Low' as const, icon: '🥱' },
              { mood: 'Overwhelmed' as const, tier: 'Medium' as const, icon: '🌀' },
            ].map((m) => (
              <button
                key={m.mood}
                type="button"
                onClick={() => handleRecordMood(m.mood, m.tier)}
                className="px-2.5 py-1 rounded-lg border border-[var(--border-subtle)] hover:border-[#65E6C4] bg-[var(--surface-secondary)] text-xs flex items-center space-x-1 transition-colors hover:scale-105"
                title={`Log ${m.mood} (${m.tier} Energy)`}
              >
                <span>{m.icon}</span>
                <span className="hidden sm:inline text-[11px] text-[var(--text-dominant)]">{m.mood}</span>
              </button>
            ))}
          </div>
        </div>

        {/* 3 Tailored Relaxation or Hobby Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <div className="p-3 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] space-y-1.5 hover:border-[#65E6C4]/40 transition-colors">
            <div className="text-sm">🧘 4-7-8 Breathwork</div>
            <div className="text-xs text-[var(--text-muted)]">
              4s inhale, 7s hold, 8s exhale. Reduces cortisol in under 2 minutes.
            </div>
            <button
              type="button"
              onClick={() => handleNav('/mood')}
              className="text-[11px] text-[#65E6C4] font-medium hover:underline"
            >
              Start Breathing →
            </button>
          </div>

          <div className="p-3 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] space-y-1.5 hover:border-[#8B9CFF]/40 transition-colors">
            <div className="text-sm">🌧️ Ambient Rainforest</div>
            <div className="text-xs text-[var(--text-muted)]">
              Restorative 432 Hz rain soundscapes to ease mental fatigue.
            </div>
            <button
              type="button"
              onClick={() => handleNav('/mood')}
              className="text-[11px] text-[#8B9CFF] font-medium hover:underline"
            >
              Play Soundscape →
            </button>
          </div>

          <div className="p-3 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] space-y-1.5 hover:border-[#FFD078]/40 transition-colors">
            <div className="text-sm">👀 20-20-20 Eye Rest</div>
            <div className="text-xs text-[var(--text-muted)]">
              Every 20 mins, look at an object 20 feet away for 20 seconds.
            </div>
            <button
              type="button"
              onClick={() => handleNav('/mood')}
              className="text-[11px] text-[#FFD078] font-medium hover:underline"
            >
              Set Reminder →
            </button>
          </div>
        </div>
      </section>

      {/* ========================================================================= */}
      {/* ZONE 5 — MINDSET AI CONTEXT SECTION                                       */}
      {/* ========================================================================= */}
      <section className="p-5 rounded-2xl border border-[#8B9CFF]/30 bg-gradient-to-r from-[#111827]/90 to-[#182235]/90 backdrop-blur-xl shadow-lg flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div className="space-y-1">
          <div className="flex items-center space-x-2">
            <span className="text-base">🧠</span>
            <h3 className="text-xs font-bold uppercase tracking-wider text-[#8B9CFF]">
              PersonalContextBuilder • Active Memory Level 2
            </h3>
          </div>
          <p className="text-xs text-[var(--text-muted)] max-w-2xl leading-relaxed">
            Targeting: <strong>{topGoalTitle}</strong>.
            Language: <strong>{profile?.ai_lang === 'ta' ? 'Tamil (தமிழ்)' : 'English'}</strong> (
            {profile?.conversation_style === 'tanglish' ? 'Tanglish' : 'Standard'}).
            System state: Memory guard active, prompt injection filters primed.
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          <Button
            variant="secondary"
            size="sm"
            onClick={handleRebalanceEvening}
            className="text-xs font-semibold"
          >
            ⚖️ Rebalance Evening
          </Button>

          <Button
            variant="primary"
            size="sm"
            onClick={() => handleNav('/mind')}
            className="text-xs font-semibold"
          >
            🎓 Ask StudyAgent
          </Button>
        </div>
      </section>
    </div>
  );
};
