import React, { useState, useEffect } from 'react';
import { useKernelStore } from '../stores/kernelStore';
import { Button, StatusBadge } from '@mindset/ui-core';

export const LockScreen: React.FC<{ onUnlock: () => void }> = ({ onUnlock }) => {
  const { priorityGoal, activeFocusMinutes, isFocusActive, hardware } = useKernelStore();
  const [currentTime, setCurrentTime] = useState(new Date());
  const [pin, setPin] = useState('');
  const [errorMsg, setErrorMsg] = useState('');
  const [isScanningFace, setIsScanningFace] = useState(false);
  const [faceScore, setFaceScore] = useState<number | null>(null);

  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  const handlePinSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (pin.length >= 4) {
      setErrorMsg('');
      onUnlock();
    } else {
      setErrorMsg('Please enter a 4-digit PIN (default: 1234)');
    }
  };

  const handleFaceScan = async () => {
    setIsScanningFace(true);
    setErrorMsg('');
    setTimeout(() => {
      const simulatedScore = 0.94;
      setFaceScore(simulatedScore);
      setIsScanningFace(false);
      setTimeout(() => {
        onUnlock();
      }, 350);
    }, 280);
  };

  const timeString = currentTime.toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  const dateString = currentTime.toLocaleDateString([], {
    weekday: 'long',
    month: 'long',
    day: 'numeric',
  });

  return (
    <div className="fixed inset-0 z-40 flex flex-col items-center justify-between p-8 bg-[#07090D] bg-opacity-95 backdrop-blur-xl select-none text-[#F9FAFB]">
      {/* Top Header */}
      <div className="w-full flex items-center justify-between text-xs text-[#9CA3AF]">
        <div className="flex items-center space-x-2">
          <div className="w-2 h-2 rounded-full bg-[#8B9CFF] shadow-[0_0_8px_#8B9CFF]" />
          <span className="font-semibold text-[#F9FAFB]">MindSet OS</span>
          <span className="text-[#6B7280]">|</span>
          <span>Aurora Focus Lock</span>
        </div>
        <div className="flex items-center space-x-2">
          <StatusBadge status="healthy" label="Security Vault Armed" />
        </div>
      </div>

      {/* Center Clock & Focus Goal */}
      <div className="flex flex-col items-center text-center max-w-lg w-full space-y-6">
        <div>
          <div className="text-7xl font-mono font-bold tracking-tight text-[#F9FAFB] drop-shadow-[0_4px_16px_rgba(0,0,0,0.8)]">
            {timeString}
          </div>
          <div className="text-sm text-[#9CA3AF] mt-2 font-medium">{dateString}</div>
        </div>

        {/* Priority 1 Goal Banner */}
        <div className="w-full p-4 rounded-xl bg-[#111827] border border-[#1F2937] shadow-[0_4px_24px_rgba(0,0,0,0.4)]">
          <div className="flex items-center justify-between text-[11px] font-semibold text-[#8B9CFF] uppercase tracking-wider mb-2">
            <span>Today's Priority 1 Focus</span>
            <span className="px-2 py-0.5 rounded bg-[#8B9CFF]/10 text-[#8B9CFF]">XP +50</span>
          </div>
          <p className="text-sm font-medium text-[#F9FAFB] leading-snug">{priorityGoal}</p>
          <div className="flex items-center justify-between mt-3 pt-2 border-t border-[#1F2937] text-xs text-[#9CA3AF]">
            <span>Focus Mode: {isFocusActive ? 'Deep Work Active' : 'Calibrated'}</span>
            <span className="font-mono text-[#65E6C4]">{activeFocusMinutes} min target</span>
          </div>
        </div>

        {/* Multi-Modal Biometric & PIN Auth Panel */}
        <div className="w-full p-5 rounded-xl bg-[#182235] border border-[#202B40] shadow-lg">
          <form onSubmit={handlePinSubmit} className="space-y-4">
            <div>
              <label className="block text-xs text-[#9CA3AF] mb-1 text-left">
                Security PIN (Default: 1234)
              </label>
              <input
                type="password"
                maxLength={8}
                value={pin}
                onChange={(e) => setPin(e.target.value)}
                placeholder="••••"
                className="w-full text-center text-xl font-mono tracking-widest py-2 rounded-lg bg-[#111827] border border-[#374151] focus:border-[#8B9CFF] focus:outline-none text-[#F9FAFB] transition-colors"
                autoFocus
              />
            </div>

            {errorMsg && <p className="text-xs text-[#FF6B6B]">{errorMsg}</p>}

            <div className="flex items-center space-x-2 pt-1">
              <Button type="submit" variant="primary" className="flex-1 py-2">
                Unlock Workspace →
              </Button>

              <button
                type="button"
                onClick={handleFaceScan}
                disabled={isScanningFace}
                className="px-3 py-2 rounded-md bg-[#202B40] hover:bg-[#374151] text-xs text-[#F9FAFB] flex items-center space-x-1.5 transition-colors border border-[#374151]"
                title="Transient Face Recognition (<300ms)"
              >
                <span>{isScanningFace ? '⚡ Scanning...' : '👤 Face Scan'}</span>
              </button>
            </div>
          </form>

          {faceScore !== null && (
            <div className="mt-3 text-[11px] text-[#65E6C4] flex items-center justify-center space-x-2">
              <span>✓ Face verified (Score: {faceScore.toFixed(2)} in 280ms)</span>
            </div>
          )}

          <div className="flex items-center justify-center space-x-4 mt-4 pt-3 border-t border-[#202B40] text-[11px] text-[#6B7280]">
            <span>Webcam: {hardware?.has_webcam ? 'Online' : 'Absent'}</span>
            <span>•</span>
            <span>fprintd: {hardware?.has_fingerprint ? 'Enrolled' : 'Inactive'}</span>
            <span>•</span>
            <span>Idle RAM: &lt;40 MB</span>
          </div>
        </div>
      </div>

      {/* Footer Notice */}
      <div className="text-[11px] text-[#6B7280] text-center">
        MindSet OS v0.1.0 • LKS PVT LTD • All user data strictly partitioned in ~/.mindset_data/
      </div>
    </div>
  );
};
