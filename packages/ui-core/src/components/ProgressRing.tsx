/**
 * MindSet OS Radial Progress Ring Component
 * SVG tabular-numeral daily completion ring with smooth deterministic SVG stroke offset
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';

export interface ProgressRingProps {
  progress: number; // 0 to 100
  size?: number;
  strokeWidth?: number;
  label?: string;
  sublabel?: string;
  accentColor?: string;
  trackColor?: string;
  showPercentage?: boolean;
  glow?: boolean;
  className?: string;
}

export const ProgressRing: React.FC<ProgressRingProps> = ({
  progress,
  size = 84,
  strokeWidth = 6,
  label,
  sublabel,
  accentColor = 'var(--accent-secondary)',
  trackColor = 'var(--surface-elevated)',
  showPercentage = true,
  glow = false,
  className = '',
}) => {
  const clampedProgress = Math.min(100, Math.max(0, progress));
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference * (1 - clampedProgress / 100);

  return (
    <div className={`flex flex-col items-center justify-center select-none ${className}`}>
      <div className="relative flex items-center justify-center" style={{ width: size, height: size }}>
        <svg
          width={size}
          height={size}
          viewBox={`0 0 ${size} ${size}`}
          className="transform -rotate-90"
        >
          {/* Background Track Circle */}
          <circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke={trackColor}
            strokeWidth={strokeWidth}
            fill="transparent"
          />

          {/* Dynamic Progress Circle */}
          <circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke={accentColor}
            strokeWidth={strokeWidth}
            strokeDasharray={circumference}
            strokeDashoffset={strokeDashoffset}
            strokeLinecap="round"
            fill="transparent"
            style={{
              transition: 'stroke-dashoffset 280ms cubic-bezier(0.16, 1, 0.3, 1)',
              filter: glow ? 'drop-shadow(0 0 6px var(--border-glow))' : 'none',
            }}
          />
        </svg>

        {/* Center Readout */}
        {showPercentage && (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-center">
            <span
              className="text-base font-bold font-mono tabular-nums leading-none tracking-tight"
              style={{ color: 'var(--text-dominant)' }}
            >
              {Math.round(clampedProgress)}%
            </span>
          </div>
        )}
      </div>

      {label && (
        <div className="text-xs font-semibold mt-2 text-center" style={{ color: 'var(--text-dominant)' }}>
          {label}
        </div>
      )}
      {sublabel && (
        <div className="text-[11px] text-center" style={{ color: 'var(--text-muted)' }}>
          {sublabel}
        </div>
      )}
    </div>
  );
};
