// Registers
#[repr(usize)]
#[allow(dead_code)]
// Registers
pub enum R {
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
// Operations
pub enum OP {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* return from interrupt */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump (return from subroutine) */
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

// Trap routines
#[repr(u16)]
pub enum TRAP {
    GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    OUT = 0x21,   /* output a character */
    PUTS = 0x22,  /* output a word string */
    IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PUTSP = 0x24, /* output a byte string */
    HALT = 0x25,  /* halt the program */
}

impl TryFrom<u16> for TRAP {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(TRAP::GETC),
            0x21 => Ok(TRAP::OUT),
            0x22 => Ok(TRAP::PUTS),
            0x23 => Ok(TRAP::IN),
            0x24 => Ok(TRAP::PUTSP),
            0x25 => Ok(TRAP::HALT),
            _ => Err(value),
        }
    }
}

// Condition flags
#[repr(u16)]
pub enum FL {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

// Memory-mapped registers
#[repr(u16)]
pub enum MR {
    KBSR = 0xFE00, /* keyboard status */
    KBDR = 0xFE02, /* keyboard data */
}
