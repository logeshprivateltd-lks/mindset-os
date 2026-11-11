/**
 * MindSet OS — Private Mood & Energy Check-in Engine
 * 9 Mood States + 4 Energy Tiers with SQLite persistence & instant restorative guidance
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';
import type { MoodState, EnergyTier } from '@mindset/balance-engine';

export interface MoodCheckinModalProps {
  isOpen?: boolean;
  onClose?: () => void;
  onSaved?: (mood: MoodState, energy: EnergyTier) => void;
}

const MOOD_OPTIONS: { id: MoodState; label: string; icon: string; prompt: string }[] = [
  { id: 'Happy', label: 'Happy', icon: '😊', prompt: 'Great time to share positive energy or celebrate a milestone.' },
  { id: 'Calm', label: 'Calm', icon: '🌿', prompt: 'Optimal neural state for deep reading, reflection, or steady work.' },
  { id: 'Excited', label: 'Excited', icon: '⚡', prompt: 'High dopamine! Channel this momentum into creative problem solving.' },
  { id: 'Curious', label: 'Curious', icon: '🔭', prompt: 'Explore an intriguing science documentary or non-syllabus chapter.' },
  { id: 'Creative', label: 'Creative', icon: '🎨', prompt: 'Perfect time for a 10-minute sketch or musical exploration.' },
  { id: 'Bored', label: 'Bored', icon: '🥱', prompt: 'Under-stimulated? Try a novel 10-minute beginner hobby challenge.' },
  { id: 'Tired', label: 'Tired', icon: '😴', prompt: 'Physical recovery needed. Drink water and take a screen-free rest.' },
  { id: 'Stressed', label: 'Stressed', icon: '😣', prompt: 'High cortisol detected. Try 4-7-8 breathwork to lower your heart rate.' },
  { id: 'Unmotivated', label: 'Unmotivated', icon: '🌫️', prompt: 'Low dopamine is normal. Take a gentle walk with zero expectations.' },
];

const ENERGY_OPTIONS: { id: EnergyTier; label: string; icon: string; desc: string }[] = [
  { id: 'HIGH', label: 'High Alert', icon: '🔋', desc: 'Peak stamina, ready for intense problem solving' },
  { id: 'MEDIUM', label: 'Balanced', icon: '⚡', desc: 'Steady focus, capable of consistent study' },
  { id: 'LOW', label: 'Low Energy', icon: '🪫', desc: 'Fatigued, best for light review or passive audio' },
  { id: 'VERY_LOW', label: 'Depleted', icon: '🛑', desc: 'Needs immediate rest, nap, or parasympathetic reset' },
];

export const MoodCheckinModal: React.FC<MoodCheckinModalProps> = ({
  isOpen = true,
  onClose,
  onSaved,
}) => {
  const [selectedMood, setSelectedMood] = useState<MoodState>('Calm');
  const [selectedEnergy, setSelectedEnergy] = useState<EnergyTier>('MEDIUM');
  const [note, setNote] = useState<string>('');
  const [isSaved, setIsSaved] = useState<boolean>(false);

  if (!isOpen) return null;

  const currentMoodObj = MOOD_OPTIONS.find((m) => m.id === selectedMood) || MOOD_OPTIONS[1]!;

  const handleSaveCheckin = async () => {
    try {
      // Map 9-mood to db-schema MoodCheckin
      const dbMoodMap: Record<MoodState, any> = {
        Happy: 'Energized',
        Calm: 'Calm',
        Excited: 'Energized',
        Curious: 'Focused',
        Creative: 'Focused',
        Bored: 'Fatigued',
        Tired: 'Fatigued',
        Stressed: 'Anxious',
        Unmotivated: 'Overwhelmed',
      };

      const dbEnergyMap: Record<EnergyTier, any> = {
        HIGH: 'High',
        MEDIUM: 'Medium',
        LOW: 'Low',
        VERY_LOW: 'Low',
      };

      await dbClient.wellbeing.createMood({
        mood: dbMoodMap[selectedMood] || 'Calm',
        energy_tier: dbEnergyMap[selectedEnergy] || 'Medium',
        context_note: `${selectedMood}: ${note.trim() || 'Quick check-in'}`,
        active_activity: null,
      });

      setIsSaved(true);
      if (onSaved) onSaved(selectedMood, selectedEnergy);

      setTimeout(() => {
        setIsSaved(false);
        if (onClose) onClose();
      }, 1500);
    } catch (err) {
      console.error('[MoodCheckin] Error saving:', err);
      if (onClose) onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xl animate-fadeIn">
      <GlassSurface variant="modal" className="max-w-xl w-full p-6 space-y-5 animate-scaleUp">
        {/* Header */}
        <div className="flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]">
          <div className="flex items-center space-x-2">
            <span className="text-xl">🌈</span>
            <div>
              <h3 className="text-sm font-bold text-[var(--text-dominant)]">
                Private Mood & Energy Check-in
              </h3>
              <p className="text-[11px] text-[var(--text-muted)]">
                Calibrates Balance AI recommendations • 100% sovereign offline
              </p>
            </div>
          </div>

          {onClose && (
            <button
              type="button"
              onClick={onClose}
              className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1 rounded-lg"
            >
              ✕
            </button>
          )}
        </div>

        {/* 1. Nine Mood States Grid */}
        <div className="space-y-2">
          <label className="text-xs font-bold text-[var(--text-muted)] block">
            1. How are you feeling right now?
          </label>
          <div className="grid grid-cols-3 sm:grid-cols-3 gap-2">
            {MOOD_OPTIONS.map((opt) => (
              <button
                key={opt.id}
                type="button"
                onClick={() => setSelectedMood(opt.id)}
                className={`p-2.5 rounded-xl border text-left flex items-center space-x-2 transition-all ${
                  selectedMood === opt.id
                    ? 'bg-[var(--accent-primary)]/15 border-[var(--accent-primary)] text-[var(--text-dominant)] font-bold shadow-sm'
                    : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                <span className="text-lg">{opt.icon}</span>
                <span className="text-xs">{opt.label}</span>
              </button>
            ))}
          </div>
        </div>

        {/* 2. Four Energy Tiers */}
        <div className="space-y-2">
          <label className="text-xs font-bold text-[var(--text-muted)] block">
            2. Current Energy Reservoir:
          </label>
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-2">
            {ENERGY_OPTIONS.map((opt) => (
              <button
                key={opt.id}
                type="button"
                onClick={() => setSelectedEnergy(opt.id)}
                className={`p-2 rounded-xl border text-center transition-all ${
                  selectedEnergy === opt.id
                    ? 'bg-[#65E6C4]/15 border-[#65E6C4] text-[#65E6C4] font-bold shadow-sm'
                    : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                <span className="text-lg block mb-0.5">{opt.icon}</span>
                <span className="text-xs block font-bold">{opt.label}</span>
                <span className="text-[9px] block text-[var(--text-muted)] opacity-80 mt-0.5 truncate">
                  {opt.desc}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Dynamic Coach Guidance */}
        <div className="p-3 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs text-[var(--accent-secondary)] flex items-start space-x-2">
          <span className="text-base">💡</span>
          <div>
            <span className="font-bold">MindSet Coach Observation: </span>
            <span>{currentMoodObj.prompt}</span>
          </div>
        </div>

        {/* Optional Context Note */}
        <div>
          <input
            type="text"
            placeholder="Optional context (e.g. Just completed Chapter 3, feeling a bit mentally saturated)..."
            value={note}
            onChange={(e) => setNote(e.target.value)}
            className="w-full text-xs px-3 py-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
          />
        </div>

        {/* Action Controls */}
        <div className="flex items-center justify-between pt-2">
          <span className="text-[11px] font-mono text-[var(--text-muted)]">
            {isSaved ? '✓ Saved to SQLite' : 'Logged to Timeline'}
          </span>

          <div className="flex items-center space-x-2">
            {onClose && (
              <Button variant="secondary" size="sm" onClick={onClose} className="text-xs font-semibold">
                Skip
              </Button>
            )}
            <Button
              variant="primary"
              size="sm"
              onClick={handleSaveCheckin}
              className="text-xs font-bold px-5"
            >
              {isSaved ? '✓ Recorded!' : 'Save Check-in'}
            </Button>
          </div>
        </div>
      </GlassSurface>
    </div>
  );
};
