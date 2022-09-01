#include <inttypes.h>

uint64_t atomic_load64 (void * addr) {
    asm __volatile__ (
        ".intel_syntax noprefix\n"
        "mov rax, 0\n"
        "lock cmpxchg qword ptr [rdi], rax\n"
        ".att_syntax prefix\n"
    );
}

uint64_t atomic_store64 (void * addr, uint64_t val) {
    asm __volatile__ (
        ".intel_syntax noprefix\n"
        "mov rax, rsi\n"
        "lock xchg qword ptr [rdi], rax\n"
        ".att_syntax prefix\n"
    );
}