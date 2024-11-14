# Description of our Assembler lang

There is a desc of architecutre of our VM representative in our Assembler lang.
We have **registers** and **operations** listed below.

Notes: let's make a very simple asm lang for our VM to start development.

Our **registers** and **operations** will be encoded in 32-bit or 4 bytes.


## Registers

We are reserving a range from `0x10000000` to `0x20000001` for all our registers.
We have 16 32-bit registers for begin.

* `R{0..F}` [0x10000000..0x1000000F]

## Operations

Reserving a range from `0x40000001` to `0x60000001`

* `MOV {i32}, R{0..F}`  [0x40000001] - move value into one of the registers
* `ADD {i32}, R{0..F}`  [0x40000002] - add value to register, R = R + V
* `SUB {i32}, R{0..F}`  [0x40000003] - sub value from register, R = R - V
* `PRT R{0..F}`         [0x40000101] - Print value of register in stdout
* `EXIT`                [0x5FFFFFFF] - Operation for exit from VM