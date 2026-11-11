/**
 * MindSet OS — Non-Shaming Focus Guard Interruption Screen
 * Calm #07090D Liquid Glass overlay gently re-centering attention when a distraction is intercepted
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';

export interface FocusInterruptionOverlayProps {
  isOpen: boolean;
  blockedAppName?: string;
  blockedDomain?: string;
  focusProfileName?: string;
  remainingTimeStr?: string;
  currentTaskTitle?: string;
  onReturnToPrevious: () => void;
  onOpenStudyTask: () => void;
  onGrantException?: (reason: string) => void;
}

const EXCEPTION_REASONS = [
  '📖 Need an academic reference video or formula sheet',
  '☕ Taking an early 5-minute hydration break',
  '🥱 Feeling mentally fatigued and need a pause',
  '💬 Urgent communication with family / teacher',
];

export const FocusInterruptionOverlay: React.FC<FocusInterruptionOverlayProps> = ({
  isOpen,
  blockedAppName = 'Restricted Application',
  blockedDomain,
  focusProfileName = 'Study Mode',
  remainingTimeStr = '24:18',
  currentTaskTitle = 'Physics: Rotational Dynamics & Problem Set',
  onReturnToPrevious,
  onOpenStudyTask,
  onGrantException,
}) => {
  const [isExceptionModalOpen, setIsExceptionModalOpen] = useState(false);
  const [selectedReason, setSelectedReason] = useState<string>(EXCEPTION_REASONS[0] ?? '📖 Academic reference');
  const [isLoggedToast, setIsLoggedToast] = useState(false);

  if (!isOpen) return null;

  const handleConfirmException = async () => {
    try {
      // Record to Distraction Journal in SQLite (non-punitive reflection)
      await dbClient.wellbeing.createJournal({
        title: `Controlled Exception: ${blockedAppName}`,
        content_md: `Requested 5-minute focus exception during ${focusProfileName}.\n\n**Reason selected:** ${selectedReason}`,
        gratitude_items_json: '[]',
        mood_tag: 'Focused',
        energy_level: 'Medium',
      });

      setIsLoggedToast(true);
      setTimeout(() => {
        setIsLoggedToast(false);
        setIsExceptionModalOpen(false);
        if (onGrantException) onGrantException(selectedReason);
      }, 1000);
    } catch (err) {
      console.error('[FocusInterruption] Log exception failed:', err);
      setIsExceptionModalOpen(false);
      if (onGrantException) onGrantException(selectedReason);
    }
  };

  return (
    <div className="fixed inset-0 z-[99999] flex items-center justify-center p-4 bg-[#07090D]/95 backdrop-blur-2xl select-none animate-fadeIn">
      {/* Background Calm Ambient Aura */}
      <div className="absolute inset-0 flex items-center justify-center pointer-events-none opacity-20">
        <div className="w-[600px] h-[600px] rounded-full bg-gradient-to-tr from-[#8B9CFF]/30 to-[#65E6C4]/20 blur-3xl" />
      </div>

      <GlassSurface
        variant="modal"
        className="relative z-10 max-w-lg w-full p-8 text-center space-y-6 border border-[#8B9CFF]/40 shadow-2xl animate-scaleUp"
      >
        {/* Calm Status Badge */}
        <div className="inline-flex items-center space-x-2 px-3.5 py-1 rounded-full bg-[#182235] border border-[#8B9CFF]/40 text-xs font-mono text-[#8B9CFF]">
          <span className="w-2 h-2 rounded-full bg-[#65E6C4] animate-pulse" />
          <span>FOCUS GUARD ACTIVE • {focusProfileName.toUpperCase()}</span>
        </div>

        {/* Supportive Re-Centering Message */}
        <div className="space-y-2">
          <h2 className="text-2xl font-bold tracking-tight text-[var(--text-dominant)]">
            Gentle Check-In
          </h2>
          <p className="text-xs text-[var(--text-muted)] max-w-sm mx-auto leading-relaxed">
            <strong>{blockedAppName || blockedDomain}</strong> is temporarily restricted to protect your flow state and cognitive momentum.
          </p>
        </div>

        {/* Current Active Commitment Box */}
        <div className="p-4 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-left space-y-2">
          <div className="flex items-center justify-between text-[11px] text-[var(--text-muted)]">
            <span>Planned Commitment:</span>
            <span className="font-mono text-[#FFD078] font-semibold">{remainingTimeStr} remaining</span>
          </div>

          <div className="text-xs font-bold text-[var(--text-dominant)] flex items-center space-x-2">
            <span>🎯</span>
            <span className="truncate">{currentTaskTitle}</span>
          </div>
        </div>

        {/* Non-Shaming Action CTAs */}
        <div className="space-y-2.5 pt-1">
          {/* Primary CTA: Return to previous app */}
          <Button
            variant="primary"
            className="w-full text-xs font-bold py-2.5 shadow-lg"
            onClick={onReturnToPrevious}
          >
            ← Return to Study Application
          </Button>

          {/* Secondary CTA: Open current task */}
          <Button
            variant="secondary"
            className="w-full text-xs font-semibold py-2"
            onClick={onOpenStudyTask}
          >
            📖 Open Current Study Task
          </Button>

          {/* Tertiary Action: Controlled Exception */}
          <button
            type="button"
            onClick={() => setIsExceptionModalOpen(true)}
            className="text-[11px] text-[var(--text-muted)] hover:text-[#8B9CFF] transition-colors underline pt-1"
          >
            Need access? Request Controlled Exception (5 mins)
          </button>
        </div>
      </GlassSurface>

      {/* ============================================================= */}
      {/* CONTROLLED EXCEPTION DIALOG                                   */}
      {/* ============================================================= */}
      {isExceptionModalOpen && (
        <div className="fixed inset-0 z-[100000] flex items-center justify-center p-4 bg-black/80 backdrop-blur-xl animate-fadeIn">
          <GlassSurface variant="modal" className="max-w-md w-full p-6 text-left space-y-4 animate-scaleUp">
            <div className="space-y-1">
              <h3 className="text-sm font-bold text-[var(--text-dominant)] flex items-center space-x-2">
                <span>⏱️</span>
                <span>Controlled 5-Minute Exception</span>
              </h3>
              <p className="text-xs text-[var(--text-muted)]">
                Life happens! Select a quick reason for the Distraction Journal so you stay mindful of your habits.
              </p>
            </div>

            {/* Reasons Picker */}
            <div className="space-y-2">
              {EXCEPTION_REASONS.map((reason) => (
                <label
                  key={reason}
                  className={`flex items-center space-x-2.5 p-2.5 rounded-xl border text-xs cursor-pointer transition-all ${
                    selectedReason === reason
                      ? 'border-[#8B9CFF] bg-[#8B9CFF]/15 text-[var(--text-dominant)] font-semibold'
                      : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:border-[var(--border-strong)]'
                  }`}
                >
                  <input
                    type="radio"
                    name="exception_reason"
                    checked={selectedReason === reason}
                    onChange={() => setSelectedReason(reason)}
                    className="accent-[#8B9CFF]"
                  />
                  <span>{reason}</span>
                </label>
              ))}
            </div>

            {isLoggedToast && (
              <div className="text-xs text-[#65E6C4] font-medium pt-1">
                ✓ Recorded in Distraction Journal. Resuming 5m access...
              </div>
            )}

            {/* Modal Buttons */}
            <div className="flex items-center justify-end space-x-2 pt-2 border-t border-[var(--border-subtle)]">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setIsExceptionModalOpen(false)}
                className="text-xs"
              >
                Cancel
              </Button>
              <Button
                variant="primary"
                size="sm"
                onClick={handleConfirmException}
                className="text-xs font-semibold"
              >
                Confirm 5m Exception
              </Button>
            </div>
          </GlassSurface>
        </div>
      )}
    </div>
  );
};
