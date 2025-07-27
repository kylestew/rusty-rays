# Rusty Rays

__PROGRESS__
https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects

A Rust executable project for ray tracing. Based on [Ray Tracing in One Weekend](https://raytracing.github.io/) by Peter Shirley.

## Getting Started

### Prerequisites

- Rust and Cargo (install from [https://rustup.rs/](https://rustup.rs/))
- WASM toolchain

### Building

Desktop:

```bash
cargo build
```

WASM:

```bash
cargo build --lib --release --target wasm32-unknown-unknown
```

### Running

```bash
cargo run
```

## Project Structure

- `src/main.rs`: Entry point for the application
- `src/lib.rs`: WASM library code (not for desktop)
- `Cargo.toml`: Project configuration and dependencies

## License

This project is open source. 
