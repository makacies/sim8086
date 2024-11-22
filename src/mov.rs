use std::collections::HashMap;

pub fn disassemble(input_machine_code: &Vec<u8>) -> String {
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

    let mut i = 0;
    let len = input_machine_code.len();
    let mut instructions: String = "bits 16".to_string();

    while i < len {
        let opcode = input_machine_code[i] & OPCODE_MASK;
        let opcode = *opcode_map.get(&opcode).unwrap();
        
        let d = input_machine_code[i] & D_MASK;
        let w = input_machine_code[i] & W_MASK;

        let reg_map = if w == 0 {
            &reg_w0_map
        } else {
            &reg_w1_map
        };

        let reg = input_machine_code[i + 1] & REG_MASK;
        let reg_w0 = reg_map.get(&reg).unwrap();

        let r_m = input_machine_code[i + 1] & R_M_MASK;
        let r_m = r_m << 3;
        let r_m_w0 = reg_map.get(&r_m).unwrap();

        let (destination, source) = if d == 0 {
            (r_m_w0, reg_w0)
        } else {
            (reg_w0, r_m_w0)
        };

        instructions = format!("{instructions}\n{opcode} {destination}, {source}");
        i = i + 2;
    };

    dbg!(&instructions);
    instructions
    // print!("original: {:08b}\n", byte);
}
