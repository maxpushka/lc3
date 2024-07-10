use crate::{
    defs::{R, TRAP},
    state::State,
};
use std::io::{Read, Write};

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
pub fn do_add(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7; // destination register (DR)
    let r1: u16 = (instr >> 6) & 0x7; // first operand (SR1)
    let imm_flag: u16 = (instr >> 5) & 1; // whether we are in immediate mode

    if imm_flag != 0 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        state.reg[r0] = state.reg[r1].wrapping_add(imm5);
    } else {
        let r2: u16 = instr & 0x7;
        state.reg[r0] = state.reg[r1].wrapping_add(state.reg[r2]);
    }

    state.reg.update_flags(r0);
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
pub fn do_ldi(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 4) & 0x7; // destination register (DR)
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    // add pc_offset to the current PC, look at that memory location to get the final address
    let pc = state.mem.read(state.reg[R::PC]);
    state.reg[r0] = state.mem.read(pc.wrapping_add(pc_offset));
    state.reg.update_flags(r0);
}

// # Assembler formats
//
// AND DR, SR1, SR2,
// AND DR, SR1, imm5
//
// # Examples
//
// AND R2, R3, R4 ; R2 <- R3 AND R4
// AND R2, R3, #7 ; R2 <- R3 AND 7
//
// # Encodings
//
// Register mode:
// 0001 xxx xxx 0 00 xxx
// AND  DR  SR1      SR2
//
// Immediate mode:
// 0001 xxx xxx 1 xxxxx
// AND  DR  SR1   imm5
pub fn do_and(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7; // destination register (DR)
    let r1: u16 = (instr >> 6) & 0x7; // first operand (SR1)
    let imm_flag: u16 = (instr >> 5) & 1; // whether we are in immediate mode

    if imm_flag != 0 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        state.reg[r0] = state.reg[r1] & imm5;
    } else {
        let r2: u16 = instr & 0x7;
        state.reg[r0] = state.reg[r1] & state.reg[r2];
    }

    state.reg.update_flags(r0);
}

// # Assembler formats
//
// NOT DR, SR
//
// # Examples
//
// NOT R4, R2 ; R4 <- NOT(R2)
//
// # Encodings
//
// 1001 xxx xxx 1 11111
// NOT  DR  SR1
pub fn do_not(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7; // destination register (DR)
    let r1: u16 = (instr >> 6) & 0x7; // first operand (SR1)

    state.reg[r0] = !state.reg[r1];

    state.reg.update_flags(r0);
}

// # Assembler formats
//
// BRn LABEL
// BRz LABEL
// BRp LABEL
// BRzp LABEL
// BPnp LABEL
// BRnz LABEL
// BRnzp LABEL ; same as `BR LABEL`
// BR LABEL    ; same as `BRnzp LABEL`
//
// # Examples
//
// BRzp LOOP ; Branch to LOOP if the last result was zero or positive
// BR   NEXT ; Unconditionally branch to NEXT
//
// # Encodings
//
// 0000 x x x xxxxxxxxx
// BR   n z p PCoffset9
pub fn do_br(instr: u16, state: &mut State) {
    let cond_flag: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    if cond_flag & state.reg[R::COND] != 0 {
        state.reg[R::PC] = state.reg[R::PC].wrapping_add(pc_offset);
    }
}

// # Assembler formats
//
// JMP BaseR
// RET
//
// # Examples
//
// JMP R2 ; PC <- R2
// RET    ; PC <- R7
//
// # Encodings
//
// JMP: 1100 000 xxx   000000
//               BaseR
//
// RET: 1100 000 111   000000
pub fn do_jmp(instr: u16, state: &mut State) {
    let r1 = (instr >> 6) & 0x7;
    state.reg[R::PC] = state.reg[r1];
}

// # Assembler formats
//
// JSR  LABEL
// JSRR BaseR
//
// # Examples
//
// JSR  QUEUE ; Put the address of the instruction following JSR into R7;
//            ; Jump to QUEUE
// JSRR R3    ; Put the address following JSRR into R7;
//            ; Jump to the address contained in R3
//
// # Encodings
//
// JSR:  0100 1 xxxxxxxxxxx
//              PCoffset11
//
// JSRR: 0100 0 00 xxx   000000
//                 BaseR
pub fn do_jsr(instr: u16, state: &mut State) {
    let long_flag: u16 = (instr >> 11) & 1;
    state.reg[R::R7] = state.reg[R::PC];

    if long_flag != 0 {
        /* JSR */
        let long_pc_offset = sign_extend(instr & 0x7FF, 11); // PCoffset11
        state.reg[R::PC] = state.reg[R::PC].wrapping_add(long_pc_offset);
    } else {
        /* JSRR */
        let r1: u16 = (instr >> 6) & 0x7;
        state.reg[R::PC] = state.reg[r1];
    }
}

// # Assembler formats
//
// LD DR, LABEL
//
// # Examples
//
// LD R4, VALUE ; R4 <- mem[VALUE]
//
// # Encodings
//
// 0010 xxx xxxxxxxxx
//      DR  PCoffset9
pub fn do_ld(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    state.reg[r0] = state.mem.read(state.reg[R::PC].wrapping_add(pc_offset));
    state.reg.update_flags(r0);
}

// # Assembler formats
//
// LDR DR, BaseR, offset6
//
// # Examples
//
// LDR R4, R2, #-5 ; R4 <- mem[R2 - 5]
//
// # Encodings
//
// 0110 xxx xxx   xxxxxx
//      DR  BaseR offset6
pub fn do_ldr(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7; // DR
    let r1: u16 = (instr >> 6) & 0x7; // BaseR
    let offset = sign_extend(instr & 0x3F, 6); // offset6

    state.reg[r0] = state.mem.read(state.reg[r1].wrapping_add(offset));
    state.reg.update_flags(r0);
}

// # Assembler formats
//
// LEA DR, LABEL
//
// # Examples
//
// LEA R4, TARGET ; R4 <- address of TARGET
//
// # Encodings
//
// 1110 xxx xxxxxxxxx
//      DR  PCoffset9
pub fn do_lea(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    state.reg[r0] = state.reg[R::PC].wrapping_add(pc_offset);
    state.reg.update_flags(r0);
}

// # Assembler formats
//
// ST SR, LABEL
//
// # Examples
//
// ST R4, HERE ; mem[HERE] <- R4
//
// # Encodings
//
// 0011 xxx xxxxxxxxx
//      SR  PCoffset9
pub fn do_st(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    let address = (R::PC as u16).wrapping_add(pc_offset);
    let value = state.reg[r0];
    state.mem.write(address, value);
}

// # Assembler formats
//
// STI SR, LABEL
//
// # Examples
//
// STI R4, NOT_HERE ; mem[mem[NOT_HERE]] <- R4
//
// # Encodings
//
// 1011 xxx xxxxxxxxx
//      SR  PCoffset9
pub fn do_sti(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9); // PCoffset9

    let address = state.mem.read((R::PC as u16).wrapping_add(pc_offset));
    let value = state.reg[r0];
    state.mem.write(address, value);
}

// # Assembler formats
//
// STR SR, BaseR, offset6
//
// # Examples
//
// STR R4, R2, #5 ; mem[R2 + 5] <- R4
//
// # Encodings
//
// 0111 xxx xxx   xxxxxx
//      SR  BaseR offset6
pub fn do_str(instr: u16, state: &mut State) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6); // offset6

    let address = state.reg[r1].wrapping_add(offset);
    let value = state.reg[r0];
    state.mem.write(address, value);
}

// # Assembler formats
//
// TRAP trapvector8
//
// # Examples
//
// TRAP 0x23 ; Directs the operating system to execute the IN system call.
//           ; The starting address of this system call is contained in
//           ; memory location x0023.
//
// # Encodings
//
// 1111 0000 xxxxxxxx
//           trapvect8
pub fn do_trap(instr: u16, state: &mut State) {
    state.reg[R::R7] = state.reg[R::PC];

    let trap_vector = TRAP::try_from(instr & 0xFF).expect("unknown trap routine");
    match trap_vector {
        TRAP::GETC => {
            state.reg[R::R0] = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();
            state.reg.update_flags(R::R0 as u16);
        }
        TRAP::OUT => {
            let c = state.reg[R::R0] as u8 as char;
            print!("{}", c);
            std::io::stdout().flush().unwrap();
        }
        TRAP::PUTS => {
            let mut address = state.reg[R::R0];
            loop {
                let c = state.mem.read(address) as u8;
                if c == 0 {
                    break;
                }
                print!("{}", c as char);
                address = address.wrapping_add(1);
            }
        }
        TRAP::IN => {
            print!("Enter a character: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if let Some(c) = input.chars().next() {
                print!("{}", c);
                std::io::stdout().flush().unwrap();
                state.reg[R::R0] = c as u16;
                state.reg.update_flags(R::R0 as u16);
            }
        }
        TRAP::PUTSP => {
            /* one char per byte (two bytes per word)
            here we need to swap back to
            big endian format */
            let mut c = state.reg[R::R0];
            while state.mem.read(c) != 0 {
                let char1 = (state.mem.read(c) & 0xFF) as u8 as char;
                print!("{}", char1);

                let char2 = (state.mem.read(c) >> 8) as u8 as char;
                print!("{}", char2);

                c = c.wrapping_add(1);
            }

            std::io::stdout().flush().unwrap();
        }
        TRAP::HALT => {
            println!("HALT");
            std::io::stdout().flush().unwrap();
            state.running = false;
        }
    };
}
