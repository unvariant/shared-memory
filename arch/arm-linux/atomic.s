    .global atomic_load64
    .global atomic_store64

    .code 32
    .section .text


    .func atomic_load64

atomic_load64:
    mov     r3,      r0
1:  ldrexd  r0,      r1,      [r3]
    strexd  r12,     r0,      r1,      [r3]
    teq     r12,     #0
    bne     1b

    mov     pc,      lr

    .endfunc


    .func atomic_store64

atomic_store64:
    push    { r4 }

    mov     r3,      r2
    mov     r4,      r0
    mov     r2,      r1

1:  ldrexd  r0,      r1,      [r4]
    strexd  r12,     r2,      r3,      [r4]
    teq     r12,     #0
    bne     1b

    pop     { r4 }
    mov     pc,      lr

    .endfunc
