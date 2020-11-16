// Fill.asm

(LOOP)

    @KBD
    D=M
    @ON
    D;JNE
    @OFF
    0;JMP

    (ON)
        @R0 // value
        M=-1
        @DISPLAY
        0;JMP
    (OFF)
        @R0 // value
        M=0
        @DISPLAY
        0;JMP

(DISPLAY)
    @SCREEN
    D=A
    @R1 // address
    M=D
    @R2
    M=0 // 0~8191

    (DLOOP)
    @R0
    D=M
    @R1
    M=D

    @R1
    M=M+1
    @R2
    MD=M+1
    @8192
    D=D-A
    @DLOOP
    D;JLT

@LOOP
0;JMP