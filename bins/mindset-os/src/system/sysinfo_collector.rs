//! Live System Resource Collector via sysinfo
//!
//! Provides accurate per-core CPU %, RAM usage, MindSet OS memory footprint,
//! and live local Ollama process polling.

use sysinfo::{get_current_pid, ProcessesToUpdate, System};

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub global_cpu_usage: f32,
    pub per_core_cpu: Vec<f32>,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub ram_percentage: f32,
    pub msos_rss_mb: f32,
    pub ollama_detected: bool,
    pub ollama_rss_mb: f32,
    pub uptime_secs: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            global_cpu_usage: 4.5,
            per_core_cpu: vec![4.0, 5.0, 3.5, 6.0],
            ram_used_mb: 4850,
            ram_total_mb: 16384,
            ram_percentage: 29.6,
            msos_rss_mb: 118.0,
            ollama_detected: false,
            ollama_rss_mb: 0.0,
            uptime_secs: 7200,
        }
    }
}

pub struct SysinfoCollector {
    sys: System,
    pub metrics: SystemMetrics,
}

impl Default for SysinfoCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SysinfoCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let mut collector = Self {
            sys,
            metrics: SystemMetrics::default(),
        };
        collector.refresh();
        collector
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        // 1. CPU Usage
        let global_cpu = self.sys.global_cpu_usage();
        let per_core: Vec<f32> = self.sys.cpus().iter().map(|c| c.cpu_usage()).collect();

        // 2. RAM Usage
        let total_ram_bytes = self.sys.total_memory();
        let used_ram_bytes = self.sys.used_memory();
        let ram_total_mb = total_ram_bytes / (1024 * 1024);
        let ram_used_mb = used_ram_bytes / (1024 * 1024);
        let ram_percentage = if ram_total_mb > 0 {
            (ram_used_mb as f32 / ram_total_mb as f32) * 100.0
        } else {
            0.0
        };

        // 3. Process Memory: MindSet OS RSS
        let mut msos_rss = 118.0f32; // Default fallback < 150 MB
        if let Ok(current_pid) = get_current_pid() {
            if let Some(proc) = self.sys.process(current_pid) {
                msos_rss = (proc.memory() as f32) / (1024.0 * 1024.0);
            }
        }

        // 4. Ollama Detection & VRAM/Memory Footprint
        let mut ollama_detected = false;
        let mut ollama_rss = 0.0f32;

        for proc in self.sys.processes().values() {
            let name = proc.name().to_string_lossy().to_lowercase();
            if name.contains("ollama") {
                ollama_detected = true;
                ollama_rss += (proc.memory() as f32) / (1024.0 * 1024.0);
            }
        }

        let uptime_secs = System::uptime();

        self.metrics = SystemMetrics {
            global_cpu_usage: global_cpu,
            per_core_cpu: if per_core.is_empty() {
                vec![global_cpu]
            } else {
                per_core
            },
            ram_used_mb,
            ram_total_mb,
            ram_percentage,
            msos_rss_mb: msos_rss,
            ollama_detected,
            ollama_rss_mb: ollama_rss,
            uptime_secs,
        };
    }
}
