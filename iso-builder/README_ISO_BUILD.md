# MindSet OS — Debian 13 ("Trixie") Derivative `.iso` Builder

Builds a reproducible, bootable, installable **Debian 13 ("Trixie") x86_64 hybrid Live ISO** (`mindset-os-amd64.hybrid.iso`) using upstream Debian `live-build` that boots directly into the Pure-Rust MindSet OS Wayland session (`labwc` + `mindset-shell`).

## Architecture

- **Bootloader:** Hybrid UEFI (`grub-efi`) + Legacy BIOS (`syslinux` / `isolinux`)
- **Base OS:** Debian 13 (`trixie`) `amd64` with `main contrib non-free non-free-firmware`
- **Wayland Session:** `greetd` autologin (`live` user) → `/usr/local/bin/mindset-session` → `labwc -C ~/.config/mindset-os/labwc` → `/usr/local/bin/mindset-shell`
- **Core Services:** `NetworkManager`, `seatd`, `bluez`, `pipewire`, `pipewire-pulse`, `wireplumber`, `espeak-ng`, `brightnessctl`
- **Graphical Installer:** `calamares` + `calamares-settings-debian`

## Automated Build on GitHub Actions

Every push to `master` or `main` (and manual `workflow_dispatch`) triggers [`.github/workflows/build-iso.yml`](../.github/workflows/build-iso.yml), which:
1. Spins up a native `debian:trixie` container so Rust release binaries link strictly against Debian 13 `glibc`.
2. Executes `cargo build --release --bin mindset-shell --bin mindset-os -p mindset-focus-guard`.
3. Runs `./build-iso.sh` inside `iso-builder/`.
4. Verifies hybrid El-Torito boot tables (`xorriso`), runs headless QEMU smoke boot, and uploads `mindset-os-amd64.hybrid.iso` + `.sha256` + `build.log`.

## Manual Local Build (Debian 13 Host / Ext4 Only)

```bash
# On native ext4/btrfs Linux filesystem with >= 20 GB free space:
sudo apt-get update && sudo apt-get install -y live-build debootstrap xorriso squashfs-tools qemu-system-x86
cargo build --release --bin mindset-shell --bin mindset-os -p mindset-focus-guard
cd iso-builder
sudo ./build-iso.sh
```

## Boot Verification in QEMU/KVM

```bash
qemu-system-x86_64 -enable-kvm -m 4096 -smp 4 \
  -vga virtio -display gtk,gl=on \
  -cdrom mindset-os-amd64.hybrid.iso
```

## Flash to USB Drive

```bash
# Linux dd (replace /dev/sdX with target USB block device):
sudo dd if=mindset-os-amd64.hybrid.iso of=/dev/sdX bs=4M status=progress oflag=sync
```
Or copy `mindset-os-amd64.hybrid.iso` directly onto a Ventoy USB partition.
