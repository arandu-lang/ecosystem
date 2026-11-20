# kururu

A CPU-based software ray tracer with Blinn-Phong shading and reflections written in Arandu, outputting binary PPM images.

```sh
arandu run . -- render.ppm --width 400 --height 300
arandu test .
arandu build .
```

## Features

- Camera with configurable field of view and aspect ratio.
- Spheres with diffuse, specular (Blinn-Phong), and recursive mirror reflection components.
- Point lights with hard shadow casting rays.
- Exports to standard portable pixmap format (PPM P6 binary).

## Compiler & Assembly Workload

### Floating-Point Operations & Vector Inlining
- `Vec3` operations (`add`, `sub`, `dot`, `scale`, `normalize`) take `self: Vec3` by value.
- Evaluates whether Cranelift passes struct arguments directly through `xmm0`–`xmm2` registers rather than generating stack copies (`movss`/`movaps`).

### Hot Inner Loop Optimization
- `raySphereIntersect` executes millions of times per frame.
- Analyzes loop invariant code motion, common subexpression elimination (evaluating discriminant `b*b - 4*a*c`), and aggressive inlining.

### Flat Framebuffer Memory Throughput
- Writes linear RGB bytes into a single flat buffer.
- Measures cache write throughput and loop index increment overhead.
