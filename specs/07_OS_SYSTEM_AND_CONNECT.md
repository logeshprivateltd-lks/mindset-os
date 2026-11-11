# MINDSET OS — SPECIFICATION 07: OS SYSTEM MANAGEMENT, CGROUPS RESOURCE LIMITER, ATOMIC UPDATER & MINDSET CONNECT

**Company:** LKS PVT LTD  
**Target Platforms:** MindSet OS Desktop (Fedora Linux / Windows) & MindSet OS Mobile (Android / iOS)  
**Core Principle:** *"Your data belongs to you, not to a particular device."*  
**RAM Budget:** < 40 MB RSS (Native Rust shell daemon + lightweight Webview UI)

---

## 1. OS Settings Hub (`/settings`), Curated App Hub & Everyday Utilities

### 1.1 Settings Categories (`/settings`)
The OS Settings Hub provides unified, persistent configuration across 10 core domains without requiring an OS restart:
1. **Personalisation & Appearance:**
   - Live selector for all 5 Neuroaesthetic Themes (*Aurora Focus, Solar Bloom, Forest Zen, Cosmic Arcade, Paper Studio*).
   - Curated high-resolution wallpapers with dynamic time-of-day shifting.
   - Liquid Glass GPU blur toggle, UI density (`compact`, `comfortable`, `spacious`), font scaling (`sm`, `md`, `lg`, `xl`), and reduced-motion toggle.
2. **Taskbar, Start Menu & Window Behavior:**
   - Persistent dock pinned apps configuration, dock auto-hide toggle, and icon magnification.
   - Multi-window Snap Layout sensitivity (low, medium, high) and 2/3/4 grid snap thresholds.
   - KDE Plasma Activity Workspace default pinning.
3. **Display & Sound:**
   - Display resolution and refresh rate (60Hz / 120Hz).
   - Scheduled **Night Light** warm blue-light filter (3400K to 4500K) with sunset-to-sunrise automatic schedule to preserve melatonin.
   - System master audio mixer and Piper/Web Speech TTS voice speed control.
4. **Language & Regional:**
   - Completely independent OS Interface Language (`i18next`) and MindSet AI Response Language (Tamil `தமிழ்`, Conversational Tanglish, English, Hindi `हिंदी`).
   - Tanglish conversation style preview and Indian standard date/time formatting.
5. **AI & Local Model Manager:**
   - Active Ollama / `llama.cpp` model switcher (`qwen2.5:3b`, `llama3.2:3b`, `deepseek-r1:7b`, `gemma2:2b`).
   - Context window ceiling (2048 to 8192 tokens) enforcing the single-model RAM discipline rule.
   - 8-point Tamil & Tanglish conversational benchmark runner.
   - 3-Tier Sovereign Memory inspector and vector memory purge/backup.
6. **Accounts, Security & Privacy:**
   - Multi-modal biometric enrollment: MindSet Hello Face Recognition, Fingerprint sensor, 6-digit PIN, and Password fallback.
   - Inactivity auto-lock timer (1, 3, 5, 10, 15, 30 minutes).
   - User Account Control (UAC) elevation strictness and one-click JSON data export / secure zero-overwrite reset.
7. **Network & Connectivity:**
   - Wi-Fi, Ethernet, and Mobile Hotspot manager.
   - First-class external USB Wi-Fi adapter monitoring (Realtek/TP-Link signal strength in dBm, auto-driver probing).
   - Local firewall toggle and DNS-over-HTTPS privacy protection.
8. **Apps & Permissions:**
   - Default app associations for Web, PDF, Video, and Code.
   - Background execution limits and per-app security permissions (Camera, Microphone, Local Network, File Vault storage access).
9. **WCAG 2.2 AA Accessibility Suite:**
   - Instant toggles for Screen Reader (Narrator), Screen Magnifier zoom, Colour-Blindness filters, Live Offline Captions bar, On-Screen Keyboard, and Sticky Keys.
10. **About System & Storage:**
    - MindSet OS version (v1.0.0 "Aurora"), Kernel version (Linux 6.11 / Windows NT Hybrid), RAM breakdown (Shell RSS <40 MB vs Ollama AI vs System Free), disk partition usage, and Ed25519 cryptographic update signatures.

### 1.2 Distraction-Free MindSet App Hub (`/app-hub`)
- **Strict Anti-Distraction Rule:** Strictly excludes all games, social media, algorithmic entertainment feeds, and short-form video apps.
- **Curated Educational Catalog:**
  - *Mathematics & Sciences:* GeoGebra, Stellarium, SciLab, Avogadro, PhET Simulations.
  - *Coding & Engineering:* VSCodium, Arduino IDE, MindSet Terminal, Git CLI, SQLite Browser.
  - *Productivity & Knowledge:* LibreOffice Suite, Obsidian Knowledge Graph, Anki Spaced Repetition, Xournal++ Stylus Notes.
  - *Creative Study:* VLC Media Player (offline), GIMP, Audacity, Blender.
- **Per-App Process Permission Governance:**
  - Fine-grained toggles for Camera, Microphone, Network, and File Vault access with real-time audit logging.

### 1.3 Everyday Zero-Distraction Utilities
- **Scientific Calculator & Unit Converter (`Super + C`):**
  - Full scientific trigonometric, logarithmic, and power functions.
  - Physics/Chemistry unit converter (Length, Mass, Pressure, Energy, Temperature: Celsius/Kelvin/Fahrenheit, Molar quantities).
- **Offline Lecture & Media Player:**
  - Clean local audio/video player with speed stepping (`0.75x`, `1x`, `1.25x`, `1.5x`, `1.75x`, `2x`, `2.5x`).
  - One-click **"Bookmark Timestamp to Notes"** linking current video second directly into SQLite study notes with zero distractions.

### 1.4 Comprehensive WCAG 2.2 AA Accessibility Suite
1. **Screen Reader (Narrator):** Built-in speech synthesis (via Piper TTS / Web Speech) announcing focused UI elements, buttons, and headings with configurable speech rate.
2. **Screen Magnifier:** Smooth GPU-accelerated 1.25x to 3.0x magnification viewport overlay with mouse following.
3. **Colour-Blindness Filters:** Hardware-accelerated SVG matrix transforms for:
   - *Protanopia* (Red-blind compensation)
   - *Deuteranopia* (Green-blind compensation)
   - *Tritanopia* (Blue-blind compensation)
   - *Monochrome Grayscale* (High-clarity distraction removal)
4. **Live Offline Captions:** Real-time speech transcription bar overlaying the bottom screen without sending audio to the cloud.
5. **On-Screen Keyboard & Sticky Keys:** Virtual touch-accessible keyboard and keyboard latching for motor accessibility.

---

## 2. Hardware & Device Manager (`/devices`)

1. **Peripheral & Port Management:** Keyboards, mice, multi-monitor displays, printers, scanners, USB storage drives (safe eject), webcams, microphones, and fingerprint sensors.
2. **External USB Wi-Fi Adapter & Driver Diagnostics:**
   - First-class monitoring and one-click driver reset (`modprobe` / DKMS check for Realtek `rtl88xxau` / TP-Link adapters), signal strength (dBm), and automatic network troubleshooter.
3. **Power, Battery & Thermal Governor:**
   - Switchable profiles: *Eco Study Mode* (quiet fans, extended battery), *Balanced Mode*, and *AI Boost Mode* (maximising CPU/GPU threads for Ollama inference).
4. **Audio/Camera Hardware Fallback Router:**
   - Automatically detects missing physical microphones or webcams and routes input smoothly to the **MindSet Connect** wireless phone bridge or text-input fallback.

---

## 3. Task Manager, `btop++` `cgroups v2` Limiter, Self-Healing Doctor & Terminal (`/status`)

1. **Task & Resource Monitor:** Real-time per-process CPU, RAM, GPU, and Disk I/O tracking—explicitly separating **MindSet OS Shell RSS (<120 MB target)** from **Ollama Local AI RAM**.
2. **Hard Process Resource Limiter (`cgroups v2`):**
   - Allows setting hard `memory.max` and `cpu.max` ceilings on individual processes so background tasks never starve the local LLM on an 8 GB or 12 GB laptop.
3. **Self-Healing System Doctor:**
   - One-click **Emergency RAM Purge**, **Ollama Runtime Recovery**, **SQLite Integrity Repair (`PRAGMA integrity_check` + `VACUUM`)**, and **Network/Driver Reset**.
4. **Advanced Developer Suite & MindSet Terminal:**
   - Integrated PTY terminal emulator (Bash/Zsh/PowerShell), Developer Mode toggles, KVM/Docker sandbox controls, and School/Parent Group Policy exam locks.

---

## 4. Atomic OTA Updater (`/updater`) & Standalone Recovery Mode

1. **Dual-Layer Update Engine:**
   - **App & Kernel Shell Updates:** Tauri 2 native delta updater with **Ed25519 cryptographic signature verification**.
   - **Base OS Atomic Updates:** Fedora Kinoite / `ostree` immutable image deployment with instant 3-second boot rollback if a startup health check fails.
2. **Four Patch Channels:** Critical Security Hotfixes, Core OS Releases, Lightweight AI Agent Prompt JSON Updates, and **Offline USB `.msos-patch` Bundles** for air-gapped school labs.
3. **Pre-Boot & Standalone Recovery Mode (`/recovery`):**
   - Ultra-lightweight rescue environment to restore encrypted system snapshots, roll back updates, repair SQLite databases, and reset broken display or Wi-Fi drivers without loading the full desktop shell.

---

## 5. MindSet Connect & MindSet OS Mobile (`packages/connect-protocol` & `apps/mindset-mobile`)

Built as a 6-module cross-device service over local Wi-Fi (LocalSend/KDE Connect style) and remote Tailscale/WireGuard tunnels:
1. **Module 1 — Device Manager:** QR-code pairing, unique Ed25519 cryptographic device keys, connection status, permissions, last-seen timestamps, and instant one-click device revocation.
2. **Module 2 — Sync Engine:** Selective folder sync (`~/.mindset_data/vault/my-laptop` and `my-phone`) plus conflict-aware SQLite change-log sync (`sync_oplog.sqlite`) that preserves conflicting file/note versions and NEVER overwrites raw `.sqlite` database files.
3. **Module 3 — Remote Access:** Permission-controlled remote file browsing, file preview without full download, and confirmation gates before remote file deletion.
4. **Module 4 — Continuity Engine ("Continue Where You Left Off"):** Real-time handoff of active PDF textbook page numbers (`last_read_page`), open study notes, tasks, and active Focus Guard timers between Laptop and Mobile.
5. **Module 5 — Communication & Hardware Bridge Engine:**
   - **Privacy-Filtered Universal Clipboard:** Syncs ordinary text/links while blocking passwords, OTPs, and sensitive clipboard strings.
   - **Notification Mirroring & Remote Controls:** Mirrors selected phone alerts to laptop; controls laptop presentations and media from the phone.
   - **Wireless Hardware Bridge:** Streams the mobile camera as a **Laptop Webcam** (for MindSet Hello Face Unlock and video classes), streams the mobile mic as a **Wireless Microphone** (for Whisper voice typing), and provides a **Mobile Document Scanner** that converts paper notes into RAG-indexed PDFs on the laptop.
6. **Module 6 — Security Engine:** End-to-end TLS/Noise encryption, folder-level access control, deleted-file version recovery, and complete audit logs.
