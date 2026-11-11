/**
 * MindSet OS — Daily Reflective Journal & Gratitude Log
 * Distraction-free Markdown editor with Daily Gratitude, Learning Milestones & SQLite persistence
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import { Button } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';
import type { JournalEntry } from '@mindset/db-schema';

export const JournalPage: React.FC = () => {
  const [entries, setEntries] = useState<JournalEntry[]>([]);
  const [selectedEntry, setSelectedEntry] = useState<JournalEntry | null>(null);
  const [isEditingNew, setIsEditingNew] = useState<boolean>(true);

  // Editor fields
  const [title, setTitle] = useState<string>('Evening Learning & Gratitude Reflection');
  const [content, setContent] = useState<string>(
    `### 🌟 What went well today?
- Completed 3 deep work blocks on Physics Rotational Dynamics.
- Maintained calm focus with 4-7-8 breathing between sessions.

### 🧠 Core Academic & Personal Insight:
- Deriving angular momentum from first principles makes moment of inertia intuitive.

### 🌿 Tomorrow's Single Most Important Focus:
- Solve 15 numerical problems from Chapter 4 without looking at solutions first.`
  );
  const [gratitude1, setGratitude1] = useState<string>('Supportive tea from parents during study');
  const [gratitude2, setGratitude2] = useState<string>('Clear morning weather for fresh air walk');
  const [gratitude3, setGratitude3] = useState<string>('Progress on difficult physics concepts');
  const [moodTag, setMoodTag] = useState<string>('Calm');
  const [energyLevel, setEnergyLevel] = useState<'Low' | 'Medium' | 'High' | 'Hyper'>('Medium');
  const [isEncrypted, setIsEncrypted] = useState<boolean>(true);
  const [isSavedToast, setIsSavedToast] = useState<boolean>(false);

  // Load existing journals
  useEffect(() => {
    loadEntries();
  }, []);

  const loadEntries = async () => {
    try {
      const list = await dbClient.wellbeing.listJournals();
      setEntries(list);
    } catch (err) {
      console.error('[JournalPage] Failed to load entries:', err);
    }
  };

  const handleSave = async () => {
    try {
      const gratitudeArr = [gratitude1, gratitude2, gratitude3].filter(Boolean);
      await dbClient.wellbeing.createJournal({
        title,
        content_md: content,
        gratitude_items_json: JSON.stringify(gratitudeArr),
        mood_tag: moodTag,
        energy_level: energyLevel,
        is_encrypted: isEncrypted ? 1 : 0,
      });

      setIsSavedToast(true);
      await loadEntries();
      setTimeout(() => setIsSavedToast(false), 3000);
    } catch (err) {
      console.error('[JournalPage] Save error:', err);
    }
  };

  const startNewEntry = () => {
    setSelectedEntry(null);
    setIsEditingNew(true);
    setTitle(`Daily Reflection — ${new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' })}`);
    setContent('### 🌟 What went well today?\n\n### 🧠 Key Concept Learned:\n\n### 🌿 Mindset & Tomorrow\'s Priority:\n');
    setGratitude1('');
    setGratitude2('');
    setGratitude3('');
  };

  const viewEntry = (entry: JournalEntry) => {
    setSelectedEntry(entry);
    setIsEditingNew(false);
  };

  return (
    <div className="h-full flex flex-col md:flex-row p-6 gap-6 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] overflow-hidden">
      {/* LEFT SIDEBAR: Past Entries & Gratitude Archive */}
      <div className="w-full md:w-72 flex flex-col space-y-3 border-r border-[var(--border-subtle)] pr-4 overflow-y-auto">
        <div className="flex items-center justify-between pb-2 border-b border-[var(--border-subtle)]">
          <div>
            <h2 className="text-sm font-bold flex items-center space-x-1.5">
              <span>✍️</span>
              <span>Reflections</span>
            </h2>
            <p className="text-[10px] text-[var(--text-muted)]">Encrypted in local SQLite</p>
          </div>

          <Button variant="primary" size="sm" onClick={startNewEntry} className="text-xs font-semibold px-2.5">
            + New
          </Button>
        </div>

        <div className="space-y-2 flex-1">
          {entries.length === 0 ? (
            <div className="text-xs text-[var(--text-muted)] p-4 text-center">
              No journal entries yet. Capture your first reflection today!
            </div>
          ) : (
            entries.map((entry) => (
              <div
                key={entry.id}
                onClick={() => viewEntry(entry)}
                className={`p-3 rounded-xl border text-left cursor-pointer transition-all ${
                  selectedEntry?.id === entry.id
                    ? 'bg-[var(--accent-primary)]/10 border-[var(--accent-primary)]'
                    : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] hover:border-[var(--accent-primary)]/50'
                }`}
              >
                <div className="text-xs font-bold truncate text-[var(--text-dominant)]">
                  {entry.title || 'Untitled Entry'}
                </div>
                <div className="flex items-center justify-between text-[10px] text-[var(--text-muted)] mt-1">
                  <span>{new Date(entry.created_at).toLocaleDateString()}</span>
                  <span>{entry.mood_tag ? `Mood: ${entry.mood_tag}` : 'Calm'}</span>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      {/* RIGHT MAIN: Markdown Editor or View Pane */}
      <div className="flex-1 flex flex-col space-y-4 overflow-y-auto pl-2">
        {/* Editor Controls & Metadata */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between pb-3 border-b border-[var(--border-subtle)] gap-3">
          <input
            type="text"
            value={isEditingNew ? title : selectedEntry?.title || ''}
            onChange={(e) => setTitle(e.target.value)}
            disabled={!isEditingNew}
            placeholder="Journal Title..."
            className="text-base font-bold bg-transparent border-none focus:outline-none text-[var(--text-dominant)] w-full"
          />

          <div className="flex items-center space-x-2">
            {/* Mood selector */}
            <select
              value={moodTag}
              onChange={(e) => setMoodTag(e.target.value)}
              disabled={!isEditingNew}
              className="text-xs px-2.5 py-1.5 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
            >
              {['Calm', 'Focused', 'Energized', 'Grateful', 'Stressed', 'Tired'].map((m) => (
                <option key={m} value={m}>
                  {m}
                </option>
              ))}
            </select>

            {/* Energy selector */}
            <select
              value={energyLevel}
              onChange={(e) => setEnergyLevel(e.target.value as any)}
              disabled={!isEditingNew}
              className="text-xs px-2.5 py-1.5 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
            >
              {['High', 'Medium', 'Low', 'Hyper'].map((e) => (
                <option key={e} value={e}>
                  {e} Energy
                </option>
              ))}
            </select>

            {/* Encryption toggle */}
            <button
              type="button"
              onClick={() => setIsEncrypted(!isEncrypted)}
              className={`text-xs px-2.5 py-1.5 rounded-lg font-semibold border transition-all ${
                isEncrypted
                  ? 'bg-[#65E6C4]/15 border-[#65E6C4]/40 text-[#65E6C4]'
                  : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] text-[var(--text-muted)]'
              }`}
              title="Encrypted in local SQLite"
            >
              🔒 {isEncrypted ? 'Encrypted' : 'Plain'}
            </button>

            {isEditingNew && (
              <Button variant="primary" size="sm" onClick={handleSave} className="text-xs font-semibold px-4">
                Save
              </Button>
            )}
          </div>
        </div>

        {/* Save confirmation toast */}
        {isSavedToast && (
          <div className="p-2.5 rounded-xl bg-[#65E6C4]/15 border border-[#65E6C4]/40 text-xs text-[#65E6C4] font-semibold animate-fadeIn">
            ✓ Journal reflection securely encrypted and saved to SQLite!
          </div>
        )}

        {/* Daily 3 Gratitudes Section */}
        {isEditingNew && (
          <div className="p-3.5 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
            <span className="text-[11px] font-bold uppercase tracking-wider text-[var(--accent-secondary)] flex items-center space-x-1">
              <span>🙏</span>
              <span>Three Daily Moments of Gratitude:</span>
            </span>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
              <input
                type="text"
                placeholder="1. A simple comfort..."
                value={gratitude1}
                onChange={(e) => setGratitude1(e.target.value)}
                className="text-xs px-3 py-1.5 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
              />
              <input
                type="text"
                placeholder="2. An everyday helper..."
                value={gratitude2}
                onChange={(e) => setGratitude2(e.target.value)}
                className="text-xs px-3 py-1.5 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
              />
              <input
                type="text"
                placeholder="3. Personal progress..."
                value={gratitude3}
                onChange={(e) => setGratitude3(e.target.value)}
                className="text-xs px-3 py-1.5 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
              />
            </div>
          </div>
        )}

        {/* Markdown Text Area */}
        <div className="flex-1 flex flex-col space-y-2">
          <textarea
            value={isEditingNew ? content : selectedEntry?.content_md || ''}
            onChange={(e) => setContent(e.target.value)}
            disabled={!isEditingNew}
            rows={14}
            className="w-full flex-1 p-4 rounded-2xl bg-[var(--surface-secondary)]/60 border border-[var(--border-subtle)] text-xs font-mono leading-relaxed text-[var(--text-dominant)] focus:outline-none resize-none"
            placeholder="Write your study notes, reflections, or insights here in Markdown..."
          />

          <div className="flex items-center justify-between text-[11px] text-[var(--text-muted)] font-mono px-2">
            <span>
              Words: {(isEditingNew ? content : selectedEntry?.content_md || '').split(/\s+/).filter(Boolean).length}
            </span>
            <span>Zero cloud syncing • 100% sovereign offline</span>
          </div>
        </div>
      </div>
    </div>
  );
};
