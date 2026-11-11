import React, { useState } from 'react';
import { HomeContinuityTab } from './tabs/HomeContinuityTab';
import { LaptopVaultTab } from './tabs/LaptopVaultTab';
import { ScannerNotesTab } from './tabs/ScannerNotesTab';
import { HardwareBridgeTab } from './tabs/HardwareBridgeTab';
import { TrustSyncTab } from './tabs/TrustSyncTab';
import {
  HomeIcon,
  FolderIcon,
  CameraScannerIcon,
  BridgeRemoteIcon,
  ShieldLockIcon,
} from './components/MobileIcons';

type TabId = 'home' | 'vault' | 'scanner' | 'bridge' | 'trust';

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabId>('home');

  return (
    <div className="flex flex-col h-screen w-screen bg-[#07090D] text-slate-100 overflow-hidden font-sans select-none">
      {/* Dynamic Content Region */}
      <div className="flex-1 overflow-y-auto">
        {activeTab === 'home' && <HomeContinuityTab />}
        {activeTab === 'vault' && <LaptopVaultTab />}
        {activeTab === 'scanner' && <ScannerNotesTab />}
        {activeTab === 'bridge' && <HardwareBridgeTab />}
        {activeTab === 'trust' && <TrustSyncTab />}
      </div>

      {/* 5-Tab Touch-First Bottom Navigation Bar */}
      <nav className="fixed bottom-0 inset-x-0 bg-slate-950/90 border-t border-slate-800/80 backdrop-blur-2xl z-40 px-2 py-1.5 flex items-center justify-around max-w-lg mx-auto shadow-2xl">
        <button
          onClick={() => setActiveTab('home')}
          className={`flex flex-col items-center py-1 px-2.5 rounded-xl transition ${
            activeTab === 'home' ? 'text-cyan-400 font-semibold' : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <HomeIcon className="w-5 h-5" />
          <span className="text-[10px] mt-1">Home</span>
        </button>

        <button
          onClick={() => setActiveTab('vault')}
          className={`flex flex-col items-center py-1 px-2.5 rounded-xl transition ${
            activeTab === 'vault' ? 'text-cyan-400 font-semibold' : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <FolderIcon className="w-5 h-5" />
          <span className="text-[10px] mt-1">Laptop</span>
        </button>

        {/* Center Prominent Scanner Button */}
        <button
          onClick={() => setActiveTab('scanner')}
          className="flex flex-col items-center -mt-5"
        >
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-tr from-cyan-600 to-blue-600 flex items-center justify-center text-white shadow-lg shadow-cyan-600/40 active:scale-95 transition">
            <CameraScannerIcon className="w-6 h-6" />
          </div>
          <span className="text-[10px] mt-1 font-semibold text-cyan-300">Scan</span>
        </button>

        <button
          onClick={() => setActiveTab('bridge')}
          className={`flex flex-col items-center py-1 px-2.5 rounded-xl transition ${
            activeTab === 'bridge' ? 'text-cyan-400 font-semibold' : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <BridgeRemoteIcon className="w-5 h-5" />
          <span className="text-[10px] mt-1">Bridge</span>
        </button>

        <button
          onClick={() => setActiveTab('trust')}
          className={`flex flex-col items-center py-1 px-2.5 rounded-xl transition ${
            activeTab === 'trust' ? 'text-cyan-400 font-semibold' : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <ShieldLockIcon className="w-5 h-5" />
          <span className="text-[10px] mt-1">Trust</span>
        </button>
      </nav>
    </div>
  );
};
