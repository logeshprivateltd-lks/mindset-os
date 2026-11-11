/**
 * MindSet OS Tasks & Goals Native Window App
 * Interactive syllabus milestone tracker and daily task manager
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import { dbClient } from '../../lib/dbClient';
import type { TaskOrGoal } from '@mindset/db-schema';

export const TasksGoalsApp: React.FC = () => {
  const [tasks, setTasks] = useState<TaskOrGoal[]>([]);
  const [filter, setFilter] = useState<'all' | 'in_progress' | 'completed'>('all');
  const [newTaskTitle, setNewTaskTitle] = useState('');

  const loadTasks = async () => {
    try {
      const list = await dbClient.tasks.list();
      setTasks(list);
    } catch {
      // Mock defaults
      setTasks([
        {
          id: 't-1',
          parent_goal_id: null,
          title: 'Complete MindSet OS Kernel Calibrations & Low-RAM Audit',
          subject_tag: 'Systems',
          priority: 1,
          status: 'in_progress',
          target_score_weight: 1.0,
          scheduled_start: null,
          scheduled_end: null,
          completed_at: null,
          xp_reward: 50,
          updated_at: new Date().toISOString(),
        },
        {
          id: 't-2',
          parent_goal_id: null,
          title: 'Differential Equations: First-Order ODEs Problem Set',
          subject_tag: 'Math II',
          priority: 2,
          status: 'in_progress',
          target_score_weight: 1.0,
          scheduled_start: null,
          scheduled_end: null,
          completed_at: null,
          xp_reward: 35,
          updated_at: new Date().toISOString(),
        },
        {
          id: 't-3',
          parent_goal_id: null,
          title: 'Modern Physics: Photoelectric Effect Summary',
          subject_tag: 'Physics',
          priority: 2,
          status: 'completed',
          target_score_weight: 1.0,
          scheduled_start: null,
          scheduled_end: null,
          completed_at: new Date().toISOString(),
          xp_reward: 30,
          updated_at: new Date().toISOString(),
        },
      ]);
    }
  };

  useEffect(() => {
    loadTasks();
  }, []);

  const handleToggleTask = async (id: string, currentStatus: string) => {
    const nextStatus = currentStatus === 'completed' ? 'in_progress' : 'completed';
    try {
      if (nextStatus === 'completed') {
        await dbClient.tasks.complete(id);
      } else {
        const target = tasks.find((t) => t.id === id);
        if (target) {
          await dbClient.tasks.update({
            ...target,
            status: 'in_progress',
            completed_at: null,
          });
        }
      }
      loadTasks();
    } catch {
      setTasks((prev) =>
        prev.map((t) => (t.id === id ? { ...t, status: nextStatus as any } : t))
      );
    }
  };

  const handleCreateTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTaskTitle.trim()) return;

    try {
      await dbClient.tasks.create({
        title: newTaskTitle.trim(),
        priority: 2,
        status: 'in_progress',
        subject_tag: 'Self-Study',
        xp_reward: 25,
      });
      setNewTaskTitle('');
      loadTasks();
    } catch {
      const newTask: TaskOrGoal = {
        id: `t-${Date.now()}`,
        parent_goal_id: null,
        title: newTaskTitle.trim(),
        subject_tag: 'Self-Study',
        priority: 2,
        status: 'in_progress',
        target_score_weight: 1.0,
        scheduled_start: null,
        scheduled_end: null,
        completed_at: null,
        xp_reward: 25,
        updated_at: new Date().toISOString(),
      };
      setTasks((prev) => [newTask, ...prev]);
      setNewTaskTitle('');
    }
  };

  const filteredTasks = tasks.filter((t) => {
    if (filter === 'all') return true;
    return t.status === filter;
  });

  return (
    <div className="h-full flex flex-col p-5 space-y-4 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)]">
      {/* Header bar */}
      <div className="flex items-center justify-between border-b border-[var(--border-subtle)] pb-3">
        <div>
          <h2 className="text-base font-bold">Tasks & Syllabus Objectives</h2>
          <p className="text-xs text-[var(--text-muted)]">Automatic timeline event recording on milestone completion</p>
        </div>

        {/* Filter Tabs */}
        <div className="flex rounded-lg bg-[var(--surface-secondary)] p-1 border border-[var(--border-subtle)]">
          {(['all', 'in_progress', 'completed'] as const).map((f) => (
            <button
              key={f}
              type="button"
              onClick={() => setFilter(f)}
              className={`px-3 py-1 text-xs rounded-md capitalize font-medium transition-colors ${
                filter === f
                  ? 'bg-[var(--surface-elevated)] text-[var(--accent-primary)] font-bold shadow-sm'
                  : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              {f.replace('_', ' ')}
            </button>
          ))}
        </div>
      </div>

      {/* Task input */}
      <form onSubmit={handleCreateTask} className="flex space-x-2">
        <input
          type="text"
          value={newTaskTitle}
          onChange={(e) => setNewTaskTitle(e.target.value)}
          placeholder="Add new study objective or milestone (e.g. Master Kirchhoff's Laws)..."
          className="flex-1 px-3.5 py-2 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] focus:border-[var(--accent-primary)] outline-none text-xs text-[var(--text-dominant)]"
        />
        <button
          type="submit"
          disabled={!newTaskTitle.trim()}
          className="px-4 py-2 rounded-lg text-xs font-bold bg-[var(--accent-primary)] text-[#07090D] hover:scale-102 transition-transform disabled:opacity-50"
        >
          Add Goal +
        </button>
      </form>

      {/* Task list */}
      <div className="flex-1 overflow-y-auto space-y-2 pr-1">
        {filteredTasks.map((t) => {
          const isDone = t.status === 'completed';
          return (
            <div
              key={t.id}
              onClick={() => handleToggleTask(t.id, t.status)}
              className={`p-3 rounded-xl border flex items-center justify-between cursor-pointer transition-all duration-150 ${
                isDone
                  ? 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] opacity-70'
                  : 'bg-[var(--surface-elevated)] border-[var(--border-subtle)] hover:border-[var(--accent-primary)]'
              }`}
            >
              <div className="flex items-center space-x-3 min-w-0">
                <div
                  className={`w-5 h-5 rounded-md border flex items-center justify-center text-xs shrink-0 transition-colors ${
                    isDone
                      ? 'bg-[var(--accent-achievement)] border-[var(--accent-achievement)] text-[#07090D] font-bold'
                      : 'border-[var(--border-subtle)] hover:border-[var(--accent-primary)]'
                  }`}
                >
                  {isDone && '✓'}
                </div>

                <div className="min-w-0">
                  <h4
                    className={`text-xs font-medium truncate ${
                      isDone ? 'line-through text-[var(--text-muted)]' : 'text-[var(--text-dominant)]'
                    }`}
                  >
                    {t.title}
                  </h4>
                  {t.subject_tag && (
                    <span className="text-[10px] text-[var(--text-muted)] mt-0.5 block">
                      Subject: {t.subject_tag} • Priority {t.priority}
                    </span>
                  )}
                </div>
              </div>

              <span className="text-xs font-mono font-bold text-[var(--accent-achievement)] shrink-0 pl-2">
                +{t.xp_reward} XP
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
};
