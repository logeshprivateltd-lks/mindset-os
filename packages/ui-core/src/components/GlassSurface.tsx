/**
 * Controlled Liquid Glass Container Component
 * Strictly regulates GPU backdrop blur to permitted spatial zones (nav, modal, hero-card)
 * while maintaining layered opaque surfaces for standard solid cards to conserve RAM and GPU.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';

export type GlassVariant = 'nav' | 'modal' | 'hero-card' | 'solid-card';

export interface GlassSurfaceProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: GlassVariant;
  glow?: boolean;
  as?: React.ElementType;
}

export const GlassSurface: React.FC<GlassSurfaceProps> = ({
  variant = 'solid-card',
  glow = false,
  as: Component = 'div',
  className = '',
  style = {},
  children,
  ...rest
}) => {
  const getVariantStyles = (): { baseClasses: string; inlineStyles: React.CSSProperties } => {
    switch (variant) {
      case 'nav':
        return {
          baseClasses: 'rounded-xl border backdrop-blur-md transition-colors',
          inlineStyles: {
            backgroundColor: 'var(--glass-bg)',
            borderColor: glow ? 'var(--accent-primary)' : 'var(--glass-border)',
            boxShadow: glow
              ? '0 4px 20px var(--border-glow)'
              : 'var(--glass-shadow, 0 8px 32px rgba(0,0,0,0.4))',
          },
        };
      case 'modal':
        return {
          baseClasses: 'rounded-2xl border backdrop-blur-xl shadow-2xl transition-all',
          inlineStyles: {
            backgroundColor: 'var(--glass-bg)',
            borderColor: glow ? 'var(--accent-primary)' : 'var(--glass-border)',
            boxShadow: '0 24px 64px rgba(0, 0, 0, 0.7), 0 0 24px var(--border-glow)',
          },
        };
      case 'hero-card':
        return {
          baseClasses: 'rounded-2xl border backdrop-blur-md transition-all',
          inlineStyles: {
            backgroundColor: 'var(--glass-bg)',
            borderColor: 'var(--accent-primary)',
            boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5), 0 0 24px var(--border-glow)',
          },
        };
      case 'solid-card':
      default:
        // Zero backdrop-filter blur! Pure layered opaque surface to protect battery & iGPU
        return {
          baseClasses: 'rounded-xl border transition-colors',
          inlineStyles: {
            backgroundColor: 'var(--surface-primary)',
            borderColor: glow ? 'var(--accent-primary)' : 'var(--border-subtle)',
            boxShadow: glow ? '0 4px 16px var(--border-glow)' : 'none',
          },
        };
    }
  };

  const { baseClasses, inlineStyles } = getVariantStyles();

  return (
    <Component
      className={`${baseClasses} ${className}`}
      style={{ ...inlineStyles, ...style }}
      {...rest}
    >
      {children}
    </Component>
  );
};
