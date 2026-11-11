/**
 * MindSet OS — Living Profile Evolution Suggestion Card
 * Non-Intrusive Consent-Driven Card: Never alters user_profile without explicit user approval.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import type { UserProfile } from '@mindset/db-schema';
import type { ProfileEvolutionSuggestion } from '@mindset/identity-engine';
import { GlassSurface } from '@mindset/ui-core';
import { dbClient } from '../lib/dbClient';

interface LivingProfileSuggestionCardProps {
  suggestion: ProfileEvolutionSuggestion;
  profile: UserProfile;
  onActionComplete: (updatedProfile?: UserProfile) => void;
}

export const LivingProfileSuggestionCard: React.FC<LivingProfileSuggestionCardProps> = ({
  suggestion,
  profile,
  onActionComplete,
}) => {
  const [isApplying, setIsApplying] = useState(false);

  const handleAccept = async () => {
    setIsApplying(true);
    try {
      const updatedProfile: UserProfile = {
        ...profile,
        ...suggestion.proposedChanges,
        updated_at: new Date().toISOString(),
      };
      await dbClient.profile.update(updatedProfile);
      setIsApplying(false);
      onActionComplete(updatedProfile);
    } catch (err) {
      console.error('Failed to update profile from suggestion:', err);
      setIsApplying(false);
      onActionComplete();
    }
  };

  const handleDismiss = () => {
    onActionComplete();
  };

  return (
    <GlassSurface
      variant="solid-card"
      className="p-4 rounded-xl border shadow-xl relative max-w-md animate-in fade-in slide-in-from-top-4 duration-300"
      style={{
        borderColor: 'var(--accent-primary)',
        backgroundColor: 'var(--bg-secondary)',
        color: 'var(--text-dominant)',
      }}
    >
      <div className="flex items-start gap-3">
        <span className="text-2xl mt-0.5">🌱</span>
        <div className="flex-1 space-y-1.5">
          <div className="flex items-center justify-between">
            <span className="text-[10px] uppercase font-mono font-bold tracking-wider text-[var(--accent-primary)]">
              Living Profile Evolution
            </span>
            <button
              onClick={handleDismiss}
              className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)]"
              title="Dismiss"
            >
              ✕
            </button>
          </div>

          <h4 className="text-xs font-bold text-[var(--text-dominant)]">
            {suggestion.title}
          </h4>

          <p className="text-xs text-[var(--text-secondary)] leading-relaxed">
            {suggestion.promptMessage}
          </p>

          <div className="flex items-center gap-2 pt-2">
            <button
              onClick={handleAccept}
              disabled={isApplying}
              className="px-3 py-1.5 rounded-lg bg-[var(--accent-primary)] text-[var(--bg-base)] text-xs font-bold hover:opacity-90 transition shadow-sm"
            >
              {isApplying ? 'Updating...' : 'Accept & Update Profile'}
            </button>
            <button
              onClick={handleDismiss}
              className="px-3 py-1.5 rounded-lg bg-[var(--bg-primary)] hover:bg-[var(--bg-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] transition"
            >
              Keep As Is
            </button>
          </div>
        </div>
      </div>
    </GlassSurface>
  );
};
