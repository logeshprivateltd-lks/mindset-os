/**
//! MindSet OS Frontend Data Access Layer (dbClient)
//! Strongly-typed asynchronous IPC client mapped to local_core.sqlite & sync_oplog.sqlite
//! Powered by LKS PVT LTD
*/

import type {
  UserProfile,
  TaskOrGoal,
  Habit,
  JournalEntry,
  MoodCheckin,
  FocusSession,
  VectorSearchResult,
  DbIntegrityReport,
} from '@mindset/db-schema';

async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[dbClient] Tauri invoke failed for command '${cmd}':`, err);
    throw err;
  }
}

export const dbClient = {
  // -------------------------------------------------------------------------
  // 1. Profile & Permissions
  // -------------------------------------------------------------------------
  profile: {
    get: async (userId?: string): Promise<UserProfile | null> => {
      try {
        return await invokeTauri<UserProfile | null>('db_get_profile', { userId });
      } catch {
        return {
          user_id: userId || 'default_student',
          display_name: 'MindSet Scholar',
          role: 'student',
          interface_lang: 'en',
          ai_lang: 'ta',
          conversation_style: 'tanglish',
          goals_json: '["Master Core Mathematics", "Score 98% in Physics"]',
          focus_areas_json: '["Mathematics", "Quantum Mechanics"]',
          routine_json: '{"wake_time":"05:30","sleep_time":"22:30"}',
          preferences_json: '{"soundscape":"rain_forest"}',
          interests_json: '["Astronomy", "Rust Programming"]',
          permissions_json: '{"strict_focus_mode":true}',
          updated_at: new Date().toISOString(),
        };
      }
    },

    update: async (profile: UserProfile): Promise<UserProfile> => {
      return await invokeTauri<UserProfile>('db_update_profile', { profile });
    },

    updatePermissions: async (userId: string, permissionsJson: string): Promise<void> => {
      return await invokeTauri<void>('db_update_permissions', { userId, permissionsJson });
    },
  },

  // -------------------------------------------------------------------------
  // 2. Tasks & Goals
  // -------------------------------------------------------------------------
  tasks: {
    list: async (status?: string, priority?: number): Promise<TaskOrGoal[]> => {
      try {
        return await invokeTauri<TaskOrGoal[]>('db_list_tasks', { status, priority });
      } catch {
        return [];
      }
    },

    create: async (task: Partial<TaskOrGoal> & { title: string }): Promise<TaskOrGoal> => {
      const fullTask: TaskOrGoal = {
        id: task.id || `task-${Date.now()}`,
        parent_goal_id: task.parent_goal_id || null,
        title: task.title,
        subject_tag: task.subject_tag || null,
        priority: task.priority || 2,
        status: task.status || 'pending',
        target_score_weight: task.target_score_weight || 1.0,
        scheduled_start: task.scheduled_start || null,
        scheduled_end: task.scheduled_end || null,
        completed_at: task.completed_at || null,
        xp_reward: task.xp_reward || 10,
        updated_at: new Date().toISOString(),
      };
      return await invokeTauri<TaskOrGoal>('db_create_task', { task: fullTask });
    },

    update: async (task: TaskOrGoal): Promise<TaskOrGoal> => {
      return await invokeTauri<TaskOrGoal>('db_update_task', { task });
    },

    complete: async (taskId: string): Promise<TaskOrGoal> => {
      return await invokeTauri<TaskOrGoal>('db_complete_task', { taskId });
    },

    delete: async (taskId: string): Promise<void> => {
      return await invokeTauri<void>('db_delete_task', { taskId });
    },
  },

  // -------------------------------------------------------------------------
  // 3. Habits
  // -------------------------------------------------------------------------
  habits: {
    list: async (): Promise<Habit[]> => {
      try {
        return await invokeTauri<Habit[]>('db_list_habits');
      } catch {
        return [];
      }
    },

    create: async (habit: Partial<Habit> & { title: string }): Promise<Habit> => {
      const fullHabit: Habit = {
        id: habit.id || `habit-${Date.now()}`,
        title: habit.title,
        category: habit.category || 'wellness',
        frequency_json: habit.frequency_json || '{"type":"daily"}',
        current_streak: habit.current_streak || 0,
        best_streak: habit.best_streak || 0,
        last_completed_date: habit.last_completed_date || null,
        updated_at: new Date().toISOString(),
      };
      return await invokeTauri<Habit>('db_create_habit', { habit: fullHabit });
    },

    update: async (habit: Habit): Promise<Habit> => {
      return await invokeTauri<Habit>('db_update_habit', { habit });
    },

    recordCompletion: async (habitId: string): Promise<Habit> => {
      return await invokeTauri<Habit>('db_record_habit_completion', { habitId });
    },

    delete: async (habitId: string): Promise<void> => {
      return await invokeTauri<void>('db_delete_habit', { habitId });
    },
  },

  // -------------------------------------------------------------------------
  // 4. Wellbeing (Journal, Mood, Focus)
  // -------------------------------------------------------------------------
  wellbeing: {
    listJournals: async (limit = 50): Promise<JournalEntry[]> => {
      try {
        return await invokeTauri<JournalEntry[]>('db_list_journals', { limit });
      } catch {
        return [];
      }
    },

    createJournal: async (
      entry: Partial<JournalEntry> & { content_md: string }
    ): Promise<JournalEntry> => {
      const fullEntry: JournalEntry = {
        id: entry.id || `journal-${Date.now()}`,
        title: entry.title || null,
        content_md: entry.content_md,
        gratitude_items_json: entry.gratitude_items_json || '[]',
        mood_tag: entry.mood_tag || null,
        energy_level: entry.energy_level || null,
        is_encrypted: entry.is_encrypted || 0,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
      return await invokeTauri<JournalEntry>('db_create_journal', { entry: fullEntry });
    },

    listMoods: async (limit = 50): Promise<MoodCheckin[]> => {
      try {
        return await invokeTauri<MoodCheckin[]>('db_list_moods', { limit });
      } catch {
        return [];
      }
    },

    createMood: async (
      checkin: Omit<MoodCheckin, 'id' | 'created_at'>
    ): Promise<MoodCheckin> => {
      const fullCheckin: MoodCheckin = {
        id: `mood-${Date.now()}`,
        mood: checkin.mood,
        energy_tier: checkin.energy_tier,
        context_note: checkin.context_note || null,
        active_activity: checkin.active_activity || null,
        created_at: new Date().toISOString(),
      };
      return await invokeTauri<MoodCheckin>('db_create_mood', { checkin: fullCheckin });
    },

    listFocusSessions: async (limit = 50): Promise<FocusSession[]> => {
      try {
        return await invokeTauri<FocusSession[]>('db_list_focus_sessions', { limit });
      } catch {
        return [];
      }
    },

    startFocusSession: async (
      session: Omit<FocusSession, 'id' | 'started_at' | 'actual_minutes' | 'interruption_count' | 'completed' | 'ended_at'>
    ): Promise<FocusSession> => {
      const fullSession: FocusSession = {
        id: `focus-${Date.now()}`,
        profile_mode: session.profile_mode,
        planned_minutes: session.planned_minutes,
        actual_minutes: 0,
        task_id: session.task_id || null,
        interruption_count: 0,
        completed: 0,
        started_at: new Date().toISOString(),
        ended_at: null,
      };
      return await invokeTauri<FocusSession>('db_start_focus_session', { session: fullSession });
    },

    endFocusSession: async (
      sessionId: string,
      actualMinutes: number,
      interruptionCount: number
    ): Promise<void> => {
      return await invokeTauri<void>('db_end_focus_session', {
        sessionId,
        actualMinutes,
        interruptionCount,
      });
    },
  },

  // -------------------------------------------------------------------------
  // 5. Vector Engine
  // -------------------------------------------------------------------------
  vectors: {
    insert: async (
      id: string,
      sourceType: string,
      sourceId: string,
      embedding: number[]
    ): Promise<void> => {
      return await invokeTauri<void>('db_insert_vector', {
        id,
        sourceType,
        sourceId,
        embedding,
      });
    },

    search: async (
      queryVec: number[],
      topK = 5,
      sourceFilter?: string
    ): Promise<VectorSearchResult[]> => {
      try {
        return await invokeTauri<VectorSearchResult[]>('db_search_vectors', {
          queryVec,
          topK,
          sourceFilter,
        });
      } catch {
        return [];
      }
    },

    purgeCache: async (): Promise<number> => {
      return await invokeTauri<number>('db_purge_vector_cache');
    },
  },

  // -------------------------------------------------------------------------
  // 6. Diagnostics & Integrity Doctor
  // -------------------------------------------------------------------------
  diagnostics: {
    runIntegrityCheckAndRepair: async (): Promise<DbIntegrityReport> => {
      return await invokeTauri<DbIntegrityReport>('db_run_integrity_check_and_repair');
    },
  },
};

