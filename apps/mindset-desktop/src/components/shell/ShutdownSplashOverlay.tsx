/**
 * MindSet OS Shutdown & Power Teardown Overlay Component
 * Visual diagnostic progress splash showing database flush, service unmount, and safe halt
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { usePersonalizationStore } from '../../stores/usePersonalizationStore';

export const ShutdownSplashOverlay: React.FC = () => {
  const {
    isShuttingDown,
    shutdownAction,
    shutdownMessage,
    teardownSteps,
    cancelShutdownOverlay,
  } = usePersonalizationStore();

  if (!isShuttingDown) return null;

  const isHaltReady = teardownSteps.every((s) => s.status === 'completed');
  const actionName =
    shutdownAction === 'restart'
      ? 'Restarting'
      : shutdownAction === 'sleep'
      ? 'Entering Sleep'
      : 'Shutting Down';

  return (
    <div
      role="alertdialog"
      aria-label="MindSet OS Power Teardown"
      className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-black/90 backdrop-blur-3xl select-none animate-fadeIn text-white p-6"
    >
      <div className="max-w-md w-full flex flex-col items-center text-center space-y-6">
        {/* MindSet OS Native Brand Emblem */}
        <div className="relative">
          <div className="w-20 h-20 rounded-3xl bg-gradient-to-tr from-[#1E1B4B] to-[#312E81] border border-[#8B9CFF]/40 flex items-center justify-center shadow-[0_0_50px_rgba(139,156,255,0.3)]">
            <span className="text-3xl font-extrabold text-[#8B9CFF] tracking-tighter">⚡</span>
          </div>
          <div className="absolute -inset-1 rounded-3xl border border-[#8B9CFF]/30 animate-pulse pointer-events-none" />
        </div>

        {/* Title & Status */}
        <div className="space-y-1.5">
          <h2 className="text-2xl font-bold tracking-tight text-slate-100">
            {actionName} MindSet OS...
          </h2>
          <p className="text-xs text-slate-400 font-mono">{shutdownMessage}</p>
        </div>

        {/* Diagnostic Teardown Sequence Checklist */}
        <div className="w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 text-left space-y-3 shadow-inner">
          <div className="text-[11px] font-semibold text-slate-400 uppercase tracking-wider flex items-center justify-between">
            <span>Deterministic Teardown Sequence</span>
            <span className="font-mono text-[#8B9CFF]">
              {teardownSteps.filter((s) => s.status === 'completed').length}/{teardownSteps.length}
            </span>
          </div>

          <div className="space-y-2.5">
            {teardownSteps.map((step) => {
              const isRunning = step.status === 'running';
              const isDone = step.status === 'completed';
              const isFailed = step.status === 'failed';

              return (
                <div key={step.id} className="flex items-center space-x-3 text-xs">
                  {/* Status Indicator Icon */}
                  <div className="w-4 h-4 flex items-center justify-center flex-shrink-0">
                    {isDone ? (
                      <span className="text-emerald-400 text-sm">✓</span>
                    ) : isRunning ? (
                      <span className="w-3.5 h-3.5 rounded-full border-2 border-[#8B9CFF] border-t-transparent animate-spin" />
                    ) : isFailed ? (
                      <span className="text-rose-400 text-sm">✗</span>
                    ) : (
                      <span className="w-2 h-2 rounded-full bg-slate-700" />
                    )}
                  </div>

                  {/* Step Label */}
                  <span
                    className={`transition-colors truncate ${
                      isDone
                        ? 'text-slate-300 font-medium'
                        : isRunning
                        ? 'text-[#8B9CFF] font-semibold'
                        : isFailed
                        ? 'text-rose-400'
                        : 'text-slate-500'
                    }`}
                  >
                    {step.label}
                  </span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Safe Halt Indicator or Abort Button */}
        {isHaltReady ? (
          <div className="flex items-center space-x-2 text-xs text-emerald-400 font-mono animate-pulse">
            <span>●</span>
            <span>All sub-systems unmounted cleanly. Powering down.</span>
          </div>
        ) : (
          <button
            type="button"
            onClick={cancelShutdownOverlay}
            className="px-4 py-1.5 rounded-xl border border-slate-700 hover:border-slate-500 text-xs text-slate-400 hover:text-slate-200 transition-colors"
          >
            Cancel Teardown
          </button>
        )}
      </div>
    </div>
  );
};

