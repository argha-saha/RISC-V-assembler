addi t0, x0, 4
j targ
snez a0, t0
addi s0, x0, 8

targ:
addi t1, t0, 12
and t1, t1, t0