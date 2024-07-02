use defs::{OP, R};
use state::State;

mod defs;
mod instr;
mod state;

fn main() {
    // @{Load Arguments}
    // @{Setup}

    let mut state = State::new();

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

    // @{Shutdown}
}
