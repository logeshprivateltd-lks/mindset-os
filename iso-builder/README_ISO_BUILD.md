# MindSet OS — Debian 13 (Trixie) Live & Installable ISO Build Guide

This directory (`iso-builder/`) contains the complete, reproducible build system for generating **`mindset-v1.iso`**, an installable, hybrid UEFI + Legacy BIOS Debian 13 Live ISO. It boots directly into the full-screen **MindSet OS** student computing shell on both physical PCs and virtual machines.

---

## 1. Architecture & Capabilities

```
+-----------------------------------------------------------------------------------+
|                        MINDSET OS LIVE ISO ARCHITECTURE                           |
+-----------------------------------------------------------------------------------+
|  Bootloader:  Hybrid GRUB 2.12 (UEFI x86_64) + ISOLINUX (Legacy BIOS)             |
|  Kernel:      Linux 6.12+ (Debian 13 Trixie amd64)                                |
|  Firmware:    firmware-linux, firmware-iwlwifi, firmware-realtek, firmware-atheros|
|  Audio:       PipeWire 1.2+ & WirePlumber (low latency, Bluetooth audio support)  |
|  Display:     Xorg + Minimal Openbox (Frameless, auto-maximized shell)            |
|  Graphics:    Smart GPU Probe (/usr/local/bin/mindset-os-launch):                 |
|               - Bare Metal PC: Native Vulkan (Intel Xe, AMD Radeon, Nvidia)       |
|               - Virtual Machine: Automatic Mesa OpenGL / LLVMpipe Failover        |
|  Session:     Zero-login friction: Auto-boots directly into 'achiever' desktop    |
|  Apps:        MindSet OS (Native Pure Rust + wgpu) + Sandboxed Chromium Browser   |
+-----------------------------------------------------------------------------------+
```

---

## 2. Prerequisites

| Requirement | Specification |
| :--- | :--- |
| **Build Host OS** | Debian 12/13, Ubuntu 22.04/24.04, or WSL2 with Debian/Ubuntu |
| **Filesystem** | **Native Linux filesystem (ext4, btrfs, xfs)**. *Cannot build on Windows mounts (`/mnt/c` or VirtualBox shared folders) due to Linux device node & symlink constraints.* |
| **Free Disk Space** | $\ge 15\text{ GB}$ (for Debian chroot, packages, and squashfs) |
| **Target USB Drive** | $\ge 8\text{ GB}$ USB 3.0 flash drive (Ventoy or Rufus) |
| **Target PC / VM** | 64-bit x86_64 CPU, $\ge 4\text{ GB}$ RAM, UEFI or Legacy BIOS |

---

## 3. Quick Start: One-Command Build (Linux VM or WSL2)

If you are running inside a Debian or Ubuntu Linux VirtualBox/VMware VM or WSL2:

```bash
# 1. Clone or copy MINDSETOS into your native Linux home directory:
cd ~
cp -r /path/to/MINDSETOS ~/mindset-workspace
cd ~/mindset-workspace/iso-builder

# 2. Run the automated one-click build wrapper:
sudo ./vm_one_click_build.sh
```

The script will automatically:
1. Verify the filesystem is native ext4/btrfs.
2. Install any missing build tools and Rust toolchain.
3. Compile `mindset-os` in release mode (`cargo build --release --bin mindset-os`).
4. Stage the binary, fonts (Noto Tamil, Malayalam, English), and assets into the chroot overlay.
5. Execute `lb clean`, `lb config`, and `lb build`.
6. Output **`mindset-v1.iso`** and its verified **`mindset-v1.iso.sha256`** checksum.

---

## 4. Manual Step-by-Step Build Pipeline

If you prefer building step-by-step:

### Step 1: Install Build Dependencies on Host
```bash
sudo apt-get update
sudo apt-get install -y \
    live-build debootstrap xorriso squashfs-tools \
    isolinux syslinux-common mtools dosfstools \
    grub-pc-bin grub-efi-amd64-bin ca-certificates curl \
    build-essential pkg-config libssl-dev libasound2-dev \
    libx11-dev libxcursor-dev libxi-dev libxrandr-dev libvulkan-dev
```

### Step 2: Compile the MindSet OS Linux Binary
From the root workspace directory (`c:\MINDSETOS` or `~/mindset-workspace`):
```bash
cargo build --release --bin mindset-os
```
*Binary output:* `target/release/mindset-os`

### Step 3: Stage Binaries & Fonts into ISO Overlay
```bash
cd iso-builder

# Create directory structure
mkdir -p config/includes.chroot/opt/mindset-os/bin
mkdir -p config/includes.chroot/opt/mindset-os/assets/fonts
mkdir -p config/includes.chroot/opt/mindset-os/assets/icons
mkdir -p config/includes.chroot/usr/local/bin

# Copy compiled binary
cp -f ../target/release/mindset-os config/includes.chroot/opt/mindset-os/bin/mindset-os
cp -f ../target/release/mindset-os config/includes.chroot/usr/local/bin/mindset-os
chmod +x config/includes.chroot/opt/mindset-os/bin/mindset-os
chmod +x config/includes.chroot/usr/local/bin/mindset-os

# Copy assets and Indic fonts
cp -u ../assets/fonts/*.ttf config/includes.chroot/opt/mindset-os/assets/fonts/
cp -u ../assets/icons/* config/includes.chroot/opt/mindset-os/assets/icons/
```

### Step 4: Run the ISO Build Engine
```bash
sudo ./build_mindset_iso.sh
```

---

## 5. Alternative: Cross-Build via Docker

You can also assemble the ISO without modifying your host system using Docker:

```bash
cd ~/mindset-workspace

docker run --privileged --rm -it \
    -v "$PWD:/workspace" \
    -w /workspace/iso-builder \
    debian:trixie-slim \
    bash -c "
        apt-get update && \
        apt-get install -y sudo live-build debootstrap xorriso squashfs-tools isolinux syslinux-common mtools dosfstools grub-pc-bin grub-efi-amd64-bin ca-certificates curl && \
        ./build_mindset_iso.sh
    "
```

---

## 6. How to Flash and Boot on Real Hardware

### Method A: Ventoy (Recommended for Multiboot USB)
1. Download [Ventoy](https://www.ventoy.net/) and install it onto your USB flash drive.
2. In Windows or Linux file manager, simply **copy and paste `mindset-v1.iso`** into the USB root partition.
3. Plug the USB into your target PC/laptop, reboot, and press the boot key (`F12`, `F11`, `F9`, or `Esc`).
4. Select `mindset-v1.iso` from the Ventoy menu. Choose **"Normal mode"**.

### Method B: Rufus (Windows)
1. Open [Rufus](https://rufus.ie/).
2. Select your USB drive under **Device**.
3. Under **Boot selection**, choose `mindset-v1.iso`.
4. Set:
   - **Partition scheme:** `GPT` (for modern UEFI laptops) or `MBR` (for older BIOS machines).
   - **Target system:** `UEFI (non CSM)` or `BIOS or UEFI`.
5. Click **START**. When prompted, choose **"Write in ISO Image mode (Recommended)"**.

### Method C: Linux `dd` Command
```bash
# Locate your USB drive letter (e.g. /dev/sdb, NOT /dev/sdb1)
lsblk

# Flash ISO directly:
sudo dd if=mindset-v1.iso of=/dev/sdX bs=4M status=progress oflag=sync
```

---

## 7. Testing in Virtual Machines

### VirtualBox Settings:
1. **Type:** Linux, **Version:** Debian (64-bit).
2. **Memory (RAM):** $\ge 4096\text{ MB}$ (4 GB).
3. **Processors:** 2 or more CPU cores.
4. **Display:** Video Memory: `128 MB`, Graphics Controller: `VMSVGA` (Enable 3D Acceleration optional).
5. **System:** Check **"Enable EFI (special OSes only)"**.
6. **Storage:** Attach `mindset-v1.iso` to the Optical Drive.
7. Start VM. The bootloader will launch, probe graphics, engage OpenGL/LLVMpipe fallback if 3D is disabled, and present the MindSet desktop.

### VMware Workstation / Player Settings:
1. Create new VM -> Install from: `mindset-v1.iso`.
2. Guest OS: `Linux` -> `Debian 12/13 64-bit`.
3. RAM: $\ge 4\text{ GB}$, Processors: $2$.
4. Acceleration: Check **"Accelerate 3D graphics"**.

---

## 8. Live Session Details & Hotkeys

- **Default User:** `achiever` (Password: `mindset`).
- **Sudo Permissions:** Passwordless (`sudo command`).
- **Emergency Console Switch:** `Ctrl + Alt + F2` switches to tty2 console; `Ctrl + Alt + F1` returns to MindSet OS.
- **Log Files:**
  - Launch & GPU probe logs: `/tmp/mindset-os.log`
  - X11 session logs: `/home/achiever/.xsession-errors`
- **Wi-Fi Connection:** Handled automatically by `NetworkManager` in the top Control Center and via `nmcli` in terminal.
