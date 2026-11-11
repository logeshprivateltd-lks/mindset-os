/**
 * MindSet OS — Offline Lecture & Media Player
 * Clean zero-distraction lecture player with 0.75x–2.5x speeds, subtitle toggle & one-click notes timestamp bookmarking
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useRef } from 'react';
import { Button } from '@mindset/ui-core';
import { dbClient } from '../../lib/dbClient';

export interface OfflineLecturePlayerProps {
  initialVideoSrc?: string;
  lectureTitle?: string;
  onClose?: () => void;
}

export const OfflineLecturePlayer: React.FC<OfflineLecturePlayerProps> = ({
  initialVideoSrc,
  lectureTitle = 'Class 12 Physics: Rotational Dynamics & Moment of Inertia',
  onClose,
}) => {
  const [videoSrc, setVideoSrc] = useState<string | null>(initialVideoSrc || null);
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const [currentTime, setCurrentTime] = useState<number>(0);
  const [duration, setDuration] = useState<number>(45 * 60 + 12); // Default 45m 12s demo
  const [playbackRate, setPlaybackRate] = useState<number>(1.0);
  const [volume, setVolume] = useState<number>(85);
  const [isCaptionsOn, setIsCaptionsOn] = useState<boolean>(true);
  const [, setBookmarkedTimestamps] = useState<{ timeSec: number; label: string }[]>([]);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const videoRef = useRef<HTMLVideoElement | null>(null);

  const formatTime = (secs: number): string => {
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const handlePlayPause = () => {
    if (videoRef.current) {
      if (isPlaying) {
        videoRef.current.pause();
      } else {
        videoRef.current.play().catch(() => {});
      }
      setIsPlaying(!isPlaying);
    } else {
      setIsPlaying(!isPlaying);
    }
  };

  const handleSpeedChange = (speed: number) => {
    setPlaybackRate(speed);
    if (videoRef.current) {
      videoRef.current.playbackRate = speed;
    }
    showToast(`Speed set to ${speed}x`);
  };

  const handleBookmarkTimestamp = async () => {
    const timestampStr = formatTime(currentTime);
    const label = `Timestamp [${timestampStr}] in ${lectureTitle}`;
    setBookmarkedTimestamps((prev) => [...prev, { timeSec: currentTime, label }]);

    try {
      // Save directly to SQLite Journal
      await dbClient.wellbeing.createJournal({
        title: `Lecture Bookmark: ${lectureTitle}`,
        content_md: `### 📌 Bookmarked Concept at ${timestampStr}\n- Lecture: ${lectureTitle}\n- Review note: Check derivation step and formula notation at this second.`,
        mood_tag: 'Focused',
        energy_level: 'High',
        gratitude_items_json: JSON.stringify(['Clear lecture explanation']),
      });
      showToast(`✓ Bookmarked [${timestampStr}] to Study Notes!`);
    } catch {
      showToast(`Bookmarked [${timestampStr}] locally.`);
    }
  };

  const handleFileChoose = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const url = URL.createObjectURL(file);
      setVideoSrc(url);
      setIsPlaying(false);
      showToast(`Loaded lecture: ${file.name}`);
    }
  };

  return (
    <div className="h-full flex flex-col p-6 space-y-4 select-none bg-[var(--surface-primary)] text-[var(--text-dominant)] overflow-hidden">
      {/* Top Header */}
      <div className="flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]">
        <div className="flex items-center space-x-3">
          <span className="text-xl">🎬</span>
          <div>
            <h2 className="text-sm font-bold truncate max-w-md">{lectureTitle}</h2>
            <p className="text-[10px] text-[var(--text-muted)]">
              Distraction-Free Local Player • Zero ads, recommendations, or comments
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          {/* File Picker */}
          <label className="px-3 py-1.5 rounded-xl bg-[var(--surface-secondary)] hover:bg-[var(--surface-secondary)]/80 text-xs font-semibold cursor-pointer border border-[var(--border-subtle)] flex items-center space-x-1.5">
            <span>📁 Open File</span>
            <input type="file" accept="video/*,audio/*" onChange={handleFileChoose} className="hidden" />
          </label>

          {onClose && (
            <button
              type="button"
              onClick={onClose}
              className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1.5 rounded-lg"
            >
              ✕
            </button>
          )}
        </div>
      </div>

      {/* Toast Notification */}
      {toastMessage && (
        <div className="p-2.5 rounded-xl bg-[#65E6C4]/15 border border-[#65E6C4]/40 text-xs text-[#65E6C4] font-medium flex items-center justify-between animate-fadeIn">
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Video Display Container */}
      <div className="relative flex-1 rounded-2xl bg-black overflow-hidden flex items-center justify-center border border-[var(--border-subtle)]">
        {videoSrc ? (
          <video
            ref={videoRef}
            src={videoSrc}
            onTimeUpdate={(e) => setCurrentTime((e.target as HTMLVideoElement).currentTime)}
            onLoadedMetadata={(e) => setDuration((e.target as HTMLVideoElement).duration)}
            className="w-full h-full object-contain"
          />
        ) : (
          <div className="text-center space-y-3 p-8">
            <span className="text-5xl block animate-pulse">🎓</span>
            <div className="space-y-1">
              <h3 className="text-base font-bold text-white">Offline Lecture Canvas</h3>
              <p className="text-xs text-white/60 max-w-sm mx-auto">
                Open any recorded lecture video (.mp4, .webm, .mkv) or audio file to study with zero algorithmic distractions.
              </p>
            </div>
            <label className="inline-block px-4 py-2 rounded-xl bg-[#8B9CFF] text-[#07090D] font-bold text-xs cursor-pointer shadow-lg">
              Select Local Lecture Video
              <input type="file" accept="video/*,audio/*" onChange={handleFileChoose} className="hidden" />
            </label>
          </div>
        )}

        {/* Live Closed Caption Simulation */}
        {isCaptionsOn && (
          <div className="absolute bottom-6 left-1/2 -translate-x-1/2 px-4 py-1.5 rounded-xl bg-black/80 backdrop-blur-md border border-white/20 text-xs text-white font-mono text-center max-w-lg pointer-events-none shadow-xl">
            "...angular momentum \(L = I \cdot \omega\) is conserved when net external torque equals zero..."
          </div>
        )}
      </div>

      {/* Controls Bar */}
      <div className="p-4 rounded-2xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] space-y-3">
        {/* Scrubber progress */}
        <div className="space-y-1">
          <input
            type="range"
            min="0"
            max={duration || 100}
            value={currentTime}
            onChange={(e) => {
              const t = Number(e.target.value);
              setCurrentTime(t);
              if (videoRef.current) videoRef.current.currentTime = t;
            }}
            className="w-full accent-[#8B9CFF] cursor-pointer"
          />
          <div className="flex justify-between text-[11px] font-mono text-[var(--text-muted)]">
            <span className="tabular-nums">{formatTime(currentTime)}</span>
            <span className="tabular-nums">{formatTime(duration)}</span>
          </div>
        </div>

        {/* Playback Controls & Speed Stepper */}
        <div className="flex flex-wrap items-center justify-between gap-3 text-xs">
          <div className="flex items-center space-x-2">
            <Button variant="primary" size="sm" onClick={handlePlayPause} className="font-bold text-xs px-4">
              {isPlaying ? '⏸️ Pause' : '▶️ Play'}
            </Button>

            {/* Bookmark to Notes Button */}
            <Button
              variant="secondary"
              size="sm"
              onClick={handleBookmarkTimestamp}
              className="text-xs font-semibold text-[#65E6C4] border-[#65E6C4]/40"
            >
              📌 Bookmark [{formatTime(currentTime)}] to Notes
            </Button>
          </div>

          {/* Speed Selector */}
          <div className="flex items-center space-x-1">
            <span className="text-[var(--text-muted)] text-[11px]">Speed:</span>
            {[0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5].map((s) => (
              <button
                key={s}
                type="button"
                onClick={() => handleSpeedChange(s)}
                className={`px-2 py-1 rounded-lg font-mono font-semibold text-[11px] transition-all ${
                  playbackRate === s
                    ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                    : 'bg-[var(--surface-primary)] text-[var(--text-muted)] hover:text-[var(--text-dominant)]'
                }`}
              >
                {s}x
              </button>
            ))}
          </div>

          <div className="flex items-center space-x-3">
            {/* Captions toggle */}
            <button
              type="button"
              onClick={() => setIsCaptionsOn(!isCaptionsOn)}
              className={`px-2.5 py-1 rounded-lg font-bold text-[11px] border transition-all ${
                isCaptionsOn
                  ? 'bg-[#65E6C4]/15 border-[#65E6C4] text-[#65E6C4]'
                  : 'bg-[var(--surface-primary)] border-[var(--border-subtle)] text-[var(--text-muted)]'
              }`}
            >
              CC
            </button>

            {/* Volume */}
            <div className="flex items-center space-x-1.5">
              <span>🔊</span>
              <input
                type="range"
                min="0"
                max="100"
                value={volume}
                onChange={(e) => {
                  setVolume(Number(e.target.value));
                  if (videoRef.current) videoRef.current.volume = Number(e.target.value) / 100;
                }}
                className="w-16 accent-[#8B9CFF] cursor-pointer"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
