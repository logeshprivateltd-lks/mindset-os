# ~/.bash_profile
# Automatically launch graphical MindSet OS session on tty1 login

if [ -z "$DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then
    exec startx -- -keeptty > ~/.xsession-errors 2>&1
fi
