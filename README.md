# SM2M SDMMC

Hard drive replacement solution for SM2M computing units based on STM32F103VBT6 MCU.

This solution implements default communication protocol for easy switch between existing mechanical hard drive and new one which uses SD card as a main storage.

Software implementation is based on [RTIC](https://rtic.rs/1/book/en/preface.html) framework.

# Capabilities
- 16-bit Parallel interface.
- MicroSD card support.
- 10K internal buffer.
- Status LED indicators.

The file name on SD card is formatted based on a 16 bit starting address (sent from SM2M) with `.bin` extention and has the following format `<address>.bin`. As an example, the file can be named starting form `0.bin` up to `65535.bin`.

# Prerequisites
## Rust
- Install Rust toolchain by following the instructions on https://rustup.rs.
- Install the `rust-std` component `thumbv7em-none-eabihf` to cross-compile for ARM Cortex-M4 MCU using the following command:
```bash
rustup target add thumbv7em-none-eabihf
```
- Install `cargo-binutils` subcommands to invoke the LLVM tools shipped with the Rust toolchain.
```bash
cargo install cargo-binutils 
```
- Install `llvm-tools-preview` component for binary inspection.
```bash
rustup component add llvm-tools-preview
```

## ARM GCC extension for Mac
Before installing extension make sure you have updated [Homebrew](https://brew.sh) packages.
- Install ARM gcc extension and open on-chip debugger.
```bash
brew install armmbed/formulae/arm-none-eabi-gcc openocd
```
- Ensure extension has been installed
```bash
arm-none-eabi-gcc -v
```
```bash
openocd -v
```

## VS Build Tools for Windows
Download the Visual Studio 2019 Build tools from the Microsoft website: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

During installation in the `Workloads` tab select `Desktop development with C++`. Select the following items on the `Installation details` page:
- MSVC v142 - VS 2019 C++ ...
- Windows 10 SDK ...
- C++ CMake tools for Windows

You can find more information about the embedded toolchains here https://docs.rust-embedded.org/book/intro/index.html.

## Visual Studio Code
- Install Visual Studio Code from https://code.visualstudio.com.
- Install the following extensions:  
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)  
    - [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)  
    - [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)  
    - [vscode-rustfmt](https://marketplace.visualstudio.com/items?itemName=statiolake.vscode-rustfmt)  
    - [Cortex-Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)  
    - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

# Build and upload debug version of firmware
```
cargo build && \
openocd -f ./openocd.cfg -c "init" -c "reset init" -c "flash write_image erase ./target/thumbv7m-none-eabi/debug/sm2m-sdmmc" -c "reset run" -c "exit"
```

# Run debug version of firmware
1. Run `openocd -f ./openocd.cfg` in separate terminal to connect to the board.
2. Run `cargo run` in separate terminal to connect GDB to the OpenOCD from step 1.
3. Execute `c` command to continue execution.
4. Observe debug logs in terminal from step 1.

In order to subsequent firmware upload run `cargo build`.

5. Press `Ctrl+C` in terminal from step 2.
6. Execute `load` command to upload built firmware from step 5 to the board.
7. Execute `c` command to continue execution.
8. Observe debug logs in terminal from step 1.

# Build and upload release version of firmware
```
cargo build --release && \
openocd -f ./openocd.cfg -c "init" -c "reset init" -c "flash write_image erase ./target/thumbv7m-none-eabi/release/sm2m-sdmmc" -c "reset run" -c "exit"
```

# Links

https://github.com/rust-embedded/cortex-m-quickstart
