# Assembler Documentation

SIC/XE assembly language implementation for the SIC-XE Assembler project.

## Overview

The assembler implements a two-pass SIC/XE assembler that converts assembly language code into machine code. It supports standard SIC/XE instructions, literals, symbols, and program blocks.

## Architecture

The assembler consists of two main passes:

### Pass 1
- Processes the source code line by line
- Builds the symbol table with label addresses
- Identifies literals and builds the literal table
- Tracks location counters for each program block
- Handles START, END, and assembler directives

### Pass 2
- Uses the symbol table from Pass 1
- Generates object code for each instruction
- Handles different instruction formats (Format 1-4)
- Resolves symbol references
- Produces the final object program

## Supported Instructions

### Format 1 Instructions (1 byte)
- `FIX` - Convert floating to fixed
- `FLOAT` - Convert fixed to floating
- `HIO` - Halt I/O
- `SIO` - Start I/O
- `TIO` - Test I/O
- `NORM` - Normalize

### Format 2 Instructions (2 bytes)
- `ADDR` - Add register to register
- `CLEAR` - Clear register
- `COMPR` - Compare registers
- `DIVR` - Divide register by register
- `MULR` - Multiply register by register
- `RMO` - Register to register move
- `SHIFTR` - Shift right register
- `SHIFTL` - Shift left register
- `SUBR` - Subtract register from register
- `SVC` - Supervisor call
- `TIXR` - Test and increment register

### Format 3 Instructions (3 bytes)
- `ADD` - Add memory to register
- `ADDF` - Add floating memory to register
- `AND` - AND memory with register
- `COMP` - Compare memory with register
- `COMPF` - Compare floating memory with register
- `DIV` - Divide register by memory
- `J` - Jump to address
- `JEQ` - Jump if equal
- `JGT` - Jump if greater than
- `JLT` - Jump if less than
- `JSUB` - Jump to subroutine
- `LDA` - Load accumulator
- `LDB` - Load base register
- `LDCH` - Load character
- `LDF` - Load floating register
- `LDL` - Load link register
- `LDS` - Load stack pointer
- `LDT` - Load temporary register
- `LDX` - Load index register
- `LPS` - Load processor status
- `MUL` - Multiply register by memory
- `MULF` - Multiply floating register by memory
- `OR` - OR memory with register
- `RD` - Read device
- `RSUB` - Return from subroutine
- `SSK` - Set storage key
- `STA` - Store accumulator
- `STB` - Store base register
- `STCH` - Store character
- `STF` - Store floating register
- `STI` - Store instruction
- `STL` - Store link register
- `STS` - Store stack pointer
- `STSW` - Store switch
- `STT` - Store temporary register
- `STX` - Store index register
- `SUB` - Subtract memory from register
- `SUBF` - Subtract floating memory from register
- `TD` - Test device
- `TIX` - Test and increment index
- `WD` - Write device

### Format 4 Instructions (4 bytes)
- `CADD` - Extended add
- `CSUB` - Extended subtract
- `CLOAD` - Extended load
- `CSTORE` - Extended store
- `CJUMP` - Extended jump

## Registers

| Register | Value | Description |
|----------|-------|-------------|
| A | 0 | Accumulator |
| X | 1 | Index register |
| L | 2 | Link register |
| B | 3 | Base register |
| S | 4 | Stack pointer |
| T | 5 | Temporary register |
| F | 6 | Floating point accumulator |
| PC | 8 | Program counter |
| SW | 9 | Status word |

## Assembler Directives

### START
Specifies the program name and starting address.

```
START 1000
```

### END
Specifies the end of the program and the first executable instruction.

```
END FIRST
```

### BYTE
Defines byte constants.

```
BYTE C'EOF'    ; Character string
BYTE X'4F'     ; Hexadecimal value
```

### WORD
Defines word constants (3 bytes).

```
WORD 12345
```

### RESB
Reserves bytes.

```
RESB 100
```

### RESW
Reserves words (3 bytes each).

```
RESW 50
```

### BASE
Sets the base register for addressing.

```
BASE TABLE
```

## Literals

Literals are defined with an asterisk (*) and are automatically placed in the literal pool.

```
LDA =X'45'     ; Literal
```

## Program Blocks

The assembler supports multiple program blocks with separate location counters.

```
USE CDATA      ; Switch to CDATA block
USE DEFAULTB   ; Switch to default block
```

## Implementation Details

### Pass 1 Processing

1. **Initialization**: Set location counter to starting address
2. **Line Processing**: Parse each line into label, instruction, and operand
3. **Symbol Table**: Add labels with their addresses
4. **Literal Table**: Track literals and their values
5. **Location Counter**: Increment based on instruction format
6. **Block Management**: Track location counters for each block

### Pass 2 Processing

1. **Symbol Table Lookup**: Resolve symbol references
2. **Opcode Lookup**: Find opcode for each instruction
3. **Format Determination**: Determine instruction format (1-4)
4. **Address Calculation**: Calculate target addresses
5. **Base Register**: Use base addressing when needed
6. **Object Code Generation**: Generate machine code bytes
7. **Literal Pool**: Generate object code for literals

### Addressing Modes

- **Direct**: Target address is specified directly
- **Indexed**: Target address is offset by index register
- **Base-relative**: Target address is offset by base register
- **Immediate**: Operand is specified directly
- **Indirect**: Address of operand is specified

## Error Handling

The assembler detects and reports various errors:

- **Undefined symbols**: Reference to undefined label
- **Duplicate symbols**: Same label defined multiple times
- **Invalid instructions**: Unknown or malformed instructions
- **Address overflow**: Address exceeds memory limits
- **Format errors**: Incorrect operand format

## Output Format

### Symbol Table
```
Symbol    Address
--------  -------
LOOP      1003
FIRST     1000
```

### Literal Table
```
Literal    Address
--------    -------
=X'45'     2000
=C'EOF'    2003
```

### Object Program
```
H^PROGAM^001000^000054
T^001000^1E^1C0003000369...
E^001000
```

## Conversion Utilities

### string_to_hex
Converts a string to hexadecimal representation.

```rust
string_to_hex("ABC") // "414243"
```

### hex_string_to_hex
Extracts hexadecimal value from string notation.

```rust
hex_string_to_hex("X'4F'") // "4F"
```

### integer_to_hex
Converts integer to hexadecimal with specified byte length.

```rust
integer_to_hex(255, 2) // "00FF"
```

### get_register_value
Returns numeric value for register name.

```rust
get_register_value("A") // 0
get_register_value("X") // 1
```

## Example Program

```
COPY    START  1000
FIRST   STL    RETADR
        LDB    #LENGTH
        BASE   LENGTH
CLOOP  LDA    =X'F1'
        STA    BUFFER
        LDA    LENGTH
        COMP   #0
        JLT    ENDLOOP
        LDA    BUFFER
        COMP   =C'EOF'
        JEQ    ENDLOOP
        LDA    BUFFER
        STA    RECORD
        TD     =X'05'
        JEQ    CLOOP
        J      CLOOP
ENDLOOP LDA    =X'05'
        STA    BUFFER
        LDA    LENGTH
        COMP   #0
        JLT    EXIT
        LDA    BUFFER
        STA    RECORD
        TD     =X'05'
        JEQ    ENDLOOP
        J      ENDLOOP
EXIT   RSUB
LENGTH  RESB   1
BUFFER  RESB   4096
RECORD  RESB   4096
RETADR  RESW   1
        END    FIRST
```

## File Structure

```
src/
├── pass1.rs          # Pass 1 implementation
├── pass2.rs          # Pass 2 implementation
└── conversions.rs    # Conversion utilities
```

## Usage

The assembler is called via the backend API:

```rust
// In handlers/assembly.rs
use crate::pass1::Pass1;
use crate::pass2::Pass2;

let mut pass1 = Pass1::new();
pass1.process_code(&code)?;

let mut pass2 = Pass2::new();
pass2.process(pass1)?;
```