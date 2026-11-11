/**
 * MindSet OS — Language Control & Live Preview Component
 * Reusable panel for /settings and /onboarding Screen 2
 * Provides 100% decoupled control over OS Interface Language (i18next)
 * and MindSet AI Response Language & Tanglish Conversation Style.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useMemo } from 'react';
import type { UserProfile } from '@mindset/db-schema';
import {
  SUPPORTED_LOCALES,
  SupportedLocale,
  ConversationStyle,
  setAppLanguage,
  getCurrentLanguage,
  useTranslation,
} from '@mindset/i18n-locales';
import { getSampleAiPreview } from '@mindset/identity-engine';
import { GlassSurface } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';

export interface LanguageSettingsPanelProps {
  initialInterfaceLang?: SupportedLocale;
  initialAiLang?: string;
  initialConversationStyle?: ConversationStyle | string;
  onSaved?: (profile: UserProfile) => void;
  onChange?: (settings: {
    interfaceLang: SupportedLocale;
    aiLang: string;
    conversationStyle: string;
  }) => void;
  isEmbeddedInOnboarding?: boolean;
}

export const LanguageSettingsPanel: React.FC<LanguageSettingsPanelProps> = ({
  initialInterfaceLang,
  initialAiLang,
  initialConversationStyle,
  onSaved,
  onChange,
  isEmbeddedInOnboarding = false,
}) => {
  const { t } = useTranslation();

  // Selected State
  const [interfaceLang, setInterfaceLang] = useState<SupportedLocale>(
    initialInterfaceLang || getCurrentLanguage()
  );
  const [aiLang, setAiLang] = useState<string>(initialAiLang || 'ta');
  const [conversationStyle, setConversationStyle] = useState<string>(
    initialConversationStyle || 'tanglish'
  );

  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState<string | null>(null);

  // Sync state if props change
  useEffect(() => {
    if (initialInterfaceLang) setInterfaceLang(initialInterfaceLang);
    if (initialAiLang) setAiLang(initialAiLang);
    if (initialConversationStyle) setConversationStyle(initialConversationStyle);
  }, [initialInterfaceLang, initialAiLang, initialConversationStyle]);

  // Handle immediate interface language change
  const handleSelectInterfaceLang = async (loc: SupportedLocale) => {
    setInterfaceLang(loc);
    await setAppLanguage(loc);
    if (onChange) {
      onChange({
        interfaceLang: loc,
        aiLang,
        conversationStyle,
      });
    }
  };

  const handleSelectAiLang = (lang: string) => {
    setAiLang(lang);
    if (onChange) {
      onChange({
        interfaceLang,
        aiLang: lang,
        conversationStyle,
      });
    }
  };

  const handleSelectConversationStyle = (style: string) => {
    setConversationStyle(style);
    if (onChange) {
      onChange({
        interfaceLang,
        aiLang,
        conversationStyle: style,
      });
    }
  };

  // Generate Sample Live Preview
  const samplePreview = useMemo(() => {
    return getSampleAiPreview(aiLang, conversationStyle);
  }, [aiLang, conversationStyle]);

  // Save changes to SQLite user_profile
  const handleSaveChanges = async () => {
    setIsSaving(true);
    try {
      const currentProfile = await dbClient.profile.get();
      const updatedProfile: UserProfile = {
        ...(currentProfile || {
          user_id: 'default_student',
          display_name: 'MindSet Scholar',
          role: 'student',
          goals_json: '[]',
          focus_areas_json: '[]',
          routine_json: '{}',
          preferences_json: '{}',
          interests_json: '[]',
          permissions_json: '{}',
          updated_at: new Date().toISOString(),
        }),
        interface_lang: interfaceLang,
        ai_lang: (['en', 'ta', 'hi', 'te', 'ml', 'kn', 'bn', 'mr'].includes(aiLang)
          ? aiLang
          : 'ta') as UserProfile['ai_lang'],
        conversation_style: (['tanglish', 'formal', 'concise', 'socratic'].includes(conversationStyle)
          ? conversationStyle
          : 'tanglish') as UserProfile['conversation_style'],
        updated_at: new Date().toISOString(),
      };

      await dbClient.profile.update(updatedProfile);
      await setAppLanguage(interfaceLang);

      setIsSaving(false);
      setSaveMessage('Language preferences applied and saved to SQLite!');
      if (onSaved) onSaved(updatedProfile);

      setTimeout(() => setSaveMessage(null), 3500);
    } catch (err) {
      console.error('Failed to save language settings:', err);
      setIsSaving(false);
      setSaveMessage('Preferences updated in active session');
      setTimeout(() => setSaveMessage(null), 3000);
    }
  };

  return (
    <div className={`space-y-6 ${isEmbeddedInOnboarding ? '' : 'p-6'}`}>
      {/* Header Info (if not in onboarding) */}
      {!isEmbeddedInOnboarding && (
        <div className="flex items-center justify-between pb-4 border-b border-[var(--border-subtle)]">
          <div>
            <h2 className="text-xl font-bold text-[var(--text-dominant)] flex items-center gap-2">
              <span>🌐</span> {t('settings.languageRegion')}
            </h2>
            <p className="text-xs text-[var(--text-muted)] mt-1">
              MindSet OS dual-layer architecture: Keep OS UI in English while chatting with MindSet AI in Tamil or Tanglish.
            </p>
          </div>
          {saveMessage && (
            <span className="text-xs font-mono font-bold text-emerald-400 animate-pulse">
              ✓ {saveMessage}
            </span>
          )}
        </div>
      )}

      {/* Two Column Configuration Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* ===================================================================== */}
        {/* LAYER A: OS Interface Language (i18next)                              */}
        {/* ===================================================================== */}
        <div className="p-4 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs uppercase tracking-wider font-bold text-[var(--accent-secondary)]">
              Layer A: OS Interface Language
            </span>
            <span className="text-[10px] font-mono text-[var(--text-muted)]">
              Active: <strong className="text-[var(--text-dominant)] uppercase">{interfaceLang}</strong>
            </span>
          </div>
          <p className="text-[11px] text-[var(--text-muted)]">
            Changes system menus, buttons, taskbar, alerts, and settings with zero reload.
          </p>

          <div className="grid grid-cols-2 gap-2 pt-1">
            {(Object.keys(SUPPORTED_LOCALES) as SupportedLocale[]).map((loc) => {
              const meta = SUPPORTED_LOCALES[loc];
              const isSelected = interfaceLang === loc;
              return (
                <button
                  key={loc}
                  type="button"
                  onClick={() => handleSelectInterfaceLang(loc)}
                  className="p-2.5 rounded-lg text-left transition border flex flex-col justify-between"
                  style={{
                    backgroundColor: isSelected ? 'var(--surface-elevated)' : 'var(--bg-primary)',
                    borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                    color: isSelected ? 'var(--accent-primary)' : 'var(--text-secondary)',
                    boxShadow: isSelected ? '0 0 12px var(--border-glow)' : 'none',
                  }}
                >
                  <div className="flex items-center justify-between w-full">
                    <span className="text-xs font-bold text-[var(--text-dominant)]">
                      {meta.nativeName}
                    </span>
                    {isSelected && <span className="text-xs text-[var(--accent-primary)] font-bold">✓</span>}
                  </div>
                  <div className="flex items-center justify-between text-[10px] text-[var(--text-muted)] mt-1">
                    <span>{meta.name}</span>
                    <span className="font-mono text-[9px] uppercase opacity-75">{meta.script}</span>
                  </div>
                </button>
              );
            })}
          </div>
        </div>

        {/* ===================================================================== */}
        {/* LAYER B: MindSet AI Response Language & Conversation Style            */}
        {/* ===================================================================== */}
        <div className="p-4 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-xs uppercase tracking-wider font-bold text-[var(--accent-primary)]">
              Layer B: AI Response Language & Style
            </span>
            <span className="text-[10px] font-mono text-[var(--text-muted)]">
              Active: <strong className="text-[var(--text-dominant)] uppercase">{aiLang}</strong> ({conversationStyle})
            </span>
          </div>
          <p className="text-[11px] text-[var(--text-muted)]">
            Injected dynamically into PersonalContextBuilder for high pedagogical relatability.
          </p>

          {/* AI Language Quick Switcher */}
          <div>
            <label className="text-[10px] uppercase font-bold text-[var(--text-muted)] block mb-1.5">
              Target Mentoring Language
            </label>
            <div className="grid grid-cols-4 gap-1.5">
              {(Object.keys(SUPPORTED_LOCALES) as SupportedLocale[]).map((loc) => {
                const isSelected = aiLang === loc;
                return (
                  <button
                    key={loc}
                    type="button"
                    onClick={() => handleSelectAiLang(loc)}
                    className="p-2 rounded-lg text-center border text-xs font-semibold transition"
                    style={{
                      backgroundColor: isSelected ? 'var(--surface-elevated)' : 'var(--bg-primary)',
                      borderColor: isSelected ? 'var(--accent-primary)' : 'var(--border-subtle)',
                      color: isSelected ? 'var(--accent-primary)' : 'var(--text-secondary)',
                    }}
                  >
                    {SUPPORTED_LOCALES[loc].nativeName}
                  </button>
                );
              })}
            </div>
          </div>

          {/* AI Speech & Code-Switching Style Selector */}
          <div>
            <label className="text-[10px] uppercase font-bold text-[var(--text-muted)] block mb-1.5">
              Conversation & Transliteration Style
            </label>
            <div className="space-y-1.5">
              <button
                type="button"
                onClick={() => handleSelectConversationStyle('tanglish')}
                className="w-full p-2.5 rounded-lg text-left text-xs border transition flex items-center justify-between"
                style={{
                  backgroundColor:
                    conversationStyle === 'tanglish'
                      ? 'var(--surface-elevated)'
                      : 'var(--bg-primary)',
                  borderColor:
                    conversationStyle === 'tanglish'
                      ? 'var(--accent-primary)'
                      : 'var(--border-subtle)',
                }}
              >
                <div>
                  <div className="font-bold text-[var(--text-dominant)] flex items-center gap-1.5">
                    <span>✨</span> Tanglish / Hinglish (Conversational Code-Switching)
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] mt-0.5">
                    Romanized regional language mixed naturally with English technical terms
                  </div>
                </div>
                {conversationStyle === 'tanglish' && (
                  <span className="text-xs text-[var(--accent-primary)] font-bold">✓</span>
                )}
              </button>

              <button
                type="button"
                onClick={() => handleSelectConversationStyle('pure_script')}
                className="w-full p-2.5 rounded-lg text-left text-xs border transition flex items-center justify-between"
                style={{
                  backgroundColor:
                    conversationStyle === 'pure_script'
                      ? 'var(--surface-elevated)'
                      : 'var(--bg-primary)',
                  borderColor:
                    conversationStyle === 'pure_script'
                      ? 'var(--accent-primary)'
                      : 'var(--border-subtle)',
                }}
              >
                <div>
                  <div className="font-bold text-[var(--text-dominant)] flex items-center gap-1.5">
                    <span>📜</span> Native Script (Contemporary Script)
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] mt-0.5">
                    Natural contemporary script (e.g. தமிழ் / हिन्दी) with scientific terms
                  </div>
                </div>
                {conversationStyle === 'pure_script' && (
                  <span className="text-xs text-[var(--accent-primary)] font-bold">✓</span>
                )}
              </button>

              <button
                type="button"
                onClick={() => handleSelectConversationStyle('mixed')}
                className="w-full p-2.5 rounded-lg text-left text-xs border transition flex items-center justify-between"
                style={{
                  backgroundColor:
                    conversationStyle === 'mixed'
                      ? 'var(--surface-elevated)'
                      : 'var(--bg-primary)',
                  borderColor:
                    conversationStyle === 'mixed'
                      ? 'var(--accent-primary)'
                      : 'var(--border-subtle)',
                }}
              >
                <div>
                  <div className="font-bold text-[var(--text-dominant)] flex items-center gap-1.5">
                    <span>🌐</span> Bilingual Mixed (Seamless Switching)
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] mt-0.5">
                    Seamless paragraph-level switching between English and chosen language
                  </div>
                </div>
                {conversationStyle === 'mixed' && (
                  <span className="text-xs text-[var(--accent-primary)] font-bold">✓</span>
                )}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* ========================================================================= */}
      {/* LIVE SYNCHRONOUS PREVIEW CARD                                             */}
      {/* ========================================================================= */}
      <GlassSurface
        variant="solid-card"
        className="p-5 rounded-2xl border space-y-4"
        style={{
          borderColor: 'var(--accent-primary)',
          backgroundColor: 'var(--surface-elevated)',
          boxShadow: '0 8px 32px rgba(0,0,0,0.3)',
        }}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" />
            <span className="text-xs font-mono font-bold uppercase tracking-wider text-[var(--accent-primary)]">
              Live Synchronous Preview (Zero-Reload Simulation)
            </span>
          </div>
          <span className="text-[10px] font-mono text-[var(--text-muted)]">
            OS UI: <strong className="text-[var(--accent-secondary)] uppercase">{interfaceLang}</strong> • AI: <strong className="text-[var(--accent-primary)]">{samplePreview.styleLabel}</strong>
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* Left Preview: OS UI Elements */}
          <div className="p-4 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] space-y-3">
            <span className="text-[10px] uppercase font-bold text-[var(--accent-secondary)] block">
              OS Interface Sample ({SUPPORTED_LOCALES[interfaceLang]?.nativeName})
            </span>

            {/* Simulated Desktop System Notification */}
            <div className="p-3 rounded-lg bg-[var(--bg-base)] border border-[var(--border-subtle)] space-y-1">
              <div className="flex items-center justify-between text-[10px]">
                <span className="font-bold text-[var(--accent-primary)] flex items-center gap-1">
                  <span>🛡️</span> {t('focusGuard.title')}
                </span>
                <span className="font-mono text-[var(--text-muted)]">Just now</span>
              </div>
              <p className="text-xs text-[var(--text-dominant)]">
                {t('focusGuard.strictMode')}
              </p>
            </div>

            {/* Simulated OS Buttons */}
            <div className="flex items-center gap-2 pt-1">
              <button
                type="button"
                className="px-3 py-1.5 rounded-lg bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold shadow"
              >
                {t('commandCenter.startFocusSession')}
              </button>
              <button
                type="button"
                className="px-3 py-1.5 rounded-lg bg-[var(--surface-elevated)] border border-[var(--border-subtle)] text-[var(--text-secondary)] text-xs font-medium"
              >
                {t('taskbar.lockScreen')}
              </button>
            </div>
          </div>

          {/* Right Preview: MindSet AI Mentor Response */}
          <div className="p-4 rounded-xl bg-[var(--surface-primary)] border border-[var(--border-subtle)] space-y-2.5">
            <div className="flex items-center justify-between text-[10px]">
              <span className="uppercase font-bold text-[var(--accent-primary)] flex items-center gap-1">
                <span>🤖</span> MindSet AI Mentor ({samplePreview.styleLabel})
              </span>
              <span className="text-[var(--text-muted)] italic font-mono text-[9px]">
                Query: "{samplePreview.query}"
              </span>
            </div>

            {/* AI Speech Bubble */}
            <div className="p-3.5 rounded-xl bg-[var(--bg-base)] border border-[var(--accent-primary)]/40 relative shadow-md">
              <p className="text-xs text-[var(--text-dominant)] leading-relaxed italic">
                "{samplePreview.response}"
              </p>
            </div>

            <div className="text-[10px] text-[var(--text-muted)] flex items-center gap-1.5">
              <span>💡</span>
              <span>Notice how academic terms and timing stay crystal-clear in conversational flow.</span>
            </div>
          </div>
        </div>
      </GlassSurface>

      {/* Save Button (when not in onboarding) */}
      {!isEmbeddedInOnboarding && (
        <div className="flex justify-end pt-2">
          <button
            type="button"
            onClick={handleSaveChanges}
            disabled={isSaving}
            className="px-6 py-2.5 rounded-xl bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold hover:opacity-90 transition shadow-lg flex items-center gap-2"
          >
            {isSaving ? 'Applying...' : '💾 Apply & Save Language Preferences'}
          </button>
        </div>
      )}
    </div>
  );
};

