/**
 * MindSet OS — Notification Guard & Focus Policy Control Center
 * 4 Focus Profiles, 3 Protection Levels, Distraction Rules & Emergency Contact Allowlist
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import { useKernelStore } from '../../stores/useKernelStore';

export type FocusProfileType = 'study' | 'deep_work' | 'sleep' | 'custom';
export type ProtectionLevelType = 'gentle' | 'balanced' | 'strict';

export interface VipContact {
  id: string;
  name: string;
  relationship: string;
  phoneOrEmail: string;
  isAlwaysAllowed: boolean;
}

const DEFAULT_VIP_CONTACTS: VipContact[] = [
  { id: 'vip-1', name: 'Mom & Dad', relationship: 'Family', phoneOrEmail: '+91 98765 43210', isAlwaysAllowed: true },
  { id: 'vip-2', name: 'Physics Tutor (Ramesh Sir)', relationship: 'Academic', phoneOrEmail: 'ramesh.physics@edu.in', isAlwaysAllowed: true },
  { id: 'vip-3', name: 'Class Study Group Coordinator', relationship: 'School', phoneOrEmail: 'anand.class12@school.in', isAlwaysAllowed: false },
];

export const FocusGuardDashboard: React.FC = () => {
  const { isFocusActive } = useKernelStore();

  const [activeProfile, setActiveProfile] = useState<FocusProfileType>('study');
  const [protectionLevel, setProtectionLevel] = useState<ProtectionLevelType>('balanced');
  const [gamingBudgetMinutes, setGamingBudgetMinutes] = useState<number>(45);
  const [gamingUsedMinutes] = useState<number>(15);
  const [vipContacts, setVipContacts] = useState<VipContact[]>(DEFAULT_VIP_CONTACTS);
  const [newContactName, setNewContactName] = useState<string>('');
  const [newContactPhone, setNewContactPhone] = useState<string>('');
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const handleAddVipContact = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newContactName.trim() || !newContactPhone.trim()) return;

    const newContact: VipContact = {
      id: `vip-${Date.now()}`,
      name: newContactName.trim(),
      relationship: 'VIP / Emergency',
      phoneOrEmail: newContactPhone.trim(),
      isAlwaysAllowed: true,
    };

    setVipContacts((prev) => [...prev, newContact]);
    setNewContactName('');
    setNewContactPhone('');
    setToastMessage(`✓ Added "${newContact.name}" to Emergency Allowlist.`);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const handleToggleVip = (id: string) => {
    setVipContacts((prev) =>
      prev.map((c) => (c.id === id ? { ...c, isAlwaysAllowed: !c.isAlwaysAllowed } : c))
    );
  };

  const handleSavePolicy = () => {
    setToastMessage('✓ Focus Guard policy synchronized across Desktop & Phone.');
    setTimeout(() => setToastMessage(null), 3500);
  };

  return (
    <div className="space-y-6 select-none p-1 max-w-6xl mx-auto">
      {/* ------------------------------------------------------------- */}
      {/* 1. TOP HEADER & STATUS                                        */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-3">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xl">🛡️</span>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Focus Guard & Digital Wellbeing Daemon
              </h2>
            </div>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              System-level multi-layer enforcement: Unbound DNS, YouTube granular filter, application monitor, and emergency allowlist.
            </p>
          </div>

          <div className="flex items-center space-x-2">
            <span className={`px-3 py-1 rounded-full text-xs font-mono font-bold border ${
              isFocusActive
                ? 'border-[#65E6C4] bg-[#65E6C4]/15 text-[#65E6C4] animate-pulse'
                : 'border-[var(--border-subtle)] bg-[var(--surface-primary)] text-[var(--text-muted)]'
            }`}>
              {isFocusActive ? '● GUARD ENFORCING' : '○ GUARD STANDBY'}
            </span>

            <Button variant="primary" size="sm" onClick={handleSavePolicy} className="text-xs font-semibold">
              Save Policy
            </Button>
          </div>
        </div>

        {/* Intelligent Warning Simulation */}
        <div className="p-3 rounded-xl bg-[#8B9CFF]/10 border border-[#8B9CFF]/30 text-xs text-[#8B9CFF] flex items-center space-x-2.5">
          <span>ℹ️</span>
          <span>
            <strong>Intelligent Circadian Warning:</strong> Evening deep study scheduled in 35 minutes (19:00). Focus Guard will transition YouTube to Educational Only mode.
          </span>
        </div>

        {toastMessage && (
          <div className="text-xs text-[#65E6C4] font-medium animate-fadeIn">
            {toastMessage}
          </div>
        )}
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 2. FOUR FOCUS PROFILES & THREE PROTECTION LEVELS              */}
      {/* ------------------------------------------------------------- */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Four Focus Profiles */}
        <GlassSurface variant="solid-card" className="p-5 space-y-4">
          <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)] flex items-center space-x-2">
            <span>🎯</span>
            <span>Focus Profiles (Circadian Modes)</span>
          </h3>

          <div className="grid grid-cols-2 gap-2 text-xs">
            {[
              {
                id: 'study' as const,
                title: 'Study Mode',
                desc: 'Blocks social & games; allows PDF tutor & lectures',
                icon: '📖',
              },
              {
                id: 'deep_work' as const,
                title: 'Deep Work',
                desc: 'Blocks all messaging; allows IDEs & compiler',
                icon: '💻',
              },
              {
                id: 'sleep' as const,
                title: 'Sleep Mode (22:00)',
                desc: 'Restricts screens, dim theme, silences alerts',
                icon: '🌙',
              },
              {
                id: 'custom' as const,
                title: 'Custom Mode',
                desc: 'User-defined app & domain policy matrix',
                icon: '⚙️',
              },
            ].map((p) => (
              <button
                key={p.id}
                type="button"
                onClick={() => setActiveProfile(p.id)}
                className={`p-3 rounded-xl border text-left transition-all ${
                  activeProfile === p.id
                    ? 'border-[#8B9CFF] bg-[#8B9CFF]/15 text-[var(--text-dominant)] shadow-sm'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:border-[var(--border-strong)]'
                }`}
              >
                <div className="text-base">{p.icon}</div>
                <div className="font-bold text-[11px] text-[var(--text-dominant)] mt-1">{p.title}</div>
                <div className="text-[10px] text-[var(--text-muted)] mt-0.5 leading-snug">{p.desc}</div>
              </button>
            ))}
          </div>
        </GlassSurface>

        {/* Three Protection Levels */}
        <GlassSurface variant="solid-card" className="p-5 space-y-4">
          <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)] flex items-center space-x-2">
            <span>⚖️</span>
            <span>Protection Levels</span>
          </h3>

          <div className="space-y-2 text-xs">
            {[
              {
                id: 'gentle' as const,
                title: 'Gentle Mode',
                badge: 'NUDGE',
                desc: 'Usage reminders and calm prompts; zero hard blocking.',
              },
              {
                id: 'balanced' as const,
                title: 'Balanced Mode',
                badge: 'RECOMMENDED',
                desc: 'Scheduled app/domain blocking with controlled 5-minute exceptions.',
              },
              {
                id: 'strict' as const,
                title: 'Strict Mode',
                badge: 'MAX FOCUS',
                desc: 'System launch interception, nftables, and UAC admin unlock verification.',
              },
            ].map((lvl) => (
              <div
                key={lvl.id}
                onClick={() => setProtectionLevel(lvl.id)}
                className={`p-3 rounded-xl border cursor-pointer flex items-center justify-between transition-all ${
                  protectionLevel === lvl.id
                    ? 'border-[#65E6C4] bg-[#65E6C4]/10 shadow-sm'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)] text-[var(--text-muted)]'
                }`}
              >
                <div>
                  <div className="flex items-center space-x-2">
                    <span className="font-bold text-xs text-[var(--text-dominant)]">{lvl.title}</span>
                    <span className="px-1.5 py-0.2 rounded text-[9px] font-mono bg-[var(--surface-primary)] text-[#8B9CFF] border border-[var(--border-subtle)]">
                      {lvl.badge}
                    </span>
                  </div>
                  <div className="text-[11px] text-[var(--text-muted)] mt-0.5">{lvl.desc}</div>
                </div>

                <input
                  type="radio"
                  name="protection_level"
                  checked={protectionLevel === lvl.id}
                  onChange={() => setProtectionLevel(lvl.id)}
                  className="accent-[#65E6C4]"
                />
              </div>
            ))}
          </div>
        </GlassSurface>
      </div>

      {/* ------------------------------------------------------------- */}
      {/* 3. GAMING BUDGET & EMERGENCY CONTACT ALLOWLIST                */}
      {/* ------------------------------------------------------------- */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Gaming & Entertainment Time Budget */}
        <GlassSurface variant="solid-card" className="p-5 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)] flex items-center space-x-2">
              <span>🎮</span>
              <span>Gaming & Recreation Budget</span>
            </h3>
            <span className="text-xs font-mono font-bold text-[#FFD078]">
              {gamingUsedMinutes} / {gamingBudgetMinutes} mins used
            </span>
          </div>

          <p className="text-xs text-[var(--text-muted)]">
            Non-shaming daily allotment for Steam, browser games, and recreational streaming.
          </p>

          <input
            type="range"
            min="15"
            max="120"
            step="15"
            value={gamingBudgetMinutes}
            onChange={(e) => setGamingBudgetMinutes(Number(e.target.value))}
            className="w-full accent-[#FFD078] cursor-pointer"
          />

          <div className="flex justify-between text-[11px] font-mono text-[var(--text-muted)]">
            <span>15m Light</span>
            <span>45m Balanced (Default)</span>
            <span>120m Weekend</span>
          </div>
        </GlassSurface>

        {/* Emergency & VIP Contact Allowlist */}
        <GlassSurface variant="solid-card" className="p-5 space-y-4">
          <div>
            <h3 className="text-xs font-bold uppercase tracking-wider text-[var(--text-dominant)] flex items-center space-x-2">
              <span>🚨</span>
              <span>Emergency & VIP Contact Allowlist</span>
            </h3>
            <p className="text-[11px] text-[var(--text-muted)] mt-0.5">
              Strict Engineering Rule: Emergency and VIP communications are NEVER blocked, even in Strict Mode.
            </p>
          </div>

          {/* Contacts List */}
          <div className="space-y-2 max-h-36 overflow-y-auto">
            {vipContacts.map((c) => (
              <div
                key={c.id}
                className="p-2.5 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex items-center justify-between text-xs"
              >
                <div>
                  <div className="font-semibold text-[var(--text-dominant)]">{c.name}</div>
                  <div className="text-[10px] text-[var(--text-muted)] font-mono">{c.phoneOrEmail}</div>
                </div>

                <button
                  type="button"
                  onClick={() => handleToggleVip(c.id)}
                  className={`px-2 py-0.5 rounded text-[10px] font-mono font-semibold border ${
                    c.isAlwaysAllowed
                      ? 'border-[#65E6C4]/50 bg-[#65E6C4]/15 text-[#65E6C4]'
                      : 'border-[var(--border-subtle)] bg-[var(--surface-elevated)] text-[var(--text-muted)]'
                  }`}
                >
                  {c.isAlwaysAllowed ? 'ALWAYS ALLOW' : 'STANDARD'}
                </button>
              </div>
            ))}
          </div>

          {/* Quick Add VIP */}
          <form onSubmit={handleAddVipContact} className="flex items-center space-x-2 pt-2 border-t border-[var(--border-subtle)]">
            <input
              type="text"
              placeholder="Name (e.g. Sister, Doctor)..."
              value={newContactName}
              onChange={(e) => setNewContactName(e.target.value)}
              className="w-1/2 bg-[var(--surface-secondary)] text-xs text-[var(--text-dominant)] px-2.5 py-1.5 rounded-lg border border-[var(--border-subtle)] focus:outline-none"
            />
            <input
              type="text"
              placeholder="Phone or Email..."
              value={newContactPhone}
              onChange={(e) => setNewContactPhone(e.target.value)}
              className="w-1/2 bg-[var(--surface-secondary)] text-xs text-[var(--text-dominant)] px-2.5 py-1.5 rounded-lg border border-[var(--border-subtle)] focus:outline-none"
            />
            <button
              type="submit"
              className="px-3 py-1.5 rounded-lg text-xs font-semibold bg-[var(--accent-primary)] text-[#07090D] shrink-0"
            >
              +
            </button>
          </form>
        </GlassSurface>
      </div>
    </div>
  );
};
