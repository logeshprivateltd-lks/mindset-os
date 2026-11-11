/**
 * MindSet OS Tabular Timer & Clock Component
 * High-precision, zero-jitter tabular numeral timer enforcing `font-variant-numeric: tabular-nums`
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';

export interface TabularTimerProps {
  secondsRemaining?: number;
  totalSeconds?: number;
  mode?: 'countdown' | 'clock' | 'stopwatch';
  isRunning?: boolean;
  format?: 'hh:mm:ss' | 'mm:ss' | 'auto';
  size?: 'sm' | 'md' | 'lg' | 'hero';
  label?: string;
  glow?: boolean;
  className?: string;
  onComplete?: () => void;
}

export const TabularTimer: React.FC<TabularTimerProps> = ({
  secondsRemaining: propSeconds,
  totalSeconds = 1500, // 25 mins default
  mode = 'countdown',
  isRunning = true,
  format = 'auto',
  size = 'md',
  label,
  glow = false,
  className = '',
  onComplete,
}) => {
  const [seconds, setSeconds] = useState<number>(propSeconds !== undefined ? propSeconds : totalSeconds);
  const [clockTime, setClockTime] = useState(new Date());

  // Clock mode internal ticker
  useEffect(() => {
    if (mode !== 'clock') return;
    const interval = setInterval(() => setClockTime(new Date()), 1000);
    return () => clearInterval(interval);
  }, [mode]);

  // Sync prop changes
  useEffect(() => {
    if (propSeconds !== undefined) {
      setSeconds(propSeconds);
    }
  }, [propSeconds]);

  // Countdown / Stopwatch ticker
  useEffect(() => {
    if (mode === 'clock' || !isRunning) return;

    const timer = setInterval(() => {
      setSeconds((prev) => {
        if (mode === 'countdown') {
          if (prev <= 1) {
            clearInterval(timer);
            if (onComplete) onComplete();
            return 0;
          }
          return prev - 1;
        } else {
          // stopwatch
          return prev + 1;
        }
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [isRunning, mode, onComplete]);

  // Format Helper
  const getFormattedTime = (): string => {
    if (mode === 'clock') {
      return clockTime.toLocaleTimeString([], {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false,
      });
    }

    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    const pad = (n: number) => String(n).padStart(2, '0');

    if (format === 'hh:mm:ss' || (format === 'auto' && hrs > 0)) {
      return `${pad(hrs)}:${pad(mins)}:${pad(secs)}`;
    }
    return `${pad(mins)}:${pad(secs)}`;
  };

  const getSizeClasses = () => {
    switch (size) {
      case 'sm':
        return 'text-sm font-semibold';
      case 'lg':
        return 'text-3xl md:text-4xl font-bold tracking-tight';
      case 'hero':
        return 'text-5xl md:text-7xl font-bold tracking-tighter drop-shadow-lg';
      case 'md':
      default:
        return 'text-xl md:text-2xl font-bold';
    }
  };

  return (
    <div className={`inline-flex flex-col items-center select-none ${className}`}>
      {label && (
        <span
          className="text-[11px] font-semibold uppercase tracking-wider mb-1"
          style={{ color: 'var(--text-muted)' }}
        >
          {label}
        </span>
      )}

      <div
        className={`font-mono tabular-nums leading-none tracking-normal transition-all ${getSizeClasses()}`}
        style={{
          color: 'var(--text-dominant)',
          fontVariantNumeric: 'tabular-nums',
          fontFeatureSettings: '"tnum"',
          textShadow: glow ? '0 0 16px var(--border-glow)' : 'none',
        }}
        aria-live="off"
        aria-label={`Timer: ${getFormattedTime()}`}
      >
        {getFormattedTime()}
      </div>
    </div>
  );
};
