/**
 * MindSet OS Desktop Context Menu Component
 * Liquid-glass right-click context menu for wallpaper canvas and desktop shortcut icons
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useEffect, useRef } from 'react';
import type { DesktopShortcut } from '@mindset/db-schema';
import { useWindowManagerStore } from '../../shell/wm/useWindowManagerStore';
import { usePersonalizationStore, WALLPAPER_PRESETS } from '../../stores/usePersonalizationStore';

export interface DesktopContextMenuProps {
  x: number;
  y: number;
  targetShortcut?: DesktopShortcut | null;
  onClose: () => void;
  onOpenStatus: () => void;
  onLock: () => void;
}

export const DesktopContextMenu: React.FC<DesktopContextMenuProps> = ({
  x,
  y,
  targetShortcut,
  onClose,
  onOpenStatus,
  onLock,
}) => {
  const menuRef = useRef<HTMLDivElement>(null);
  const { openWindow } = useWindowManagerStore();
  const {
    deleteShortcut,
    wallpaperPath,
    setWallpaperPreset,
    executePowerAction,
  } = usePersonalizationStore();

  // Clamp menu coordinates inside viewport
  const menuWidth = 230;
  const menuHeight = targetShortcut ? 180 : 360;
  const clampedX = Math.min(window.innerWidth - menuWidth - 12, Math.max(12, x));
  const clampedY = Math.min(window.innerHeight - menuHeight - 12, Math.max(12, y));

  // Global click-away listener
  useEffect(() => {
    const handleOutsideClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };
    window.addEventListener('mousedown', handleOutsideClick);
    return () => window.removeEventListener('mousedown', handleOutsideClick);
  }, [onClose]);

  // Next Wallpaper Quick Cycle
  const handleCycleNextWallpaper = () => {
    const currentIndex = WALLPAPER_PRESETS.findIndex((p) => p.id === wallpaperPath);
    const nextIndex = (currentIndex + 1) % WALLPAPER_PRESETS.length;
    const nextPreset = WALLPAPER_PRESETS[nextIndex] || WALLPAPER_PRESETS[0];
    if (nextPreset) {
      setWallpaperPreset(nextPreset.id);
    }
    onClose();
  };

  return (
    <div
      ref={menuRef}
      role="menu"
      aria-label="Desktop Context Menu"
      onClick={(e) => e.stopPropagation()}
      className="fixed z-50 p-1.5 rounded-2xl border shadow-2xl backdrop-blur-2xl animate-fadeIn select-none text-xs text-[var(--text-dominant)] space-y-0.5"
      style={{
        top: clampedY,
        left: clampedX,
        width: menuWidth,
        backgroundColor: 'rgba(15, 23, 42, 0.88)',
        borderColor: 'rgba(139, 156, 255, 0.35)',
        boxShadow: '0 20px 50px rgba(0, 0, 0, 0.7), 0 0 20px rgba(139, 156, 255, 0.2)',
      }}
    >
      {targetShortcut ? (
        // Shortcut-specific Context Menu
        <>
          <div className="px-3 py-1.5 flex items-center space-x-2 border-b border-[var(--border-subtle)] mb-1">
            <span className="text-base">{targetShortcut.icon}</span>
            <span className="font-semibold truncate">{targetShortcut.title}</span>
          </div>

          <button
            type="button"
            onClick={() => {
              openWindow(targetShortcut.target_payload, {
                title: targetShortcut.title,
                icon: targetShortcut.icon,
              });
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🚀</span>
            <span>Open Application</span>
          </button>

          <button
            type="button"
            onClick={() => {
              deleteShortcut(targetShortcut.id);
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 text-red-400 hover:bg-red-500/15 transition-colors"
          >
            <span>🗑️</span>
            <span>Remove from Desktop</span>
          </button>
        </>
      ) : (
        // Wallpaper Canvas Context Menu
        <>
          <button
            type="button"
            onClick={() => {
              openWindow('journal', { title: 'Quick Reflection', icon: '✍️' });
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>✍️</span>
            <span>New Note / Reflection</span>
          </button>

          <button
            type="button"
            onClick={() => {
              openWindow('terminal', { title: 'MindSet Terminal', icon: '💻' });
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>💻</span>
            <span>Open Terminal (Ctrl + ~)</span>
          </button>

          <button
            type="button"
            onClick={() => {
              openWindow('files', { title: 'Study Vault & RAG', icon: '🗂️' });
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>📁</span>
            <span>Browse Study Files</span>
          </button>

          <div className="h-px my-1 bg-[var(--border-subtle)]" />

          <button
            type="button"
            onClick={handleCycleNextWallpaper}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🖼️</span>
            <span>Next Wallpaper Theme</span>
          </button>

          <button
            type="button"
            onClick={() => {
              openWindow('settings', { title: 'Personalization Studio', icon: '🎨' });
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🎨</span>
            <span>Personalization Studio...</span>
          </button>

          <button
            type="button"
            onClick={() => {
              onOpenStatus();
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>⚡</span>
            <span>System Telemetry & Doctor</span>
          </button>

          <div className="h-px my-1 bg-[var(--border-subtle)]" />

          <button
            type="button"
            onClick={() => {
              onLock();
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🔒</span>
            <span>Lock Screen (Super + L)</span>
          </button>

          <button
            type="button"
            onClick={() => {
              executePowerAction('sleep');
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 text-amber-300 hover:bg-amber-400/15 transition-colors"
          >
            <span>🌙</span>
            <span>Sleep (Zero-VRAM Mode)</span>
          </button>

          <button
            type="button"
            onClick={() => {
              executePowerAction('shutdown');
              onClose();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2.5 text-red-400 hover:bg-red-500/15 transition-colors"
          >
            <span>⏻</span>
            <span>Shut Down MindSet OS</span>
          </button>
        </>
      )}
    </div>
  );
};
