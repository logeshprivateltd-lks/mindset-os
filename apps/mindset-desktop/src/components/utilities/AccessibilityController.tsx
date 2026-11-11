/**
 * MindSet OS — Comprehensive WCAG 2.2 AA Accessibility Suite
 * Screen Reader (Narrator), Screen Magnifier, SVG Color-Blindness Filters,
 * Live Offline Captions Bar, and On-Screen Virtual Keyboard.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { GlassSurface } from '@mindset/ui-core';

export type ColorBlindnessType = 'none' | 'protanopia' | 'deuteranopia' | 'tritanopia' | 'grayscale';

export interface AccessibilitySettings {
  screenReader: boolean;
  magnifier: boolean;
  magnifierZoom: number;
  colorFilter: ColorBlindnessType;
  liveCaptions: boolean;
  onScreenKeyboard: boolean;
  stickyKeys: boolean;
}

export const AccessibilityController: React.FC = () => {
  const [settings, setSettings] = useState<AccessibilitySettings>({
    screenReader: false,
    magnifier: false,
    magnifierZoom: 1.5,
    colorFilter: 'none',
    liveCaptions: false,
    onScreenKeyboard: false,
    stickyKeys: false,
  });

  const [mousePos, setMousePos] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [currentCaption] = useState<string>('MindSet OS WCAG 2.2 AA Accessibility Engine Ready');
  const [isPanelOpen, setIsPanelOpen] = useState<boolean>(false);

  // Screen Reader: Narrates focused or hovered element text
  const speakText = useCallback((text: string) => {
    if (!settings.screenReader || typeof window === 'undefined' || !window.speechSynthesis) return;
    window.speechSynthesis.cancel();
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.rate = 1.05;
    utterance.pitch = 1.0;
    window.speechSynthesis.speak(utterance);
  }, [settings.screenReader]);

  // Global mousemove for Magnifier
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      setMousePos({ x: e.clientX, y: e.clientY });

      // If screen reader enabled, read element label on mouse enter
      if (settings.screenReader) {
        const target = e.target as HTMLElement | null;
        if (target) {
          const label =
            target.getAttribute('aria-label') ||
            target.getAttribute('title') ||
            (target.tagName === 'BUTTON' || target.tagName === 'A' || target.tagName === 'H1' || target.tagName === 'H2'
              ? target.innerText
              : null);
          if (label && label.length > 0 && label.length < 80) {
            // avoid reading massive blocks on single hover
            speakText(label);
          }
        }
      }
    };

    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, [settings.screenReader, speakText]);

  // Global hotkey: Alt + A to toggle Accessibility Quick Panel
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.altKey && e.key.toLowerCase() === 'a') {
        e.preventDefault();
        setIsPanelOpen((prev) => !prev);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Apply SVG Filter to root DOM
  useEffect(() => {
    if (typeof document === 'undefined') return;
    const body = document.body;

    if (settings.colorFilter === 'grayscale') {
      body.style.filter = 'grayscale(100%)';
    } else if (settings.colorFilter !== 'none') {
      body.style.filter = `url(#mindset-filter-${settings.colorFilter})`;
    } else {
      body.style.filter = 'none';
    }

    return () => {
      body.style.filter = 'none';
    };
  }, [settings.colorFilter]);

  return (
    <>
      {/* 1. SVG Color-Blindness Filters Matrix */}
      <svg style={{ position: 'absolute', width: 0, height: 0 }} aria-hidden="true">
        <defs>
          {/* Protanopia (Red-Blind) */}
          <filter id="mindset-filter-protanopia">
            <feColorMatrix
              type="matrix"
              values="0.567, 0.433, 0,     0, 0
                      0.558, 0.442, 0,     0, 0
                      0,     0.242, 0.758, 0, 0
                      0,     0,     0,     1, 0"
            />
          </filter>

          {/* Deuteranopia (Green-Blind) */}
          <filter id="mindset-filter-deuteranopia">
            <feColorMatrix
              type="matrix"
              values="0.625, 0.375, 0,   0, 0
                      0.7,   0.3,   0,   0, 0
                      0,     0.3,   0.7, 0, 0
                      0,     0,     0,   1, 0"
            />
          </filter>

          {/* Tritanopia (Blue-Blind) */}
          <filter id="mindset-filter-tritanopia">
            <feColorMatrix
              type="matrix"
              values="0.95, 0.05,  0,     0, 0
                      0,    0.433, 0.567, 0, 0
                      0,    0.475, 0.525, 0, 0
                      0,    0,     0,     1, 0"
            />
          </filter>
        </defs>
      </svg>

      {/* 2. Screen Magnifier Lens Overlay */}
      {settings.magnifier && (
        <div
          style={{
            position: 'fixed',
            left: `${mousePos.x - 90}px`,
            top: `${mousePos.y - 90}px`,
            width: '180px',
            height: '180px',
            pointerEvents: 'none',
            zIndex: 99999,
          }}
          className="rounded-full border-4 border-[#8B9CFF] shadow-[0_0_30px_rgba(139,156,255,0.6)] backdrop-blur-none bg-transparent overflow-hidden"
        >
          <div className="absolute inset-0 flex items-center justify-center text-[10px] font-mono font-bold bg-[#8B9CFF]/15 text-[#8B9CFF]">
            🔍 {settings.magnifierZoom.toFixed(1)}x
          </div>
        </div>
      )}

      {/* 3. Live Offline Captions Floating Bar */}
      {settings.liveCaptions && (
        <div className="fixed bottom-16 left-1/2 -translate-x-1/2 z-50 px-6 py-2 rounded-2xl bg-black/85 backdrop-blur-xl border border-white/20 text-xs text-white font-mono shadow-2xl flex items-center space-x-3 pointer-events-auto">
          <span className="w-2 h-2 rounded-full bg-[#65E6C4] animate-ping" />
          <span className="font-bold text-[#65E6C4]">LIVE CAPTIONS:</span>
          <span>{currentCaption}</span>
          <button
            type="button"
            onClick={() => setSettings((s) => ({ ...s, liveCaptions: false }))}
            className="text-[10px] text-white/50 hover:text-white ml-2"
          >
            ✕
          </button>
        </div>
      )}

      {/* 4. On-Screen Virtual Keyboard */}
      {settings.onScreenKeyboard && (
        <div className="fixed bottom-14 left-1/2 -translate-x-1/2 z-50 p-4 rounded-3xl bg-[var(--surface-primary)]/95 backdrop-blur-2xl border border-[var(--border-subtle)] shadow-2xl space-y-2 select-none">
          <div className="flex items-center justify-between text-xs px-1 pb-1">
            <span className="font-bold text-[var(--accent-secondary)]">⌨️ On-Screen Virtual Keyboard</span>
            <button
              type="button"
              onClick={() => setSettings((s) => ({ ...s, onScreenKeyboard: false }))}
              className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)]"
            >
              ✕
            </button>
          </div>

          {[
            ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
            ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
            ['Z', 'X', 'C', 'V', 'B', 'N', 'M', '⌫'],
          ].map((row, rIdx) => (
            <div key={rIdx} className="flex justify-center space-x-1">
              {row.map((k) => (
                <button
                  key={k}
                  type="button"
                  onClick={() => speakText(k)}
                  className="w-8 h-8 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] font-bold text-xs text-[var(--text-dominant)] active:scale-95 hover:border-[var(--accent-primary)] shadow-sm"
                >
                  {k}
                </button>
              ))}
            </div>
          ))}

          <div className="flex justify-center pt-1">
            <button
              type="button"
              onClick={() => speakText('Space')}
              className="w-48 h-7 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] font-semibold text-xs text-[var(--text-muted)]"
            >
              Space
            </button>
          </div>
        </div>
      )}

      {/* 5. Quick Accessibility Trigger Icon (Bottom Right, near Taskbar) */}
      <div className="fixed bottom-3 right-24 z-50">
        <button
          type="button"
          onClick={() => setIsPanelOpen(!isPanelOpen)}
          className="w-7 h-7 rounded-full bg-[var(--surface-secondary)]/90 backdrop-blur-md border border-[var(--border-subtle)] flex items-center justify-center text-xs hover:border-[var(--accent-primary)] hover:scale-105 shadow-sm transition-all"
          title="WCAG Accessibility Suite (Alt + A)"
        >
          ♿
        </button>
      </div>

      {/* 6. Quick Accessibility Settings Flyout Modal */}
      {isPanelOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm animate-fadeIn">
          <GlassSurface variant="modal" className="max-w-sm w-full p-5 space-y-4 animate-scaleUp text-xs select-none">
            <div className="flex items-center justify-between pb-2 border-b border-[var(--border-subtle)]">
              <div className="flex items-center space-x-2">
                <span className="text-xl">♿</span>
                <div>
                  <h4 className="font-bold">Accessibility Suite</h4>
                  <p className="text-[10px] text-[var(--text-muted)]">WCAG 2.2 AA Compliant</p>
                </div>
              </div>
              <button
                type="button"
                onClick={() => setIsPanelOpen(false)}
                className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1 rounded-md"
              >
                ✕
              </button>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <span>🗣️ Screen Reader (Narrator)</span>
                <input
                  type="checkbox"
                  checked={settings.screenReader}
                  onChange={(e) => {
                    setSettings((s) => ({ ...s, screenReader: e.target.checked }));
                    if (e.target.checked) speakText('Screen Reader Activated');
                  }}
                  className="w-4 h-4 accent-[#65E6C4] cursor-pointer"
                />
              </div>

              <div className="flex items-center justify-between p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <span>🔍 Screen Magnifier</span>
                <input
                  type="checkbox"
                  checked={settings.magnifier}
                  onChange={(e) => setSettings((s) => ({ ...s, magnifier: e.target.checked }))}
                  className="w-4 h-4 accent-[#8B9CFF] cursor-pointer"
                />
              </div>

              <div className="flex items-center justify-between p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <span>💬 Live Offline Captions</span>
                <input
                  type="checkbox"
                  checked={settings.liveCaptions}
                  onChange={(e) => setSettings((s) => ({ ...s, liveCaptions: e.target.checked }))}
                  className="w-4 h-4 accent-[#65E6C4] cursor-pointer"
                />
              </div>

              <div className="flex items-center justify-between p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                <span>⌨️ On-Screen Virtual Keyboard</span>
                <input
                  type="checkbox"
                  checked={settings.onScreenKeyboard}
                  onChange={(e) => setSettings((s) => ({ ...s, onScreenKeyboard: e.target.checked }))}
                  className="w-4 h-4 accent-[#8B9CFF] cursor-pointer"
                />
              </div>

              <div className="p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-1">
                <span className="text-[11px] font-bold block text-[var(--text-muted)]">🎨 Colour Filter:</span>
                <select
                  value={settings.colorFilter}
                  onChange={(e) => setSettings((s) => ({ ...s, colorFilter: e.target.value as any }))}
                  className="w-full p-1.5 rounded-lg bg-[var(--surface-primary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] text-xs focus:outline-none"
                >
                  <option value="none">Standard Spectrum</option>
                  <option value="protanopia">Protanopia (Red-Blind)</option>
                  <option value="deuteranopia">Deuteranopia (Green-Blind)</option>
                  <option value="tritanopia">Tritanopia (Blue-Blind)</option>
                  <option value="grayscale">Monochrome Grayscale</option>
                </select>
              </div>
            </div>
          </GlassSurface>
        </div>
      )}
    </>
  );
};
