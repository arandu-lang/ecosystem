# guara — Bare-Metal RISC-V (RV32I) Micro-Emulator

**guara** is a clean, deterministic RISC-V 32-bit Base Integer (`RV32I`) instruction set emulator written in pure Arandu (`edition = "2026"`).

## Compiler Workload & Purpose

`guara` is designed to exercise the Arandu compiler's **integer bitwise pipeline, sign-extension semantics, jump-table / dense `match` lowering, and bounds-checked memory accesses**:

- **Bitfield Extraction & Sign-Extension**: Decodes all six RV32I instruction formats (`R`, `I`, `S`, `B`, `U`, `J`) with arithmetic right-shift sign extension (`((inst as i32) >> 20)`) and scrambled immediate bit reconstruction (`B`-type and `J`-type).
- **Hardwired `x0` Invariant**: Enforces `regs[0] = 0` architectural immunity on every writeback without branching in the ALU paths.
- **PC-Relative Control Flow**: Evaluates `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, `BGEU`, `JAL`, and `JALR` (with LSB clearing `(rs1 + imm_i) & 0xFFFFFFFE`) relative to the instruction's own address (`inst_pc`).
- **Little-Endian Linear Bus**: Byte-addressable RAM supporting `LB`/`LBU`, `LH`/`LHU`, `LW`, `SB`, `SH`, `SW` with sign extension on narrow signed loads.

## Usage

```bash
arandu run guara -- fib
arandu run guara -- gcd
arandu run guara -- sum
arandu test guara
```
