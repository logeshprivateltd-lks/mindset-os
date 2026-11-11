/**
 * MindSet OS — Canonical Route Navigation Engine
 * Routes requests across Windows, Modals, and Dashboard Zones:
 * / (Command Center), /mind, /tasks, /habits, /journal, /mood,
 * /insights, /timeline, /files, /devices, /settings, /status
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export type CanonicalRoute =
  | '/'
  | '/dashboard'
  | '/mind'
  | '/tasks'
  | '/habits'
  | '/journal'
  | '/mood'
  | '/insights'
  | '/timeline'
  | '/files'
  | '/devices'
  | '/settings'
  | '/status'
  | '/focus'
  | '/focus-guard'
  | '/balance'
  | '/app-hub'
  | '/updater'
  | '/recovery'
  | '/connect';

export interface RouteNavigationEvent {
  route: CanonicalRoute;
  params?: Record<string, unknown>;
  timestamp: number;
}

type RouteListener = (event: RouteNavigationEvent) => void;

class NavigationRouter {
  private currentRoute: CanonicalRoute = '/';
  private listeners: Set<RouteListener> = new Set();

  public getCurrentRoute(): CanonicalRoute {
    return this.currentRoute;
  }

  public navigate(route: CanonicalRoute, params?: Record<string, unknown>): void {
    this.currentRoute = route;
    const event: RouteNavigationEvent = {
      route,
      params,
      timestamp: Date.now(),
    };

    // Emit event to subscribers
    this.listeners.forEach((listener) => {
      try {
        listener(event);
      } catch (err) {
        console.error('[NavigationRouter] Listener failed:', err);
      }
    });

    // Also dispatch custom DOM event for decoupled components
    if (typeof window !== 'undefined') {
      window.dispatchEvent(
        new CustomEvent('mindset:navigate', {
          detail: event,
        })
      );
    }
  }

  public subscribe(listener: RouteListener): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }
}

export const navigationRouter = new NavigationRouter();

export const CANONICAL_ROUTES_META: Record<
  CanonicalRoute,
  { label: string; icon: string; description: string; category: 'core' | 'productivity' | 'wellbeing' | 'system' }
> = {
  '/': {
    label: 'Command Center',
    icon: '⚡',
    description: 'Personalised 5-zone daily command center and focus dashboard',
    category: 'core',
  },
  '/dashboard': {
    label: 'Command Center',
    icon: '⚡',
    description: 'Personalised 5-zone daily command center and focus dashboard',
    category: 'core',
  },
  '/mind': {
    label: 'Mind & Memory',
    icon: '🧠',
    description: 'Sovereign AI 3-tier memory inspector and neural profile graph',
    category: 'wellbeing',
  },
  '/tasks': {
    label: 'Tasks & Goals',
    icon: '🎯',
    description: 'Syllabus milestones, daily study checklists, and exam targets',
    category: 'productivity',
  },
  '/habits': {
    label: 'Daily Habits',
    icon: '🔥',
    description: 'Non-punitive streak tracking and daily momentum routines',
    category: 'productivity',
  },
  '/journal': {
    label: 'Reflective Journal',
    icon: '✍️',
    description: 'Distraction-free Markdown journaling with gratitude logs',
    category: 'wellbeing',
  },
  '/mood': {
    label: 'Mood & Energy',
    icon: '🌈',
    description: 'Real-time energy check-in and emotional state logging',
    category: 'wellbeing',
  },
  '/insights': {
    label: 'AI Insights',
    icon: '💡',
    description: 'Living user profile evolution and study pattern analytics',
    category: 'core',
  },
  '/timeline': {
    label: 'Daily Timeline',
    icon: '⏱️',
    description: 'Vertical chronological schedule and study commitment blocks',
    category: 'core',
  },
  '/files': {
    label: 'Study Vault & Files',
    icon: '🗂️',
    description: 'Distraction-free file manager with PDF RAG tutor and phone sync',
    category: 'core',
  },
  '/devices': {
    label: 'Connected Devices',
    icon: '📱',
    description: 'MindSet Connect peer pairing, Wi-Fi adapters, and phone vault',
    category: 'system',
  },
  '/settings': {
    label: 'OS Settings',
    icon: '⚙️',
    description: 'Interface language, Tanglish AI voice, themes, and audio',
    category: 'system',
  },
  '/status': {
    label: 'Kernel Telemetry',
    icon: '📊',
    description: 'Rust service health registry, RAM breakdown, and hardware probe',
    category: 'system',
  },
  '/focus': {
    label: 'Immersive Focus',
    icon: '⏳',
    description: 'Distraction-free fullscreen Aurora countdown with Web Audio synthesizer',
    category: 'productivity',
  },
  '/focus-guard': {
    label: 'Focus Guard & Shield',
    icon: '🛡️',
    description: 'Non-shaming distraction shield, gaming budget limits, and VIP allowlist',
    category: 'productivity',
  },
  '/balance': {
    label: 'MindSet Balance AI',
    icon: '🌿',
    description: '4-7-8 breathwork, free-time gap planner, 12-hobby explorer, and intentional play',
    category: 'wellbeing',
  },
  '/app-hub': {
    label: 'MindSet App Hub',
    icon: '📦',
    description: 'Curated distraction-free software catalog for STEM, coding, and productivity',
    category: 'system',
  },
  '/updater': {
    label: 'Atomic OTA Updater',
    icon: '🚀',
    description: 'Dual-layer delta updater with Ed25519 signatures, ostree A/B slots, and 3s rollback',
    category: 'system',
  },
  '/recovery': {
    label: 'MindSet Recovery Console',
    icon: '🛟',
    description: 'Pre-boot standalone rescue environment for encrypted snapshots, SQLite repairs, and driver resets',
    category: 'system',
  },
  '/connect': {
    label: 'MindSet Connect',
    icon: '📱',
    description: '6-Module cross-device mesh for phone webcam, Whisper mic, continuity handoff, and selective sync',
    category: 'system',
  },
};

