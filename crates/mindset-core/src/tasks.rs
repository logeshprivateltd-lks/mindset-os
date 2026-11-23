//! MindSet OS — Tasks, Goals & Syllabus Tracker Model (`tasks.rs`)
//! Pure Rust syllabus tracking with granular +10m study logging and progress state.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectQuota {
    pub name: String,
    pub icon: String,
    pub completed_minutes: u32,
    pub target_minutes: u32,
    pub color_hex: String,
}

impl SubjectQuota {
    pub fn new(name: &str, icon: &str, target_minutes: u32, color_hex: &str) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            completed_minutes: 0,
            target_minutes,
            color_hex: color_hex.to_string(),
        }
    }

    pub fn log_10m(&mut self) {
        self.completed_minutes += 10;
    }

    pub fn progress(&self) -> f32 {
        if self.target_minutes == 0 {
            0.0
        } else {
            (self.completed_minutes as f32 / self.target_minutes as f32).clamp(0.0, 1.0)
        }
    }

    pub fn display_hours(&self) -> String {
        let done_h = self.completed_minutes as f32 / 60.0;
        let tgt_h = self.target_minutes as f32 / 60.0;
        format!("{:.1}h / {:.0}h", done_h, tgt_h)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub completed_minutes: u32,
    pub target_minutes: u32,
    pub is_completed: bool,
}

impl TaskItem {
    pub fn new(id: &str, title: &str, subject: &str, target_minutes: u32) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            subject: subject.to_string(),
            completed_minutes: 0,
            target_minutes,
            is_completed: false,
        }
    }

    pub fn log_10m(&mut self) {
        self.completed_minutes += 10;
        if self.completed_minutes >= self.target_minutes {
            self.is_completed = true;
        }
    }

    pub fn toggle_completed(&mut self) {
        self.is_completed = !self.is_completed;
        if self.is_completed && self.completed_minutes < self.target_minutes {
            self.completed_minutes = self.target_minutes;
        }
    }

    pub fn progress(&self) -> f32 {
        if self.target_minutes == 0 {
            0.0
        } else {
            (self.completed_minutes as f32 / self.target_minutes as f32).clamp(0.0, 1.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskQuotaStore {
    pub subjects: Vec<SubjectQuota>,
    pub tasks: Vec<TaskItem>,
}

impl Default for TaskQuotaStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQuotaStore {
    pub fn new() -> Self {
        let subjects = vec![
            SubjectQuota {
                name: "Physics".to_string(),
                icon: "⚡".to_string(),
                completed_minutes: 90,
                target_minutes: 180, // 3h
                color_hex: "#38BDF8".to_string(),
            },
            SubjectQuota {
                name: "Chemistry".to_string(),
                icon: "🧪".to_string(),
                completed_minutes: 70,
                target_minutes: 150, // 2.5h
                color_hex: "#F59E0B".to_string(),
            },
            SubjectQuota {
                name: "Biology / CS".to_string(),
                icon: "🧬".to_string(),
                completed_minutes: 110,
                target_minutes: 180, // 3h
                color_hex: "#10B981".to_string(),
            },
        ];

        let tasks = vec![
            TaskItem {
                id: "t1".to_string(),
                title: "Derive Gauss Law for spherical charged shell".to_string(),
                subject: "Physics".to_string(),
                completed_minutes: 25,
                target_minutes: 25,
                is_completed: true,
            },
            TaskItem {
                id: "t2".to_string(),
                title: "Daniell cell EMF calculation at 298 K".to_string(),
                subject: "Chemistry".to_string(),
                completed_minutes: 10,
                target_minutes: 30,
                is_completed: false,
            },
            TaskItem {
                id: "t3".to_string(),
                title: "Huygens wavelets reflection & refraction proofs".to_string(),
                subject: "Physics".to_string(),
                completed_minutes: 0,
                target_minutes: 45,
                is_completed: false,
            },
            TaskItem {
                id: "t4".to_string(),
                title: "DNA Replication & Okazaki Fragments synthesis".to_string(),
                subject: "Biology / CS".to_string(),
                completed_minutes: 20,
                target_minutes: 40,
                is_completed: false,
            },
        ];

        Self { subjects, tasks }
    }

    /// Log study minutes directly to a subject quota.
    pub fn log_study_minutes(&mut self, subject_name: &str, minutes: u32) {
        if let Some(subject) = self.subjects.iter_mut().find(|s| s.name == subject_name) {
            subject.completed_minutes = subject.completed_minutes.saturating_add(minutes);
        }
    }

    /// Add 10 minutes to a task and its parent subject quota simultaneously.
    pub fn log_10m_for_task(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.log_10m();
            let subject_name = task.subject.clone();
            if let Some(subject) = self.subjects.iter_mut().find(|s| s.name == subject_name) {
                subject.log_10m();
            }
        }
    }

    /// Toggle task completion flag.
    pub fn toggle_task(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.toggle_completed();
        }
    }

    /// Add a new custom study task.
    pub fn add_task(&mut self, title: &str, subject: &str, target_minutes: u32) {
        let id = format!("task_{}", self.tasks.len() + 1);
        self.tasks
            .push(TaskItem::new(&id, title, subject, target_minutes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_quota_10m_logging() {
        let mut store = TaskQuotaStore::new();
        let initial_chem_m = store
            .subjects
            .iter()
            .find(|s| s.name == "Chemistry")
            .unwrap()
            .completed_minutes;
        let initial_task_m = store
            .tasks
            .iter()
            .find(|t| t.id == "t2")
            .unwrap()
            .completed_minutes;

        store.log_10m_for_task("t2");

        let updated_task_m = store
            .tasks
            .iter()
            .find(|t| t.id == "t2")
            .unwrap()
            .completed_minutes;
        let updated_chem_m = store
            .subjects
            .iter()
            .find(|s| s.name == "Chemistry")
            .unwrap()
            .completed_minutes;

        assert_eq!(updated_task_m, initial_task_m + 10);
        assert_eq!(updated_chem_m, initial_chem_m + 10);
    }

    #[test]
    fn test_task_completion_toggle() {
        let mut store = TaskQuotaStore::new();
        assert!(
            !store
                .tasks
                .iter()
                .find(|t| t.id == "t3")
                .unwrap()
                .is_completed
        );
        store.toggle_task("t3");
        assert!(
            store
                .tasks
                .iter()
                .find(|t| t.id == "t3")
                .unwrap()
                .is_completed
        );
    }
}
