# Contributing to MindSet OS

Thank you for contributing to **MindSet OS**, an intentional, non-shaming, AI-assisted neuroaesthetic desktop operating environment designed for students, powered by **LKS PVT LTD**.

---

## 🛠️ Prerequisites

Before developing locally, ensure you have the following installed:

1. **Rust (Stable):** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Target: `x86_64-pc-windows-msvc` (on Windows) or Linux/macOS equivalents.
2. **Node.js 20+ & pnpm:**
   ```bash
   corepack enable
   corepack prepare pnpm@latest --activate
   ```
3. **Tauri CLI v2:** Installed via package scripts.
4. **Ollama (Optional, for offline Laya AI):** Download from [ollama.ai](https://ollama.ai) and pull `qwen2.5:3b-instruct-q4_K_M`.

---

## 🚀 Quick Start

### 1. Install Dependencies
```bash
git clone https://github.com/lks-pvt-ltd/mindset-os.git
cd mindset-os
pnpm install
```

### 2. Run the Full Desktop Application (Rust Kernel + React Shell)
```bash
pnpm tauri:dev
# OR simply
pnpm dev
```

### 3. Run the Frontend Only (Instant Hot Reload via Vite)
If you only want to work on UI components without compiling Rust:
```bash
pnpm dev:web
```
Open your browser at `http://localhost:1420`. The frontend includes graceful mock fallbacks when running outside the native Tauri runtime.

---

## 📁 Repository Architecture

```
MINDSETOS/
├── apps/
│   ├── mindset-desktop/        # Native Tauri 2 Desktop Shell (React + Rust)
│   │   ├── src/                # Neuroaesthetic UI, Pages, Window Manager
│   │   └── src-tauri/          # High-performance Rust Kernel, SQLite, Devices, Auth
│   └── mindset-mobile/         # Mobile Companion App (Capacitor PWA)
├── packages/
│   ├── ui-core/                # Shared Design System, Aurora Glass, Neuroaesthetic Themes
│   ├── i18n-locales/           # 8 Indian Languages + Tanglish Engine
│   ├── balance-engine/         # Free-time detection, Cortisol breathwork, Activity planner
│   ├── focus-guard/            # Non-shaming focus session & distraction journal engine
│   ├── identity-engine/        # Living Student Profile & local memory inspector
│   ├── laya-router/            # Low-RAM persona routing & GBNF tool caller
│   ├── connect-protocol/       # Cross-device sync, HLC oplog, hardware bridge
│   └── db-schema/              # Shared SQLite types & DTOs
└── specs/                      # Architecture blueprints & prompt implementation reports
```

---

## 🧪 Testing & Validation

```bash
# Typecheck all packages
pnpm typecheck

# Verify Rust Kernel compilation
cargo check --manifest-path apps/mindset-desktop/src-tauri/Cargo.toml

# Build Desktop production package
pnpm build:desktop
```

---

## 📜 Core Guiding Principles

1. **Non-Shaming Attention:** MindSet OS never humiliates, locks students out in emergencies, or converts every free minute into forced study.
2. **Local-First & Zero-RAM Waste:** Local SQLite, single-model Ollama discipline (<120 MB OS footprint), and offline resilience.
3. **Neuroaesthetic Design:** Calm colors, gentle typography, and predictable interactions.

