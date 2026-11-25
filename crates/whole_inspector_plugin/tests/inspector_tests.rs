use std::fs;
use std::path::PathBuf;
use whole_inspector_plugin::analyzers::WorkspaceContext;
use whole_inspector_plugin::analyzers::{
    ai_agent_tts, files_hygiene, iso_readiness, ui_theme_i18n,
};
use whole_inspector_plugin::models::{ActionSeverity, CapabilityStatus, InspectorConfig};
use whole_inspector_plugin::report::ReportGenerator;
use whole_inspector_plugin::run_whole_inspection;

fn create_temp_test_dir(test_name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique_id = format!("mindset_test_{}_{}", test_name, std::process::id());
    dir.push(unique_id);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("Failed to create temp test dir");
    dir
}

#[test]
fn test_detection_of_leftover_tsx_files() {
    let temp = create_temp_test_dir("tsx_detection");
    let tsx_file = temp.join("App.tsx");
    fs::write(
        &tsx_file,
        "export default function App() { return <div>Legacy</div>; }",
    )
    .expect("Failed to write mock tsx");

    let ctx = WorkspaceContext::scan(&temp);
    let result = files_hygiene::audit(&ctx);

    let clutter_action = result
        .actions
        .iter()
        .find(|a| a.title.contains("legacy web/Tauri clutter"));

    assert!(
        clutter_action.is_some(),
        "Leftover .tsx file was not detected by files_hygiene analyzer"
    );
    let action = clutter_action.unwrap();
    assert_eq!(action.severity, ActionSeverity::High);
    assert!(action.location.contains("App.tsx"));

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_detection_of_crlf_in_sh_scripts() {
    let temp = create_temp_test_dir("crlf_detection");
    let sh_file = temp.join("build_mindset_iso.sh");
    // Write CRLF content explicitly
    let crlf_bytes = b"#!/bin/sh\r\necho \"Building ISO\"\r\nexit 0\r\n";
    fs::write(&sh_file, crlf_bytes).expect("Failed to write crlf script");

    let ctx = WorkspaceContext::scan(&temp);
    let result = iso_readiness::audit(&ctx);

    let crlf_action = result
        .actions
        .iter()
        .find(|a| a.title.contains("CRLF line endings"));

    assert!(
        crlf_action.is_some(),
        "CRLF script was not detected by iso_readiness analyzer"
    );
    let action = crlf_action.unwrap();
    assert_eq!(action.severity, ActionSeverity::Critical);
    assert!(action.location.contains("build_mindset_iso.sh"));

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_detection_of_missing_i18n_keys() {
    let temp = create_temp_test_dir("i18n_missing");
    let i18n_dir = temp.join("bins/mindset-os/src/i18n");
    fs::create_dir_all(&i18n_dir).expect("Failed to create i18n dir");

    let en_json = r#"{"key1": "Hello", "extra_key": "Only in English"}"#;
    let ta_json = r#"{"key1": "வணக்கம்"}"#;
    let ml_json = r#"{"key1": "നമസ്കാരം"}"#;

    fs::write(i18n_dir.join("en.json"), en_json).expect("write en.json");
    fs::write(i18n_dir.join("ta.json"), ta_json).expect("write ta.json");
    fs::write(i18n_dir.join("ml.json"), ml_json).expect("write ml.json");

    let ctx = WorkspaceContext::scan(&temp);
    let result = ui_theme_i18n::audit(&ctx);

    let i18n_entry = result
        .entries
        .iter()
        .find(|e| e.feature.contains("Multi-Language"));

    assert!(i18n_entry.is_some());
    assert_eq!(i18n_entry.unwrap().status, CapabilityStatus::Partial);

    let i18n_action = result
        .actions
        .iter()
        .find(|a| a.title.contains("missing localization keys"));
    assert!(i18n_action.is_some());
    assert!(i18n_action.unwrap().finding.contains("extra_key"));

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_detection_of_hardcoded_mock_strings() {
    let temp = create_temp_test_dir("mock_strings");
    let ui_file = temp.join("bins/mindset-os/src/shell/test_view.rs");
    fs::create_dir_all(ui_file.parent().unwrap()).expect("create dir");

    let code = r#"
        pub fn show_weather() {
            let city = "Bratislava";
            println!("Weather in {}", city);
        }
    "#;
    fs::write(&ui_file, code).expect("write test_view.rs");

    let ctx = WorkspaceContext::scan(&temp);
    let result = ui_theme_i18n::audit(&ctx);

    let mock_entry = result
        .entries
        .iter()
        .find(|e| e.feature.contains("Mock String Purge"));

    assert!(mock_entry.is_some());
    assert_eq!(mock_entry.unwrap().status, CapabilityStatus::Mock);

    let mock_action = result
        .actions
        .iter()
        .find(|a| a.finding.contains("Bratislava"));
    assert!(mock_action.is_some());
    assert!(mock_action.unwrap().location.contains("test_view.rs"));

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_detection_of_unguarded_ai_permission_gate() {
    let temp = create_temp_test_dir("unguarded_ai");
    let intel_dir = temp.join("crates/mindset-intelligence/src");
    fs::create_dir_all(&intel_dir).expect("create intel dir");

    // Intentionally omit permission gate check
    let broken_tools = r#"
        pub enum AgentAction { ReadDocument { file_path: String } }
        pub fn requires_permission(_action: &AgentAction) -> bool { false }
    "#;
    fs::write(intel_dir.join("agent_tools.rs"), broken_tools).expect("write agent_tools.rs");

    let ctx = WorkspaceContext::scan(&temp);
    let result = ai_agent_tts::audit(&ctx);

    let perm_action = result
        .actions
        .iter()
        .find(|a| a.title.contains("permission modal gate"));

    assert!(
        perm_action.is_some(),
        "Incomplete permission gate was not flagged as critical"
    );
    assert_eq!(perm_action.unwrap().severity, ActionSeverity::Critical);

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_end_to_end_mock_workspace_report_generation() {
    let temp = create_temp_test_dir("e2e_report");
    fs::write(temp.join("Cargo.toml"), "[workspace]\nmembers = []\n").expect("write root cargo");

    let config = InspectorConfig {
        workspace_root: temp.clone(),
        run_cargo: false,
        output_md: Some(temp.join("TEST_REPORT.md")),
        output_json: Some(temp.join("test_report.json")),
        verbose: false,
    };

    let report = run_whole_inspection(&temp, config.clone());

    assert_eq!(report.module_results.len(), 7);
    assert!(!report.capability_matrix.is_empty());

    // Verify artifact saving
    assert!(ReportGenerator::save_artifacts(
        &report,
        config.output_md.as_deref(),
        config.output_json.as_deref()
    )
    .is_ok());

    assert!(temp.join("TEST_REPORT.md").exists());
    assert!(temp.join("test_report.json").exists());

    let md_content = fs::read_to_string(temp.join("TEST_REPORT.md")).expect("read md");
    assert!(md_content.contains("MindSet OS: Whole System Diagnostic"));
    assert!(md_content.contains("Comprehensive Capability Matrix Table"));

    let _ = fs::remove_dir_all(&temp);
}
