/**
 * MindSet OS — Academic Goal & Syllabus Skill-Tree Engine (/goals & /tasks)
 * Target Calculator (e.g. 590/600), Chapter Mastery Skill-Tree & Auto-Scheduler
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useMemo } from 'react';
import {
  GlassSurface,
  ProgressRing,
  Button,
  useThemeStore,
} from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';

export type ChapterStatus = 'not_started' | 'learning' | 'revision_1' | 'mastered';

export interface ChapterNode {
  id: string;
  name: string;
  weightageMarks: number;
  status: ChapterStatus;
  prerequisites?: string[];
  notes?: string;
}

export interface SubjectTarget {
  id: string;
  name: string;
  code: string;
  maxMarks: number;
  targetMarks: number;
  color: string;
  chapters: ChapterNode[];
}

const INITIAL_SUBJECTS: SubjectTarget[] = [
  {
    id: 'sub-tamil',
    name: 'Tamil (தமிழ்)',
    code: 'TAM-12',
    maxMarks: 100,
    targetMarks: 98,
    color: '#FF8EAA',
    chapters: [
      { id: 'tam-1', name: 'உரைநடை: இளம் தமிழே & செய்யுள்', weightageMarks: 20, status: 'mastered' },
      { id: 'tam-2', name: 'கவிதைப்பேழை: பாரதியார் & பாரதிதாசன்', weightageMarks: 20, status: 'revision_1' },
      { id: 'tam-3', name: 'துணைப்பாடம் & விரிவானம்', weightageMarks: 20, status: 'learning' },
      { id: 'tam-4', name: 'இலக்கணம்: வல்லினம் மிகும் & மிகா இடங்கள்', weightageMarks: 20, status: 'mastered' },
      { id: 'tam-5', name: 'படைப்பாற்றல் & கட்டுரை வரைதல்', weightageMarks: 20, status: 'revision_1' },
    ],
  },
  {
    id: 'sub-english',
    name: 'English Language & Literature',
    code: 'ENG-12',
    maxMarks: 100,
    targetMarks: 96,
    color: '#8B9CFF',
    chapters: [
      { id: 'eng-1', name: 'Prose: Two Gentlemen of Verona', weightageMarks: 20, status: 'mastered' },
      { id: 'eng-2', name: 'Poem: The Castle & Casuarina Tree', weightageMarks: 20, status: 'mastered' },
      { id: 'eng-3', name: 'Supplementary: God Sees the Truth', weightageMarks: 20, status: 'revision_1' },
      { id: 'eng-4', name: 'Grammar: Direct/Indirect & Modals', weightageMarks: 20, status: 'learning' },
      { id: 'eng-5', name: 'Letter Writing & Reading Comprehension', weightageMarks: 20, status: 'mastered' },
    ],
  },
  {
    id: 'sub-math',
    name: 'Mathematics',
    code: 'MAT-12',
    maxMarks: 100,
    targetMarks: 100,
    color: '#65E6C4',
    chapters: [
      { id: 'mat-1', name: 'Applications of Matrices & Determinants', weightageMarks: 15, status: 'mastered' },
      { id: 'mat-2', name: 'Complex Numbers & De Moivre Theorem', weightageMarks: 15, status: 'mastered' },
      { id: 'mat-3', name: 'Theory of Equations & Polynomials', weightageMarks: 10, status: 'revision_1' },
      { id: 'mat-4', name: 'Inverse Trigonometric Functions', weightageMarks: 10, status: 'mastered' },
      { id: 'mat-5', name: 'Two Dimensional Analytical Geometry II', weightageMarks: 15, status: 'learning' },
      { id: 'mat-6', name: 'Differential Calculus & Applications', weightageMarks: 15, status: 'learning' },
      { id: 'mat-7', name: 'Integral Calculus & Differential Equations', weightageMarks: 20, status: 'not_started' },
    ],
  },
  {
    id: 'sub-physics',
    name: 'Physics',
    code: 'PHY-12',
    maxMarks: 100,
    targetMarks: 98,
    color: '#FFD078',
    chapters: [
      { id: 'phy-1', name: 'Electrostatics & Coulomb Law', weightageMarks: 15, status: 'mastered' },
      { id: 'phy-2', name: 'Current Electricity & Kirchhoff Rules', weightageMarks: 15, status: 'mastered' },
      { id: 'phy-3', name: 'Magnetism & Magnetic Effects of Current', weightageMarks: 15, status: 'revision_1' },
      { id: 'phy-4', name: 'Electromagnetic Induction & AC Currents', weightageMarks: 15, status: 'learning' },
      { id: 'phy-5', name: 'Electromagnetic Waves & Wave Optics', weightageMarks: 15, status: 'learning' },
      { id: 'phy-6', name: 'Dual Nature of Radiation and Matter', weightageMarks: 10, status: 'mastered' },
      { id: 'phy-7', name: 'Atomic & Nuclear Physics', weightageMarks: 15, status: 'not_started' },
    ],
  },
  {
    id: 'sub-chemistry',
    name: 'Chemistry',
    code: 'CHE-12',
    maxMarks: 100,
    targetMarks: 99,
    color: '#A78BFA',
    chapters: [
      { id: 'che-1', name: 'Metallurgy & Extraction Processes', weightageMarks: 10, status: 'mastered' },
      { id: 'che-2', name: 'p-Block Elements I & II', weightageMarks: 15, status: 'revision_1' },
      { id: 'che-3', name: 'Transition & Inner Transition Elements', weightageMarks: 15, status: 'learning' },
      { id: 'che-4', name: 'Coordination Chemistry & Isomerism', weightageMarks: 15, status: 'mastered' },
      { id: 'che-5', name: 'Solid State & Crystal Lattices', weightageMarks: 15, status: 'mastered' },
      { id: 'che-6', name: 'Chemical Kinetics & Rate Laws', weightageMarks: 15, status: 'revision_1' },
      { id: 'che-7', name: 'Organic Chemistry: Hydroxy Compounds', weightageMarks: 15, status: 'not_started' },
    ],
  },
  {
    id: 'sub-cs',
    name: 'Computer Science',
    code: 'CS-12',
    maxMarks: 100,
    targetMarks: 99,
    color: '#38BDF8',
    chapters: [
      { id: 'cs-1', name: 'Function Abstraction & Data Structures', weightageMarks: 15, status: 'mastered' },
      { id: 'cs-2', name: 'Control Structures & Iteration in Python', weightageMarks: 15, status: 'mastered' },
      { id: 'cs-3', name: 'Object Oriented Programming & Classes', weightageMarks: 20, status: 'mastered' },
      { id: 'cs-4', name: 'Database Management Systems & SQL', weightageMarks: 20, status: 'revision_1' },
      { id: 'cs-5', name: 'Python with SQLite Integration', weightageMarks: 15, status: 'mastered' },
      { id: 'cs-6', name: 'Data Visualisation using Pyplot', weightageMarks: 15, status: 'learning' },
    ],
  },
];

export const GoalsAndSyllabusView: React.FC = () => {
  const { setTheme } = useThemeStore();
  const [subjects, setSubjects] = useState<SubjectTarget[]>(INITIAL_SUBJECTS);
  const [selectedSubjectId, setSelectedSubjectId] = useState<string>('sub-math');
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const selectedSubject: SubjectTarget = useMemo(() => {
    return subjects.find((s) => s.id === selectedSubjectId) || subjects[0] || INITIAL_SUBJECTS[0]!;
  }, [subjects, selectedSubjectId]);

  // Total Marks Calculations
  const totalTargetMarks = useMemo(() => {
    return subjects.reduce((sum, s) => sum + s.targetMarks, 0);
  }, [subjects]);

  const totalMaxMarks = useMemo(() => {
    return subjects.reduce((sum, s) => sum + s.maxMarks, 0);
  }, [subjects]);

  // Engineering Cutoff (Maths / 100 * 100 + Physics / 100 * 50 + Chemistry / 100 * 50) = 200
  const engineeringCutoff = useMemo(() => {
    const math = subjects.find((s) => s.id === 'sub-math')?.targetMarks || 100;
    const phy = subjects.find((s) => s.id === 'sub-physics')?.targetMarks || 98;
    const chem = subjects.find((s) => s.id === 'sub-chemistry')?.targetMarks || 99;
    return (math + phy / 2 + chem / 2).toFixed(1);
  }, [subjects]);

  // Overall Syllabus Completion Percentage
  const syllabusProgress = useMemo(() => {
    let totalNodes = 0;
    let completedPoints = 0;
    subjects.forEach((s) => {
      s.chapters.forEach((c) => {
        totalNodes += 1;
        if (c.status === 'mastered') completedPoints += 1;
        else if (c.status === 'revision_1') completedPoints += 0.75;
        else if (c.status === 'learning') completedPoints += 0.35;
      });
    });
    return totalNodes > 0 ? Math.round((completedPoints / totalNodes) * 100) : 0;
  }, [subjects]);

  // Advance Chapter Status (Non-punitive ethical dopamine)
  const handleAdvanceStatus = (subjectId: string, chapterId: string) => {
    setSubjects((prev) =>
      prev.map((sub) => {
        if (sub.id !== subjectId) return sub;
        return {
          ...sub,
          chapters: sub.chapters.map((ch) => {
            if (ch.id !== chapterId) return ch;
            const nextStatus: ChapterStatus =
              ch.status === 'not_started'
                ? 'learning'
                : ch.status === 'learning'
                ? 'revision_1'
                : ch.status === 'revision_1'
                ? 'mastered'
                : 'mastered';
            return { ...ch, status: nextStatus };
          }),
        };
      })
    );

    setToastMessage(`Level up! Advanced chapter to higher mastery (+35 XP logged).`);
    setTimeout(() => setToastMessage(null), 3000);
  };

  // One-click Auto-Schedule Weak Chapters into SQLite
  const handleAutoScheduleWeakChapters = async () => {
    const weakChapters: { subject: string; chapter: ChapterNode }[] = [];
    subjects.forEach((s) => {
      s.chapters.forEach((c) => {
        if (c.status === 'not_started' || c.status === 'learning') {
          weakChapters.push({ subject: s.name, chapter: c });
        }
      });
    });

    try {
      // Schedule top 3 priority chapters into SQLite tasks
      const top3 = weakChapters.slice(0, 3);
      for (const item of top3) {
        await dbClient.tasks.create({
          title: `Study Session: ${item.chapter.name}`,
          subject_tag: item.subject,
          priority: 1,
          target_score_weight: 1.0,
          xp_reward: 50,
        });
      }

      setToastMessage(
        `PlannerAgent: Auto-scheduled ${top3.length} weak chapter study blocks into /tasks.`
      );
      setTimeout(() => setToastMessage(null), 4500);
    } catch (err) {
      console.error('[Goals] Auto-schedule failed:', err);
    }
  };

  const getStatusBadge = (status: ChapterStatus) => {
    switch (status) {
      case 'mastered':
        return <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-[#65E6C4]/20 text-[#65E6C4] border border-[#65E6C4]/40">Mastered</span>;
      case 'revision_1':
        return <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-[#FFD078]/20 text-[#FFD078] border border-[#FFD078]/40">Revision 1</span>;
      case 'learning':
        return <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-[#8B9CFF]/20 text-[#8B9CFF] border border-[#8B9CFF]/40">Learning</span>;
      case 'not_started':
        return <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-[var(--surface-elevated)] text-[var(--text-muted)] border border-[var(--border-subtle)]">Not Started</span>;
    }
  };

  return (
    <div className="space-y-6 select-none p-1">
      {/* ------------------------------------------------------------- */}
      {/* 1. TOP TARGET CALCULATOR & AMBITIOUS TARGET BANNER             */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-4">
        <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xl">🎯</span>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Class 12 State Board Target Calculator
              </h2>
            </div>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              Break down ambitious benchmarks into subject weightages, chapter skill-trees, and revision queues.
            </p>
          </div>

          {/* Target Score Readout */}
          <div className="flex items-center space-x-4">
            <div className="text-right">
              <div className="text-[10px] uppercase font-bold text-[var(--text-muted)]">Target Total</div>
              <div className="text-2xl font-mono font-bold tabular-nums text-[#65E6C4]">
                {totalTargetMarks} / {totalMaxMarks}
              </div>
              <div className="text-[11px] font-mono text-[#8B9CFF]">
                Cut-off: {engineeringCutoff} / 200
              </div>
            </div>

            <ProgressRing
              progress={syllabusProgress}
              size={64}
              strokeWidth={5}
              accentColor="#65E6C4"
              sublabel="Covered"
            />
          </div>
        </div>

        {/* Action Controls */}
        <div className="flex flex-wrap items-center justify-between gap-2 pt-2 border-t border-[var(--border-subtle)]">
          <div className="flex items-center space-x-2">
            <Button
              variant="primary"
              size="sm"
              onClick={handleAutoScheduleWeakChapters}
              className="text-xs font-semibold"
            >
              ⚡ Auto-Schedule Weak Chapters
            </Button>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => setTheme('cosmic-arcade')}
              title="Switch to Cosmic Arcade (#100D25) for gamified skill-tree viewing"
              className="text-xs"
            >
              🕹️ Cosmic Arcade View
            </Button>
          </div>

          {toastMessage && (
            <div className="text-xs text-[#65E6C4] font-medium animate-fadeIn">
              ✓ {toastMessage}
            </div>
          )}
        </div>
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 2. SUBJECT-WISE TARGET SELECTOR TABS                          */}
      {/* ------------------------------------------------------------- */}
      <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
        {subjects.map((sub) => {
          const isSelected = sub.id === selectedSubjectId;
          const masteredCount = sub.chapters.filter((c) => c.status === 'mastered').length;

          return (
            <button
              key={sub.id}
              type="button"
              onClick={() => setSelectedSubjectId(sub.id)}
              className={`p-3 rounded-xl border text-left transition-all ${
                isSelected
                  ? 'border-[var(--accent-primary)] bg-[var(--surface-elevated)] shadow-lg scale-102'
                  : 'border-[var(--border-subtle)] bg-[var(--surface-primary)] hover:border-[var(--border-strong)]'
              }`}
            >
              <div className="text-xs font-bold text-[var(--text-dominant)] truncate">{sub.name}</div>
              <div className="flex items-baseline justify-between mt-1 text-xs">
                <span className="font-mono font-bold" style={{ color: sub.color }}>
                  {sub.targetMarks}/100
                </span>
                <span className="text-[10px] text-[var(--text-muted)] font-mono">
                  {masteredCount}/{sub.chapters.length} ✓
                </span>
              </div>
            </button>
          );
        })}
      </div>

      {/* ------------------------------------------------------------- */}
      {/* 3. INTERACTIVE CHAPTER SKILL-TREE & LEARNING JOURNEY MAP      */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-sm font-bold text-[var(--text-dominant)] flex items-center space-x-2">
              <span style={{ color: selectedSubject.color }}>●</span>
              <span>{selectedSubject.name} — Syllabus Skill-Tree</span>
            </h3>
            <p className="text-xs text-[var(--text-muted)]">
              Click any chapter node to advance your revision stage. Mastered nodes earn +50 XP.
            </p>
          </div>

          <div className="text-xs font-mono text-[var(--text-muted)]">
            Total Weightage: <strong>100 Marks</strong>
          </div>
        </div>

        {/* Skill-Tree Node Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          {selectedSubject.chapters.map((chapter, idx) => {
            const isMastered = chapter.status === 'mastered';

            return (
              <div
                key={chapter.id}
                onClick={() => handleAdvanceStatus(selectedSubject.id, chapter.id)}
                className={`p-4 rounded-xl border cursor-pointer transition-all duration-150 hover:scale-[1.02] ${
                  isMastered
                    ? 'border-[#65E6C4]/50 bg-[#65E6C4]/10 shadow-[0_0_15px_rgba(101,230,196,0.15)]'
                    : chapter.status === 'revision_1'
                    ? 'border-[#FFD078]/40 bg-[#FFD078]/5'
                    : chapter.status === 'learning'
                    ? 'border-[#8B9CFF]/40 bg-[#8B9CFF]/5'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] opacity-85'
                }`}
              >
                <div className="flex items-start justify-between">
                  <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-[var(--surface-primary)] text-[var(--text-muted)]">
                    Node #{idx + 1}
                  </span>
                  {getStatusBadge(chapter.status)}
                </div>

                <div className="font-semibold text-xs text-[var(--text-dominant)] mt-2 leading-snug">
                  {chapter.name}
                </div>

                <div className="flex items-center justify-between text-[11px] text-[var(--text-muted)] mt-3 pt-2 border-t border-[var(--border-subtle)]">
                  <span>Weightage: {chapter.weightageMarks}m</span>
                  <span className="text-[#65E6C4] font-medium">Click to Advance →</span>
                </div>
              </div>
            );
          })}
        </div>
      </GlassSurface>
    </div>
  );
};
