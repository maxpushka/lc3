use std::{
    fs::File,
    io::{self, Read},
};

use defs::{OP, R};
use state::State;
use terminal::InputBuffering;

mod defs;
mod instr;
mod state;
mod terminal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        /* show usage string */
        println!("lc3 [image-file1] ...");
        return;
    }

    let mut state = State::new();
    for image in args {
        if let Err(e) = read_image_file(&image, &mut state) {
            println!("failed to load image: {}", e);
        }
    }

    // Disable input buffering.
    // Restore buffering on drop.
    let _ = InputBuffering::disable();

    loop {
        let instr = state.mem.read(state.reg[R::PC]);
        state.reg[R::PC] += 1;

        let op = instr >> 12;
        match OP::try_from(op).expect("unknown opcode") {
            OP::BR => instr::do_br(instr, &mut state),
            OP::ADD => instr::do_add(instr, &mut state),
            OP::LD => instr::do_ld(instr, &mut state),
            OP::ST => instr::do_st(instr, &mut state),
            OP::JSR => instr::do_jsr(instr, &mut state),
            OP::AND => instr::do_and(instr, &mut state),
            OP::LDR => instr::do_ldr(instr, &mut state),
            OP::STR => instr::do_str(instr, &mut state),
            OP::RTI => return, // not simulated // TODO
            OP::NOT => instr::do_not(instr, &mut state),
            OP::LDI => instr::do_ldi(instr, &mut state),
            OP::STI => instr::do_sti(instr, &mut state),
            OP::JMP => instr::do_jmp(instr, &mut state),
            OP::RES => return,
            OP::LEA => instr::do_lea(instr, &mut state),
            OP::TRAP => instr::do_trap(instr, &mut state),
        }
    }
}

fn read_image_file(path: &String, state: &mut State) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; std::mem::size_of::<u16>()];

    /* the origin tells us where in memory to place the image */
    file.read_exact(&mut buffer)?;
    let origin = swap16(u16::from_ne_bytes(buffer));

    /* read the rest of the file */
    let mut address = origin;
    while let Ok(_) = file.read_exact(&mut buffer) {
        let read = swap16(u16::from_ne_bytes(buffer));
        state.mem.write(address, read);
        address += 1;
    }

    Ok(())
}

fn swap16(x: u16) -> u16 {
    x << 8 | x >> 8
}
