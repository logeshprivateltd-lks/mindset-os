/**
 * MindSet OS Balance Engine — Free Time Detector
 * Analyzes scheduled commitments, detects open windows, and protects unplanned buffer & sleep time.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { FreeTimeSlot, ScheduledBlock } from './types';

export interface FreeTimeDetectorOptions {
  dayStartTime?: string;     // Default '06:00'
  sleepStartTime?: string;   // Default '22:00' (Sleep Protected Window)
  defaultBufferMinutes?: number; // Default 15 mins unplanned buffer
}

export class FreeTimeDetector {
  private dayStartMinutes: number;
  private sleepStartMinutes: number;
  private defaultBufferMinutes: number;

  constructor(options?: FreeTimeDetectorOptions) {
    this.dayStartMinutes = this.timeToMinutes(options?.dayStartTime ?? '06:00');
    this.sleepStartMinutes = this.timeToMinutes(options?.sleepStartTime ?? '22:00');
    this.defaultBufferMinutes = options?.defaultBufferMinutes ?? 15;
  }

  /**
   * Convert "HH:MM" string to minutes from midnight
   */
  public timeToMinutes(timeStr: string): number {
    const parts = timeStr.split(':').map((p) => parseInt(p, 10));
    const hours = parts[0] ?? 0;
    const minutes = parts[1] ?? 0;
    return hours * 60 + minutes;
  }

  /**
   * Convert minutes from midnight to "HH:MM" 24h format
   */
  public minutesToTime(totalMinutes: number): string {
    const clamped = Math.max(0, Math.min(24 * 60 - 1, totalMinutes));
    const hours = Math.floor(clamped / 60);
    const minutes = clamped % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
  }

  /**
   * Detects available free-time windows in a daily timetable
   */
  public detectFreeSlots(
    blocks: ScheduledBlock[],
    currentMinutes?: number
  ): FreeTimeSlot[] {
    const effectiveStart = currentMinutes !== undefined
      ? Math.max(this.dayStartMinutes, currentMinutes)
      : this.dayStartMinutes;

    if (effectiveStart >= this.sleepStartMinutes) {
      return []; // Night sleep protected period
    }

    // Sort blocks chronologically
    const sorted = [...blocks]
      .map((b) => ({
        ...b,
        startMin: this.timeToMinutes(b.startTime),
        endMin: this.timeToMinutes(b.endTime),
      }))
      .filter((b) => b.endMin > effectiveStart && b.startMin < this.sleepStartMinutes)
      .sort((a, b) => a.startMin - b.startMin);

    const freeSlots: FreeTimeSlot[] = [];
    let pointer = effectiveStart;

    for (const block of sorted) {
      if (block.startMin > pointer) {
        const gap = block.startMin - pointer;
        if (gap >= 15) {
          const slot = this.createSlot(pointer, block.startMin);
          if (slot.usableMinutes >= 10) {
            freeSlots.push(slot);
          }
        }
      }
      pointer = Math.max(pointer, block.endMin);
    }

    // Check gap between last block and sleep window
    if (pointer < this.sleepStartMinutes) {
      const gap = this.sleepStartMinutes - pointer;
      if (gap >= 15) {
        const slot = this.createSlot(pointer, this.sleepStartMinutes);
        if (slot.usableMinutes >= 10) {
          freeSlots.push(slot);
        }
      }
    }

    return freeSlots;
  }

  /**
   * Creates a FreeTimeSlot with dynamic buffer allocation
   */
  private createSlot(startMin: number, endMin: number): FreeTimeSlot {
    const totalMinutes = endMin - startMin;
    
    // Allocate proportional unplanned buffer
    let buffer = this.defaultBufferMinutes;
    if (totalMinutes <= 25) {
      buffer = 5;
    } else if (totalMinutes <= 45) {
      buffer = 10;
    } else if (totalMinutes >= 90) {
      buffer = 20;
    }

    const usableMinutes = Math.max(0, totalMinutes - buffer);
    // Evening wind-down check: if slot overlaps after 21:00 (1260 mins)
    const isWindDownWindow = endMin >= 1260 || startMin >= 1260;

    return {
      startTime: this.minutesToTime(startMin),
      endTime: this.minutesToTime(endMin),
      totalMinutes,
      usableMinutes,
      bufferMinutes: buffer,
      isWindDownWindow,
    };
  }
}
