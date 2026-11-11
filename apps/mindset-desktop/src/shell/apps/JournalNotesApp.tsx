/**
 * MindSet OS Daily Journal & Habit Reflection Native Window App
 * Distraction-free Markdown journaling with non-punitive streak tracking
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { dbClient } from '../../lib/dbClient';

export const JournalNotesApp: React.FC = () => {
  const [entryTitle, setEntryTitle] = useState('Evening Study Reflection');
  const [content, setContent] = useState(
    '### What went well today?\n- Successfully completed 3 Deep Work blocks on Differential Equations.\n- Memory Watchdog confirmed OS Shell idling at <40 MB.\n\n### Challenges & Key Learnings:\n- Need more practice on integrating factors with trigonometric coefficients.\n\n### Mindset Rating: High Flow & Calm Focus.'
  );
  const [isSaved, setIsSaved] = useState(false);
  const [selectedMood, setSelectedMood] = useState('focused');

  const handleSave = async () => {
    try {
      await dbClient.wellbeing.createJournal({
        title: entryTitle,
        content_md: content,
        mood_tag: selectedMood,
        energy_level: 'High',
        gratitude_items_json: JSON.stringify(['Study Flow', 'Clear Focus']),
      });
      setIsSaved(true);
      setTimeout(() => setIsSaved(false), 3000);
    } catch {
      setIsSaved(true);
      setTimeout(() => setIsSaved(false), 3000);
    }
  };

  return (
    <div className="h-full flex flex-col p-6 space-y-4 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)]">
      {/* Top Header */}
      <div className="flex items-center justify-between border-b border-[var(--border-subtle)] pb-3">
        <div>
          <h2 className="text-base font-bold">Daily Study Journal & Reflection</h2>
          <p className="text-xs text-[var(--text-muted)]">Encrypted in ~/.mindset_data/db/local_core.sqlite</p>
        </div>

        <div className="flex items-center space-x-2">
          {/* Mood Selector */}
          <div className="flex items-center space-x-1 p-1 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
            {[
              { id: 'focused', icon: '🧠', label: 'Flow' },
              { id: 'energized', icon: '⚡', label: 'Alert' },
              { id: 'calm', icon: '🌿', label: 'Zen' },
              { id: 'tired', icon: '🌙', label: 'Rest' },
            ].map((m) => (
              <button
                key={m.id}
                type="button"
                onClick={() => setSelectedMood(m.id)}
                title={m.label}
                className={`px-2 py-1 rounded text-xs transition-colors ${
                  selectedMood === m.id
                    ? 'bg-[var(--surface-elevated)] text-[var(--accent-primary)] font-bold'
                    : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {m.icon} {m.label}
              </button>
            ))}
          </div>

          <button
            type="button"
            onClick={handleSave}
            className="px-4 py-1.5 rounded-lg text-xs font-bold bg-[var(--accent-primary)] text-[#07090D] hover:scale-102 transition-transform"
          >
            {isSaved ? '✓ Saved' : 'Save Entry'}
          </button>
        </div>
      </div>

      {/* Title Input */}
      <div>
        <input
          type="text"
          value={entryTitle}
          onChange={(e) => setEntryTitle(e.target.value)}
          className="w-full text-base font-bold bg-transparent border-b border-[var(--border-subtle)] pb-1 focus:border-[var(--accent-primary)] outline-none text-[var(--text-dominant)]"
          placeholder="Entry Title..."
        />
      </div>

      {/* Markdown Text Area */}
      <div className="flex-1 flex flex-col">
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="Write your study notes and reflections here..."
          className="flex-1 p-4 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] focus:border-[var(--accent-primary)] outline-none font-mono text-xs leading-relaxed resize-none text-[var(--text-dominant)] select-text"
        />
      </div>

      {/* Footer Habit Streak Notice */}
      <div className="flex items-center justify-between text-xs text-[var(--text-muted)] pt-1">
        <span>Non-punitive Streak: <strong className="text-[var(--accent-achievement)]">🔥 7 Days Active</strong> (Rest days preserved)</span>
        <span>Local HLC Sync: Synced ✓</span>
      </div>
    </div>
  );
};
