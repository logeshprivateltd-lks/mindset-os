/**
 * MindSet OS Native Power Management IPC Client
 * Communicates with Kernel PowerManager for deterministic teardown, safe lock, and wallpaper saving
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type {
  PowerAction,
  PowerActionResult,
  PowerStateResponse,
  WallpaperAssetResponse,
} from '@mindset/db-schema';

async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[powerClient] Tauri invoke failed for command '${cmd}':`, err);
    throw err;
  }
}

export const powerClient = {
  /**
   * Executes deterministic OS power action: shutdown, restart, sleep, lock, exit_to_host
   */
  executeAction: async (action: PowerAction, reason?: string): Promise<PowerActionResult> => {
    try {
      return await invokeTauri<PowerActionResult>('kernel_execute_power_action', {
        action,
        reason,
      });
    } catch {
      // Browser fallback simulation
      return {
        success: true,
        action,
        message: `Simulation: Power action '${action}' processed cleanly.`,
        timestamp_utc: new Date().toISOString(),
      };
    }
  },

  /**
   * Reads current kernel power and lock state
   */
  getState: async (): Promise<PowerStateResponse> => {
    try {
      return await invokeTauri<PowerStateResponse>('kernel_get_power_state');
    } catch {
      return {
        is_locked: false,
        is_sleeping: false,
        last_action: null,
        uptime_secs: 120,
        safe_teardown_ready: true,
      };
    }
  },

  /**
   * Validates and copies custom wallpaper file into ~/.mindset_data/wallpapers/
   */
  saveWallpaperAsset: async (
    sourcePath: string,
    filename?: string
  ): Promise<WallpaperAssetResponse> => {
    try {
      return await invokeTauri<WallpaperAssetResponse>('kernel_save_wallpaper_asset', {
        sourcePath,
        filename,
      });
    } catch (err) {
      // In browser/dev mode fallback
      return {
        file_path: sourcePath,
        file_name: filename || 'custom_wallpaper.png',
        file_size_bytes: 1024 * 1024,
        mime_type: 'image/png',
      };
    }
  },
};

