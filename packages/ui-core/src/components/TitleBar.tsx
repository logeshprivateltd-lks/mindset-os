import React from 'react';

export interface TitleBarProps {
  title?: string;
  subtitle?: string;
  memoryUsageMb?: number;
  kernelPhase?: string;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onClose?: () => void;
}

export const TitleBar: React.FC<TitleBarProps> = ({
  title = "MindSet OS — Powered by LKS PVT LTD",
  subtitle = "Native Core v0.1.0",
  memoryUsageMb,
  kernelPhase,
  onMinimize,
  onMaximize,
  onClose,
}) => {
  return (
    <header
      data-tauri-drag-region
      className="flex items-center justify-between h-9 px-3 bg-[#07090D] border-b border-[#1F2937] select-none text-xs text-[#9CA3AF] z-50 fixed top-0 left-0 right-0"
      style={{ WebkitAppRegion: 'drag' } as React.CSSProperties}
    >
      <div className="flex items-center space-x-2 pointer-events-none" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        <div className="w-2.5 h-2.5 rounded-full bg-[#8B9CFF] shadow-[0_0_8px_#8B9CFF]" />
        <span className="font-semibold text-[#F9FAFB] tracking-wide">{title}</span>
        <span className="text-[#6B7280]">|</span>
        <span className="text-[#9CA3AF]">{subtitle}</span>
      </div>

      <div className="flex items-center space-x-3" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        {kernelPhase && (
          <div className="flex items-center space-x-1.5 px-2 py-0.5 rounded bg-[#182235] border border-[#202B40] text-[10px]">
            <span className="text-[#65E6C4]">●</span>
            <span className="font-mono text-[#E5E7EB]">{kernelPhase}</span>
          </div>
        )}

        {typeof memoryUsageMb === 'number' && (
          <div className="flex items-center space-x-1.5 px-2 py-0.5 rounded bg-[#111827] border border-[#1F2937] text-[10px]">
            <span className="text-[#9CA3AF]">RSS:</span>
            <span className={`font-mono font-semibold ${memoryUsageMb > 100 ? 'text-[#FFD078]' : 'text-[#65E6C4]'}`}>
              {memoryUsageMb.toFixed(1)} MB
            </span>
          </div>
        )}

        <div className="flex items-center space-x-1">
          <button
            type="button"
            onClick={onMinimize}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-[#182235] text-[#9CA3AF] hover:text-[#F9FAFB] transition-colors"
            aria-label="Minimize"
          >
            <svg width="10" height="2" viewBox="0 0 10 2" fill="currentColor">
              <rect width="10" height="2" rx="1" />
            </svg>
          </button>
          <button
            type="button"
            onClick={onMaximize}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-[#182235] text-[#9CA3AF] hover:text-[#F9FAFB] transition-colors"
            aria-label="Maximize"
          >
            <svg width="9" height="9" viewBox="0 0 9 9" fill="none" stroke="currentColor" strokeWidth="1.5">
              <rect x="0.75" y="0.75" width="7.5" height="7.5" rx="1" />
            </svg>
          </button>
          <button
            type="button"
            onClick={onClose}
            className="w-6 h-6 flex items-center justify-center rounded hover:bg-[#FF6B6B]/20 text-[#9CA3AF] hover:text-[#FF6B6B] transition-colors"
            aria-label="Close"
          >
            <svg width="9" height="9" viewBox="0 0 9 9" stroke="currentColor" strokeWidth="1.5">
              <line x1="1" y1="1" x2="8" y2="8" />
              <line x1="8" y1="1" x2="1" y2="8" />
            </svg>
          </button>
        </div>
      </div>
    </header>
  );
};

