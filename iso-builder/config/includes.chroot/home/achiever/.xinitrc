#!/bin/bash
# ~/.xinitrc — MindSet OS X11 Session Entrypoint

export XDG_SESSION_TYPE=x11
export XDG_CURRENT_DESKTOP=MindSetOS
export DESKTOP_SESSION=mindset

# 1. Start D-Bus session bus if not already running
if [ -z "$DBUS_SESSION_BUS_ADDRESS" ]; then
    eval $(dbus-launch --sh-syntax --exit-with-session)
fi

# 2. Export PulseAudio / PipeWire runtime path
export PULSE_SERVER="unix:/run/user/$(id -u)/pulse/native"

# 3. Start minimal Openbox window manager session
exec openbox-session
