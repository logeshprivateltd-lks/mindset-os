/**
 * MindSet OS Tasks, Goals & Syllabus Native Window App
 * Interactive 3-in-1 Workspace: Tasks Checklist, Goals & Syllabus Skill-Tree, and Smart Timetable
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { dbClient } from '../../lib/dbClient';
import type { TaskOrGoal } from '@mindset/db-schema';
import { GoalsAndSyllabusView, SmartPlannerPage } from '../../pages/tasks';

export const TasksGoalsApp: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'tasks' | 'goals' | 'planner'>('tasks');
  const [tasks, setTasks] = useState<TaskOrGoal[]>([]);
  const [filter, setFilter] = useState<'all' | 'pending' | 'completed'>('all');
  const [newTaskTitle, setNewTaskTitle] = useState('');
  const [newTaskSubject, setNewTaskSubject] = useState('Mathematics');
  const [newTaskPriority, setNewTaskPriority] = useState<1 | 2 | 3>(2);

  const loadTasks = useCallback(async () => {
    try {
      const list = await dbClient.tasks.list();
      setTasks(list);
    } catch {
      setTasks([]);
    }
  }, []);

  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  const handleToggleTask = async (id: string, currentStatus: string) => {
    const nextStatus = currentStatus === 'completed' ? 'pending' : 'completed';
    try {
      if (nextStatus === 'completed') {
        await dbClient.tasks.complete(id);
      } else {
        const target = tasks.find((t) => t.id === id);
        if (target) {
          await dbClient.tasks.update({
            ...target,
            status: 'pending',
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
        subject_tag: newTaskSubject,
        priority: newTaskPriority,
        xp_reward: newTaskPriority === 1 ? 50 : 25,
      });
      setNewTaskTitle('');
      loadTasks();
    } catch (err) {
      console.error('[TasksApp] Create task failed:', err);
    }
  };

  const filteredTasks = tasks.filter((t) => {
    if (filter === 'all') return true;
    if (filter === 'completed') return t.status === 'completed';
    return t.status === 'pending' || t.status === 'in_progress';
  });

  return (
    <div className="flex flex-col h-full bg-[var(--surface-primary)] select-none">
      {/* ------------------------------------------------------------- */}
      {/* TOP NAVIGATION TABS                                           */}
      {/* ------------------------------------------------------------- */}
      <div className="flex items-center justify-between px-4 py-2.5 border-b border-[var(--border-subtle)] bg-[var(--surface-secondary)]">
        <div className="flex items-center space-x-1.5">
          <button
            type="button"
            onClick={() => setActiveTab('tasks')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center space-x-1.5 transition-all ${
              activeTab === 'tasks'
                ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            <span>🎯</span>
            <span>Daily Tasks</span>
          </button>

          <button
            type="button"
            onClick={() => setActiveTab('goals')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center space-x-1.5 transition-all ${
              activeTab === 'goals'
                ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            <span>🌳</span>
            <span>Syllabus Skill-Tree (590/600)</span>
          </button>

          <button
            type="button"
            onClick={() => setActiveTab('planner')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center space-x-1.5 transition-all ${
              activeTab === 'planner'
                ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            <span>📅</span>
            <span>Smart Timetable</span>
          </button>
        </div>

        <div className="text-[11px] font-mono text-[var(--accent-secondary)]">
          {tasks.filter((t) => t.status === 'completed').length} of {tasks.length} Done
        </div>
      </div>

      {/* ------------------------------------------------------------- */}
      {/* TAB CONTENT AREA                                              */}
      {/* ------------------------------------------------------------- */}
      <div className="flex-1 overflow-y-auto p-4">
        {activeTab === 'goals' && <GoalsAndSyllabusView />}
        {activeTab === 'planner' && <SmartPlannerPage />}
        {activeTab === 'tasks' && (
          <div className="space-y-4 max-w-4xl mx-auto">
            {/* Quick Task Creation Form */}
            <form onSubmit={handleCreateTask} className="p-4 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-secondary)] space-y-3">
              <input
                type="text"
                value={newTaskTitle}
                onChange={(e) => setNewTaskTitle(e.target.value)}
                placeholder="What is your next study milestone or micro-task?"
                className="w-full bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] placeholder-[var(--text-muted)] px-3.5 py-2.5 rounded-xl border border-[var(--border-subtle)] focus:outline-none focus:border-[#8B9CFF]"
              />

              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex items-center space-x-2">
                  <select
                    value={newTaskSubject}
                    onChange={(e) => setNewTaskSubject(e.target.value)}
                    className="bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] px-3 py-1.5 rounded-lg border border-[var(--border-subtle)]"
                  >
                    <option value="Mathematics">Mathematics</option>
                    <option value="Physics">Physics</option>
                    <option value="Chemistry">Chemistry</option>
                    <option value="Computer Science">Computer Science</option>
                    <option value="Tamil">Tamil</option>
                    <option value="English">English</option>
                  </select>

                  <select
                    value={newTaskPriority}
                    onChange={(e) => setNewTaskPriority(Number(e.target.value) as 1 | 2 | 3)}
                    className="bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] px-3 py-1.5 rounded-lg border border-[var(--border-subtle)]"
                  >
                    <option value={1}>Priority 1 (Crucial)</option>
                    <option value={2}>Priority 2 (Standard)</option>
                    <option value={3}>Priority 3 (Low)</option>
                  </select>
                </div>

                <button
                  type="submit"
                  className="px-4 py-1.5 rounded-lg text-xs font-semibold bg-[var(--accent-primary)] hover:bg-[#65E6C4] text-[#07090D] shadow-sm"
                >
                  + Add Study Target
                </button>
              </div>
            </form>

            {/* Filter Chips */}
            <div className="flex items-center space-x-2 text-xs">
              {(['all', 'pending', 'completed'] as const).map((mode) => (
                <button
                  key={mode}
                  type="button"
                  onClick={() => setFilter(mode)}
                  className={`px-3 py-1 rounded-lg font-medium transition-all ${
                    filter === mode
                      ? 'bg-[var(--surface-elevated)] text-[var(--text-dominant)] border border-[var(--border-strong)]'
                      : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                  }`}
                >
                  {mode === 'all' ? 'All Tasks' : mode === 'pending' ? 'Pending' : 'Completed'}
                </button>
              ))}
            </div>

            {/* Task Checklist Items */}
            <div className="space-y-2">
              {filteredTasks.length === 0 ? (
                <div className="p-8 text-center text-xs text-[var(--text-muted)]">
                  No tasks matching this filter.
                </div>
              ) : (
                filteredTasks.map((t) => {
                  const isDone = t.status === 'completed';

                  return (
                    <div
                      key={t.id}
                      onClick={() => handleToggleTask(t.id, t.status)}
                      className={`p-3.5 rounded-xl border flex items-center justify-between cursor-pointer transition-all ${
                        isDone
                          ? 'border-[var(--border-subtle)] bg-[var(--surface-primary)] opacity-60'
                          : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] hover:border-[#8B9CFF]'
                      }`}
                    >
                      <div className="flex items-center space-x-3">
                        <span className={`w-4 h-4 rounded-md border flex items-center justify-center text-[10px] ${
                          isDone
                            ? 'border-[#65E6C4] bg-[#65E6C4] text-[#07090D] font-bold'
                            : 'border-[var(--border-strong)] bg-transparent'
                        }`}>
                          {isDone ? '✓' : ''}
                        </span>

                        <div>
                          <div className={`text-xs font-semibold ${isDone ? 'line-through text-[var(--text-muted)]' : 'text-[var(--text-dominant)]'}`}>
                            {t.title}
                          </div>
                          <div className="text-[10px] text-[var(--text-muted)] flex items-center space-x-2 mt-0.5">
                            <span>{t.subject_tag || 'Study'}</span>
                            <span>•</span>
                            <span className="text-[#FFD078] font-mono">+{t.xp_reward || 10} XP</span>
                          </div>
                        </div>
                      </div>

                      <span className={`px-2 py-0.5 rounded text-[10px] font-mono ${
                        t.priority === 1
                          ? 'bg-[#FF8EAA]/20 text-[#FF8EAA] border border-[#FF8EAA]/30'
                          : 'bg-[var(--surface-elevated)] text-[var(--text-muted)]'
                      }`}>
                        P{t.priority}
                      </span>
                    </div>
                  );
                })
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
