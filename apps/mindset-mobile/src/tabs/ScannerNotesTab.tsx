import React, { useState } from 'react';
import { CameraScannerIcon, CheckCircleIcon } from '../components/MobileIcons';

export const ScannerNotesTab: React.FC = () => {
  const [subject, setSubject] = useState<'Physics' | 'Chemistry' | 'Mathematics' | 'General'>('Physics');
  const [docTitle, setDocTitle] = useState('');
  const [isScanning, setIsScanning] = useState(false);
  const [scanSuccess, setScanSuccess] = useState(false);
  const [recentScans, setRecentScans] = useState([
    {
      id: 'scn-01',
      title: 'Optics Double Slit Blackboard Derivation',
      subject: 'Physics',
      pages: 2,
      time: '15m ago',
    },
    {
      id: 'scn-02',
      title: 'Aldol Condensation Hand-drawn Mechanism',
      subject: 'Chemistry',
      pages: 1,
      time: '2h ago',
    },
  ]);

  const handleCaptureAndBeam = () => {
    if (!docTitle.trim()) {
      alert('Please enter a note or textbook title before scanning.');
      return;
    }

    setIsScanning(true);
    setTimeout(() => {
      setIsScanning(false);
      setScanSuccess(true);
      const newScan = {
        id: `scn-${Date.now().toString().slice(-4)}`,
        title: docTitle,
        subject,
        pages: 1,
        time: 'Just now',
      };
      setRecentScans([newScan, ...recentScans]);
      setDocTitle('');

      setTimeout(() => setScanSuccess(false), 3000);
    }, 1500);
  };

  return (
    <div className="p-4 space-y-4 pb-24 overflow-y-auto max-w-lg mx-auto">
      {/* Header */}
      <div className="pt-2">
        <span className="text-[11px] font-semibold text-cyan-400 uppercase tracking-wider">
          Module 5: Mobile Document Scanner
        </span>
        <h1 className="text-xl font-bold text-white tracking-tight">Scanner & RAG Beam</h1>
        <p className="text-xs text-slate-400 mt-0.5">
          Scan blackboard or textbook pages; auto-deskew and beam directly to Laptop RAG
        </p>
      </div>

      {/* Camera Capture Card */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl space-y-3">
        <div>
          <label className="text-xs font-semibold text-slate-300">Document Title</label>
          <input
            type="text"
            value={docTitle}
            onChange={(e) => setDocTitle(e.target.value)}
            placeholder="e.g. Wave Optics Young's Slit Blackboard..."
            className="w-full mt-1 px-3.5 py-2.5 rounded-xl bg-slate-950 border border-slate-800 text-xs text-slate-200 focus:outline-none focus:border-cyan-500"
          />
        </div>

        <div>
          <label className="text-xs font-semibold text-slate-300">Subject Tag</label>
          <div className="mt-1 grid grid-cols-4 gap-1.5 text-xs">
            {(['Physics', 'Chemistry', 'Mathematics', 'General'] as const).map((s) => (
              <button
                key={s}
                onClick={() => setSubject(s)}
                className={`py-1.5 rounded-lg font-medium transition text-center ${
                  subject === s ? 'bg-cyan-600 text-white' : 'bg-slate-800 text-slate-400 hover:text-white'
                }`}
              >
                {s}
              </button>
            ))}
          </div>
        </div>

        <div className="pt-2">
          <button
            onClick={handleCaptureAndBeam}
            disabled={isScanning}
            className="w-full py-3 rounded-xl bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white text-xs font-bold shadow-lg shadow-cyan-600/30 flex items-center justify-center space-x-2 transition disabled:opacity-50"
          >
            <CameraScannerIcon className="w-5 h-5" />
            <span>{isScanning ? 'Auto-Deskewing & Beaming...' : 'Snap & Beam to Laptop RAG'}</span>
          </button>
        </div>

        {scanSuccess && (
          <div className="p-3 rounded-xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 text-xs font-medium flex items-center space-x-2">
            <CheckCircleIcon className="w-4 h-4 text-emerald-400" />
            <span>Beamed to Laptop RAG Pipeline! Indexed into zero-RAM memory.</span>
          </div>
        )}
      </div>

      {/* Inbound Scans History */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <h3 className="text-sm font-bold text-white pb-2 border-b border-slate-800">
          Recently Beamed Scans
        </h3>

        <div className="mt-3 space-y-2">
          {recentScans.map((scan) => (
            <div
              key={scan.id}
              className="p-3 rounded-xl bg-slate-950/80 border border-slate-800 flex items-center justify-between"
            >
              <div>
                <h4 className="text-xs font-semibold text-slate-200">{scan.title}</h4>
                <p className="text-[10px] text-slate-400 mt-0.5">
                  <span className="text-cyan-400 font-medium">{scan.subject}</span> • {scan.pages} page • {scan.time}
                </p>
              </div>
              <span className="text-[10px] px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 font-medium">
                Indexed in RAG
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
