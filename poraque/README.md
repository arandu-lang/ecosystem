# poraque

A hex viewer and ELF64 binary inspector written in Arandu.

```sh
arandu run . -- hex /path/to/binary --length 256
arandu run . -- elf /path/to/binary --sections
arandu test .
arandu build .
```

## Features

- **Hex dump**: 16-byte aligned table with 8-digit hex offsets, spaced byte groupings, and ASCII printable sidebar.
- **ELF64 Inspector**: decodes ELF header flags, machine architectures (x86_64, AArch64, RISC-V), entry points, and section header tables (`.shstrtab` string resolution).

## Compiler & Assembly Workload

### Allocation-Free Output Formatting
- Converts integers to hex bytes without intermediate string allocations, using preallocated fixed-size line buffers.
- Verifies whether Cranelift keeps character conversion lookup tables in registers or `.rodata`.

### Zero-Copy Binary Deserialization
- Decodes multi-byte integers (`u16`, `u32`, `u64`) directly from slices without allocating object representations on the heap.
- Stresses endianness arithmetic and bit shift syntheses.

### String Table Lifetime Borrowing
- C-style null-terminated strings inside `.shstrtab` and `.strtab` sections are accessed via borrowed `str` slices directly over memory-mapped binary buffers.
