// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
  
  // Memory[i] = 1
  @i
  M=1

  // Memory[target] = 1
  @target
  M=1

  // R2 = 0
  @R2
  M=0

  // R3 = R0
  @R0
  D=M
  @R3
  M=D

(LOOP)
  @i
  D=M
  @15
  D=D-A // D=i-15
  @END
  D;JGT // if (i - 15) > 100 then goto END

  // if (R1 & target) <= 0 then goto SKIP
  @R1
  D=M
  @target
  D=D&M
  @SKIP
  D;JLE

  // R2 = R2 + R3
  @R3
  D=M
  @R2
  M=D+M

(SKIP)
  // R3 = R3 + R3
  @R3
  D=M
  @R3
  M=D+M

  // target = target + target
  @target
  D=M
  @target
  M=D+M

  // i = i + 1
  @i
  M=M+1

  @LOOP
  0;JMP

(END)
  @END
  0;JMP

