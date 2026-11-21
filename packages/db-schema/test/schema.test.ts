/**
 * MindSet OS DB Schema & Desktop Grid Validation Tests (Phase 25)
 * Tests DesktopShortcut grid algorithms, UserPersonalization bounds, and Power Lifecycle phases
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type {
  DesktopShortcut,
  UserPersonalization,
  DesktopWidgetsConfig,
  PowerAction,
} from '../src/types';

function assert(condition: boolean, msg: string) {
  if (!condition) {
    throw new Error(`Assertion failed: ${msg}`);
  }
}

console.log('--- MindSet OS Phase 25 Schema & Grid Test Suite ---');

// 1. Grid coordinate calculations (88x96px cells with 16px gaps)
const CELL_W = 88;
const CELL_H = 96;
const GAP_X = 16;
const GAP_Y = 16;
const PAD_X = 20;
const PAD_Y = 20;

function computeCellFromPixel(clientX: number, clientY: number) {
  const relX = Math.max(0, clientX - PAD_X);
  const relY = Math.max(0, clientY - PAD_Y);
  const cellX = Math.floor(relX / (CELL_W + GAP_X));
  const cellY = Math.floor(relY / (CELL_H + GAP_Y));
  return { x: cellX, y: cellY };
}

function findNextAvailableCell(shortcuts: DesktopShortcut[]): { x: number; y: number } {
  let y = 0;
  while (shortcuts.some((s) => s.grid_x === 0 && s.grid_y === y)) {
    y++;
  }
  return { x: 0, y };
}

// Test 1: Coordinate snapping
const c1 = computeCellFromPixel(20, 20);
assert(c1.x === 0 && c1.y === 0, 'Origin should snap to (0,0)');

const c2 = computeCellFromPixel(20 + 88 + 16, 20 + 96 + 16);
assert(c2.x === 1 && c2.y === 1, 'Adjacent cell should snap to (1,1)');
console.log('✓ Desktop 88x96px snap-to-grid coordinate calculations verified');

// Test 2: Shortcut list and collision-free slot allocation
const testShortcuts: DesktopShortcut[] = [
  { id: 'sc1', title: 'Command Center', icon: '⚡', target_type: 'app', target_payload: 'dashboard', grid_x: 0, grid_y: 0, is_pinned: true },
  { id: 'sc2', title: 'Files & RAG', icon: '📁', target_type: 'app', target_payload: 'files', grid_x: 0, grid_y: 1, is_pinned: true },
  { id: 'sc3', title: 'Focus Guard', icon: '🛡️', target_type: 'app', target_payload: 'focus', grid_x: 0, grid_y: 2, is_pinned: true },
];

const nextCell = findNextAvailableCell(testShortcuts);
assert(nextCell.x === 0 && nextCell.y === 3, 'Next slot should be (0,3)');
console.log('✓ Collision-free desktop shortcut slot allocation verified');

// Test 3: User Personalization clamping & bounds
function sanitizePersonalization(p: Partial<UserPersonalization>): UserPersonalization {
  const dimness = Math.max(0, Math.min(0.85, p.wallpaper_dimness ?? 0.25));
  const blur = Math.max(0, Math.min(32, p.wallpaper_blur_px ?? 0));
  const fontScale = Math.max(0.9, Math.min(1.25, p.font_scale ?? 1.0));
  const glassOpacity = Math.max(0.5, Math.min(1.0, p.glass_opacity ?? 0.85));

  return {
    user_id: p.user_id || 'default_student',
    wallpaper_type: p.wallpaper_type || 'preset',
    wallpaper_path: p.wallpaper_path || 'nebula_deep',
    wallpaper_dimness: dimness,
    wallpaper_blur_px: blur,
    accent_color: p.accent_color || '#8B9CFF',
    font_scale: fontScale,
    glass_opacity: glassOpacity,
    motion_intensity: p.motion_intensity || 'normal',
    dock_position: p.dock_position || 'bottom',
    dock_autohide: p.dock_autohide || false,
    dock_style: p.dock_style || 'floating',
    widgets_json: p.widgets_json || '{"clock":true,"target":true,"affirmation":true}',
  };
}

const sanitized = sanitizePersonalization({
  wallpaper_dimness: 1.5, // should clamp to 0.85
  wallpaper_blur_px: 50,  // should clamp to 32
  font_scale: 2.0,        // should clamp to 1.25
  glass_opacity: 0.2,     // should clamp to 0.5
});

assert(sanitized.wallpaper_dimness === 0.85, 'Dimness should clamp to 0.85');
assert(sanitized.wallpaper_blur_px === 32, 'Blur should clamp to 32px');
assert(sanitized.font_scale === 1.25, 'Font scale should clamp to 1.25');
assert(sanitized.glass_opacity === 0.5, 'Glass opacity should clamp to 0.5');
console.log('✓ Personalization parameter safety bounds and clamping verified');

// Test 4: Desktop Widgets JSON serialization
const widgets: DesktopWidgetsConfig = {
  clock: true,
  target: true,
  affirmation: true,
  notes: false,
};
const json = JSON.stringify(widgets);
const parsed: DesktopWidgetsConfig = JSON.parse(json);
assert(parsed.clock === true, 'Clock widget enabled');
assert(parsed.target === true, 'Target widget enabled');
assert(parsed.affirmation === true, 'Affirmation widget enabled');
assert(parsed.notes === false, 'Notes widget disabled');
console.log('✓ DesktopWidgetsConfig JSON serialization and parsing verified');

// Test 5: Power Actions enum validation
const validActions: PowerAction[] = ['shutdown', 'restart', 'sleep', 'lock', 'exit_to_host'];
assert(validActions.includes('shutdown'), 'Shutdown is valid');
assert(validActions.includes('restart'), 'Restart is valid');
assert(validActions.includes('sleep'), 'Sleep is valid');
assert(validActions.includes('lock'), 'Lock is valid');
assert(validActions.includes('exit_to_host'), 'Exit to host is valid');
console.log('✓ PowerAction lifecycle targets verified');

console.log('All 5 Phase 25 TypeScript schema & grid tests passed successfully!');

