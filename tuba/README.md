# tuba

A high-throughput LZ4-like block compression and decompression utility written in Arandu.

```sh
arandu run . -- compress <input> <output>
arandu run . -- decompress <input> <output>
arandu test .
arandu build .
```

## Binary Container Format

`tuba` writes a compact binary header:
- `TUBA` (4 bytes magic)
- Original uncompressed file size (8 bytes, little-endian `u64`)
- Consecutive compressed sequences:
  - Token byte: `(literal_length_nibble << 4) | match_length_nibble`
  - Optional variable-length literal size overflow bytes (`255` run)
  - Literal byte stream
  - 2-byte little-endian match offset (`u16`)
  - Optional variable-length match size overflow bytes (`255` run)

## Compiler & Assembly Workload

### Register Spilling Under Hash-Table Pressure
- The sliding-window hash table (`CompressState`) contains 4,096 entries indexed by a 4-byte rolling hash.
- Checks whether Cranelift preserves hot table registers or spills heavily to stack frames during sequence emission.

### Bitwise Logic & Shift Optimization
- Heavily uses byte masking, shifts (`<<`, `>>`), and bitwise operations (`|`, `^`, `&`).
- Exercises compiler peephole optimizations to combine nibble packing and unpacking into single instructions.

### Buffer Overlap & RLE Decompression Loops
- LZ decompression allows back-references that overlap with current output positions (Run Length Encoding behavior).
- Validates the codegen of overlapping copy loops without intermediate buffer copies.
