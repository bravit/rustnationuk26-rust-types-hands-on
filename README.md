# Rust Types — Hands‑On Exercises

This repository contains a small collection of hands‑on exercises for practicing Rust’s type system and related patterns. The modules cover topics such as:

- GATs (Generic Associated Types)
- Trait bounds and generics
- Higher‑Ranked Trait Bounds (HRTB)

The exercises are organized as Rust modules under `src/`. The entry point that wires them together is `src/lib.rs`.

## Important note about commented modules
In `src/lib.rs` all the `mod` declarations are commented out on purpose. 
They currently include compilation errors by design. To work on them, you should uncomment the corresponding `mod` line, then fix the code until the project compiles.

You can enable modules one by one, depending on which exercise you want to tackle.

## Getting started
1. Ensure you have a recent Rust toolchain installed (via rustup).
2. Build the project to see the current state:
   - `cargo build` (or `cargo check`)
3. Pick an exercise module and enable it by uncommenting the corresponding line in `src/lib.rs`. For example, change `// mod hrtb;` to `mod hrtb;`.
4. Run `cargo check` or `cargo build` to see compiler errors for that module.
5. Open the file under `src/` that matches the module name (e.g., `src/hrtb.rs`) and follow the comments/instructions inside to solve the exercise.
6. Repeat the cycle (edit → build) until the code compiles. Then move on to the next module.

Tips:
- If you wish to focus on a single exercise at a time, keep other exercise modules commented out.
- Use `cargo test` if test cases are provided for a given module.

## Repository layout
- `src/lib.rs` — toggles which exercise modules are compiled.
- `src/gats_ab.rs`, `src/gats_c.rs`, `src/hrtb.rs`, `src/tbg.rs` — individual exercise modules.

Have fun exploring Rust’s type system and learning through compiler‑guided feedback!