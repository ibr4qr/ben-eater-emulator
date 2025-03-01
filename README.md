# Ben Eater Machine

Simple Toolchain around the Simple As Possible Ben Eater computer architecure, including a VM ( emulator ), assembler, compiler, decompiler.

This Project is for learning purposes, we are trying to build a toolchain around the famous SAP (Simple-As-Possible) ben eater computer architecture.

[Ben Eater Simple as Possible Architecture](https://en.wikipedia.org/wiki/Simple-As-Possible_computer)

- [Ben Eater Machine](#initial)
- [Overview](#overview)
  - [Ben Eater Computer Emualtor](#ben-eater-computer-emulator)
  - [Assembler](#assembler)
  - [Compiler](#compiler)
  - [Decompiler](#decompiler)
- [Ben Eater Computer Emualtor Design](#ben-eater-computer-emulator-design)
  - [Registers](#registers)
  - [RAM ( Random Access Memory )](#random-access-memory)
  - [Program Counter](#program-counter)
  - [Arithmetic logic unit](#arithmetic-logic-unit)
- [How everything works together](#how-everything-works-together)
- [Instruction Set](#instruction-set)
- [Modules](#modules)
  - [High Level Overview](#high-level-overview)
  - [Uml Package Diagram](#uml-package-diagram)
  - [Uml Component Diagram](#uml-component-diagram)
- [Compiling](#compiling)
- [Executing](#executing)

## Architecture of the Physical Machine

[Architecture of physical machine](https://en.wikipedia.org/wiki/Simple-As-Possible_computer#/media/File:Ben_Eater_SAP_High_Level_Overview.jpg)

## Overview

The Design is pretty simple, and will include 4 modules.

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

Let's start with registers, the architecture features three registers, named, Register A, Register B, Register IR

So in this simple design we have two general purposes registers.

- Register A
- Register B

In this architecture we have special registers as well:

- Program Counter PC ( Holds the address of next instruction to be executed )
- Memory Address Register MI ( holds the the address of main memory )
- Instruction Register IR ( holds the current instruction that's being executed)

And we have a special purpose register, specifically the IR register will hold the current instruction that's being executed.

### RAM ( Random Access Memory )

The RAM, stores the program currently the computer is executing and the data.
The computer uses 4-bit addresses which means it will only have 16 bytes of RAM, limiting the size and complexity of programs it can run. This is by far its biggest limitation.

### Program Counter

The program counter (PC) counts in binary to keep track of which instruction the computer is currently executing

### Arithmetic logic unit

The arithmetic logic unit (ALU) part of a CPU is usually capable of performing various arithmetic, bitwise, and comparison operations on binary numbers. In our simple breadboard CPU, the ALU is just able to add and subtract. It’s connected to the A and B registers and outputs either the sum of A+B or the difference of A-B.

## How everything works together

[The logic of the computer](https://www.bencode.net/posts/oidc/)

## Instruction Set

Instructions are 8 bit long, and include.

- NOP => 0x0
- LDA => 0x1 @ address[3:0]
- ADD => 0x2 @ address[3:0]
- SUB => 0x3 @ address[3:0]
- STA => 0x4 @ address[3:0]
- LDI => 0x5 @ value[3:0]
- JMP => 0x6 @ address[3:0]
- JC => 0x7 @ address[3:0]
- JNC => 0x8 @ address[3:0]
- OUT => 0xE
- HALT => 0xF @ 0b0000

## Modules

As said before we have 4 modules, the VM ( emulator ), the Assembler, the Compiler, and the Decompiler.

### High Level Overview

![High Level Modules](./docs/images/high-level-architecture.png?raw=true)

### Uml Package Diagram

![UML Package Diagram](./docs/images/uml-package-diagram.png?raw=true)

### Uml Component Diagram

![UML Component Diagram](./docs/images/uml-component-diagram.png?raw=true)

We will split the into 2 groups:

- Compiling ( Assembler, Compiler, Decompiler)
- Executing ( Emulator )

## Compiling

Compiling Module will have all submodules that enable us to write code in a higher level human readable manner.

## Executing

Executing Modules is simpler, will have one submodule for now, and will execute the code that is compliant with the instruction set.

Here we have a simple UML componente diagram
[UML component diagram draw.io version](https://drive.google.com/file/d/1cKRcOjizPqRWN8UWJAC8LEJyJIH5rOlo/view?usp=sharing)

![UML component diagram image version](./docs/images/uml-component-diagram.png?raw=true)

## Starting with Executing

To use the vm, go to the vm crate and use the utility below
to produce a binary file, which store a binary program that implements a counter from 10 to 1.

```
printf '\x51\x4F\x5A\x0\x3F\x73' > output.bin
```

## Assembly language examples:

#### basic LDI, STA, OUT

```
LDI 12
OUT
STA 14
LDI 3
OUT
LDA 14
OUT
```

### De counter from 15 to 0

```
LDI 15
STA 15
LDI 1
STA 14
LDA 15
SUB 14
OUT
JC 4
```

### from 0 to 254, step of 2

```
LDI 10
STA 14
LDI 2
STA 15
LDI 0
ADD 15
OUT
JMP 5
```

### From 0 to 10, step 2

```
LDI 0
STA 13
LDI 2
STA 15
LDI 10
STA 14
LDA 13
ADD 15
OUT
STA 13
LDA 14
SUB 13
JC 6
```

### Fibonacci series

```
LDI 1
STA 13
LDI 0
STA 14
LDA 13
STA 15
ADD 14
OUT
STA 13
LDA 15
STA 14
JMP 4
```

### Compiling from higher level language to Assembly Language

#### Lexer

#### Parser

#### Assembler Code Generator

##### Expression Grammar:

- program => declaration\* EOF;
- declaration => varDecl | statement;
- varDecl => "var" IDENTIFIER "=" expression ";";
- statement => exprStmt | printStmt;
- exprStmt => expression ";";
- printStmt => "print" expression ";"
- expression => assignment;
- assignment => (call "." )? IDENTIFIER "=" assignment | equality;
- equality => comparison ( ("!=" | "==") comparison)\*;
- comparison => term ( (">" | ">=" | "<" ) term)\*;
- term => primary ( ("-" | "+") primary )\*
- primary => NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER;

###### Examples:

```
print(10+20); var me = 100;
```

will be parsed producing this ast.

first declaration:

```
PrintDecl { argument: BinaryExpr { operator: Token { token_type: Plus, lexeme: "+" }, right: Literal { value: 20 }, left: Literal { value: 10 } } }
```

second declaration:

```
declaration: VarDecl { identifier: "me", initializer: Literal { value: 100 } }
```
