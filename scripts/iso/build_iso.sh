#!/usr/bin/env bash
# ==============================================================================
# MindSet OS — Live Bootable ISO Generator (Debian 12 Bookworm + Cage Wayland)
# Target Architecture: x86_64 (UEFI + Legacy BIOS Hybrid Boot)
# ==============================================================================

set -euo pipefail

echo "=========================================================="
echo "  MindSet OS v1.0.0 — Sovereign Bootable ISO Builder     "
echo "  Powered by LKS PVT LTD                                  "
echo "=========================================================="

# Check root privileges
if [ "$EUID" -ne 0 ]; then
  echo "[-] ERROR: This script must be run as root (or with sudo)."
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
WORK_DIR="/tmp/mindsetos-iso-build"
OUTPUT_DIR="${REPO_ROOT}/dist-release"

mkdir -p "${OUTPUT_DIR}"
rm -rf "${WORK_DIR}"
mkdir -p "${WORK_DIR}"
cd "${WORK_DIR}"

echo "[+] Step 1: Installing Host Prerequisites..."
apt-get update -y
apt-get install -y --no-install-recommends \
  live-build \
  debootstrap \
  squashfs-tools \
  xorriso \
  isolinux \
  syslinux-efi \
  grub-pc-bin \
  grub-efi-amd64-bin \
  mtools \
  dosfstools

echo "[+] Step 2: Locating MindSet OS Linux Package (.deb)..."
DEB_FILE=$(find "${REPO_ROOT}/dist-release" "${REPO_ROOT}/apps/mindset-desktop/src-tauri/target/release/bundle/deb" -name "*.deb" 2>/dev/null | head -n 1 || true)

if [ -z "${DEB_FILE}" ] || [ ! -f "${DEB_FILE}" ]; then
  echo "[-] WARNING: No prebuilt .deb found in dist-release or target. Building now..."
  cd "${REPO_ROOT}"
  pnpm install --frozen-lockfile
  pnpm build
  cargo build --release -p mindset-focus-guard
  mkdir -p apps/mindset-desktop/src-tauri/binaries
  cp target/release/mindset-focus-guard apps/mindset-desktop/src-tauri/binaries/mindset-focus-guard-x86_64-unknown-linux-gnu
  cp target/release/mindset-focus-guard apps/mindset-desktop/src-tauri/binaries/mindset-focus-guard
  pnpm --filter @mindset/desktop tauri build --bundles deb
  DEB_FILE=$(find "${REPO_ROOT}/apps/mindset-desktop/src-tauri/target/release/bundle/deb" -name "*.deb" | head -n 1)
  cd "${WORK_DIR}"
fi

echo "[+] Using Debian package: ${DEB_FILE}"

echo "[+] Step 3: Configuring Live-Build Environment..."
lb config \
  --mode debian \
  --system live \
  --distribution bookworm \
  --architectures amd64 \
  --archive-areas "main contrib non-free non-free-firmware" \
  --bootloader grub-efi \
  --binary-images iso-hybrid \
  --iso-application "MindSet OS Sovereign Desktop" \
  --iso-publisher "LKS PVT LTD <engineering@lkspvtltd.com>" \
  --iso-volume "MINDSETOS_1_0_0" \
  --memtest none \
  --linux-packages "linux-image" \
  --linux-flavours "amd64" \
  --security true \
  --updates true

echo "[+] Step 4: Staging Package Inclusions..."
mkdir -p config/packages.chroot/
cp "${DEB_FILE}" config/packages.chroot/mindset-os.deb

# Define base system and GUI runtime packages
cat << 'EOF' > config/package-lists/mindset-base.list.chroot
# Kernel and Hardware Firmware
linux-image-amd64
firmware-linux-free
firmware-linux-nonfree
firmware-misc-nonfree
firmware-realtek
firmware-iwlwifi

# Wayland Kiosk & Audio Stack
cage
xwayland
seatd
pipewire
pipewire-pulse
pipewire-alsa
wireplumber
libspa-0.2-bluetooth
mesa-va-drivers
mesa-vulkan-drivers

# MindSet OS GUI & Microkernel System Dependencies
libwebkit2gtk-4.1-0
libayatana-appindicator3-1
librsvg2-common
libssl3
libsqlite3-0
network-manager
network-manager-gnome
bluez
polkit-1
dbus-user-session
sudo
curl
ca-certificates
fonts-inter
fonts-noto-color-emoji
fonts-indic

# System Installer (Optional GUI Install to SSD)
calamares
calamares-settings-debian
parted
dosfstools
e2fsprogs
EOF

echo "[+] Step 5: Configuring Autologin and Wayland Kiosk Session..."
mkdir -p config/includes.chroot/etc/systemd/system/getty@tty1.service.d/
cat << 'EOF' > config/includes.chroot/etc/systemd/system/getty@tty1.service.d/autologin.conf
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin mindset --noclear %I $TERM
Type=idle
EOF

# User session auto-start script on login
mkdir -p config/includes.chroot/etc/skel/
cat << 'EOF' > config/includes.chroot/etc/skel/.profile
# Auto-start Cage Wayland Compositor on TTY1 login
if [ -z "$DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then
  exec cage -s -- /usr/local/bin/mindset-session.sh
fi
EOF

# Launcher script managing Focus Guard and Desktop Shell
mkdir -p config/includes.chroot/usr/local/bin/
cat << 'EOF' > config/includes.chroot/usr/local/bin/mindset-session.sh
#!/bin/bash
export XDG_CURRENT_DESKTOP=MindSetOS
export XDG_SESSION_TYPE=wayland
export GDK_BACKEND=wayland,x11
export WEBKIT_DISABLE_COMPOSITING_MODE=0

# Ensure Audio daemon is running
gentle_pulse=$(pgrep -x pipewire || true)
if [ -z "$gentle_pulse" ]; then
  pipewire &
  wireplumber &
fi

# Launch MindSet OS full-screen sovereign session
exec /usr/bin/mindset-os
EOF
chmod +x config/includes.chroot/usr/local/bin/mindset-session.sh

# User creation and permission hook
mkdir -p config/hooks/live/
cat << 'EOF' > config/hooks/live/01-create-mindset-user.hook.chroot
#!/bin/sh
set -e

# Create non-root user 'mindset' with empty password (Live session)
if ! id -u mindset >/dev/null 2>&1; then
  useradd -m -s /bin/bash -G sudo,audio,video,render,netdev,plugdev,input mindset
  passwd -d mindset
fi

# Enable passwordless sudo for live session installer
echo "mindset ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/mindset-live
chmod 0440 /etc/sudoers.d/mindset-live

# Set hostname
echo "mindsetos" > /etc/hostname

# Enable NetworkManager and Bluetooth
systemctl enable NetworkManager.service || true
systemctl enable bluetooth.service || true
EOF
chmod +x config/hooks/live/01-create-mindset-user.hook.chroot

echo "[+] Step 6: Building Live Bootable Hybrid ISO..."
lb build 2>&1 | tee "${WORK_DIR}/live-build.log"

ISO_RESULT=$(find "${WORK_DIR}" -maxdepth 1 -name "*.iso" | head -n 1)

if [ -n "${ISO_RESULT}" ] && [ -f "${ISO_RESULT}" ]; then
  FINAL_ISO="${OUTPUT_DIR}/MindSet_OS_1.0.0_amd64.iso"
  cp "${ISO_RESULT}" "${FINAL_ISO}"
  echo "=========================================================="
  echo "  SUCCESS! MindSet OS Bootable ISO Generated:            "
  echo "  Path: ${FINAL_ISO}                                      "
  echo "  Size: $(du -h "${FINAL_ISO}" | cut -f1)                 "
  echo "=========================================================="
  sha256sum "${FINAL_ISO}" > "${FINAL_ISO}.sha256"
else
  echo "[-] ERROR: ISO build failed. Inspect log at ${WORK_DIR}/live-build.log"
  exit 1
fi
