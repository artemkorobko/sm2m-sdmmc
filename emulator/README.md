# SM2M SDMMC Adapter

Hard drive replacement solution for SM2M computing units based on STM32F103VBT6 MCU.

This solution implements default communication protocol for easy switch between existing mechanical hard drive and new one which uses SD card as a main storage.

Software implementation is based on [RTIC](https://rtic.rs/1/book/en/preface.html) framework.

# Capabilities
- 16-bit Parallel interface.
- MicroSD card support.
- 10K internal buffer.
- Status LED indicators.

The file name on SD card is generated after the 16 bit starting address (sent from SM2M) with `.bin` extention and has the following format `<address>.bin`. As an example, the file can be named starting form `0.bin` up to `65535.bin`.

[SM2M SDMMC Adapter Bus Documentation](doc/BUS.md)  
[SM2M SDMMC Adapter Functional Design](doc/FUNC.md)

# Prerequisites
## Rust
- Install Rust toolchain by following the instructions on https://rustup.rs.
- Install the `rust-std` component `thumbv7m-none-eabi` to cross-compile for ARM Cortex-M3 MCU using the following command:
```bash
rustup target add thumbv7m-none-eabi
```
- Install `cargo-binutils` subcommands to invoke the LLVM tools shipped with the Rust toolchain, `cargo-flash` to be able to flash target MCU, `cargo-embed` to vew logs and debug MCU in terminal and `probe-rs-debugger` to be able to debug MCU from VSCode.
```bash
cargo install cargo-binutils cargo-flash cargo-embed probe-rs-debugger
```
- Install `llvm-tools-preview` component for binary inspection.
```bash
rustup component add llvm-tools-preview
```

## ARM GCC extension for Mac
Before installing extension make sure you have updated [Homebrew](https://brew.sh) packages.
- Install ARM gcc extension.
```bash
brew install armmbed/formulae/arm-none-eabi-gcc
```
- Ensure extension has been installed
```bash
arm-none-eabi-gcc -v
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
    - [Debugger for probe-rs](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger)
    - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

# Build and flash firmware
Before flashing make sure probe has been attached to your computer and operates properly using command `cargo flash --list-probes`.

## Build and flash debug version of firmware
```
cargo flash --chip STM32F107VC
```

## Build and flash release version of firmware
```
cargo flash --release --chip STM32F107VC
```

# Run and monitor firmware

## Run debug version of firmware
```
cargo embed
```

## Run release version of firmware
```
DEFMT_LOG=debug cargo embed --release
```

# Links

[cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart)  
[rtic.rs](https://rtic.rs/1/book/en/)  
[probe.rs](https://probe.rs)