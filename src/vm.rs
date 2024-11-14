mod maps;

use maps::*;

fn main() {
    println!("Virtual Machine start ...");

    let insts = vec![
        OP_MOV, 3, R0,
        OP_ADD, 2, R0,
        OP_PRT, R0,
        OP_EXIT,
    ];

    let mut ip = 0;
    loop {
        let mut ins = &insts[ip];
        match ins {
            &OP_MOV => {
                mov(&insts[ip+1], &insts[ip+2]);
                ip += 2;
            },
            &OP_ADD => {
                add(&insts[ip+1], &insts[ip+2]);
                ip += 2;
            },
            &OP_SUB => {
                sub(&insts[ip+1], &insts[ip+2]);
                ip += 2;
            },
            &OP_PRT => {
                println!("PRT");
                ip += 1;
            },
            &OP_EXIT => {
                println!("EXIT");
                break;
            },
            _ => println!("Unknown Operand[{}]", ins)
        }
        ip += 1;
    }
}

fn mov(value:&i32, register:&i32) {
    println!("MOV");
}

fn add(value:&i32, register:&i32) {
    println!("ADD");
}

fn sub(value:&i32, register:&i32) {
    println!("SUB");
}
