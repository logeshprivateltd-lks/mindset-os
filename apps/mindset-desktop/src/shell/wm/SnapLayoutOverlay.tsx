/**
 * MindSet OS Snap Layout Overlay Component
 * Translucent Liquid Glass snap preview rendered when dragging windows or picking layouts
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { useWindowManagerStore, computeSnapBounds } from './useWindowManagerStore';

export const SnapLayoutOverlay: React.FC = () => {
  const { hoveredSnapZone } = useWindowManagerStore();

  if (!hoveredSnapZone) return null;

  const bounds = computeSnapBounds(hoveredSnapZone);

  return (
    <div
      className="fixed pointer-events-none z-40 rounded-2xl border-2 transition-all duration-150 animate-fadeIn"
      style={{
        left: bounds.x + 4,
        top: bounds.y + 4,
        width: bounds.width - 8,
        height: bounds.height - 8,
        backgroundColor: 'rgba(139, 156, 255, 0.12)',
        borderColor: 'var(--accent-primary, #8B9CFF)',
        boxShadow: '0 0 32px var(--border-glow), inset 0 0 24px rgba(139, 156, 255, 0.15)',
        backdropFilter: 'blur(16px)',
        WebkitBackdropFilter: 'blur(16px)',
      }}
    >
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="px-4 py-2 rounded-xl bg-[var(--surface-primary)] border border-[var(--accent-primary)] shadow-lg flex items-center space-x-2">
          <span className="text-lg">🔲</span>
          <span className="text-xs font-bold font-mono uppercase text-[var(--accent-primary)]">
            Snap: {hoveredSnapZone.replace('-', ' ')}
          </span>
        </div>
      </div>
    </div>
  );
};

