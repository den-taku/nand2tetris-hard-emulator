// Mult.asm
// Caluculate R0 * R1 -> R2
// R0 >= 0, R1 >= 0, R0 * Rq < 32768

@R2
M=0

(LOOP)
    @R0
    D=M
    @R2
    M=M+D // R2 += R0

    @R1
    MD=M-1 // R1 -= 1

    @LOOP
    D;JGT
(END)
    @END
    0;JMP