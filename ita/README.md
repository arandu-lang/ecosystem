# ita

SHA-256 file hasher written in Arandu. Produces output identical to `sha256sum`.

```sh
arandu run . -- <file> [<file> ...]
arandu test .
arandu build .
```

Output format: `<sha256hex>  <filename>` (two spaces, same as POSIX `sha256sum`).

## Development commands

```sh
../Arandu-Lang/target/debug/arandu_cli check . --stdlib-path ../Arandu-Lang/stdlib
../Arandu-Lang/target/debug/arandu_cli test . --stdlib-path ../Arandu-Lang/stdlib
../Arandu-Lang/target/debug/arandu_cli build . --stdlib-path ../Arandu-Lang/stdlib --release
```

## Compiler behaviors exercised

### Inlining of SHA-256 helper functions
`sigma0`, `sigma1`, `gamma0`, `gamma1`, `ch`, `maj`, and `rotr32` are called inside
the 64-round compression loop. Each is a trivial expression. If the compiler inlines
them correctly, the assembly for `sha256Compress` will contain no `call` instructions,
only `ror`, `xor`, `and`, `add`, etc. Any `call`/`ret` pair inside the round body is
a missed inline.

Verify: `objdump -d ita | grep -A 300 '<sha256Compress>'` — should show no `callq`.

### Struct-by-value vs. stack copy of the 8-word hash state
`sha256Compress` takes `state: mut ref Sha256State`. The 8 working variables `a..h` are
local `u32` values that should be allocated in 8 distinct GP registers (`eax`, `ebx`,
etc.). If Cranelift lacks a good register allocator, it may spill them to the stack on
every round iteration, producing unnecessary `mov [rbp-N], eax` sequences.

Verify: look for `movl %eax, -N(%rbp)` inside the round body — ideally absent.

### Loop unrolling / fully-unrolled 64-round compression
`sha256Compress` is written as 64 explicit round statements rather than a `while` loop.
This forces the compiler to either:
- Emit 64 blocks of straight-line code (ideal — no loop overhead), or
- Fail to optimize the repeated pattern and produce redundant loads.

Verify: count unique `ror`/`xor` instruction addresses in the function — should be ~64×5.

### Zero heap allocation
The entire SHA-256 computation uses no heap. `sha256Init`, `sha256Update`, and
`sha256Finalize` all operate on a `Sha256State` living on the caller's stack.
The only heap allocation is the final `String` for hex output.

Verify: `valgrind --tool=massif ./ita /dev/null` should show peak heap ≈ the hex String size (~64 bytes).

## Test vectors

| Input | Expected SHA-256 |
|-------|-----------------|
| `""` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `"abc"` | `ba7816bf8f01cfea414140de5dae2ec73b00361bbef0469352390a96f6bc347a` |
| `"abcdbcdecde..."` (448-bit message) | `248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1` |
