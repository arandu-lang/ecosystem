# jabuti — In-Memory SIMD Vector Similarity Search Engine

**jabuti** is an in-memory vector similarity engine (k-NN / Dot Product & Cosine Similarity) written in pure Arandu (`edition = "2026"`).

## Compiler Workload & Purpose

`jabuti` is designed as a realistic benchmark for the Arandu compiler's **128-bit SIMD intrinsics (`std.core.simd.F32x4`)**, slice bounds-check elimination, and floating-point register allocation:

- **4-Way Unrolled SIMD Accumulators**: `dotProductSimd` maintains four independent `F32x4` accumulators (`acc0`, `acc1`, `acc2`, `acc3`), processing 16 `f32` lanes per loop iteration to hide FMA/multiplication pipeline latency on x86-64 (SSE/AVX), AArch64 (NEON), and WASM (`simd128`).
- **Delayed Horizontal Reduction**: Horizontal lane reduction (`extractLane(0..3)`) happens **once** outside the hot loop, avoiding expensive `haddps`/`shufps` instructions inside the inner loop.
- **Unaligned Tail Handling**: Dimensions that are not multiples of 16 or 4 cleanly fall back to a 4-lane SIMD loop and a scalar remainder tail.
- **Contiguous Flat Arena**: `FlatIndex` stores all embeddings in a single contiguous `Vec<f32>` (`count * dim`) to maximize L1/L2 cache locality and hardware prefetching.

## Usage

```bash
arandu run jabuti -- demo
arandu run jabuti -- bench
arandu test jabuti
```
