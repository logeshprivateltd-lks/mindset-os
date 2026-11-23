//! Structured Agent Tool Protocol for MindSet OS
//!
//! Provides JSON schema definitions and parser for OS agent actions emitted
//! by Maindy AI inside `<os_action>{...}</os_action>` tags.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", content = "params")]
pub enum AgentAction {
    /// Automatically generates a new Study Capsule, adds it to the Daily Capsule Queue,
    /// and launches it in the Study Session Reader.
    CreateAndShowCapsule {
        subject: String,
        topic: String,
        duration_mins: u32,
        priority: String,
        notes_summary: String,
    },
    /// Inspects and lists filenames within an authorized student directory.
    ListFolder {
        path: String,
    },
    /// Reads a local text/markdown/note document (capped at 16 KB) to synthesize study notes.
    ReadDocument {
        file_path: String,
    },
    /// Opens a MindSet OS native app window.
    OpenApp {
        app_id: String,
    },
    /// System control commands: themes, language, focus sessions, or browser navigation.
    ControlSystem {
        action: SystemControlAction,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value")]
pub enum SystemControlAction {
    ToggleTheme(String),
    SwitchLanguage(String),
    StartPomodoro { duration_mins: u32 },
    OpenBrowserUrl(String),
}

/// Checks whether an action requires explicit user permission approval.
///
/// CRITICAL RULE: Maindy AI MUST NEVER read any folder, open any file, or
/// mutate OS settings silently. External video & internet applications also
/// require explicit student authorization.
pub fn requires_permission(action: &AgentAction) -> bool {
    match action {
        AgentAction::ListFolder { .. } => true,
        AgentAction::ReadDocument { .. } => true,
        AgentAction::ControlSystem { .. } => true,
        AgentAction::CreateAndShowCapsule { .. } => false,
        AgentAction::OpenApp { app_id } => {
            let id = app_id.to_lowercase();
            matches!(
                id.as_str(),
                "youtube"
                    | "youtube_study"
                    | "yt"
                    | "freetube"
                    | "browser"
                    | "mindset_browser"
                    | "chromium"
            )
        }
    }
}

/// Returns a human-readable title and target description for the permission sheet.
pub fn action_description(action: &AgentAction) -> (&'static str, String) {
    match action {
        AgentAction::ListFolder { path } => ("List Folder Contents", path.clone()),
        AgentAction::ReadDocument { file_path } => ("Read Document & Notes", file_path.clone()),
        AgentAction::ControlSystem { action } => match action {
            SystemControlAction::ToggleTheme(mode) => {
                ("Change Desktop Theme", format!("Switch theme to {}", mode))
            }
            SystemControlAction::SwitchLanguage(lang) => {
                ("Change UI Language", format!("Switch language to {}", lang))
            }
            SystemControlAction::StartPomodoro { duration_mins } => (
                "Start Focus Pomodoro",
                format!("Set focus timer for {} minutes", duration_mins),
            ),
            SystemControlAction::OpenBrowserUrl(url) => {
                ("Open Browser Destination", url.clone())
            }
        },
        AgentAction::CreateAndShowCapsule { subject, topic, .. } => (
            "Create & Show Study Capsule",
            format!("{} • {}", subject, topic),
        ),
        AgentAction::OpenApp { app_id } => {
            let id = app_id.to_lowercase();
            match id.as_str() {
                "youtube" | "youtube_study" | "yt" => (
                    "Launch YouTube Desktop App",
                    "Open real installed YouTube application for lecture study".into(),
                ),
                "freetube" => (
                    "Launch FreeTube (Ad-Free)",
                    "Open FreeTube ad-free desktop player for distraction-free study".into(),
                ),
                "browser" | "mindset_browser" | "chromium" => (
                    "Launch MindSet Browser",
                    "Open Chromium browser with student research profile".into(),
                ),
                _ => ("Open MindSet App", app_id.clone()),
            }
        }
    }
}

/// Extracts any `<os_action>{...}</os_action>` payload from the LLM output.
/// Returns the parsed `Option<AgentAction>` and the cleaned display text with tags removed.
pub fn extract_agent_action(raw: &str) -> (Option<AgentAction>, String) {
    let start_tag = "<os_action>";
    let end_tag = "</os_action>";

    if let Some(start_idx) = raw.find(start_tag) {
        let after_start = &raw[start_idx + start_tag.len()..];
        if let Some(end_idx) = after_start.find(end_tag) {
            let json_str = after_start[..end_idx].trim();

            let action = serde_json::from_str::<AgentAction>(json_str).ok();

            // Strip the <os_action>...</os_action> block from visible chat text
            let before = &raw[..start_idx];
            let after = &after_start[end_idx + end_tag.len()..];
            let mut cleaned = format!("{}{}", before, after);
            cleaned = cleaned.trim().to_string();

            return (action, cleaned);
        }
    }

    (None, raw.trim().to_string())
}

/// System prompt extension instructing Maindy AI how and when to invoke OS actions
pub fn agent_tools_system_prompt() -> &'static str {
    r#"=== MINDSET OS AGENT TOOLS ===
You are an authorized MindSet OS Agent. When the student asks you to create study capsules, inspect notes, or launch apps, you can emit an OS tool action block wrapped inside `<os_action>{...}</os_action>` tags alongside your friendly chat reply.

Available Action Schemas:
1. Create and Show Study Capsule:
<os_action>
{
  "action": "CreateAndShowCapsule",
  "params": {
    "subject": "Physics",
    "topic": "Photoelectric Effect & Work Function",
    "duration_mins": 30,
    "priority": "High",
    "notes_summary": "Einstein equation: hf = W + KE_max. Key focus on stopping potential."
  }
}
</os_action>

2. Read Document / Student Notes:
<os_action>
{
  "action": "ReadDocument",
  "params": {
    "file_path": "Formula Sheets/Physics_HighYield_Formulas.txt"
  }
}
</os_action>

3. List Folder Contents:
<os_action>
{
  "action": "ListFolder",
  "params": {
    "path": "Study Capsules"
  }
}
</os_action>

4. Open Native App Window:
<os_action>
{
  "action": "OpenApp",
  "params": {
    "app_id": "FileVault"
  }
}
</os_action>
(Supported app_ids: "FileVault", "MindSetBrowser", "YouTubeStudy", "Preferences", "VerificationBoss", "SystemMonitor", "Tasks", "StudyReader")

5. Control System (Theme / Focus Timer / Browser):
<os_action>
{
  "action": "ControlSystem",
  "params": {
    "action": {
      "type": "StartPomodoro",
      "value": { "duration_mins": 25 }
    }
  }
}
</os_action>

CRITICAL PERMISSION RULE:
Whenever you emit ListFolder, ReadDocument, or ControlSystem, the OS will automatically pause and ask the student for explicit approval before reading files or changing settings. Never apologize for asking permission."#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_create_capsule_action() {
        let input = "Vanakkam mapla! I have created the capsule for you.\n\n\
        <os_action>\n\
        {\n\
          \"action\": \"CreateAndShowCapsule\",\n\
          \"params\": {\n\
            \"subject\": \"Chemistry\",\n\
            \"topic\": \"Electrochemistry Nernst Equation\",\n\
            \"duration_mins\": 45,\n\
            \"priority\": \"High\",\n\
            \"notes_summary\": \"Nernst equation application at 298 K\"\n\
          }\n\
        }\n\
        </os_action>\n\
        Let's master this concept now!";

        let (action, cleaned) = extract_agent_action(input);
        assert!(action.is_some());
        match action.unwrap() {
            AgentAction::CreateAndShowCapsule {
                subject,
                topic,
                duration_mins,
                ..
            } => {
                assert_eq!(subject, "Chemistry");
                assert_eq!(topic, "Electrochemistry Nernst Equation");
                assert_eq!(duration_mins, 45);
            }
            _ => panic!("Expected CreateAndShowCapsule"),
        }

        assert!(!cleaned.contains("<os_action>"));
        assert!(!cleaned.contains("</os_action>"));
        assert!(cleaned.contains("Vanakkam mapla!"));
        assert!(cleaned.contains("Let's master this concept now!"));
    }

    #[test]
    fn test_permission_requirement_checks() {
        let read_act = AgentAction::ReadDocument {
            file_path: "Formula Sheets/Physics.txt".into(),
        };
        assert!(requires_permission(&read_act));

        let list_act = AgentAction::ListFolder {
            path: "Documents".into(),
        };
        assert!(requires_permission(&list_act));

        let ctrl_act = AgentAction::ControlSystem {
            action: SystemControlAction::ToggleTheme("DarkSpace".into()),
        };
        assert!(requires_permission(&ctrl_act));

        let capsule_act = AgentAction::CreateAndShowCapsule {
            subject: "Physics".into(),
            topic: "Gauss Law".into(),
            duration_mins: 30,
            priority: "High".into(),
            notes_summary: "Flux calculation".into(),
        };
        assert!(!requires_permission(&capsule_act));

        let app_act = AgentAction::OpenApp {
            app_id: "FileVault".into(),
        };
        assert!(!requires_permission(&app_act));
    }
}
