/**
 * MindSet OS — Distraction-Free MindSet App Hub
 * Curates strictly educational, scientific, coding, and productivity tools.
 * STRICT RULE: Zero games, zero social media, zero algorithmic feeds.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useMemo } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';

export type AppCategory = 'all' | 'math_science' | 'coding' | 'productivity' | 'creative';

export interface HubApp {
  id: string;
  name: string;
  category: AppCategory;
  categoryLabel: string;
  icon: string;
  version: string;
  sizeMb: number;
  description: string;
  isInstalled: boolean;
  hasUpdate: boolean;
  permissions: {
    camera: boolean;
    microphone: boolean;
    network: boolean;
    vaultAccess: boolean;
    autoStart: boolean;
  };
}

const CURATED_APPS: HubApp[] = [
  // 1. Math & Science
  {
    id: 'geogebra',
    name: 'GeoGebra Math Suite',
    category: 'math_science',
    categoryLabel: 'Math & Calculus',
    icon: '📐',
    version: 'v6.0.8',
    sizeMb: 85,
    description: 'Dynamic graphing, 3D geometry, algebra, and calculus visualization for State Board & JEE/NEET.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'stellarium',
    name: 'Stellarium Planetarium',
    category: 'math_science',
    categoryLabel: 'Astronomy',
    icon: '🔭',
    version: 'v24.3',
    sizeMb: 240,
    description: 'Realistic 3D night sky simulator showing planetary motions, constellations, and gravitational paths.',
    isInstalled: true,
    hasUpdate: true,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: false, autoStart: false },
  },
  {
    id: 'scilab',
    name: 'Scilab Numerical Lab',
    category: 'math_science',
    categoryLabel: 'Physics / Numerical',
    icon: '🧮',
    version: 'v2024.1',
    sizeMb: 180,
    description: 'Matrix computations, numerical differential equations, and scientific signal processing algorithms.',
    isInstalled: false,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'avogadro',
    name: 'Avogadro 3D Molecular Editor',
    category: 'math_science',
    categoryLabel: 'Chemistry',
    icon: '🧪',
    version: 'v1.99',
    sizeMb: 95,
    description: 'Interactive molecular builder and chemical crystallography simulator for organic & inorganic chemistry.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },

  // 2. Coding & Tech
  {
    id: 'vscodium',
    name: 'VSCodium (Telemetry-Free)',
    category: 'coding',
    categoryLabel: 'Code Editor',
    icon: '💻',
    version: 'v1.96',
    sizeMb: 110,
    description: 'Open-source, privacy-first code editor supporting Python, Rust, C++, and Web technologies.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: true, vaultAccess: true, autoStart: false },
  },
  {
    id: 'arduino',
    name: 'Arduino IDE & Robotics',
    category: 'coding',
    categoryLabel: 'Embedded Systems',
    icon: '🤖',
    version: 'v2.3.2',
    sizeMb: 160,
    description: 'Microcontroller programming environment for hardware robotics, sensors, and IoT projects.',
    isInstalled: false,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: true, vaultAccess: true, autoStart: false },
  },
  {
    id: 'sqlite_browser',
    name: 'DB Browser for SQLite',
    category: 'coding',
    categoryLabel: 'Database',
    icon: '🗄️',
    version: 'v3.13',
    sizeMb: 45,
    description: 'Visual database inspector for exploring and executing SQL queries on local educational databases.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },

  // 3. Productivity & Notes
  {
    id: 'libreoffice',
    name: 'LibreOffice Office Suite',
    category: 'productivity',
    categoryLabel: 'Documents & Calc',
    icon: '📄',
    version: 'v24.8',
    sizeMb: 320,
    description: 'Distraction-free document writing, spreadsheets, and presentation decks with zero cloud dependency.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'obsidian',
    name: 'Obsidian Knowledge Vault',
    category: 'productivity',
    categoryLabel: 'Personal Wiki',
    icon: '💎',
    version: 'v1.7.7',
    sizeMb: 90,
    description: 'Second-brain bidirectional Markdown link engine for structuring comprehensive subject notes.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'anki',
    name: 'Anki Spaced Repetition',
    category: 'productivity',
    categoryLabel: 'Flashcards',
    icon: '🗂️',
    version: 'v24.06',
    sizeMb: 75,
    description: 'Scientifically proven active-recall flashcard system to lock Board exam definitions into memory.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: true, vaultAccess: true, autoStart: false },
  },
  {
    id: 'xournal',
    name: 'Xournal++ Stylus Notebook',
    category: 'productivity',
    categoryLabel: 'Handwritten Notes',
    icon: '✏️',
    version: 'v1.2.3',
    sizeMb: 55,
    description: 'Handwritten mathematical derivation and stylus PDF markup tool with pressure sensitivity.',
    isInstalled: false,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },

  // 4. Creative Tools
  {
    id: 'vlc',
    name: 'VLC Media Player',
    category: 'creative',
    categoryLabel: 'Video / Audio',
    icon: '🎬',
    version: 'v3.0.21',
    sizeMb: 60,
    description: 'Clean offline video playback with zero algorithmic recommendations, ads, or comments.',
    isInstalled: true,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'gimp',
    name: 'GIMP Image Editor',
    category: 'creative',
    categoryLabel: 'Design & Art',
    icon: '🎨',
    version: 'v2.10',
    sizeMb: 210,
    description: 'Open-source image manipulation and graphic diagram editor for scientific illustrations.',
    isInstalled: false,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
  {
    id: 'blender',
    name: 'Blender 3D Suite',
    category: 'creative',
    categoryLabel: '3D Simulation',
    icon: '🧊',
    version: 'v4.3',
    sizeMb: 340,
    description: '3D modeling, physics simulation, and anatomical/mechanical rendering engine.',
    isInstalled: false,
    hasUpdate: false,
    permissions: { camera: false, microphone: false, network: false, vaultAccess: true, autoStart: false },
  },
];

export const AppHubPage: React.FC = () => {
  const [apps, setApps] = useState<HubApp[]>(CURATED_APPS);
  const [activeCategory, setActiveCategory] = useState<AppCategory>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedAppForPerms, setSelectedAppForPerms] = useState<HubApp | null>(null);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const handleToggleInstall = (id: string) => {
    setApps((prev) =>
      prev.map((a) => {
        if (a.id === id) {
          const nextState = !a.isInstalled;
          showToast(nextState ? `Installed ${a.name} safely!` : `Uninstalled ${a.name}`);
          return { ...a, isInstalled: nextState, hasUpdate: false };
        }
        return a;
      })
    );
  };

  const handleUpdate = (id: string) => {
    setApps((prev) =>
      prev.map((a) => (a.id === id ? { ...a, hasUpdate: false } : a))
    );
    showToast('Updated to latest version!');
  };

  const handleTogglePerm = (perm: keyof HubApp['permissions']) => {
    if (!selectedAppForPerms) return;
    const appId = selectedAppForPerms.id;
    const updatedPerms = {
      ...selectedAppForPerms.permissions,
      [perm]: !selectedAppForPerms.permissions[perm],
    };

    setApps((prev) =>
      prev.map((a) => (a.id === appId ? { ...a, permissions: updatedPerms } : a))
    );
    setSelectedAppForPerms((prev) => (prev ? { ...prev, permissions: updatedPerms } : null));
    showToast(`Updated permissions for ${selectedAppForPerms.name}`);
  };

  const filteredApps = useMemo(() => {
    return apps.filter((a) => {
      const matchesCat = activeCategory === 'all' || a.category === activeCategory;
      const matchesQuery =
        a.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        a.description.toLowerCase().includes(searchQuery.toLowerCase());
      return matchesCat && matchesQuery;
    });
  }, [apps, activeCategory, searchQuery]);

  return (
    <div className="h-full flex flex-col p-6 space-y-6 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] overflow-y-auto">
      {/* Top Header & Anti-Distraction Charter */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-[var(--border-subtle)] gap-4">
        <div>
          <div className="flex items-center space-x-2">
            <span className="text-xl">📦</span>
            <h1 className="text-lg font-bold">MindSet App Hub</h1>
            <span className="text-[10px] px-2.5 py-0.5 rounded-full bg-[#65E6C4]/15 text-[#65E6C4] font-semibold border border-[#65E6C4]/30">
              Verified Educational & Sovereign
            </span>
          </div>
          <p className="text-xs text-[var(--text-muted)] mt-1">
            Strictly curated tools for study, science, and coding • Zero games • Zero algorithmic social feeds
          </p>
        </div>

        {/* Search input */}
        <div className="w-full sm:w-64">
          <input
            type="text"
            placeholder="Search verified tools..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full text-xs px-3.5 py-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
          />
        </div>
      </div>

      {/* Toast Notification */}
      {toastMessage && (
        <div className="p-3 rounded-xl bg-[#65E6C4]/15 border border-[#65E6C4]/40 text-xs text-[#65E6C4] font-medium flex items-center justify-between animate-fadeIn">
          <span>{toastMessage}</span>
          <button type="button" onClick={() => setToastMessage(null)} className="text-xs font-bold hover:underline">
            Dismiss
          </button>
        </div>
      )}

      {/* Category Filter Tabs */}
      <div className="flex items-center space-x-2 overflow-x-auto pb-1 text-xs">
        {[
          { id: 'all', label: 'All Curated Tools' },
          { id: 'math_science', label: '📐 Math & Science' },
          { id: 'coding', label: '💻 Coding & Tech' },
          { id: 'productivity', label: '📄 Productivity & Notes' },
          { id: 'creative', label: '🎨 Creative Study' },
        ].map((tab) => (
          <button
            key={tab.id}
            type="button"
            onClick={() => setActiveCategory(tab.id as AppCategory)}
            className={`px-3 py-1.5 rounded-xl font-semibold whitespace-nowrap transition-all ${
              activeCategory === tab.id
                ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)] shadow-sm'
                : 'bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Apps Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredApps.map((app) => (
          <GlassSurface
            key={app.id}
            variant="solid-card"
            className="p-5 rounded-2xl flex flex-col justify-between space-y-4 hover:border-[var(--accent-primary)] transition-all"
          >
            <div className="space-y-2.5">
              <div className="flex items-start justify-between">
                <div className="flex items-center space-x-3">
                  <span className="text-3xl p-2 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]">
                    {app.icon}
                  </span>
                  <div>
                    <h3 className="text-sm font-bold text-[var(--text-dominant)]">{app.name}</h3>
                    <span className="text-[10px] font-mono text-[var(--text-muted)]">
                      {app.categoryLabel} • {app.version}
                    </span>
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => setSelectedAppForPerms(app)}
                  className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1.5 rounded-lg hover:bg-[var(--surface-secondary)]"
                  title="Configure Process Permissions"
                >
                  🔒
                </button>
              </div>

              <p className="text-xs text-[var(--text-muted)] leading-relaxed">{app.description}</p>
            </div>

            <div className="pt-3 border-t border-[var(--border-subtle)]/60 flex items-center justify-between text-xs">
              <span className="text-[11px] font-mono text-[var(--text-muted)]">
                Size: {app.sizeMb} MB
              </span>

              <div className="flex items-center space-x-2">
                {app.hasUpdate && (
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={() => handleUpdate(app.id)}
                    className="text-xs font-semibold px-2.5 bg-[#FFD078]/15 text-[#FFD078] border-[#FFD078]/40"
                  >
                    Update
                  </Button>
                )}

                <Button
                  variant={app.isInstalled ? 'secondary' : 'primary'}
                  size="sm"
                  onClick={() => handleToggleInstall(app.id)}
                  className="text-xs font-semibold px-4"
                >
                  {app.isInstalled ? 'Uninstall' : 'Install'}
                </Button>
              </div>
            </div>
          </GlassSurface>
        ))}
      </div>

      {/* ============================================================= */}
      {/* PROCESS PERMISSION MODAL                                      */}
      {/* ============================================================= */}
      {selectedAppForPerms && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xl animate-fadeIn">
          <GlassSurface variant="modal" className="max-w-md w-full p-6 space-y-4 animate-scaleUp">
            <div className="flex items-center justify-between pb-2 border-b border-[var(--border-subtle)]">
              <div className="flex items-center space-x-2">
                <span className="text-2xl">{selectedAppForPerms.icon}</span>
                <div>
                  <h4 className="text-sm font-bold">{selectedAppForPerms.name}</h4>
                  <p className="text-[10px] text-[var(--text-muted)]">Security & Sandbox Permissions</p>
                </div>
              </div>
              <button
                type="button"
                onClick={() => setSelectedAppForPerms(null)}
                className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1 rounded-lg"
              >
                ✕
              </button>
            </div>

            <div className="space-y-2.5 text-xs">
              {[
                { key: 'vaultAccess' as const, label: 'File Vault Storage Access', icon: '🗂️', desc: 'Read/write study files' },
                { key: 'network' as const, label: 'Outbound Network Access', icon: '🌐', desc: 'Package and plugin updates' },
                { key: 'microphone' as const, label: 'Microphone Hardware Access', icon: '🎙️', desc: 'Audio recording features' },
                { key: 'camera' as const, label: 'Webcam Hardware Access', icon: '📷', desc: 'Video and visual analysis' },
                { key: 'autoStart' as const, label: 'Launch on System Startup', icon: '⚡', desc: 'Runs in background on boot' },
              ].map((p) => (
                <div
                  key={p.key}
                  className="flex items-center justify-between p-2.5 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)]"
                >
                  <div className="flex items-center space-x-2">
                    <span>{p.icon}</span>
                    <div>
                      <div className="font-semibold">{p.label}</div>
                      <div className="text-[9px] text-[var(--text-muted)]">{p.desc}</div>
                    </div>
                  </div>
                  <input
                    type="checkbox"
                    checked={selectedAppForPerms.permissions[p.key]}
                    onChange={() => handleTogglePerm(p.key)}
                    className="w-4 h-4 accent-[#65E6C4] cursor-pointer"
                  />
                </div>
              ))}
            </div>

            <div className="pt-2">
              <Button
                variant="primary"
                size="sm"
                onClick={() => setSelectedAppForPerms(null)}
                className="w-full text-xs font-bold py-2"
              >
                Done
              </Button>
            </div>
          </GlassSurface>
        </div>
      )}
    </div>
  );
};

