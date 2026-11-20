# sarapo

A lightweight static file HTTP/1.1 server written in Arandu.

```sh
arandu run . -- /var/www/html --port 8080
arandu test .
arandu build .
```

## Features

- Non-allocating zero-copy request parsing directly from socket stream buffers.
- Supports `GET` and `HEAD` verbs.
- Automatic MIME type inference for common static assets (HTML, JS, CSS, JSON, SVG, PNG, WASM).
- Path traversal protection.

## Compiler & Assembly Workload

### Finite State Machine (FSM) Header Parsing
- Inspects byte-by-byte loops with delimiter checks (`\r\n\r\n`).
- Evaluates whether the compiler can hoist invariant checks and compile clean jump tables for method matching (`GET`, `HEAD`, `POST`).

### Escape Analysis on Request Handlers
- Connection buffers (`[4096]u8`) are stack-allocated per loop iteration.
- Tests whether Cranelift allows buffers to remain on the stack without promoting them to heap allocations.

### Syscall & Network Abstraction Overhead
- Exercises Arandu's `std.net` socket runtime layer (`TcpListener`, `TcpStream`).
- Measures per-connection overhead, socket read/write calls, and cleanup efficiency.
