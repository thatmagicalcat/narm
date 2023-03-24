#!/usr/bin/env sh

qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-os.bin -display spice-app
