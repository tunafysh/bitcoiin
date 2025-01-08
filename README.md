# bitcoiin
A bitcoin miner for nintendo Wii that uses rust

Build command:
```sh
cargo +nightly build -Zbuild-std=core,alloc --target powerpc-unknown-eabi.json
```

then use `elf2dol` to convert the elf file