use std::{collections::HashMap, env, fs};

fn main() {
    const OPCODE_MASK: u8 = 0b11111100;
    const D_MASK: u8 = 0b00000010;
    const W_MASK: u8 = 0b00000001;
    const MOD_MASK: u8 = 0b11000000;
    const REG_W0_MASK: u8 = 0b00111000;
    const REG_W1_MASK: u8 = 0b00000111;
    
    let opcode_map: HashMap<u8, &str> = HashMap::from([
        (0b10001000, "MOV")
    ]);
    let reg_w0_map: HashMap<u8, &str> = HashMap::from([
        (0b00000000, "AL"),
        (0b00001000, "CL"),
        (0b00010000, "DL"),
        (0b00011000, "BL"),
        (0b00100000, "AH"),
        (0b00101000, "CH"),
        (0b00110000, "DH"),
        (0b00111000, "BH"),
    ]);
    let reg_w1_map: HashMap<u8, &str> = HashMap::from([
        (0b00000000, "AX"),
        (0b00001000, "CX"),
        (0b00010000, "DX"),
        (0b00011000, "BX"),
        (0b00100000, "SP"),
        (0b00101000, "BP"),
        (0b00110000, "SI"),
        (0b00111000, "DI"),
    ]);
    let mod_map: HashMap<u8, HashMap<u8, &str>> = HashMap::from([
        (0b11000000, reg_w1_map),
    ]);

    let path = env::current_dir().unwrap().join("src/tests/listing_0037_single_register_mov");
    let stream = fs::read(path).unwrap();

    let mut i = 0;
    let len = stream.len();
    let mut instructions = String::default();

    while i < len {
        let opcode = stream[i] & OPCODE_MASK;
        let opcode = *opcode_map.get(&opcode).unwrap();

        let reg_w0 = stream[i + 1] & REG_W0_MASK;
        let reg_w0 = *reg_w0_map.get(&reg_w0).unwrap();

        let reg_mod = stream[i + 1] & MOD_MASK;
        let reg_w1_mod_map: &HashMap<u8, &str> = mod_map.get(&reg_mod).unwrap();


        let reg_w1 = stream[i + 1] & REG_W0_MASK;
        let reg_w1 = *reg_w1_mod_map.get(&reg_w1).unwrap();

        instructions = format!("{opcode} {reg_w0}, {reg_w1}");
        i = i + 2;
    };

    dbg!(instructions);
    // TODO: 
    // - incorrect answer => check D bit
    // - update to lowercase
    // - write output to a file
    // - add tests
    // - add to a repository

    // print!("original: {:08b}\n", byte);
}
