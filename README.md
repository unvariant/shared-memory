# Shared Memory
Testing shared memory on linux and macos with Rust and C and Assembly.

## Atomics
### x86_64
 - memory accesses that are aligned to their access size are guaranteed to be atomic
 - atomics implemented using lock cmpxchg and lock xchg for unaligned accesses
### arm32 (arm)
 - ldrexd/strexd faults if the base register is not 8 byte aligned
 - ldrex/strex have no alignment restrictions
 - atomics implemented using load/store exclusives
### arm64 (aarch64)
 - ldxr/stxr
 - atomics implemented using load/store exclusives

## x86_64-linux
 - `cargo run` to setup shared memory
 - compile and run consume.c to write to shared memory
 - for some strange reason nasm does not export the global symbols, so the atomics are written for the `as` assembler instead of `nasm`

## arm-linux
 - `cargo run` to setup shared memory
 - compile and run consume.c to write to shared memory

## aarch64-linux
 - WIP

## aarch64-macos
 - `cargo run` to setup shared memory
 - compile and run consume.c to write to shared memory
    - strangely running consume.c and attempting to access shared memory fails with Permission Denied, but works fine when run using `sudo`
 - symbols exported from assembly must have `_` prefixed to them, a quirk of macos
 - when building a static library use the `ar` provided by the system

## Creating a static library
1. compile the assembler files using `as`
2. generate a static library using `ar`
3. library file names *must* always start with lib

example assembler file tmp.s
```
as tmp.s -o tmp.o
ar -rcs libtmp.a tmp.o
```