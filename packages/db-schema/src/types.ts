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

export interface VectorSearchResult {
  id: string;
  source_type: string;
  source_id: string;
  distance: number;
  similarity: number;
}

export interface DbIntegrityReport {
  core_integrity: string;
  oplog_integrity: string;
  core_size_bytes: number;
  oplog_size_bytes: number;
  repaired: boolean;
  timestamp_utc: string;
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

export interface AuthSession {
  session_token: string;
  user_id: string;
  authenticated_at: string;
  expires_at: string;
  auth_method: string;
}

export interface AuthStatus {
  is_locked: boolean;
  is_privacy_blurred: boolean;
  is_uac_elevated: boolean;
  active_user: string | null;
  auth_method: string | null;
  auto_lock_minutes: number;
}

export interface BiometricMatchResult {
  success: boolean;
  match_score: number;
  inference_latency_ms: number;
  method: string;
}

export interface FingerprintResult {
  success: boolean;
  enrolled: boolean;
  message: string;
}

// -------------------------------------------------------------------------
// Hardware & Device Manager DTOs (/devices)
// -------------------------------------------------------------------------

export interface UsbDevice {
  id: string;
  name: string;
  device_type: 'keyboard' | 'mouse' | 'storage' | 'printer' | 'scanner' | 'audio' | 'camera' | 'network' | 'other' | string;
  vendor_id?: string | null;
  product_id?: string | null;
  connected_port?: string | null;
  is_removable: boolean;
  status: string;
}

export interface DisplayInfo {
  id: string;
  name: string;
  resolution: string;
  refresh_rate_hz: number;
  is_primary: boolean;
  scale_factor: number;
  connection_type: string;
}

export interface StorageDrive {
  id: string;
  name: string;
  mount_point: string;
  fs_type: string;
  total_bytes: number;
  available_bytes: number;
  used_bytes: number;
  used_percent: number;
  is_removable: boolean;
  is_system: boolean;
}

export interface DiskCleanupCategory {
  name: string;
  bytes: number;
  description: string;
}

export interface DiskCleanupReport {
  drive_mount: string;
  total_cleanable_bytes: number;
  categories: DiskCleanupCategory[];
  timestamp_utc: string;
  cleaned: boolean;
}

export interface PeripheralReport {
  usb_devices: UsbDevice[];
  displays: DisplayInfo[];
  storage_drives: StorageDrive[];
  keyboards_count: number;
  mice_count: number;
  printers_count: number;
  scanners_count: number;
  timestamp_utc: string;
}

export interface WifiAdapterInfo {
  adapter_name: string;
  description: string;
  mac_address: string;
  chipset: string;
  is_external_usb: boolean;
  is_connected: boolean;
  ssid?: string | null;
  link_quality_percent: number;
  signal_strength_dbm: number;
  driver_version: string;
  driver_status: string;
  ip_address?: string | null;
  gateway?: string | null;
  dns_servers: string[];
  hotspot_active: boolean;
}

export interface NetworkDiagnosticsReport {
  adapters: WifiAdapterInfo[];
  primary_adapter?: string | null;
  internet_reachable: boolean;
  dns_resolving: boolean;
  latency_ms?: number | null;
  is_external_wifi_detected: boolean;
  suggested_actions: string[];
  timestamp_utc: string;
}

export interface DriverResetResult {
  success: boolean;
  message: string;
  requires_elevation: boolean;
  steps_taken: string[];
}

export type GovernorProfileType = 'EcoStudy' | 'Balanced' | 'AiBoost';

export interface PowerThermalReport {
  active_profile: GovernorProfileType | string;
  display_name: string;
  profile_description: string;
  battery_percent?: number | null;
  is_charging: boolean;
  time_remaining_minutes?: number | null;
  battery_health_percent: number;
  cpu_temp_celsius: number;
  ram_temp_celsius?: number | null;
  fan_speed_rpm?: number | null;
  power_consumption_watts?: number | null;
  thermal_throttling: boolean;
  timestamp_utc: string;
}

export interface MediaDevice {
  id: string;
  name: string;
  kind: 'audio_input' | 'audio_output' | 'video_input' | string;
  is_default: boolean;
  is_fallback_virtual: boolean;
  status: string;
}

export interface MediaRoutingReport {
  audio_inputs: MediaDevice[];
  audio_outputs: MediaDevice[];
  video_inputs: MediaDevice[];
  active_audio_input_id: string;
  active_audio_output_id: string;
  active_video_input_id: string;
  mic_fallback_active: boolean;
  mic_fallback_label: string;
  camera_fallback_active: boolean;
  camera_fallback_label: string;
  bluetooth_fallback_active: boolean;
  bluetooth_fallback_label: string;
  timestamp_utc: string;
}

// -------------------------------------------------------------------------
// Task Manager, btop++ Resource Monitor & Terminal DTOs (/status)
// -------------------------------------------------------------------------

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_percent: number;
  memory_rss_mb: number;
  category: 'MindSet Core' | 'Local AI (Ollama)' | 'Study App' | 'Background Service' | string;
  disk_read_bytes_sec: number;
  disk_written_bytes_sec: number;
  is_capped: boolean;
  cgroup_limit_ram_mb?: number | null;
  cgroup_limit_cpu_percent?: number | null;
}

export interface SystemResourceSnapshot {
  os_shell_rss_mb: number;
  ollama_ai_rss_mb: number;
  total_system_memory_mb: number;
  free_system_memory_mb: number;
  available_for_ollama_mb: number;
  os_shell_ceiling_mb: number; // 120.0 MB
  cpu_total_percent: number;
  cpu_cores_percent: number[];
  disk_read_mbs: number;
  disk_write_mbs: number;
  processes: ProcessInfo[];
  timestamp_utc: string;
}

export interface ResourceLimitResult {
  pid: number;
  success: boolean;
  applied_ram_mb?: number | null;
  applied_cpu_percent?: number | null;
  mechanism: string;
  message: string;
}

export interface DoctorActionResult {
  routine: string;
  success: boolean;
  freed_mb: number;
  message: string;
  timestamp_utc: string;
}

export interface DoctorCheckItem {
  name: string;
  status: 'Healthy' | 'Optimized' | 'Warning' | string;
  detail: string;
}

export interface FullDoctorReport {
  health_score: number;
  status: 'Optimal' | 'Degraded' | 'Action Required' | string;
  checks: DoctorCheckItem[];
  actions_recommended: string[];
  total_freed_mb: number;
  timestamp_utc: string;
}

export interface TerminalCommandResult {
  command: string;
  output: string;
  exit_code: number;
  execution_time_ms: number;
}

// ============================================================================
// 12. ATOMIC OTA UPDATER & RECOVERY SUB-SYSTEM DTOs
// ============================================================================

export type UpdateChannelType = 'core_os' | 'critical_security' | 'ai_prompts' | 'offline_bundle';

export interface OSVersionInfo {
  os_name: string;
  os_version: string;
  codename: string;
  kernel_version: string;
  build_id: string;
  active_slot: string;
  inactive_slot: string;
  ostree_commit: string;
  signature_key_fingerprint: string;
  last_checked_at: string;
  update_in_progress: boolean;
  staged_update?: UpdatePackageInfo | null;
}

export interface UpdatePackageInfo {
  id: string;
  channel: string;
  version: string;
  title: string;
  release_notes: string[];
  size_bytes: number;
  checksum_sha256: string;
  signature_ed25519: string;
  published_at: string;
  requires_reboot: boolean;
  diff_type: string; // 'delta' | 'binary_patch' | 'json_manifest' | 'full_image'
  is_staged: boolean;
  is_applied: boolean;
}

export interface UpdateStageProgress {
  update_id: string;
  stage: string; // 'verifying_manifest' | 'downloading' | 'verifying_signature' | 'staging_ab_slot' | 'ready_to_apply' | 'applied' | 'failed'
  progress_percent: number;
  bytes_downloaded: number;
  total_bytes: number;
  status_message: string;
  signature_valid: boolean;
}

export interface OfflineBundleManifest {
  bundle_name: string;
  target_channel: string;
  version: string;
  created_by: string;
  description: string;
  file_count: number;
  total_size_bytes: number;
  signature_valid: boolean;
}

export interface SystemSnapshot {
  id: string;
  label: string;
  created_at: string;
  os_version: string;
  size_bytes: number;
  checksum_sha256: string;
  is_automatic: boolean;
  database_count: number;
  files_count: number;
}

export interface RecoveryActionResult {
  success: boolean;
  action: string;
  message: string;
  details: string[];
  timestamp: string;
  rollback_window_seconds?: number | null;
}

export interface DatabaseRepairReport {
  databases_checked: string[];
  integrity_status: string; // 'clean' | 'repaired' | 'corrupted'
  wal_truncated_mb: number;
  vacuum_recovered_bytes: number;
  indexes_rebuilt: number;
}

export interface DriverResetReport {
  wifi_adapter_reloaded: boolean;
  display_mode_reset: boolean;
  audio_sink_defaulted: boolean;
  details: string;
}

export interface OstreeDeploymentInfo {
  active_commit: string;
  rollback_commit: string;
  deployment_source: string;
  is_healthy: boolean;
  watchdog_armed: boolean;
  watchdog_seconds_remaining: number;
  pre_update_snapshot_id?: string | null;
}

export interface RollbackExecutionResult {
  success: boolean;
  previous_commit: string;
  active_commit: string;
  message: string;
  execution_time_ms: number;
  user_data_preserved: boolean;
}

export interface OfflinePatchResult {
  success: boolean;
  bundle_name: string;
  channel: string;
  version: string;
  author_authority: string;
  signature_ed25519: string;
  signature_valid: boolean;
  components_applied: number;
  message: string;
  applied_at: string;
}




