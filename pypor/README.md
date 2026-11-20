# Pypor

An Arandu line counter used as an end-to-end compiler workload. It recursively
scans a directory, classifies code, comment, and blank lines, and prints totals.

```sh
arandu run . -- /path/to/project
arandu run . -- /path/to/project --files
arandu test .
arandu build .
```

`--files` prints full paths so the selected file set can be audited rather than
inferred from basenames.

## Compile-time scanner classes

This cut requires the compiler **and stdlib from the current 0.1.9 development
checkout**, not the published 0.1.8 toolchain. From this directory, for example:

```sh
../Arandu-Lang/target/debug/arandu_cli check . --stdlib-path ../Arandu-Lang/stdlib
../Arandu-Lang/target/debug/arandu_cli test . --stdlib-path ../Arandu-Lang/stdlib --jobs 2
../Arandu-Lang/target/debug/arandu_cli build . --stdlib-path ../Arandu-Lang/stdlib --release
```

The scanner uses `std.core.ascii.ByteSet` for horizontal whitespace and ASCII
identifier bytes. `comptime ascii.byteSet("...")` constructs each 32-byte set
while compiling; runtime membership only reads the frozen words. LF is not
horizontal whitespace, and form-feed/vertical-tab retain the previous behavior.

The exhaustive smoke test checks both classes and `ascii.byteTable` against the
original predicates for all 256 byte values. The scanner dispatches once per
file into shared compile-time feature profiles (0, 12, 20, 51, 52), not one
hand-copied implementation per language. Profile 128 preserves arbitrary public
`Language` configurations dynamically. On Linux (2026-10-03), all 13 package
tests passed, including the custom nested-comment fallback.
The pre-change and updated release binaries produced identical output for the
local Linux kernel corpus: 65,479 files, 28,649,834 code lines, 4,648,685 comment
lines, 4,696,647 blank lines, and 37,995,166 total lines.

Three warm-cache runs per binary, alternating order, used `--workers 2` and
`taskset -c 0,1`. Local medians were 6.61 s before and 4.81 s after; user CPU
time was 12.13 s and 8.52 s. Peak RSS medians were approximately 187 MiB and
202 MiB, respectively: this experiment does **not** establish a memory reduction.
These are development-host observations, not a portable performance guarantee
or a new comparison against Tokei.

## Eight-core specialization experiment

Five warm-cache runs per binary, alternating order, used `--workers 8` and
`taskset -c 0-7` (eight distinct physical cores on this host). External timings
include traversal, reading, counting and cleanup; they are not the scanner's
internal timer. Against the previous ByteSet-only executable, feature profiles
reduced median wall time from 1.575 s to 1.465 s and user CPU time from 8.988 s
to 7.752 s. Median peak RSS was 223.4 MiB before and 225.4 MiB after. Counts
remained exactly equal on all 65,479 selected kernel files.

A separate five-run comparison of specialized ByteSets and 256-byte tables
measured 1.494 s / 227.0 MiB versus 1.524 s / 273.5 MiB. Tables were therefore
not adopted in the production scanner. Cranelift currently materializes these
Copy aggregates with heap backing: the assembly contains malloc calls, so
compile-time construction is **not** yet allocation-free residual storage.
The table API remains exercised by exhaustive tests, not presented as a speed
or memory improvement. Background desktop activity remains a source of noise.

## Residual-storage compiler validation

On Linux (2026-10-04), a clean temporary copy of the same sources passed all
13 package tests with the development compiler. Its new release executable and
the previous `pypor-305e9188d3acbf41` executable produced identical kernel totals
on all runs: 65,479 files and 37,995,166 total lines.

Five alternating warm-cache runs used eight workers and the same eight physical
cores (`taskset -c 0-7`). External process timing (including traversal, reading,
counting and cleanup) had median wall time 1.31 s before versus 1.28 s after;
user CPU was 6.90 s versus 6.84 s. This small timing difference is not evidence
of a large speedup. Median peak RSS fell from 233,648 KiB to 202,528 KiB, about
13%. Timing and peak RSS were measured with the shell's process resource report,
not the scanner's internal timer.

Separate malloc interposition counted 4,780,729 versus 3,826,624 calls (about
20% fewer). Requested bytes stayed near 6.2 GB over the entire execution; this
is cumulative allocation traffic, not simultaneous resident memory or a leak
measurement. Process-wide counts include traversal, runtime and teardown.

The earlier blanket heap-backing observation above describes the old compiler.
The updated compiler uses independent frames or invocation-owned heap scratch
for admitted aggregates, and bounded immutable initializer data. Foreign
retention, borrowed carriers and aggregate loop-phi lifetimes still fall back
conservatively. These results do not justify replacing ByteSets with 256-byte
tables without a new experiment, nor establish semantic parity with Tokei.

These measurements compare Pypor revisions, not a new Tokei run. The earlier
matched-file audit still found classification differences; preserving Pypor's
counts does not establish semantic parity with Tokei.
