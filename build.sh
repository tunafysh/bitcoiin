#!/bin/bash

printf "\e[1;36mSetting up...\n"
export DEVKITPRO=/opt/devkitpro
export DEVKITARM=${DEVKITPRO}/devkitARM
export DEVKITPPC=${DEVKITPRO}/devkitPPC
export PATH=${DEVKITPRO}/tools/bin:$PATH
export PATH=$PATH:${DEVKITPRO}/devkitPPC/bin
printf "\e[1;32mDone.\n"

0printf "\e[1;36mCompiling...\n"
cargo +nightly build -Zbuild-std=core,alloc --target powerpc-unknown-eabi.json

if [ $? -ne 0 ]; then
    printf "\e[1;31mError in target\n"
    exit 1
else 
    printf "\e[1;32mDone.\n"
fi

0printf "\e[1;36mCopying file and creating Homebrew app...\n"
if ! command -v elf2dol &> /dev/null; then
    printf "\e[1;31melf2dol could not be found. Please install it.\n"
    exit 1
fi

elf2dol target/powerpc-unknown-eabi/debug/bitcoiin boot.dol

if [[ -d build ]]; then
    rm -rf build
fi

mkdir -p build/apps/bitcoiin
cp boot.dol build/apps/bitcoiin/boot.dol
cp assets/meta.xml build/apps/bitcoiin/meta.xml
cp assets/icon.png build/apps/bitcoiin/icon.png
cd build || exit
zip -r bitcoiin.zip apps
cp bitcoiin.zip ../bitcoiin.zip
cd ..
rm -rf build
printf "\e[1;32mDone.\n"