/**
 * MindSet OS — MemoryInspectorService
 * Sovereign Student Memory Control: Inspect, edit, export, and delete AI memory records.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { UserProfile } from '@mindset/db-schema';
import type {
  MemoryLevel1_Profile,
  MemoryLevel2_ActiveContext,
  MemoryLevel3_Record,
  MemoryInspectionFilter,
  MemoryExportPayload,
} from './types';
import { PersonalContextBuilder } from './PersonalContextBuilder';

export class MemoryInspectorService {
  /**
   * Inspects and returns Level 1 Personal Profile Memory
   */
  public static inspectLevel1(profile: UserProfile): MemoryLevel1_Profile {
    return PersonalContextBuilder.parseProfileMemory(profile);
  }

  /**
   * Inspects and formats Level 2 Active Volatile Working Context
   */
  public static inspectLevel2(
    activeContext?: MemoryLevel2_ActiveContext
  ): MemoryLevel2_ActiveContext {
    return (
      activeContext || {
        currentTime: new Date().toISOString(),
        availableFreeMinutes: 45,
        nextCommitment: null,
        activeFocusMode: 'DeepWork',
        pendingTasks: [],
        recentMood: 'Focused',
        recentEnergy: 'High',
        activeActivity: 'deep-study',
      }
    );
  }

  /**
   * Filters and lists Level 3 Epistemic / Long-Term Progress Memories
   */
  public static listLevel3Memories(
    memories: MemoryLevel3_Record[],
    filter?: MemoryInspectionFilter
  ): MemoryLevel3_Record[] {
    let result = [...memories];

    if (!filter) return result;

    if (filter.sourceType) {
      result = result.filter((m) => m.sourceType === filter.sourceType);
    }

    if (filter.tag) {
      const tagLower = filter.tag.toLowerCase();
      result = result.filter((m) =>
        m.tags.some((t) => t.toLowerCase() === tagLower)
      );
    }

    if (filter.searchTerm) {
      const termLower = filter.searchTerm.toLowerCase();
      result = result.filter(
        (m) =>
          m.title.toLowerCase().includes(termLower) ||
          m.details.toLowerCase().includes(termLower)
      );
    }

    if (filter.limit && filter.limit > 0) {
      result = result.slice(0, filter.limit);
    }

    return result;
  }

  /**
   * Creates a new Level 3 Memory record
   */
  public static addMemory(
    memories: MemoryLevel3_Record[],
    record: Omit<MemoryLevel3_Record, 'id' | 'timestamp'>
  ): { updatedMemories: MemoryLevel3_Record[]; newRecord: MemoryLevel3_Record } {
    const newRecord: MemoryLevel3_Record = {
      ...record,
      id: `mem-${Date.now()}-${Math.random().toString(36).substring(2, 7)}`,
      timestamp: new Date().toISOString(),
    };

    return {
      updatedMemories: [newRecord, ...memories],
      newRecord,
    };
  }

  /**
   * Updates an existing Level 3 memory record
   */
  public static updateMemory(
    memories: MemoryLevel3_Record[],
    updated: MemoryLevel3_Record
  ): MemoryLevel3_Record[] {
    return memories.map((m) => (m.id === updated.id ? updated : m));
  }

  /**
   * Deletes a specific Level 3 memory record
   */
  public static deleteMemory(
    memories: MemoryLevel3_Record[],
    id: string
  ): MemoryLevel3_Record[] {
    return memories.filter((m) => m.id !== id);
  }

  /**
   * Clears all Level 3 memory records
   */
  public static clearAllMemories(): MemoryLevel3_Record[] {
    return [];
  }

  /**
   * Generates a complete memory export payload for student data portability
   */
  public static buildExportPayload(
    profile: UserProfile,
    activeContext: MemoryLevel2_ActiveContext | null,
    memories: MemoryLevel3_Record[]
  ): MemoryExportPayload {
    return {
      exportedAt: new Date().toISOString(),
      version: '1.0.0',
      level1Profile: this.inspectLevel1(profile),
      level2ContextSnapshot: activeContext,
      level3Memories: memories,
    };
  }

  /**
   * Exports the student's complete memory history as JSON
   */
  public static exportAsJson(payload: MemoryExportPayload): string {
    return JSON.stringify(payload, null, 2);
  }

  /**
   * Exports the student's complete memory history as Markdown
   */
  public static exportAsMarkdown(payload: MemoryExportPayload): string {
    const p1 = payload.level1Profile;
    const goalsMd = p1.goals
      .map(
        (g) =>
          `- [${g.completed ? 'x' : ' '}] **${g.title}** (Priority ${g.priority}, Progress: ${g.progressPercent}%)`
      )
      .join('\n');

    const memoriesMd = payload.level3Memories.length > 0
      ? payload.level3Memories
          .map(
            (m) =>
              `### ${m.title}\n- **Source:** \`${m.sourceType}\` | **Date:** ${m.timestamp}\n- **Tags:** ${m.tags.join(', ') || 'None'}\n\n${m.details}`
          )
          .join('\n\n')
      : '_No long-term epistemic memories recorded yet._';

    return `# MindSet OS — Student Epistemic Memory Export
**Exported At:** ${payload.exportedAt}  
**Student Name:** ${p1.displayName}  
**Role:** ${p1.role}  
**Languages:** Interface: \`${p1.interfaceLang}\` | AI: \`${p1.aiLang}\` (${p1.conversationStyle})

---

## 1. Level 1: Profile & Ambitions
### Primary Goals
${goalsMd || '_No active goals configured._'}

### Focus Subjects
${p1.focusAreas.map((f) => `- ${f}`).join('\n') || '_None_'}

### Learning Preferences
- **Learning Style:** ${p1.preferences.learningStyle}
- **AI Tone:** ${p1.preferences.aiTone}
- **Planning Style:** ${p1.preferences.planningStyle}
- **Tanglish Enabled:** ${p1.preferences.tanglishEnabled ? 'Yes' : 'No'}

---

## 2. Level 3: Long-Term Epistemic Memories (${payload.level3Memories.length} Records)
${memoriesMd}
`;
  }
}
