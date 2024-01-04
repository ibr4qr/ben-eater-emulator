# Ben Eater Machine
Simple Toolchain around the Simple As Possible Ben Eater computer architecure, including a VM ( emulator ), assembler, compiler, decompiler.

This Project is for learning purpose, in fact we are trying to build a toolchain around the famous SAP (Simple-As-Possible) ben eater computer architecture.


[Ben Eater Simple as Possible Architecture](https://en.wikipedia.org/wiki/Simple-As-Possible_computer)


## Overview


The Design is pretty simple, and will include 3 modules. 

- Ben Eater Computer Emulator
- Assembler
- Compiler
- Decompiler


### Ben Eater Computer Emualtor
This module will produce a ( VM ) Virtual Machine, which is actually an emualtor of the ben Eater computer, so will execute the instruction set of the physical device.

### Assembler
This module will produce a system software that translates a higher level human readable code into machine code.


### Compiler
This module will produce a system software that will compile a higher level human readable code, more structured, into assembly language.

### Decompiler
This module will produce a system software that will reverse an assembly file into an higher level language. 




## Ben Eater Computer Emualtor Design

### Registers
Let's start with registers, the architecture features two registers, named, Register A and Register B.

So in this simple design we have two registers.
- Register A
- Register B