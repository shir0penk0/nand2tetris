// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(LOOP)

  // Get pressed key
  @KBD
  D=M
  
  // if (Memory[KBD]) > 0 then goto FILL else goto CLEAR
  @FILL
  D;JGT
  @CLEAR
  0;JMP

(FILL)
  // Initialize
  @i
  M=0
  
(FILL_LOOP)
  // Fill Memory[SCREEN + i]
  @i
  D=M
  @SCREEN
  A=D+A
  // DEC(-1) is equal to BIN(1111111111111111)
  M=-1
  
  // Increment i
  @i
  MD=M+1

  // if (i - 8192 < 0) then goto FILL_LOOP
  // 512 * 256 / 16 = 8192
  @8192
  D=D-A
  @FILL_LOOP
  D;JLT

  @LOOP
  0;JMP

// Similar to FILL
(CLEAR)
  @i
  M=0
  
(CLEAR_LOOP)
  @i
  D=M
  @SCREEN
  A=D+A
  M=0
  
  @i
  MD=M+1

  @8192
  D=D-A

  @CLEAR_LOOP
  D;JLT

  @LOOP
  0;JMP
