#!/bin/bash
# ==============================================================================
# MindSet OS — Reproducible Debian 13 ("Trixie") Live ISO Builder
# Output: mindset-os-amd64.hybrid.iso + mindset-os-amd64.hybrid.iso.sha256
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=========================================================================="
echo "      MindSet OS — Debian 13 (Trixie) Hybrid Live ISO Build Pipeline      "
echo "=========================================================================="

# 1. Superuser Check
if [ "$(id -u)" -ne 0 ]; then
    echo "ERROR: Run as root: sudo ./build-iso.sh"
    exit 1
fi

# 2. Native Linux Filesystem Safety Check (never build on Windows /mnt/c mounts)
FS_TYPE="$(stat -f -c %T . 2>/dev/null || df -T . | tail -1 | awk '{print $2}')"
case "$FS_TYPE" in
    vboxsf|drvfs|9p|cifs|smb*|fuse.sshfs)
        echo "ERROR: Unsupported non-Linux filesystem ($FS_TYPE). Copy repo to ext4/btrfs first."
        exit 1
        ;;
    *)
        echo "[FS Check] Native Linux filesystem ($FS_TYPE). OK."
        ;;
esac

echo "[Disk Check] Free space:"
df -h .

# 3. Verify Host Build Tools
for cmd in lb debootstrap xorriso mksquashfs; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "ERROR: Missing required tool '$cmd'. Install live-build debootstrap xorriso squashfs-tools."
        exit 1
    fi
done

# 4. Stage Binaries, Wallpapers, Icons & Indic Fonts
echo "[Stage] Preparing overlay files in config/includes.chroot/..."
mkdir -p \
    config/includes.chroot/usr/local/bin \
    config/includes.chroot/usr/share/mindset-os/icons \
    config/includes.chroot/usr/share/mindset-os/fonts \
    config/includes.chroot/usr/share/wayland-sessions \
    config/includes.chroot/usr/share/fonts/truetype/mindset \
    config/includes.chroot/opt/mindset-os/bin \
    config/includes.chroot/opt/mindset-os/assets/fonts \
    config/includes.chroot/opt/mindset-os/assets/icons \
    config/includes.chroot/etc/greetd \
    config/includes.chroot/etc/skel/.config/mindset-os/labwc \
    config/includes.chroot/etc/NetworkManager \
    config/includes.chroot/etc/sudoers.d \
    config/includes.chroot/etc/systemd/system/getty@tty1.service.d

chmod +x auto/* 2>/dev/null || true
chmod +x config/hooks/live/* 2>/dev/null || true
chmod +x config/includes.chroot/usr/local/bin/* 2>/dev/null || true
chmod +x config/includes.chroot/etc/skel/.config/mindset-os/labwc/autostart 2>/dev/null || true

if [ -f "$WORKSPACE_ROOT/assets/wallpaper.jpg" ]; then
    cp -f "$WORKSPACE_ROOT/assets/wallpaper.jpg" config/includes.chroot/usr/share/mindset-os/wallpaper.jpg
    cp -f "$WORKSPACE_ROOT/assets/wallpaper.jpg" config/includes.chroot/opt/mindset-os/assets/wallpaper.jpg
fi
if [ -d "$WORKSPACE_ROOT/assets/fonts" ]; then
    cp -f "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/usr/share/mindset-os/fonts/ 2>/dev/null || true
    cp -f "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/usr/share/fonts/truetype/mindset/ 2>/dev/null || true
    cp -f "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/opt/mindset-os/assets/fonts/ 2>/dev/null || true
fi
if [ -d "$WORKSPACE_ROOT/assets/icons" ]; then
    cp -f "$WORKSPACE_ROOT/assets/icons"/* config/includes.chroot/usr/share/mindset-os/icons/ 2>/dev/null || true
    cp -f "$WORKSPACE_ROOT/assets/icons"/* config/includes.chroot/opt/mindset-os/assets/icons/ 2>/dev/null || true
fi

# Inject compiled Rust release binaries (prefer Debian 13 native target/release/)
SHELL_BIN=""
for candidate in \
    "$WORKSPACE_ROOT/target/release/mindset-shell" \
    "$WORKSPACE_ROOT/target/release/mindset-os" \
    "$WORKSPACE_ROOT/target/x86_64-unknown-linux-gnu/release/mindset-shell" \
    "$WORKSPACE_ROOT/target/x86_64-unknown-linux-gnu/release/mindset-os" \
    "config/includes.chroot/usr/local/bin/mindset-shell"; do
    if [ -x "$candidate" ]; then
        SHELL_BIN="$candidate"
        break
    fi
done

if [ -n "$SHELL_BIN" ]; then
    echo "[Binary] Staging release shell: $SHELL_BIN"
    if [ "$SHELL_BIN" != "config/includes.chroot/usr/local/bin/mindset-shell" ]; then
        cp -f "$SHELL_BIN" config/includes.chroot/usr/local/bin/mindset-shell
    fi
    cp -f config/includes.chroot/usr/local/bin/mindset-shell config/includes.chroot/usr/local/bin/mindset-os
    cp -f config/includes.chroot/usr/local/bin/mindset-shell config/includes.chroot/opt/mindset-os/bin/mindset-os
    chmod +x \
        config/includes.chroot/usr/local/bin/mindset-shell \
        config/includes.chroot/usr/local/bin/mindset-os \
        config/includes.chroot/opt/mindset-os/bin/mindset-os
else
    echo "ERROR: Compiled Linux release binary not found under $WORKSPACE_ROOT/target/release/."
    echo "Compile inside Debian 13 first: cargo build --release -p mindset-os --bin mindset-shell --bin mindset-os"
    exit 1
fi

if [ -x "$WORKSPACE_ROOT/target/release/mindset-focus-guard" ]; then
    cp -f "$WORKSPACE_ROOT/target/release/mindset-focus-guard" config/includes.chroot/usr/local/bin/mindset-focus-guard
    chmod +x config/includes.chroot/usr/local/bin/mindset-focus-guard
fi

# 5. Clean, Configure & Build Hybrid ISO
echo "[Phase E] Running lb clean && lb config && lb build..."
lb clean --purge
lb config
lb build 2>&1 | tee build.log

# 6. Standardize Filename & Compute SHA-256 Checksum
TARGET_ISO="mindset-os-amd64.hybrid.iso"
FOUND_ISO=""
for f in mindset-os-amd64.hybrid.iso live-image-amd64.hybrid.iso mindset-os.iso; do
    if [ -f "$f" ]; then
        FOUND_ISO="$f"
        break
    fi
done

if [ -z "$FOUND_ISO" ] || [ ! -f "$FOUND_ISO" ]; then
    echo "ERROR: Generated hybrid ISO not found! Check $PWD/build.log."
    exit 1
fi

if [ "$FOUND_ISO" != "$TARGET_ISO" ]; then
    mv -f "$FOUND_ISO" "$TARGET_ISO"
fi

sha256sum "$TARGET_ISO" > "${TARGET_ISO}.sha256"
ISO_BYTES="$(stat -c%s "$TARGET_ISO")"
ISO_HUMAN="$(du -h "$TARGET_ISO" | awk '{print $1}')"
SHA_VAL="$(awk '{print $1}' "${TARGET_ISO}.sha256")"

echo "=========================================================================="
echo " SUCCESS: MindSet OS Debian 13 Hybrid ISO Built"
echo " Image:    $PWD/$TARGET_ISO ($ISO_HUMAN / $ISO_BYTES bytes)"
echo " SHA-256:  $SHA_VAL"
echo " USB Flash (dd): sudo dd if=$TARGET_ISO of=/dev/sdX bs=4M status=progress oflag=sync"
echo "=========================================================================="
