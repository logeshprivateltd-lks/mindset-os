# MINDSETOS — PROMPT 27: SOVEREIGN BOOTABLE ISO BUILD PROCEDURE & RUNBOOK

> **Document ID:** `27_BOOTABLE_ISO_CREATION_PROCEDURE.md`  
> **Release Version:** `v1.0.0` (Golden Master ISO Extension)  
> **Date:** November 23, 2026  
> **Author:** Lead Systems Architect & Infrastructure Lead, LKS PVT LTD  
> **Target Media:** Bootable Hybrid ISO (UEFI + Legacy BIOS) for USB, Bare-Metal & Virtual Machines (QEMU/KVM, VirtualBox)  

---

## 1. Architectural Overview: Turning MindSet OS into a Bootable OS

MindSet OS is a sovereign desktop environment engineered with a Tauri 2 / Rust microkernel, an isolated sidecar daemon (`mindset-focus-guard`), local SQLite databases (`local_core.sqlite`, `sync_oplog.sqlite`), and a neuroaesthetic frontend shell.

To turn this into a **bootable `.iso` image**, the application is married to a minimal, hardened Linux base system configured to launch MindSet OS as the exclusive display session:

```
+-------------------------------------------------------------------------------+
|                        MINDSET OS BOOTABLE ISO STACK                          |
+-------------------------------------------------------------------------------+
|  Application Layer     | MindSet OS Desktop Shell + Focus Guard Sidecar       |
|  Display Compositor    | Cage Wayland Kiosk (Fullscreen, 0 border, 4MB RAM)   |
|  Audio & Hardware Stack| PipeWire + WirePlumber + NetworkManager + Seatd      |
|  System Management     | systemd (autologin into 'mindset' user via tty1)     |
|  Kernel & Drivers      | Linux LTS Kernel (amd64) + Non-Free Hardware Firmware|
|  Filesystem Layout     | OverlayFS / SquashFS (Compressed read-only live root)|
|  Bootloader            | Dual GRUB 2 EFI + Syslinux Hybrid MBR/GPT            |
|  Installer Option      | Calamares GUI installer (Live mode -> Install to SSD)|
+-------------------------------------------------------------------------------+
```

---

## 2. Prerequisites & Compilation

### 2.1 Compiling the Linux Binaries (`.deb` & Sidecar)

Before building the ISO filesystem, the native Linux binaries must be compiled:

```bash
# 1. Install frontend and microkernel dependencies
pnpm install --frozen-lockfile

# 2. Build production web bundles
pnpm build

# 3. Build Focus Guard daemon in release mode
cargo build --release -p mindset-focus-guard
mkdir -p apps/mindset-desktop/src-tauri/binaries
cp target/release/mindset-focus-guard apps/mindset-desktop/src-tauri/binaries/mindset-focus-guard-x86_64-unknown-linux-gnu
cp target/release/mindset-focus-guard apps/mindset-desktop/src-tauri/binaries/mindset-focus-guard

# 4. Package Tauri Debian package
pnpm --filter @mindset/desktop tauri build --bundles deb
```
This produces `apps/mindset-desktop/src-tauri/target/release/bundle/deb/mindset-os_1.0.0_amd64.deb`.

---

## 3. Method 1: Automated Debian Live-Build (`scripts/iso/build_iso.sh`)

This is the primary, production-tested method. It uses Debian 12 Bookworm Minimal with `live-build`.

### 3.1 Host Dependencies
```bash
sudo apt-get update
sudo apt-get install -y live-build debootstrap squashfs-tools xorriso \
  isolinux syslinux-efi grub-pc-bin grub-efi-amd64-bin mtools dosfstools
```

### 3.2 Execution
Run the automated builder script:
```bash
sudo bash scripts/iso/build_iso.sh
```

### 3.3 What the Script Automates:
1. **Live-Build Configuration (`lb config`):** Sets up hybrid ISO flags, architecture `amd64`, UEFI GRUB + MBR isolinux.
2. **Debian Package Staging:** Drops `mindset-os.deb` into `config/packages.chroot/` for native package installation.
3. **Wayland Kiosk Setup:** Installs `cage` (Wayland kiosk compositor).
4. **Autologin & Kiosk Session:**
   - Overrides `getty@tty1.service` to auto-login the unprivileged `mindset` user.
   - Appends `exec cage -s -- /usr/local/bin/mindset-session.sh` to `/home/mindset/.profile`.
5. **Kernel & Firmware:** Bundles `linux-image-amd64` alongside Wi-Fi and GPU firmware (`firmware-linux`, `firmware-realtek`, `firmware-iwlwifi`).
6. **Output Generation:** Builds `dist-release/MindSet_OS_1.0.0_amd64.iso` and generates `SHA256SUM`.

---

## 4. Method 2: GitHub Actions Cloud Build (Zero Setup on Windows)

Since host OS is Windows 11, the simplest way to produce a clean ISO without setting up local Linux loop devices is GitHub Actions:

1. Push your changes to your GitHub repository:
   ```bash
   git add .
   git commit -m "feat(iso): add live bootable ISO builder pipeline"
   git push origin create_iso_procedure
   ```
2. Navigate to **GitHub Actions** -> **Build MindSet OS Bootable ISO**.
3. Click **Run workflow**.
4. In ~8-12 minutes, download the ready-to-flash `MindSet_OS_1.0.0_amd64.iso` from the **Artifacts** section!

---

## 5. Method 3: Fedora Kinoite / bootc Container-to-Disk ISO

For immutable, atomic deployments (as specified in Prompts 21 & 26):

1. **Build Container Image:**
   ```bash
   podman build -t mindset-os:latest -f scripts/iso/Containerfile.bootc .
   ```
2. **Generate Bootable ISO via bootc-image-builder:**
   ```bash
   podman run --privileged --rm \
     -v $(pwd)/dist-release:/output \
     quay.io/centos-bootc/bootc-image-builder:latest \
     --type iso \
     mindset-os:latest
   ```

---

## 6. Testing & Flashing the ISO

### 6.1 Testing in Virtual Machines

**Using QEMU:**
```bash
qemu-system-x86_64 \
  -enable-kvm \
  -m 4G \
  -smp 4 \
  -cdrom dist-release/MindSet_OS_1.0.0_amd64.iso \
  -boot d \
  -vga virtio \
  -device AC97
```

**Using VirtualBox:**
1. Create new VM -> Type: `Linux`, Version: `Debian (64-bit)`.
2. Memory: `4096 MB`, Processors: `2-4 vCPUs`.
3. Storage: Attach `MindSet_OS_1.0.0_amd64.iso` to the Optical Drive.
4. Enable EFI in System Settings -> Motherboard -> `Enable EFI (special OSes only)`.
5. Start VM.

### 6.2 Flashing to Physical USB Drive (Live Boot)

- **On Windows (Rufus):**
  1. Plug in USB drive (>= 8 GB).
  2. Open **Rufus**.
  3. Select `MindSet_OS_1.0.0_amd64.iso`.
  4. Partition Scheme: `GPT`, Target System: `UEFI (non CSM)`.
  5. When prompted, select **Write in DD Image mode**.
  6. Click **Start**.

- **On Linux / macOS (`dd`):**
  ```bash
  sudo dd if=dist-release/MindSet_OS_1.0.0_amd64.iso of=/dev/sdX bs=4M status=progress conv=fsync
  ```
