
# d3d8shim

A tiny experimental project for dynamic library loading that will eventually back a Direct3D 8 → Direct3D 9 shim.

Right now it’s a **cross-platform dynamic loader demo** using [`libloading`](https://crates.io/crates/libloading):

- On **Linux** it loads `libm.so.6` and looks up the `cos` symbol.
- On **Windows** (when built/ran there) it loads `d3d9.dll` and looks up `Direct3DCreate9`.

## Build and run

### Prerequisites

- Rust (stable)
- Cargo

### Linux

```bash
