# aratu

An embedded append-only Key-Value store with in-memory index written in Arandu.

```sh
arandu run . -- /path/to/data.db put user:1001 "Alice Smith"
arandu run . -- /path/to/data.db get user:1001
arandu run . -- /path/to/data.db del user:1001
arandu run . -- /path/to/data.db list
arandu test .
arandu build .
```

## Binary Record Format

Every record appended to the log follows:
- `Type`: 1 byte (`0x01` = Put, `0x02` = Delete/Tombstone)
- `Key Length`: 4 bytes, little-endian `u32`
- `Value Length`: 4 bytes, little-endian `u32`
- `Key Data`: raw bytes
- `Value Data`: raw bytes
- `CRC32 Checksum`: 4 bytes, IEEE polynomial over type, lengths, and data

## Compiler & Assembly Workload

### Struct Layout & Alignment
- Evaluates memory layout and padding of `Record` and `IndexEntry`.
- Checks for cache-line friendliness and unnecessary field offsets.

### Little-Endian Serialization
- Translates `u32` to/from byte slices using bit-shift logic.
- Verifies whether Cranelift synthesizes unaligned loads/stores into single instructions (`mov` / `bswap` where applicable).

### Persistent State Reconstruction
- The database startup path performs sequential record reads and verified CRC calculations over the entire file.
- Measures heap allocation behavior and buffer re-use during startup.
