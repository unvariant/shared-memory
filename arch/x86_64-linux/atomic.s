    .global atomic_load64
    .global atomic_store64

    .intel_syntax noprefix
    .section .text


    .func atomic_load64

atomic_load64:
    mov rax, 0
    lock cmpxchg qword ptr [rdi], rax
    ret

    .endfunc


    .func atomic_store64

atomic_store64:
    lock xchg qword ptr [rdi], rsi
    mov rax, rsi
    ret

    .endfunc
