# kandiru

A streaming, arena-backed JSON query tool written in Arandu, inspired by `jq`.

```sh
arandu run . -- <file.json> [<query>] [--compact] [-r]
arandu test .
arandu build .
```

## Examples

```sh
# Pretty print formatted JSON
arandu run . -- package.json .

# Query a top-level key
arandu run . -- package.json .dependencies

# Nested access and indexing
arandu run . -- data.json .users.[0].name

# Extract string without quotes
arandu run . -- config.json .version -r

# Compact output
arandu run . -- large.json .items --compact
```

## Compiler & Runtime Workloads Exercised

### Tagged Unions / Enums & Match Dispatch
- Evaluates `JsonKind` and `QueryStep` dispatch in Cranelift.
- Tests whether recursive descent parsing branches convert into jump tables, binary search trees of branches, or linear comparison cascades.

### Arena Allocation vs. Heap Fragmentation
- All AST nodes (`JsonNode`) and object entries (`ObjectEntry`) are allocated into contiguous `vec.Vec` structures within `JsonArena`.
- Compares memory footprint and cache locality against individual `malloc`/`free` nodes.

### Zero-Copy String Slicing
- Keys and string values borrow byte spans directly from the mmap'd or loaded source buffer without allocating individual heap strings.
- Stresses slice borrowing across lifetime boundaries and verifying zero heap allocation in string reading paths.
