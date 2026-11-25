use std::env;
use std::path::PathBuf;
use std::process::exit;

use whole_inspector_plugin::models::InspectorConfig;
use whole_inspector_plugin::report::ReportGenerator;
use whole_inspector_plugin::run_whole_inspection;

fn print_help() {
    println!(
        r#"MindSet OS Whole Inspector Plugin CLI

USAGE:
    whole_inspector_plugin [OPTIONS]

OPTIONS:
    --root <DIR>       Path to workspace root (defaults to '.')
    --md <PATH>        Output Markdown report path (e.g. WHOLE_INSPECTOR_REPORT.md)
    --json <PATH>      Output JSON report path (e.g. whole_inspector_report.json)
    --run-cargo        Execute live cargo checks and tests
    --verbose          Enable detailed inspection logs
    -h, --help         Print this help message

EXIT CODES:
    0                  Demo & OS Ready (all capability scores >= 80, 0 critical blockers)
    1                  Non-critical warnings found (all capability scores >= 60)
    2                  Critical blockers found (immediate fix required)
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = InspectorConfig::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                if i + 1 < args.len() {
                    config.workspace_root = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "--md" => {
                if i + 1 < args.len() {
                    config.output_md = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--json" => {
                if i + 1 < args.len() {
                    config.output_json = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--run-cargo" => {
                config.run_cargo = true;
            }
            "--verbose" => {
                config.verbose = true;
            }
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            unknown => {
                eprintln!("Unknown argument: {}", unknown);
                print_help();
                exit(1);
            }
        }
        i += 1;
    }

    let root = &config.workspace_root;
    if !root.exists() {
        eprintln!("Error: Target root does not exist: {}", root.display());
        exit(2);
    }

    println!("================================================================================");
    println!("🔍 MindSet OS Whole Inspector Plugin: Deep Workspace Verification");
    println!(
        "Target Root: {}",
        root.canonicalize()
            .unwrap_or_else(|_| root.clone())
            .display()
    );
    println!("Mode: Strict Read-Only Deep Diagnostic");
    println!("================================================================================\n");

    let report = run_whole_inspection(root, config.clone());

    // Print summary to stdout
    let s = &report.executive_summary;
    println!("--------------------------------------------------------------------------------");
    println!("EXECUTIVE VERDICT: {}", s.verdict_status);
    println!("Exit Code: {}", report.exit_code);
    println!("Overall Readiness Score: {} / 100", s.overall_score);
    println!("--------------------------------------------------------------------------------");
    println!(
        "  1. Standard OS Capability:       {:>3} / 100",
        s.normal_os_score
    );
    println!(
        "  2. Bootable Debian 13 ISO:       {:>3} / 100",
        s.iso_readiness_score
    );
    println!(
        "  3. Student OS + Maindy AI:       {:>3} / 100",
        s.student_os_ai_score
    );
    println!("--------------------------------------------------------------------------------");
    println!(
        "Features Audited: {} | Pass: {} | Partial: {} | Mock: {} | Missing: {}",
        s.total_features_checked, s.pass_count, s.partial_count, s.mock_count, s.missing_count
    );
    println!(
        "Action Items: Critical: {} | High: {} | Medium: {} | Low: {}",
        s.critical_count, s.high_count, s.medium_count, s.low_count
    );
    println!("--------------------------------------------------------------------------------\n");

    if !report.action_queue.is_empty() {
        println!("PRIORITIZED ACTION QUEUE:");
        for (idx, action) in report.action_queue.iter().enumerate() {
            println!(
                "  [{}] {}: {} ({})",
                action.severity.badge(),
                idx + 1,
                action.title,
                action.location
            );
        }
        println!();
    }

    // Save artifacts if requested
    if let Err(e) = ReportGenerator::save_artifacts(
        &report,
        config.output_md.as_deref(),
        config.output_json.as_deref(),
    ) {
        eprintln!("Error saving reports: {}", e);
        exit(2);
    }

    if let Some(md_p) = &config.output_md {
        println!("📄 Markdown Report saved to: {}", md_p.display());
    }
    if let Some(json_p) = &config.output_json {
        println!("📊 JSON Report saved to: {}", json_p.display());
    }

    println!("\n✅ Inspection completed successfully.");
    exit(report.exit_code);
}
