/**
 * MindSet OS — Balance AI & Digital Wellbeing Hub
 * 4-7-8 & Box Breathing Visualizer, Free-Time Gap Planner, 12-Hobby Hub, and Intentional Entertainment Manager
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useMemo } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import {
  FreeTimeDetector,
  SmartFreeTimePlanner,
  ActivityRecommender,
  HobbyCategory,
  FreeTimeSlot,
  MicroBreakPlan,
  EnergyTier,
  MoodState,
} from '@mindset/balance-engine';
import { dbClient } from '../../lib/dbClient';

export type BalanceTab = 'breathing' | 'freetime' | 'hobbies' | 'entertainment';

export const BalanceAiPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<BalanceTab>('breathing');
  const [breathingMode, setBreathingMode] = useState<'4-7-8' | 'box'>('4-7-8');
  const [breathPhase, setBreathPhase] = useState<'Inhale' | 'Hold' | 'Exhale' | 'HoldEmpty'>('Inhale');
  const [breathSeconds, setBreathSeconds] = useState<number>(4);
  const [isBreathingActive, setIsBreathingActive] = useState<boolean>(false);

  // Hobby explorer state
  const [selectedHobbyCat, setSelectedHobbyCat] = useState<HobbyCategory | 'all'>('all');
  const [completedChallenges, setCompletedChallenges] = useState<string[]>([]);

  // Free-time planner state
  const [detectedSlots, setDetectedSlots] = useState<FreeTimeSlot[]>([]);
  const [activePlan, setActivePlan] = useState<MicroBreakPlan | null>(null);
  const [selectedEnergy, setSelectedEnergy] = useState<EnergyTier>('MEDIUM');
  const [selectedMood, setSelectedMood] = useState<MoodState>('Calm');

  // Intentional entertainment state
  const [entertainmentDuration, setEntertainmentDuration] = useState<number>(45);
  const [selectedActivityName, setSelectedActivityName] = useState<string>('Curated Indie Game / Chess');
  const [, setEntertainmentActive] = useState<boolean>(false);
  const [allowanceToast, setAllowanceToast] = useState<string | null>(null);

  const recommender = useMemo(() => new ActivityRecommender(), []);
  const planner = useMemo(() => new SmartFreeTimePlanner(recommender), [recommender]);
  const detector = useMemo(() => new FreeTimeDetector(), []);

  // Detect free slots from mock/daily tasks
  useEffect(() => {
    // Generate realistic daily blocks
    const sampleBlocks = [
      { id: 'sb-1', title: 'School / Morning Lectures', startTime: '08:30', endTime: '12:30', isMandatory: true },
      { id: 'sb-2', title: 'Lunch & Family Time', startTime: '12:30', endTime: '13:30', isMandatory: false },
      { id: 'sb-3', title: 'Physics: Rotational Dynamics', startTime: '14:00', endTime: '16:00', isMandatory: true },
      { id: 'sb-4', title: 'Maths Problem Set Practice', startTime: '17:30', endTime: '19:15', isMandatory: true },
      { id: 'sb-5', title: 'Dinner & Family Catch-Up', startTime: '19:30', endTime: '20:30', isMandatory: true },
    ];

    const slots = detector.detectFreeSlots(sampleBlocks);
    setDetectedSlots(slots);
    if (slots.length > 0) {
      const firstSlot = slots[0]!;
      const plan = planner.generatePlan(firstSlot, selectedEnergy, selectedMood);
      setActivePlan(plan);
    }
  }, [detector, planner, selectedEnergy, selectedMood]);

  // Guided breathing loop
  useEffect(() => {
    if (!isBreathingActive) return;

    const timer = setInterval(() => {
      setBreathSeconds((prev) => {
        if (prev <= 1) {
          if (breathingMode === '4-7-8') {
            if (breathPhase === 'Inhale') {
              setBreathPhase('Hold');
              return 7;
            } else if (breathPhase === 'Hold') {
              setBreathPhase('Exhale');
              return 8;
            } else {
              setBreathPhase('Inhale');
              return 4;
            }
          } else {
            // Box Breathing 4-4-4-4
            if (breathPhase === 'Inhale') {
              setBreathPhase('Hold');
              return 4;
            } else if (breathPhase === 'Hold') {
              setBreathPhase('Exhale');
              return 4;
            } else if (breathPhase === 'Exhale') {
              setBreathPhase('HoldEmpty');
              return 4;
            } else {
              setBreathPhase('Inhale');
              return 4;
            }
          }
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [isBreathingActive, breathPhase, breathingMode]);

  const toggleBreathing = () => {
    setIsBreathingActive(!isBreathingActive);
    if (!isBreathingActive) {
      setBreathPhase('Inhale');
      setBreathSeconds(4);
    }
  };

  const handleToggleChallenge = (id: string) => {
    setCompletedChallenges((prev) =>
      prev.includes(id) ? prev.filter((c) => c !== id) : [...prev, id]
    );
  };

  // Handshake with Focus Guard for Intentional Entertainment
  const handleGrantIntentionalAllowance = async () => {
    setEntertainmentActive(true);
    setAllowanceToast(
      `🛡️ Focus Guard Handshake: Allowed "${selectedActivityName}" for ${entertainmentDuration} minutes without interruption.`
    );

    try {
      // Record to SQLite timeline and journal reflection
      await dbClient.wellbeing.createJournal({
        title: `Planned Relaxation: ${selectedActivityName}`,
        content_md: `Enjoying a deliberate ${entertainmentDuration}-minute intentional rest block. Guilt-free, authorized enjoyment.`,
        mood_tag: selectedMood,
        energy_level: 'Medium',
        gratitude_items_json: JSON.stringify(['Permission to rest', 'Intentional play']),
      });
    } catch {
      // Ignore
    }

    setTimeout(() => {
      setAllowanceToast(null);
    }, 6000);
  };

  const filteredChallenges = useMemo(() => {
    return recommender.getHobbyChallenges(selectedHobbyCat === 'all' ? undefined : selectedHobbyCat);
  }, [recommender, selectedHobbyCat]);

  return (
    <div className="h-full flex flex-col p-6 space-y-6 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] overflow-y-auto">
      {/* Top Header & Philosophy Banner */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-[var(--border-subtle)] gap-4">
        <div>
          <div className="flex items-center space-x-2">
            <span className="text-xl">🌿</span>
            <h1 className="text-lg font-bold">MindSet Balance AI</h1>
            <span className="text-[10px] px-2 py-0.5 rounded-full bg-[#65E6C4]/15 text-[#65E6C4] font-semibold border border-[#65E6C4]/30">
              Anti-Burnout System
            </span>
          </div>
          <p className="text-xs text-[var(--text-muted)] italic mt-1">
            "Productivity is not about filling every minute. It is about using time meaningfully while leaving room to live."
          </p>
        </div>

        {/* Navigation Tabs */}
        <div className="flex items-center space-x-1 p-1 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs font-medium">
          {[
            { id: 'breathing', label: '🧘 Breathwork' },
            { id: 'freetime', label: '⏱️ Free-Time Planner' },
            { id: 'hobbies', label: '🎨 Hobby Hub' },
            { id: 'entertainment', label: '🎮 Intentional Play' },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id as BalanceTab)}
              className={`px-3 py-1.5 rounded-lg transition-all ${
                activeTab === tab.id
                  ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] font-bold shadow-sm'
                  : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* Allowance Toast */}
      {allowanceToast && (
        <div className="p-3 rounded-xl bg-[#65E6C4]/15 border border-[#65E6C4]/40 text-xs text-[#65E6C4] font-medium flex items-center justify-between animate-fadeIn">
          <span>{allowanceToast}</span>
          <button
            type="button"
            onClick={() => setAllowanceToast(null)}
            className="text-xs font-bold px-2 py-0.5 hover:underline"
          >
            Dismiss
          </button>
        </div>
      )}

      {/* ============================================================= */}
      {/* TAB 1: GUIDED PARASYMPATHETIC BREATHWORK                      */}
      {/* ============================================================= */}
      {activeTab === 'breathing' && (
        <div className="flex flex-col items-center justify-center space-y-6 py-6 text-center max-w-xl mx-auto">
          {/* Breathing Mode Selector */}
          <div className="flex items-center space-x-2 p-1 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs">
            <button
              type="button"
              onClick={() => {
                setBreathingMode('4-7-8');
                setBreathPhase('Inhale');
                setBreathSeconds(4);
              }}
              className={`px-4 py-1.5 rounded-lg font-semibold transition-all ${
                breathingMode === '4-7-8'
                  ? 'bg-[#65E6C4] text-[#07090D] shadow-sm'
                  : 'text-[var(--text-muted)]'
              }`}
            >
              🌿 4-7-8 Cortisol Relief
            </button>

            <button
              type="button"
              onClick={() => {
                setBreathingMode('box');
                setBreathPhase('Inhale');
                setBreathSeconds(4);
              }}
              className={`px-4 py-1.5 rounded-lg font-semibold transition-all ${
                breathingMode === 'box'
                  ? 'bg-[#8B9CFF] text-[#07090D] shadow-sm'
                  : 'text-[var(--text-muted)]'
              }`}
            >
              🟦 Box Breathing (4-4-4-4)
            </button>
          </div>

          {/* Animated Breathing Circle */}
          <div className="relative flex items-center justify-center w-64 h-64 my-4">
            <div
              className={`absolute inset-0 rounded-full transition-all duration-1000 ease-in-out ${
                isBreathingActive
                  ? breathPhase === 'Inhale'
                    ? 'scale-110 bg-[#65E6C4]/20 blur-2xl'
                    : breathPhase === 'Hold' || breathPhase === 'HoldEmpty'
                    ? 'scale-105 bg-[#8B9CFF]/25 blur-xl'
                    : 'scale-90 bg-transparent blur-md'
                  : 'bg-[var(--surface-secondary)]/50 blur-lg'
              }`}
            />

            <div
              className={`relative flex flex-col items-center justify-center w-52 h-52 rounded-full border-2 transition-all duration-700 ${
                isBreathingActive
                  ? breathPhase === 'Inhale'
                    ? 'border-[#65E6C4] bg-[#65E6C4]/10 shadow-[0_0_30px_rgba(101,230,196,0.3)]'
                    : breathPhase === 'Hold' || breathPhase === 'HoldEmpty'
                    ? 'border-[#8B9CFF] bg-[#8B9CFF]/10 shadow-[0_0_30px_rgba(139,156,255,0.3)]'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)]/80'
                  : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)]'
              }`}
            >
              <span className="text-sm font-semibold tracking-wider uppercase text-[var(--accent-secondary)]">
                {isBreathingActive ? breathPhase : 'Ready'}
              </span>
              <span className="text-5xl font-mono font-black tabular-nums my-1">
                {isBreathingActive ? breathSeconds : '4'}
              </span>
              <span className="text-[11px] text-[var(--text-muted)]">
                {breathingMode === '4-7-8'
                  ? breathPhase === 'Inhale'
                    ? 'Breathe in calmly'
                    : breathPhase === 'Hold'
                    ? 'Gently retain air'
                    : 'Release fully'
                  : 'Steady autonomic pace'}
              </span>
            </div>
          </div>

          <div className="space-y-3">
            <Button
              variant={isBreathingActive ? 'danger' : 'primary'}
              size="md"
              onClick={toggleBreathing}
              className="px-8 font-bold text-sm"
            >
              {isBreathingActive ? '⏹️ End Session' : '▶️ Begin Guided Breathing'}
            </Button>
            <p className="text-[11px] text-[var(--text-muted)] max-w-sm">
              Stimulates the vagus nerve and shifts brain states from sympathetic alert to parasympathetic restorative flow.
            </p>
          </div>
        </div>
      )}

      {/* ============================================================= */}
      {/* TAB 2: SMART FREE-TIME PLANNER                                */}
      {/* ============================================================= */}
      {activeTab === 'freetime' && (
        <div className="space-y-6 max-w-3xl mx-auto">
          {/* Energy & Mood Check Controls */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
            <div>
              <label className="text-xs font-bold text-[var(--text-muted)] block mb-1.5">
                Current Energy Tier:
              </label>
              <div className="flex items-center space-x-1">
                {(['HIGH', 'MEDIUM', 'LOW', 'VERY_LOW'] as EnergyTier[]).map((tier) => (
                  <button
                    key={tier}
                    type="button"
                    onClick={() => setSelectedEnergy(tier)}
                    className={`px-2.5 py-1 text-xs rounded-lg font-semibold transition-all ${
                      selectedEnergy === tier
                        ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                        : 'bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                    }`}
                  >
                    {tier}
                  </button>
                ))}
              </div>
            </div>

            <div>
              <label className="text-xs font-bold text-[var(--text-muted)] block mb-1.5">
                Current Mood State:
              </label>
              <select
                value={selectedMood}
                onChange={(e) => setSelectedMood(e.target.value as MoodState)}
                className="w-full text-xs px-3 py-1.5 rounded-lg bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
              >
                {[
                  'Happy',
                  'Calm',
                  'Excited',
                  'Bored',
                  'Tired',
                  'Stressed',
                  'Unmotivated',
                  'Curious',
                  'Creative',
                ].map((m) => (
                  <option key={m} value={m}>
                    {m}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Detected Open Windows */}
          <div className="space-y-3">
            <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
              Detected Free-Time Windows Today ({detectedSlots.length})
            </h3>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
              {detectedSlots.map((slot, idx) => (
                <div
                  key={idx}
                  onClick={() => {
                    const plan = planner.generatePlan(slot, selectedEnergy, selectedMood);
                    setActivePlan(plan);
                  }}
                  className={`p-3.5 rounded-xl border cursor-pointer transition-all ${
                    activePlan?.slotStartTime === slot.startTime
                      ? 'bg-[#65E6C4]/10 border-[#65E6C4]/50 shadow-sm'
                      : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] hover:border-[var(--accent-primary)]'
                  }`}
                >
                  <div className="flex items-center justify-between text-xs font-bold">
                    <span>
                      {slot.startTime} – {slot.endTime}
                    </span>
                    <span className="text-[#65E6C4]">{slot.usableMinutes}m open</span>
                  </div>
                  <div className="text-[11px] text-[var(--text-muted)] mt-1 flex justify-between">
                    <span>Total: {slot.totalMinutes}m</span>
                    <span>Buffer: {slot.bufferMinutes}m protected</span>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Active Generated Micro-Break Plan */}
          {activePlan && (
            <GlassSurface variant="solid-card" className="p-5 space-y-4 rounded-2xl">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-sm font-bold flex items-center space-x-2">
                    <span>✨</span>
                    <span>{activePlan.title}</span>
                  </h4>
                  <p className="text-xs text-[var(--text-muted)] mt-0.5">
                    Start time: {activePlan.slotStartTime} • Optional & non-judgmental recharge sequence
                  </p>
                </div>

                <Button
                  variant="primary"
                  size="sm"
                  onClick={() => {
                    setAllowanceToast('🌱 Recharge Plan Accepted! Focus Guard adjusted.');
                  }}
                  className="text-xs font-semibold"
                >
                  Accept Plan
                </Button>
              </div>

              {/* Sequential Micro-Phases */}
              <div className="space-y-2.5 pt-2">
                {activePlan.phases.map((phase) => (
                  <div
                    key={phase.phaseIndex}
                    className="flex items-start space-x-3 p-3 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-xs"
                  >
                    <span className="text-lg">{phase.icon}</span>
                    <div className="flex-1">
                      <div className="flex items-center justify-between">
                        <span className="font-bold text-[var(--text-dominant)]">
                          Phase {phase.phaseIndex}: {phase.title}
                        </span>
                        <span className="font-mono text-[10px] px-2 py-0.5 rounded-full bg-[var(--surface-secondary)] text-[var(--text-muted)]">
                          {phase.startMinute}m – {phase.endMinute}m
                        </span>
                      </div>
                      <p className="text-[11px] text-[var(--text-muted)] mt-0.5">{phase.description}</p>
                    </div>
                  </div>
                ))}
              </div>
            </GlassSurface>
          )}
        </div>
      )}

      {/* ============================================================= */}
      {/* TAB 3: PERSONAL HOBBY DISCOVERY HUB                           */}
      {/* ============================================================= */}
      {activeTab === 'hobbies' && (
        <div className="space-y-6">
          {/* Category Filter Pills */}
          <div className="flex items-center space-x-1.5 overflow-x-auto pb-2 text-xs">
            <button
              type="button"
              onClick={() => setSelectedHobbyCat('all')}
              className={`px-3 py-1 rounded-full font-medium whitespace-nowrap transition-all ${
                selectedHobbyCat === 'all'
                  ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] font-bold'
                  : 'bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
              }`}
            >
              All Categories
            </button>

            {[
              { id: 'art', label: '🎨 Art & Sketch' },
              { id: 'tech', label: '💻 Creative Tech' },
              { id: 'music', label: '🎵 Music' },
              { id: 'sports', label: '🏃 Sports' },
              { id: 'reading', label: '📚 Reading' },
              { id: 'photography', label: '📷 Photo' },
              { id: 'gardening', label: '🌱 Garden' },
              { id: 'cooking', label: '🍳 Culinary' },
              { id: 'writing', label: '✍️ Writing' },
              { id: 'outdoor', label: '🌳 Outdoors' },
              { id: 'science', label: '🔭 Science' },
              { id: 'diy', label: '✂️ DIY Origami' },
            ].map((cat) => (
              <button
                key={cat.id}
                type="button"
                onClick={() => setSelectedHobbyCat(cat.id as HobbyCategory)}
                className={`px-3 py-1 rounded-full font-medium whitespace-nowrap transition-all ${
                  selectedHobbyCat === cat.id
                    ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] font-bold'
                    : 'bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {cat.label}
              </button>
            ))}
          </div>

          {/* 10-Minute Beginner Challenges Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filteredChallenges.map((challenge) => {
              const isDone = completedChallenges.includes(challenge.id);
              return (
                <div
                  key={challenge.id}
                  className={`p-4 rounded-2xl border transition-all flex flex-col justify-between space-y-3 ${
                    isDone
                      ? 'bg-[#65E6C4]/5 border-[#65E6C4]/40'
                      : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] hover:border-[var(--accent-primary)]'
                  }`}
                >
                  <div className="space-y-2">
                    <div className="flex items-center justify-between text-xs">
                      <span className="font-mono text-[10px] uppercase font-bold text-[var(--accent-secondary)]">
                        {challenge.category} • {challenge.difficulty}
                      </span>
                      <span className="font-mono text-[10px] text-[var(--text-muted)]">
                        ⏱️ {challenge.durationMinutes}m
                      </span>
                    </div>

                    <h4 className="text-sm font-bold text-[var(--text-dominant)]">{challenge.title}</h4>
                    <p className="text-xs text-[var(--text-muted)] leading-relaxed">{challenge.prompt}</p>
                  </div>

                  <div className="pt-2 border-t border-[var(--border-subtle)]/60 flex items-center justify-between text-xs">
                    <span className="text-[10px] text-[var(--text-muted)] truncate max-w-[160px]">
                      📦 {challenge.materialsNeeded}
                    </span>

                    <button
                      type="button"
                      onClick={() => handleToggleChallenge(challenge.id)}
                      className={`px-2.5 py-1 rounded-lg font-semibold text-[11px] transition-all ${
                        isDone
                          ? 'bg-[#65E6C4] text-[#07090D]'
                          : 'bg-[var(--surface-primary)] border border-[var(--border-subtle)] hover:text-[var(--text-dominant)]'
                      }`}
                    >
                      {isDone ? '✓ Completed' : 'Mark Done'}
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* ============================================================= */}
      {/* TAB 4: INTENTIONAL ENTERTAINMENT BUDGET MANAGER               */}
      {/* ============================================================= */}
      {activeTab === 'entertainment' && (
        <div className="max-w-2xl mx-auto space-y-6">
          <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-3">
            <div className="flex items-center space-x-2">
              <span className="text-xl">🎮</span>
              <div>
                <h3 className="text-sm font-bold">Intentional Entertainment vs. Unintentional Consumption</h3>
                <p className="text-xs text-[var(--text-muted)]">
                  Planned movies, music, or gaming are completely legitimate. This bridge allows Focus Guard to pause distraction shields for your exact planned duration.
                </p>
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <div>
              <label className="text-xs font-bold text-[var(--text-muted)] block mb-1.5">
                Select Planned Activity:
              </label>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                {[
                  { name: 'Curated Indie Game / Chess', icon: '♟️', mins: 30 },
                  { name: 'Planned Movie / Anime Episode', icon: '🎬', mins: 45 },
                  { name: 'Full Album Music Listening', icon: '🎧', mins: 40 },
                  { name: 'Casual Creative Gaming Session', icon: '🕹️', mins: 60 },
                ].map((act) => (
                  <button
                    key={act.name}
                    type="button"
                    onClick={() => {
                      setSelectedActivityName(act.name);
                      setEntertainmentDuration(act.mins);
                    }}
                    className={`p-3 rounded-xl border text-left text-xs font-semibold flex items-center space-x-3 transition-all ${
                      selectedActivityName === act.name
                        ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] border-transparent shadow-sm'
                        : 'bg-[var(--surface-secondary)] border-[var(--border-subtle)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                    }`}
                  >
                    <span className="text-lg">{act.icon}</span>
                    <div className="flex-1">
                      <div className="font-bold">{act.name}</div>
                      <div className="text-[10px] opacity-80">{act.mins} minutes suggested</div>
                    </div>
                  </button>
                ))}
              </div>
            </div>

            <div>
              <div className="flex justify-between text-xs font-bold mb-1">
                <span className="text-[var(--text-muted)]">Session Duration:</span>
                <span className="font-mono text-[#8B9CFF]">{entertainmentDuration} minutes</span>
              </div>
              <input
                type="range"
                min="15"
                max="90"
                step="5"
                value={entertainmentDuration}
                onChange={(e) => setEntertainmentDuration(Number(e.target.value))}
                className="w-full accent-[#8B9CFF] cursor-pointer"
              />
            </div>

            <div className="pt-2">
              <Button
                variant="primary"
                size="md"
                onClick={handleGrantIntentionalAllowance}
                className="w-full font-bold text-xs py-3"
              >
                🛡️ Signal Focus Guard: Allow {entertainmentDuration} Mins Without Interruption
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
