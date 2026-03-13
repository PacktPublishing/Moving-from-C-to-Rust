# Chapter 9 - Working with C++ in Rust

This directory contains examples for working with C and C++ from Rust using FFI (Foreign Function Interface).

## Examples

### 1. bind_c_manual - Manual SDL2 bindings
Demonstrates manually writing FFI bindings to call C libraries (SDL2).

**Requirements:** SDL2 development libraries must be installed on your system.
- Ubuntu/Debian: `sudo apt-get install libsdl2-dev`
- macOS: `brew install sdl2`
- Windows: Download SDL2 development libraries

```bash
cd bind_c_manual
cargo run
```

### 2. from_c - Exposing Rust to C
Shows how to create a Rust library that can be called from C code.

```bash
cd from_c
cargo build

# Linux/macOS
clang -o test -L ./target/debug/ -lfrom_c -Wl,-rpath=target/debug test.c
./test

# Windows (MSVC) - Note: Rust creates from_c.dll.lib, not from_c.lib
clang -o test.exe -L .\target\debug\ from_c.dll.lib test.c
```

### 3. bind_c_bindgen - Automatic bindings with bindgen
Uses bindgen to automatically generate Rust bindings from C headers.

**Requirements:** libclang must be installed.

```bash
cd bind_c_bindgen
# Downloads and builds SDL2 automatically
cargo build
```

### 4. bind_cpp_manual - Manual C++ bindings
Creates a C API wrapper around C++ code (toml++ library).

**Requirements:** C++ compiler with C++17 support.

```bash
cd bind_cpp_manual
cargo run
```

### 5. bind_cpp_macros - Using the cpp crate
Embeds C++ code directly in Rust using the cpp crate.

**Requirements:** C++ compiler with C++17 support.

```bash
cd bind_cpp_macros
cargo run
```

### 6. sdl_safe_wrapper - Safe wrappers for unsafe FFI
Demonstrates creating safe Rust abstractions over unsafe FFI code.

**Requirements:** SDL2 development libraries (same as bind_c_manual).

```bash
cd sdl_safe_wrapper
cargo run
```

## Important Notes

- FFI code is inherently `unsafe` - these examples show both the unsafe bindings and how to create safe wrappers
- Many examples require external system libraries (SDL2, C++ toolchain)
- The `bind_c_bindgen` example downloads and builds SDL2 automatically as part of the build process
- Cross-platform differences:
  - Windows creates `library.dll.lib` files for dynamic libraries (MSVC)
  - SDL2 library names differ: Windows uses `SDL2main` and `SDL2-static`, Linux uses `SDL2`
  - The `bind_c_bindgen/build.rs` demonstrates platform-specific linking with `cfg!(target_os)`
- See individual example directories for specific build requirements
