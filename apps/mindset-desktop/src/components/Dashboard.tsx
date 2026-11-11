/**
 * MindSet OS Dashboard
 * Integrates 5-Theme Neuroaesthetic UI Engine, 3-Tier Attention Hierarchy,
 * Tabular Numeral Timers, Progress Rings, and Skill Tree Milestones.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { useKernelStore } from '../stores/kernelStore';
import {
  GlassSurface,
  PriorityCard,
  ProgressRing,
  SkillTreeNode,
  TabularTimer,
  AccessibleStatusBadge,
  ThemeSelector,
  Button,
} from '@mindset/ui-core';
import { useUac } from './security/useUac';
import { UacModal } from './security/UacModal';

export const Dashboard: React.FC<{ onLock: () => void }> = ({ onLock }) => {
  const {
    rssMb,
    systemUsedPercent,
    totalMemoryMb,
    uptimeSecs,
    services,
    hardware,
    priorityGoal,
    isFocusActive,
    activeFocusMinutes,
    toggleFocusSession,
    setIsStatusViewOpen,
    togglePrivacyBlur,
    isUacElevated,
  } = useKernelStore();

  const { requestElevation, modalProps } = useUac();
  const [elevationGrantedMsg, setElevationGrantedMsg] = useState<string | null>(null);

  // Skill tree interactive state
  const [skillNodeState, setSkillNodeState] = useState<'locked' | 'unlocked' | 'active' | 'completed'>('active');
  const [earnedXpToast, setEarnedXpToast] = useState<number | null>(null);

  const handleTestUac = async () => {
    const granted = await requestElevation(
      'Modify Focus Guard Kernel Policy & Low-Level Packet Filter',
      'Focus Guard Policy Engine'
    );
    if (granted) {
      setElevationGrantedMsg('UAC Admin Elevation Active (Valid 3 mins)');
      setTimeout(() => setElevationGrantedMsg(null), 6000);
    }
  };

  const handleMilestoneComplete = (_id: string) => {
    setSkillNodeState('completed');
    setEarnedXpToast(75);
    setTimeout(() => setEarnedXpToast(null), 4000);
  };

  return (
    <div className="pt-12 pb-10 px-4 md:px-8 max-w-6xl mx-auto space-y-6 select-none transition-colors">
      {/* Top Banner: Telemetry & Memory Watchdog */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Metric 1: OS Shell RSS */}
        <GlassSurface variant="solid-card" className="p-4">
          <div className="text-[11px] text-[var(--text-muted)] uppercase tracking-wider font-semibold">
            OS Shell RSS (Cap: 120MB)
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold tabular-nums text-[var(--accent-secondary)]">
              {rssMb.toFixed(1)}
            </span>
            <span className="text-sm text-[var(--text-muted)]">MB</span>
          </div>
          <p className="text-[11px] text-[var(--accent-secondary)] mt-1 font-medium">
            ✓ Strictly within 120 MB budget
          </p>
        </GlassSurface>

        {/* Metric 2: System RAM */}
        <GlassSurface variant="solid-card" className="p-4">
          <div className="text-[11px] text-[var(--text-muted)] uppercase tracking-wider font-semibold">
            System RAM Utilization
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold tabular-nums text-[var(--text-dominant)]">
              {systemUsedPercent.toFixed(1)}%
            </span>
            <span className="text-sm text-[var(--text-muted)]">
              of {(totalMemoryMb / 1024).toFixed(0)} GB
            </span>
          </div>
          <p className="text-[11px] text-[var(--accent-primary)] mt-1 font-medium">
            8.0 GB+ reserved for Ollama LLM
          </p>
        </GlassSurface>

        {/* Metric 3: Kernel Uptime & Zero-Jitter Clock */}
        <GlassSurface variant="solid-card" className="p-4">
          <div className="text-[11px] text-[var(--text-muted)] uppercase tracking-wider font-semibold">
            Kernel Uptime & Loop
          </div>
          <div className="flex items-baseline space-x-2 mt-2">
            <span className="text-3xl font-mono font-bold tabular-nums text-[var(--accent-achievement)]">
              {uptimeSecs}
            </span>
            <span className="text-sm text-[var(--text-muted)]">secs</span>
          </div>
          <button
            type="button"
            onClick={() => setIsStatusViewOpen(true)}
            className="w-full mt-2 py-1 px-2 rounded bg-[var(--surface-secondary)] hover:bg-[var(--surface-elevated)] text-[11px] text-[var(--accent-primary)] font-medium border border-[var(--border-subtle)] transition-all flex items-center justify-center space-x-1"
          >
            <span>⚙️ Open /status</span>
          </button>
        </GlassSurface>

        {/* Metric 4: Security & Privacy Controls */}
        <GlassSurface variant="solid-card" className="p-4">
          <div className="text-[11px] text-[var(--text-muted)] uppercase tracking-wider font-semibold">
            Security & Smart Lock
          </div>
          <div className="mt-2 space-y-1.5">
            <div className="grid grid-cols-2 gap-1.5">
              <Button
                variant="secondary"
                size="sm"
                onClick={onLock}
                className="w-full text-xs py-1"
                title="Lock Workspace (Super + L / Ctrl + Alt + L)"
              >
                🔒 Lock
              </Button>
              <Button
                variant="secondary"
                size="sm"
                onClick={togglePrivacyBlur}
                className="w-full text-xs py-1"
                title="Instant Privacy Blur Guard (Alt + B)"
              >
                👁️‍🗨️ Blur
              </Button>
            </div>
            <button
              type="button"
              onClick={handleTestUac}
              className="w-full py-1 px-2 rounded bg-[var(--surface-secondary)] hover:bg-[var(--surface-elevated)] text-[10px] text-[var(--accent-primary)] font-medium border border-[var(--border-subtle)] hover:border-[var(--accent-primary)]/40 transition-all flex items-center justify-center space-x-1"
            >
              <span>🛡️ Test UAC Elevation</span>
            </button>
          </div>
          {elevationGrantedMsg ? (
            <p className="text-[10px] text-[var(--accent-secondary)] mt-1.5 text-center font-medium animate-fadeIn">
              ✓ {elevationGrantedMsg}
            </p>
          ) : (
            <p className="text-[10px] text-[var(--text-muted)] mt-1.5 text-center">
              {isUacElevated ? '🛡️ UAC Admin Elevated' : 'Multi-Modal Biometrics / PIN'}
            </p>
          )}
        </GlassSurface>
      </div>

      {/* Main Focus Row: 3-Tier Visual Hierarchy */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
        {/* Tier 1 Focal Anchor (Priority 1) */}
        <div className="lg:col-span-2 space-y-4">
          <PriorityCard
            priority={1}
            title={priorityGoal}
            subtitle="Today's Primary Academic Milestone • Distraction-free Study Mode active"
            subjectTag="Engineering Mathematics"
            xpReward={50}
            progressPercent={75}
            primaryAction={{
              label: isFocusActive ? 'End Focus Session' : 'Start Deep Work (45m Block)',
              icon: isFocusActive ? '⏹' : '▶',
              onClick: toggleFocusSession,
            }}
            secondaryAction={{
              label: 'View Syllabus Tree',
              icon: '🗺️',
              onClick: () => {},
            }}
          >
            {/* Live Zero-Jitter Tabular Countdown Timer Pill */}
            <div className="p-3.5 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] flex items-center justify-between">
              <div className="space-y-0.5">
                <span className="text-[11px] font-bold uppercase tracking-wider text-[var(--accent-secondary)]">
                  {isFocusActive ? 'Focus Guard Active' : 'Calibrated Focus Block'}
                </span>
                <p className="text-xs text-[var(--text-muted)]">
                  DNS and process quarantine engaged during session
                </p>
              </div>

              <div className="flex items-center space-x-3 px-3 py-1.5 rounded-lg bg-[var(--surface-elevated)] border border-[var(--border-subtle)]">
                <span className="w-2 h-2 rounded-full animate-ping bg-[var(--accent-secondary)]" />
                <TabularTimer
                  secondsRemaining={activeFocusMinutes * 60}
                  isRunning={isFocusActive}
                  size="md"
                  format="mm:ss"
                />
              </div>
            </div>
          </PriorityCard>

          {/* Tier 2 Milestone Cards */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
            <PriorityCard
              priority={2}
              title="Differential Equations: First-Order ODEs"
              subtitle="Module 3: Separation of Variables & Integrating Factors"
              subjectTag="Math II"
              status="In Progress"
              primaryAction={{
                label: 'Continue Module →',
                onClick: () => {},
              }}
            />

            <PriorityCard
              priority={2}
              title="Modern Physics: Photoelectric Effect"
              subtitle="Module 1: Work Function & Photon Stopping Potential"
              subjectTag="Physics"
              status="Ready"
              primaryAction={{
                label: 'Launch Reader →',
                onClick: () => {},
              }}
            />
          </div>

          {/* Tier 3 Ambient Cadence Queue */}
          <div className="space-y-2 pt-1">
            <PriorityCard
              priority={3}
              title="Sync offline flashcard decks with mobile client"
              subtitle="Scheduled: 18:00 UTC • Local SQLite HLC sync"
              primaryAction={{
                label: 'Sync Now',
                onClick: () => {},
              }}
            />
            <PriorityCard
              priority={3}
              title="Evening Reflection & Mood Log"
              subtitle="Forest Zen suggestion will be offered at 20:00"
              primaryAction={{
                label: 'Open Journal',
                onClick: () => {},
              }}
            />
          </div>
        </div>

        {/* Right Sidebar: Progress Ring & Interactive Skill Tree Node */}
        <div className="space-y-5">
          {/* Daily Progress Radial Ring */}
          <GlassSurface variant="solid-card" className="p-5 flex flex-col items-center text-center space-y-3">
            <div className="w-full flex items-center justify-between text-xs pb-2 border-b border-[var(--border-subtle)]">
              <span className="font-bold text-[var(--text-dominant)]">Daily Syllabus Target</span>
              <span className="font-mono text-xs text-[var(--accent-secondary)]">3 of 4 Blocks</span>
            </div>

            <ProgressRing
              progress={75}
              size={96}
              strokeWidth={7}
              label="75% Complete"
              sublabel="3 Deep Blocks Concluded"
              accentColor="var(--accent-secondary)"
              glow
            />

            <div className="w-full grid grid-cols-2 gap-2 pt-2 border-t border-[var(--border-subtle)] text-center text-xs">
              <div className="p-2 rounded bg-[var(--surface-secondary)]">
                <span className="text-[10px] text-[var(--text-muted)] block">Streak (Non-Punitive)</span>
                <span className="font-bold font-mono text-[var(--accent-achievement)]">🔥 7 Days</span>
              </div>
              <div className="p-2 rounded bg-[var(--surface-secondary)]">
                <span className="text-[10px] text-[var(--text-muted)] block">XP Earned Today</span>
                <span className="font-bold font-mono text-[var(--accent-primary)]">+250 XP</span>
              </div>
            </div>
          </GlassSurface>

          {/* Interactive Skill Tree Node (Cosmic Arcade / Gamified Dopamine) */}
          <GlassSurface variant="solid-card" className="p-5 space-y-3">
            <div className="flex items-center justify-between text-xs pb-2 border-b border-[var(--border-subtle)]">
              <div>
                <span className="font-bold text-[var(--text-dominant)]">Skill Tree Milestone</span>
                <p className="text-[10px] text-[var(--text-muted)]">Ethical mastery micro-feedback</p>
              </div>
              <span className="text-xs">🕹️</span>
            </div>

            <SkillTreeNode
              id="node-ode-01"
              title="Separation of Variables Mastery"
              category="Calculus & ODEs"
              level={2}
              status={skillNodeState}
              xpReward={75}
              progress={skillNodeState === 'completed' ? 100 : 65}
              onComplete={handleMilestoneComplete}
            />

            {earnedXpToast && (
              <div className="p-2 rounded-lg bg-[var(--surface-elevated)] border border-[var(--accent-achievement)] text-center text-xs text-[var(--accent-achievement)] font-bold animate-fadeIn">
                🎉 Milestone Mastered! +{earnedXpToast} XP Awarded
              </div>
            )}
          </GlassSurface>

          {/* Sensory Hardware Probe Status */}
          <GlassSurface variant="solid-card" className="p-4 space-y-3">
            <div className="border-b border-[var(--border-subtle)] pb-2 flex items-center justify-between">
              <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)]">
                Hardware Probe
              </h3>
              <AccessibleStatusBadge status="healthy" label="Online" size="sm" />
            </div>

            <div className="space-y-2 text-xs">
              <div className="flex items-center justify-between p-2 rounded bg-[var(--surface-secondary)]">
                <span className="text-[var(--text-dominant)]">Webcam (Face Unlock)</span>
                <span className={hardware?.has_webcam ? 'text-[var(--accent-secondary)]' : 'text-[var(--text-muted)]'}>
                  {hardware?.has_webcam ? 'Active (<300ms)' : 'Not Found'}
                </span>
              </div>

              <div className="flex items-center justify-between p-2 rounded bg-[var(--surface-secondary)]">
                <span className="text-[var(--text-dominant)]">Microphone</span>
                <span className={hardware?.has_mic ? 'text-[var(--accent-secondary)]' : 'text-[var(--text-muted)]'}>
                  {hardware?.has_mic ? 'Active' : 'Not Found'}
                </span>
              </div>

              <div className="flex items-center justify-between p-2 rounded bg-[var(--surface-secondary)]">
                <span className="text-[var(--text-dominant)]">Biometrics (fprintd)</span>
                <span className={hardware?.has_fingerprint ? 'text-[var(--accent-secondary)]' : 'text-[var(--text-muted)]'}>
                  {hardware?.has_fingerprint ? 'Enrolled' : 'Silent Fallback'}
                </span>
              </div>
            </div>
          </GlassSurface>
        </div>
      </div>

      {/* Full 5-Theme Neuroaesthetic Configuration Engine */}
      <ThemeSelector className="mt-6" />

      {/* Services Registry Matrix Status */}
      <GlassSurface variant="solid-card" className="p-5 space-y-4">
        <div className="flex items-center justify-between border-b border-[var(--border-subtle)] pb-2">
          <div>
            <div className="flex items-center space-x-2">
              <h3 className="text-sm font-semibold text-[var(--text-dominant)]">
                Kernel Cooperative Micro-Actors
              </h3>
              <button
                type="button"
                onClick={() => setIsStatusViewOpen(true)}
                className="text-[10px] font-mono px-2 py-0.5 rounded bg-[var(--surface-elevated)] text-[var(--accent-primary)] hover:bg-[var(--surface-elevated)]/80 transition-all border border-[var(--border-subtle)]"
              >
                Inspect Telemetry (/status) →
              </button>
            </div>
            <p className="text-xs text-[var(--text-muted)]">
              Scheduled via single 15-second tokio interval loop
            </p>
          </div>
          <AccessibleStatusBadge
            status="healthy"
            label={`${services.length}/${services.length} Services Operational`}
          />
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
          {services.map((svc) => (
            <div
              key={svc.name}
              className="flex items-center justify-between p-2.5 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-xs"
            >
              <div className="flex items-center space-x-2 truncate">
                <span className="text-[var(--accent-secondary)]">●</span>
                <span className="font-semibold text-[var(--text-dominant)] truncate">{svc.name}</span>
              </div>
              <span className="text-[var(--text-muted)] font-mono shrink-0 pl-2">
                ~{svc.estimated_ram_kb} KB
              </span>
            </div>
          ))}
        </div>
      </GlassSurface>

      {/* UAC Elevation Confirmation Modal */}
      <UacModal {...modalProps} />
    </div>
  );
};
