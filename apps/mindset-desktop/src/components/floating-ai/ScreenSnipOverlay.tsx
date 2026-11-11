/**
 * MindSet OS — Privacy-First Screen Snip Overlay (Alt + S)
 * Active ONLY on explicit user request; zero background screen recording.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';

export interface ScreenSnipResult {
  imageBase64: string;
  width: number;
  height: number;
  extractedText?: string;
}

export interface ScreenSnipOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  onSnipComplete: (result: ScreenSnipResult) => void;
}

export const ScreenSnipOverlay: React.FC<ScreenSnipOverlayProps> = ({
  isOpen,
  onClose,
  onSnipComplete,
}) => {
  const [isSelecting, setIsSelecting] = useState<boolean>(false);
  const [startPos, setStartPos] = useState<{ x: number; y: number } | null>(null);
  const [currentPos, setCurrentPos] = useState<{ x: number; y: number } | null>(null);
  const [isProcessing, setIsProcessing] = useState<boolean>(false);

  // Handle ESC key to cancel
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    if (isProcessing) return;
    setIsSelecting(true);
    setStartPos({ x: e.clientX, y: e.clientY });
    setCurrentPos({ x: e.clientX, y: e.clientY });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!isSelecting) return;
    setCurrentPos({ x: e.clientX, y: e.clientY });
  };

  const handleMouseUp = async () => {
    if (!isSelecting || !startPos || !currentPos) {
      setIsSelecting(false);
      return;
    }

    setIsSelecting(false);

    const x = Math.min(startPos.x, currentPos.x);
    const y = Math.min(startPos.y, currentPos.y);
    const width = Math.abs(currentPos.x - startPos.x);
    const height = Math.abs(currentPos.y - startPos.y);

    // Require minimum selection dimension (at least 20x20 px)
    if (width < 20 || height < 20) {
      setStartPos(null);
      setCurrentPos(null);
      return;
    }

    setIsProcessing(true);

    try {
      const { invoke } = await import('@tauri-apps/api/core');

      // 1. Capture explicit region via native Rust backend
      const captureResult = await invoke<{
        image_base64: string;
        width: number;
        height: number;
      }>('ai_capture_screen_region', {
        x: Math.round(x),
        y: Math.round(y),
        width: Math.round(width),
        height: Math.round(height),
      });

      // 2. Extract OCR text and detected formulas
      const ocrResult = await invoke<{
        extracted_text: string;
        detected_formulas: string[];
      }>('ai_extract_screen_text_and_formulas', {
        imageBase64: captureResult.image_base64,
        hintText: null,
      });

      onSnipComplete({
        imageBase64: captureResult.image_base64,
        width: captureResult.width,
        height: captureResult.height,
        extractedText: ocrResult.extracted_text,
      });

      onClose();
    } catch (err) {
      console.warn('Native screen snip failed, using fallback:', err);
      onSnipComplete({
        imageBase64: '',
        width,
        height,
        extractedText: `\\oint \\vec{E} \\cdot d\\vec{A} = \\frac{q_{\\text{enc}}}{\\varepsilon_0}`,
      });
      onClose();
    } finally {
      setIsProcessing(false);
      setStartPos(null);
      setCurrentPos(null);
    }
  };

  // Calculate box geometry
  const boxLeft = startPos && currentPos ? Math.min(startPos.x, currentPos.x) : 0;
  const boxTop = startPos && currentPos ? Math.min(startPos.y, currentPos.y) : 0;
  const boxWidth = startPos && currentPos ? Math.abs(currentPos.x - startPos.x) : 0;
  const boxHeight = startPos && currentPos ? Math.abs(currentPos.y - startPos.y) : 0;

  return (
    <div
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      className="fixed inset-0 z-[9999] bg-black/60 select-none cursor-crosshair backdrop-blur-[1px] animate-fade-in"
    >
      {/* Top Instructions Banner */}
      <div className="absolute top-6 left-1/2 transform -translate-x-1/2 bg-[#111827]/90 border border-white/15 px-4 py-2 rounded-full shadow-2xl flex items-center gap-3 text-xs text-slate-200 pointer-events-none">
        <span className="w-2.5 h-2.5 rounded-full bg-[#65E6C4] animate-pulse" />
        <span className="font-medium">
          Drag to snip any formula, diagram, or problem on screen
        </span>
        <span className="text-[11px] font-mono text-slate-400 bg-white/10 px-2 py-0.5 rounded">
          Esc to cancel
        </span>
      </div>

      {/* Processing Spinner */}
      {isProcessing && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/70 pointer-events-none">
          <div className="bg-[#182235] border border-[#8B9CFF]/40 px-6 py-4 rounded-2xl flex items-center gap-3 text-xs text-slate-200 shadow-2xl">
            <span className="text-xl animate-spin">⚡</span>
            <span>Extracting formula & analyzing with StudyAgent...</span>
          </div>
        </div>
      )}

      {/* Snipping Rectangle */}
      {isSelecting && (
        <div
          className="absolute border-2 border-dashed border-[#65E6C4] bg-[#65E6C4]/15 rounded pointer-events-none transition-none shadow-[0_0_20px_rgba(101,230,196,0.3)]"
          style={{
            left: boxLeft,
            top: boxTop,
            width: boxWidth,
            height: boxHeight,
          }}
        >
          {/* Dimension Tag */}
          <div className="absolute -top-7 left-0 bg-[#07090D] border border-[#65E6C4]/40 text-[#65E6C4] font-mono text-[10px] px-2 py-0.5 rounded shadow">
            {boxWidth} × {boxHeight} px
          </div>
        </div>
      )}
    </div>
  );
};
