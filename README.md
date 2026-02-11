# fast_vec

High-performance 2D and 3D vector types using SIMD for maximum speed.

## Features

- **SIMD-accelerated**: Uses Rust's `portable_simd` for hardware-optimized vector operations
- **Zero-cost abstractions**: All operations are inlined for maximum performance
- **Copy types**: Vectors implement `Copy`, making them cheap to pass by value
- **Comprehensive API**: Supports all common vector operations
- **Well-tested**: Extensive test coverage for correctness

## Vector Types

### `Vector2`

2D vector using `f64x2` SIMD internally.

```rust
use fast_vec::Vector2;

let a = Vector2::new(1.0, 2.0);
let b = Vector2::new(3.0, 4.0);

let sum = a + b;
let dot = a.dot(b);
let normalized = a.normalize();
let distance = a.distance(b);
```

### `Vector3`

3D vector using `f64x4` SIMD internally.

```rust
use fast_vec::Vector3;

let a = Vector3::new(1.0, 2.0, 3.0);
let b = Vector3::new(4.0, 5.0, 6.0);

let sum = a + b;
let cross = a.cross(b);
let magnitude = a.magnitude();
let normalized = a.normalize();
```

## Operations

All vector types support:

- **Construction**: `new()`, `zeros()`, `ones()`
- **Accessors**: `x()`, `y()`, `z()` (Vector3 only)
- **Mutators**: `set_x()`, `set_y()`, `set_z()` (Vector3 only)
- **Arithmetic**: `+`, `-`, `*` (scalar), `/` (scalar), unary `-`
- **Compound assignment**: `+=`, `-=`, `*=`, `/=`
- **Vector operations**:
  - `dot()` - Dot product
  - `magnitude()` / `magnitude_squared()` - Vector length
  - `normalize()` - Normalize to unit vector
  - `distance()` / `distance_squared()` - Distance between vectors
  - `cross()` - Cross product (Vector2 returns f64, Vector3 returns Vector3)

## Requirements

- Rust nightly compiler (for `portable_simd` feature)
- x86-64 or ARM64 architecture with SIMD support

## Testing

Run tests:

```bash
cargo test
```

## Benchmarking

To run benchmarks comparing with `nalgebra`:

```bash
# Run all benchmarks
cargo bench --bench vec2_bench
cargo bench --bench vec3_bench

# Or run a quick dummy benchmark for testing
cargo bench --bench dummy
```

To generate a comparison table (requires `jq` and `bc`):

```bash
# Run all benchmarks
bash scripts/run_benchmarks.sh

# Generate comparison from existing benchmark results
bash scripts/compare_benchmarks.sh
```

This will create `BENCHMARK_RESULTS.md` with performance comparison tables for both Vector2 and Vector3.
