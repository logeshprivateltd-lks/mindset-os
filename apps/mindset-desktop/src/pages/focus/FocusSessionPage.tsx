/**
 * MindSet OS — Immersive Focus Session Interface
 * Minimal #07090D Aurora Focus interface with tabular timer, ambient audio synthesis & completion modal
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useRef, useCallback } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import { useKernelStore } from '../../stores/useKernelStore';
import { dbClient } from '../../lib/dbClient';
import { ambientSoundSynthesizer, AmbientSoundType } from './ambientSoundSynthesizer';
import type { TaskOrGoal } from '@mindset/db-schema';

export interface FocusSessionPageProps {
  initialMinutes?: number;
  task?: TaskOrGoal | null;
  onExit?: () => void;
  onOpenBalance?: () => void;
}

export const FocusSessionPage: React.FC<FocusSessionPageProps> = ({
  initialMinutes = 45,
  task,
  onExit,
  onOpenBalance,
}) => {
  const { setFocusActive } = useKernelStore();

  const [totalSeconds] = useState<number>(initialMinutes * 60);
  const [secondsRemaining, setSecondsRemaining] = useState<number>(initialMinutes * 60);
  const [isPaused, setIsPaused] = useState<boolean>(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [activeSound, setActiveSound] = useState<AmbientSoundType>('none');
  const [soundVolume, setSoundVolume] = useState<number>(40);
  const [showSummary, setShowSummary] = useState<boolean>(false);
  const [interruptionCount] = useState<number>(0);

  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Initialize SQLite focus session on start and activate Focus Guard
  useEffect(() => {
    let mounted = true;
    setFocusActive(true);

    dbClient.wellbeing
      .startFocusSession({
        profile_mode: 'DeepWork',
        planned_minutes: initialMinutes,
        task_id: task?.id || null,
      })
      .then((session) => {
        if (mounted) setSessionId(session.id);
      })
      .catch((err) => console.error('[FocusSession] Failed to start:', err));

    return () => {
      mounted = false;
      setFocusActive(false);
      ambientSoundSynthesizer.stop();
    };
  }, [initialMinutes, task, setFocusActive]);

  // Countdown timer loop
  useEffect(() => {
    if (isPaused || showSummary) {
      if (timerRef.current) clearInterval(timerRef.current);
      return;
    }

    timerRef.current = setInterval(() => {
      setSecondsRemaining((prev) => {
        if (prev <= 1) {
          clearInterval(timerRef.current!);
          handleSessionComplete();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, [isPaused, showSummary]);

  const handleSoundChange = (type: AmbientSoundType) => {
    setActiveSound(type);
    ambientSoundSynthesizer.play(type);
  };

  const handleVolumeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const vol = Number(e.target.value);
    setSoundVolume(vol);
    ambientSoundSynthesizer.setVolume(vol / 100);
  };

  const handleSessionComplete = useCallback(async () => {
    ambientSoundSynthesizer.stop();
    setShowSummary(true);

    const actualMinutes = Math.round((totalSeconds - secondsRemaining) / 60);
    if (sessionId) {
      try {
        await dbClient.wellbeing.endFocusSession(sessionId, actualMinutes, interruptionCount);
      } catch (err) {
        console.error('[FocusSession] End session error:', err);
      }
    }
  }, [totalSeconds, secondsRemaining, sessionId, interruptionCount]);

  const handleEndEarly = () => {
    handleSessionComplete();
  };

  const progressPercent = Math.min(
    100,
    Math.round(((totalSeconds - secondsRemaining) / totalSeconds) * 100)
  );

  const formatTime = (secs: number) => {
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  return (
    <div className="relative w-full h-full min-h-[600px] flex flex-col items-center justify-between p-8 bg-[#07090D] text-[var(--text-dominant)] select-none overflow-hidden transition-colors">
      {/* Ambient background glow ring */}
      <div className="absolute inset-0 flex items-center justify-center pointer-events-none opacity-25">
        <div className="w-[500px] h-[500px] rounded-full bg-gradient-to-tr from-[#8B9CFF]/30 via-[#65E6C4]/20 to-transparent blur-3xl animate-pulse" />
      </div>

      {/* Top Header: Focus Profile Badge & Interrupt count */}
      <div className="relative z-10 w-full max-w-3xl flex items-center justify-between text-xs">
        <div className="flex items-center space-x-2">
          <span className="w-2.5 h-2.5 rounded-full bg-[#65E6C4] shadow-[0_0_8px_#65E6C4] animate-ping" />
          <span className="font-mono font-bold tracking-wider text-[#65E6C4]">
            FOCUS GUARD ACTIVE • DEEP STUDY
          </span>
        </div>

        <div className="flex items-center space-x-3 text-[var(--text-muted)] font-mono">
          <span>Session Target: {initialMinutes}m</span>
          <span>•</span>
          <span>{interruptionCount === 0 ? 'Zero Interruptions' : `${interruptionCount} Interruptions`}</span>
        </div>
      </div>

      {/* Center Stage: Large Tabular Countdown & Task Title */}
      <div className="relative z-10 flex flex-col items-center space-y-6 my-auto text-center">
        {/* Priority 1 Task Banner */}
        <div className="px-4 py-1.5 rounded-full bg-[#182235]/80 border border-[#8B9CFF]/40 text-xs text-[#8B9CFF] font-medium flex items-center space-x-2 shadow-lg">
          <span>🎯 Current Target:</span>
          <span className="font-bold text-[var(--text-dominant)]">
            {task?.title || 'Master Core Concept Problem Set'}
          </span>
        </div>

        {/* Large Monospace Tabular Countdown Display */}
        <div className="text-7xl sm:text-8xl md:text-9xl font-mono font-extrabold tracking-tight tabular-nums text-[var(--text-dominant)] drop-shadow-[0_0_35px_rgba(139,156,255,0.25)]">
          {formatTime(secondsRemaining)}
        </div>

        {/* Progress Bar */}
        <div className="w-72 sm:w-96 space-y-2">
          <div className="w-full h-2 rounded-full bg-[#111827] overflow-hidden border border-[var(--border-subtle)]">
            <div
              className="h-full bg-gradient-to-r from-[#8B9CFF] to-[#65E6C4] rounded-full transition-all duration-300"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
          <div className="flex justify-between text-[11px] font-mono text-[var(--text-muted)]">
            <span>{progressPercent}% Complete</span>
            <span>{Math.round(secondsRemaining / 60)}m Remaining</span>
          </div>
        </div>
      </div>

      {/* Bottom Controls Bar: Ambient Synthesizer & Pause/End Actions */}
      <div className="relative z-10 w-full max-w-3xl flex flex-col sm:flex-row items-center justify-between gap-4 pt-4 border-t border-[var(--border-subtle)]/50">
        {/* Offline Ambient Sound Controls */}
        <div className="flex items-center space-x-3 text-xs">
          <span className="text-[var(--text-muted)] flex items-center space-x-1">
            <span>🎧</span>
            <span>Ambient:</span>
          </span>

          <div className="flex items-center space-x-1">
            {[
              { id: 'none' as const, label: 'Silent' },
              { id: 'rain' as const, label: '🌧️ Rain' },
              { id: 'brown_noise' as const, label: '📻 Brown' },
              { id: 'binaural_focus' as const, label: '🧠 Binaural' },
            ].map((snd) => (
              <button
                key={snd.id}
                type="button"
                onClick={() => handleSoundChange(snd.id)}
                className={`px-2.5 py-1 rounded-lg font-medium transition-all ${
                  activeSound === snd.id
                    ? 'bg-[#8B9CFF] text-[#07090D] font-bold shadow-sm'
                    : 'bg-[#182235] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {snd.label}
              </button>
            ))}
          </div>

          {activeSound !== 'none' && (
            <input
              type="range"
              min="0"
              max="100"
              value={soundVolume}
              onChange={handleVolumeChange}
              className="w-20 accent-[#8B9CFF] cursor-pointer"
              title={`Ambient Volume: ${soundVolume}%`}
            />
          )}
        </div>

        {/* Action Controls */}
        <div className="flex items-center space-x-3">
          <Button
            variant="secondary"
            size="sm"
            onClick={() => setIsPaused(!isPaused)}
            className="text-xs font-semibold px-4"
          >
            {isPaused ? '▶️ Resume' : '⏸️ Pause'}
          </Button>

          <Button
            variant="danger"
            size="sm"
            onClick={handleEndEarly}
            className="text-xs font-semibold px-4"
          >
            ⏹️ End Session
          </Button>
        </div>
      </div>

      {/* ============================================================= */}
      {/* COMPLETION SUMMARY MODAL                                      */}
      {/* ============================================================= */}
      {showSummary && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-xl animate-fadeIn">
          <GlassSurface variant="modal" className="max-w-md w-full p-6 text-center space-y-5 animate-scaleUp">
            <span className="text-4xl">🌟</span>
            <div className="space-y-1">
              <h3 className="text-xl font-bold text-[var(--text-dominant)]">
                Focus Session Complete!
              </h3>
              <p className="text-xs text-[var(--text-muted)]">
                Outstanding effort. You protected your attention and advanced your goals.
              </p>
            </div>

            {/* Achievement Matrix */}
            <div className="grid grid-cols-2 gap-3 py-2 text-left">
              <div className="p-3 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <div className="text-[10px] text-[var(--text-muted)] uppercase font-bold">Focus Time</div>
                <div className="text-xl font-mono font-bold text-[#65E6C4]">
                  {Math.round((totalSeconds - secondsRemaining) / 60)} mins
                </div>
              </div>

              <div className="p-3 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <div className="text-[10px] text-[var(--text-muted)] uppercase font-bold">Reward</div>
                <div className="text-xl font-mono font-bold text-[#FFD078]">
                  +50 XP Earned
                </div>
              </div>
            </div>

            {/* MindSet Balance AI Recommendation */}
            <div className="p-3.5 rounded-xl bg-[#65E6C4]/10 border border-[#65E6C4]/30 text-xs text-[#65E6C4] text-left space-y-1">
              <div className="font-bold flex items-center space-x-1.5">
                <span>🌿</span>
                <span>MindSet Balance AI Break Advice:</span>
              </div>
              <div>Take a 5-minute physical walk, drink water, and let your subconscious integrate what you studied.</div>
            </div>

            {/* Modal Exit Buttons */}
            <div className="flex items-center space-x-3 pt-2">
              {onOpenBalance && (
                <Button
                  variant="secondary"
                  className="w-1/2 text-xs font-semibold"
                  onClick={() => {
                    setShowSummary(false);
                    onOpenBalance();
                  }}
                >
                  🧘 4-7-8 Breathwork
                </Button>
              )}

              <Button
                variant="primary"
                className="w-full text-xs font-semibold"
                onClick={() => {
                  setShowSummary(false);
                  if (onExit) onExit();
                }}
              >
                Return to Dashboard
              </Button>
            </div>
          </GlassSurface>
        </div>
      )}
    </div>
  );
};
