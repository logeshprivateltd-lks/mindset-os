import React, { useState } from 'react';
import {
  BridgeRemoteIcon,
  CameraScannerIcon,
} from '../components/MobileIcons';

export const HardwareBridgeTab: React.FC = () => {
  const [webcamStreaming, setWebcamStreaming] = useState(true);
  const [micStreaming, setMicStreaming] = useState(true);
  const [useFrontCamera, setUseFrontCamera] = useState(false);
  const [clickerStatus, setClickerStatus] = useState<string | null>(null);

  const handleSendClicker = (action: string) => {
    setClickerStatus(`Sent: ${action}`);
    setTimeout(() => setClickerStatus(null), 1500);
  };

  return (
    <div className="p-4 space-y-4 pb-24 overflow-y-auto max-w-lg mx-auto">
      {/* Header */}
      <div className="pt-2">
        <span className="text-[11px] font-semibold text-cyan-400 uppercase tracking-wider">
          Module 5: Hardware Bridge & Seminar Clicker
        </span>
        <h1 className="text-xl font-bold text-white tracking-tight">Phone as Laptop Hardware</h1>
        <p className="text-xs text-slate-400 mt-0.5">
          Stream phone sensors wirelessly to overcome hardware limits on older laptops
        </p>
      </div>

      {/* Wireless Webcam Switch */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2.5">
            <span className="p-2 rounded-xl bg-cyan-500/10 text-cyan-400">
              <CameraScannerIcon className="w-5 h-5" />
            </span>
            <div>
              <h3 className="text-sm font-bold text-white">Wireless Phone Webcam</h3>
              <p className="text-[11px] text-slate-400">Feeds MindSet Hello & video calls at 1080p</p>
            </div>
          </div>
          <button
            onClick={() => setWebcamStreaming(!webcamStreaming)}
            className={`px-3 py-1.5 rounded-xl text-xs font-bold transition ${
              webcamStreaming
                ? 'bg-emerald-500 text-slate-950 shadow-md shadow-emerald-500/20'
                : 'bg-slate-800 text-slate-400'
            }`}
          >
            {webcamStreaming ? 'Streaming' : 'Off'}
          </button>
        </div>

        {webcamStreaming && (
          <div className="mt-3 pt-3 border-t border-slate-800/80 flex items-center justify-between text-xs text-slate-400">
            <button
              onClick={() => setUseFrontCamera(!useFrontCamera)}
              className="px-2.5 py-1 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-[11px]"
            >
              Switch to {useFrontCamera ? 'Back Camera (50MP)' : 'Front Camera (12MP)'}
            </button>
            <span className="text-emerald-400 font-mono font-semibold tabular-nums">
              1080p • 18ms latency
            </span>
          </div>
        )}
      </div>

      {/* Wireless Microphone Switch */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2.5">
            <span className="p-2 rounded-xl bg-indigo-500/10 text-indigo-400">
              <BridgeRemoteIcon className="w-5 h-5" />
            </span>
            <div>
              <h3 className="text-sm font-bold text-white">Whisper AI Microphone</h3>
              <p className="text-[11px] text-slate-400">16kHz audio stream for voice typing</p>
            </div>
          </div>
          <button
            onClick={() => setMicStreaming(!micStreaming)}
            className={`px-3 py-1.5 rounded-xl text-xs font-bold transition ${
              micStreaming
                ? 'bg-emerald-500 text-slate-950 shadow-md shadow-emerald-500/20'
                : 'bg-slate-800 text-slate-400'
            }`}
          >
            {micStreaming ? 'Active' : 'Muted'}
          </button>
        </div>

        {micStreaming && (
          <div className="mt-3 pt-3 border-t border-slate-800/80 flex items-center justify-between text-xs text-slate-400">
            <span>Noise Cancellation: ON</span>
            <span className="text-cyan-400 font-mono font-semibold tabular-nums">
              Level: -24.5 dB
            </span>
          </div>
        )}
      </div>

      {/* Classroom Seminar Clicker */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between pb-3 border-b border-slate-800/80">
          <div>
            <h3 className="text-sm font-bold text-white">Classroom Seminar Clicker</h3>
            <p className="text-[11px] text-slate-400">Control laptop slides from your phone during presentations</p>
          </div>
          {clickerStatus && (
            <span className="text-[10px] text-cyan-400 font-semibold font-mono animate-pulse">
              {clickerStatus}
            </span>
          )}
        </div>

        <div className="mt-4 grid grid-cols-2 gap-3">
          <button
            onClick={() => handleSendClicker('Previous Slide')}
            className="py-4 rounded-2xl bg-slate-950 hover:bg-slate-800 border border-slate-800 active:scale-95 text-slate-200 text-sm font-bold transition flex flex-col items-center justify-center space-y-1"
          >
            <span className="text-xl">◀</span>
            <span className="text-xs">Prev Slide</span>
          </button>
          <button
            onClick={() => handleSendClicker('Next Slide')}
            className="py-4 rounded-2xl bg-cyan-600/30 hover:bg-cyan-600/40 border border-cyan-500/40 active:scale-95 text-cyan-200 text-sm font-bold transition flex flex-col items-center justify-center space-y-1"
          >
            <span className="text-xl">▶</span>
            <span className="text-xs">Next Slide</span>
          </button>
        </div>

        <div className="mt-3 grid grid-cols-2 gap-2 text-xs">
          <button
            onClick={() => handleSendClicker('Laser Pointer Active')}
            className="py-2.5 rounded-xl bg-slate-950 border border-slate-800 text-slate-300 font-semibold"
          >
            🔴 Virtual Laser
          </button>
          <button
            onClick={() => handleSendClicker('Blank Screen')}
            className="py-2.5 rounded-xl bg-slate-950 border border-slate-800 text-slate-300 font-semibold"
          >
            ⬛ Blank Screen
          </button>
        </div>
      </div>
    </div>
  );
};
