/**
 * MindSet OS — Always-On-Top Floating AI Orb (Alt + Space)
 * Draggable Liquid Glass Orb with Focus Mode Dimming and Global Snip Solver (Alt + S)
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useRef, useCallback } from 'react';
import { useKernelStore } from '../../stores/useKernelStore';
import { FloatingAiConsole } from './FloatingAiConsole';
import { ScreenSnipOverlay, ScreenSnipResult } from './ScreenSnipOverlay';

export const FloatingAiOrb: React.FC = () => {
  const { isFocusActive } = useKernelStore();

  const [isConsoleOpen, setIsConsoleOpen] = useState<boolean>(false);
  const [isSnipOpen, setIsSnipOpen] = useState<boolean>(false);
  const [activeSnip, setActiveSnip] = useState<ScreenSnipResult | null>(null);

  // Floating coordinates on screen
  const [position, setPosition] = useState<{ x: number; y: number }>(() => ({
    x: typeof window !== 'undefined' ? window.innerWidth - 80 : 1200,
    y: typeof window !== 'undefined' ? window.innerHeight - 150 : 700,
  }));

  const isDraggingRef = useRef<boolean>(false);
  const dragStartRef = useRef<{ startX: number; startY: number; initialX: number; initialY: number }>({
    startX: 0,
    startY: 0,
    initialX: 0,
    initialY: 0,
  });
  const hasMovedRef = useRef<boolean>(false);

  // Keep orb within viewport on window resize
  useEffect(() => {
    const handleResize = () => {
      setPosition((prev) => ({
        x: Math.max(16, Math.min(window.innerWidth - 72, prev.x)),
        y: Math.max(16, Math.min(window.innerHeight - 72, prev.y)),
      }));
    };
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  // Global Keyboard Shortcuts:
  // Alt + Space -> Toggle AI Floating Console
  // Alt + S     -> Trigger Screen Snip / Doubt Solver
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Check for Alt + Space
      if (e.altKey && e.code === 'Space') {
        e.preventDefault();
        setIsConsoleOpen((prev) => !prev);
      }
      // Check for Alt + S (or Alt + s)
      else if (e.altKey && (e.key === 's' || e.key === 'S')) {
        e.preventDefault();
        setIsSnipOpen(true);
        setIsConsoleOpen(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Drag interaction handlers
  const handlePointerDown = (e: React.PointerEvent) => {
    if (e.button !== 0) return; // Only primary mouse button
    isDraggingRef.current = true;
    hasMovedRef.current = false;
    dragStartRef.current = {
      startX: e.clientX,
      startY: e.clientY,
      initialX: position.x,
      initialY: position.y,
    };

    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!isDraggingRef.current) return;

    const dx = e.clientX - dragStartRef.current.startX;
    const dy = e.clientY - dragStartRef.current.startY;

    if (Math.hypot(dx, dy) > 4) {
      hasMovedRef.current = true;
    }

    const nextX = Math.max(16, Math.min(window.innerWidth - 72, dragStartRef.current.initialX + dx));
    const nextY = Math.max(16, Math.min(window.innerHeight - 72, dragStartRef.current.initialY + dy));

    setPosition({ x: nextX, y: nextY });
  };

  const handlePointerUp = (e: React.PointerEvent) => {
    if (!isDraggingRef.current) return;
    isDraggingRef.current = false;
    try {
      (e.target as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {
      // Ignore if pointer capture was lost
    }

    // If it was a quick click without significant drag movement, toggle the console
    if (!hasMovedRef.current) {
      setIsConsoleOpen((prev) => !prev);
    }
  };

  const handleSnipComplete = useCallback((result: ScreenSnipResult) => {
    setActiveSnip(result);
    setIsSnipOpen(false);
    setIsConsoleOpen(true);
  }, []);

  return (
    <>
      {/* 1. Draggable Floating Liquid Glass Orb */}
      <div
        style={{
          position: 'fixed',
          left: `${position.x}px`,
          top: `${position.y}px`,
          zIndex: 9990,
          touchAction: 'none',
        }}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        className={`group cursor-grab active:cursor-grabbing select-none transition-opacity duration-300 ${
          isFocusActive ? 'opacity-20 hover:opacity-100' : 'opacity-90 hover:opacity-100'
        }`}
        title="MindSet AI Orb (Alt + Space) • Screen Solver (Alt + S)"
      >
        {/* Ambient Ring Glow */}
        <div className="relative flex items-center justify-center w-14 h-14 rounded-full bg-[#111827]/80 backdrop-blur-xl border border-[#8B9CFF]/40 shadow-[0_0_24px_rgba(139,156,255,0.35)] transition-all duration-300 group-hover:scale-105 group-hover:border-[#65E6C4]/60 group-hover:shadow-[0_0_32px_rgba(101,230,196,0.45)]">
          {/* Inner pulsating gradient aura */}
          <div className="absolute inset-1 rounded-full bg-gradient-to-tr from-[#8B9CFF]/20 via-[#65E6C4]/15 to-[#FFD078]/20 animate-pulse" />

          {/* Central AI Logo / Sparkle glyph */}
          <div className="relative flex items-center justify-center w-10 h-10 rounded-full bg-[#182235]/90 border border-[#8B9CFF]/30">
            <svg
              className="w-5 h-5 text-[#8B9CFF] group-hover:text-[#65E6C4] transition-colors"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
            </svg>
          </div>

          {/* Subtle Online Dot */}
          <span className="absolute bottom-0.5 right-0.5 flex h-3 w-3">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#65E6C4] opacity-75" />
            <span className="relative inline-flex rounded-full h-3 w-3 bg-[#65E6C4] border-2 border-[#111827]" />
          </span>
        </div>

        {/* Hover Shortcut Pill */}
        <div className="absolute -top-7 left-1/2 -translate-x-1/2 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap px-2 py-0.5 rounded text-[10px] font-mono bg-[#07090D]/90 text-[#8B9CFF] border border-[#8B9CFF]/30 shadow-lg">
          Alt+Space
        </div>
      </div>

      {/* 2. Floating AI Mini-Console (Alt + Space) */}
      <FloatingAiConsole
        isOpen={isConsoleOpen}
        onClose={() => setIsConsoleOpen(false)}
        onTriggerSnip={() => {
          setIsConsoleOpen(false);
          setIsSnipOpen(true);
        }}
        activeSnipResult={activeSnip}
        onClearSnip={() => setActiveSnip(null)}
      />

      {/* 3. Privacy-First Screen Snip Overlay (Alt + S) */}
      <ScreenSnipOverlay
        isOpen={isSnipOpen}
        onClose={() => setIsSnipOpen(false)}
        onSnipComplete={handleSnipComplete}
      />
    </>
  );
};

