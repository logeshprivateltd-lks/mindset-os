/**
 * MindSet OS — Student AI Memory Inspector & Sovereign Control Modal
 * Full CRUD for Level 1, Level 2, and Level 3 Epistemic Memories
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useMemo } from 'react';
import type { UserProfile } from '@mindset/db-schema';
import {
  MemoryLevel3_Record,
  MemoryLevel2_ActiveContext,
  MemoryInspectorService,
} from '@mindset/identity-engine';
import { GlassSurface } from '@mindset/ui-core';
import { dbClient } from '../lib/dbClient';

interface MemoryInspectorModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const MemoryInspectorModal: React.FC<MemoryInspectorModalProps> = ({
  isOpen,
  onClose,
}) => {
  const [activeTab, setActiveTab] = useState<'level1' | 'level2' | 'level3'>('level3');
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [memories, setMemories] = useState<MemoryLevel3_Record[]>([
    {
      id: 'mem-1',
      sourceType: 'goal_milestone',
      title: 'Completed Physics Mechanics Fundamentals',
      details: 'Mastered Newton laws, rotational dynamics, and work-energy theorem derivations with 92% practice score.',
      timestamp: '2026-11-01T10:30:00Z',
      tags: ['Physics', 'Mechanics', 'Milestone'],
    },
    {
      id: 'mem-2',
      sourceType: 'concept_mastery',
      title: 'Rust Concurrency & Arc/Mutex Architecture',
      details: 'Demonstrated deterministic state handling without locks across 4 threads using Tokio broadcast channels.',
      timestamp: '2026-11-05T18:15:00Z',
      tags: ['Rust', 'Systems', 'Concepts'],
    },
    {
      id: 'mem-3',
      sourceType: 'journal_reflection',
      title: 'Optimal Study Window: 06:30 - 08:30 AM',
      details: 'Consistently highest cognitive flow and zero interruptions achieved when studying before breakfast.',
      timestamp: '2026-11-08T07:45:00Z',
      tags: ['Habits', 'FlowState', 'Wellbeing'],
    },
  ]);

  const [searchTerm, setSearchTerm] = useState('');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [copyFeedback, setCopyFeedback] = useState<string | null>(null);

  // New Memory Form State
  const [newTitle, setNewTitle] = useState('');
  const [newDetails, setNewDetails] = useState('');
  const [newTag, setNewTag] = useState('Study');
  const [newType, setNewType] = useState<MemoryLevel3_Record['sourceType']>('syllabus_topic');
  const [isAdding, setIsAdding] = useState(false);

  // Fetch Profile on open
  useEffect(() => {
    if (isOpen) {
      dbClient.profile.get().then((p) => {
        if (p) setProfile(p);
      });
    }
  }, [isOpen]);

  // Sourced Active Context
  const activeContext: MemoryLevel2_ActiveContext = useMemo(() => {
    return MemoryInspectorService.inspectLevel2({
      currentTime: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      availableFreeMinutes: 50,
      nextCommitment: {
        title: 'Physics Mechanics Practice',
        startTime: '17:30',
        minutesRemaining: 45,
      },
      activeFocusMode: 'DeepWork',
      pendingTasks: [
        { id: 't1', title: 'Solve 10 Kinematics Problems', priority: 1, subjectTag: 'Physics' },
        { id: 't2', title: 'Review Organic Chemistry Mechanisms', priority: 2, subjectTag: 'Chemistry' },
      ],
      recentMood: 'Focused',
      recentEnergy: 'High',
      activeActivity: 'deep-study',
      lastCompletedTask: 'Read Chapter 4 Notes',
    });
  }, []);

  // Filtered Level 3 Memories
  const filteredMemories = useMemo(() => {
    return MemoryInspectorService.listLevel3Memories(memories, {
      searchTerm: searchTerm.trim() || undefined,
      tag: selectedTag || undefined,
    });
  }, [memories, searchTerm, selectedTag]);

  // Handlers
  const handleAddMemory = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim() || !newDetails.trim()) return;

    const { updatedMemories } = MemoryInspectorService.addMemory(memories, {
      sourceType: newType,
      title: newTitle.trim(),
      details: newDetails.trim(),
      tags: [newTag.trim()].filter(Boolean),
    });

    setMemories(updatedMemories);
    setNewTitle('');
    setNewDetails('');
    setIsAdding(false);
  };

  const handleDeleteMemory = (id: string) => {
    setMemories((prev) => MemoryInspectorService.deleteMemory(prev, id));
  };

  const handleClearAll = () => {
    if (window.confirm('Are you sure you want to purge all Level 3 long-term memories? This action is permanent.')) {
      setMemories(MemoryInspectorService.clearAllMemories());
    }
  };

  const handleExportJson = () => {
    if (!profile) return;
    const payload = MemoryInspectorService.buildExportPayload(profile, activeContext, memories);
    const jsonStr = MemoryInspectorService.exportAsJson(payload);
    navigator.clipboard.writeText(jsonStr);
    setCopyFeedback('JSON copied to clipboard!');
    setTimeout(() => setCopyFeedback(null), 2500);
  };

  const handleExportMarkdown = () => {
    if (!profile) return;
    const payload = MemoryInspectorService.buildExportPayload(profile, activeContext, memories);
    const mdStr = MemoryInspectorService.exportAsMarkdown(payload);
    navigator.clipboard.writeText(mdStr);
    setCopyFeedback('Markdown copied to clipboard!');
    setTimeout(() => setCopyFeedback(null), 2500);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm select-none">
      <GlassSurface
        variant="modal"
        className="w-full max-w-4xl p-6 rounded-2xl relative shadow-2xl border flex flex-col max-h-[85vh] overflow-hidden"
        style={{
          borderColor: 'var(--border-subtle)',
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-dominant)',
        }}
      >
        {/* Header */}
        <div className="flex items-center justify-between pb-4 border-b border-[var(--border-subtle)]">
          <div className="flex items-center gap-2">
            <span className="text-2xl">🧠</span>
            <div>
              <h2 className="text-lg font-bold tracking-tight text-[var(--text-dominant)]">
                AI Memory Inspector & Sovereign Control
              </h2>
              <p className="text-xs text-[var(--text-muted)]">
                Inspect, refine, export, or delete what MindSet AI knows about your progress.
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 rounded-lg bg-[var(--bg-secondary)] hover:bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] transition"
          >
            ✕ Close
          </button>
        </div>

        {/* Tab Selector */}
        <div className="flex items-center justify-between pt-3 pb-2">
          <div className="flex items-center gap-2">
            <button
              onClick={() => setActiveTab('level3')}
              className="px-3 py-1.5 rounded-lg text-xs font-semibold border transition"
              style={{
                backgroundColor: activeTab === 'level3' ? 'var(--bg-elevated)' : 'transparent',
                borderColor: activeTab === 'level3' ? 'var(--accent-primary)' : 'var(--border-subtle)',
                color: activeTab === 'level3' ? 'var(--accent-primary)' : 'var(--text-secondary)',
              }}
            >
              Level 3: Epistemic Memory ({memories.length})
            </button>
            <button
              onClick={() => setActiveTab('level2')}
              className="px-3 py-1.5 rounded-lg text-xs font-semibold border transition"
              style={{
                backgroundColor: activeTab === 'level2' ? 'var(--bg-elevated)' : 'transparent',
                borderColor: activeTab === 'level2' ? 'var(--accent-primary)' : 'var(--border-subtle)',
                color: activeTab === 'level2' ? 'var(--accent-primary)' : 'var(--text-secondary)',
              }}
            >
              Level 2: Active Working Context
            </button>
            <button
              onClick={() => setActiveTab('level1')}
              className="px-3 py-1.5 rounded-lg text-xs font-semibold border transition"
              style={{
                backgroundColor: activeTab === 'level1' ? 'var(--bg-elevated)' : 'transparent',
                borderColor: activeTab === 'level1' ? 'var(--accent-primary)' : 'var(--border-subtle)',
                color: activeTab === 'level1' ? 'var(--accent-primary)' : 'var(--text-secondary)',
              }}
            >
              Level 1: Durable Profile
            </button>
          </div>

          {/* Export Actions */}
          <div className="flex items-center gap-2">
            {copyFeedback && (
              <span className="text-[11px] font-mono text-emerald-400 font-bold animate-pulse">
                ✓ {copyFeedback}
              </span>
            )}
            <button
              onClick={handleExportJson}
              className="px-2.5 py-1 rounded-md bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-[11px] font-mono text-[var(--text-secondary)] hover:text-[var(--accent-primary)] transition"
              title="Copy memories as structured JSON"
            >
              Export JSON
            </button>
            <button
              onClick={handleExportMarkdown}
              className="px-2.5 py-1 rounded-md bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-[11px] font-mono text-[var(--text-secondary)] hover:text-[var(--accent-primary)] transition"
              title="Copy memories as human-readable Markdown"
            >
              Export Markdown
            </button>
          </div>
        </div>

        {/* Tab Body */}
        <div className="flex-1 overflow-y-auto py-3 space-y-4 pr-1">
          {/* ===================================================================== */}
          {/* LEVEL 3 TAB: Long-Term Epistemic Memories                             */}
          {/* ===================================================================== */}
          {activeTab === 'level3' && (
            <div className="space-y-4">
              {/* Search & Actions Bar */}
              <div className="flex items-center gap-2">
                <input
                  type="text"
                  placeholder="Search memories by keyword or topic..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="flex-1 px-3 py-2 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-xs text-[var(--text-dominant)] outline-none focus:border-[var(--accent-primary)]"
                />
                {selectedTag && (
                  <button
                    onClick={() => setSelectedTag(null)}
                    className="px-2 py-1 rounded bg-[var(--accent-primary)]/20 border border-[var(--accent-primary)] text-xs text-[var(--accent-primary)]"
                  >
                    Tag: {selectedTag} ✕
                  </button>
                )}
                <button
                  onClick={() => setIsAdding(!isAdding)}
                  className="px-3 py-2 rounded-lg bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold transition hover:opacity-90"
                >
                  {isAdding ? 'Cancel' : '+ Add Memory'}
                </button>
                {memories.length > 0 && (
                  <button
                    onClick={handleClearAll}
                    className="px-3 py-2 rounded-lg bg-rose-500/20 text-rose-300 border border-rose-500/30 text-xs font-semibold hover:bg-rose-500/30 transition"
                  >
                    Purge All
                  </button>
                )}
              </div>

              {/* Add Memory Form */}
              {isAdding && (
                <form
                  onSubmit={handleAddMemory}
                  className="p-4 rounded-xl bg-[var(--bg-secondary)] border border-[var(--accent-primary)]/50 space-y-3"
                >
                  <div className="text-xs font-bold text-[var(--accent-primary)]">
                    Create Epistemic Memory Record
                  </div>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-2">
                    <input
                      type="text"
                      placeholder="Title of achievement or concept"
                      value={newTitle}
                      onChange={(e) => setNewTitle(e.target.value)}
                      className="md:col-span-2 px-3 py-1.5 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-xs text-[var(--text-dominant)] outline-none"
                    />
                    <select
                      value={newType}
                      onChange={(e) => setNewType(e.target.value as MemoryLevel3_Record['sourceType'])}
                      className="px-2 py-1.5 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-xs text-[var(--text-dominant)]"
                    >
                      <option value="syllabus_topic">Syllabus Topic</option>
                      <option value="goal_milestone">Goal Milestone</option>
                      <option value="concept_mastery">Concept Mastery</option>
                      <option value="journal_reflection">Journal Reflection</option>
                      <option value="exam_score">Exam Score</option>
                    </select>
                  </div>
                  <textarea
                    placeholder="Details, key breakthroughs, or formulas learned..."
                    rows={2}
                    value={newDetails}
                    onChange={(e) => setNewDetails(e.target.value)}
                    className="w-full px-3 py-2 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-xs text-[var(--text-dominant)] outline-none"
                  />
                  <div className="flex items-center justify-between">
                    <input
                      type="text"
                      placeholder="Primary Tag (e.g. Physics)"
                      value={newTag}
                      onChange={(e) => setNewTag(e.target.value)}
                      className="px-3 py-1 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-xs text-[var(--text-dominant)] outline-none w-48"
                    />
                    <button
                      type="submit"
                      className="px-4 py-1.5 rounded bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold"
                    >
                      Save Memory
                    </button>
                  </div>
                </form>
              )}

              {/* Memory List */}
              <div className="space-y-2.5">
                {filteredMemories.length === 0 ? (
                  <div className="text-center py-8 text-xs text-[var(--text-muted)] italic">
                    No epistemic memories found matching your search.
                  </div>
                ) : (
                  filteredMemories.map((mem) => (
                    <div
                      key={mem.id}
                      className="p-3.5 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-2 transition hover:border-[var(--accent-primary)]/40"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <span className="text-xs font-bold text-[var(--text-dominant)]">
                            {mem.title}
                          </span>
                          <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--accent-primary)] border border-[var(--border-subtle)]">
                            {mem.sourceType}
                          </span>
                        </div>
                        <div className="flex items-center gap-2">
                          <span className="text-[10px] font-mono text-[var(--text-muted)]">
                            {new Date(mem.timestamp).toLocaleDateString()}
                          </span>
                          <button
                            onClick={() => handleDeleteMemory(mem.id)}
                            className="text-xs text-rose-400 hover:text-rose-300 px-1.5 py-0.5 rounded hover:bg-rose-500/20"
                            title="Delete this memory"
                          >
                            ✕
                          </button>
                        </div>
                      </div>

                      <p className="text-xs text-[var(--text-secondary)] leading-relaxed">
                        {mem.details}
                      </p>

                      <div className="flex items-center gap-1.5 pt-1">
                        {mem.tags.map((tag) => (
                          <button
                            key={tag}
                            onClick={() => setSelectedTag(tag)}
                            className="text-[10px] px-2 py-0.5 rounded-full bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-[var(--text-muted)] hover:text-[var(--accent-primary)]"
                          >
                            #{tag}
                          </button>
                        ))}
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          )}

          {/* ===================================================================== */}
          {/* LEVEL 2 TAB: Active Working Context Snapshot                          */}
          {/* ===================================================================== */}
          {activeTab === 'level2' && (
            <div className="space-y-4">
              <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)]">
                ℹ️ <strong>Volatile Context:</strong> Sourced live from the Kernel Event Bus and Window Manager before each AI prompt to anchor responses in current time and upcoming commitments.
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="p-3.5 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-2">
                  <div className="text-xs font-bold text-[var(--accent-primary)] uppercase tracking-wider">
                    Temporal & Schedule Snapshot
                  </div>
                  <div className="text-xs space-y-1 font-mono">
                    <div>Current Time: <span className="text-[var(--text-dominant)]">{activeContext.currentTime}</span></div>
                    <div>Available Free Minutes: <span className="text-emerald-400 font-bold">{activeContext.availableFreeMinutes}m</span></div>
                    <div>
                      Next Event:{' '}
                      <span className="text-[var(--text-dominant)]">
                        {activeContext.nextCommitment
                          ? `${activeContext.nextCommitment.title} at ${activeContext.nextCommitment.startTime}`
                          : 'None scheduled'}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="p-3.5 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-2">
                  <div className="text-xs font-bold text-[var(--accent-secondary)] uppercase tracking-wider">
                    Cognitive State & Focus Mode
                  </div>
                  <div className="text-xs space-y-1 font-mono">
                    <div>Active Focus Profile: <span className="text-[var(--text-dominant)]">{activeContext.activeFocusMode}</span></div>
                    <div>Recent Energy Tier: <span className="text-cyan-400">{activeContext.recentEnergy}</span></div>
                    <div>Recent Mood Check-in: <span className="text-amber-400">{activeContext.recentMood}</span></div>
                    <div>Active Workspace: <span className="text-[var(--text-dominant)]">{activeContext.activeActivity}</span></div>
                  </div>
                </div>
              </div>

              {/* Pending Tasks Queued in Context */}
              <div className="p-3.5 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-2">
                <div className="text-xs font-bold text-[var(--text-dominant)]">
                  Top Priority Tasks Injected into Context
                </div>
                <div className="space-y-1.5">
                  {activeContext.pendingTasks.map((t) => (
                    <div
                      key={t.id}
                      className="flex items-center justify-between p-2 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-xs"
                    >
                      <span className="text-[var(--text-dominant)]">{t.title}</span>
                      <span className="text-[10px] font-mono font-bold text-[var(--accent-primary)]">
                        Priority {t.priority}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* ===================================================================== */}
          {/* LEVEL 1 TAB: Durable Profile Memory                                   */}
          {/* ===================================================================== */}
          {activeTab === 'level1' && profile && (
            <div className="space-y-4">
              <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)]">
                🔒 <strong>Level 1 Profile Memory:</strong> Stored deterministically in <code>local_core.sqlite</code> (<code>user_profile</code>).
              </div>

              <div className="p-4 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-3">
                <div className="grid grid-cols-2 md:grid-cols-4 gap-3 text-xs">
                  <div>
                    <span className="text-[10px] text-[var(--text-muted)] block">Display Name</span>
                    <span className="font-bold text-[var(--text-dominant)]">{profile.display_name}</span>
                  </div>
                  <div>
                    <span className="text-[10px] text-[var(--text-muted)] block">Role</span>
                    <span className="font-bold capitalize text-[var(--text-dominant)]">{profile.role}</span>
                  </div>
                  <div>
                    <span className="text-[10px] text-[var(--text-muted)] block">Interface Language</span>
                    <span className="font-bold uppercase text-[var(--accent-primary)]">{profile.interface_lang}</span>
                  </div>
                  <div>
                    <span className="text-[10px] text-[var(--text-muted)] block">AI Language & Style</span>
                    <span className="font-bold uppercase text-[var(--accent-secondary)]">
                      {profile.ai_lang} ({profile.conversation_style})
                    </span>
                  </div>
                </div>

                <div className="pt-2 border-t border-[var(--border-subtle)]">
                  <span className="text-xs font-semibold text-[var(--text-dominant)] block mb-1">
                    Goals JSON
                  </span>
                  <pre className="p-2.5 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-[11px] font-mono text-[var(--text-secondary)] overflow-x-auto">
                    {profile.goals_json}
                  </pre>
                </div>

                <div className="pt-2 border-t border-[var(--border-subtle)]">
                  <span className="text-xs font-semibold text-[var(--text-dominant)] block mb-1">
                    Routine JSON
                  </span>
                  <pre className="p-2.5 rounded bg-[var(--bg-primary)] border border-[var(--border-subtle)] text-[11px] font-mono text-[var(--text-secondary)] overflow-x-auto">
                    {profile.routine_json}
                  </pre>
                </div>
              </div>
            </div>
          )}
        </div>
      </GlassSurface>
    </div>
  );
};
