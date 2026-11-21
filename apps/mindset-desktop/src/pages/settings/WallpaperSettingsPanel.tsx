/**
 * MindSet OS Personalization Studio — Wallpaper Engine Settings Panel
 * 10 Presets, Custom File Upload with path traversal protection, Blur & Dim Sliders
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useRef, useState } from 'react';
import {
  usePersonalizationStore,
  WALLPAPER_PRESETS,
} from '../../stores/usePersonalizationStore';
import { powerClient } from '../../lib/powerClient';

export const WallpaperSettingsPanel: React.FC = () => {
  const {
    wallpaperType,
    wallpaperPath,
    wallpaperDimness,
    wallpaperBlurPx,
    setWallpaperPreset,
    setCustomWallpaper,
    setWallpaperDimness,
    setWallpaperBlur,
  } = usePersonalizationStore();

  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploadStatus, setUploadStatus] = useState<string | null>(null);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setUploadStatus('Uploading wallpaper asset...');
    try {
      const reader = new FileReader();
      reader.onload = async () => {
        const base64Data = reader.result as string;
        try {
          const res = await powerClient.saveWallpaperAsset(base64Data, file.name);
          setCustomWallpaper(res.file_path);
          setUploadStatus(`Wallpaper applied: ${file.name}`);
          setTimeout(() => setUploadStatus(null), 4000);
        } catch (err: any) {
          setUploadStatus(`Upload failed: ${err.message || err}`);
          setTimeout(() => setUploadStatus(null), 5000);
        }
      };
      reader.readAsDataURL(file);
    } catch (err: any) {
      setUploadStatus(`Error: ${err.message || err}`);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h4 className="text-sm font-bold text-[var(--text-dominant)]">
          Wallpaper Engine & Dynamic Ambience
        </h4>
        <p className="text-xs text-[var(--text-muted)]">
          Calibrated neuroaesthetic backgrounds engineered to prevent ocular fatigue during marathon study blocks.
        </p>
      </div>

      {/* 10 Presets Visual Grid */}
      <div className="space-y-2.5">
        <div className="flex items-center justify-between text-xs">
          <span className="font-semibold text-[var(--text-dominant)]">Curated Wallpaper Presets:</span>
          <span className="text-[11px] text-[var(--text-muted)] font-mono">10 Available</span>
        </div>

        <div className="grid grid-cols-2 sm:grid-cols-5 gap-3">
          {WALLPAPER_PRESETS.map((preset) => {
            const isSelected = wallpaperType === 'preset' && wallpaperPath === preset.id;
            return (
              <button
                key={preset.id}
                type="button"
                onClick={() => setWallpaperPreset(preset.id)}
                className={`group flex flex-col rounded-2xl overflow-hidden border text-left transition-all duration-150 p-1.5 ${
                  isSelected
                    ? 'border-[var(--accent-primary)] ring-2 ring-[var(--accent-primary)]/50 bg-[var(--surface-elevated)] shadow-lg scale-[1.02]'
                    : 'border-[var(--border-subtle)] bg-[var(--surface-secondary)]/50 hover:border-[var(--accent-primary)]/50 hover:bg-[var(--surface-secondary)]'
                }`}
              >
                {/* Visual Thumbnail */}
                <div
                  className="w-full h-16 rounded-xl relative shadow-inner overflow-hidden"
                  style={{ background: preset.cssGradient }}
                >
                  {isSelected && (
                    <span className="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-[var(--accent-primary)] text-[var(--surface-primary)] text-[10px] font-bold flex items-center justify-center shadow">
                      ✓
                    </span>
                  )}
                </div>

                <div className="px-1 py-1.5">
                  <div className="text-xs font-semibold text-[var(--text-dominant)] truncate">
                    {preset.name}
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] capitalize">
                    {preset.category}
                  </div>
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* Custom Upload Section */}
      <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <div className="text-xs font-bold text-[var(--text-dominant)]">Custom Image or Animated Video</div>
          <div className="text-[11px] text-[var(--text-muted)]">
            Whitelisted types: PNG, JPG, WebP, SVG, GIF, MP4, WebM (Auto-paused during focus mode)
          </div>
          {uploadStatus && (
            <div className="text-xs text-[var(--accent-primary)] font-mono mt-1 animate-pulse">
              {uploadStatus}
            </div>
          )}
        </div>

        <div>
          <input
            ref={fileInputRef}
            type="file"
            accept=".png,.jpg,.jpeg,.webp,.svg,.gif,.mp4,.webm"
            className="hidden"
            onChange={handleFileUpload}
          />
          <button
            type="button"
            onClick={() => fileInputRef.current?.click()}
            className="px-4 py-2 rounded-xl border border-[var(--accent-primary)] bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] hover:bg-[var(--accent-primary)]/20 text-xs font-semibold transition-all"
          >
            📁 Choose File...
          </button>
        </div>
      </div>

      {/* Sliders: Dimness & Blur */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {/* Dimness Slider */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-[var(--text-dominant)]">Wallpaper Dimness:</span>
            <span className="font-mono text-[var(--accent-primary)]">{Math.round(wallpaperDimness * 100)}%</span>
          </div>
          <input
            type="range"
            min="0"
            max="0.85"
            step="0.05"
            value={wallpaperDimness}
            onChange={(e) => setWallpaperDimness(parseFloat(e.target.value))}
            className="w-full accent-[var(--accent-primary)] cursor-pointer"
          />
          <div className="text-[10px] text-[var(--text-muted)] flex justify-between">
            <span>0% (Vivid)</span>
            <span>85% (High Contrast Focus)</span>
          </div>
        </div>

        {/* Blur Slider */}
        <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-2">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-[var(--text-dominant)]">Wallpaper Frost Blur:</span>
            <span className="font-mono text-[var(--accent-primary)]">{wallpaperBlurPx}px</span>
          </div>
          <input
            type="range"
            min="0"
            max="32"
            step="2"
            value={wallpaperBlurPx}
            onChange={(e) => setWallpaperBlur(parseInt(e.target.value, 10))}
            className="w-full accent-[var(--accent-primary)] cursor-pointer"
          />
          <div className="text-[10px] text-[var(--text-muted)] flex justify-between">
            <span>0px (Sharp)</span>
            <span>32px (Liquid Frost)</span>
          </div>
        </div>
      </div>
    </div>
  );
};

