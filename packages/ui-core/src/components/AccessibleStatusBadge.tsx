/**
 * MindSet OS Accessible Status Badge Component
 * Colour-independent status indicator combining WCAG 2.2 AA contrast,
 * distinct geometric shapes (checkmark, warning triangle, alert octagon), and full aria support.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';

export type AccessibleStatus = 'healthy' | 'warning' | 'critical' | 'neutral' | 'active';

export interface AccessibleStatusBadgeProps {
  status: AccessibleStatus;
  label: string;
  subtext?: string;
  size?: 'sm' | 'md' | 'lg';
  showIcon?: boolean;
  className?: string;
}

export const AccessibleStatusBadge: React.FC<AccessibleStatusBadgeProps> = ({
  status,
  label,
  subtext,
  size = 'md',
  showIcon = true,
  className = '',
}) => {
  const getStatusConfig = () => {
    switch (status) {
      case 'healthy':
        return {
          icon: '✓',
          shape: 'circle',
          bg: 'rgba(101, 230, 196, 0.12)',
          border: 'rgba(101, 230, 196, 0.35)',
          text: 'var(--accent-secondary, #65E6C4)',
          ariaText: 'Status: Healthy, normal operation',
        };
      case 'active':
        return {
          icon: '●',
          shape: 'pulse-circle',
          bg: 'rgba(139, 156, 255, 0.15)',
          border: 'rgba(139, 156, 255, 0.40)',
          text: 'var(--accent-primary, #8B9CFF)',
          ariaText: 'Status: Active, currently executing',
        };
      case 'warning':
        return {
          icon: '▲',
          shape: 'triangle',
          bg: 'rgba(255, 208, 120, 0.12)',
          border: 'rgba(255, 208, 120, 0.40)',
          text: 'var(--accent-achievement, #FFD078)',
          ariaText: 'Status: Warning, attention required',
        };
      case 'critical':
        return {
          icon: '⬢',
          shape: 'octagon',
          bg: 'rgba(255, 107, 107, 0.15)',
          border: 'rgba(255, 107, 107, 0.45)',
          text: 'var(--accent-danger, #FF6B6B)',
          ariaText: 'Status: Critical, immediate action needed',
        };
      case 'neutral':
      default:
        return {
          icon: '○',
          shape: 'circle',
          bg: 'var(--surface-elevated)',
          border: 'var(--border-subtle)',
          text: 'var(--text-muted)',
          ariaText: 'Status: Neutral or idle',
        };
    }
  };

  const config = getStatusConfig();

  const getSizeClasses = () => {
    switch (size) {
      case 'sm':
        return 'px-2 py-0.5 text-[10px] space-x-1';
      case 'lg':
        return 'px-3.5 py-1.5 text-sm space-x-2';
      case 'md':
      default:
        return 'px-2.5 py-1 text-xs space-x-1.5';
    }
  };

  return (
    <div
      role="status"
      aria-label={`${config.ariaText}: ${label}`}
      className={`inline-flex items-center rounded-full font-medium border select-none transition-colors ${getSizeClasses()} ${className}`}
      style={{
        backgroundColor: config.bg,
        borderColor: config.border,
        color: config.text,
      }}
    >
      {showIcon && (
        <span
          className={`font-bold leading-none ${
            status === 'active' ? 'animate-pulse' : ''
          }`}
          aria-hidden="true"
        >
          {config.icon}
        </span>
      )}
      <span className="font-semibold">{label}</span>
      {subtext && (
        <span className="opacity-80 text-[0.9em]" aria-hidden="true">
          • {subtext}
        </span>
      )}
    </div>
  );
};

