/**
 * MindSet OS Personalization Studio — Taskbar Dock & Desktop Shortcuts Panel
 * Dock placement, floating vs edge style, autohide, and desktop shortcuts manager
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { usePersonalizationStore } from '../../stores/usePersonalizationStore';

export const DockSettingsPanel: React.FC = () => {
  const {
    dockPosition,
    dockAutohide,
    dockStyle,
    shortcuts,
    setDockPosition,
    setDockAutohide,
    setDockStyle,
    pinShortcut,
    deleteShortcut,
  } = usePersonalizationStore();

  const [newTitle, setNewTitle] = useState('');
  const [newIcon, setNewIcon] = useState('📦');
  const [newTarget, setNewTarget] = useState('dashboard');
  const [isAdding, setIsAdding] = useState(false);

  const handleAddShortcut = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;

    await pinShortcut({
      id: `sc_custom_${Date.now()}`,
      title: newTitle.trim(),
      icon: newIcon || '📦',
      target_type: 'app',
      target_payload: newTarget,
      is_pinned: true,
    });

    setNewTitle('');
    setIsAdding(false);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h4 className="text-sm font-bold text-[var(--text-dominant)]">
          Taskbar Dock & Desktop Shortcut Grid
        </h4>
        <p className="text-xs text-[var(--text-muted)]">
          Customize dock positioning, liquid glass translucency, and manage pinned desktop shortcuts.
        </p>
      </div>

      {/* Dock Positioning & Style */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        {/* Position */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <label className="text-xs font-bold text-[var(--text-dominant)] block">
            Screen Position:
          </label>
          <div className="flex items-center space-x-1.5">
            {(['bottom', 'left', 'top'] as const).map((pos) => (
              <button
                key={pos}
                type="button"
                onClick={() => setDockPosition(pos)}
                className={`flex-1 py-1.5 rounded-lg text-xs font-semibold capitalize transition-all ${
                  dockPosition === pos
                    ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                    : 'bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {pos}
              </button>
            ))}
          </div>
        </div>

        {/* Style */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <label className="text-xs font-bold text-[var(--text-dominant)] block">
            Dock Aesthetic Style:
          </label>
          <div className="flex items-center space-x-1.5">
            {(['floating', 'edge'] as const).map((st) => (
              <button
                key={st}
                type="button"
                onClick={() => setDockStyle(st)}
                className={`flex-1 py-1.5 rounded-lg text-xs font-semibold capitalize transition-all ${
                  dockStyle === st
                    ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                    : 'bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {st}
              </button>
            ))}
          </div>
        </div>

        {/* Auto-hide */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex items-center justify-between">
          <div>
            <div className="text-xs font-bold text-[var(--text-dominant)]">Smart Auto-Hide</div>
            <div className="text-[10px] text-[var(--text-muted)]">Hide when windows maximize</div>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={dockAutohide}
            onClick={() => setDockAutohide(!dockAutohide)}
            className={`w-11 h-6 rounded-full transition-colors relative flex-shrink-0 ${
              dockAutohide ? 'bg-[var(--accent-primary)]' : 'bg-slate-700'
            }`}
          >
            <span
              className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow-md transform transition-transform ${
                dockAutohide ? 'translate-x-5' : 'translate-x-0'
              }`}
            />
          </button>
        </div>
      </div>

      {/* Pinned Desktop Shortcuts Manager */}
      <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-3">
        <div className="flex items-center justify-between">
          <div>
            <div className="text-xs font-bold text-[var(--text-dominant)]">
              Pinned Desktop Shortcuts ({shortcuts.length})
            </div>
            <div className="text-[10px] text-[var(--text-muted)]">
              Interactive 88x96px grid shortcuts with drag-to-reposition coordinate persistence
            </div>
          </div>
          <button
            type="button"
            onClick={() => setIsAdding(!isAdding)}
            className="px-3 py-1.5 rounded-xl border border-[var(--accent-primary)] bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] text-xs font-semibold hover:bg-[var(--accent-primary)]/20 transition-all"
          >
            {isAdding ? 'Cancel' : '+ New Shortcut'}
          </button>
        </div>

        {/* New Shortcut Form */}
        {isAdding && (
          <form
            onSubmit={handleAddShortcut}
            className="p-3 rounded-xl border border-[var(--accent-primary)]/30 bg-[var(--surface-elevated)] space-y-2.5 animate-fadeIn"
          >
            <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
              <input
                type="text"
                placeholder="Shortcut Name (e.g. Chemistry Notes)"
                value={newTitle}
                onChange={(e) => setNewTitle(e.target.value)}
                className="px-3 py-1.5 rounded-lg border border-[var(--border-subtle)] bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] focus:outline-none focus:border-[var(--accent-primary)]"
                required
              />
              <input
                type="text"
                placeholder="Emoji Icon (e.g. 🧪)"
                value={newIcon}
                onChange={(e) => setNewIcon(e.target.value)}
                className="px-3 py-1.5 rounded-lg border border-[var(--border-subtle)] bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] focus:outline-none focus:border-[var(--accent-primary)]"
              />
              <select
                value={newTarget}
                onChange={(e) => setNewTarget(e.target.value)}
                className="px-3 py-1.5 rounded-lg border border-[var(--border-subtle)] bg-[var(--surface-primary)] text-xs text-[var(--text-dominant)] focus:outline-none focus:border-[var(--accent-primary)]"
              >
                <option value="dashboard">⚡ Command Center</option>
                <option value="files">📁 Files & Study Vault</option>
                <option value="focus">🛡️ Focus Guard</option>
                <option value="journal">✍️ Reflection Journal</option>
                <option value="terminal">💻 MindSet Terminal</option>
                <option value="reader">📖 Study Reader</option>
                <option value="tasks">🎯 Tasks & Goals</option>
                <option value="app-hub">📦 App Hub</option>
              </select>
            </div>
            <div className="flex justify-end">
              <button
                type="submit"
                className="px-4 py-1.5 rounded-lg bg-[var(--accent-primary)] text-[var(--surface-primary)] text-xs font-bold shadow hover:brightness-110 transition-all"
              >
                Pin to Desktop Grid
              </button>
            </div>
          </form>
        )}

        {/* Shortcuts List */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
          {shortcuts.map((sc) => (
            <div
              key={sc.id}
              className="p-2.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--surface-primary)] flex items-center justify-between"
            >
              <div className="flex items-center space-x-2.5">
                <span className="text-xl">{sc.icon}</span>
                <div>
                  <div className="text-xs font-semibold text-[var(--text-dominant)]">
                    {sc.title}
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] font-mono">
                    Cell ({sc.grid_x}, {sc.grid_y}) • {sc.target_type}:{sc.target_payload}
                  </div>
                </div>
              </div>

              <button
                type="button"
                onClick={() => deleteShortcut(sc.id)}
                className="p-1 rounded-lg hover:bg-red-500/15 text-slate-500 hover:text-red-400 text-xs transition-colors"
                title="Remove Shortcut"
              >
                🗑️
              </button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

