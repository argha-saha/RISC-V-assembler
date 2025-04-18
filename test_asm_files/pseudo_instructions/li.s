li a0, 100         # Just uses addi
li a1, 1000000     # Expands into lui and addi
beq x0, x0, taken
addi a2, a0, 10
li a3, 0x12345678  # Expands into lui and addi
taken:
addi t0, x0, 16
addi t1, t0, 16