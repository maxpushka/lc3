fn main() {
    // @{Load Arguments}
    // @{Setup}

    let memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
    let mut reg: [u16; R::COUNT as usize] = [0; R::COUNT as usize];

    // since exactly one condition flag should be set at any given time, set the Z flag
    reg[R::COND as usize] = FL::ZRO as u16;

    // set the PC to starting position
    // 0x3000 is the default
    const PC_START: u16 = 0x3000;
    reg[R::PC as usize] = PC_START;

    loop {
        let instr = mem_read(reg[R::PC as usize]);
        reg[R::PC as usize] += 1;

        let op = instr >> 12;
        match OP::try_from(op).expect("unknown opcode") {
            OP::BR => todo!(),
            OP::ADD => do_add(instr, &mut reg),
            OP::LD => todo!(),
            OP::ST => todo!(),
            OP::JSR => todo!(),
            OP::AND => todo!(),
            OP::LDR => todo!(),
            OP::STR => todo!(),
            OP::RTI => todo!(),
            OP::NOT => todo!(),
            OP::LDI => do_ldi(instr, &mut reg),
            OP::STI => todo!(),
            OP::JMP => todo!(),
            OP::RES => todo!(),
            OP::LEA => todo!(),
            OP::TRAP => todo!(),
        }
    }

    // @{Shutdown}
}

const MEMORY_MAX: usize = 1 << 16;

// Registers
#[repr(usize)]
enum R {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, /* program counter */
    COND,
    COUNT,
}

// Opcodes
#[repr(u16)]
enum OP {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP,   /* execute trap */
}

impl TryFrom<u16> for OP {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OP::BR),
            1 => Ok(OP::ADD),
            2 => Ok(OP::LD),
            3 => Ok(OP::ST),
            4 => Ok(OP::JSR),
            5 => Ok(OP::AND),
            6 => Ok(OP::LDR),
            7 => Ok(OP::STR),
            8 => Ok(OP::RTI),
            9 => Ok(OP::NOT),
            10 => Ok(OP::LDI),
            11 => Ok(OP::STI),
            12 => Ok(OP::JMP),
            13 => Ok(OP::RES),
            14 => Ok(OP::LEA),
            15 => Ok(OP::TRAP),
            _ => Err(value),
        }
    }
}

// Condition flags
#[repr(u16)]
enum FL {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

fn mem_read(address: u16) -> u16 {
    0 // TODO
}

fn update_flags(reg: &mut [u16; R::COUNT as usize], r: u16) {
    if reg[r as usize] == 0 {
        reg[R::COND as usize] = FL::ZRO as u16;
    } else if (reg[r as usize] >> 15) != 0 {
        // a 1 in the left-most bit indicates negative
        reg[R::COND as usize] = FL::NEG as u16;
    } else {
        reg[R::COND as usize] = FL::POS as u16;
    }
}

fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

// # Assembler formats
//
// ADD DR, SR1, SR2,
// ADD DR, SR1, imm5
//
// # Examples
//
// ADD R2, R3, R4 ; R2 <- R3 + R4
// ADD R2, R3, #7 ; R2 <- R3 + 7
//
// # Encodings
//
// Register mode:
// 0001 xxx xxx 0 00 xxx
// ADD  DR  SR1      SR2
//
// Immediate mode:
// 0001 xxx xxx 1 xxxxx
// ADD  DR  SR1   imm5
fn do_add(instr: u16, mut reg: &mut [u16; R::COUNT as usize]) {
    let r0: u16 = (instr >> 9) & 0x7; // destination register (DR)
    let r1: u16 = (instr >> 6) & 0x7; // first operand (SR1)
    let imm_flag: u16 = (instr >> 5) & 0x1; // whether we are in immediate mode

    if imm_flag != 0 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] + imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }

    update_flags(&mut reg, r0);
}

// # Assembler formats
//
// LDI DR, LABEL
//
// # Examples
//
// LDI R4, ONEMORE ; R4 <- mem[mem[ONEMORE]]
//
// # Encodings
//
// 1010 xxx xxxxxxxxx
//      DR  PCoffset9
fn do_ldi(instr: u16, mut reg: &mut [u16; R::COUNT as usize]) {
    let r0: u16 = (instr >> 4) & 0x7; // destination register (DR)
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    // add pc_offset to the current PC, look at that memory location to get the final address
    reg[r0 as usize] = mem_read(mem_read(reg[R::PC as usize]) + pc_offset);
    update_flags(&mut reg, r0);
}
