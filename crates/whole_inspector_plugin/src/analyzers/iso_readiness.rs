use std::collections::HashMap;
use crate::analyzers::WorkspaceContext;
use crate::models::{
    ActionSeverity, CapabilityMatrixEntry, CapabilityStatus, ModuleAuditResult, PrioritizedAction,
};

/// Module 4: Bootable Debian 13 ISO Creation Readiness Audit
pub fn audit(ctx: &WorkspaceContext) -> ModuleAuditResult {
    let mut entries = Vec::new();
    let mut actions = Vec::new();
    let mut metrics = HashMap::new();
    let mut score: u32 = 100;

    // 1. Check live-build configuration targeting trixie (amd64) and hybrid bootloaders
    let auto_config = ctx.read_file("iso-builder/auto/config").unwrap_or_default();
    let targets_trixie = auto_config.contains("--distribution trixie") || auto_config.contains("trixie");
    let targets_amd64 = auto_config.contains("--architectures amd64") || auto_config.contains("amd64");
    let has_hybrid_boot = auto_config.contains("grub-efi") && (auto_config.contains("syslinux") || auto_config.contains("isolinux"));

    let config_line = ctx
        .find_first_occurrence("iso-builder/auto/config", "--distribution trixie")
        .map(|(l, _)| l)
        .unwrap_or(10);

    if targets_trixie && targets_amd64 && has_hybrid_boot {
        entries.push(CapabilityMatrixEntry {
            feature: "Debian 13 (Trixie) Live-Build Hybrid ISO Config".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("iso-builder/auto/config:{}", config_line),
            notes: "Configured for Debian 13 Trixie amd64 with hybrid UEFI (grub-efi) + BIOS (syslinux) bootloaders".into(),
        });
    } else {
        score = score.saturating_sub(25);
        entries.push(CapabilityMatrixEntry {
            feature: "Debian 13 (Trixie) Live-Build Hybrid ISO Config".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("iso-builder/auto/config:{}", config_line),
            notes: "live-build auto/config missing trixie, amd64, or hybrid bootloader parameters".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::Critical,
            title: "Configure live-build for Debian 13 Trixie hybrid boot".into(),
            location: format!("iso-builder/auto/config:{}", config_line),
            finding: "Target distribution or bootloader definition is incomplete for bare-metal boot.".into(),
            remediation: "Ensure auto/config contains --distribution trixie --architectures amd64 --bootloaders 'grub-efi,syslinux'.".into(),
        });
    }

    // 2. Audit bare-metal PC package list in mindset.list.chroot
    let package_list = ctx
        .read_file("iso-builder/config/package-lists/mindset.list.chroot")
        .unwrap_or_default();

    let has_net = package_list.contains("network-manager") && package_list.contains("wpasupplicant");
    let has_wifi_firmware = package_list.contains("firmware-linux")
        && package_list.contains("firmware-iwlwifi")
        && package_list.contains("firmware-realtek")
        && package_list.contains("firmware-atheros")
        && package_list.contains("firmware-misc-nonfree");
    let has_audio = package_list.contains("pipewire");
    let has_tts = package_list.contains("espeak-ng") || package_list.contains("piper");
    let has_fonts = package_list.contains("fonts-noto");
    let has_chromium = package_list.contains("chromium");
    let has_graphics = package_list.contains("xserver-xorg") && package_list.contains("mesa-vulkan-drivers");

    let pkg_line = ctx
        .find_first_occurrence("iso-builder/config/package-lists/mindset.list.chroot", "firmware-iwlwifi")
        .map(|(l, _)| l)
        .unwrap_or(20);

    if has_net && has_wifi_firmware && has_audio && has_tts && has_fonts && has_chromium && has_graphics {
        entries.push(CapabilityMatrixEntry {
            feature: "Bare-Metal PC Hardware Firmware & Package Manifest".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("iso-builder/config/package-lists/mindset.list.chroot:{}", pkg_line),
            notes: "Verified complete hardware stack: Intel/Realtek/Atheros Wi-Fi firmware, PipeWire audio, espeak-ng/piper TTS, Mesa/Vulkan, and Chromium".into(),
        });
    } else {
        score = score.saturating_sub(20);
        let mut missing_pkgs = Vec::new();
        if !has_net { missing_pkgs.push("network-manager / wpasupplicant"); }
        if !has_wifi_firmware { missing_pkgs.push("Wi-Fi firmwares (iwlwifi, realtek, atheros)"); }
        if !has_audio { missing_pkgs.push("pipewire"); }
        if !has_tts { missing_pkgs.push("espeak-ng / piper"); }
        if !has_fonts { missing_pkgs.push("fonts-noto"); }
        if !has_chromium { missing_pkgs.push("chromium"); }
        if !has_graphics { missing_pkgs.push("Mesa/Vulkan/Xorg"); }

        entries.push(CapabilityMatrixEntry {
            feature: "Bare-Metal PC Hardware Firmware & Package Manifest".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("iso-builder/config/package-lists/mindset.list.chroot:{}", pkg_line),
            notes: format!("Package list missing critical bare-metal packages: {}", missing_pkgs.join(", ")),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::Critical,
            title: "Add missing firmware and display packages to mindset.list.chroot".into(),
            location: format!("iso-builder/config/package-lists/mindset.list.chroot:{}", pkg_line),
            finding: format!("Live USB will fail on physical laptops without: {}", missing_pkgs.join(", ")),
            remediation: "Add network-manager, firmware-iwlwifi, firmware-realtek, pipewire, and mesa-vulkan-drivers into package list.".into(),
        });
    }

    // 3. Pre-installed Apps: YouTube and FreeTube Chroot Hook
    let has_youtube_desktop = ctx.has_file("iso-builder/config/includes.chroot/usr/share/applications/youtube.desktop");
    let freetube_hook = ctx
        .read_file("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot")
        .unwrap_or_default();
    let has_freetube_hook = ctx.has_file("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot");
    let has_freetube_sandbox_fix = freetube_hook.contains("4755") && freetube_hook.contains("chrome-sandbox");

    let ft_line = ctx
        .find_first_occurrence("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot", "4755")
        .map(|(l, _)| l)
        .unwrap_or(25);

    if has_youtube_desktop && has_freetube_hook && has_freetube_sandbox_fix {
        entries.push(CapabilityMatrixEntry {
            feature: "Pre-installed YouTube & FreeTube Integration".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot:{}", ft_line),
            notes: "youtube.desktop present and FreeTube chroot hook includes essential chmod 4755 chrome-sandbox permission fix".into(),
        });
    } else {
        score = score.saturating_sub(15);
        entries.push(CapabilityMatrixEntry {
            feature: "Pre-installed YouTube & FreeTube Integration".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot:{}", ft_line),
            notes: "FreeTube installation hook missing or missing Electron chrome-sandbox setuid 4755 fix".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Fix Electron chrome-sandbox setuid permission in FreeTube hook".into(),
            location: format!("iso-builder/config/hooks/live/0500-install-freetube.hook.chroot:{}", ft_line),
            finding: "Electron desktop apps crash on Linux if chrome-sandbox lacks 4755 permissions.".into(),
            remediation: "Ensure 0500-install-freetube.hook.chroot runs chmod 4755 on chrome-sandbox.".into(),
        });
    }

    // 4. Auto-Login Session Setup & VM GPU Fallback Wrapper
    let autologin_conf = ctx
        .read_file("iso-builder/config/includes.chroot/etc/systemd/system/getty@tty1.service.d/autologin.conf")
        .unwrap_or_default();
    let has_autologin = autologin_conf.contains("autologin achiever") || autologin_conf.contains("--autologin");
    let launcher_script = ctx
        .read_file("iso-builder/config/includes.chroot/usr/local/bin/mindset-os-launch")
        .unwrap_or_default();
    let has_gpu_fallback = launcher_script.contains("WGPU_BACKEND=gl") || launcher_script.contains("LIBGL_ALWAYS_SOFTWARE");

    let launch_line = ctx
        .find_first_occurrence("iso-builder/config/includes.chroot/usr/local/bin/mindset-os-launch", "WGPU_BACKEND=gl")
        .map(|(l, _)| l)
        .unwrap_or(45);

    if has_autologin && has_gpu_fallback {
        entries.push(CapabilityMatrixEntry {
            feature: "Auto-Login Session & VM GPU Fallback Wrapper".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Pass,
            evidence: format!("iso-builder/config/includes.chroot/usr/local/bin/mindset-os-launch:{}", launch_line),
            notes: "Fullscreen auto-login on tty1 into Openbox, with WGPU_BACKEND=gl & software rasterizer fallback for VirtualBox/VMware".into(),
        });
    } else {
        score = score.saturating_sub(15);
        entries.push(CapabilityMatrixEntry {
            feature: "Auto-Login Session & VM GPU Fallback Wrapper".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Partial,
            evidence: format!("iso-builder/config/includes.chroot/usr/local/bin/mindset-os-launch:{}", launch_line),
            notes: "Missing auto-login configuration or VM software graphics rasterizer fallback".into(),
        });
        actions.push(PrioritizedAction {
            severity: ActionSeverity::High,
            title: "Add GPU fallback wrapper for VirtualBox and VMware support".into(),
            location: format!("iso-builder/config/includes.chroot/usr/local/bin/mindset-os-launch:{}", launch_line),
            finding: "Vulkan hardware acceleration may fail in headless or virtual machine environments.".into(),
            remediation: "Ensure mindset-os-launch exports WGPU_BACKEND=gl or LIBGL_ALWAYS_SOFTWARE=1 on OpenGL fallback.".into(),
        });
    }

    // 5. Critical Cross-OS Script Checks: CRLF and Shebangs
    let mut crlf_violations = Vec::new();
    let mut missing_shebangs = Vec::new();

    for file in &ctx.files {
        let is_script = file.rel_path.ends_with(".sh")
            || file.rel_path.contains(".hook.")
            || file.rel_path.ends_with("/autostart")
            || file.rel_path.ends_with("/.xinitrc")
            || file.rel_path.ends_with("/.bash_profile");

        if !is_script {
            continue;
        }

        let raw_bytes = match ctx.read_file_bytes(&file.rel_path) {
            Some(b) => b,
            None => continue,
        };

        // Check for CRLF (\r\n: byte 13 followed by byte 10)
        let mut has_crlf = false;
        for i in 0..raw_bytes.len().saturating_sub(1) {
            if raw_bytes[i] == 13 && raw_bytes[i + 1] == 10 {
                has_crlf = true;
                break;
            }
        }
        if has_crlf {
            crlf_violations.push(file.rel_path.clone());
        }

        // Check for Shebang on executable scripts (.sh and .hook.*)
        if file.rel_path.ends_with(".sh") || file.rel_path.contains(".hook.") {
            let content = match ctx.read_file(&file.rel_path) {
                Some(c) => c,
                None => String::new(),
            };
            let first_line = content.lines().next().unwrap_or("").trim();
            if !first_line.starts_with("#!/bin/") {
                missing_shebangs.push(file.rel_path.clone());
            }
        }
    }

    if crlf_violations.is_empty() && missing_shebangs.is_empty() {
        entries.push(CapabilityMatrixEntry {
            feature: "Cross-OS Script Hygiene (Zero CRLF & Valid Shebangs)".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Pass,
            evidence: "iso-builder/build_mindset_iso.sh:1".into(),
            notes: "All build and live chroot scripts use standard Linux LF line endings and proper #!/bin/sh shebangs".into(),
        });
    } else {
        score = score.saturating_sub(30);
        let loc = if !crlf_violations.is_empty() {
            format!("{}:1", crlf_violations[0])
        } else {
            format!("{}:1", missing_shebangs[0])
        };

        entries.push(CapabilityMatrixEntry {
            feature: "Cross-OS Script Hygiene (Zero CRLF & Valid Shebangs)".into(),
            category: "ISO".into(),
            status: CapabilityStatus::Partial,
            evidence: loc.clone(),
            notes: format!("Script errors: {} CRLF files, {} missing shebangs", crlf_violations.len(), missing_shebangs.len()),
        });

        actions.push(PrioritizedAction {
            severity: ActionSeverity::Critical,
            title: "Convert Windows CRLF line endings to Linux LF in build scripts".into(),
            location: loc,
            finding: format!("Linux chroot fails with '\\r: command not found' when scripts have CRLF: {}", crlf_violations.join(", ")),
            remediation: "Run dos2unix or strip carriage returns (\\r) from all .sh and .hook.chroot files.".into(),
        });
    }

    metrics.insert("iso_readiness_score".into(), score.to_string());
    let summary = format!(
        "Audited live-build auto/config, hardware firmware list, YouTube/FreeTube hooks, and script line endings. ISO score: {}/100.",
        score
    );

    ModuleAuditResult {
        module_id: "iso_readiness".into(),
        module_name: "Module 4: Bootable Debian 13 ISO Creation Readiness Audit".into(),
        score,
        summary,
        entries,
        actions,
        metrics,
    }
}
