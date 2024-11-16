mod maps;

use std::collections::HashMap;

use maps::*;

fn main() {
    println!("Virtual Machine start ...");

    let insts = vec![
        OP_MOV, 3, R0,
        OP_ADD, 2, R0,
        OP_PRT, R0,
        OP_EXIT,
    ];

    let mut rmap = HashMap::new();
    rmap.insert(R0, 0);

    let mut ip = 0;
    loop {
        let ins = &insts[ip];
        match ins {
            &OP_MOV => {
                println!("MOV");
                rmap.insert(insts[ip+2], insts[ip+1]);
                ip += 2;
            },
            &OP_ADD => {
                println!("ADD");
                let sum = rmap.get(&insts[ip+2]).unwrap() + insts[ip+1];
                rmap.insert(insts[ip+2], sum);
                ip += 2;
            },
            &OP_SUB => {
                println!("SUB");
                let sub = rmap.get(&insts[ip+2]).unwrap() - insts[ip+1];
                rmap.insert(insts[ip+2], sub);
                ip += 2;
            },
            &OP_PRT => {
                println!("PRT {:?}", rmap.get(&insts[ip+1]).unwrap());
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
