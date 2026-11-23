---
name: whole-inspector-plugin
description: Audits the entire MindSet OS workspace across filesystem hygiene, dependencies, normal OS capabilities, Debian 13 ISO boot readiness, UI themes, permission-gated Maindy AI tools, and student vision. Produces evidence-backed verdicts with exact file:line references.
---

# 🔍 WHOLE_INSPECTOR_PLUGIN: MindSet OS Architectural & System Verifier

`whole_inspector_plugin` is a comprehensive, strict read-only diagnostic plugin and CLI engine for **MindSet OS**.
It deeply inspects the entire project repository—every crate, binary, service, UI view, AI agent tool, Debian 13 ISO pipeline, and student vision feature—and computes three executive readiness scores (`0–100`) backed by exact `file:line` references.

---

## 1. When to Use This Skill

Activate this skill when:
- Verifying whether MindSet OS is ready for live physical PC boot, VirtualBox/VMware testing, or executive demos.
- Auditing codebase cleanliness for leftover legacy web/Tauri clutter (`.tsx`, `.ts`, `node_modules`, `package.json`).
- Verifying cross-platform health (`#[cfg(target_os = "windows")]` vs `#[cfg(target_os = "linux")]`) and checking for Windows CRLF line endings in ISO build scripts.
- Checking Maindy AI agent tools to guarantee that directory reading, document access, and OS controls are strictly gated behind user permission prompts.
- Auditing Student OS vision features: dynamic Exam Scout blueprints, Proof-of-Study Boss Battle (>=70 pass threshold), and 3D Syllabus Odyssey Trail widgets.

---

## 2. Running the Inspector

### Standalone CLI Binary
Run directly from the root of the workspace:
```bash
cargo run -p whole_inspector_plugin -- --root . --md WHOLE_INSPECTOR_REPORT.md --json whole_inspector_report.json
```

### Library Plugin API (Inside MindSet OS)
```rust
use whole_inspector_plugin::{run_whole_inspection, InspectorConfig};

let config = InspectorConfig::default();
let report = run_whole_inspection(Path::new("."), config);
println!("Readiness Score: {} / 100", report.executive_summary.overall_score);
```

---

## 3. The 8 Inspection Modules

| Module | Scope & Guardrails |
| :--- | :--- |
| **Module 1: Filesystem & Hygiene** | Catalogs all workspace crates/lines; flags `.tsx`/`.ts`/Tauri clutter; checks font bundling (NotoSansTamil, NotoSansMalayalam); verifies `archive/` exclusion. |
| **Module 2: Dependencies & Build** | Audits `iced 0.14.0` features; checks release profile optimizations (`opt-level = "s"`, `lto = "thin"`, `panic = "abort"`); verifies platform `cfg` isolation. |
| **Module 3: Standard OS Capabilities** | Verifies boot splash (`HELLO ACHIEVER`), window dragging/resizing, viewport clamping, 3-zone top bar, real `std::fs` File Vault, Chromium/YouTube/FreeTube launch, and sysinfo monitoring. |
| **Module 4: Debian 13 ISO Readiness** | Audits `live-build` Trixie amd64 hybrid boot, bare-metal Wi-Fi firmware (`iwlwifi`, `realtek`, `atheros`), FreeTube `chmod 4755` fix, auto-login, VM GPU fallback, and CRLF script hygiene. |
| **Module 5: UI & Multi-Language** | Checks Neumorphic/Claymorphic design tokens; purges hardcoded mock strings (`Bratislava`, `NASDAQ`); verifies 100% i18n key parity across English, Tamil, and Malayalam. |
| **Module 6: Maindy AI & Security** | Verifies hybrid BYOK/Ollama/Offline streaming; verifies student memory injection; strictly audits permission gates (`Allow Once` / `Allow Session` / `Deny`) on file access; verifies Thanglish TTS. |
| **Module 7: Student OS Vision** | Verifies Exam Scout track blueprints, strict Proof-of-Study Boss Battle (>=70 pass threshold, NO fake mark-complete checkboxes), and 3D Syllabus Odyssey Trail. |
| **Module 8: Master Scoring Engine** | Calculates the 3 Executive Verdicts, formats the master Capability Matrix Table, and prioritizes remediation actions. |

---

## 4. Exit Code Contract

- **`0`**: Demo / OS Ready (No critical blockers, all 3 capability scores >= 80).
- **`1`**: Non-critical warnings (No critical blockers, scores >= 60).
- **`2`**: Critical blockers found (Immediate remediation required before ISO generation).
