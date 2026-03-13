# Rust for C++ Developers - Code Examples

This repository contains code examples from the book "Rust for C++ Developers" by Packt Publishing.

## Repository Structure

### Chapter 6 - Reading and Writing Files

The `chapter_06/` directory contains examples demonstrating:
- Working with filesystem paths (`PathBuf`, `Path`)
- Error handling with `Result<T, E>` and the `?` operator
- High-level I/O abstractions (`read_to_string`, `read`, `write`)
- Working with files using `File`, `OpenOptions`
- Fundamental I/O traits (`Read`, `Write`, `Seek`)
- Buffered I/O with `BufReader` and `BufWriter`
- In-memory I/O with `Cursor`
- Byte order handling with the `byteorder` crate
- Serialization/deserialization with `serde` (JSON and TOML)

**Key files:**
- `src/bin/paths.rs` - Path manipulation examples
- `src/bin/error_handling.rs` - Error handling patterns
- `src/bin/file_io.rs` - File operations
- `src/bin/traits.rs` - I/O trait usage (Read, Write, Seek)
- `src/bin/byteorder_example.rs` - Endianness handling
- `src/bin/serde_test.rs` - JSON/TOML serialization
- `pets.json` - Sample data file for serde examples
- `file.txt` - Sample text file for I/O examples

**Running the examples:**
```bash
cd chapter_06

# Run specific examples
cargo run --bin paths
cargo run --bin error_handling
cargo run --bin file_io
cargo run --bin traits
cargo run --bin byteorder_example
cargo run --bin serde_test
```

### Chapter 9 - Working with C++ in Rust (FFI)

The `chapter_09/` directory contains examples for Foreign Function Interface (FFI):
- Manual C bindings (SDL2)
- Exposing Rust functions to C
- Automatic binding generation with `bindgen`
- Manual C++ bindings with C API wrappers
- Using the `cpp` crate for inline C++
- Creating safe Rust wrappers for unsafe FFI code

**Key examples:**
- `bind_c_manual/` - Manual SDL2 FFI bindings
- `from_c/` - Creating Rust libraries callable from C
- `bind_c_bindgen/` - Automatic binding generation with bindgen
- `bind_cpp_manual/` - C++ bindings via C API wrapper
- `bind_cpp_macros/` - Inline C++ with the cpp crate
- `sdl_safe_wrapper/` - Safe abstractions over unsafe FFI

**Important Notes:**
- Many examples require external system libraries (SDL2)
- Some require C++ toolchain and C++17 support
- `bind_c_bindgen` downloads and builds SDL2 automatically
- See individual example READMEs for specific requirements

**Running the examples:**
```bash
cd chapter_09

# Simple example (Rust to C)
cd from_c
cargo build

# Examples requiring SDL2 (must be installed)
cd bind_c_manual
cargo run

cd sdl_safe_wrapper
cargo run

# Examples with automatic builds
cd bind_c_bindgen
cargo build

cd bind_cpp_manual
cargo run
```

### Chapter 10 - Optimization in Rust

The `chapter_10/` directory contains a complete performance testing project demonstrating:
- Matrix multiplication with multiple optimization strategies
- Profiling with `samply` and flame graphs
- Benchmarking with Criterion
- Unsafe Rust for performance optimization
- SIMD optimization (requires nightly Rust)

**Key files:**
- `src/lib.rs` - Library with matrix multiplication implementations
- `src/bin/cli.rs` - CLI tool for running performance tests
- `src/simd.rs` - SIMD-optimized matrix multiplication (nightly only)
- `benches/multiply.rs` - Criterion benchmarks

**Running the examples:**
```bash
cd chapter_10

# Build and run the CLI
cargo run --release -- 100 100 100

# Run benchmarks
cargo bench

# Run with nightly and SIMD (requires nightly toolchain)
cargo +nightly bench --features nightly
```

### Chapter 11 - Multithreading in Rust

The `chapter_11/` directory contains examples demonstrating Rust's "fearless concurrency":
- Creating and managing threads with `std::thread`
- Thread spawning and JoinHandle
- Move closures for data ownership
- Scoped threads for non-static data
- Thread pooling with `rayon`
- Safe shared state with `Arc<Mutex<T>>`
- Parallel iterators with `rayon::prelude`
- MPSC channels for thread communication
- Atomic operations and lock-free programming

**Key files:**
- `src/bin/basic_threads.rs` - Thread spawning and JoinHandle
- `src/bin/move_closures.rs` - Transferring data to threads
- `src/bin/scoped_threads.rs` - Scoped threads and data sharing
- `src/bin/rayon_pool.rs` - Thread pooling with rayon
- `src/bin/arc_mutex.rs` - Shared mutable state pattern
- `src/bin/parallel_iterators.rs` - Parallel iteration with rayon
- `src/bin/channels.rs` - MPSC channel communication
- `src/bin/atomics.rs` - Atomic operations and CAS

**Running the examples:**
```bash
cd chapter_11

# Check all examples compile
cargo check

# Run specific examples
cargo run --bin basic_threads
cargo run --bin move_closures
cargo run --bin scoped_threads
cargo run --bin rayon_pool
cargo run --bin arc_mutex
cargo run --bin parallel_iterators
cargo run --bin channels
cargo run --bin atomics
```

### Chapter 12 - Metaprogramming with Macros

The `chapter_12/` directory contains examples of Rust macros:
- Declarative macros (`macro_rules!`)
- Procedural macros (custom derive, attributes, function-like)

**Key components:**
- `src/main.rs` - Examples using all macro types
- `prettyprint_derive/` - Custom derive macro for pretty printing structs
- `log_call/` - Attribute macro for logging function calls
- `stringulate/` - Function-like macro that converts tokens to strings

**Running the examples:**
```bash
cd chapter_12
cargo run
```

## Requirements

- Rust 1.82.0 or newer
- For Chapter 10 SIMD examples: Rust nightly toolchain
- For Chapter 10 profiling: `cargo install --locked samply`

