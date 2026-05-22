use crate::vm::Opcode;

pub fn encode(sequence: &[Opcode]) -> String {
    let mut hex_string = String::new();
    for op in sequence {
        let (byte1, byte2) = match op {
            Opcode::Nop => (0x00, 0x00),
            Opcode::Inc(r) => (0x01, *r),
            Opcode::Dec(r) => (0x02, *r),
            Opcode::Add(d, s) => (0x03, (d << 4) | s),
            Opcode::Sub(d, s) => (0x04, (d << 4) | s),
            Opcode::Mul(d, s) => (0x05, (d << 4) | s),
            Opcode::Div(d, s) => (0x06, (d << 4) | s),
            Opcode::Mov(d, s) => (0x07, (d << 4) | s),
            Opcode::Ldi(d, imm) => (0x08, ((d & 0x0F) << 4) | ((*imm as u8) & 0x0F)),
            Opcode::Jmp(offset) => (0x09, (*offset as u8) & 0xFF),
            Opcode::Jz(r, offset) => (0x0A, ((r & 0x0F) << 4) | ((*offset as u8) & 0x0F)),
            Opcode::IoOut(r) => (0x0B, *r),
            Opcode::Ld(d, a) => (0x0C, (d << 4) | a),
            Opcode::St(a, s) => (0x0D, (a << 4) | s),
            Opcode::Hlt => (0x0E, 0x00),
        };
        hex_string.push_str(&format!("{:02X}{:02X}", byte1, byte2));
    }
    hex_string
}

pub fn decode(hex_str: &str) -> Vec<Opcode> {
    let mut sequence = Vec::new();
    let chars: Vec<char> = hex_str.chars().collect();
    
    for i in (0..chars.len()).step_by(4) {
        if i + 3 >= chars.len() {
            break; // Truncated
        }
        let b1_str: String = chars[i..i+2].iter().collect();
        let b2_str: String = chars[i+2..i+4].iter().collect();
        
        let byte1 = u8::from_str_radix(&b1_str, 16).unwrap_or(0);
        let byte2 = u8::from_str_radix(&b2_str, 16).unwrap_or(0);

        let op = match byte1 {
            0x00 => Opcode::Nop,
            0x01 => Opcode::Inc(byte2),
            0x02 => Opcode::Dec(byte2),
            0x03 => Opcode::Add(byte2 >> 4, byte2 & 0x0F),
            0x04 => Opcode::Sub(byte2 >> 4, byte2 & 0x0F),
            0x05 => Opcode::Mul(byte2 >> 4, byte2 & 0x0F),
            0x06 => Opcode::Div(byte2 >> 4, byte2 & 0x0F),
            0x07 => Opcode::Mov(byte2 >> 4, byte2 & 0x0F),
            0x08 => Opcode::Ldi(byte2 >> 4, ((byte2 & 0x0F) as i8) as i32), // Simple signed extension
            0x09 => Opcode::Jmp(byte2 as i8 as i32),
            0x0A => Opcode::Jz(byte2 >> 4, ((byte2 & 0x0F) as i8) as i32),
            0x0B => Opcode::IoOut(byte2),
            0x0C => Opcode::Ld(byte2 >> 4, byte2 & 0x0F),
            0x0D => Opcode::St(byte2 >> 4, byte2 & 0x0F),
            _ => Opcode::Hlt,
        };
        sequence.push(op);
    }
    sequence
}
