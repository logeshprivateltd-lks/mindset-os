/**
 * MindSet OS Study Reader Native Window App
 * Textbook PDF & Syllabus Markdown Reader with distraction-free reading controls
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';

export const StudyReaderApp: React.FC = () => {
  const [activeChapter, setActiveChapter] = useState('ch-1');
  const [zoomLevel, setZoomLevel] = useState(100);

  const chapters = [
    { id: 'ch-1', title: '1. First-Order ODEs: Separation of Variables' },
    { id: 'ch-2', title: '2. Exact Differential Equations & Integrating Factors' },
    { id: 'ch-3', title: '3. Second-Order Linear Homogeneous ODEs' },
    { id: 'ch-4', title: '4. Non-Homogeneous Equations: Method of Undetermined Coeffs' },
    { id: 'ch-5', title: '5. Laplace Transforms & Inverse Operations' },
  ];

  return (
    <div className="flex h-full text-[var(--text-dominant)] select-text bg-[var(--surface-primary)]">
      {/* Chapter Sidebar */}
      <div className="w-64 border-r border-[var(--border-subtle)] p-3 flex flex-col justify-between shrink-0 bg-[var(--surface-secondary)]">
        <div className="space-y-2">
          <div className="text-[11px] font-bold uppercase tracking-wider text-[var(--accent-primary)] mb-2">
            Table of Contents
          </div>
          <div className="space-y-1">
            {chapters.map((ch) => (
              <button
                key={ch.id}
                type="button"
                onClick={() => setActiveChapter(ch.id)}
                className={`w-full text-left p-2 rounded-lg text-xs transition-colors ${
                  activeChapter === ch.id
                    ? 'bg-[var(--surface-elevated)] text-[var(--accent-primary)] font-semibold border border-[var(--border-subtle)]'
                    : 'text-[var(--text-muted)] hover:text-[var(--text-dominant)] hover:bg-[var(--surface-primary)]'
                }`}
              >
                {ch.title}
              </button>
            ))}
          </div>
        </div>

        {/* Reader Zoom Bar */}
        <div className="pt-3 border-t border-[var(--border-subtle)] flex items-center justify-between text-xs text-[var(--text-muted)]">
          <span>Zoom: {zoomLevel}%</span>
          <div className="space-x-1">
            <button
              type="button"
              onClick={() => setZoomLevel((z) => Math.max(75, z - 10))}
              className="px-2 py-0.5 rounded bg-[var(--surface-primary)] border border-[var(--border-subtle)]"
            >
              -
            </button>
            <button
              type="button"
              onClick={() => setZoomLevel((z) => Math.min(150, z + 10))}
              className="px-2 py-0.5 rounded bg-[var(--surface-primary)] border border-[var(--border-subtle)]"
            >
              +
            </button>
          </div>
        </div>
      </div>

      {/* Reader Content Body */}
      <div className="flex-1 p-6 md:p-8 overflow-y-auto space-y-4 font-serif leading-relaxed" style={{ fontSize: `${(15 * zoomLevel) / 100}px` }}>
        <div className="border-b border-[var(--border-subtle)] pb-3 mb-4">
          <span className="text-xs font-mono font-bold uppercase text-[var(--accent-secondary)]">
            Engineering Mathematics II • Chapter 1
          </span>
          <h1 className="text-2xl font-bold font-sans mt-1 text-[var(--text-dominant)]">
            First-Order Ordinary Differential Equations
          </h1>
        </div>

        <p className="text-[var(--text-dominant)] leading-relaxed">
          A first-order differential equation represents a mathematical relationship between an independent
          variable x, a dependent function y(x), and its first derivative dy/dx.
          In classical Newtonian mechanics and circuit analysis, these equations model instantaneous rates of change.
        </p>

        <div className="p-4 my-4 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] font-mono text-sm space-y-1">
          <div><strong>General Form:</strong> dy/dx = f(x, y)</div>
          <div><strong>Separable Form:</strong> g(y) dy = h(x) dx</div>
        </div>

        <h2 className="text-xl font-bold font-sans pt-2 text-[var(--text-dominant)]">
          Method of Separation of Variables
        </h2>
        <p className="text-[var(--text-dominant)] leading-relaxed">
          If the function f(x, y) can be factored into a product of a function strictly dependent on x
          and a function strictly dependent on y, such that f(x, y) = g(x) · h(y), the variables can
          be directly separated onto opposing sides of the equality sign:
        </p>

        <div className="p-4 my-4 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] font-mono text-sm">
          ∫ [1 / h(y)] dy = ∫ g(x) dx + C
        </div>

        <p className="text-[var(--text-muted)] text-sm italic">
          Tip: Press Alt + B at any time to engage Instant Privacy Blur if reading sensitive notes in a library.
        </p>
      </div>
    </div>
  );
};
