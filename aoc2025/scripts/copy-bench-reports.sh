#!/bin/bash
# Copy Criterion benchmark reports to a visible location
mkdir -p benches/criterion
cp -r target/criterion/report/* benches/criterion/ 2>/dev/null || true
echo "Benchmark reports copied to benches/criterion/"
