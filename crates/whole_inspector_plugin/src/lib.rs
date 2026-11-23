use std::path::Path;

pub mod analyzers;
pub mod models;
pub mod report;

pub use models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ExecutiveSummary, InspectorConfig,
    ModuleAuditResult, PrioritizedAction, WholeInspectionReport,
};
pub use report::ReportGenerator;

/// Library Plugin API: Deeply inspects the MindSet OS workspace in strict read-only mode.
/// Produces a comprehensive, serializable diagnostic report with exact `file:line` references.
pub fn run_whole_inspection(root: &Path, config: InspectorConfig) -> WholeInspectionReport {
    let ctx = analyzers::WorkspaceContext::scan(root);

    let m1 = analyzers::files_hygiene::audit(&ctx);
    let m2 = analyzers::deps_build::audit(&ctx, config.run_cargo);
    let m3 = analyzers::os_capabilities::audit(&ctx);
    let m4 = analyzers::iso_readiness::audit(&ctx);
    let m5 = analyzers::ui_theme_i18n::audit(&ctx);
    let m6 = analyzers::ai_agent_tts::audit(&ctx);
    let m7 = analyzers::student_features::audit(&ctx);

    let modules = vec![m1, m2, m3, m4, m5, m6, m7];
    ReportGenerator::compile(root, modules)
}
