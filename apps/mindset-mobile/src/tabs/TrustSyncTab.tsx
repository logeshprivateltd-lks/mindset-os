import React, { useState } from 'react';
import {
  QrCodeIcon,
  CheckCircleIcon,
} from '../components/MobileIcons';

export const TrustSyncTab: React.FC = () => {
  const [syncPaused, setSyncPaused] = useState(false);
  const [clipboardPrivacy, setClipboardPrivacy] = useState(true);
  const [isScanningQr, setIsScanningQr] = useState(false);
  const [pairSuccess, setPairSuccess] = useState(false);

  const handleScanLaptopQr = () => {
    setIsScanningQr(true);
    setTimeout(() => {
      setIsScanningQr(false);
      setPairSuccess(true);
      setTimeout(() => setPairSuccess(false), 3000);
    }, 1500);
  };

  return (
    <div className="p-4 space-y-4 pb-24 overflow-y-auto max-w-lg mx-auto">
      {/* Header */}
      <div className="pt-2">
        <span className="text-[11px] font-semibold text-cyan-400 uppercase tracking-wider">
          Module 1 & 6: Identity & Security
        </span>
        <h1 className="text-xl font-bold text-white tracking-tight">Trust & Cryptographic Sync</h1>
        <p className="text-xs text-slate-400 mt-0.5">
          Ed25519 peer pairing, pause sync, and clipboard privacy filter
        </p>
      </div>

      {/* QR Pairing Action */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between pb-3 border-b border-slate-800/80">
          <div className="flex items-center space-x-2.5">
            <span className="p-2 rounded-xl bg-cyan-500/10 text-cyan-400">
              <QrCodeIcon className="w-5 h-5" />
            </span>
            <div>
              <h3 className="text-sm font-bold text-white">Scan Laptop QR Code</h3>
              <p className="text-[11px] text-slate-400">Establish Ed25519 trusted handshake</p>
            </div>
          </div>
        </div>

        <div className="mt-3">
          <button
            onClick={handleScanLaptopQr}
            disabled={isScanningQr}
            className="w-full py-2.5 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-bold transition flex items-center justify-center space-x-2 disabled:opacity-50"
          >
            <QrCodeIcon className="w-4 h-4" />
            <span>{isScanningQr ? 'Verifying Ed25519 Challenge...' : 'Open QR Scanner Camera'}</span>
          </button>
        </div>

        {pairSuccess && (
          <div className="mt-3 p-3 rounded-xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 text-xs font-medium flex items-center space-x-2">
            <CheckCircleIcon className="w-4 h-4 text-emerald-400" />
            <span>Laptop Handshake verified and session established!</span>
          </div>
        )}
      </div>

      {/* Sync Preferences & Privacy Guard */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl space-y-4">
        <h3 className="text-sm font-bold text-white pb-2 border-b border-slate-800">
          Sync & Security Controls
        </h3>

        {/* Pause Sync */}
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-xs font-semibold text-slate-200">Pause Cross-Device Sync</h4>
            <p className="text-[11px] text-slate-400">Temporarily stop exchanging oplog files</p>
          </div>
          <button
            onClick={() => setSyncPaused(!syncPaused)}
            className={`w-12 h-6 rounded-full transition-colors relative p-0.5 ${
              syncPaused ? 'bg-amber-500' : 'bg-slate-700'
            }`}
          >
            <div
              className={`w-5 h-5 rounded-full bg-white transition-transform ${
                syncPaused ? 'translate-x-6' : 'translate-x-0'
              }`}
            />
          </button>
        </div>

        {/* Rule 2: Clipboard Privacy Guard */}
        <div className="flex items-center justify-between pt-2 border-t border-slate-800/80">
          <div>
            <h4 className="text-xs font-semibold text-slate-200 flex items-center gap-1.5">
              <span>Rule 2: Clipboard Privacy Guard</span>
              <span className="text-[10px] px-1.5 py-0.2 rounded bg-emerald-500/20 text-emerald-300 font-mono">
                Active
              </span>
            </h4>
            <p className="text-[11px] text-slate-400">
              Never broadcasts OTPs, passwords, or authentication tokens
            </p>
          </div>
          <button
            onClick={() => setClipboardPrivacy(!clipboardPrivacy)}
            className={`w-12 h-6 rounded-full transition-colors relative p-0.5 ${
              clipboardPrivacy ? 'bg-cyan-500' : 'bg-slate-700'
            }`}
          >
            <div
              className={`w-5 h-5 rounded-full bg-white transition-transform ${
                clipboardPrivacy ? 'translate-x-6' : 'translate-x-0'
              }`}
            />
          </button>
        </div>
      </div>

      {/* Device Info */}
      <div className="p-4 rounded-2xl bg-slate-950/80 border border-slate-800/80 text-xs text-slate-400 space-y-1 font-mono">
        <div className="flex justify-between">
          <span>Companion Device:</span>
          <span className="text-slate-200">Galaxy S24 (SM-S921B)</span>
        </div>
        <div className="flex justify-between">
          <span>Public Key:</span>
          <span className="text-cyan-400 font-mono">ed25519:pub:phone:9f42b38c...</span>
        </div>
        <div className="flex justify-between">
          <span>Active Mesh IP:</span>
          <span className="text-slate-200">192.168.1.112 (Wi-Fi)</span>
        </div>
      </div>
    </div>
  );
};
