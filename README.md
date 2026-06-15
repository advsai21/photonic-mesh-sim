# High-Performance Photonic Mesh Simulator Core

A bare-metal, zero-heap-allocation optical hardware accelerator simulator built from scratch in Rust. This engine bypasses standard digital boolean logic by simulating continuous electromagnetic light wave interactions (phase and amplitude) passing through a programmable 2D silicon waveguide mesh.

## 🚀 Key Features
- **Pure Rust Performance:** Built completely from scratch with zero external dependencies, leveraging stack-allocated memory layout primitives for blistering execution speed.
- **Mach-Zehnder Interferometer (MZI) Physics:** Realistically models the constructive and destructive wave interference mechanics used in next-generation optical processors.
- **AOT Phase Decomposition Compiler:** Features an Ahead-of-Time compiler frontend that takes abstract mathematical target matrices and analytically solves the inverse-optics equations to generate physical hardware phase shifts ($\theta$ and $\phi$).
- **Clements 2D Grid Topology:** Simulates a 4-channel cascading waveguide lattice mesh to execute high-dimensional linear algebra transformations at the simulated speed of light.

## 📐 Hardware Architecture Model
Instead of tracking standard electrical voltages, the engine models light waves as complex numbers where:

$$E = A \cdot e^{i\phi} = \text{real} + \text{imaginary} \cdot i$$

By controlling the physical phase shifters inside the MZI grid mesh, the system alters wave phases to route energy or solve complex matrix-vector multiplications ($Y = W \cdot X$) instantly through optical propagation.

## 💻 Quick Start

### Prerequisites
Ensure you have the Rust toolchain installed:
```bash
cargo --version
Run the Simulator Grid
Clone the repository and compile the hardware architecture using the release optimization pipeline:

Bash
cargo run --release
Example Simulation Output
Plaintext
=============================================================
📡 HARDWARE LOCK-IN: PURE RUST 4-CHANNEL PHOTONIC MESH SIMULATOR
=============================================================
Input State Vector Intensities: [1.0, 0.0, 0.0, 0.0]

--- PHOTONIC ACCELERATION GRID COMPLETE ---
=> Waveguide Port Output [0]: Intensity = 0.1770
=> Waveguide Port Output [1]: Intensity = 0.3541
=> Waveguide Port Output [2]: Intensity = 0.2312
=> Waveguide Port Output [3]: Intensity = 0.0875
-------------------------------------------------------------
Lattice Energy Checksum: 0.8498 (Physical Integrity Valid)
=============================================================