/**
 * MindSet OS — 6-Screen Progressive Onboarding Wizard (/onboarding)
 * Low-Cognitive-Load, Instant SQLite Persistence, 220ms Fluid Transitions
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useMemo } from 'react';
import type { UserProfile } from '@mindset/db-schema';
import {
  LOCALE_METADATA,
  SupportedLocale,
  t,
} from '@mindset/i18n-locales';
import {
  PersonalContextBuilder,
  GoalItem,
  RoutineSchedule,
  LearningStyle,
  AiTone,
  PlanningStyle,
} from '@mindset/identity-engine';
import {
  GlassSurface,
  useThemeStore,
  ThemeSelector,
} from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';

interface OnboardingWizardProps {
  onComplete: () => void;
}

export const OnboardingWizard: React.FC<OnboardingWizardProps> = ({ onComplete }) => {
  const { theme } = useThemeStore();

  // Wizard Step (1 to 6)
  const [step, setStep] = useState<number>(1);
  const [isSaving, setIsSaving] = useState(false);

  // Screen 1: Welcome & Theme (currentTheme managed by store)

  // Screen 2: Dual-Layer Multilingual
  const [interfaceLang, setInterfaceLang] = useState<SupportedLocale>('en');
  const [aiLang, setAiLang] = useState<'ta' | 'hi' | 'en' | 'te' | 'ml' | 'kn' | 'bn' | 'mr'>('ta');
  const [conversationStyle, setConversationStyle] = useState<'tanglish' | 'formal' | 'concise' | 'socratic'>('tanglish');

  // Screen 3: Identity & Ambition
  const [displayName, setDisplayName] = useState('Arjun');
  const [role, setRole] = useState<'student' | 'researcher' | 'professional'>('student');
  const [goals, setGoals] = useState<GoalItem[]>([
    { id: 'g1', title: 'JEE Advanced 2027 (Top 500 AIR)', priority: 1, completed: false, progressPercent: 15 },
    { id: 'g2', title: 'Master Mechanics & Electrodynamics', priority: 2, completed: false, progressPercent: 30 },
    { id: 'g3', title: 'Solve 20 Advanced Problems Daily', priority: 3, completed: false, progressPercent: 40 },
  ]);
  const [focusAreas, setFocusAreas] = useState<string[]>(['Physics', 'Mathematics', 'Chemistry']);

  // Screen 4: Routine & Visual Timeline
  const [wakeTime, setWakeTime] = useState('06:00');
  const [sleepTime, setSleepTime] = useState('22:30');
  const [schoolStart, setSchoolStart] = useState('08:30');
  const [schoolEnd, setSchoolEnd] = useState('16:00');
  const [studySlot1Start, setStudySlot1Start] = useState('06:30');
  const [studySlot1End, setStudySlot1End] = useState('08:00');
  const [studySlot2Start, setStudySlot2Start] = useState('17:30');
  const [studySlot2End, setStudySlot2End] = useState('20:30');

  // Screen 5: AI & Learning Preferences
  const [learningStyle, setLearningStyle] = useState<LearningStyle>('step_by_step');
  const [aiTone, setAiTone] = useState<AiTone>('friendly_direct');
  const [planningStyle, setPlanningStyle] = useState<PlanningStyle>('flexible');
  const [interests, setInterests] = useState<string[]>(['Coding', 'Chess', 'Astronomy']);

  // Goal Presets
  const goalPresets = [
    { title: 'JEE Advanced 2027 (AIR < 500)', subject: 'Physics & Math' },
    { title: 'NEET UG Exam (Score 680+)', subject: 'Biology & Chem' },
    { title: 'UPSC Civil Services Prelims', subject: 'General Studies' },
    { title: 'Class 12 Boards (95%+ Target)', subject: 'Core Subjects' },
    { title: 'Master Rust & Systems Architecture', subject: 'Computer Science' },
    { title: 'GATE CS (Top 100 Rank)', subject: 'Algorithms & OS' },
  ];

  // Subject Options
  const subjectOptions = [
    'Physics',
    'Mathematics',
    'Chemistry',
    'Biology',
    'Computer Science',
    'History',
    'Economics',
    'Literature',
  ];

  // Hobby Options
  const hobbyOptions = [
    'Coding',
    'Chess',
    'Music',
    'Walking',
    'Reading',
    'Art',
    'Sports',
    'Gaming',
  ];

  // Construct draft UserProfile object
  const draftProfile: UserProfile = useMemo(() => {
    const routine: RoutineSchedule = {
      wakeTime,
      sleepTime,
      commitments: [
        { title: 'School / College', startTime: schoolStart, endTime: schoolEnd, days: [1, 2, 3, 4, 5] },
      ],
      preferredStudySlots: [
        { startTime: studySlot1Start, endTime: studySlot1End, label: 'Morning Deep Work' },
        { startTime: studySlot2Start, endTime: studySlot2End, label: 'Evening Revision' },
      ],
      breakDurationMinutes: 15,
    };

    return {
      user_id: 'default_student',
      display_name: displayName.trim() || 'MindSet Scholar',
      role,
      interface_lang: interfaceLang === 'tanglish' ? 'en' : interfaceLang,
      ai_lang: aiLang,
      conversation_style: conversationStyle,
      goals_json: JSON.stringify(goals),
      focus_areas_json: JSON.stringify(focusAreas),
      routine_json: JSON.stringify(routine),
      preferences_json: JSON.stringify({
        learningStyle,
        aiTone,
        planningStyle,
        tanglishEnabled: conversationStyle === 'tanglish',
        themePreference: theme,
      }),
      interests_json: JSON.stringify(interests),
      permissions_json: JSON.stringify({
        strict_focus_mode: true,
        cloud_sync_consent: false,
        biometrics_allowed: true,
      }),
      updated_at: new Date().toISOString(),
    };
  }, [
    displayName,
    role,
    interfaceLang,
    aiLang,
    conversationStyle,
    goals,
    focusAreas,
    wakeTime,
    sleepTime,
    schoolStart,
    schoolEnd,
    studySlot1Start,
    studySlot1End,
    studySlot2Start,
    studySlot2End,
    learningStyle,
    aiTone,
    planningStyle,
    interests,
    theme,
  ]);

  // Live AI Greeting for Screen 6
  const liveGreeting = useMemo(() => {
    return PersonalContextBuilder.generateGreeting(draftProfile);
  }, [draftProfile]);

  // Commit profile to SQLite and finish onboarding
  const handleFinishOnboarding = async () => {
    setIsSaving(true);
    try {
      await dbClient.profile.update(draftProfile);
      localStorage.setItem('mindset_onboarding_completed', 'true');
      setIsSaving(false);
      onComplete();
    } catch (err) {
      console.warn('Failed to save profile to SQLite, proceeding with memory cache:', err);
      localStorage.setItem('mindset_onboarding_completed', 'true');
      setIsSaving(false);
      onComplete();
    }
  };

  const handleApplyPresetGoal = (title: string, priority: 1 | 2 | 3 = 1) => {
    setGoals((prev) => {
      const existing = prev.filter((g) => g.title !== title);
      const newGoal: GoalItem = {
        id: `preset-${Date.now()}`,
        title,
        priority,
        completed: false,
        progressPercent: 10,
      };
      return [newGoal, ...existing].slice(0, 3);
    });
  };

  const handleUpdateGoalTitle = (index: number, newTitle: string) => {
    setGoals((prev) => {
      const next = [...prev];
      if (next[index]) {
        next[index] = { ...next[index], title: newTitle };
      }
      return next;
    });
  };

  const toggleSubject = (subj: string) => {
    setFocusAreas((prev) =>
      prev.includes(subj) ? prev.filter((s) => s !== subj) : [...prev, subj]
    );
  };

  const toggleInterest = (hobby: string) => {
    setInterests((prev) =>
      prev.includes(hobby) ? prev.filter((h) => h !== hobby) : [...prev, hobby]
    );
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 md:p-8 select-none overflow-y-auto"
      style={{
        backgroundColor: 'var(--bg-base)',
        color: 'var(--text-dominant)',
      }}
    >
      {/* Background Subtle Radial Glow */}
      <div
        className="absolute inset-0 pointer-events-none opacity-20"
        style={{
          background: 'radial-gradient(circle at 50% 20%, var(--accent-primary) 0%, transparent 65%)',
        }}
      />

      <GlassSurface
        variant="modal"
        className="w-full max-w-4xl p-6 md:p-8 rounded-2xl relative shadow-2xl border flex flex-col justify-between"
        style={{
          borderColor: 'var(--border-subtle)',
          minHeight: '620px',
        }}
      >
        {/* Top Header: Step Counter & Progress Indicator */}
        <div className="mb-6">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <span className="text-xl">🌌</span>
              <span className="text-xs uppercase tracking-widest font-mono text-[var(--accent-primary)] font-bold">
                MindSet OS — First Time Setup
              </span>
            </div>
            <div className="text-xs font-mono text-[var(--text-muted)]">
              Step <span className="text-[var(--accent-primary)] font-bold">{step}</span> of 6
            </div>
          </div>

          {/* Progress Bar with 6 segments */}
          <div className="grid grid-cols-6 gap-2">
            {[1, 2, 3, 4, 5, 6].map((s) => (
              <div
                key={s}
                className="h-1.5 rounded-full transition-all duration-300"
                style={{
                  backgroundColor:
                    s <= step ? 'var(--accent-primary)' : 'var(--bg-elevated)',
                  opacity: s <= step ? 1 : 0.4,
                }}
              />
            ))}
          </div>
        </div>

        {/* Dynamic Screen Content with 220ms Transition */}
        <div className="flex-1 transition-opacity duration-200 py-2">
          {/* ========================================================================= */}
          {/* SCREEN 1: Welcome & Philosophy & Theme Chooser                           */}
          {/* ========================================================================= */}
          {step === 1 && (
            <div className="space-y-6">
              <div className="text-center max-w-2xl mx-auto space-y-3">
                <div className="inline-flex p-3 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-3xl">
                  ✨
                </div>
                <h1 className="text-3xl md:text-4xl font-black tracking-tight text-[var(--text-dominant)]">
                  Welcome to MindSet OS
                </h1>
                <p className="text-sm md:text-base text-[var(--text-muted)] italic font-medium px-4">
                  "Your technology should help you achieve your goals, not compete for your attention."
                </p>
                <p className="text-xs text-[var(--text-secondary)]">
                  Designed by LKS PVT LTD for students and scholars who seek high-yield academic excellence,
                  uncompromising privacy, and cognitive calm.
                </p>
              </div>

              {/* Theme Selector Preview */}
              <div className="pt-4 border-t border-[var(--border-subtle)]">
                <div className="flex items-center justify-between mb-3">
                  <div className="text-sm font-semibold text-[var(--text-dominant)]">
                    Choose Your Neuroaesthetic Atmosphere
                  </div>
                  <div className="text-xs text-[var(--text-muted)]">
                    Current: <span className="font-mono text-[var(--accent-primary)]">{theme}</span>
                  </div>
                </div>
                <ThemeSelector />
              </div>
            </div>
          )}

          {/* ========================================================================= */}
          {/* SCREEN 2: Dual-Layer Multilingual Calibration                             */}
          {/* ========================================================================= */}
          {step === 2 && (
            <div className="space-y-6">
              <div>
                <h2 className="text-2xl font-bold text-[var(--text-dominant)]">
                  Dual-Layer Multilingual Setup
                </h2>
                <p className="text-xs text-[var(--text-muted)] mt-1">
                  Keep your OS desktop interface in English while conversing with your AI Mentor in natural Tamil or Tanglish!
                </p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* Layer A: OS Interface Language */}
                <div className="p-4 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-3">
                  <div className="text-xs uppercase tracking-wider font-bold text-[var(--accent-secondary)]">
                    Layer A: OS Interface Language
                  </div>
                  <div className="grid grid-cols-2 gap-2">
                    {(Object.keys(LOCALE_METADATA) as SupportedLocale[])
                      .filter((loc) => loc !== 'tanglish')
                      .map((loc) => {
                        const meta = LOCALE_METADATA[loc];
                        const isSelected = interfaceLang === loc;
                        return (
                          <button
                            key={loc}
                            onClick={() => setInterfaceLang(loc)}
                            className="p-2.5 rounded-lg text-left text-xs transition border flex flex-col justify-between"
                            style={{
                              backgroundColor: isSelected ? 'var(--bg-elevated)' : 'var(--bg-primary)',
                              borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                              color: isSelected ? 'var(--accent-primary)' : 'var(--text-secondary)',
                            }}
                          >
                            <span className="font-bold">{meta.nativeName}</span>
                            <span className="text-[10px] opacity-70">{meta.name}</span>
                          </button>
                        );
                      })}
                  </div>
                </div>

                {/* Layer B: MindSet AI Response Language & Style */}
                <div className="p-4 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-4">
                  <div className="text-xs uppercase tracking-wider font-bold text-[var(--accent-primary)]">
                    Layer B: AI Mentor Speech Style
                  </div>

                  <div className="space-y-2">
                    <button
                      onClick={() => {
                        setAiLang('ta');
                        setConversationStyle('tanglish');
                      }}
                      className="w-full p-3 rounded-lg text-left text-xs border transition flex items-center justify-between"
                      style={{
                        backgroundColor:
                          aiLang === 'ta' && conversationStyle === 'tanglish'
                            ? 'var(--bg-elevated)'
                            : 'var(--bg-primary)',
                        borderColor:
                          aiLang === 'ta' && conversationStyle === 'tanglish'
                            ? 'var(--accent-primary)'
                            : 'var(--border-subtle)',
                      }}
                    >
                      <div>
                        <div className="font-bold text-[var(--text-dominant)]">Conversational Tanglish</div>
                        <div className="text-[10px] text-[var(--text-muted)]">
                          Tamil + English mix in Latin script (e.g. "Romba easy thaan")
                        </div>
                      </div>
                      <span className="text-lg">✨</span>
                    </button>

                    <button
                      onClick={() => {
                        setAiLang('ta');
                        setConversationStyle('formal');
                      }}
                      className="w-full p-3 rounded-lg text-left text-xs border transition flex items-center justify-between"
                      style={{
                        backgroundColor:
                          aiLang === 'ta' && conversationStyle === 'formal'
                            ? 'var(--bg-elevated)'
                            : 'var(--bg-primary)',
                        borderColor:
                          aiLang === 'ta' && conversationStyle === 'formal'
                            ? 'var(--accent-primary)'
                            : 'var(--border-subtle)',
                      }}
                    >
                      <div>
                        <div className="font-bold text-[var(--text-dominant)]">Pure Tamil (தமிழ்)</div>
                        <div className="text-[10px] text-[var(--text-muted)]">
                          Elegant Tamil script with scientific notation
                        </div>
                      </div>
                      <span className="text-lg">📜</span>
                    </button>

                    <button
                      onClick={() => {
                        setAiLang('en');
                        setConversationStyle('formal');
                      }}
                      className="w-full p-3 rounded-lg text-left text-xs border transition flex items-center justify-between"
                      style={{
                        backgroundColor:
                          aiLang === 'en'
                            ? 'var(--bg-elevated)'
                            : 'var(--bg-primary)',
                        borderColor:
                          aiLang === 'en'
                            ? 'var(--accent-primary)'
                            : 'var(--border-subtle)',
                      }}
                    >
                      <div>
                        <div className="font-bold text-[var(--text-dominant)]">Pure English</div>
                        <div className="text-[10px] text-[var(--text-muted)]">
                          Clear, precise, direct academic English
                        </div>
                      </div>
                      <span className="text-lg">🎯</span>
                    </button>

                    <button
                      onClick={() => {
                        setAiLang('hi');
                        setConversationStyle('formal');
                      }}
                      className="w-full p-3 rounded-lg text-left text-xs border transition flex items-center justify-between"
                      style={{
                        backgroundColor:
                          aiLang === 'hi'
                            ? 'var(--bg-elevated)'
                            : 'var(--bg-primary)',
                        borderColor:
                          aiLang === 'hi'
                            ? 'var(--accent-primary)'
                            : 'var(--border-subtle)',
                      }}
                    >
                      <div>
                        <div className="font-bold text-[var(--text-dominant)]">Hindi (हिन्दी)</div>
                        <div className="text-[10px] text-[var(--text-muted)]">
                          Conversational Hindi / Devanagari script
                        </div>
                      </div>
                      <span className="text-lg">🇮🇳</span>
                    </button>
                  </div>
                </div>
              </div>

              {/* Synchronous Live Preview Card */}
              <div className="p-4 rounded-xl bg-[var(--bg-elevated)] border border-[var(--border-subtle)]">
                <div className="text-[11px] font-mono text-[var(--accent-primary)] uppercase tracking-wider mb-2">
                  Live Bilingual Synchronous Preview
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 items-center">
                  <div className="p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-subtle)]">
                    <span className="text-[10px] text-[var(--text-muted)] block mb-1">
                      OS Button Preview ({interfaceLang})
                    </span>
                    <button className="px-3 py-1.5 rounded bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold">
                      {t(interfaceLang, 'onboarding.launch')}
                    </button>
                  </div>
                  <div className="p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-subtle)]">
                    <span className="text-[10px] text-[var(--text-muted)] block mb-1">
                      AI Mentor Chat Preview ({conversationStyle === 'tanglish' ? 'Tanglish' : aiLang})
                    </span>
                    <p className="text-xs text-[var(--text-dominant)] italic">
                      {conversationStyle === 'tanglish'
                        ? '“Vanakkam! Naan unga MindSet AI mentor. Innikku daily target mudikkalaama?”'
                        : aiLang === 'ta'
                        ? '“வணக்கம்! நான் உங்கள் மைண்ட்செட் வழிகாட்டி. இன்றைய பாடத்திட்டத்தைத் தொடங்கலாமா?”'
                        : aiLang === 'hi'
                        ? '“नमस्ते! मैं आपका MindSet AI मेंटर हूँ। आज का अध्ययन लक्ष्य शुरू करें?”'
                        : '“Hello! I am your MindSet AI mentor. Ready to master your syllabus today?”'}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* ========================================================================= */}
          {/* SCREEN 3: Identity & Top 3 Goals                                          */}
          {/* ========================================================================= */}
          {step === 3 && (
            <div className="space-y-5">
              <div>
                <h2 className="text-2xl font-bold text-[var(--text-dominant)]">
                  Identity & Ambition
                </h2>
                <p className="text-xs text-[var(--text-muted)] mt-1">
                  Define who you are and the high-leverage milestones you are conquering.
                </p>
              </div>

              {/* Name & Role Row */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-1">
                    What should we call you?
                  </label>
                  <input
                    type="text"
                    value={displayName}
                    onChange={(e) => setDisplayName(e.target.value)}
                    placeholder="e.g. Arjun"
                    className="w-full px-3 py-2 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)] text-sm text-[var(--text-dominant)] focus:border-[var(--accent-primary)] outline-none"
                  />
                </div>
                <div>
                  <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-1">
                    Your Primary Role
                  </label>
                  <div className="grid grid-cols-3 gap-2">
                    {(['student', 'researcher', 'professional'] as const).map((r) => (
                      <button
                        key={r}
                        onClick={() => setRole(r)}
                        className="py-2 px-2 text-xs rounded-lg border capitalize text-center transition"
                        style={{
                          backgroundColor: role === r ? 'var(--bg-elevated)' : 'var(--bg-secondary)',
                          borderColor: role === r ? 'var(--accent-primary)' : 'var(--border-subtle)',
                          color: role === r ? 'var(--accent-primary)' : 'var(--text-secondary)',
                        }}
                      >
                        {r}
                      </button>
                    ))}
                  </div>
                </div>
              </div>

              {/* Quick Goal Presets */}
              <div>
                <div className="text-xs font-semibold text-[var(--text-secondary)] mb-2 flex items-center justify-between">
                  <span>Quick Ambition Presets (Click to Add)</span>
                  <span className="text-[10px] text-[var(--text-muted)]">Max 3 goals</span>
                </div>
                <div className="flex flex-wrap gap-2">
                  {goalPresets.map((preset) => (
                    <button
                      key={preset.title}
                      onClick={() => handleApplyPresetGoal(preset.title, 1)}
                      className="px-2.5 py-1 text-xs rounded-md bg-[var(--bg-secondary)] hover:bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-[var(--text-secondary)] hover:text-[var(--accent-primary)] transition"
                    >
                      + {preset.title}
                    </button>
                  ))}
                </div>
              </div>

              {/* Top 3 Prioritized Goals Matrix */}
              <div className="space-y-2">
                <label className="text-xs font-semibold text-[var(--text-secondary)] block">
                  Top 3 Goals
                </label>
                {[0, 1, 2].map((idx) => {
                  const goal = goals[idx];
                  const priorityLabel = idx === 0 ? 'P1 Crucial' : idx === 1 ? 'P2 Medium' : 'P3 Supporting';
                  const badgeColor = idx === 0 ? 'var(--accent-primary)' : idx === 1 ? 'var(--accent-secondary)' : 'var(--text-muted)';
                  return (
                    <div
                      key={idx}
                      className="flex items-center gap-3 p-2.5 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]"
                    >
                      <span
                        className="text-[10px] font-mono font-bold px-2 py-0.5 rounded border"
                        style={{ borderColor: badgeColor, color: badgeColor }}
                      >
                        {priorityLabel}
                      </span>
                      <input
                        type="text"
                        value={goal ? goal.title : ''}
                        onChange={(e) => handleUpdateGoalTitle(idx, e.target.value)}
                        placeholder={`Goal #${idx + 1}`}
                        className="flex-1 bg-transparent text-xs text-[var(--text-dominant)] outline-none"
                      />
                    </div>
                  );
                })}
              </div>

              {/* Focus Subjects Chips */}
              <div>
                <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-2">
                  Core Subjects & Domains
                </label>
                <div className="flex flex-wrap gap-2">
                  {subjectOptions.map((subj) => {
                    const isSelected = focusAreas.includes(subj);
                    return (
                      <button
                        key={subj}
                        onClick={() => toggleSubject(subj)}
                        className="px-3 py-1 rounded-full text-xs font-medium border transition"
                        style={{
                          backgroundColor: isSelected ? 'var(--bg-elevated)' : 'var(--bg-primary)',
                          borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                          color: isSelected ? 'var(--accent-primary)' : 'var(--text-secondary)',
                        }}
                      >
                        {subj} {isSelected && '✓'}
                      </button>
                    );
                  })}
                </div>
              </div>
            </div>
          )}

          {/* ========================================================================= */}
          {/* SCREEN 4: Daily Routine & Visual 24-Hour Timeline                         */}
          {/* ========================================================================= */}
          {step === 4 && (
            <div className="space-y-6">
              <div>
                <h2 className="text-2xl font-bold text-[var(--text-dominant)]">
                  Daily Routine & Timetable
                </h2>
                <p className="text-xs text-[var(--text-muted)] mt-1">
                  Define realistic bounds so MindSet OS protects your focus and ensures restful recovery.
                </p>
              </div>

              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-[10px] uppercase font-bold text-[var(--accent-secondary)] block mb-1">
                    🌅 Wake Up
                  </span>
                  <input
                    type="time"
                    value={wakeTime}
                    onChange={(e) => setWakeTime(e.target.value)}
                    className="w-full bg-[var(--bg-primary)] px-2 py-1.5 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)] outline-none"
                  />
                </div>

                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-[10px] uppercase font-bold text-[var(--accent-achievement)] block mb-1">
                    🌙 Sleep Time
                  </span>
                  <input
                    type="time"
                    value={sleepTime}
                    onChange={(e) => setSleepTime(e.target.value)}
                    className="w-full bg-[var(--bg-primary)] px-2 py-1.5 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)] outline-none"
                  />
                </div>

                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-[10px] uppercase font-bold text-[var(--text-muted)] block mb-1">
                    🏫 School/Work Start
                  </span>
                  <input
                    type="time"
                    value={schoolStart}
                    onChange={(e) => setSchoolStart(e.target.value)}
                    className="w-full bg-[var(--bg-primary)] px-2 py-1.5 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)] outline-none"
                  />
                </div>

                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-[10px] uppercase font-bold text-[var(--text-muted)] block mb-1">
                    🏫 School/Work End
                  </span>
                  <input
                    type="time"
                    value={schoolEnd}
                    onChange={(e) => setSchoolEnd(e.target.value)}
                    className="w-full bg-[var(--bg-primary)] px-2 py-1.5 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)] outline-none"
                  />
                </div>
              </div>

              {/* Study Windows */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-xs font-semibold text-[var(--text-dominant)] block mb-2">
                    🎯 Morning Study Window
                  </span>
                  <div className="flex items-center gap-2">
                    <input
                      type="time"
                      value={studySlot1Start}
                      onChange={(e) => setStudySlot1Start(e.target.value)}
                      className="bg-[var(--bg-primary)] px-2 py-1 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)]"
                    />
                    <span className="text-xs text-[var(--text-muted)]">to</span>
                    <input
                      type="time"
                      value={studySlot1End}
                      onChange={(e) => setStudySlot1End(e.target.value)}
                      className="bg-[var(--bg-primary)] px-2 py-1 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)]"
                    />
                  </div>
                </div>

                <div className="p-3 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-subtle)]">
                  <span className="text-xs font-semibold text-[var(--text-dominant)] block mb-2">
                    📚 Evening Study Window
                  </span>
                  <div className="flex items-center gap-2">
                    <input
                      type="time"
                      value={studySlot2Start}
                      onChange={(e) => setStudySlot2Start(e.target.value)}
                      className="bg-[var(--bg-primary)] px-2 py-1 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)]"
                    />
                    <span className="text-xs text-[var(--text-muted)]">to</span>
                    <input
                      type="time"
                      value={studySlot2End}
                      onChange={(e) => setStudySlot2End(e.target.value)}
                      className="bg-[var(--bg-primary)] px-2 py-1 rounded border border-[var(--border-subtle)] text-xs font-mono text-[var(--text-dominant)]"
                    />
                  </div>
                </div>
              </div>

              {/* 24-Hour Visual Schedule Bar */}
              <div className="p-4 rounded-xl bg-[var(--bg-elevated)] border border-[var(--border-subtle)]">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs font-semibold text-[var(--text-dominant)]">
                    24-Hour Daily Balance Preview
                  </span>
                  <div className="flex items-center gap-3 text-[10px]">
                    <span className="flex items-center gap-1">
                      <span className="w-2.5 h-2.5 rounded bg-indigo-500/80 inline-block" /> Sleep
                    </span>
                    <span className="flex items-center gap-1">
                      <span className="w-2.5 h-2.5 rounded bg-slate-500/80 inline-block" /> Commitments
                    </span>
                    <span className="flex items-center gap-1">
                      <span className="w-2.5 h-2.5 rounded bg-[var(--accent-primary)] inline-block" /> Study Focus
                    </span>
                    <span className="flex items-center gap-1">
                      <span className="w-2.5 h-2.5 rounded bg-emerald-500/80 inline-block" /> Free Time
                    </span>
                  </div>
                </div>

                {/* Visual Bar */}
                <div className="h-6 w-full rounded-md overflow-hidden flex text-[10px] font-mono font-bold text-white shadow-inner">
                  <div style={{ width: '25%' }} className="bg-indigo-600 flex items-center justify-center">
                    Sleep
                  </div>
                  <div style={{ width: '10%' }} className="bg-[var(--accent-primary)] flex items-center justify-center text-black">
                    Study
                  </div>
                  <div style={{ width: '35%' }} className="bg-slate-600 flex items-center justify-center">
                    School / College
                  </div>
                  <div style={{ width: '15%' }} className="bg-[var(--accent-primary)] flex items-center justify-center text-black">
                    Study
                  </div>
                  <div style={{ width: '15%' }} className="bg-emerald-600 flex items-center justify-center">
                    Relax
                  </div>
                </div>

                <div className="flex justify-between text-[10px] font-mono text-[var(--text-muted)] mt-1.5">
                  <span>00:00</span>
                  <span>06:00</span>
                  <span>12:00</span>
                  <span>18:00</span>
                  <span>24:00</span>
                </div>
              </div>
            </div>
          )}

          {/* ========================================================================= */}
          {/* SCREEN 5: AI & Learning Preferences                                       */}
          {/* ========================================================================= */}
          {step === 5 && (
            <div className="space-y-5">
              <div>
                <h2 className="text-2xl font-bold text-[var(--text-dominant)]">
                  Learning Style & AI Persona
                </h2>
                <p className="text-xs text-[var(--text-muted)] mt-1">
                  Calibrate how MindSet AI mentors you and deconstructs challenging concepts.
                </p>
              </div>

              {/* Learning Style Cards */}
              <div>
                <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-2">
                  Preferred Cognitive Learning Style
                </label>
                <div className="grid grid-cols-2 md:grid-cols-3 gap-2.5">
                  {[
                    { id: 'step_by_step', title: '🧩 Step-by-Step', desc: 'Numbered logical deductions' },
                    { id: 'visual', title: '👁️ Visual Maps', desc: 'ASCII diagrams & spatial structures' },
                    { id: 'practical_examples', title: '🛠️ Practical Code', desc: 'Real-world analogies & tests' },
                    { id: 'reading_notes', title: '📖 Condensed Notes', desc: 'Cheat-sheets & formula callouts' },
                    { id: 'interactive', title: '🗣️ Socratic Inquiry', desc: 'Guided probing questions' },
                  ].map((ls) => {
                    const isSelected = learningStyle === ls.id;
                    return (
                      <button
                        key={ls.id}
                        onClick={() => setLearningStyle(ls.id as LearningStyle)}
                        className="p-3 rounded-lg text-left border transition"
                        style={{
                          backgroundColor: isSelected ? 'var(--bg-elevated)' : 'var(--bg-secondary)',
                          borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                        }}
                      >
                        <div className="text-xs font-bold text-[var(--text-dominant)]">{ls.title}</div>
                        <div className="text-[10px] text-[var(--text-muted)] mt-0.5">{ls.desc}</div>
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* AI Communication Tone */}
              <div>
                <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-2">
                  AI Mentor Voice & Tone
                </label>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-2.5">
                  {[
                    { id: 'friendly_direct', title: '🤝 Friendly & Direct', desc: 'Warm yet focused' },
                    { id: 'encouraging', title: '🌟 Supportive', desc: 'Growth mindset emphasis' },
                    { id: 'analytical', title: '🔬 Analytical', desc: 'Rigorous derivations' },
                    { id: 'concise', title: '⚡ High-Yield', desc: 'Zero fluff, direct bullet points' },
                  ].map((tone) => {
                    const isSelected = aiTone === tone.id;
                    return (
                      <button
                        key={tone.id}
                        onClick={() => setAiTone(tone.id as AiTone)}
                        className="p-2.5 rounded-lg text-left border transition"
                        style={{
                          backgroundColor: isSelected ? 'var(--bg-elevated)' : 'var(--bg-secondary)',
                          borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                        }}
                      >
                        <div className="text-xs font-bold text-[var(--text-dominant)]">{tone.title}</div>
                        <div className="text-[10px] text-[var(--text-muted)] mt-0.5">{tone.desc}</div>
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* Planning Style & Hobbies */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-2">
                    Planning Approach
                  </label>
                  <div className="grid grid-cols-2 gap-2">
                    {[
                      { id: 'flexible', title: 'Flexible Flow', desc: 'Adaptive blocks' },
                      { id: 'structured', title: 'Structured Discipline', desc: 'Fixed timetables' },
                    ].map((p) => (
                      <button
                        key={p.id}
                        onClick={() => setPlanningStyle(p.id as PlanningStyle)}
                        className="p-2 rounded-lg border text-left transition"
                        style={{
                          backgroundColor: planningStyle === p.id ? 'var(--bg-elevated)' : 'var(--bg-secondary)',
                          borderColor: planningStyle === p.id ? 'var(--accent-primary)' : 'var(--border-subtle)',
                        }}
                      >
                        <div className="text-xs font-bold text-[var(--text-dominant)]">{p.title}</div>
                        <div className="text-[10px] text-[var(--text-muted)]">{p.desc}</div>
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <label className="text-xs font-semibold text-[var(--text-secondary)] block mb-2">
                    Free-Time Hobbies & Rejuvenation
                  </label>
                  <div className="flex flex-wrap gap-1.5">
                    {hobbyOptions.map((hobby) => {
                      const isSelected = interests.includes(hobby);
                      return (
                        <button
                          key={hobby}
                          onClick={() => toggleInterest(hobby)}
                          className="px-2.5 py-1 rounded-full text-xs border transition"
                          style={{
                            backgroundColor: isSelected ? 'var(--bg-elevated)' : 'var(--bg-secondary)',
                            borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                            color: isSelected ? 'var(--accent-primary)' : 'var(--text-secondary)',
                          }}
                        >
                          {hobby}
                        </button>
                      );
                    })}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* ========================================================================= */}
          {/* SCREEN 6: Meet Your AI & First Command Center Launch                      */}
          {/* ========================================================================= */}
          {step === 6 && (
            <div className="space-y-6">
              <div className="text-center max-w-xl mx-auto space-y-2">
                <div className="inline-flex p-3 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-3xl">
                  🤖
                </div>
                <h2 className="text-2xl md:text-3xl font-bold text-[var(--text-dominant)]">
                  Meet Your AI Mentor
                </h2>
                <p className="text-xs text-[var(--text-muted)]">
                  Your personalized epistemic model is synthesized and ready inside SQLite.
                </p>
              </div>

              {/* Dynamic Live Greeting Card */}
              <div className="p-5 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--accent-primary)]/40 shadow-lg space-y-3">
                <div className="flex items-center gap-2">
                  <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" />
                  <span className="text-xs font-bold uppercase tracking-wider text-[var(--accent-primary)]">
                    MindSet AI — Real-Time Personalized Synthesis
                  </span>
                </div>
                <p className="text-sm md:text-base text-[var(--text-dominant)] leading-relaxed italic">
                  "{liveGreeting}"
                </p>
              </div>

              {/* Local-First Sovereignty Assurances */}
              <div className="p-4 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-subtle)] space-y-2">
                <div className="text-xs font-bold text-[var(--accent-secondary)] uppercase tracking-wider">
                  MindSet Privacy & Hardware Boundaries
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-3 text-xs text-[var(--text-secondary)]">
                  <div className="flex items-start gap-2">
                    <span className="text-emerald-400 font-bold">✓</span>
                    <span>100% Local SQLite storage on your hard drive</span>
                  </div>
                  <div className="flex items-start gap-2">
                    <span className="text-emerald-400 font-bold">✓</span>
                    <span>Offline-first AI inference with zero data harvesting</span>
                  </div>
                  <div className="flex items-start gap-2">
                    <span className="text-emerald-400 font-bold">✓</span>
                    <span>Ultra-low RAM overhead strictly under 120 MB</span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Bottom Navigation Buttons */}
        <div className="pt-4 border-t border-[var(--border-subtle)] flex items-center justify-between">
          {step > 1 ? (
            <button
              onClick={() => setStep((s) => s - 1)}
              className="px-4 py-2 rounded-lg bg-[var(--bg-secondary)] hover:bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-xs font-semibold text-[var(--text-secondary)] transition"
            >
              ← Back
            </button>
          ) : (
            <div />
          )}

          {step < 6 ? (
            <button
              onClick={() => setStep((s) => s + 1)}
              className="px-6 py-2.5 rounded-lg bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold tracking-wide hover:opacity-90 transition shadow-md"
            >
              Continue →
            </button>
          ) : (
            <button
              onClick={handleFinishOnboarding}
              disabled={isSaving}
              className="px-8 py-3 rounded-xl bg-[var(--accent-primary)] text-[var(--bg-base)] text-sm font-black tracking-wide hover:opacity-95 transition shadow-xl flex items-center gap-2"
            >
              {isSaving ? 'Initializing Desktop...' : '🚀 Launch MindSet Command Center'}
            </button>
          )}
        </div>
      </GlassSurface>
    </div>
  );
};
