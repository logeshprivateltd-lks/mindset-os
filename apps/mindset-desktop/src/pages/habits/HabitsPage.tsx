/**
 * MindSet OS — Non-Punitive Habit Tracker (/habits)
 * Pre-seeded Subconscious Conditioning, Consistency Heatmaps & Supportive Recovery
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { GlassSurface, Button, ProgressRing } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';
import type { Habit } from '@mindset/db-schema';

const DEFAULT_SEEDED_HABITS = [
  {
    title: '06:00 AM Morning Subconscious Conditioning & Goal Visualization',
    category: 'mindset' as const,
    frequency_json: '{"type":"daily"}',
  },
  {
    title: '10:00 PM Night Visualization & Sleep Routine',
    category: 'mindset' as const,
    frequency_json: '{"type":"daily"}',
  },
  {
    title: 'Daily 3 Gratitude Reflections',
    category: 'wellness' as const,
    frequency_json: '{"type":"daily"}',
  },
  {
    title: '90m Uninterrupted Deep Study Block',
    category: 'study' as const,
    frequency_json: '{"type":"daily"}',
  },
  {
    title: 'Hydration & 20-20-20 Eye Rest Guide',
    category: 'wellness' as const,
    frequency_json: '{"type":"daily"}',
  },
];

export const HabitsPage: React.FC = () => {
  const [habits, setHabits] = useState<Habit[]>([]);
  const [newTitle, setNewTitle] = useState<string>('');
  const [newCategory, setNewCategory] = useState<'mindset' | 'study' | 'wellness' | 'routine'>('study');
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const loadHabits = useCallback(async () => {
    try {
      let list = await dbClient.habits.list();
      if (list.length === 0) {
        // Pre-seed default habits on first launch
        for (const seed of DEFAULT_SEEDED_HABITS) {
          await dbClient.habits.create({
            title: seed.title,
            category: seed.category,
            frequency_json: seed.frequency_json,
          });
        }
        list = await dbClient.habits.list();
      }
      setHabits(list);
    } catch {
      setHabits([]);
    }
  }, []);

  useEffect(() => {
    loadHabits();
  }, [loadHabits]);

  // Tick Habit Completion (awards XP, increments streak, updates SQLite, logs timeline event)
  const handleRecordCompletion = async (habitId: string) => {
    try {
      const updated = await dbClient.habits.recordCompletion(habitId);
      setToastMessage(`✓ Great momentum! Logged "${updated.title}" (+25 XP).`);
      setTimeout(() => setToastMessage(null), 3500);
      loadHabits();
    } catch (err) {
      console.error('[Habits] Record completion failed:', err);
    }
  };

  // Create Custom Habit
  const handleCreateHabit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;

    try {
      await dbClient.habits.create({
        title: newTitle.trim(),
        category: newCategory,
        frequency_json: '{"type":"daily"}',
      });
      setNewTitle('');
      setToastMessage('✓ New habit created successfully.');
      setTimeout(() => setToastMessage(null), 3000);
      loadHabits();
    } catch (err) {
      console.error('[Habits] Create habit failed:', err);
    }
  };

  // Completion calculation for today
  const todayStr = new Date().toISOString().slice(0, 10);
  const completedTodayCount = habits.filter((h) => h.last_completed_date === todayStr).length;
  const habitCompletionRate = habits.length > 0 ? Math.round((completedTodayCount / habits.length) * 100) : 0;

  return (
    <div className="space-y-6 select-none p-1">
      {/* ------------------------------------------------------------- */}
      {/* 1. HEADER BANNER: ETHICAL DOPAMINE & RECOVERY PROMPT          */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-3">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xl">🔥</span>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Non-Punitive Habit & Conditioning Tracker
              </h2>
            </div>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              Build lifelong neuro-associations without guilt. Every session is a fresh start toward effortless discipline.
            </p>
          </div>

          <div className="flex items-center space-x-4">
            <div className="text-right">
              <div className="text-[10px] uppercase font-bold text-[var(--text-muted)]">Today's Habits</div>
              <div className="text-xl font-mono font-bold text-[#65E6C4]">
                {completedTodayCount} of {habits.length}
              </div>
            </div>

            <ProgressRing
              progress={habitCompletionRate}
              size={56}
              strokeWidth={5}
              accentColor="#65E6C4"
              sublabel="Done"
            />
          </div>
        </div>

        {/* Ethical Dopamine & Supportive Recovery Prompt */}
        <div className="p-3 rounded-xl bg-[#65E6C4]/10 border border-[#65E6C4]/30 text-xs text-[#65E6C4] flex items-center space-x-2.5">
          <span className="text-base">🌿</span>
          <span>
            <strong>Ethical Dopamine Rule:</strong> Missed a day? No problem! Consistency is about long-term identity, not brittle streaks. Start today with zero pressure.
          </span>
        </div>

        {toastMessage && (
          <div className="text-xs text-[#65E6C4] font-medium animate-fadeIn">
            {toastMessage}
          </div>
        )}
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 2. CREATE NEW HABIT FORM                                      */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-4">
        <form onSubmit={handleCreateHabit} className="flex flex-col sm:flex-row items-center gap-3">
          <input
            type="text"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            placeholder="Add new habit (e.g. 15m Physics formula recall, Evening meditation)..."
            className="w-full bg-[var(--surface-secondary)] text-xs text-[var(--text-dominant)] placeholder-[var(--text-muted)] px-3.5 py-2.5 rounded-xl border border-[var(--border-subtle)] focus:outline-none focus:border-[#8B9CFF]"
          />

          <div className="flex items-center space-x-2 shrink-0 w-full sm:w-auto">
            <select
              value={newCategory}
              onChange={(e) => setNewCategory(e.target.value as any)}
              className="bg-[var(--surface-secondary)] text-xs text-[var(--text-dominant)] px-3 py-2.5 rounded-xl border border-[var(--border-subtle)] focus:outline-none"
            >
              <option value="study">Study</option>
              <option value="mindset">Mindset</option>
              <option value="wellness">Wellness</option>
              <option value="routine">Routine</option>
            </select>

            <Button variant="primary" size="sm" type="submit" className="text-xs font-semibold whitespace-nowrap">
              + Add Habit
            </Button>
          </div>
        </form>
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 3. HABITS LIST & MINI HEATMAP CONSISTENCY GRIDS              */}
      {/* ------------------------------------------------------------- */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {habits.map((habit) => {
          const isDoneToday = habit.last_completed_date === todayStr;

          return (
            <GlassSurface
              key={habit.id}
              variant="solid-card"
              className={`p-4 space-y-3 transition-all ${
                isDoneToday ? 'border-[#65E6C4]/40 bg-[#65E6C4]/5' : ''
              }`}
            >
              <div className="flex items-start justify-between gap-3">
                <div className="space-y-1">
                  <span className="px-2 py-0.5 rounded text-[10px] font-mono uppercase bg-[var(--surface-elevated)] text-[var(--accent-primary)] border border-[var(--border-subtle)]">
                    {habit.category}
                  </span>
                  <div className="font-semibold text-xs text-[var(--text-dominant)] leading-snug">
                    {habit.title}
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => handleRecordCompletion(habit.id)}
                  disabled={isDoneToday}
                  className={`px-3 py-1.5 rounded-xl text-xs font-semibold flex items-center space-x-1.5 transition-all ${
                    isDoneToday
                      ? 'bg-[#65E6C4]/20 text-[#65E6C4] border border-[#65E6C4]/40 cursor-default'
                      : 'bg-[var(--accent-primary)] hover:bg-[#65E6C4] text-[#07090D] shadow-md hover:scale-105'
                  }`}
                >
                  <span>{isDoneToday ? '✓ Done' : 'Complete'}</span>
                </button>
              </div>

              {/* Consistency Mini-Heatmap (14 Days) */}
              <div className="pt-2 border-t border-[var(--border-subtle)] flex items-center justify-between">
                <div className="flex items-center space-x-1.5">
                  <span className="text-[10px] text-[var(--text-muted)] mr-1">14-Day:</span>
                  {[...Array(14)].map((_, i) => {
                    const isFilled = i >= 14 - Math.min(habit.current_streak, 14);
                    return (
                      <span
                        key={i}
                        className={`w-2.5 h-2.5 rounded-sm transition-colors ${
                          isFilled
                            ? 'bg-[#65E6C4] shadow-[0_0_6px_rgba(101,230,196,0.5)]'
                            : 'bg-[var(--surface-elevated)] border border-[var(--border-subtle)]'
                        }`}
                        title={`Day -${14 - i}`}
                      />
                    );
                  })}
                </div>

                <div className="text-[11px] font-mono text-[#FFD078] font-bold">
                  🔥 {habit.current_streak} streak
                </div>
              </div>
            </GlassSurface>
          );
        })}
      </div>
    </div>
  );
};

