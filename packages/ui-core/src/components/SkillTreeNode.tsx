/**
 * MindSet OS Skill Tree Milestone Node Component
 * Interactive Cosmic Arcade / Aurora Focus syllabus milestone with ethical completion feedback
 * (smooth progress bar fill, subtle #FFD078 achievement badge, zero predatory flash/streak shaming).
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';

export type SkillNodeStatus = 'locked' | 'unlocked' | 'active' | 'completed';

export interface SkillTreeNodeProps {
  id: string;
  title: string;
  category?: string;
  level?: number;
  status: SkillNodeStatus;
  xpReward?: number;
  icon?: string | React.ReactNode;
  progress?: number; // 0 to 100
  onSelect?: (id: string) => void;
  onComplete?: (id: string) => void;
  className?: string;
}

export const SkillTreeNode: React.FC<SkillTreeNodeProps> = ({
  id,
  title,
  category,
  level = 1,
  status,
  xpReward = 50,
  icon,
  progress = 0,
  onSelect,
  onComplete,
  className = '',
}) => {
  const [showAura, setShowAura] = useState(false);

  const handleNodeClick = () => {
    if (status === 'locked') return;
    if (onSelect) onSelect(id);
  };

  const handleCompleteClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (status === 'locked' || status === 'completed') return;

    // Trigger subtle ethical 200ms golden aura
    setShowAura(true);
    setTimeout(() => setShowAura(false), 450);

    if (onComplete) onComplete(id);
  };

  const getStatusBorder = () => {
    switch (status) {
      case 'completed':
        return 'border-[var(--accent-achievement,#FFD078)] shadow-[0_0_16px_rgba(255,208,120,0.3)]';
      case 'active':
        return 'border-[var(--accent-secondary,#65E6C4)] shadow-[0_0_16px_var(--border-glow)] animate-pulse';
      case 'unlocked':
        return 'border-[var(--accent-primary,#8B9CFF)] hover:shadow-[0_0_12px_var(--border-glow)]';
      case 'locked':
      default:
        return 'border-[var(--border-subtle)] opacity-60 cursor-not-allowed';
    }
  };

  return (
    <div
      onClick={handleNodeClick}
      className={`relative p-3.5 rounded-xl border transition-all duration-200 select-none ${getStatusBorder()} ${
        status !== 'locked' ? 'cursor-pointer hover:scale-[1.02]' : ''
      } ${className}`}
      style={{
        backgroundColor:
          status === 'completed'
            ? 'rgba(255, 208, 120, 0.05)'
            : 'var(--surface-primary)',
      }}
    >
      {/* Gentle Achievement Aura Ripple */}
      {showAura && (
        <div
          className="absolute inset-0 rounded-xl pointer-events-none animate-ping"
          style={{
            border: '2px solid var(--accent-achievement, #FFD078)',
            backgroundColor: 'rgba(255, 208, 120, 0.15)',
          }}
        />
      )}

      <div className="flex items-center space-x-3">
        {/* Node Glyph Circle */}
        <div
          className="w-10 h-10 rounded-xl flex items-center justify-center text-lg border shrink-0 transition-colors"
          style={{
            backgroundColor: 'var(--surface-elevated)',
            borderColor:
              status === 'completed'
                ? 'var(--accent-achievement, #FFD078)'
                : 'var(--border-subtle)',
          }}
        >
          {status === 'completed' ? (
            <span role="img" aria-label="Completed" className="text-base text-[var(--accent-achievement,#FFD078)]">
              ✓
            </span>
          ) : status === 'locked' ? (
            <span role="img" aria-label="Locked" className="text-xs text-[var(--text-muted)]">
              🔒
            </span>
          ) : (
            icon || <span>★</span>
          )}
        </div>

        {/* Title & Category Info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between">
            <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
              {category || `Level ${level}`}
            </span>
            <span
              className="text-[11px] font-mono font-bold tabular-nums"
              style={{
                color:
                  status === 'completed'
                    ? 'var(--accent-achievement, #FFD078)'
                    : 'var(--accent-primary)',
              }}
            >
              +{xpReward} XP
            </span>
          </div>

          <h4
            className="text-xs font-bold truncate mt-0.5"
            style={{ color: 'var(--text-dominant)' }}
          >
            {title}
          </h4>

          {/* Progress / State indicator */}
          <div className="flex items-center justify-between mt-2 pt-1.5 border-t border-[var(--border-subtle)] text-[10px]">
            <span style={{ color: 'var(--text-muted)' }}>
              {status === 'completed'
                ? 'Mastered'
                : status === 'active'
                ? 'In Progress'
                : status === 'unlocked'
                ? 'Available'
                : 'Prerequisite Required'}
            </span>

            {status === 'active' && onComplete && (
              <button
                type="button"
                onClick={handleCompleteClick}
                className="px-2 py-0.5 rounded text-[10px] font-bold transition-all hover:scale-105"
                style={{
                  backgroundColor: 'var(--accent-achievement, #FFD078)',
                  color: '#07090D',
                }}
              >
                Mark Mastered ✓
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Progress fill bar if partial */}
      {status === 'active' && progress > 0 && progress < 100 && (
        <div className="w-full h-1 mt-2.5 rounded-full overflow-hidden bg-[var(--surface-secondary)]">
          <div
            className="h-full rounded-full transition-all duration-300"
            style={{
              width: `${progress}%`,
              backgroundColor: 'var(--accent-secondary)',
            }}
          />
        </div>
      )}
    </div>
  );
};
