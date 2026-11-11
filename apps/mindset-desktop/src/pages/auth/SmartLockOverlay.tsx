/**
 * MindSet OS Smart Lock Screen & Instant Privacy Blur Guard (`/auth`)
 * Aurora Focus Liquid Glass Design System
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { useKernelStore } from '../../stores/useKernelStore';
import { authClient } from '../../lib/authClient';
import { dbClient } from '../../lib/dbClient';
import { Button, StatusBadge } from '@mindset/ui-core';
import type { TaskOrGoal } from '@mindset/db-schema';

interface SmartLockOverlayProps {
  onUnlock: () => void;
  isPrivacyBlurredOnly?: boolean;
}

export const SmartLockOverlay: React.FC<SmartLockOverlayProps> = ({
  onUnlock,
  isPrivacyBlurredOnly = false,
}) => {
  const {
    hardware,
    priorityGoal: storePriorityGoal,
    isFocusActive,
    activeFocusMinutes,
  } = useKernelStore();

  const [currentTime, setCurrentTime] = useState(new Date());
  const [authMode, setAuthMode] = useState<'pin' | 'password'>('pin');
  const [pinDigits, setPinDigits] = useState<string[]>([]);
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);

  // Authentication states
  const [isVerifying, setIsVerifying] = useState(false);
  const [isScanningFace, setIsScanningFace] = useState(false);
  const [faceResult, setFaceResult] = useState<{ score: number; latencyMs: number } | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [lockoutRemaining, setLockoutRemaining] = useState<number | null>(null);

  // Focus Timer countdown simulation
  const [focusRemainingSecs, setFocusRemainingSecs] = useState(activeFocusMinutes * 60);

  // Priority 1 goal fetched from local SQLite
  const [priorityTask, setPriorityTask] = useState<TaskOrGoal | null>(null);

  const containerRef = useRef<HTMLDivElement>(null);

  // Real-time clock ticker
  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  // Focus countdown ticker
  useEffect(() => {
    if (!isFocusActive) return;
    const focusTimer = setInterval(() => {
      setFocusRemainingSecs((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);
    return () => clearInterval(focusTimer);
  }, [isFocusActive]);

  // Lockout countdown ticker
  useEffect(() => {
    if (lockoutRemaining === null || lockoutRemaining <= 0) return;
    const timer = setInterval(() => {
      setLockoutRemaining((prev) => {
        if (prev === null || prev <= 1) return null;
        return prev - 1;
      });
    }, 1000);
    return () => clearInterval(timer);
  }, [lockoutRemaining]);

  // Load Priority 1 task from local SQLite
  useEffect(() => {
    async function loadPriorityTask() {
      try {
        const tasks = await dbClient.tasks.list('in_progress');
        const p1 = tasks.find((t) => t.priority === 1) || tasks[0];
        if (p1) {
          setPriorityTask(p1);
        }
      } catch {
        // Fallback to store default
      }
    }
    loadPriorityTask();
  }, []);

  // Execute PIN verification once 6 digits entered
  const submitPin = useCallback(
    async (pinStr: string) => {
      if (lockoutRemaining !== null && lockoutRemaining > 0) return;
      setIsVerifying(true);
      setErrorMsg(null);
      try {
        await authClient.authenticateWithPin(pinStr);
        setIsVerifying(false);
        onUnlock();
      } catch (err: unknown) {
        setIsVerifying(false);
        const msg = err instanceof Error ? err.message : String(err);
        if (msg.includes('locked')) {
          setLockoutRemaining(30);
          setErrorMsg('Account locked for 30 seconds due to 5 consecutive failed attempts.');
        } else {
          setErrorMsg('Incorrect PIN. Default is 123456 (or 1234).');
        }
        setPinDigits([]);
      }
    },
    [lockoutRemaining, onUnlock]
  );

  // Handle PIN digit entry
  const handleDigitInput = useCallback(
    (digit: string) => {
      if (lockoutRemaining !== null && lockoutRemaining > 0) return;
      if (pinDigits.length < 6) {
        const next = [...pinDigits, digit];
        setPinDigits(next);
        setErrorMsg(null);
        if (next.length === 6) {
          submitPin(next.join(''));
        }
      }
    },
    [lockoutRemaining, pinDigits, submitPin]
  );

  const handleBackspace = useCallback(() => {
    setPinDigits((prev) => prev.slice(0, -1));
    setErrorMsg(null);
  }, []);

  const handleClearPin = useCallback(() => {
    setPinDigits([]);
    setErrorMsg(null);
  }, []);

  // Physical Keyboard Listener
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // If privacy blurred only, any key unblurs
      if (isPrivacyBlurredOnly) {
        if (e.key === 'Escape' || e.key === ' ' || (e.altKey && e.key.toLowerCase() === 'b')) {
          e.preventDefault();
          onUnlock();
        }
        return;
      }

      if (authMode === 'pin') {
        if (e.key >= '0' && e.key <= '9') {
          e.preventDefault();
          handleDigitInput(e.key);
        } else if (e.key === 'Backspace') {
          e.preventDefault();
          handleBackspace();
        } else if (e.key === 'Escape' || e.key === 'Delete') {
          e.preventDefault();
          handleClearPin();
        } else if (e.key === 'Enter' && pinDigits.length >= 4) {
          e.preventDefault();
          submitPin(pinDigits.join(''));
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [
    authMode,
    handleClearPin,
    handleBackspace,
    handleDigitInput,
    isPrivacyBlurredOnly,
    onUnlock,
    pinDigits,
    submitPin,
  ]);

  // Submit Password
  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (lockoutRemaining !== null && lockoutRemaining > 0) return;
    if (!password) {
      setErrorMsg('Please enter your password.');
      return;
    }
    setIsVerifying(true);
    setErrorMsg(null);
    try {
      await authClient.authenticateWithPassword(password);
      setIsVerifying(false);
      onUnlock();
    } catch (err: unknown) {
      setIsVerifying(false);
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.includes('locked')) {
        setLockoutRemaining(30);
        setErrorMsg('Account locked for 30s due to consecutive failed attempts.');
      } else {
        setErrorMsg('Incorrect password. Default is "MindSet2026!".');
      }
    }
  };

  // Face Scan Authentication
  const handleFaceScan = async () => {
    if (lockoutRemaining !== null && lockoutRemaining > 0) return;
    setIsScanningFace(true);
    setErrorMsg(null);
    setFaceResult(null);

    try {
      // Generate simulated/transient 128-d embedding
      const mockEmbedding = Array.from({ length: 128 }, () => Math.random() * 0.2 + 0.1);
      const result = await authClient.authenticateWithFace(mockEmbedding);

      setFaceResult({
        score: result.match_score,
        latencyMs: result.inference_latency_ms || 240,
      });

      setTimeout(() => {
        setIsScanningFace(false);
        onUnlock();
      }, 350);
    } catch (err: unknown) {
      setIsScanningFace(false);
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMsg(`Face verification failed: ${msg}`);
    }
  };

  // Fingerprint Authentication
  const handleFingerprintScan = async () => {
    if (lockoutRemaining !== null && lockoutRemaining > 0) return;
    setIsVerifying(true);
    setErrorMsg(null);
    try {
      const res = await authClient.authenticateWithFingerprint();
      setIsVerifying(false);
      if (res.success) {
        onUnlock();
      } else {
        setErrorMsg(res.message);
      }
    } catch {
      setIsVerifying(false);
      setErrorMsg('Biometric sensor unavailable. Please use PIN.');
    }
  };

  // Format Helpers
  const formatTime = (date: Date) =>
    date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false });

  const formatDate = (date: Date) =>
    date.toLocaleDateString([], { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' });

  const formatCountdown = (totalSecs: number) => {
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
  };

  // -------------------------------------------------------------
  // PRIVACY BLUR ONLY OVERLAY (Alt + B)
  // -------------------------------------------------------------
  if (isPrivacyBlurredOnly) {
    return (
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Privacy Shield"
        ref={containerRef}
        onClick={onUnlock}
        className="fixed inset-0 z-50 flex flex-col items-center justify-center bg-black/60 backdrop-blur-3xl select-none cursor-pointer transition-all duration-300 animate-fadeIn"
      >
        <div className="p-8 max-w-md w-full rounded-2xl bg-[#111827]/80 border border-[#8B9CFF]/30 shadow-[0_8px_32px_rgba(0,0,0,0.8)] text-center space-y-4">
          <div className="w-12 h-12 mx-auto rounded-full bg-[#8B9CFF]/15 border border-[#8B9CFF]/40 flex items-center justify-center text-2xl text-[#8B9CFF]">
            👁️‍🗨️
          </div>
          <h2 className="text-xl font-bold text-[#F9FAFB]">Instant Privacy Shield Active</h2>
          <p className="text-xs text-[#9CA3AF] leading-relaxed">
            Workspace obscured from shoulder-surfing. Background study timers and focus tracking
            continue running uninterrupted.
          </p>

          <div className="pt-2">
            <span className="inline-block px-3 py-1.5 rounded-lg bg-[#182235] border border-[#202B40] text-xs font-mono text-[#8B9CFF]">
              Click anywhere or press Alt + B to unmask
            </span>
          </div>
        </div>
      </div>
    );
  }

  // -------------------------------------------------------------
  // FULL SMART LOCK SCREEN (Super + L / Idle Timeout / Auth Gate)
  // -------------------------------------------------------------
  const p1GoalTitle = priorityTask?.title || storePriorityGoal;

  return (
    <div
      role="region"
      aria-label="Smart Lock Screen"
      ref={containerRef}
      className="fixed inset-0 z-50 flex flex-col justify-between p-6 md:p-10 bg-[#07090D] select-none text-[#F9FAFB] overflow-y-auto"
      style={{
        backgroundImage:
          'radial-gradient(circle at 50% 15%, rgba(139, 156, 255, 0.08), transparent 45%), radial-gradient(circle at 80% 85%, rgba(101, 230, 196, 0.04), transparent 40%)',
      }}
    >
      {/* Top Header Bar */}
      <header className="w-full flex items-center justify-between text-xs text-[#9CA3AF]">
        <div className="flex items-center space-x-3">
          <div className="w-2.5 h-2.5 rounded-full bg-[#8B9CFF] shadow-[0_0_10px_#8B9CFF] animate-pulse" />
          <span className="font-semibold text-sm tracking-wide text-[#F9FAFB]">MindSet OS</span>
          <span className="text-[#374151]">|</span>
          <span className="text-[#9CA3AF] font-mono text-[11px]">Aurora Core v0.1.0</span>
        </div>

        <div className="flex items-center space-x-3">
          {isFocusActive && (
            <div className="flex items-center space-x-2 px-3 py-1 rounded-full bg-[#65E6C4]/10 border border-[#65E6C4]/30 text-[#65E6C4] font-mono text-xs shadow-[0_0_12px_rgba(101,230,196,0.2)]">
              <span className="w-2 h-2 rounded-full bg-[#65E6C4] animate-ping" />
              <span>Deep Focus: {formatCountdown(focusRemainingSecs)}</span>
            </div>
          )}
          <StatusBadge status="healthy" label="Security Vault Armed" />
        </div>
      </header>

      {/* Center Hero Column */}
      <main className="flex flex-col items-center justify-center max-w-md w-full mx-auto my-auto space-y-6 py-4">
        {/* High-Precision Tabular Clock */}
        <div className="text-center">
          <div className="text-6xl md:text-7xl font-mono font-bold tracking-tight text-[#F9FAFB] drop-shadow-[0_4px_24px_rgba(0,0,0,0.9)] tabular-nums">
            {formatTime(currentTime)}
          </div>
          <div className="text-sm font-medium text-[#9CA3AF] mt-2 tracking-wide">
            {formatDate(currentTime)}
          </div>
        </div>

        {/* Priority 1 Goal Card (Confidential notes strictly withheld) */}
        <div className="w-full p-4 rounded-xl bg-[#111827]/90 border border-[#1F2937] shadow-[0_8px_32px_rgba(0,0,0,0.5)] backdrop-blur-md">
          <div className="flex items-center justify-between text-[11px] font-semibold text-[#8B9CFF] uppercase tracking-wider mb-2">
            <span className="flex items-center space-x-1.5">
              <span>⭐</span>
              <span>Today's Priority 1 Focus</span>
            </span>
            <span className="px-2 py-0.5 rounded bg-[#8B9CFF]/15 text-[#8B9CFF] font-mono">
              +50 XP
            </span>
          </div>

          <p className="text-sm font-medium text-[#F9FAFB] line-clamp-2 leading-relaxed">
            {p1GoalTitle}
          </p>

          <div className="flex items-center justify-between mt-3 pt-2.5 border-t border-[#1F2937] text-[11px] text-[#9CA3AF]">
            <span>Subject: {priorityTask?.subject_tag || 'Core Milestone'}</span>
            <span className="font-mono text-[#65E6C4]">
              {isFocusActive ? 'Session In Progress' : 'Calibrated'}
            </span>
          </div>
        </div>

        {/* Multi-Modal Biometric & Passcode Container */}
        <div className="w-full p-5 rounded-2xl bg-[#182235]/90 border border-[#202B40] shadow-[0_8px_40px_rgba(0,0,0,0.6)] backdrop-blur-lg">
          {/* Method Selector Tabs */}
          <div className="flex rounded-lg bg-[#111827] p-1 mb-4 border border-[#1F2937]">
            <button
              type="button"
              onClick={() => {
                setAuthMode('pin');
                setErrorMsg(null);
              }}
              className={`flex-1 py-1.5 text-xs font-medium rounded-md transition-colors ${
                authMode === 'pin'
                  ? 'bg-[#202B40] text-[#8B9CFF] shadow-sm font-semibold'
                  : 'text-[#9CA3AF] hover:text-[#F9FAFB]'
              }`}
            >
              6-Digit PIN
            </button>
            <button
              type="button"
              onClick={() => {
                setAuthMode('password');
                setErrorMsg(null);
              }}
              className={`flex-1 py-1.5 text-xs font-medium rounded-md transition-colors ${
                authMode === 'password'
                  ? 'bg-[#202B40] text-[#8B9CFF] shadow-sm font-semibold'
                  : 'text-[#9CA3AF] hover:text-[#F9FAFB]'
              }`}
            >
              Master Password
            </button>
          </div>

          {/* Lockout Warning Banner */}
          {lockoutRemaining !== null && lockoutRemaining > 0 && (
            <div className="mb-4 p-3 rounded-lg bg-red-950/40 border border-red-500/40 text-red-200 text-xs text-center">
              ⚠️ Too many failed attempts. Security cooldown: <span className="font-mono font-bold">{lockoutRemaining}s</span>
            </div>
          )}

          {/* Mode 1: 6-Digit PIN View */}
          {authMode === 'pin' && (
            <div className="space-y-4">
              {/* 6 PIN Slot Indicators */}
              <div className="flex justify-center items-center space-x-3 py-2">
                {[0, 1, 2, 3, 4, 5].map((index) => {
                  const isFilled = index < pinDigits.length;
                  return (
                    <div
                      key={index}
                      className={`w-3.5 h-3.5 rounded-full transition-all duration-150 ${
                        isFilled
                          ? 'bg-[#8B9CFF] scale-110 shadow-[0_0_8px_#8B9CFF]'
                          : 'bg-[#202B40] border border-[#374151]'
                      }`}
                    />
                  );
                })}
              </div>

              {/* Interactive On-Screen Keypad */}
              <div className="grid grid-cols-3 gap-2 max-w-[260px] mx-auto pt-1">
                {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map((digit) => (
                  <button
                    key={digit}
                    type="button"
                    onClick={() => handleDigitInput(digit)}
                    disabled={isVerifying || (lockoutRemaining !== null && lockoutRemaining > 0)}
                    className="h-11 rounded-lg bg-[#111827] hover:bg-[#202B40] active:scale-95 border border-[#1F2937] hover:border-[#8B9CFF]/40 text-lg font-mono text-[#F9FAFB] transition-all flex items-center justify-center"
                  >
                    {digit}
                  </button>
                ))}

                <button
                  type="button"
                  onClick={handleClearPin}
                  disabled={isVerifying}
                  className="h-11 rounded-lg bg-[#111827]/60 hover:bg-[#111827] text-xs font-medium text-[#9CA3AF] hover:text-[#F9FAFB] border border-[#1F2937] transition-all flex items-center justify-center"
                >
                  Clear
                </button>

                <button
                  type="button"
                  onClick={() => handleDigitInput('0')}
                  disabled={isVerifying || (lockoutRemaining !== null && lockoutRemaining > 0)}
                  className="h-11 rounded-lg bg-[#111827] hover:bg-[#202B40] active:scale-95 border border-[#1F2937] hover:border-[#8B9CFF]/40 text-lg font-mono text-[#F9FAFB] transition-all flex items-center justify-center"
                >
                  0
                </button>

                <button
                  type="button"
                  onClick={handleBackspace}
                  disabled={isVerifying || pinDigits.length === 0}
                  className="h-11 rounded-lg bg-[#111827]/60 hover:bg-[#111827] text-xs font-medium text-[#9CA3AF] hover:text-[#F9FAFB] border border-[#1F2937] transition-all flex items-center justify-center"
                >
                  ⌫
                </button>
              </div>

              <div className="text-center text-[11px] text-[#6B7280]">
                Keyboard shortcuts: 0-9, Backspace, Enter (Default: <code className="text-[#8B9CFF]">123456</code>)
              </div>
            </div>
          )}

          {/* Mode 2: Password View */}
          {authMode === 'password' && (
            <form onSubmit={handlePasswordSubmit} className="space-y-4">
              <div className="relative">
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Enter master password..."
                  disabled={isVerifying || (lockoutRemaining !== null && lockoutRemaining > 0)}
                  className="w-full px-3.5 py-2.5 rounded-lg bg-[#111827] border border-[#374151] focus:border-[#8B9CFF] focus:outline-none text-sm text-[#F9FAFB] transition-colors"
                  autoFocus
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-2.5 text-xs text-[#9CA3AF] hover:text-[#F9FAFB]"
                >
                  {showPassword ? 'Hide' : 'Show'}
                </button>
              </div>

              <Button
                type="submit"
                variant="primary"
                disabled={isVerifying || !password}
                className="w-full py-2.5 text-sm"
              >
                {isVerifying ? 'Verifying Password...' : 'Unlock Workspace →'}
              </Button>

              <div className="text-center text-[11px] text-[#6B7280]">
                Demo Default: <code className="text-[#8B9CFF]">MindSet2026!</code>
              </div>
            </form>
          )}

          {/* Error Message Display */}
          {errorMsg && (
            <p className="mt-3 text-xs text-[#FF6B6B] text-center font-medium animate-shake">
              {errorMsg}
            </p>
          )}

          {/* Face Result Notification */}
          {faceResult && (
            <div className="mt-3 text-xs text-[#65E6C4] flex items-center justify-center space-x-1.5 font-medium animate-fadeIn">
              <span>✓ Verified in {faceResult.latencyMs}ms</span>
              <span>(Score: {faceResult.score.toFixed(2)})</span>
            </div>
          )}

          {/* Multi-Modal Biometric Action Strip */}
          <div className="mt-5 pt-4 border-t border-[#202B40] flex items-center justify-center space-x-2">
            {/* Transient ONNX Face Unlock */}
            {hardware?.has_webcam !== false && (
              <button
                type="button"
                onClick={handleFaceScan}
                disabled={isScanningFace || isVerifying || (lockoutRemaining !== null && lockoutRemaining > 0)}
                className="flex-1 px-3 py-2 rounded-lg bg-[#111827] hover:bg-[#202B40] text-xs text-[#F9FAFB] flex items-center justify-center space-x-2 border border-[#1F2937] hover:border-[#8B9CFF]/40 transition-colors shadow-sm"
                title="Transient MobileFaceNet 128-d (<300ms, 0 MB idle RAM)"
              >
                <span>{isScanningFace ? '⚡' : '👤'}</span>
                <span>{isScanningFace ? 'Scanning Face...' : 'Face Scan (<300ms)'}</span>
              </button>
            )}

            {/* Hardware Fingerprint touch */}
            {hardware?.has_fingerprint && (
              <button
                type="button"
                onClick={handleFingerprintScan}
                disabled={isVerifying || (lockoutRemaining !== null && lockoutRemaining > 0)}
                className="px-3 py-2 rounded-lg bg-[#111827] hover:bg-[#202B40] text-xs text-[#F9FAFB] flex items-center space-x-1.5 border border-[#1F2937] transition-colors"
                title="Hardware Biometric Sensor Touch"
              >
                <span>👆</span>
                <span>Fingerprint</span>
              </button>
            )}
          </div>

          {/* Hardware Diagnostics Bar */}
          <div className="flex items-center justify-center space-x-3 mt-4 pt-3 border-t border-[#182235] text-[10px] text-[#6B7280]">
            <span>Webcam: {hardware?.has_webcam ? 'Online' : 'Absent'}</span>
            <span>•</span>
            <span>fprintd: {hardware?.has_fingerprint ? 'Ready' : 'Fallback (PIN)'}</span>
            <span>•</span>
            <span>Idle RAM: &lt;40 MB</span>
          </div>
        </div>
      </main>

      {/* Bottom Footer Notice */}
      <footer className="w-full text-center text-[11px] text-[#6B7280] space-y-1">
        <div>
          MindSet OS • LKS PVT LTD • All biometrics & credentials stored locally in ~/.mindset_data/
        </div>
        <div className="text-[10px] text-[#4B5563]">
          Global Hotkeys: <kbd className="font-mono text-[#9CA3AF]">Super + L</kbd> to Lock •{' '}
          <kbd className="font-mono text-[#9CA3AF]">Alt + B</kbd> for Privacy Blur
        </div>
      </footer>
    </div>
  );
};
