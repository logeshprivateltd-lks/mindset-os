/**
 * MindSet OS 3-Tier Priority Card Component
 * Enforces Cognitive Caloric Conservation by visually delineating:
 * - Priority 1: Dominant Focal Anchor (~50% attention, glass blur, border glow, single dominant CTA)
 * - Priority 2: Secondary Context (~35% attention, layered opaque surface, secondary CTA)
 * - Priority 3: Ambient Cadence (~15% attention, compact, muted typography)
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { GlassSurface } from './GlassSurface';

export interface PriorityAction {
  label: string;
  onClick: () => void;
  icon?: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'accent';
  disabled?: boolean;
}

export interface PriorityCardProps {
  priority: 1 | 2 | 3;
  title: string;
  subtitle?: string;
  subjectTag?: string;
  xpReward?: number;
  status?: string;
  progressPercent?: number;
  primaryAction?: PriorityAction;
  secondaryAction?: PriorityAction;
  children?: React.ReactNode;
  className?: string;
  onClick?: () => void;
}

export const PriorityCard: React.FC<PriorityCardProps> = ({
  priority,
  title,
  subtitle,
  subjectTag,
  xpReward,
  status,
  progressPercent,
  primaryAction,
  secondaryAction,
  children,
  className = '',
  onClick,
}) => {
  // -------------------------------------------------------------
  // TIER 1: DOMINANT FOCAL ANCHOR (Priority 1)
  // -------------------------------------------------------------
  if (priority === 1) {
    return (
      <GlassSurface
        variant="hero-card"
        glow
        onClick={onClick}
        className={`p-5 md:p-6 space-y-4 ${className}`}
      >
        {/* Header Ribbon */}
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <span className="w-2.5 h-2.5 rounded-full animate-pulse" style={{ backgroundColor: 'var(--accent-primary)' }} />
            <span
              className="text-xs font-bold uppercase tracking-wider"
              style={{ color: 'var(--accent-primary)' }}
            >
              Priority 1 Focus Anchor
            </span>
            {subjectTag && (
              <span
                className="px-2 py-0.5 rounded text-[11px] font-medium border"
                style={{
                  backgroundColor: 'var(--surface-elevated)',
                  borderColor: 'var(--border-subtle)',
                  color: 'var(--text-dominant)',
                }}
              >
                {subjectTag}
              </span>
            )}
          </div>

          {xpReward !== undefined && (
            <span
              className="px-2.5 py-1 rounded text-xs font-bold font-mono border"
              style={{
                backgroundColor: 'rgba(255, 208, 120, 0.12)',
                borderColor: 'rgba(255, 208, 120, 0.35)',
                color: 'var(--accent-achievement, #FFD078)',
              }}
            >
              +{xpReward} XP
            </span>
          )}
        </div>

        {/* Title & Subtitle */}
        <div>
          <h2
            className="text-xl md:text-2xl font-bold tracking-tight leading-snug"
            style={{ color: 'var(--text-dominant)' }}
          >
            {title}
          </h2>
          {subtitle && (
            <p className="text-sm mt-1 leading-relaxed" style={{ color: 'var(--text-muted)' }}>
              {subtitle}
            </p>
          )}
        </div>

        {/* Optional Progress Bar */}
        {progressPercent !== undefined && (
          <div className="space-y-1.5 pt-1">
            <div className="flex justify-between text-xs tabular-nums">
              <span style={{ color: 'var(--text-muted)' }}>Progress Target</span>
              <span className="font-semibold" style={{ color: 'var(--accent-secondary)' }}>
                {Math.round(progressPercent)}%
              </span>
            </div>
            <div
              className="w-full h-2 rounded-full overflow-hidden"
              style={{ backgroundColor: 'var(--surface-secondary)' }}
            >
              <div
                className="h-full rounded-full transition-all duration-300 ease-out"
                style={{
                  width: `${Math.min(100, Math.max(0, progressPercent))}%`,
                  backgroundColor: 'var(--accent-secondary)',
                }}
              />
            </div>
          </div>
        )}

        {children && <div className="pt-2">{children}</div>}

        {/* Action Row: Single Dominant Primary CTA */}
        {(primaryAction || secondaryAction) && (
          <div className="flex items-center space-x-3 pt-2">
            {primaryAction && (
              <button
                type="button"
                onClick={primaryAction.onClick}
                disabled={primaryAction.disabled}
                className="flex-1 py-2.5 px-4 rounded-xl text-sm font-bold shadow-lg transition-all hover:scale-[1.01] active:scale-[0.99] flex items-center justify-center space-x-2"
                style={{
                  backgroundColor: 'var(--accent-primary)',
                  color: '#07090D',
                }}
              >
                {primaryAction.icon && <span>{primaryAction.icon}</span>}
                <span>{primaryAction.label}</span>
              </button>
            )}

            {secondaryAction && (
              <button
                type="button"
                onClick={secondaryAction.onClick}
                disabled={secondaryAction.disabled}
                className="py-2.5 px-4 rounded-xl text-sm font-semibold border transition-all hover:bg-[var(--surface-elevated)]"
                style={{
                  backgroundColor: 'var(--surface-secondary)',
                  borderColor: 'var(--border-subtle)',
                  color: 'var(--text-dominant)',
                }}
              >
                {secondaryAction.icon && <span>{secondaryAction.icon}</span>}
                <span>{secondaryAction.label}</span>
              </button>
            )}
          </div>
        )}
      </GlassSurface>
    );
  }

  // -------------------------------------------------------------
  // TIER 2: SECONDARY CONTEXT (Priority 2)
  // -------------------------------------------------------------
  if (priority === 2) {
    return (
      <GlassSurface
        variant="solid-card"
        onClick={onClick}
        className={`p-4 space-y-3 ${className}`}
      >
        <div className="flex items-center justify-between text-xs">
          <div className="flex items-center space-x-2">
            <span
              className="text-[11px] font-bold uppercase tracking-wider"
              style={{ color: 'var(--accent-secondary)' }}
            >
              Priority 2 Milestone
            </span>
            {subjectTag && (
              <span
                className="px-1.5 py-0.5 rounded text-[10px] border"
                style={{
                  backgroundColor: 'var(--surface-secondary)',
                  borderColor: 'var(--border-subtle)',
                  color: 'var(--text-muted)',
                }}
              >
                {subjectTag}
              </span>
            )}
          </div>

          {status && (
            <span
              className="text-[11px] font-medium px-2 py-0.5 rounded"
              style={{
                backgroundColor: 'var(--surface-elevated)',
                color: 'var(--accent-secondary)',
              }}
            >
              {status}
            </span>
          )}
        </div>

        <div>
          <h3
            className="text-base font-semibold leading-snug"
            style={{ color: 'var(--text-dominant)' }}
          >
            {title}
          </h3>
          {subtitle && (
            <p className="text-xs mt-1" style={{ color: 'var(--text-muted)' }}>
              {subtitle}
            </p>
          )}
        </div>

        {children}

        {primaryAction && (
          <div className="pt-1">
            <button
              type="button"
              onClick={primaryAction.onClick}
              disabled={primaryAction.disabled}
              className="w-full py-1.5 px-3 rounded-lg text-xs font-semibold border transition-all hover:bg-[var(--surface-elevated)]"
              style={{
                backgroundColor: 'var(--surface-secondary)',
                borderColor: 'var(--border-subtle)',
                color: 'var(--text-dominant)',
              }}
            >
              {primaryAction.label}
            </button>
          </div>
        )}
      </GlassSurface>
    );
  }

  // -------------------------------------------------------------
  // TIER 3: AMBIENT CADENCE (Priority 3)
  // -------------------------------------------------------------
  return (
    <div
      onClick={onClick}
      className={`p-3 rounded-lg border transition-colors flex items-center justify-between space-x-3 ${className}`}
      style={{
        backgroundColor: 'var(--surface-secondary)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <div className="flex-1 min-w-0">
        <div className="flex items-center space-x-2">
          <span className="text-[10px] uppercase font-bold" style={{ color: 'var(--text-dim, #6B7280)' }}>
            P3
          </span>
          <h4
            className="text-xs font-medium truncate"
            style={{ color: 'var(--text-dominant)' }}
          >
            {title}
          </h4>
        </div>
        {subtitle && (
          <p className="text-[11px] truncate mt-0.5" style={{ color: 'var(--text-muted)' }}>
            {subtitle}
          </p>
        )}
      </div>

      {primaryAction && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            primaryAction.onClick();
          }}
          className="text-xs px-2 py-1 rounded text-[var(--accent-primary)] hover:bg-[var(--surface-elevated)] transition-colors shrink-0"
        >
          {primaryAction.label}
        </button>
      )}
    </div>
  );
};

