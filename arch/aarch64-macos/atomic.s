    .global _atomic_load64
    .global _atomic_store64

    .code 64
    .section .text


    .func _atomic_load64

_atomic_load64:
    mov     x9,      x0
1:  ldxr    x0,      [x9]
    stxr    w10,     x0,      [x9]
    cbnz    w10,     1b
    ret

    .endfunc


    .func _atomic_store64

_atomic_store64:
    mov     x9,      x0
1:  ldxr    x0,      [x9]
    stxr    w10,     x1,      [x9]
    cbnz    w10,     1b
    ret

    .endfunc
