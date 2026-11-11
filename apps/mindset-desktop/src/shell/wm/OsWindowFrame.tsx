/**
 * MindSet OS Window Frame Component
 * 60 FPS draggable, resizable window chrome with snap layouts, titlebar, and focus management
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useRef, useCallback, useEffect } from 'react';
import {
  useWindowManagerStore,
  type OsWindow,
  type SnapZone,
  getWorkAreaBounds,
} from './useWindowManagerStore';
import { SnapLayoutPicker } from './SnapLayoutPicker';

export interface OsWindowFrameProps {
  window: OsWindow;
  children: React.ReactNode;
}

export const OsWindowFrame: React.FC<OsWindowFrameProps> = ({ window: win, children }) => {
  const {
    activeWindowId,
    bringToFront,
    closeWindow,
    minimizeWindow,
    toggleMaximize,
    updateWindowBounds,
    snapWindow,
    setHoveredSnapZone,
    setDraggingWindowId,
    setResizingWindowId,
  } = useWindowManagerStore();

  const [isPickerOpen, setIsPickerOpen] = useState(false);
  const isFocused = activeWindowId === win.id;

  const frameRef = useRef<HTMLDivElement>(null);

  // Dragging state
  const dragStartRef = useRef<{ mouseX: number; mouseY: number; startX: number; startY: number } | null>(null);

  // Resizing state
  const resizeStartRef = useRef<{
    direction: string;
    mouseX: number;
    mouseY: number;
    startX: number;
    startY: number;
    startW: number;
    startH: number;
  } | null>(null);

  // -------------------------------------------------------------
  // 60 FPS DRAG HANDLER
  // -------------------------------------------------------------
  const handleTitlePointerDown = useCallback(
    (e: React.PointerEvent) => {
      // Don't drag if clicking buttons
      if ((e.target as HTMLElement).closest('button')) return;

      e.preventDefault();
      bringToFront(win.id);

      if (win.isMaximized || win.snapZone) {
        // If snapped or maximized, dragging detaches and centers window
        const normalW = win.prevBounds?.width || 800;
        const normalH = win.prevBounds?.height || 540;
        const newX = e.clientX - normalW / 2;
        const newY = e.clientY - 16;
        updateWindowBounds(win.id, { x: newX, y: newY, width: normalW, height: normalH });
        dragStartRef.current = {
          mouseX: e.clientX,
          mouseY: e.clientY,
          startX: newX,
          startY: newY,
        };
      } else {
        dragStartRef.current = {
          mouseX: e.clientX,
          mouseY: e.clientY,
          startX: win.bounds.x,
          startY: win.bounds.y,
        };
      }

      setDraggingWindowId(win.id);
      (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    },
    [bringToFront, setDraggingWindowId, updateWindowBounds, win]
  );

  const handleTitlePointerMove = useCallback(
    (e: React.PointerEvent) => {
      if (!dragStartRef.current) return;

      const deltaX = e.clientX - dragStartRef.current.mouseX;
      const deltaY = e.clientY - dragStartRef.current.mouseY;

      const newX = dragStartRef.current.startX + deltaX;
      const newY = dragStartRef.current.startY + deltaY;

      updateWindowBounds(win.id, { x: newX, y: newY });

      // Edge Snap Detection
      const work = getWorkAreaBounds();
      const margin = 24;

      if (e.clientX <= margin) {
        if (e.clientY < work.y + work.height / 3) {
          setHoveredSnapZone('quad-top-left');
        } else if (e.clientY > work.y + (work.height * 2) / 3) {
          setHoveredSnapZone('quad-bottom-left');
        } else {
          setHoveredSnapZone('left-50');
        }
      } else if (e.clientX >= work.width - margin) {
        if (e.clientY < work.y + work.height / 3) {
          setHoveredSnapZone('3-grid-top-right');
        } else if (e.clientY > work.y + (work.height * 2) / 3) {
          setHoveredSnapZone('3-grid-bottom-right');
        } else {
          setHoveredSnapZone('right-50');
        }
      } else if (e.clientY <= work.y + 12) {
        setHoveredSnapZone('maximize');
      } else {
        setHoveredSnapZone(null);
      }
    },
    [setHoveredSnapZone, updateWindowBounds, win.id]
  );

  const handleTitlePointerUp = useCallback(
    (e: React.PointerEvent) => {
      if (!dragStartRef.current) return;
      dragStartRef.current = null;
      setDraggingWindowId(null);

      try {
        (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
      } catch {
        // Safe pointer release
      }

      // If hovering a snap zone, execute snap
      const currentZone = useWindowManagerStore.getState().hoveredSnapZone;
      if (currentZone) {
        snapWindow(win.id, currentZone);
        setHoveredSnapZone(null);
      }
    },
    [setDraggingWindowId, setHoveredSnapZone, snapWindow, win.id]
  );

  // -------------------------------------------------------------
  // 60 FPS RESIZE HANDLERS
  // -------------------------------------------------------------
  const handleResizePointerDown = (direction: string, e: React.PointerEvent) => {
    e.preventDefault();
    e.stopPropagation();
    bringToFront(win.id);

    resizeStartRef.current = {
      direction,
      mouseX: e.clientX,
      mouseY: e.clientY,
      startX: win.bounds.x,
      startY: win.bounds.y,
      startW: win.bounds.width,
      startH: win.bounds.height,
    };

    setResizingWindowId(win.id);
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  };

  const handleResizePointerMove = (e: React.PointerEvent) => {
    if (!resizeStartRef.current) return;

    const { direction, mouseX, mouseY, startX, startY, startW, startH } = resizeStartRef.current;
    const deltaX = e.clientX - mouseX;
    const deltaY = e.clientY - mouseY;

    let newX = startX;
    let newY = startY;
    let newW = startW;
    let newH = startH;

    const minW = 340;
    const minH = 220;

    if (direction.includes('e')) newW = Math.max(minW, startW + deltaX);
    if (direction.includes('s')) newH = Math.max(minH, startH + deltaY);
    if (direction.includes('w')) {
      const prospectiveW = startW - deltaX;
      if (prospectiveW >= minW) {
        newW = prospectiveW;
        newX = startX + deltaX;
      }
    }
    if (direction.includes('n')) {
      const prospectiveH = startH - deltaY;
      if (prospectiveH >= minH) {
        newH = prospectiveH;
        newY = startY + deltaY;
      }
    }

    updateWindowBounds(win.id, { x: newX, y: newY, width: newW, height: newH });
  };

  const handleResizePointerUp = (e: React.PointerEvent) => {
    if (!resizeStartRef.current) return;
    resizeStartRef.current = null;
    setResizingWindowId(null);
    try {
      (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {
      // safe
    }
  };

  // Keyboard shortcut listener for Super + Arrows
  useEffect(() => {
    if (!isFocused) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.metaKey || e.ctrlKey) {
        if (e.key === 'ArrowLeft') {
          e.preventDefault();
          snapWindow(win.id, 'left-50');
        } else if (e.key === 'ArrowRight') {
          e.preventDefault();
          snapWindow(win.id, 'right-50');
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          toggleMaximize(win.id);
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          minimizeWindow(win.id);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isFocused, minimizeWindow, snapWindow, toggleMaximize, win.id]);

  if (win.isMinimized) return null;

  return (
    <div
      ref={frameRef}
      role="dialog"
      aria-label={win.title}
      onMouseDown={() => bringToFront(win.id)}
      className="fixed flex flex-col rounded-xl border shadow-2xl select-none overflow-hidden transition-shadow"
      style={{
        transform: `translate3d(${win.bounds.x}px, ${win.bounds.y}px, 0)`,
        width: win.bounds.width,
        height: win.bounds.height,
        zIndex: win.zIndex,
        backgroundColor: 'var(--surface-primary)',
        borderColor: isFocused ? 'var(--accent-primary)' : 'var(--border-subtle)',
        boxShadow: isFocused
          ? '0 16px 48px rgba(0, 0, 0, 0.7), 0 0 24px var(--border-glow)'
          : '0 8px 32px rgba(0, 0, 0, 0.45)',
      }}
    >
      {/* --------------------------------------------------------- */}
      {/* WINDOW TITLEBAR / CHROME (Draggable Header) */}
      {/* --------------------------------------------------------- */}
      <div
        onPointerDown={handleTitlePointerDown}
        onPointerMove={handleTitlePointerMove}
        onPointerUp={handleTitlePointerUp}
        onDoubleClick={() => toggleMaximize(win.id)}
        className="h-9 px-3 flex items-center justify-between border-b cursor-move shrink-0"
        style={{
          backgroundColor: isFocused ? 'var(--surface-elevated)' : 'var(--surface-secondary)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        {/* Left: Icon & Title */}
        <div className="flex items-center space-x-2 truncate min-w-0 pr-2">
          <span className="text-sm shrink-0" role="img" aria-label="App Icon">
            {win.icon}
          </span>
          <span
            className="text-xs font-semibold truncate tracking-wide"
            style={{ color: isFocused ? 'var(--text-dominant)' : 'var(--text-muted)' }}
          >
            {win.title}
          </span>
          {win.subjectTag && (
            <span
              className="text-[10px] px-1.5 py-0.5 rounded border font-mono truncate"
              style={{
                backgroundColor: 'var(--surface-primary)',
                borderColor: 'var(--border-subtle)',
                color: 'var(--accent-secondary)',
              }}
            >
              {win.subjectTag}
            </span>
          )}
        </div>

        {/* Right: Window Controls */}
        <div className="flex items-center space-x-1 shrink-0 relative">
          {/* Snap Layout Picker Trigger Button */}
          <div className="relative">
            <button
              type="button"
              onClick={() => setIsPickerOpen(!isPickerOpen)}
              onMouseEnter={() => setIsPickerOpen(true)}
              title="Snap Layouts (Super + Arrow Keys)"
              className="w-6 h-6 flex items-center justify-center rounded text-[11px] hover:bg-[var(--surface-primary)] transition-colors"
              style={{ color: 'var(--text-muted)' }}
              aria-label="Snap Layouts"
            >
              ⊞
            </button>

            {isPickerOpen && (
              <SnapLayoutPicker
                onSelectZone={(zone: SnapZone) => {
                  snapWindow(win.id, zone);
                  setIsPickerOpen(false);
                }}
                onClose={() => setIsPickerOpen(false)}
              />
            )}
          </div>

          {/* Minimize Button */}
          <button
            type="button"
            onClick={() => minimizeWindow(win.id)}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-[var(--surface-primary)] transition-colors"
            style={{ color: 'var(--text-muted)' }}
            aria-label="Minimize Window"
          >
            <span className="font-bold text-xs">—</span>
          </button>

          {/* Maximize / Restore Button */}
          <button
            type="button"
            onClick={() => toggleMaximize(win.id)}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-[var(--surface-primary)] transition-colors"
            style={{ color: 'var(--text-muted)' }}
            aria-label={win.isMaximized ? 'Restore Window' : 'Maximize Window'}
          >
            <span className="text-[10px]">{win.isMaximized ? '❐' : '▢'}</span>
          </button>

          {/* Close Button */}
          <button
            type="button"
            onClick={() => closeWindow(win.id)}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-red-500/20 hover:text-red-400 transition-colors"
            style={{ color: 'var(--text-muted)' }}
            aria-label="Close Window"
          >
            <span className="text-xs">✕</span>
          </button>
        </div>
      </div>

      {/* --------------------------------------------------------- */}
      {/* WINDOW INTERIOR CONTENT */}
      {/* --------------------------------------------------------- */}
      <div className="flex-1 overflow-auto relative bg-[var(--surface-primary)]">
        {children}
      </div>

      {/* --------------------------------------------------------- */}
      {/* 8 RESIZE HANDLERS (Only when not maximized) */}
      {/* --------------------------------------------------------- */}
      {!win.isMaximized && (
        <>
          <div
            onPointerDown={(e) => handleResizePointerDown('n', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute top-0 left-2 right-2 h-1 cursor-ns-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('s', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute bottom-0 left-2 right-2 h-1.5 cursor-ns-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('w', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute top-2 bottom-2 left-0 w-1.5 cursor-ew-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('e', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute top-2 bottom-2 right-0 w-1.5 cursor-ew-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('nw', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute top-0 left-0 w-3 h-3 cursor-nwse-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('ne', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute top-0 right-0 w-3 h-3 cursor-nesw-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('sw', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute bottom-0 left-0 w-3 h-3 cursor-nesw-resize"
          />
          <div
            onPointerDown={(e) => handleResizePointerDown('se', e)}
            onPointerMove={handleResizePointerMove}
            onPointerUp={handleResizePointerUp}
            className="absolute bottom-0 right-0 w-3 h-3 cursor-nwse-resize"
          />
        </>
      )}
    </div>
  );
};

