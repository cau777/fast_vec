#!/bin/bash

set -e

echo "Running benchmarks..."
echo ""
echo "Running Vector2 benchmarks..."
cargo bench --bench vec2_bench
echo ""
echo "Running Vector3 benchmarks..."
cargo bench --bench vec3_bench

./scripts/compare_benchmarks.sh
