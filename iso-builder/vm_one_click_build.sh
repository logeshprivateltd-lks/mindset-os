#!/bin/bash
# ==============================================================================
# MindSet OS — One-Command Linux VM Build Wrapper
# Automates: Environment Checks -> Native Compilation -> ISO Assembly -> SHA256
# ==============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=========================================================================="
echo "    MindSet OS — One-Command Live ISO Automated Pipeline (Debian 13)    "
echo "=========================================================================="

# 1. Native Linux Filesystem Interlock Check
FS_TYPE="$(stat -f -c %T "$SCRIPT_DIR" 2>/dev/null || df -T "$SCRIPT_DIR" | tail -1 | awk '{print $2}')"
IS_WINDOWS_MOUNT=false

case "$FS_TYPE" in
    vboxsf|drvfs|9p|cifs|smb*|fuse.sshfs)
        IS_WINDOWS_MOUNT=true
        ;;
esac

if [[ "$SCRIPT_DIR" == /mnt/* ]] || [ "$IS_WINDOWS_MOUNT" = true ]; then
    echo "=========================================================================="
    echo "CRITICAL: WINDOWS MOUNT DETECTED ($FS_TYPE at $SCRIPT_DIR)"
    echo "Debian live-build requires a native Linux filesystem (ext4/btrfs/xfs) with"
    echo "POSIX permissions, hardlinks, and device node (mknod) capabilities."
    echo ""
    echo "AUTOMATIC WORKSPACE RELOCATION:"
    TARGET_DIR="/home/${SUDO_USER:-$USER}/mindset-workspace"
    echo "Cloning workspace to native Linux path: $TARGET_DIR..."
    mkdir -p "$TARGET_DIR"
    cp -r "$WORKSPACE_ROOT"/* "$TARGET_DIR/"
    echo "Done! Please run the build inside the native directory:"
    echo "  cd $TARGET_DIR/iso-builder"
    echo "  sudo ./vm_one_click_build.sh"
    echo "=========================================================================="
    exit 1
fi

echo "[Environment] Verified Native Linux Filesystem ($FS_TYPE). Proceeding..."

# 2. Ensure Sudo / Root Access
if [ "$EUID" -ne 0 ]; then
    echo "Re-executing with sudo privileges for live-build chroot execution..."
    exec sudo -E "$0" "$@"
fi

# 3. Check / Install Rust Toolchain & C System Libraries
echo "=========================================================================="
echo " Step 1/4: Checking Rust Compiler & Native Build Dependencies..."
echo "=========================================================================="

BUILD_DEPS="build-essential pkg-config libssl-dev libasound2-dev libx11-dev libxcursor-dev libxi-dev libxrandr-dev libvulkan-dev curl"
MISSING_BUILD_DEPS=""
for dep in $BUILD_DEPS; do
    if ! dpkg -s "$dep" >/dev/null 2>&1; then
        MISSING_BUILD_DEPS="$MISSING_BUILD_DEPS $dep"
    fi
done

if [ -n "$MISSING_BUILD_DEPS" ]; then
    echo "[Deps] Installing build packages:$MISSING_BUILD_DEPS..."
    apt-get update -y
    apt-get install -y --no-install-recommends $MISSING_BUILD_DEPS
fi

if ! which cargo >/dev/null 2>&1; then
    echo "[Rust] Cargo not found on PATH. Checking /root/.cargo/bin or \$HOME/.cargo/bin..."
    if [ -f "/home/${SUDO_USER:-$USER}/.cargo/env" ]; then
        source "/home/${SUDO_USER:-$USER}/.cargo/env"
    elif [ -f "/root/.cargo/env" ]; then
        source "/root/.cargo/env"
    else
        echo "[Rust] Installing Rust toolchain via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
fi

echo "[Rust Toolchain] $(cargo --version) detected."

# 4. Compile MindSet OS Native Linux Release Binary
echo "=========================================================================="
echo " Step 2/4: Compiling mindset-os Native Binary (Release Mode)..."
echo "=========================================================================="
cd "$WORKSPACE_ROOT"

# Run cargo build as original user if invoked via sudo to respect user's cargo cache
if [ -n "$SUDO_USER" ] && [ "$SUDO_USER" != "root" ]; then
    echo "[Cargo] Compiling as user '$SUDO_USER'..."
    su - "$SUDO_USER" -c "cd '$WORKSPACE_ROOT' && cargo build --release --bin mindset-os"
else
    cargo build --release --bin mindset-os
fi

RELEASE_BIN="$WORKSPACE_ROOT/target/release/mindset-os"
if [ ! -f "$RELEASE_BIN" ]; then
    echo "ERROR: Compilation failed! Binary not found at $RELEASE_BIN"
    exit 1
fi
echo "[Cargo] Compiled successfully: $RELEASE_BIN ($(du -h "$RELEASE_BIN" | awk '{print $1}'))"

# 5. Populate ISO Chroot Overlay
echo "=========================================================================="
echo " Step 3/4: Staging Binary, Fonts & Assets into Chroot Overlay..."
echo "=========================================================================="
cd "$SCRIPT_DIR"

mkdir -p config/includes.chroot/opt/mindset-os/bin
mkdir -p config/includes.chroot/opt/mindset-os/assets/fonts
mkdir -p config/includes.chroot/opt/mindset-os/assets/icons
mkdir -p config/includes.chroot/usr/local/bin
mkdir -p config/includes.chroot/usr/share/fonts/truetype/mindset

# Copy binary
cp -f "$RELEASE_BIN" config/includes.chroot/opt/mindset-os/bin/mindset-os
cp -f "$RELEASE_BIN" config/includes.chroot/usr/local/bin/mindset-os
chmod +x config/includes.chroot/opt/mindset-os/bin/mindset-os
chmod +x config/includes.chroot/usr/local/bin/mindset-os

# Copy fonts (Tamil, Malayalam, English Noto Sans)
if [ -d "$WORKSPACE_ROOT/assets/fonts" ]; then
    echo "[Assets] Bundling Noto Indic & Latin typography..."
    cp -u "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/opt/mindset-os/assets/fonts/ 2>/dev/null || true
    cp -u "$WORKSPACE_ROOT/assets/fonts"/*.ttf config/includes.chroot/usr/share/fonts/truetype/mindset/ 2>/dev/null || true
fi

# Copy icons
if [ -d "$WORKSPACE_ROOT/assets/icons" ]; then
    echo "[Assets] Bundling SVG icons and branding..."
    cp -u "$WORKSPACE_ROOT/assets/icons"/* config/includes.chroot/opt/mindset-os/assets/icons/ 2>/dev/null || true
fi

# 6. Execute ISO Build Script
echo "=========================================================================="
echo " Step 4/4: Launching Debian live-build Engine..."
echo "=========================================================================="
./build_mindset_iso.sh

echo "=========================================================================="
echo " ONE-CLICK BUILD COMPLETE!"
echo " Output Image: $PWD/mindset-v1.iso"
echo " Checksum:     $(cat mindset-v1.iso.sha256)"
echo "=========================================================================="
