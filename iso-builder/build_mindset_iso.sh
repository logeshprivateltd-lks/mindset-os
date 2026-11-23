#!/bin/bash
# ==============================================================================
# MindSet OS — Debian 13 (Trixie) Live & Installable ISO Builder
# Generates: mindset-v1.iso (Hybrid UEFI + Legacy BIOS bootable)
# Target: Bare-Metal PC (Ventoy / Rufus USB) + VirtualBox / VMware
# ==============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=========================================================================="
echo "         MindSet OS v1.0 — Debian 13 (Trixie) ISO Build Engine           "
echo "=========================================================================="

# 1. Require Superuser / Root Privileges
if [ "$EUID" -ne 0 ]; then
    echo "ERROR: This script must be run as root (sudo ./build_mindset_iso.sh)."
    echo "live-build requires root privileges to mount pseudo-filesystems and build chroots."
    exit 1
fi

# 2. Filesystem Safety Interlock: Must run on native Linux filesystem (ext4/btrfs/xfs)
FS_TYPE="$(stat -f -c %T . 2>/dev/null || df -T . | tail -1 | awk '{print $2}')"
case "$FS_TYPE" in
    vboxsf|drvfs|9p|cifs|smb*|fuse.sshfs)
        echo "=========================================================================="
        echo "CRITICAL ERROR: Unsupported Filesystem Detected ($FS_TYPE)!"
        echo "live-build requires a native Linux filesystem with POSIX permissions,"
        echo "device nodes (mknod), and hardlinks. Running inside Windows shared drives"
        echo "(e.g. /mnt/c or /media/sf_*) WILL cause tar/mknod permission failures."
        echo ""
        echo "To fix: Copy this workspace to a native ext4 directory (e.g. /tmp or ~/):"
        echo "  mkdir -p /home/\$SUDO_USER/build-workspace"
        echo "  cp -r $(pwd)/.. /home/\$SUDO_USER/build-workspace/MINDSETOS"
        echo "  cd /home/\$SUDO_USER/build-workspace/MINDSETOS/iso-builder"
        echo "  sudo ./build_mindset_iso.sh"
        echo "=========================================================================="
        exit 1
        ;;
    *)
        echo "[FS Check] Building on native Linux filesystem ($FS_TYPE). OK."
        ;;
esac

# 3. Check and Install Required Host Build Dependencies
REQUIRED_PKGS="live-build debootstrap xorriso squashfs-tools isolinux syslinux-common mtools dosfstools grub-pc-bin grub-efi-amd64-bin ca-certificates curl"
MISSING_PKGS=""

for pkg in $REQUIRED_PKGS; do
    if ! dpkg -s "$pkg" >/dev/null 2>&1; then
        MISSING_PKGS="$MISSING_PKGS $pkg"
    fi
done

if [ -n "$MISSING_PKGS" ]; then
    echo "[Deps] Installing missing host packages:$MISSING_PKGS..."
    apt-get update -y
    apt-get install -y --no-install-recommends $MISSING_PKGS
fi

# 4. Ensure Overlay Directories and Permissions
echo "[Setup] Preparing chroot directory hierarchy..."
mkdir -p config/package-lists
mkdir -p config/hooks/live
mkdir -p config/includes.chroot/opt/mindset-os/bin
mkdir -p config/includes.chroot/opt/mindset-os/assets/fonts
mkdir -p config/includes.chroot/opt/mindset-os/assets/icons
mkdir -p config/includes.chroot/usr/local/bin
mkdir -p config/includes.chroot/etc/skel/.config/openbox
mkdir -p config/includes.chroot/etc/systemd/system/getty@tty1.service.d
mkdir -p config/includes.chroot/etc/sudoers.d
mkdir -p config/includes.chroot/etc/NetworkManager

# Ensure scripts are executable
chmod +x auto/config auto/clean auto/build 2>/dev/null || true
chmod +x config/hooks/live/* 2>/dev/null || true
chmod +x config/includes.chroot/usr/local/bin/* 2>/dev/null || true
chmod +x config/includes.chroot/etc/skel/.xinitrc 2>/dev/null || true
chmod +x config/includes.chroot/etc/skel/.config/openbox/autostart 2>/dev/null || true

# 5. Populate Assets from Workspace (if available)
WORKSPACE_ROOT="$(cd .. && pwd)"
if [ -d "$WORKSPACE_ROOT/assets/fonts" ]; then
    echo "[Assets] Syncing fonts into chroot overlay..."
    cp -u "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/opt/mindset-os/assets/fonts/ 2>/dev/null || true
fi
if [ -d "$WORKSPACE_ROOT/assets/icons" ]; then
    echo "[Assets] Syncing icons into chroot overlay..."
    cp -u "$WORKSPACE_ROOT/assets/icons"/* config/includes.chroot/opt/mindset-os/assets/icons/ 2>/dev/null || true
fi

# Check for pre-built mindset-os binary
if [ -f "$WORKSPACE_ROOT/target/release/mindset-os" ]; then
    echo "[Binary] Detected target/release/mindset-os. Injecting into chroot..."
    cp -f "$WORKSPACE_ROOT/target/release/mindset-os" config/includes.chroot/opt/mindset-os/bin/mindset-os
    cp -f "$WORKSPACE_ROOT/target/release/mindset-os" config/includes.chroot/usr/local/bin/mindset-os
    chmod +x config/includes.chroot/opt/mindset-os/bin/mindset-os
    chmod +x config/includes.chroot/usr/local/bin/mindset-os
elif [ -f "$WORKSPACE_ROOT/target/x86_64-unknown-linux-gnu/release/mindset-os" ]; then
    echo "[Binary] Detected cross-built Linux binary. Injecting into chroot..."
    cp -f "$WORKSPACE_ROOT/target/x86_64-unknown-linux-gnu/release/mindset-os" config/includes.chroot/opt/mindset-os/bin/mindset-os
    cp -f "$WORKSPACE_ROOT/target/x86_64-unknown-linux-gnu/release/mindset-os" config/includes.chroot/usr/local/bin/mindset-os
    chmod +x config/includes.chroot/opt/mindset-os/bin/mindset-os
    chmod +x config/includes.chroot/usr/local/bin/mindset-os
else
    echo "NOTICE: No compiled 'mindset-os' Linux binary found in target/release/."
    echo "A placeholder script will be used if building without compiling first."
    if [ ! -f config/includes.chroot/opt/mindset-os/bin/mindset-os ]; then
        cat << 'EOF' > config/includes.chroot/opt/mindset-os/bin/mindset-os
#!/bin/bash
x-terminal-emulator -e "echo 'MindSet OS binary not pre-compiled! Compile via cargo build --release --bin mindset-os and rebuild ISO.'; bash"
EOF
        chmod +x config/includes.chroot/opt/mindset-os/bin/mindset-os
        cp config/includes.chroot/opt/mindset-os/bin/mindset-os config/includes.chroot/usr/local/bin/mindset-os
    fi
fi

# 6. Execute live-build Pipeline
echo "=========================================================================="
echo " Stage 1/3: Cleaning previous build artifacts..."
echo "=========================================================================="
lb clean --purge

echo "=========================================================================="
echo " Stage 2/3: Generating Debian 13 (Trixie) Configuration..."
echo "=========================================================================="
lb config \
    --mode debian \
    --distribution trixie \
    --architectures amd64 \
    --archive-areas "main contrib non-free non-free-firmware" \
    --binary-images iso-hybrid \
    --iso-application "MindSet OS" \
    --iso-publisher "LKS PVT LTD" \
    --iso-volume "MINDSET_V1" \
    --memtest none \
    --win32-loader false \
    --security false \
    --apt-recommends true

echo "=========================================================================="
echo " Stage 3/3: Assembling Live & Installable ISO (lb build)..."
echo " (This stage bootstraps Debian Trixie chroot, packages, and kernel)"
echo "=========================================================================="
lb build

# 7. Post-Build Standardization: Rename to mindset-v1.iso and generate SHA256
echo "=========================================================================="
echo " Build Assembly Completed! Finalizing ISO Image..."
echo "=========================================================================="

GENERATED_ISO="$(ls -1 live-image-amd64.hybrid.iso live-image-amd64.iso 2>/dev/null | head -n 1 || true)"

if [ -n "$GENERATED_ISO" ] && [ -f "$GENERATED_ISO" ]; then
    mv -f "$GENERATED_ISO" mindset-v1.iso
    sha256sum mindset-v1.iso > mindset-v1.iso.sha256

    ISO_SIZE="$(du -h mindset-v1.iso | awk '{print $1}')"
    SHA_VAL="$(cat mindset-v1.iso.sha256 | awk '{print $1}')"

    echo "=========================================================================="
    echo " SUCCESS! MindSet OS Debian 13 Live ISO Ready:"
    echo " File:      $PWD/mindset-v1.iso"
    echo " Size:      $ISO_SIZE"
    echo " SHA256:    $SHA_VAL"
    echo " Checksum:  $PWD/mindset-v1.iso.sha256"
    echo "=========================================================================="
    echo " Deployment Instructions:"
    echo " 1. Real PC (Ventoy): Copy 'mindset-v1.iso' onto your Ventoy USB drive."
    echo " 2. Real PC (Rufus):  Select 'mindset-v1.iso' -> Target: UEFI (or MBR/BIOS)."
    echo " 3. VirtualBox / VMware: Attach 'mindset-v1.iso' as optical drive (RAM >= 4GB)."
    echo "=========================================================================="
else
    echo "ERROR: ISO file was not found after lb build! Check build logs for errors."
    exit 1
fi
