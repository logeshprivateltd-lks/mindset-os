/**
 * MindSet OS — SQLite Database Schema TypeScript Interfaces
 * Strictly typed 1:1 mappings for local_core.sqlite & sync_oplog.sqlite
 * Powered by LKS PVT LTD
 */

export interface SchemaMigration {
  version: number;
  applied_at: string;
}

export interface UserProfile {
  user_id: string;
  display_name: string;
  role: 'student' | 'researcher' | 'professional' | 'admin';
  interface_lang: 'en' | 'ta' | 'hi' | 'te' | 'ml' | 'kn' | 'bn' | 'mr';
  ai_lang: 'en' | 'ta' | 'hi' | 'te' | 'ml' | 'kn' | 'bn' | 'mr';
  conversation_style: 'tanglish' | 'formal' | 'concise' | 'socratic';
  goals_json: string; // JSON array of goal strings or goal objects
  focus_areas_json: string; // JSON array of subject tags
  routine_json: string; // JSON timetable schema
  preferences_json: string; // Audio, notifications, ambient sound config
  interests_json: string; // JSON array of academic/personal interests
  permissions_json: string; // Feature flags & elevated mode toggles
  updated_at: string; // ISO 8601 UTC
}

export interface AuthCredentials {
  user_id: string;
  password_hash: string;
  pin_hash: string | null;
  face_embedding_blob: Uint8Array | null;
  fingerprint_enrolled: number; // 0 or 1
  auto_lock_minutes: number;
  failed_attempts: number;
  locked_until: string | null;
}

export interface TaskOrGoal {
  id: string;
  parent_goal_id: string | null;
  title: string;
  subject_tag: string | null;
  priority: 1 | 2 | 3; // 1 = Urgent/Crucial, 2 = Medium, 3 = Low
  status: 'pending' | 'in_progress' | 'completed' | 'archived';
  target_score_weight: number;
  scheduled_start: string | null; // ISO 8601
  scheduled_end: string | null; // ISO 8601
  completed_at: string | null;
  xp_reward: number;
  updated_at: string;
}

export interface Habit {
  id: string;
  title: string;
  category: 'wellness' | 'study' | 'mindset' | 'routine';
  frequency_json: string; // e.g. {"type":"daily"} or {"days":[1,2,3,4,5]}
  current_streak: number;
  best_streak: number;
  last_completed_date: string | null; // YYYY-MM-DD
  updated_at: string;
}

export interface JournalEntry {
  id: string;
  title: string | null;
  content_md: string;
  gratitude_items_json: string; // JSON array of 3 gratitude items
  mood_tag: string | null;
  energy_level: 'Low' | 'Medium' | 'High' | 'Hyper' | null;
  is_encrypted: number; // 0 or 1
  created_at: string;
  updated_at: string;
}

export interface MoodCheckin {
  id: string;
  mood: 'Energized' | 'Calm' | 'Focused' | 'Anxious' | 'Fatigued' | 'Overwhelmed';
  energy_tier: 'Low' | 'Medium' | 'High' | 'Hyper';
  context_note: string | null;
  active_activity: string | null;
  created_at: string;
}

export interface FocusSession {
  id: string;
  profile_mode: 'ExamPrep' | 'Coding' | 'Reading' | 'DeepWork' | 'Revision';
  planned_minutes: number;
  actual_minutes: number;
  task_id: string | null;
  interruption_count: number;
  completed: number; // 0 or 1
  started_at: string;
  ended_at: string | null;
}

export interface TimelineEvent {
  id: string;
  event_type: 'focus_session' | 'task_completion' | 'mood_logged' | 'habit_tick' | 'system_boot' | 'lock_screen';
  title: string;
  metadata_json: string;
  created_at: string;
}

export interface RagDocument {
  id: string;
  file_path: string;
  title: string;
  subject_tag: string | null;
  total_pages: number;
  last_read_page: number;
  indexed_at: string;
}

export interface RagChunk {
  id: string;
  document_id: string;
  page_number: number;
  chunk_index: number;
  content: string;
}

export interface VecMemoryFallback {
  id: string;
  embedding_json: string;
  source_type: string;
  source_id: string;
  created_at: string;
}

// -------------------------------------------------------------------------
// sync_oplog.sqlite Interfaces (Deterministic HLC Peer-to-Peer Sync)
// -------------------------------------------------------------------------

export type OplogOperation = 'INSERT' | 'UPDATE' | 'DELETE';

export interface OplogEntry {
  op_id: string;
  hlc_timestamp: string; // <epoch_ms>:<counter>:<device_id>
  device_id: string;
  table_name: string;
  row_id: string;
  operation: OplogOperation;
  payload_json: string;
  synced_devices_json: string; // JSON array of device_ids
}

export interface TrustedDevice {
  device_id: string;
  device_name: string;
  device_type: 'desktop' | 'mobile';
  public_key_ed25519: string;
  permissions_json: string;
  last_seen_at: string;
  is_revoked: number; // 0 or 1
}

// -------------------------------------------------------------------------
// System Hardware & Kernel DTOs
// -------------------------------------------------------------------------

export interface HardwareCapabilities {
  has_mic: boolean;
  has_webcam: boolean;
  has_fingerprint: boolean;
  has_bluetooth: boolean;
  has_wifi_monitor: boolean;
  wifi_adapter_name?: string | null;
  is_external_wifi?: boolean;
  detected_at_utc: string;
}

export type KernelBootPhase = 
  | 'BOOT_INIT'
  | 'DB_MIGRATE'
  | 'HARDWARE_PROBE'
  | 'SERVICE_REGISTER'
  | 'SYSTEM_READY';

export type ServiceStatusKind = 'Healthy' | 'Degraded' | 'Stopped' | 'Critical';

export interface ServiceHealthReport {
  name: string;
  status: ServiceStatusKind | { Degraded: string } | { Critical: string };
  status_message?: string;
  estimated_ram_kb: number;
  last_heartbeat_utc: string;
}

export interface RamBreakdown {
  os_shell_mb: number;
  ollama_mb: number;
  free_system_mb: number;
  total_system_mb: number;
  system_used_percent: number;
}

export interface KernelStatusResponse {
  boot_stage: string;
  uptime_secs: number;
  hardware: HardwareCapabilities;
  ram_breakdown: RamBreakdown;
  services: ServiceHealthReport[];
}

export interface EmergencyPurgeResult {
  freed_mb: number;
  current_rss_mb: number;
  timestamp_utc: string;
}

export type KernelEvent =
  | { type: 'SystemReady'; payload: { uptime_ms: number; hardware: HardwareCapabilities } }
  | { type: 'ServiceStateChanged'; payload: { service_name: string; status: string } }
  | { type: 'FocusSessionStateChanged'; payload: { session_id: string; state: string; elapsed_secs: number } }
  | { type: 'TaskCompleted'; payload: { task_id: string; xp_earned: number } }
  | { type: 'MoodLogged'; payload: { mood: string; energy_tier: string } }
  | { type: 'FreeTimeWindowDetected'; payload: { start_time: string; duration_minutes: number } }
  | { type: 'RamThresholdWarning'; payload: { process_rss_mb: number; system_used_percent: number } }
  | { type: 'EmergencyRamPurgeExecuted'; payload: { reclaimed_bytes: number; os_shell_rss_mb: number } }
  | { type: 'HardwareStateChanged'; payload: { capabilities: HardwareCapabilities } }
  | { type: 'SecurityLockTriggered'; payload: { reason: string } }
  | { type: 'SyncOplogAppended'; payload: { op_id: string; row_id: string; table_name: string } };

