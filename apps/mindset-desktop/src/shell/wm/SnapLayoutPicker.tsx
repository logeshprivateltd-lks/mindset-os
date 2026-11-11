/**
 * MindSet OS Snap Layout Hover Picker Component
 * Interactive Windows 11 / KDE Plasma style grid picker appearing when hovering over
 * the window maximize button, enabling one-click 2-grid, 3-grid, or 4-grid placement.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import type { SnapZone } from './useWindowManagerStore';

export interface SnapLayoutPickerProps {
  onSelectZone: (zone: SnapZone) => void;
  onClose: () => void;
}

export const SnapLayoutPicker: React.FC<SnapLayoutPickerProps> = ({ onSelectZone, onClose }) => {
  return (
    <div
      role="menu"
      aria-label="Snap Layouts"
      onMouseLeave={onClose}
      className="absolute top-8 right-2 z-50 p-3 rounded-xl border shadow-2xl backdrop-blur-xl animate-fadeIn select-none"
      style={{
        backgroundColor: 'var(--glass-bg)',
        borderColor: 'var(--accent-primary)',
        boxShadow: '0 16px 40px rgba(0, 0, 0, 0.7), 0 0 16px var(--border-glow)',
        width: 250,
      }}
    >
      <div className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)] mb-2.5">
        Multi-Window Snap Grids
      </div>

      <div className="grid grid-cols-3 gap-2.5">
        {/* Layout 1: 2-Grid 50/50 Split */}
        <div className="space-y-1">
          <div className="text-[9px] text-[var(--text-muted)] font-mono text-center">2-Split (50/50)</div>
          <div className="h-14 rounded-lg border border-[var(--border-subtle)] p-1 flex space-x-1 bg-[var(--surface-primary)]">
            <button
              type="button"
              onClick={() => onSelectZone('left-50')}
              title="Snap Left 50% (Study Reader)"
              className="flex-1 rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
            <button
              type="button"
              onClick={() => onSelectZone('right-50')}
              title="Snap Right 50% (Notes / Tasks)"
              className="flex-1 rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
          </div>
        </div>

        {/* Layout 2: 3-Grid Study Split (50% Left / 25% Top-Right / 25% Bottom-Right) */}
        <div className="space-y-1">
          <div className="text-[9px] text-[var(--text-muted)] font-mono text-center">3-Study (50/25/25)</div>
          <div className="h-14 rounded-lg border border-[var(--border-subtle)] p-1 flex space-x-1 bg-[var(--surface-primary)]">
            <button
              type="button"
              onClick={() => onSelectZone('3-grid-left')}
              title="Snap Main 50% Left"
              className="flex-1 rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
            <div className="flex-1 flex flex-col space-y-1">
              <button
                type="button"
                onClick={() => onSelectZone('3-grid-top-right')}
                title="Snap Top-Right 25% (AI Tutor)"
                className="flex-1 rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
              />
              <button
                type="button"
                onClick={() => onSelectZone('3-grid-bottom-right')}
                title="Snap Bottom-Right 25% (Tasks)"
                className="flex-1 rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
              />
            </div>
          </div>
        </div>

        {/* Layout 3: 4-Grid Quadrant Split */}
        <div className="space-y-1">
          <div className="text-[9px] text-[var(--text-muted)] font-mono text-center">4-Quadrant</div>
          <div className="h-14 rounded-lg border border-[var(--border-subtle)] p-1 grid grid-cols-2 gap-1 bg-[var(--surface-primary)]">
            <button
              type="button"
              onClick={() => onSelectZone('quad-top-left')}
              title="Snap Top-Left"
              className="rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
            <button
              type="button"
              onClick={() => onSelectZone('quad-top-right')}
              title="Snap Top-Right"
              className="rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
            <button
              type="button"
              onClick={() => onSelectZone('quad-bottom-left')}
              title="Snap Bottom-Left"
              className="rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
            <button
              type="button"
              onClick={() => onSelectZone('quad-bottom-right')}
              title="Snap Bottom-Right"
              className="rounded bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)] hover:opacity-90 transition-colors"
            />
          </div>
        </div>
      </div>

      <div className="mt-2.5 pt-2 border-t border-[var(--border-subtle)] text-center">
        <span className="text-[10px] text-[var(--text-muted)] font-mono">
          Hotkey: <kbd className="text-[var(--accent-primary)]">Super + Arrow Keys</kbd>
        </span>
      </div>
    </div>
  );
};

