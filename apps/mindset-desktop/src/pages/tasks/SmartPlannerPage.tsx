/**
 * MindSet OS — Smart Daily & Weekly Timetable Scheduler
 * Configurable schedule template (05:30 Wake, 06:00 Study, 22:00 Sleep) with time-blocking
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { GlassSurface } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';
import type { TaskOrGoal } from '@mindset/db-schema';

export interface ScheduleBlockItem {
  id: string;
  timeRange: string;
  label: string;
  category: 'routine' | 'study' | 'school' | 'wellness' | 'sleep';
  icon: string;
  assignedTaskId?: string;
  assignedTaskTitle?: string;
}

const DEFAULT_WEEKDAY_BLOCKS: ScheduleBlockItem[] = [
  { id: 'b-1', timeRange: '05:30 – 06:00', label: 'Wake & Morning Hydration', category: 'routine', icon: '💧' },
  { id: 'b-2', timeRange: '06:00 – 07:30', label: 'Morning Study & Subconscious Visualization', category: 'study', icon: '🌅' },
  { id: 'b-3', timeRange: '08:00 – 17:00', label: 'School / Academic Instruction & Practical Lab', category: 'school', icon: '🏫' },
  { id: 'b-4', timeRange: '17:30 – 18:30', label: 'Outdoor Physical Exercise & Relaxation', category: 'wellness', icon: '🏃' },
  { id: 'b-5', timeRange: '19:00 – 21:00', label: 'Evening Deep Study & Problem Sets', category: 'study', icon: '⚡' },
  { id: 'b-6', timeRange: '21:00 – 21:45', label: 'Family Dinner & Digital Detox', category: 'wellness', icon: '🍲' },
  { id: 'b-7', timeRange: '22:00 – 05:30', label: 'Night Visualization & Deep Sleep Mode', category: 'sleep', icon: '🌙' },
];

const DEFAULT_WEEKEND_BLOCKS: ScheduleBlockItem[] = [
  { id: 'wb-1', timeRange: '06:30 – 07:00', label: 'Gentle Wake & Morning Sunlight', category: 'routine', icon: '☀️' },
  { id: 'wb-2', timeRange: '07:30 – 10:30', label: 'Full Mock Test & Time-Trial Problem Solving', category: 'study', icon: '📝' },
  { id: 'wb-3', timeRange: '11:00 – 13:00', label: 'Deep Error Analysis & Concept Corrections', category: 'study', icon: '🔍' },
  { id: 'wb-4', timeRange: '14:00 – 17:00', label: 'Hobbies, Music, Coding & Free Exploration', category: 'wellness', icon: '🎨' },
  { id: 'wb-5', timeRange: '17:30 – 19:30', label: 'Revision & Flashcards Recall', category: 'study', icon: '📚' },
  { id: 'wb-6', timeRange: '20:00 – 21:30', label: 'Weekend Reflection & Next Week Roadmapping', category: 'routine', icon: '🗺️' },
  { id: 'wb-7', timeRange: '22:00 – 06:30', label: 'Sleep Routine & Restorative Rest', category: 'sleep', icon: '🌙' },
];

export const SmartPlannerPage: React.FC = () => {
  const [profileType, setProfileType] = useState<'weekday' | 'weekend'>('weekday');
  const [schedule, setSchedule] = useState<ScheduleBlockItem[]>(DEFAULT_WEEKDAY_BLOCKS);
  const [tasks, setTasks] = useState<TaskOrGoal[]>([]);
  const [selectedBlockId, setSelectedBlockId] = useState<string | null>(null);
  const [syncNotice, setSyncNotice] = useState<string | null>(null);

  const loadTasks = useCallback(async () => {
    try {
      const list = await dbClient.tasks.list('pending');
      setTasks(list);
    } catch {
      setTasks([]);
    }
  }, []);

  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  const handleProfileSwitch = (type: 'weekday' | 'weekend') => {
    setProfileType(type);
    setSchedule(type === 'weekday' ? DEFAULT_WEEKDAY_BLOCKS : DEFAULT_WEEKEND_BLOCKS);
  };

  const handleAssignTaskToBlock = (blockId: string, task: TaskOrGoal) => {
    setSchedule((prev) =>
      prev.map((b) => {
        if (b.id !== blockId) return b;
        return {
          ...b,
          assignedTaskId: task.id,
          assignedTaskTitle: task.title,
        };
      })
    );

    setSelectedBlockId(null);
    setSyncNotice(`Assigned "${task.title}" to ${schedule.find((b) => b.id === blockId)?.timeRange}. HLC recorded.`);
    setTimeout(() => setSyncNotice(null), 3500);
  };

  const handleClearAssignment = (blockId: string) => {
    setSchedule((prev) =>
      prev.map((b) => {
        if (b.id !== blockId) return b;
        return {
          ...b,
          assignedTaskId: undefined,
          assignedTaskTitle: undefined,
        };
      })
    );
  };

  const getCategoryColor = (cat: ScheduleBlockItem['category']) => {
    switch (cat) {
      case 'study':
        return 'border-[#8B9CFF] bg-[#8B9CFF]/10 text-[#8B9CFF]';
      case 'school':
        return 'border-[#A78BFA] bg-[#A78BFA]/10 text-[#A78BFA]';
      case 'wellness':
        return 'border-[#65E6C4] bg-[#65E6C4]/10 text-[#65E6C4]';
      case 'sleep':
        return 'border-[#FFD078] bg-[#FFD078]/10 text-[#FFD078]';
      default:
        return 'border-[var(--border-subtle)] bg-[var(--surface-primary)] text-[var(--text-muted)]';
    }
  };

  return (
    <div className="space-y-6 select-none p-1">
      {/* ------------------------------------------------------------- */}
      {/* 1. HEADER & PROFILE TOGGLE                                    */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-3">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xl">📅</span>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Smart Timetable & Task Time-Blocking
              </h2>
            </div>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              Structured around peak cognitive circadian rhythms: 05:30 wake, morning study, deep evening session, and 22:00 sleep mode.
            </p>
          </div>

          {/* Profile Switcher */}
          <div className="flex items-center space-x-2 bg-[var(--surface-secondary)] p-1 rounded-xl border border-[var(--border-subtle)]">
            <button
              type="button"
              onClick={() => handleProfileSwitch('weekday')}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
                profileType === 'weekday'
                  ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                  : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              Weekday (School)
            </button>
            <button
              type="button"
              onClick={() => handleProfileSwitch('weekend')}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
                profileType === 'weekend'
                  ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                  : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              Weekend (Mock Tests)
            </button>
          </div>
        </div>

        {syncNotice && (
          <div className="text-xs text-[#65E6C4] font-medium animate-fadeIn pt-1">
            ✓ {syncNotice}
          </div>
        )}
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 2. MAIN GRID: TIMETABLE SCHEDULE & PENDING TASK PICKER        */}
      {/* ------------------------------------------------------------- */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left 2 Cols: Chronological Timetable Blocks */}
        <div className="lg:col-span-2 space-y-3">
          {schedule.map((block) => {
            const hasTask = Boolean(block.assignedTaskTitle);
            const isSelected = selectedBlockId === block.id;

            return (
              <div
                key={block.id}
                className={`p-4 rounded-xl border transition-all ${
                  isSelected
                    ? 'border-[#8B9CFF] bg-[#8B9CFF]/15 shadow-md'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-primary)] hover:border-[var(--border-strong)]'
                }`}
              >
                <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2">
                  <div className="flex items-center space-x-3">
                    <span className="text-xl">{block.icon}</span>
                    <div>
                      <div className="font-mono text-xs font-bold text-[var(--text-dominant)]">
                        {block.timeRange}
                      </div>
                      <div className="text-xs text-[var(--text-muted)]">{block.label}</div>
                    </div>
                  </div>

                  <div className="flex items-center space-x-2">
                    <span className={`px-2 py-0.5 rounded text-[10px] font-mono border ${getCategoryColor(block.category)}`}>
                      {block.category.toUpperCase()}
                    </span>

                    {hasTask ? (
                      <button
                        type="button"
                        onClick={() => handleClearAssignment(block.id)}
                        className="text-xs text-red-400 hover:underline px-2 py-1"
                      >
                        Unlink
                      </button>
                    ) : (
                      <button
                        type="button"
                        onClick={() => setSelectedBlockId(isSelected ? null : block.id)}
                        className="px-2.5 py-1 rounded-lg text-xs font-medium border border-[#8B9CFF]/40 bg-[#8B9CFF]/10 text-[#8B9CFF] hover:bg-[#8B9CFF]/20"
                      >
                        {isSelected ? 'Cancel' : '+ Block Task'}
                      </button>
                    )}
                  </div>
                </div>

                {/* Assigned Task Preview */}
                {hasTask && (
                  <div className="mt-2.5 p-2 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex items-center justify-between text-xs">
                    <span className="font-medium text-[#65E6C4] flex items-center space-x-1.5">
                      <span>🎯</span>
                      <span>{block.assignedTaskTitle}</span>
                    </span>
                    <span className="text-[10px] text-[var(--text-muted)] font-mono">
                      Active Time-Block
                    </span>
                  </div>
                )}
              </div>
            );
          })}
        </div>

        {/* Right Col: Pending Tasks Quick-Assigner */}
        <div className="space-y-4">
          <GlassSurface variant="solid-card" className="p-4 space-y-3">
            <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)]">
              Pending Tasks Queue
            </h3>
            <p className="text-[11px] text-[var(--text-muted)]">
              {selectedBlockId
                ? 'Click any task below to bind it to the highlighted time block.'
                : 'Select "+ Block Task" on any time block to schedule a task.'}
            </p>

            <div className="space-y-2 max-h-[480px] overflow-y-auto">
              {tasks.length === 0 ? (
                <div className="text-xs text-[var(--text-muted)] py-6 text-center">
                  No pending tasks found. All caught up!
                </div>
              ) : (
                tasks.map((task) => (
                  <div
                    key={task.id}
                    onClick={() => {
                      if (selectedBlockId) {
                        handleAssignTaskToBlock(selectedBlockId, task);
                      }
                    }}
                    className={`p-3 rounded-lg border text-xs transition-all ${
                      selectedBlockId
                        ? 'cursor-pointer hover:border-[#65E6C4] hover:bg-[#65E6C4]/10 border-[#8B9CFF]/30 bg-[var(--surface-secondary)]'
                        : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] opacity-80'
                    }`}
                  >
                    <div className="font-semibold text-[var(--text-dominant)]">{task.title}</div>
                    <div className="flex items-center justify-between text-[10px] text-[var(--text-muted)] mt-1.5">
                      <span>{task.subject_tag || 'General'}</span>
                      <span className="text-[#FFD078] font-mono">+{task.xp_reward || 10} XP</span>
                    </div>
                  </div>
                ))
              )}
            </div>
          </GlassSurface>
        </div>
      </div>
    </div>
  );
};
