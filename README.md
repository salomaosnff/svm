# SVM - Simple Virtual Machine

## CLI

Compila um arquivo .svm para .svm.bin

```bash
svmc file.svm
```

Executa um arquivo .svm.bin

```bash
svm file.svm.bin
```

## OpCodes

|Mnemonic  |Categoria |Uso                                       |Descrição                                                                                               |Pilha              |
|----------|----------|------------------------------------------|--------------------------------------------------------------------------------------------------------|-------------------|
|NOP       |Misc      |NOP                                       |Nenhuma Operação                                                                                        |                   |
|HALT      |Misc      |HALT                                      |Para a execução do programa                                                                             |                   |
|MOV       |Misc      |MOV [type] [reg:u8] [value:type]          |Move um valor do tipo <type> de até 8 bytes para um registrador                                         |                   |
|REG       |Misc      |MOV [type] [reg:u8]                       |Carrega um valor do tipo <type> de um registrador para a pilha                                          |[+type]            |
|PC        |Misc      |PC                                        |Carrega o valor do Program Counter para a pilha                                                         |[+usize]           |
|SP        |Stack     |SP                                        |Carrega o valor do Stack Pointer para a pilha                                                           |[+usize]           |
|MSP       |Stack     |MSP [offset:usize]                        |Move o stack pointer em <offset> bytes                                                                  |                   |
|PUSH      |Stack     |PUSH [type] [value:type]                  |Empilha um valor do tipo <type>                                                                         |[+type]            |
|PUSH_u8   |Stack     |PUSH_u8 [size:u8] [type] [...items:type]  |Empilha até 255 itens do tipo <type>                                                                    |[*type]            |
|PUSH_u16  |Stack     |PUSH_u16 [size:u16] [type] [...items:type]|Empilha até 2^16-1 itens do tipo <type>                                                                 |[*type]            |
|PUSH_u32  |Stack     |PUSH_u32 [size:u32] [type] [...items:type]|Empilha até 2^32-1 itens do tipo <type>                                                                 |[*type]            |
|PUSH_u64  |Stack     |PUSH_u64 [size:u64] [type] [...items:type]|Empilha até 2^64-1 itens do tipo <type>                                                                 |[*type]            |
|PUSH_BYTES|Stack     |PUSH_bytes [size:usize] [...items:u8]     |Empilha até <size> bytes                                                                                |[*u8]              |
|POP       |Stack     |POP [type] [reg:u8]                       |Desempilha um valor do tipo <type> e move para o registrador <reg>                                      |[-type]            |
|COPY      |Stack     |COPY [type] [reg:u8]                      |Copia um valor do tipo <type> da pilha para um registrador                                              |                   |
|INC       |Stack     |INC [type] [reg:u8]                       |Incrementa um valor do tipo <type> na pilha                                                             |[-type,+type]      |
|DEC       |Stack     |DEC [type] [reg:u8]                       |Decrementa um valor do tipo <type> na pilha                                                             |[-type,+type]      |
|ADD       |Arithmetic|ADD [type]                                |Soma dois valores do tipo <type> na pilha e empilha o resultado                                         |[-type,-type,+type]|
|SUB       |Arithmetic|SUB [type]                                |Subtrai dois valores do tipo <type> na pilha e empilha o resultado                                      |[-type,-type,+type]|
|MUL       |Arithmetic|MUL [type]                                |Multiplica dois valores do tipo <type> na pilha e empilha o resultado                                   |[-type,-type,+type]|
|DIV       |Arithmetic|DIV [type]                                |Divide dois valores do tipo <type> na pilha e empilha o resultado                                       |[-type,-type,+type]|
|MOD       |Arithmetic|MOD [type]                                |Divide dois valores do tipo <type> na pilha e empilha o resto da divisão                                |[-type,-type,+type]|
|NEG       |Arithmetic|NEG [type]                                |Inverte o sinal de um valor do tipo <type> na pilha e empilha o resultado                               |[-type,+type]      |
|POW       |Arithmetic|POW [type]                                |Eleva um valor do tipo <type> na pilha a um valor do tipo <type> na pilha e empilha o resultado         |[-type,-type,+type]|
|AND       |Bitwise   |AND [type]                                |Faz a operação AND lógico em dois valores do tipo <type> e empilha o resultado                          |[-type,-type,+bool]|
|OR        |Bitwise   |OR [type]                                 |Faz a operação OR lógico em dois valores do tipo <type> e empilha o resultado                           |[-type,-type,+bool]|
|NOT       |Bitwise   |OR [type]                                 |Faz a operação NOT lógico em um valor do tipo <type> na pilha e empilha o resultado                     |[-type,+type]      |
|SHL       |Bitwise   |SHL [type]                                |Faz a operação de shift left nos dois valores da pilha do tipo <type> e empilha o resultado             |[-type,-type,+type]|
|SHR       |Bitwise   |SHR [type]                                |Faz a operação de shift right nos dois valores da pilha do tipo <type> e empilha o resultado            |[-type,-type,+type]|
|EQ        |Comparison|EQ [type]                                 |Compara dois valores do tipo <type> e empilha um bool se eles são iguais                                |[-type,-type,+bool]|
|NEQ       |Comparison|NEQ [type]                                |Compara dois valores do tipo <type> e empilha um bool se eles são diferentes                            |[-type,-type,+bool]|
|GT        |Comparison|GT [type]                                 |Compara dois valores do tipo <type> e empilha um bool se o primeiro é maior que o segundo               |[-type,-type,+bool]|
|GTE       |Comparison|GTE [type]                                |Compara dois valores do tipo <type> e empilha um bool se o primeiro é maior ou igual que o segundo      |[-type,-type,+bool]|
|LT        |Comparison|LT [type]                                 |Compara dois valores do tipo <type> e empilha um bool se o primeiro é menor que o segundo               |[-type,-type,+bool]|
|LTE       |Comparison|LTE [type]                                |Compara dois valores do tipo <type> e empilha um bool se o primeiro é menor ou igual que o segundo      |[-type,-type,+bool]|
|JMP       |Control   |JMP [pc:usize]                            |Pula para o endereço especificado                                                                       |                   |
|JNZ       |Control   |JNZ [pc:usize]                            |Pula para o endereço especificado se o valor do topo da pilha for diferente de zero                     |[-bool]            |
|JZ        |Control   |JZ [pc:usize]                             |Pula para o endereço especificado se o valor do topo da pilha for igual a zero                          |[-bool]            |
|GOTO      |Control   |GOTO                                      |Pula para o endereço especificado no registrador %addr                                                  |                   |
|GNZ       |Control   |GNZ                                       |Pula para o endereço especificado no registrador %addr se o valor do topo da pilha for diferente de zero|[-bool]            |
|GZ        |Control   |GZ                                        |Pula para o endereço especificado no registrador %addr se o valor do topo da pilha for igual a zero     |[-bool]            |
|EXT       |Extension |EXT [fn:usize]                            |Chama uma função externa de um módulo                                                                   |                   |
|CALL      |Function  |CALL [address:usize]                      |Chama uma função                                                                                        |                   |
|RET       |Function  |RET                                       |Retorna de uma função                                                                                   |                   |
