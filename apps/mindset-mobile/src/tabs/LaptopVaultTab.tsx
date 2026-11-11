import React, { useState } from 'react';
import { FolderIcon, ChevronRightIcon } from '../components/MobileIcons';

interface RemoteFile {
  name: string;
  size: string;
  type: 'pdf' | 'doc' | 'scan';
  lastPage?: number;
  preview: string;
}

export const LaptopVaultTab: React.FC = () => {
  const [activeFolder, setActiveFolder] = useState<'laptop' | 'textbooks'>('laptop');
  const [selectedFile, setSelectedFile] = useState<RemoteFile | null>(null);

  const files: RemoteFile[] = [
    {
      name: 'Class 12 Physics Complete Reference.pdf',
      size: '24.5 MB',
      type: 'pdf',
      lastPage: 84,
      preview: 'MindSet Lightweight 64KB Preview:\nChapter 4: Ray Optics and Optical Instruments\n- Reflection from spherical mirrors\n- Refraction at spherical surfaces & lenses\n- Total Internal Reflection and optical fibres...',
    },
    {
      name: 'Chemistry Organic Mechanisms Revision.pdf',
      size: '12.4 MB',
      type: 'pdf',
      lastPage: 29,
      preview: 'MindSet Lightweight 64KB Preview:\nUnit 12: Aldehydes, Ketones and Carboxylic Acids\n- Nucleophilic addition reactions (HCN, NaHSO3)\n- Cannizzaro Reaction mechanism\n- Aldol condensation step-by-step...',
    },
    {
      name: 'Maths Master Formulae & Integrals.md',
      size: '48 KB',
      type: 'doc',
      preview: '# Mathematics HSC Master Formulae\n1. Integration by Parts: int(u dv) = uv - int(v du)\n2. Bernoulli formula for repeated polynomials\n3. Definite Integral properties (King\'s rule)...',
    },
    {
      name: 'scan_blackboard_optics_derivation.png',
      size: '1.2 MB',
      type: 'scan',
      preview: '[High-Resolution Scanned Blackboard Image]\nSubject: Physics\nCaptured via Mobile Camera Bridge\nContrast: Boosted for chalk text legibility.',
    },
  ];

  return (
    <div className="p-4 space-y-4 pb-24 overflow-y-auto max-w-lg mx-auto">
      {/* Header */}
      <div className="pt-2">
        <span className="text-[11px] font-semibold text-cyan-400 uppercase tracking-wider">
          Module 3: Remote Access
        </span>
        <h1 className="text-xl font-bold text-white tracking-tight">Laptop Vault & Textbooks</h1>
        <p className="text-xs text-slate-400 mt-0.5">
          Browse permitted laptop files on your phone with instant 64KB preview
        </p>
      </div>

      {/* Root Switcher */}
      <div className="flex p-1 rounded-xl bg-slate-900 border border-slate-800">
        <button
          onClick={() => setActiveFolder('laptop')}
          className={`flex-1 py-1.5 rounded-lg text-xs font-semibold transition ${
            activeFolder === 'laptop' ? 'bg-cyan-600 text-white shadow' : 'text-slate-400 hover:text-white'
          }`}
        >
          My Laptop Vault
        </button>
        <button
          onClick={() => setActiveFolder('textbooks')}
          className={`flex-1 py-1.5 rounded-lg text-xs font-semibold transition ${
            activeFolder === 'textbooks' ? 'bg-cyan-600 text-white shadow' : 'text-slate-400 hover:text-white'
          }`}
        >
          Study Textbooks
        </button>
      </div>

      {/* File List */}
      <div className="space-y-2.5">
        {files.map((file) => (
          <div
            key={file.name}
            onClick={() => setSelectedFile(file)}
            className="p-3.5 rounded-2xl bg-slate-900/80 border border-slate-800/80 hover:border-slate-700 transition cursor-pointer flex items-center justify-between"
          >
            <div className="flex items-center space-x-3 overflow-hidden">
              <span className="p-2.5 rounded-xl bg-slate-800 text-cyan-400 shrink-0">
                <FolderIcon className="w-5 h-5" />
              </span>
              <div className="overflow-hidden">
                <h4 className="text-xs font-semibold text-slate-100 truncate">{file.name}</h4>
                <p className="text-[11px] text-slate-400 mt-0.5 flex items-center gap-2">
                  <span className="tabular-nums">{file.size}</span>
                  {file.lastPage && (
                    <span className="text-cyan-400 font-medium">Page {file.lastPage}</span>
                  )}
                </p>
              </div>
            </div>
            <ChevronRightIcon className="w-4 h-4 text-slate-500 shrink-0" />
          </div>
        ))}
      </div>

      {/* 64KB Fast Preview Modal */}
      {selectedFile && (
        <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex items-center justify-center p-4">
          <div className="bg-slate-900 border border-slate-800 rounded-3xl p-5 max-w-sm w-full shadow-2xl space-y-3">
            <div className="flex items-center justify-between pb-2 border-b border-slate-800">
              <span className="text-xs font-semibold text-cyan-400 uppercase tracking-wider">
                Instant 64KB Preview
              </span>
              <button
                onClick={() => setSelectedFile(null)}
                className="text-slate-400 hover:text-white p-1"
              >
                ✕
              </button>
            </div>

            <h3 className="text-sm font-bold text-white leading-snug">{selectedFile.name}</h3>
            <p className="text-[11px] text-slate-400">
              Zero battery drain preview loaded without downloading full {selectedFile.size} payload.
            </p>

            <div className="p-3 rounded-xl bg-slate-950 border border-slate-800/80 text-xs font-mono text-slate-300 max-h-48 overflow-y-auto whitespace-pre-wrap">
              {selectedFile.preview}
            </div>

            <div className="pt-2 flex gap-2">
              <button
                onClick={() => setSelectedFile(null)}
                className="flex-1 py-2 rounded-xl bg-slate-800 text-slate-300 text-xs font-semibold"
              >
                Close Preview
              </button>
              <button
                onClick={() => {
                  alert(`Beaming "${selectedFile.name}" to Laptop active workspace.`);
                  setSelectedFile(null);
                }}
                className="flex-1 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold"
              >
                Open on Laptop
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
