/**
 * MindSet OS Interactive Desktop Icon Grid Component
 * Snap-to-grid (88x96px) shortcuts with drag-and-drop coordinate persistence and selection
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useRef, useEffect, useCallback } from 'react';
import type { DesktopShortcut } from '@mindset/db-schema';
import { usePersonalizationStore } from '../../stores/usePersonalizationStore';
import { useWindowManagerStore } from '../../shell/wm/useWindowManagerStore';
import { useActivityStore } from '../../shell/activities/useActivityStore';

const CELL_WIDTH = 88;
const CELL_HEIGHT = 96;
const CELL_GAP_X = 16;
const CELL_GAP_Y = 16;
const PADDING_TOP = 20;
const PADDING_LEFT = 20;

export interface DesktopIconGridProps {
  onContextMenuShortcut?: (shortcut: DesktopShortcut, x: number, y: number) => void;
}

export const DesktopIconGrid: React.FC<DesktopIconGridProps> = ({ onContextMenuShortcut }) => {
  const {
    shortcuts,
    selectedShortcutId,
    selectShortcut,
    moveShortcut,
    deleteShortcut,
    loadShortcuts,
  } = usePersonalizationStore();

  const { openWindow } = useWindowManagerStore();
  const { currentActivityId } = useActivityStore();

  const gridRef = useRef<HTMLDivElement>(null);
  const [draggedId, setDraggedId] = useState<string | null>(null);
  const [dragOverCell, setDragOverCell] = useState<{ x: number; y: number } | null>(null);

  // Marquee selection state
  const [isMarqueeActive, setIsMarqueeActive] = useState(false);
  const [marqueeStart, setMarqueeStart] = useState<{ x: number; y: number } | null>(null);
  const [marqueeCurrent, setMarqueeCurrent] = useState<{ x: number; y: number } | null>(null);

  useEffect(() => {
    loadShortcuts();
  }, [loadShortcuts]);

  // Launch shortcut action
  const handleLaunch = useCallback(
    (sc: DesktopShortcut) => {
      selectShortcut(sc.id);
      if (sc.target_type === 'app') {
        openWindow(sc.target_payload, { title: sc.title, icon: sc.icon }, currentActivityId);
      } else if (sc.target_type === 'url') {
        window.open(sc.target_payload, '_blank');
      } else {
        openWindow('files', { title: sc.title, icon: sc.icon }, currentActivityId);
      }
    },
    [currentActivityId, openWindow, selectShortcut]
  );

  // Drag and drop handlers
  const handleDragStart = (e: React.DragEvent, id: string) => {
    e.dataTransfer.setData('text/plain', id);
    e.dataTransfer.effectAllowed = 'move';
    setDraggedId(id);
    selectShortcut(id);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';

    if (!gridRef.current) return;
    const rect = gridRef.current.getBoundingClientRect();
    const relX = Math.max(0, e.clientX - rect.left - PADDING_LEFT);
    const relY = Math.max(0, e.clientY - rect.top - PADDING_TOP);

    const cellX = Math.floor(relX / (CELL_WIDTH + CELL_GAP_X));
    const cellY = Math.floor(relY / (CELL_HEIGHT + CELL_GAP_Y));

    if (!dragOverCell || dragOverCell.x !== cellX || dragOverCell.y !== cellY) {
      setDragOverCell({ x: Math.max(0, cellX), y: Math.max(0, cellY) });
    }
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    const shortcutId = e.dataTransfer.getData('text/plain') || draggedId;
    if (shortcutId && dragOverCell) {
      // Snap to target cell
      await moveShortcut(shortcutId, dragOverCell.x, dragOverCell.y);
    }
    setDraggedId(null);
    setDragOverCell(null);
  };

  const handleDragEnd = () => {
    setDraggedId(null);
    setDragOverCell(null);
  };

  // Keyboard navigation & deletion
  const handleKeyDown = (e: React.KeyboardEvent, sc: DesktopShortcut) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleLaunch(sc);
    } else if (e.key === 'Delete' || e.key === 'Backspace') {
      e.preventDefault();
      deleteShortcut(sc.id);
    }
  };

  // Marquee selection handlers on grid container
  const handleMouseDown = (e: React.MouseEvent) => {
    if (e.target === gridRef.current) {
      selectShortcut(null);
      setIsMarqueeActive(true);
      setMarqueeStart({ x: e.clientX, y: e.clientY });
      setMarqueeCurrent({ x: e.clientX, y: e.clientY });
    }
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isMarqueeActive) {
      setMarqueeCurrent({ x: e.clientX, y: e.clientY });
    }
  };

  const handleMouseUp = () => {
    if (isMarqueeActive) {
      setIsMarqueeActive(false);
      setMarqueeStart(null);
      setMarqueeCurrent(null);
    }
  };

  // Compute marquee rectangle
  const marqueeBox =
    isMarqueeActive && marqueeStart && marqueeCurrent
      ? {
          left: Math.min(marqueeStart.x, marqueeCurrent.x),
          top: Math.min(marqueeStart.y, marqueeCurrent.y),
          width: Math.abs(marqueeCurrent.x - marqueeStart.x),
          height: Math.abs(marqueeCurrent.y - marqueeStart.y),
        }
      : null;

  return (
    <div
      ref={gridRef}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      className="absolute inset-0 select-none overflow-hidden"
      style={{ padding: `${PADDING_TOP}px 0 0 ${PADDING_LEFT}px` }}
    >
      {/* Drop Target Indicator Ghost Cell */}
      {dragOverCell && draggedId && (
        <div
          className="absolute rounded-2xl border-2 border-dashed border-[var(--accent-primary)]/70 bg-[var(--accent-primary)]/10 pointer-events-none transition-all duration-100 ease-out z-10"
          style={{
            left: PADDING_LEFT + dragOverCell.x * (CELL_WIDTH + CELL_GAP_X),
            top: PADDING_TOP + dragOverCell.y * (CELL_HEIGHT + CELL_GAP_Y),
            width: CELL_WIDTH,
            height: CELL_HEIGHT,
          }}
        />
      )}

      {/* Marquee Selection Box */}
      {marqueeBox && (
        <div
          className="fixed border border-[var(--accent-primary)] bg-[var(--accent-primary)]/15 pointer-events-none z-30 rounded backdrop-blur-[1px]"
          style={{
            left: marqueeBox.left,
            top: marqueeBox.top,
            width: marqueeBox.width,
            height: marqueeBox.height,
          }}
        />
      )}

      {/* Render Snap-to-Grid Shortcuts */}
      {shortcuts.map((sc) => {
        const isSelected = selectedShortcutId === sc.id;
        const isBeingDragged = draggedId === sc.id;
        const left = PADDING_LEFT + sc.grid_x * (CELL_WIDTH + CELL_GAP_X);
        const top = PADDING_TOP + sc.grid_y * (CELL_HEIGHT + CELL_GAP_Y);

        return (
          <div
            key={sc.id}
            tabIndex={0}
            draggable
            onDragStart={(e) => handleDragStart(e, sc.id)}
            onDragEnd={handleDragEnd}
            onClick={(e) => {
              e.stopPropagation();
              selectShortcut(sc.id);
            }}
            onDoubleClick={(e) => {
              e.stopPropagation();
              handleLaunch(sc);
            }}
            onContextMenu={(e) => {
              e.preventDefault();
              e.stopPropagation();
              selectShortcut(sc.id);
              if (onContextMenuShortcut) {
                onContextMenuShortcut(sc, e.clientX, e.clientY);
              }
            }}
            onKeyDown={(e) => handleKeyDown(e, sc)}
            className={`absolute flex flex-col items-center justify-center p-2 rounded-2xl cursor-pointer outline-none transition-transform duration-100 ease-out group ${
              isBeingDragged ? 'opacity-30 scale-95' : 'opacity-100'
            } ${
              isSelected
                ? 'bg-[var(--surface-elevated)]/80 ring-2 ring-[var(--accent-primary)] shadow-2xl backdrop-blur-md'
                : 'hover:bg-[var(--surface-secondary)]/40'
            }`}
            style={{
              left,
              top,
              width: CELL_WIDTH,
              height: CELL_HEIGHT,
            }}
          >
            {/* Shortcut Icon Badge */}
            <div
              className={`w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-md border transition-all duration-150 group-hover:scale-105 ${
                isSelected
                  ? 'border-[var(--accent-primary)] bg-[var(--surface-elevated)]'
                  : 'border-[var(--border-subtle)] bg-[var(--surface-primary)]'
              }`}
            >
              {sc.icon}
            </div>

            {/* Shortcut Label with Contrast Text-Shadow */}
            <span
              className="text-[11px] font-medium mt-1.5 line-clamp-2 px-1 text-center tracking-tight text-[var(--text-dominant)] leading-tight select-none"
              style={{
                textShadow: '0 1px 3px rgba(0, 0, 0, 0.9), 0 0 8px rgba(0,0,0,0.8)',
              }}
            >
              {sc.title}
            </span>
          </div>
        );
      })}
    </div>
  );
};

