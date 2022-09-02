#include <inttypes.h>

/// https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#the-base-procedure-call-standard

uint64_t atomic_load64 (void * addr) {
    asm __volatile__ (
    "   mov     x9,      r0\n"
    "1: ldxr    x0,      [x9]\n"
    "   stxr    x10,     x0,      [x9]\n"
    "   cbnz    x10,     1b\n"
    );
}

uint64_t atomic_store64 (void * addr, uint64_t val) {
    asm __volatile__ (
    "   mov     x9,      r0\n"
    "1: ldxr    x0,      [x9]\n"
    "   stxr    x10,     x1,      [x9]\n"
    "   cbnz    x10,     1b\n"
    )
}