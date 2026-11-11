/**
 * MindSet OS — Chronological Activity Timeline (/timeline)
 * Typed, filterable event stream logging tasks, habits, focus blocks, and milestones
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useCallback } from 'react';
import { GlassSurface, Button } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';
import type { TimelineEvent } from '@mindset/db-schema';

type FilterType = 'all' | 'task_completion' | 'focus_session' | 'habit_tick' | 'mood_logged' | 'system_boot';

export const TimelinePage: React.FC = () => {
  const [events, setEvents] = useState<TimelineEvent[]>([]);
  const [activeFilter, setActiveFilter] = useState<FilterType>('all');
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const loadTimeline = useCallback(async () => {
    setIsLoading(true);
    try {
      const typeArg = activeFilter === 'all' ? undefined : activeFilter;
      const list = await dbClient.timeline.list(100, typeArg);
      setEvents(list);
    } catch (err) {
      console.error('[Timeline] Failed to load events:', err);
      setEvents([]);
    } finally {
      setIsLoading(false);
    }
  }, [activeFilter]);

  useEffect(() => {
    loadTimeline();
  }, [loadTimeline]);

  const getEventMeta = (event: TimelineEvent) => {
    switch (event.event_type) {
      case 'task_completion':
        return {
          icon: '🎯',
          badge: 'TASK COMPLETED',
          badgeColor: 'border-[#65E6C4]/40 bg-[#65E6C4]/15 text-[#65E6C4]',
          xp: '+50 XP',
        };
      case 'focus_session':
        return {
          icon: '⚡',
          badge: 'FOCUS BLOCK',
          badgeColor: 'border-[#8B9CFF]/40 bg-[#8B9CFF]/15 text-[#8B9CFF]',
          xp: '+35 XP',
        };
      case 'habit_tick':
        return {
          icon: '🔥',
          badge: 'HABIT STREAK',
          badgeColor: 'border-[#FFD078]/40 bg-[#FFD078]/15 text-[#FFD078]',
          xp: '+25 XP',
        };
      case 'mood_logged':
        return {
          icon: '🌈',
          badge: 'WELLBEING',
          badgeColor: 'border-[#A78BFA]/40 bg-[#A78BFA]/15 text-[#A78BFA]',
          xp: '+10 XP',
        };
      case 'system_boot':
      case 'lock_screen':
        return {
          icon: '💻',
          badge: 'SYSTEM',
          badgeColor: 'border-[var(--border-subtle)] bg-[var(--surface-elevated)] text-[var(--text-muted)]',
          xp: null,
        };
      default:
        return {
          icon: '📌',
          badge: 'ACTIVITY',
          badgeColor: 'border-[var(--border-subtle)] bg-[var(--surface-elevated)] text-[var(--text-muted)]',
          xp: null,
        };
    }
  };

  const formatTimestamp = (isoStr: string) => {
    try {
      const d = new Date(isoStr);
      return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) + ' • ' + d.toLocaleDateString([], { month: 'short', day: 'numeric' });
    } catch {
      return isoStr;
    }
  };

  return (
    <div className="space-y-6 select-none p-1">
      {/* ------------------------------------------------------------- */}
      {/* 1. HEADER & FILTER CHIPS                                      */}
      {/* ------------------------------------------------------------- */}
      <GlassSurface variant="solid-card" className="p-5 space-y-4">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xl">⏱️</span>
              <h2 className="text-lg font-bold text-[var(--text-dominant)]">
                Chronological Activity Timeline
              </h2>
            </div>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              Immutable ledger of study accomplishments, completed focus blocks, habit streaks, and wellness check-ins.
            </p>
          </div>

          <Button variant="secondary" size="sm" onClick={loadTimeline} className="text-xs">
            🔄 Refresh Stream
          </Button>
        </div>

        {/* Filter Pills */}
        <div className="flex flex-wrap items-center gap-1.5 pt-1">
          {[
            { id: 'all' as const, label: 'All Activities', icon: '📋' },
            { id: 'task_completion' as const, label: 'Tasks', icon: '🎯' },
            { id: 'focus_session' as const, label: 'Focus Blocks', icon: '⚡' },
            { id: 'habit_tick' as const, label: 'Habits', icon: '🔥' },
            { id: 'mood_logged' as const, label: 'Wellbeing', icon: '🌈' },
            { id: 'system_boot' as const, label: 'System', icon: '💻' },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveFilter(tab.id)}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center space-x-1.5 transition-all ${
                activeFilter === tab.id
                  ? 'bg-[var(--accent-primary)] text-[#07090D] shadow-sm'
                  : 'bg-[var(--surface-secondary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)] border border-[var(--border-subtle)]'
              }`}
            >
              <span>{tab.icon}</span>
              <span>{tab.label}</span>
            </button>
          ))}
        </div>
      </GlassSurface>

      {/* ------------------------------------------------------------- */}
      {/* 2. CHRONOLOGICAL EVENT STREAM                                 */}
      {/* ------------------------------------------------------------- */}
      <div className="relative pl-6 space-y-4 before:absolute before:left-2.5 before:top-3 before:bottom-3 before:w-0.5 before:bg-[var(--border-subtle)]">
        {isLoading ? (
          <div className="text-xs text-[var(--text-muted)] py-12 text-center">
            Loading timeline events from SQLite...
          </div>
        ) : events.length === 0 ? (
          <GlassSurface variant="solid-card" className="p-8 text-center text-xs text-[var(--text-muted)] space-y-2">
            <span className="text-3xl">🌱</span>
            <div className="font-semibold text-[var(--text-dominant)]">No activity logged in this category yet.</div>
            <div>Complete a study task, tick a habit, or start a focus timer to record your first milestone!</div>
          </GlassSurface>
        ) : (
          events.map((event) => {
            const meta = getEventMeta(event);

            return (
              <div key={event.id} className="relative group">
                {/* Node Dot on vertical timeline line */}
                <div className="absolute -left-6 top-3 w-4 h-4 rounded-full bg-[var(--surface-primary)] border-2 border-[var(--accent-primary)] flex items-center justify-center shadow-md">
                  <div className="w-1.5 h-1.5 rounded-full bg-[var(--accent-primary)]" />
                </div>

                {/* Event Card */}
                <GlassSurface variant="solid-card" className="p-4 space-y-2 group-hover:border-[var(--border-strong)] transition-all">
                  <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-1 text-xs">
                    <div className="flex items-center space-x-2">
                      <span className="text-lg">{meta.icon}</span>
                      <span className="font-bold text-[var(--text-dominant)]">{event.title}</span>
                      <span className={`px-2 py-0.5 rounded text-[10px] font-mono font-semibold border ${meta.badgeColor}`}>
                        {meta.badge}
                      </span>
                    </div>

                    <div className="flex items-center space-x-2 font-mono text-[11px] text-[var(--text-muted)]">
                      {meta.xp && <span className="text-[#FFD078] font-bold">{meta.xp}</span>}
                      <span>{formatTimestamp(event.created_at)}</span>
                    </div>
                  </div>

                  {event.metadata_json && event.metadata_json !== '{}' && (
                    <div className="text-[11px] font-mono text-[var(--text-muted)] bg-[var(--surface-secondary)] p-2 rounded-lg border border-[var(--border-subtle)] truncate">
                      {event.metadata_json}
                    </div>
                  )}
                </GlassSurface>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};

