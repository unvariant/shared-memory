#include <inttypes.h>

uint64_t atomic_load64 (void * addr) {
    asm __volatile__ (
    "   mov     r3,      r0"
    "1: ldrexd  r0,      r1,      [r3]\n"
    "   strexd  r12,     r0,      r1,      [r3]\n"
    "   teq     r12,     #0\n"
    "   bne     1b\n"
    )
}

uint64_t atomic_store64 (void * addr, uint64_t val) {
    void * tmp;
    asm __volatile__ (
    "   mov     r3,      r2\n"
    "   mov     r4,      r0\n"
    "   mov     r2,      r1\n"
    "1: ldrexd  r0,      r1,      [r4]\n"
    "   strexd  r12,     r0,      r1,      [r4]\n"
    "   teq     r12,     #0\n"
    "   bne     1b\n"
    :
    :
    : "r4"
    );
}

uint32_t atomic_load32 (void * addr) {
    asm __volatile__ (
    "1: ldrex   r1,      [r0]\n"
    "   strex   r12,     r1,      [r0]\n"
    "   teq     r12,     #0\n"
    "   bne     1b\n"
    )
}

uint32_t atomic_store32 (void * addr, uint32_t val) {
    asm __volatile__ (
    "   mov     r2,     r0\n"
    "1: ldrex   r0,     [r2]\n"
    "   strex   r12,    r0,      [r2]\n"
    "   teq     r12,    #0\n"
    "   bne     1b\n"
    );
}