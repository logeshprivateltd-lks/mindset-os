/**
 * MindSet OS — Global React Error Boundary
 * Prevents black/blank screens by intercepting runtime errors and rendering
 * a helpful neuroaesthetic diagnostic recovery card.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallbackTitle?: string;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
    errorInfo: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('[MindSet ErrorBoundary] Caught uncaught component error:', error, errorInfo);
    this.setState({ errorInfo });
  }

  private handleReload = () => {
    window.location.reload();
  };

  private handleResetOnboarding = () => {
    localStorage.removeItem('mindset_onboarding_completed');
    window.location.reload();
  };

  private handleBypassToDesktop = () => {
    localStorage.setItem('mindset_onboarding_completed', 'true');
    this.setState({ hasError: false, error: null, errorInfo: null });
  };

  public render() {
    if (this.state.hasError) {
      return (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-6 bg-[#07090D] text-[#F9FAFB] select-none font-sans">
          <div className="w-full max-w-xl p-8 rounded-2xl bg-[#111827] border border-red-500/30 shadow-[0_12px_48px_rgba(0,0,0,0.8)] space-y-6">
            <div className="flex items-center space-x-3 pb-4 border-b border-[#1F2937]">
              <div className="w-10 h-10 rounded-xl bg-red-950/60 border border-red-500/40 flex items-center justify-center text-xl text-red-400">
                ⚠️
              </div>
              <div>
                <h2 className="text-base font-bold text-[#F9FAFB]">
                  {this.props.fallbackTitle || 'MindSet OS Component Recovery'}
                </h2>
                <p className="text-xs text-[#9CA3AF]">
                  An unexpected UI exception was intercepted to prevent a blank screen.
                </p>
              </div>
            </div>

            {/* Error Message */}
            <div className="p-4 rounded-xl bg-[#07090D] border border-[#1F2937] font-mono text-xs text-red-300 space-y-2 max-h-48 overflow-y-auto">
              <div className="font-bold text-red-400">
                {this.state.error?.name || 'Error'}: {this.state.error?.message || 'Unknown render exception'}
              </div>
              {this.state.errorInfo?.componentStack && (
                <pre className="text-[10px] text-[#6B7280] whitespace-pre-wrap">
                  {this.state.errorInfo.componentStack.slice(0, 500)}
                </pre>
              )}
            </div>

            {/* Recovery Action Buttons */}
            <div className="flex flex-wrap items-center justify-end gap-3 pt-2">
              <button
                type="button"
                onClick={this.handleBypassToDesktop}
                className="px-4 py-2 rounded-xl text-xs font-semibold bg-[#1F2937] hover:bg-[#374151] text-[#F9FAFB] transition-colors"
              >
                Skip to Desktop →
              </button>
              <button
                type="button"
                onClick={this.handleResetOnboarding}
                className="px-4 py-2 rounded-xl text-xs font-semibold bg-amber-950/40 border border-amber-600/30 text-amber-200 hover:bg-amber-900/50 transition-colors"
              >
                Reset Setup Wizard
              </button>
              <button
                type="button"
                onClick={this.handleReload}
                className="px-5 py-2 rounded-xl text-xs font-bold bg-[#8B9CFF] hover:bg-[#7A8BEE] text-[#07090D] shadow-lg shadow-[#8B9CFF]/20 transition-colors"
              >
                Reload MindSet OS ⟳
              </button>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
