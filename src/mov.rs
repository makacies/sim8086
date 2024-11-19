use std::{collections::HashMap, env, fs};

pub fn disassemble() -> Vec<String> {
    const OPCODE_MASK: u8 = 0b11111100;
    const D_MASK: u8 = 0b00000010;
    const W_MASK: u8 = 0b00000001;
    const MOD_MASK: u8 = 0b11000000;
    const REG_MASK: u8 = 0b00111000;
    const R_M_MASK: u8 = 0b00000111;
    
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

    // let path = env::current_dir().unwrap().join("src/listings/listing_0037_single_register_mov");
    let path = env::current_dir().unwrap().join("src/listings/listing_0038_many_register_mov");

    let stream = fs::read(path).unwrap();

    let mut i = 0;
    let len = stream.len();
    let mut instructions: Vec<String> = Vec::new();

    while i < len {
        let opcode = stream[i] & OPCODE_MASK;
        let opcode = *opcode_map.get(&opcode).unwrap();
        
        let d = stream[i] & D_MASK;
        let w = stream[i] & W_MASK;

        let reg_map = if w == 0 {
            &reg_w0_map
        } else {
            &reg_w1_map
        };

        let reg = stream[i + 1] & REG_MASK;
        let reg_w0 = reg_map.get(&reg).unwrap();

        let r_m = stream[i + 1] & R_M_MASK;
        let r_m = r_m << 3;
        let r_m_w0 = reg_map.get(&r_m).unwrap();

        let (destination, source) = if d == 0 {
            (r_m_w0, reg_w0)
        } else {
            (reg_w0, r_m_w0)
        };

        instructions.push(format!("{opcode} {destination}, {source}"));
        i = i + 2;
    };

    dbg!(&instructions);
    instructions
    // print!("original: {:08b}\n", byte);
}
