FROM gitpod/workspace-full

RUN sudo apt-get update \
 && sudo apt-get install -y \
    nasm \
    qemu \
    binutils-arm-none-eabi \
    binutils-aarch64-linux-gnu \
 && sudo rm -rf /var/lib/apt/lists/*
