#!/bin/sh
echo $DEVKITPRO
command -v elf2dol > /dev/null 2>&1
if [ $? -ne 0 ]; then
        printf "\e[31mError: elf2dol is not installed.\e[0m"
fi
mode="release"

if [ "$1" = "-d" ]; then

    mode="debug"

fi

export DEVKITPRO=/opt/devkitpro
export PATH=$PATH:${DEVKITPRO}/devkitPPC/bin
export PATH=${DEVKITPRO}/tools/bin:$PATH
if [ -f boot.elf ] && [ -f boot.dol ]; then
    rm boot.elf boot.dol
fi
cargo +nightly build -Zbuild-std=core,alloc --target powerpc-unknown-eabi.json --$mode
if [ $? -eq 0 ]; then
    cp target/powerpc-unknown-eabi/$mode/indigo boot.elf
    elf2dol boot.elf boot.dol
fi