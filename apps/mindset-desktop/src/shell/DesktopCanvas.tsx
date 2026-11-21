/**
 * MindSet OS Desktop Canvas Component
 * Renders calibrated wallpaper, live desktop widgets, interactive snap-to-grid icon canvas, and context menu
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import type { DesktopShortcut } from '@mindset/db-schema';
import {
  usePersonalizationStore,
  WALLPAPER_PRESETS,
} from '../stores/usePersonalizationStore';
import { DesktopIconGrid } from '../components/shell/DesktopIconGrid';
import { DesktopContextMenu } from '../components/shell/DesktopContextMenu';

export interface DesktopCanvasProps {
  onOpenStatus: () => void;
  onLock: () => void;
}

interface ContextMenuState {
  isOpen: boolean;
  x: number;
  y: number;
  targetShortcut?: DesktopShortcut | null;
}

export const DesktopCanvas: React.FC<DesktopCanvasProps> = ({ onOpenStatus, onLock }) => {
  const {
    wallpaperType,
    wallpaperPath,
    wallpaperDimness,
    wallpaperBlurPx,
    widgets,
    initialize,
  } = usePersonalizationStore();

  const [contextMenu, setContextMenu] = useState<ContextMenuState>({
    isOpen: false,
    x: 0,
    y: 0,
    targetShortcut: null,
  });

  const [currentTime, setCurrentTime] = useState(new Date());

  useEffect(() => {
    initialize();
  }, [initialize]);

  // Clock tick every second for Hero Clock Widget
  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  // Wallpaper style resolution
  const activePreset = WALLPAPER_PRESETS.find((p) => p.id === wallpaperPath) || WALLPAPER_PRESETS[0];

  const wallpaperBackground =
    wallpaperType === 'custom'
      ? `url("${wallpaperPath}")`
      : activePreset ? activePreset.cssGradient : WALLPAPER_PRESETS[0]!.cssGradient;

  // Handle right-click on Wallpaper Canvas
  const handleContextMenuCanvas = (e: React.MouseEvent) => {
    e.preventDefault();
    setContextMenu({
      isOpen: true,
      x: e.clientX,
      y: e.clientY,
      targetShortcut: null,
    });
  };

  // Handle right-click on Shortcut
  const handleContextMenuShortcut = (sc: DesktopShortcut, x: number, y: number) => {
    setContextMenu({
      isOpen: true,
      x,
      y,
      targetShortcut: sc,
    });
  };

  const closeContextMenu = () => {
    if (contextMenu.isOpen) {
      setContextMenu((prev) => ({ ...prev, isOpen: false }));
    }
  };

  return (
    <div
      onContextMenu={handleContextMenuCanvas}
      onClick={closeContextMenu}
      className="absolute inset-0 select-none overflow-hidden transition-all duration-300 ease-out"
      style={{
        background: wallpaperBackground,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
      }}
    >
      {/* 1. Wallpaper Dimness & Blur Shroud */}
      <div
        className="absolute inset-0 pointer-events-none transition-all duration-200"
        style={{
          backgroundColor: `rgba(0, 0, 0, ${wallpaperDimness})`,
          backdropFilter: wallpaperBlurPx > 0 ? `blur(${wallpaperBlurPx}px)` : 'none',
          WebkitBackdropFilter: wallpaperBlurPx > 0 ? `blur(${wallpaperBlurPx}px)` : 'none',
        }}
      />

      {/* 2. Desktop Interactive Widgets (Top Right) */}
      <div className="absolute top-6 right-8 flex flex-col items-end space-y-3 z-10 pointer-events-auto">
        {/* Widget A: Hero Clock & Date */}
        {widgets.clock && (
          <div className="px-5 py-3 rounded-2xl border border-white/10 bg-black/40 backdrop-blur-xl shadow-2xl flex flex-col items-end text-right transition-transform hover:scale-[1.02]">
            <div className="text-3xl font-extrabold tracking-tight text-white font-mono leading-none drop-shadow-md">
              {currentTime.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })}
            </div>
            <div className="text-[11px] font-medium text-slate-300 tracking-wide mt-1.5 flex items-center space-x-2">
              <span>{currentTime.toLocaleDateString([], { weekday: 'short', month: 'short', day: 'numeric', year: 'numeric' })}</span>
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
            </div>
          </div>
        )}

        {/* Widget B: Class 12 Exam Target Card */}
        {widgets.target && (
          <div className="px-4 py-2.5 rounded-2xl border border-[var(--accent-primary)]/30 bg-black/40 backdrop-blur-xl shadow-2xl max-w-xs text-right space-y-1 transition-transform hover:scale-[1.02]">
            <div className="flex items-center justify-end space-x-2">
              <span className="text-xs">🎯</span>
              <span className="text-[11px] font-bold text-[var(--accent-primary)] uppercase tracking-wider">
                Exam Target: 590 / 600
              </span>
            </div>
            <div className="text-[10px] text-slate-300 font-mono">
              Class 12 Board & JEE Mastery • 98.3% Target
            </div>
            {/* Progress bar */}
            <div className="w-48 h-1.5 rounded-full bg-slate-800 overflow-hidden ml-auto mt-1">
              <div className="h-full bg-gradient-to-r from-[var(--accent-primary)] to-[#65E6C4] rounded-full" style={{ width: '84%' }} />
            </div>
          </div>
        )}

        {/* Widget C: Daily Student Affirmation */}
        {widgets.affirmation && (
          <div className="px-3.5 py-2 rounded-xl border border-white/10 bg-black/35 backdrop-blur-lg shadow-lg text-right max-w-xs transition-opacity hover:opacity-100 opacity-90">
            <p className="text-[11px] italic text-slate-200 leading-snug">
              &ldquo;Calm, sustained focus outperforms frantic effort.&rdquo;
            </p>
            <span className="text-[9px] text-[#FFD078] font-mono block mt-0.5">MindSet Stoic Flow</span>
          </div>
        )}
      </div>

      {/* 3. Interactive Snap-to-Grid Desktop Icon Grid */}
      <DesktopIconGrid onContextMenuShortcut={handleContextMenuShortcut} />

      {/* 4. Desktop Context Menu */}
      {contextMenu.isOpen && (
        <DesktopContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          targetShortcut={contextMenu.targetShortcut}
          onClose={closeContextMenu}
          onOpenStatus={onOpenStatus}
          onLock={onLock}
        />
      )}
    </div>
  );
};
