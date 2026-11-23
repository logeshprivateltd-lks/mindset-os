# MindSet OS — Debian 13 (Trixie) Live & Installable ISO Build System

## 1. Overview & Directory Topology

The `iso-builder/` directory provides a self-contained, reproducible Debian 13 (Trixie) amd64 build engine powered by Debian `live-build`. The resulting **`mindset-v1.iso`** is an installable hybrid ISO capable of booting on both modern 64-bit UEFI hardware and Legacy BIOS PCs, as well as VirtualBox and VMware environments.

```
iso-builder/
├── build_mindset_iso.sh                    # Live-build orchestrator (trixie, amd64, hybrid ISO)
├── vm_one_click_build.sh                   # One-command automated VM build & compilation pipeline
├── README_ISO_BUILD.md                     # Complete cross-compilation, flashing & VM guide
├── auto/
│   ├── config                              # lb config wrapper for trixie + non-free-firmware
│   ├── clean                               # lb clean wrapper
│   └── build                               # lb build wrapper
└── config/
    ├── package-lists/
    │   └── mindset.list.chroot             # Firmware, PipeWire, Xorg, Chromium, Noto fonts
    ├── hooks/
    │   └── live/
    │       ├── 0100-user-setup.hook.chroot # Configures 'achiever' user & sudo permissions
    │       ├── 0200-services.hook.chroot   # Enables NetworkManager, PipeWire, font caches
    │       └── 0300-boot-splash.hook.binary# Injects "MindSet OS v1.0 — Boot Deep Focus"
    └── includes.chroot/
        ├── etc/
        │   ├── systemd/system/getty@tty1.service.d/
        │   │   └── autologin.conf          # Systemd tty1 autologin for user 'achiever'
        │   ├── sudoers.d/
        │   │   └── 01-achiever-nopasswd    # Passwordless sudo for live session
        │   ├── skel/
        │   │   ├── .bash_profile           # Auto-starts X11 on tty1
        │   │   ├── .xinitrc                # D-Bus & Openbox session launcher
        │   │   └── .config/openbox/
        │   │       ├── autostart           # Screen blanking off, PipeWire, GPU launcher
        │   │       └── rc.xml              # Frameless full-screen rules for MindSet OS
        │   └── NetworkManager/
        │       └── NetworkManager.conf     # Unmanaged device prevention
        ├── usr/local/bin/
        │   ├── mindset-os-launch           # Smart GPU probe (Vulkan vs Software GL fallback)
        │   ├── mindset-browser             # Chromium isolated study profile wrapper
        │   └── mindset-setup-ollama        # Optional offline local AI tutor installer
        └── opt/mindset-os/
            ├── bin/                        # Runtime home for mindset-os binary
            └── assets/                     # Bundled Noto Indic fonts, icons & wallpapers
```

---

## 2. Key Technical Implementations

### A. Hybrid UEFI + Legacy BIOS Boot Engine
- Configured with `--binary-images iso-hybrid` and `--bootloaders "grub-efi,syslinux"`.
- Uses official Debian 13 archive components: `main contrib non-free non-free-firmware`.
- Boots seamlessly from Ventoy USB, Rufus (GPT/MBR), and direct `dd` imaging.

### B. Hardware Firmware & Audio/Display Subsystems
- **Wi-Fi & Bluetooth Firmware:** Pre-bundles Intel (`firmware-iwlwifi`), Realtek (`firmware-realtek`), Atheros (`firmware-atheros`), and Linux non-free microcode.
- **Modern Low-Latency Audio:** Full PipeWire 1.2+ stack with WirePlumber session manager and PulseAudio emulation (`pipewire-pulse`).
- **Typography:** System-wide `fonts-noto-core`, `fonts-noto-ui-core`, `fonts-noto-color-emoji`, plus bundled Noto Tamil and Malayalam fonts.

### C. Zero-Friction Auto-Login & True OS Experience
- **Auto-Login:** Systemd `agetty` automatically logs user `achiever` into `tty1` on boot.
- **Instant Graphical Shell:** `.bash_profile` automatically runs `startx`, entering Openbox.
- **Window Management:** Openbox `rc.xml` strips borders and maximizes `mindset-os` to eliminate desktop environment clunkiness, while allowing child windows (Chromium, YouTube, dialogs) to float and capture keyboard/mouse focus.

### D. Smart GPU & Virtualization Fallback (`mindset-os-launch`)
- Probes `/dev/dri/renderD*` and executes `vulkaninfo --summary`.
- If native Vulkan is supported (bare-metal Intel, AMD, or Nvidia), exports `WGPU_BACKEND=vulkan`.
- If running under VirtualBox, VMware, or QEMU without 3D acceleration, automatically engages Mesa software rendering:
  ```bash
  export WGPU_BACKEND="gl"
  export LIBGL_ALWAYS_SOFTWARE=1
  export MESA_LOADER_DRIVER_OVERRIDE="llvmpipe"
  ```
- Includes an auto-recovery loop that catches unexpected exits and gracefully fails over to the software rasterizer.

---

## 3. Build & Execution Workflow

To generate the ISO from any Debian/Ubuntu VM or WSL2:
```bash
cd ~/mindset-workspace/iso-builder
sudo ./vm_one_click_build.sh
```
Output: `mindset-v1.iso` and `mindset-v1.iso.sha256`.
