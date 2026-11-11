/**
 * MindSet OS — Weekly Life-Balance & Wellbeing Analytics
 * 9-Dimension Balanced Distribution, Focus Trends & Supportive Coach Observations
 * STRICT RULE: Zero Toxic "Productivity Scores" or Competitive Leaderboards
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { GlassSurface } from '@mindset/ui-core';
import {
  ResponsiveContainer,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  Radar,
  AreaChart,
  Area,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
} from 'recharts';

// 9-Dimension Weekly Hours Breakdown
const NINE_DIMENSIONS_DATA = [
  { dimension: 'Study & Classes', hours: 26, target: 28, fullMark: 35 },
  { dimension: 'Board Exam Goals', hours: 14, target: 15, fullMark: 20 },
  { dimension: 'Curious Learning', hours: 6, target: 5, fullMark: 10 },
  { dimension: 'Physical Movement', hours: 7, target: 7, fullMark: 10 },
  { dimension: 'Mental Relaxation', hours: 5, target: 4, fullMark: 8 },
  { dimension: 'Intentional Play', hours: 6, target: 6, fullMark: 10 },
  { dimension: 'Family & Social', hours: 9, target: 8, fullMark: 12 },
  { dimension: 'Unplanned Buffer', hours: 7, target: 7, fullMark: 10 },
  { dimension: 'Sleep & Recovery', hours: 53, target: 54, fullMark: 60 },
];

// Daily Focus vs Rest Minutes (Mon–Sun)
const DAILY_FOCUS_TRENDS = [
  { day: 'Mon', focusMins: 195, restMins: 60, breathworkCount: 3 },
  { day: 'Tue', focusMins: 220, restMins: 75, breathworkCount: 4 },
  { day: 'Wed', focusMins: 180, restMins: 90, breathworkCount: 2 },
  { day: 'Thu', focusMins: 240, restMins: 80, breathworkCount: 3 },
  { day: 'Fri', focusMins: 210, restMins: 70, breathworkCount: 5 },
  { day: 'Sat', focusMins: 160, restMins: 110, breathworkCount: 3 },
  { day: 'Sun', focusMins: 130, restMins: 140, breathworkCount: 4 },
];

// Energy state distribution
const ENERGY_DISTRIBUTION = [
  { tier: 'High Alert', count: 12, fill: '#65E6C4' },
  { tier: 'Balanced', count: 18, fill: '#8B9CFF' },
  { tier: 'Low Energy', count: 6, fill: '#FFD078' },
  { tier: 'Depleted', count: 2, fill: '#FF8A8A' },
];

export const InsightsPage: React.FC = () => {
  const [activeView, setActiveView] = useState<'radar' | 'timeline'>('radar');

  return (
    <div className="h-full flex flex-col p-6 space-y-6 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] overflow-y-auto">
      {/* Top Header & Anti-Toxic Productivity Guarantee */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-[var(--border-subtle)] gap-4">
        <div>
          <div className="flex items-center space-x-2">
            <span className="text-xl">📊</span>
            <h1 className="text-lg font-bold">Weekly Life-Balance Analytics</h1>
            <span className="text-[10px] px-2.5 py-0.5 rounded-full bg-[#65E6C4]/15 text-[#65E6C4] font-semibold border border-[#65E6C4]/30">
              Non-Punitive Wellbeing
            </span>
          </div>
          <p className="text-xs text-[var(--text-muted)] mt-1">
            Holistic distribution across all 9 life dimensions • No shaming scores • Zero competitive pressure
          </p>
        </div>

        <div className="flex items-center space-x-1 p-1 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs">
          <button
            type="button"
            onClick={() => setActiveView('radar')}
            className={`px-3 py-1.5 rounded-lg font-semibold transition-all ${
              activeView === 'radar'
                ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] shadow-sm'
                : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            🕸️ 9-Dimension Radar
          </button>
          <button
            type="button"
            onClick={() => setActiveView('timeline')}
            className={`px-3 py-1.5 rounded-lg font-semibold transition-all ${
              activeView === 'timeline'
                ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] shadow-sm'
                : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            ⏱️ Daily Focus vs Rest
          </button>
        </div>
      </div>

      {/* Main Charts Area */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left 2 Columns: Visual Analytics */}
        <div className="lg:col-span-2 space-y-6">
          <GlassSurface variant="solid-card" className="p-5 rounded-2xl space-y-4">
            <div className="flex items-center justify-between text-xs">
              <span className="font-bold text-[var(--text-dominant)] flex items-center space-x-2">
                <span>{activeView === 'radar' ? '🕸️ 9-Pillar Balance Symmetry' : '📈 Daily Study vs Rest Minutes'}</span>
              </span>
              <span className="text-[11px] font-mono text-[var(--text-muted)]">Past 7 Days</span>
            </div>

            <div className="h-72 w-full">
              {activeView === 'radar' ? (
                <ResponsiveContainer width="100%" height="100%">
                  <RadarChart cx="50%" cy="50%" outerRadius="75%" data={NINE_DIMENSIONS_DATA}>
                    <PolarGrid stroke="var(--border-subtle)" strokeOpacity={0.6} />
                    <PolarAngleAxis
                      dataKey="dimension"
                      tick={{ fill: 'var(--text-muted)', fontSize: 10, fontWeight: 500 }}
                    />
                    <PolarRadiusAxis angle={30} domain={[0, 60]} tick={false} axisLine={false} />
                    <Radar
                      name="Actual Hours"
                      dataKey="hours"
                      stroke="#65E6C4"
                      fill="#65E6C4"
                      fillOpacity={0.4}
                    />
                    <Radar
                      name="Harmonious Target"
                      dataKey="target"
                      stroke="#8B9CFF"
                      fill="#8B9CFF"
                      fillOpacity={0.15}
                    />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: '#111827',
                        borderColor: '#8B9CFF40',
                        borderRadius: '12px',
                        fontSize: '11px',
                      }}
                    />
                  </RadarChart>
                </ResponsiveContainer>
              ) : (
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={DAILY_FOCUS_TRENDS}>
                    <defs>
                      <linearGradient id="focusGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#8B9CFF" stopOpacity={0.5} />
                        <stop offset="95%" stopColor="#8B9CFF" stopOpacity={0.0} />
                      </linearGradient>
                      <linearGradient id="restGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#65E6C4" stopOpacity={0.5} />
                        <stop offset="95%" stopColor="#65E6C4" stopOpacity={0.0} />
                      </linearGradient>
                    </defs>
                    <XAxis dataKey="day" stroke="var(--text-muted)" fontSize={11} />
                    <YAxis stroke="var(--text-muted)" fontSize={11} unit="m" />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: '#111827',
                        borderColor: '#65E6C440',
                        borderRadius: '12px',
                        fontSize: '11px',
                      }}
                    />
                    <Area
                      type="monotone"
                      dataKey="focusMins"
                      name="Focus Minutes"
                      stroke="#8B9CFF"
                      strokeWidth={2}
                      fillOpacity={1}
                      fill="url(#focusGradient)"
                    />
                    <Area
                      type="monotone"
                      dataKey="restMins"
                      name="Rest & Movement"
                      stroke="#65E6C4"
                      strokeWidth={2}
                      fillOpacity={1}
                      fill="url(#restGradient)"
                    />
                  </AreaChart>
                </ResponsiveContainer>
              )}
            </div>

            <div className="flex items-center justify-center space-x-6 text-[11px] font-mono text-[var(--text-muted)] pt-2 border-t border-[var(--border-subtle)]/50">
              <span className="flex items-center space-x-1.5">
                <span className="w-2.5 h-2.5 rounded-full bg-[#65E6C4]" />
                <span>Actual Invested Hours</span>
              </span>
              <span className="flex items-center space-x-1.5">
                <span className="w-2.5 h-2.5 rounded-full bg-[#8B9CFF]" />
                <span>Balanced Guideline</span>
              </span>
            </div>
          </GlassSurface>

          {/* Energy Distribution Bar */}
          <GlassSurface variant="solid-card" className="p-5 rounded-2xl space-y-3">
            <div className="flex items-center justify-between text-xs">
              <span className="font-bold text-[var(--text-dominant)]">⚡ Weekly Energy State Distribution</span>
              <span className="text-[11px] font-mono text-[var(--text-muted)]">38 Check-ins</span>
            </div>

            <div className="h-32 w-full">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={ENERGY_DISTRIBUTION} layout="vertical">
                  <XAxis type="number" stroke="var(--text-muted)" fontSize={10} />
                  <YAxis dataKey="tier" type="category" stroke="var(--text-muted)" fontSize={10} width={80} />
                  <Tooltip
                    contentStyle={{
                      backgroundColor: '#111827',
                      borderColor: '#65E6C440',
                      borderRadius: '8px',
                      fontSize: '11px',
                    }}
                  />
                  <Bar dataKey="count" radius={[0, 6, 6, 0]} fill="#65E6C4" />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </GlassSurface>
        </div>

        {/* Right Column: CoachAgent Written Observations (Anti-Toxic Feedback) */}
        <div className="space-y-4">
          <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-3">
            <div className="flex items-center space-x-2">
              <span className="text-xl">🤖</span>
              <div>
                <h3 className="text-sm font-bold text-[var(--text-dominant)]">
                  CoachAgent Observations
                </h3>
                <p className="text-[10px] text-[var(--text-muted)]">
                  Empathetic qualitative reflection • No shaming
                </p>
              </div>
            </div>

            <div className="space-y-3 pt-2 text-xs leading-relaxed text-[var(--text-muted)]">
              <div className="p-3 rounded-xl bg-[var(--surface-primary)] border border-[#65E6C4]/30 space-y-1">
                <span className="text-xs font-bold text-[#65E6C4] flex items-center space-x-1">
                  <span>🌿</span>
                  <span>Circadian Sleep Protection</span>
                </span>
                <p className="text-[11px]">
                  You honored your 22:00 sleep routine on 6 out of 7 nights. This allowed deep-stage hippocampal consolidation of your Class 12 Physics syllabus.
                </p>
              </div>

              <div className="p-3 rounded-xl bg-[var(--surface-primary)] border border-[#8B9CFF]/30 space-y-1">
                <span className="text-xs font-bold text-[#8B9CFF] flex items-center space-x-1">
                  <span>🚶</span>
                  <span>Physical Capillary Flow</span>
                </span>
                <p className="text-[11px]">
                  Logging 7 hours of outdoor walks and stretching directly corresponded with lower afternoon fatigue ratings on Tuesday and Thursday.
                </p>
              </div>

              <div className="p-3 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] space-y-1">
                <span className="text-xs font-bold text-[#FFD078] flex items-center space-x-1">
                  <span>🎮</span>
                  <span>Intentional Play Integration</span>
                </span>
                <p className="text-[11px]">
                  Your 6 hours of intentional, budgeted gaming were completely guilt-free and did not spill into late-night hours. Excellent self-regulation!
                </p>
              </div>
            </div>
          </div>

          {/* 9 Pillars Quick Status Checklist */}
          <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
            <span className="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
              Pillar Harmony Index
            </span>
            <div className="space-y-1.5 text-xs">
              {NINE_DIMENSIONS_DATA.slice(0, 5).map((d) => (
                <div key={d.dimension} className="flex items-center justify-between py-1 border-b border-[var(--border-subtle)]/40">
                  <span className="text-[var(--text-muted)]">{d.dimension}</span>
                  <span className="font-mono font-bold text-[#65E6C4]">
                    {d.hours}h <span className="text-[10px] text-[var(--text-muted)]">/ {d.target}h</span>
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
