/**
 * MindSet OS Desktop Canvas Component
 * Renders calibrated wallpaper, desktop shortcut icons, and rich right-click context menu
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import { useWindowManagerStore } from './wm/useWindowManagerStore';
import { useActivityStore, ACTIVITIES } from './activities/useActivityStore';
import { useThemeStore } from '@mindset/ui-core';

export interface DesktopCanvasProps {
  onOpenStatus: () => void;
  onLock: () => void;
}

interface ContextMenuState {
  isOpen: boolean;
  x: number;
  y: number;
}

export const DesktopCanvas: React.FC<DesktopCanvasProps> = ({ onOpenStatus, onLock }) => {
  const { openWindow } = useWindowManagerStore();
  const { currentActivityId, toggleSwitcher } = useActivityStore();
  const { theme } = useThemeStore();

  const [contextMenu, setContextMenu] = useState<ContextMenuState>({ isOpen: false, x: 0, y: 0 });
  const [selectedShortcut, setSelectedShortcut] = useState<string | null>(null);

  const activeActivity = ACTIVITIES[currentActivityId] || ACTIVITIES['deep-study'];

  // Handle right-click for Desktop Context Menu
  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    setContextMenu({
      isOpen: true,
      x: Math.min(window.innerWidth - 220, e.clientX),
      y: Math.min(window.innerHeight - 240, e.clientY),
    });
  };

  const closeContextMenu = () => {
    if (contextMenu.isOpen) {
      setContextMenu({ isOpen: false, x: 0, y: 0 });
    }
  };

  useEffect(() => {
    const handleGlobalClick = () => closeContextMenu();
    window.addEventListener('click', handleGlobalClick);
    return () => window.removeEventListener('click', handleGlobalClick);
  });

  const handleShortcutLaunch = (appId: string, id: string) => {
    setSelectedShortcut(id);
    openWindow(appId, {}, currentActivityId);
  };

  return (
    <div
      onContextMenu={handleContextMenu}
      onClick={() => {
        setSelectedShortcut(null);
        closeContextMenu();
      }}
      className="absolute inset-0 pt-9 pb-12 select-none overflow-hidden transition-colors"
      style={{
        backgroundImage:
          theme === 'aurora-focus'
            ? 'radial-gradient(circle at 50% 20%, rgba(139, 156, 255, 0.08), transparent 50%), radial-gradient(circle at 80% 80%, rgba(101, 230, 196, 0.05), transparent 45%)'
            : theme === 'cosmic-arcade'
            ? 'radial-gradient(circle at 50% 25%, rgba(216, 71, 255, 0.12), transparent 50%), radial-gradient(circle at 20% 80%, rgba(0, 240, 255, 0.08), transparent 45%)'
            : theme === 'forest-zen'
            ? 'radial-gradient(circle at 50% 20%, rgba(82, 199, 148, 0.09), transparent 50%), radial-gradient(circle at 85% 75%, rgba(130, 217, 179, 0.06), transparent 40%)'
            : 'none',
      }}
    >
      {/* --------------------------------------------------------- */}
      {/* DESKTOP SHORTCUT ICONS GRID */}
      {/* --------------------------------------------------------- */}
      <div className="p-6 grid grid-flow-col grid-rows-6 gap-y-4 gap-x-2 w-max">
        {activeActivity.desktopShortcuts.map((sc) => {
          const isSelected = selectedShortcut === sc.id;
          return (
            <div
              key={sc.id}
              onClick={(e) => {
                e.stopPropagation();
                setSelectedShortcut(sc.id);
              }}
              onDoubleClick={(e) => {
                e.stopPropagation();
                handleShortcutLaunch(sc.appId, sc.id);
              }}
              className={`w-24 p-2 rounded-xl flex flex-col items-center justify-center text-center cursor-pointer transition-all duration-150 ${
                isSelected
                  ? 'bg-[var(--surface-elevated)] border border-[var(--accent-primary)] shadow-lg'
                  : 'hover:bg-[var(--surface-secondary)]/60 border border-transparent'
              }`}
            >
              <div
                className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-md border"
                style={{
                  backgroundColor: 'var(--surface-primary)',
                  borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                }}
              >
                {sc.icon}
              </div>
              <span
                className="text-[11px] font-medium mt-1.5 line-clamp-2 px-1 rounded tracking-tight text-[var(--text-dominant)] leading-tight"
                style={{ textShadow: '0 1px 3px rgba(0,0,0,0.8)' }}
              >
                {sc.title}
              </span>
            </div>
          );
        })}
      </div>

      {/* --------------------------------------------------------- */}
      {/* DESKTOP RIGHT-CLICK CONTEXT MENU */}
      {/* --------------------------------------------------------- */}
      {contextMenu.isOpen && (
        <div
          role="menu"
          aria-label="Desktop Context Menu"
          onClick={(e) => e.stopPropagation()}
          className="fixed z-50 w-52 p-1.5 rounded-xl border shadow-2xl backdrop-blur-xl animate-fadeIn select-none text-xs"
          style={{
            top: contextMenu.y,
            left: contextMenu.x,
            backgroundColor: 'var(--glass-bg)',
            borderColor: 'var(--accent-primary)',
            boxShadow: '0 16px 48px rgba(0, 0, 0, 0.7), 0 0 16px var(--border-glow)',
          }}
        >
          <button
            type="button"
            onClick={() => {
              openWindow('terminal', {}, currentActivityId);
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>💻</span>
            <span>Open MindSet Terminal</span>
          </button>

          <button
            type="button"
            onClick={() => {
              openWindow('reader', {}, currentActivityId);
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>📖</span>
            <span>Open Study Reader</span>
          </button>

          <button
            type="button"
            onClick={() => {
              openWindow('tasks', {}, currentActivityId);
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🎯</span>
            <span>Tasks & Milestones</span>
          </button>

          <div className="h-px my-1 bg-[var(--border-subtle)]" />

          <button
            type="button"
            onClick={() => {
              toggleSwitcher(true);
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>🌌</span>
            <span>Switch Activity (Super + Tab)</span>
          </button>

          <button
            type="button"
            onClick={() => {
              onOpenStatus();
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-[var(--text-dominant)] hover:bg-[var(--surface-elevated)] transition-colors"
          >
            <span>⚙️</span>
            <span>Kernel Telemetry (/status)</span>
          </button>

          <div className="h-px my-1 bg-[var(--border-subtle)]" />

          <button
            type="button"
            onClick={() => {
              onLock();
              closeContextMenu();
            }}
            className="w-full text-left px-3 py-1.5 rounded-lg flex items-center space-x-2 text-red-400 hover:bg-red-500/15 transition-colors"
          >
            <span>🔒</span>
            <span>Lock Screen (Super + L)</span>
          </button>
        </div>
      )}
    </div>
  );
};
