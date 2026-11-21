/**
 * MindSet OS Multi-Window Manager Store
 * Hardware-accelerated 60 FPS window management with 2/3/4-grid snap layouts
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { create } from 'zustand';

export type SnapZone =
  | 'left-50'
  | 'right-50'
  | '3-grid-left'
  | '3-grid-top-right'
  | '3-grid-bottom-right'
  | 'quad-top-left'
  | 'quad-top-right'
  | 'quad-bottom-left'
  | 'quad-bottom-right'
  | 'maximize';

export interface WindowBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface OsWindow {
  id: string;
  appId: string;
  title: string;
  icon: string;
  subjectTag?: string;
  bounds: WindowBounds;
  prevBounds?: WindowBounds;
  zIndex: number;
  isMinimized: boolean;
  isMaximized: boolean;
  snapZone?: SnapZone;
  activityId: string; // Belongs to specific activity workspace or 'all'
  pinnedToAllActivities?: boolean;
}

export interface WindowManagerStoreState {
  windows: OsWindow[];
  activeWindowId: string | null;
  draggingWindowId: string | null;
  resizingWindowId: string | null;
  hoveredSnapZone: SnapZone | null;
  highestZIndex: number;

  // Actions
  openWindow: (
    appId: string,
    initialProps?: Partial<Omit<OsWindow, 'id' | 'appId'>>,
    targetActivityId?: string
  ) => string;
  closeWindow: (id: string) => void;
  minimizeWindow: (id: string) => void;
  maximizeWindow: (id: string) => void;
  restoreWindow: (id: string) => void;
  toggleMaximize: (id: string) => void;
  focusWindow: (id: string) => void;
  bringToFront: (id: string) => void;
  updateWindowBounds: (id: string, bounds: Partial<WindowBounds>) => void;
  snapWindow: (id: string, zone: SnapZone) => void;
  setDraggingWindowId: (id: string | null) => void;
  setResizingWindowId: (id: string | null) => void;
  setHoveredSnapZone: (zone: SnapZone | null) => void;
}

const TITLEBAR_HEIGHT = 36;
const TASKBAR_HEIGHT = 48;

export function getWorkAreaBounds(): { x: number; y: number; width: number; height: number } {
  if (typeof window === 'undefined') {
    return { x: 0, y: TITLEBAR_HEIGHT, width: 1280, height: 720 };
  }
  return {
    x: 0,
    y: TITLEBAR_HEIGHT,
    width: window.innerWidth,
    height: Math.max(400, window.innerHeight - TITLEBAR_HEIGHT - TASKBAR_HEIGHT),
  };
}

export function computeSnapBounds(zone: SnapZone): WindowBounds {
  const work = getWorkAreaBounds();
  const halfW = Math.round(work.width / 2);
  const halfH = Math.round(work.height / 2);

  switch (zone) {
    case 'left-50':
    case '3-grid-left':
      return { x: 0, y: work.y, width: halfW, height: work.height };

    case 'right-50':
      return { x: halfW, y: work.y, width: work.width - halfW, height: work.height };

    case '3-grid-top-right':
      return { x: halfW, y: work.y, width: work.width - halfW, height: halfH };

    case '3-grid-bottom-right':
      return { x: halfW, y: work.y + halfH, width: work.width - halfW, height: work.height - halfH };

    case 'quad-top-left':
      return { x: 0, y: work.y, width: halfW, height: halfH };

    case 'quad-bottom-left':
      return { x: 0, y: work.y + halfH, width: halfW, height: work.height - halfH };

    case 'quad-top-right':
      return { x: halfW, y: work.y, width: work.width - halfW, height: halfH };

    case 'quad-bottom-right':
      return { x: halfW, y: work.y + halfH, width: work.width - halfW, height: work.height - halfH };

    case 'maximize':
    default:
      return { x: 0, y: work.y, width: work.width, height: work.height };
  }
}

export const useWindowManagerStore = create<WindowManagerStoreState>((set, get) => ({
  windows: [],
  activeWindowId: null,
  draggingWindowId: null,
  resizingWindowId: null,
  hoveredSnapZone: null,
  highestZIndex: 10,

  openWindow: (appId, initialProps = {}, targetActivityId = 'deep-study') => {
    const existing = get().windows.find((w) => w.appId === appId);
    if (existing) {
      // If already open, restore if minimized and bring to front
      set((state) => ({
        windows: state.windows.map((w) =>
          w.id === existing.id
            ? { ...w, isMinimized: false, zIndex: state.highestZIndex + 1 }
            : w
        ),
        activeWindowId: existing.id,
        highestZIndex: state.highestZIndex + 1,
      }));
      return existing.id;
    }

    const work = getWorkAreaBounds();
    const count = get().windows.length;
    const cascadeOffset = (count % 6) * 28;

    const defaultWidth = Math.min(840, Math.round(work.width * 0.72));
    const defaultHeight = Math.min(560, Math.round(work.height * 0.76));

    const defaultBounds: WindowBounds = {
      x: Math.max(20, Math.round((work.width - defaultWidth) / 2) + cascadeOffset),
      y: Math.max(work.y + 10, work.y + 30 + cascadeOffset),
      width: defaultWidth,
      height: defaultHeight,
    };

    const newWindowId = `win-${appId}-${Date.now().toString(36)}`;
    const newZ = get().highestZIndex + 1;

    const newWindow: OsWindow = {
      id: newWindowId,
      appId,
      title: initialProps.title || appId.toUpperCase(),
      icon: initialProps.icon || '📦',
      subjectTag: initialProps.subjectTag,
      bounds: initialProps.bounds || defaultBounds,
      zIndex: newZ,
      isMinimized: false,
      isMaximized: false,
      activityId: targetActivityId || 'deep-study',
      pinnedToAllActivities: initialProps.pinnedToAllActivities ?? false,
      ...initialProps,
    };

    set((state) => ({
      windows: [...state.windows, newWindow],
      activeWindowId: newWindowId,
      highestZIndex: newZ,
    }));

    return newWindowId;
  },

  closeWindow: (id) => {
    set((state) => {
      const remaining = state.windows.filter((w) => w.id !== id);
      const nextActive =
        state.activeWindowId === id
          ? remaining.length > 0
            ? remaining[remaining.length - 1]?.id || null
            : null
          : state.activeWindowId;
      return {
        windows: remaining,
        activeWindowId: nextActive,
      };
    });
  },

  minimizeWindow: (id) => {
    set((state) => ({
      windows: state.windows.map((w) => (w.id === id ? { ...w, isMinimized: true } : w)),
      activeWindowId: state.activeWindowId === id ? null : state.activeWindowId,
    }));
  },

  maximizeWindow: (id) => {
    const win = get().windows.find((w) => w.id === id);
    if (!win) return;

    if (!win.isMaximized) {
      const snapBounds = computeSnapBounds('maximize');
      set((state) => ({
        windows: state.windows.map((w) =>
          w.id === id
            ? {
                ...w,
                prevBounds: w.bounds,
                bounds: snapBounds,
                isMaximized: true,
                snapZone: 'maximize',
                isMinimized: false,
                zIndex: state.highestZIndex + 1,
              }
            : w
        ),
        activeWindowId: id,
        highestZIndex: state.highestZIndex + 1,
      }));
    }
  },

  restoreWindow: (id) => {
    const win = get().windows.find((w) => w.id === id);
    if (!win) return;

    const restoreBounds = win.prevBounds || {
      x: 100,
      y: 60,
      width: 800,
      height: 520,
    };

    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id
          ? {
              ...w,
              bounds: restoreBounds,
              isMaximized: false,
              snapZone: undefined,
              isMinimized: false,
              zIndex: state.highestZIndex + 1,
            }
          : w
      ),
      activeWindowId: id,
      highestZIndex: state.highestZIndex + 1,
    }));
  },

  toggleMaximize: (id) => {
    const win = get().windows.find((w) => w.id === id);
    if (!win) return;
    if (win.isMaximized || win.snapZone) {
      get().restoreWindow(id);
    } else {
      get().maximizeWindow(id);
    }
  },

  focusWindow: (id) => {
    get().bringToFront(id);
  },

  bringToFront: (id) => {
    set((state) => {
      const nextZ = state.highestZIndex + 1;
      return {
        windows: state.windows.map((w) =>
          w.id === id ? { ...w, zIndex: nextZ, isMinimized: false } : w
        ),
        activeWindowId: id,
        highestZIndex: nextZ,
      };
    });
  },

  updateWindowBounds: (id, newBounds) => {
    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id
          ? {
              ...w,
              bounds: { ...w.bounds, ...newBounds },
              isMaximized: false,
              snapZone: undefined,
            }
          : w
      ),
    }));
  },

  snapWindow: (id, zone) => {
    const targetBounds = computeSnapBounds(zone);

    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id
          ? {
              ...w,
              prevBounds: w.snapZone || w.isMaximized ? w.prevBounds : w.bounds,
              bounds: targetBounds,
              snapZone: zone,
              isMaximized: zone === 'maximize',
              isMinimized: false,
              zIndex: state.highestZIndex + 1,
            }
          : w
      ),
      activeWindowId: id,
      hoveredSnapZone: null,
      highestZIndex: state.highestZIndex + 1,
    }));
  },

  setDraggingWindowId: (draggingWindowId) => set({ draggingWindowId }),
  setResizingWindowId: (resizingWindowId) => set({ resizingWindowId }),
  setHoveredSnapZone: (hoveredSnapZone) => set({ hoveredSnapZone }),
}));
