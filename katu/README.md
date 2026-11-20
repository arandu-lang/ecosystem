# katu

Fast recursive byte-pattern searcher written in Arandu. Modeled after `grep`/`ripgrep` as a compiler workload.

```sh
arandu run . -- <pattern> <path>
arandu run . -- <pattern> <path> --count
arandu run . -- <pattern> <path> -l
arandu run . -- <pattern> <path> -i
arandu test .
arandu build .
```

## Flags

| Flag | Description |
|------|-------------|
| `--count` | Print only match count per file |
| `-l`, `--files-with-matches` | Print only filenames with at least one match |
| `-i`, `--ignore-case` | Case-insensitive ASCII search |

Output format (default): `filename:lineno:line content`

## Compiler behaviors exercised

### Bounds Check Elimination (BCE)
The inner loop in `searcher.findMatches` iterates `i < hlen` where `hlen = slice.len<u8>(haystack)`.
Every `haystack[i]` access is proven in-bounds by the loop condition — a correct BCE pass must
eliminate the redundant bound checks and emit a simple `movzbl (%rsi, %rdi)` rather than a guarded load.
Check the assembly with `objdump -d` and verify no `ud2` or branch-to-panic around the array index.

### Zero-copy slice borrowing
`lineSlice(haystack, line_start, line_end)` returns a `[]u8` fat pointer into the original
buffer without any `malloc`. The `Match` struct stores `line_start`/`line_end` offsets rather than
owned `String` copies. Verify with `valgrind --tool=massif` that the heap profile shows only the
results `Vec<Match>` allocation, with zero per-match heap entries.

### Register pressure — loop-invariant hoisting
`nlen` and the `ignore_case` bool are read inside the inner `bytesMatchAt` loop but never change.
The compiler must hoist them outside the loop (LICM). Inspect the assembly: `nlen` should appear
in a callee-saved register across iterations, not reloaded from the stack on every call.

### Loop structure exposed to the backend
The outer loop body has exactly two paths: a fast `b == '\n'` branch that increments line counter
and restarts, and the match-attempt path. This gives Cranelift/LLVM a clean diamond CFG to
optimize — look for a tail-duplication or loop-rotation in the disassembly.

## Development commands (current checkout)

```sh
../Arandu-Lang/target/debug/arandu_cli check . --stdlib-path ../Arandu-Lang/stdlib
../Arandu-Lang/target/debug/arandu_cli test . --stdlib-path ../Arandu-Lang/stdlib
../Arandu-Lang/target/debug/arandu_cli build . --stdlib-path ../Arandu-Lang/stdlib --release
```
