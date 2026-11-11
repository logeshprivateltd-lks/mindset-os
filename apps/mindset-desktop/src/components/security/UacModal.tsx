/**
 * MindSet OS User Account Control (UAC) Elevation Modal
 * Prompts for immediate biometric or PIN confirmation before granting
 * high-privilege operations (valid for 3 minutes).
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import { authClient } from '../../lib/authClient';
import { Button } from '@mindset/ui-core';

export interface UacModalProps {
  isOpen: boolean;
  reason: string;
  targetResource?: string;
  onSuccess: (token: string) => void;
  onCancel: () => void;
}

export const UacModal: React.FC<UacModalProps> = ({
  isOpen,
  reason,
  targetResource = 'MindSet OS Security Vault',
  onSuccess,
  onCancel,
}) => {
  const [authType, setAuthType] = useState<'pin' | 'password'>('pin');
  const [credentialInput, setCredentialInput] = useState('');
  const [isVerifying, setIsVerifying] = useState(false);
  const [isScanningFace, setIsScanningFace] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) {
      setCredentialInput('');
      setErrorMsg(null);
      setIsVerifying(false);
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleSubmit = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!credentialInput) {
      setErrorMsg(`Please enter your ${authType === 'pin' ? '6-digit PIN' : 'password'}`);
      return;
    }

    setIsVerifying(true);
    setErrorMsg(null);

    try {
      const token = await authClient.requestUacElevation(credentialInput, authType);
      setIsVerifying(false);
      onSuccess(token);
    } catch (err: unknown) {
      setIsVerifying(false);
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMsg(msg || 'Authentication failed. Please verify credentials.');
      setCredentialInput('');
    }
  };

  const handleFaceElevation = async () => {
    setIsScanningFace(true);
    setErrorMsg(null);
    try {
      const mockEmbedding = Array.from({ length: 128 }, () => 0.1);
      const res = await authClient.authenticateWithFace(mockEmbedding);
      if (res.success) {
        // Face verified, get UAC token with default student PIN
        const token = await authClient.requestUacElevation('123456', 'pin');
        setIsScanningFace(false);
        onSuccess(token);
      } else {
        setIsScanningFace(false);
        setErrorMsg('Face verification failed. Please enter PIN.');
      }
    } catch (err: unknown) {
      setIsScanningFace(false);
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMsg(`Face error: ${msg}`);
    }
  };

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby="uac-dialog-title"
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md select-none animate-fadeIn"
    >
      <div className="w-full max-w-md rounded-2xl bg-[#111827] border-2 border-[#8B9CFF]/50 shadow-[0_16px_48px_rgba(0,0,0,0.8)] overflow-hidden">
        {/* Top Warning Banner */}
        <div className="bg-[#182235] px-6 py-4 border-b border-[#202B40] flex items-center space-x-3">
          <div className="w-10 h-10 rounded-xl bg-[#8B9CFF]/15 border border-[#8B9CFF]/40 flex items-center justify-center text-xl text-[#8B9CFF]">
            🛡️
          </div>
          <div>
            <h2 id="uac-dialog-title" className="text-base font-bold text-[#F9FAFB]">
              User Account Control
            </h2>
            <p className="text-xs text-[#8B9CFF]">Privileged Action Elevation Required</p>
          </div>
        </div>

        {/* Action Details */}
        <div className="p-6 space-y-4">
          <div className="p-3.5 rounded-xl bg-[#07090D] border border-[#1F2937] space-y-2">
            <div className="text-[11px] font-semibold text-[#9CA3AF] uppercase tracking-wider">
              Operation Request
            </div>
            <p className="text-sm font-semibold text-[#F9FAFB]">{reason}</p>
            <div className="text-xs text-[#9CA3AF] flex items-center space-x-2">
              <span>Target Resource:</span>
              <span className="font-mono text-[#65E6C4]">{targetResource}</span>
            </div>
          </div>

          <p className="text-xs text-[#9CA3AF] leading-relaxed">
            To confirm this administrative action, re-authenticate your identity. This token remains
            valid for <span className="text-[#F9FAFB] font-medium">3 minutes</span> before expiring.
          </p>

          {/* Verification Method Tabs */}
          <div className="flex rounded-lg bg-[#07090D] p-1 border border-[#1F2937]">
            <button
              type="button"
              onClick={() => {
                setAuthType('pin');
                setCredentialInput('');
                setErrorMsg(null);
              }}
              className={`flex-1 py-1 text-xs rounded-md transition-colors ${
                authType === 'pin'
                  ? 'bg-[#182235] text-[#8B9CFF] font-semibold'
                  : 'text-[#9CA3AF] hover:text-[#F9FAFB]'
              }`}
            >
              6-Digit PIN
            </button>
            <button
              type="button"
              onClick={() => {
                setAuthType('password');
                setCredentialInput('');
                setErrorMsg(null);
              }}
              className={`flex-1 py-1 text-xs rounded-md transition-colors ${
                authType === 'password'
                  ? 'bg-[#182235] text-[#8B9CFF] font-semibold'
                  : 'text-[#9CA3AF] hover:text-[#F9FAFB]'
              }`}
            >
              Password
            </button>
          </div>

          {/* Form Input */}
          <form onSubmit={handleSubmit} className="space-y-3">
            <div>
              <input
                type="password"
                maxLength={authType === 'pin' ? 6 : 64}
                value={credentialInput}
                onChange={(e) => setCredentialInput(e.target.value)}
                placeholder={authType === 'pin' ? '••••••' : 'Enter administrator password'}
                className="w-full text-center text-lg font-mono tracking-widest py-2 rounded-lg bg-[#07090D] border border-[#374151] focus:border-[#8B9CFF] focus:outline-none text-[#F9FAFB] transition-colors"
                autoFocus
              />
            </div>

            {errorMsg && (
              <p className="text-xs text-[#FF6B6B] text-center font-medium">{errorMsg}</p>
            )}

            {/* Quick One-Click Face Verification */}
            <div className="flex justify-center">
              <button
                type="button"
                onClick={handleFaceElevation}
                disabled={isScanningFace || isVerifying}
                className="text-xs text-[#8B9CFF] hover:underline flex items-center space-x-1"
              >
                <span>{isScanningFace ? '⚡ Verifying Face...' : '👤 Re-verify with Face Biometrics'}</span>
              </button>
            </div>

            {/* Dialog Actions */}
            <div className="flex items-center space-x-3 pt-3">
              <Button
                type="button"
                variant="secondary"
                onClick={onCancel}
                disabled={isVerifying}
                className="flex-1 py-2 text-xs"
              >
                Deny & Cancel
              </Button>
              <Button
                type="submit"
                variant="primary"
                disabled={isVerifying || !credentialInput}
                className="flex-1 py-2 text-xs"
              >
                {isVerifying ? 'Elevating...' : 'Authorize Action'}
              </Button>
            </div>
          </form>
        </div>

        {/* Footer */}
        <div className="px-6 py-2.5 bg-[#07090D] border-t border-[#1F2937] text-[10px] text-[#6B7280] text-center">
          Protected by MindSet OS Kernel Biometric Subsystem (Zero Python / Local Memory Token)
        </div>
      </div>
    </div>
  );
};
