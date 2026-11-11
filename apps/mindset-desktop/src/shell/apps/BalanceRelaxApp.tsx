/**
 * MindSet OS Balance AI & Relaxation Native Window App
 * Cortisol-reducing breathwork guide, mood logs, and acoustic relaxation
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';

export const BalanceRelaxApp: React.FC = () => {
  const [phase, setPhase] = useState<'Inhale' | 'Hold' | 'Exhale'>('Inhale');
  const [seconds, setSeconds] = useState(4);
  const [isActive, setIsActive] = useState(false);

  useEffect(() => {
    if (!isActive) return;

    const timer = setInterval(() => {
      setSeconds((prev) => {
        if (prev <= 1) {
          if (phase === 'Inhale') {
            setPhase('Hold');
            return 7;
          } else if (phase === 'Hold') {
            setPhase('Exhale');
            return 8;
          } else {
            setPhase('Inhale');
            return 4;
          }
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [isActive, phase]);

  const toggleBreathwork = () => {
    setIsActive(!isActive);
    if (!isActive) {
      setPhase('Inhale');
      setSeconds(4);
    }
  };

  return (
    <div className="h-full flex flex-col items-center justify-between p-8 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] text-center">
      {/* Header */}
      <div>
        <span className="text-xs font-bold uppercase tracking-wider text-[var(--accent-secondary)]">
          MindSet Balance AI • Biophilic Restoration
        </span>
        <h2 className="text-xl font-bold mt-1 text-[var(--text-dominant)]">
          4-7-8 Cortisol-Reducing Breathwork
        </h2>
        <p className="text-xs text-[var(--text-muted)] mt-1 max-w-sm mx-auto">
          Calms the sympathetic nervous system between high-intensity study blocks.
        </p>
      </div>

      {/* Interactive Breathing Orb */}
      <div className="flex flex-col items-center justify-center my-6">
        <div
          className={`w-48 h-48 rounded-full flex flex-col items-center justify-center transition-all duration-1000 border-2 shadow-2xl ${
            isActive && phase === 'Inhale'
              ? 'scale-125 border-[var(--accent-secondary)] shadow-[0_0_36px_var(--border-glow)]'
              : isActive && phase === 'Hold'
              ? 'scale-125 border-[var(--accent-primary)] shadow-[0_0_36px_rgba(139,156,255,0.4)]'
              : 'scale-90 border-[var(--border-subtle)]'
          }`}
          style={{
            backgroundColor: 'var(--surface-secondary)',
          }}
        >
          <span className="text-lg font-bold uppercase tracking-wider text-[var(--accent-secondary)]">
            {isActive ? phase : 'Ready'}
          </span>
          <span className="text-5xl font-mono font-bold tabular-nums text-[var(--text-dominant)] mt-2">
            {isActive ? seconds : '4-7-8'}
          </span>
          <span className="text-[10px] text-[var(--text-muted)] mt-1">
            {isActive ? 'Seconds' : 'Tap to start'}
          </span>
        </div>
      </div>

      {/* Controls & Ambient Sounds */}
      <div className="w-full max-w-xs space-y-3">
        <button
          type="button"
          onClick={toggleBreathwork}
          className="w-full py-2.5 rounded-xl font-bold text-xs transition-all hover:scale-102"
          style={{
            backgroundColor: isActive ? 'var(--accent-danger, #FF6B6B)' : 'var(--accent-secondary)',
            color: '#07090D',
          }}
        >
          {isActive ? 'Stop Session' : 'Start 4-7-8 Breathing'}
        </button>

        <div className="flex justify-center space-x-2 text-xs text-[var(--text-muted)]">
          <span>🌿 Forest Ambient</span>
          <span>•</span>
          <span>🌧️ Gentle Rain</span>
          <span>•</span>
          <span>🌊 Tide Flow</span>
        </div>
      </div>
    </div>
  );
};

