# SVM - Simple Virtual Machine

## OpCodes

| OpCode | Mnemonic | Description                                                                  | Usage                    |
| ------ | -------- | ---------------------------------------------------------------------------- | ------------------------ |
| 0x00   | NOP      | No operation                                                                 | NOP                      |
| 0x01   | HALT     | Halt program                                                                 | HALT                     |
| 0x02   | COPY     | Duplicate top of stack                                                       | COPY <type>              |
| 0x03   | PUSH     | Push value to stack                                                          | PUSH <value>             |
| 0x04   | PUSHALL  | Push all values to stack                                                     | PUSHALL <size> <...data> |
| 0x05   | POP      | Pop value from stack and move to register                                    | POP <type>               |
| 0x06   | ADD      | Add top two values on stack                                                  | ADD <type>               |
| 0x07   | SUB      | Subtract top two values on stack                                             | SUB <type>               |
| 0x08   | MUL      | Multiply top two values on stack                                             | MUL <type>               |
| 0x09   | DIV      | Divide top two values on stack                                               | DIV <type>               |
| 0x0A   | MOD      | Modulus top two values on stack                                              | MOD <type>               |
| 0x0B   | POW      | Power top two values on stack                                                | POW <type>               |
| 0x0C   | INC      | Increment top value on stack                                                 | INC <type>               |
| 0x0D   | DEC      | Decrement top value on stack                                                 | DEC <type>               |
| 0x0E   | WRITE    | Write value to memory                                                        | WRITE                    |
| 0x0F   | JUMP     | Jump to address in top of stack                                              | JUMP                     |
| 0x10   | CMP      | Jump to address in top of stack if penultimate value is true                 | CMP                      |
| 0x11   | LT       | Pop top two values and push true if first is less than second                | LT <type>                |
| 0x12   | LTE      | Pop top two values and push true if first is less than or equal to second    | LTE <type>               |
| 0x13   | EQ       | Pop top two values and push true if first is equal to second                 | EQ <type>                |
| 0x14   | NEQ      | Pop top two values and push true if first is not equal to second             | NEQ <type>               |
| 0x15   | GT       | Pop top two values and push true if first is greater than second             | GT <type>                |
| 0x16   | GTE      | Pop top two values and push true if first is greater than or equal to second | GTE <type>               |
| 0x17   | MSP      | Move value to stack pointer                                                  | MSP <value>              |
| 0x18   | SP       | Push stack pointer to stack                                                  | SP                       |
| 0x19   | PC       | Push program counter to stack                                                | PC                       |
| 0x1A   | AND      | Pop top two values and push true if both are true                            | AND                      |
| 0x1B   | OR       | Pop top two values and push true if either are true                          | OR                       |
| 0x1C   | XOR      | Pop top two values and push true if only one is true                         | XOR                      |
| 0x1D   | NOT      | Pop top value and push true if false                                         | NOT                      |
| 0x1E   | SHL      | Pop top two values and push first shifted left by second                     | SHL <type>               |
| 0x1F   | SHR      | Pop top two values and push first shifted right by second                    | SHR <type>               |
| 0x20   | MOV      | Move value to register                                                       | MOV <register> <value>   |
| 0x21   | REG      | Push register value to stack                                                 | REG <register> <type>    |
