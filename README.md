# MicroCPU

Project of a simple 8-bit processor made in Logisim, using only basic gates and latches.

## Design

### General Principles

MicroCPU is a general-purpose processor with a set of instructions being able to execute simple programs loaded in ROM memory. As most of the CPUs ever made, it operates using a three-step flow with the following stages:
 - fetch stage - CPU loads a byte from memory using an address stored in the Program Counter Register (PC) and stores it into the Instruction Register (IR)
 - decode stage - loaded instruction is used to determine what operations must be performed
 - execute stage - processor activates specific components based on microcode to perform an operation, this might be loading/storing data from/to memory, doing a math operation in ALU, conditional jump to a different address etc.

While MicroCPU executes one stage per clock cycle to maintain design simplicity, modern processors with highly sophisticated pipelines are able to execute all stages concurrently - it means that while one instruction is executed, the next ones are fetched and decoded at the same time. This might lead to issues like how to handle branches or dependencies between instructions next to each other, so various techniques are used to mitigate them.

All components are controlled by control signals, with their inputs/outputs connected to the shared 8-bit bus. This means that while it makes it easy to move data between them, it's important to allow only one part of the CPU at a time to write to the bus, to avoid any collisions.

![CPU](/Docs/cpu.jpg)

### Registers

A register is the most basic form of storing data in the processor, being able to hold 1 byte in the case of MicroCPU. Internally it's built using a serie of D latches, activated using fall edge of the clock.

MicroCPU contains 9 registers, which includes:
 - 2 for general-purpose operations
 - 1 for storing ALU flags
 - 1 for storing memory addresses
 - 1 for storing loaded instructions
 - 4 for output

![Register](/Docs/register.jpg)

### Counters

A counter is a specialized form of register, with the ability to increment stored value - this can be achieved by a set of JK latches connected together.

MicroCPU uses 2 crucial counters:
 - Program Counter - stores an address of the next instruction to fetch
 - Step Counter - controls which microcode operation is currently executed

![Counter](/Docs/counter.jpg)

### Arithmetic Logic Unit

ALU contains all logic related to operations on integer numbers - both inputs are directly connected to general purpose registers A/B and the operation is performed every time one of the signal lines is active. In MicroCPU these are:
 - arithmetic: adding, subtracting, negating
 - bitwise: AND, OR, XOR, NOT, shift left/right

Additionally, 4 flags (Carry, Overflow, Sign, Zero) are used to provide additional details about the result and are usually stored into Flag Register by the CPU.

![Arithmetic Logic Unit](/Docs/alu.jpg)

### Memory

MicroCPU supports up to 256 bytes of memory, with address space shared between 2 components:
 - ROM (from 0x00 to 0x7f) - for non-volatile memory with program instructions
 - RAM (from 0x80 to 0xff) - for variables and other volatile program data

The last bit of the address indicates which chip should be used, with RAM addresses being offset so 0x80 is mapped to address 0x00, 0x81 to 0x01 etc.

### Microcode

Decoding a loaded instruction is done by reading microinstructions stored in 4 microcode banks, each being one byte of control word. Every bit in the control word represents a single control signal activating a specific part of the processor.

| Signal   | Control Word | Name                       | Description |
|----------|--------------|----------------------------|-------------|
| PCR      | 0x80000000   | Program Counter Read       | Reads value stored in the Program Counter Register and writes it to the shared bus |
| PCW      | 0x40000000   | Program Counter Write      | Reads value from the shared bus and writes it to the Program Counter Register |
| PCI      | 0x20000000   | Program Counter Increment  | Increments value stored in the Program Counter Register |
| MAW      | 0x10000000   | Memory Address Write       | Reads value from the shared bus and writes it into Memory Address Register |
| IRW      | 0x08000000   | Instruction Register Write | Reads value from the shared bus and writes it into Instruction Register |
| MMR      | 0x04000000   | Memory Read                | Reads value from memory using address stored in the Memory Address Register and writes it to the shared bus |
| MMW      | 0x02000000   | Memory Write               | Reads value from the shared bus and writes it to memory using address stored in the Memory Address Register |
| ARR      | 0x01000000   | A Register Read            | Reads value stored in A Register and writes it to the shared bus |
| ARW      | 0x00800000   | A Register Write           | Reads value from the shared bus and writes it into A Register |
| BRR      | 0x00400000   | B Register Read            | Reads value stored in B Register and writes it to the shared bus |
| BRW      | 0x00200000   | B Register Write           | Reads value from the shared bus and writes it into B Register |
| ADD      | 0x00100000   | Addition                   | Adds values stored in registers A/B and outputs the result to the shared bus |
| SUB      | 0x00080000   | Substraction               | Substracts value stored the in the register B from the value stored in the register A and outputs the result to the shared bus |
| NEG      | 0x00040000   | Negation                   | Negates the value stored the in the register A and outputs the result to the shared bus |
| AND      | 0x00020000   | Bitwise AND                | Performs bitwise AND operation with values stored in registers A/B and outputs the result to the shared bus |
| OR       | 0x00010000   | Bitwise OR                 | Performs bitwise OR operation with values stored in registers A/B and outputs the result to the shared bus |
| XOR      | 0x00008000   | Bitwise XOR                | Performs bitwise XOR operation with values stored in registers A/B and outputs the result to the shared bus |
| NOT      | 0x00004000   | Bitwise NOT                | Performs bitwise NOT operation with value stored the in the register A and outputs the result to the shared bus |
| SHL      | 0x00002000   | Shift Left                 | Performs a bitwise left shift operation with the value stored the in the register A and outputs the result to the shared bus |
| SHR      | 0x00001000   | Shift Right                | Performs a bitwise right shift operation with the value stored the in the register A and outputs the result to the shared bus |
| OAW      | 0x00000800   | Output A Write             | Reads value from the shared bus and writes it to the output register A |
| OBW      | 0x00000400   | Output B Write             | Reads value from the shared bus and writes it to the output register B |
| OCW      | 0x00000200   | Output C Write             | Reads value from the shared bus and writes it to the output register C |
| ODW      | 0x00000100   | Output D Write             | Reads value from the shared bus and writes it to the output register D |
| JEQ      | 0x00000080   | Jump If Equal              | Allows write to the Program Counter if Zero Flag = 1 |
| JNQ      | 0x00000040   | Jump If Not Equal          | Allows write to the Program Counter if Zero Flag = 0 |
| JGR      | 0x00000020   | Jump If Greater            | Allows write to the Program Counter if Zero Flag = 0 and Sign Flag = Overflow Flag |
| JGQ      | 0x00000010   | Jump If Greater Or Equal   | Allows write to the Program Counter if Sign Flag = Overflow Flag |
| JLE      | 0x00000008   | Jump If Less               | Allows write to the Program Counter if Sign Flag != Overflow Flag |
| JLQ      | 0x00000004   | Jump If Less Or Equal      | Allows write to the Program Counter if Zero Flag = 1 or Sign Flag != Overflow Flag |
| RSC      | 0x00000002   | Reset Step Counter         | Sets Step Counter to 0 |
| HLT      | 0x00000001   | Halt                       | Stops the main clock |

## Software

### Machine code

MicroCPU's machine code is composed of 32 instructions, each having a unique opcode and representing a set of operations encoded in microcode to run. A more detailed table can be found in [./Docs/Opcodes.xsls](./Opcodes.xlsx).

| Mnemonic | Opcode | Parameters | Description |
|----------|--------|------------|-------------|
| HLT      | 0x01   | -          | Stops execution of the program |
| JMP      | 0x02   | &addr      | Jumps to the immediate address unconditionally |
| JEQ      | 0x03   | &addr      | Jumps to the immediate address if Zero Flag = 1 |
| JNQ      | 0x04   | &addr      | Jumps to the immediate address if Zero Flag = 0 |
| JGR      | 0x05   | &addr      | Jumps to the immediate address if Zero Flag = 0 and Sign Flag = Overflow Flag |
| JGQ      | 0x06   | &addr      | Jumps to the immediate address if Sign Flag = Overflow Flag |
| JLE      | 0x07   | &addr      | Jumps to the immediate address if Sign Flag != Overflow Flag |
| JLQ      | 0x08   | &addr      | Jumps to the immediate address if Zero Flag = 1 or Sign Flag != Overflow Flag |
| LDA      | 0x09   | val        | Loads immediate value to the register A |
| LDA      | 0x0a   | &addr      | Moves value from memory at the immediate address to the register A |
| LDA      | 0x0b   | &&addr     | Moves value from memory at the dereferenced address to the register A |
| LDB      | 0x0c   | val        | Loads immediate value to the register B |
| LDB      | 0x0d   | &addr      | Moves value from memory at the immediate address to the register B |
| LDB      | 0x0e   | &&addr     | Moves value from memory at the dereferenced address to the register B |
| STA      | 0x0f   | &addr      | Moves value from the register A into memory at the immediate address |
| STA      | 0x10   | &&addr     | Moves value from the register A into memory at the dereferenced address |
| STB      | 0x11   | &addr      | Moves value from the register B into memory at the immediate address |
| STB      | 0x12   | &&addr     | Moves value from the register B into memory at the dereferenced address |
| ADD      | 0x13   | -          | Adds the values from registers A and B, then stores the result in the register A |
| SUB      | 0x14   | -          | Subtracts value in the register B from value in the register A, then stores the result in the register A |
| NEG      | 0x15   | -          | Negates the value in the register A, then stores the result in the register A |
| AND      | 0x16   | -          | Performs bitwise AND operation with values stored in registers A/B, then stores the result in the register A |
| OR       | 0x17   | -          | Performs bitwise OR operation with values stored in registers A/B, then stores the result in the register A |
| XOR      | 0x18   | -          | Performs bitwise XOR operation with values stored in registers A/B, then stores the result in the register A |
| NOT      | 0x19   | -          | Performs bitwise XOR operation with value stored in the register A, then stores the result in the register A |
| SHL      | 0x1a   | -          | Performs bitwise left shift operation with value stored in the register A, then stores the result in the register A |
| SHR      | 0x1b   | -          | Performs bitwise right shift operation with value stored in the register A, then stores the result in the register A |
| CMP      | 0x1c   | -          | Compares values stored in registers A/B and updates Flag Register |
| OUTA     | 0x1d   | -          | Moves value from the register A to the output register A |
| OUTB     | 0x1e   | -          | Moves value from the register A to the output register B |
| OUTC     | 0x1f   | -          | Moves value from the register A to the output register C |
| OUTD     | 0x20   | -          | Moves value from the register A to the output register D |

### Assembler

Writing a program by directly using opcodes to make a binary image is a tedious task, so a simple assembler was made. In its simplest form, it takes a list of mnemonics and translates them into corresponding bytes, understandable by the processor.

```
loop:
LDA 0x3E      -> 0x09 0x3E
OUTA          -> 0x1D
LDA 0x5B      -> 0x09 0x5B
OUTB          -> 0x1E
LDA 0x52      -> 0x09 0x52
OUTC          -> 0x1F
LDA 0x77      -> 0x09 0x77
OUTD          -> 0x20
LDA 0x00      -> 0x09 0x00
OUTA          -> 0x1D
LDA 0x00      -> 0x09 0x00
OUTB          -> 0x1E
LDA 0x00      -> 0x09 0x00
OUTC          -> 0x1F
LDA 0x00      -> 0x09 0x00
OUTD          -> 0x20
JMP &loop     -> 0x02 0x00
```

Additionally, assembler supports syntax that isn't directly translated into opcodes but allows to make variables and labels which later can be referenced by name in the source code.

```
; This is the comment ;

; This is a declaration of a variable with a size of 1 byte at address 0x80 ;
; Each next one will be one byte further, so 0x81, 0x82, etc. ;
VAR capybara

; Variable can be referenced by name using prefix &
LDA &capybara

; Value stored in the variable can also be treated as an address to another variable ;
; This is called a dereference and can be used with prefix && ;
LDA &&capybara

; This is a label which can be used in jump instructions ;
loop:
JMP &loop
```

A program can be compiled using CLI with two parameters pointing at the input and output file - the latter might be loaded in Logisim to a ROM component and run (with the cavetat that the result cannot exceed 128 bytes to fit there).

```
assembler.exe -in helo.src -out helo.bin
```

